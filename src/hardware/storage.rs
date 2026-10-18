use hidapi::{HidApi, HidDevice};
use std::fs;
use std::path::Path;

pub use crate::actions::app_launch::*;
pub use crate::actions::desktop_apps::*;
pub use crate::actions::web_api::*;
pub use crate::hardware::firmware::*;
pub use crate::hardware::msc::*;
pub use crate::hardware::profile::*;

pub const DUCKYPAD_VID: u16 = 0x0483;
pub const PID_DUCKYPAD_PRO: u16 = 0xd11d;
pub const PID_DUCKYPAD_2020: u16 = 0xd11c;

pub const HID_REPORT_ID_OUT: u8 = 0x05;

pub const HID_CMD_QUERY_INFO: u8 = 0x00;
pub const HID_CMD_GOTO_PROFILE_BY_NUM: u8 = 0x01;
pub const HID_CMD_SET_LED_SINGLE: u8 = 0x04;
pub const HID_CMD_SOFTWARE_RESET: u8 = 0x14;
pub const HID_CMD_GOTO_PROFILE_BY_NAME: u8 = 0x17;
pub const HID_CMD_DUMP_PGV: u8 = 0x18;
pub const HID_CMD_WRITE_PGV: u8 = 0x19;

pub const PGV_COUNT: usize = 32;
pub const MAX_INPUT_SLOTS: usize = 58;
pub const ONBOARD_KEY_COUNT: usize = 20;

pub static HID_IO_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
pub static MSC_SYNC_IN_PROGRESS: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct ConnectedDuckyPadInfo {
    pub pid: u16,
    pub hw_model: u8,
    pub fw_version: String,
    pub serial: String,
    pub disk_label: String,
    pub active_profile: u8,
}

pub fn open_duckypad_hid() -> Result<HidDevice, String> {
    let api = HidApi::new().map_err(|e| format!("Failed to initialize hidapi: {e}"))?;
    for pid in [PID_DUCKYPAD_PRO, PID_DUCKYPAD_2020] {
        if let Ok(device) = api.open(DUCKYPAD_VID, pid) {
            return Ok(device);
        }
    }
    Err("No duckyPad Pro (0xd11d) or duckyPad (0xd11c) found on USB bus".into())
}

pub fn is_duckypad_hid_present() -> bool {
    let Ok(entries) = fs::read_dir("/sys/class/hidraw") else {
        return open_duckypad_hid().is_ok();
    };
    for entry in entries.flatten() {
        if let Ok(uevent) = fs::read_to_string(entry.path().join("device/uevent")) {
            let upper = uevent.to_ascii_uppercase();
            if upper.contains("00000483:0000D11D") || upper.contains("00000483:0000D11C") {
                let dev_node = Path::new("/dev").join(entry.file_name());
                if dev_node.exists() {
                    return true;
                }
            }
        }
    }
    false
}

pub fn query_connected_duckypad_info() -> Result<ConnectedDuckyPadInfo, String> {
    let _guard = HID_IO_LOCK.lock().ok();
    let api = HidApi::new().map_err(|e| format!("HID init error: {e}"))?;
    let mut found_dev: Option<(HidDevice, u16)> = None;
    for pid in [PID_DUCKYPAD_PRO, PID_DUCKYPAD_2020] {
        if let Ok(dev) = api.open(DUCKYPAD_VID, pid) {
            found_dev = Some((dev, pid));
            break;
        }
    }
    let Some((device, pid)) = found_dev else {
        return Err("No duckyPad Pro (0xd11d) or duckyPad 2020 (0xd11c) found".to_string());
    };

    let default_model = if pid == PID_DUCKYPAD_PRO { 24u8 } else { 20u8 };
    let mut fallback_serial = "-".to_string();
    let mut fallback_disk_label = String::new();
    if let Ok(Some(usb_serial)) = device.get_serial_number_string() {
        let trimmed = usb_serial.trim();
        if let Some((_, ser)) = trimmed.split_once('_') {
            if !ser.is_empty() {
                fallback_serial = ser.to_uppercase();
                if ser.len() >= 4 {
                    let suffix = &ser[ser.len() - 4..].to_uppercase();
                    fallback_disk_label = format!("DP{default_model}_{suffix}");
                }
            }
        } else if !trimmed.is_empty() {
            fallback_serial = trimmed.to_uppercase();
        }
    }

    for attempt in 0..3u8 {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x01u8.wrapping_add(attempt);
        tx[2] = HID_CMD_QUERY_INFO;

        if device.write(&tx).is_ok() {
            let mut rx = [0u8; 64];
            if let Ok(len) = device.read_timeout(&mut rx, 500) {
                if len >= 12 && rx[2] == 2 {
                    std::thread::sleep(std::time::Duration::from_millis(60));
                    continue;
                }
                if len >= 12 && rx[6] != 0 {
                    let hw_model = rx[6];
                    let raw_ver = (rx[3], rx[4], rx[5]);
                    let has_dds_sig = len >= 23 && &rx[20..23] == b"DDS";
                    let has_rtc_payload = len >= 20 && rx[16..20].iter().any(|&b| b != 0);
                    let fw_version = if raw_ver == (3, 4, 0)
                        || has_dds_sig
                        || (raw_ver == (1, 0, 0) && has_rtc_payload)
                    {
                        "1.0.0".to_string()
                    } else if raw_ver == (1, 0, 0) {
                        "1.0.0 (Official)".to_string()
                    } else {
                        format!("{}.{}.{}", rx[3], rx[4], rx[5])
                    };
                    let serial =
                        format!("{:02X}{:02X}{:02X}{:02X}", rx[7], rx[8], rx[9], rx[10]);
                    let disk_label = format!("DP{}_{:02X}{:02X}", rx[6], rx[9], rx[10]);
                    let active_profile = rx[11].max(1);
                    return Ok(ConnectedDuckyPadInfo {
                        pid,
                        hw_model,
                        fw_version,
                        serial,
                        disk_label,
                        active_profile,
                    });
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(40));
    }

    Ok(ConnectedDuckyPadInfo {
        pid,
        hw_model: default_model,
        fw_version: "-".to_string(),
        serial: fallback_serial,
        disk_label: fallback_disk_label,
        active_profile: 1,
    })
}

pub fn poll_hardware_active_profile() -> Option<u8> {
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return None;
    }
    let _guard = HID_IO_LOCK.try_lock().ok()?;
    let device = open_duckypad_hid().ok()?;

    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x2E;
    tx[2] = HID_CMD_QUERY_INFO;

    if device.write(&tx).is_err() {
        return None;
    }

    let mut rx = [0u8; 64];
    let len = device.read_timeout(&mut rx, 120).ok()?;
    if len >= 12 && rx[2] == 0 && rx[6] != 0 && rx[11] > 0 {
        return Some(rx[11]);
    }
    None
}


pub fn read_all_pgvs(device: &HidDevice) -> Result<[i32; PGV_COUNT], String> {
    let mut pgvs = [0i32; PGV_COUNT];

    for (seq, &start_idx) in [0u8, 15u8, 30u8].iter().enumerate() {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = (seq as u8).wrapping_add(0x20);
        tx[2] = HID_CMD_DUMP_PGV;
        tx[3] = start_idx;

        device
            .write(&tx)
            .map_err(|e| format!("HID write failed during PGV dump (idx {start_idx}): {e}"))?;

        let mut rx = [0u8; 64];
        let bytes_read = device
            .read_timeout(&mut rx, 500)
            .map_err(|e| format!("HID read timeout during PGV dump (idx {start_idx}): {e}"))?;

        if bytes_read < 7 || rx[2] != 0 {
            return Err(format!(
                "Invalid HID response for PGV dump at index {start_idx} (status byte: 0x{:02X})",
                rx[2]
            ));
        }

        let count_in_chunk = (PGV_COUNT - start_idx as usize).min(15);
        for offset in 0..count_in_chunk {
            let byte_pos = 3 + (offset * 4);
            if byte_pos + 4 <= bytes_read {
                let raw_bytes: [u8; 4] = [
                    rx[byte_pos],
                    rx[byte_pos + 1],
                    rx[byte_pos + 2],
                    rx[byte_pos + 3],
                ];
                pgvs[start_idx as usize + offset] = i32::from_le_bytes(raw_bytes);
            }
        }
    }

    Ok(pgvs)
}

pub fn write_pgvs(device: &HidDevice, updates: &[(u8, i32)]) -> Result<(), String> {
    if updates.is_empty() {
        return Ok(());
    }

    for &(idx, _) in updates {
        if idx as usize >= PGV_COUNT {
            return Err(format!(
                "PGV index _{idx} out of range (must be 0..=31 for _GV0.._GV31)"
            ));
        }
    }

    for (chunk_seq, chunk) in updates.chunks(12).enumerate() {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = (chunk_seq as u8).wrapping_add(0x40);
        tx[2] = HID_CMD_WRITE_PGV;

        let mut cursor = 3usize;
        for &(pgv_index, value) in chunk {
            tx[cursor] = pgv_index | 0x80;
            let le = value.to_le_bytes();
            tx[cursor + 1..cursor + 5].copy_from_slice(&le);
            cursor += 5;
        }

        for attempt in 0..5u8 {
            tx[1] = (chunk_seq as u8).wrapping_add(0x40).wrapping_add(attempt);
            device
                .write(&tx)
                .map_err(|e| format!("HID write failed for PGV batch #{chunk_seq}: {e}"))?;

            let mut rx = [0u8; 64];
            let n = device
                .read_timeout(&mut rx, 500)
                .map_err(|e| format!("HID ACK timeout on PGV write batch #{chunk_seq}: {e}"))?;
            if n >= 3 && rx[2] == 2 && attempt < 4 {
                std::thread::sleep(std::time::Duration::from_millis(30));
                continue;
            }
            break;
        }
    }

    Ok(())
}

pub fn write_single_pgv(device: &HidDevice, pgv_index: u8, value: i32) -> Result<(), String> {
    write_pgvs(device, &[(pgv_index, value)])
}

pub fn goto_profile_by_name(device: &HidDevice, profile_name: &str) -> Result<(), String> {
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x55;
    tx[2] = HID_CMD_GOTO_PROFILE_BY_NAME;

    let name_bytes = profile_name.as_bytes();
    let max_len = name_bytes.len().min(60);
    tx[3..3 + max_len].copy_from_slice(&name_bytes[..max_len]);

    device
        .write(&tx)
        .map_err(|e| format!("Failed to switch to profile '{profile_name}': {e}"))?;
    let mut rx = [0u8; 64];
    if let Err(e) = device.read_timeout(&mut rx, 200) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "HID",
            format!("HID ACK read timeout in goto_profile_by_name('{profile_name}'): {e}"),
        );
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_catalog_and_crc32_integrity() {
        let list = list_available_duckypad_pro_firmwares("1.0.0 (DuckyDeck Studio)");
        assert!(
            !list.is_empty(),
            "Expected at least 1 DuckyDeck Studio firmware release, got {}",
            list.len()
        );
        // First entry should be v1.0.0 (DuckyDeck Studio Firmware)
        let studio_entries: Vec<&FirmwareVersionEntry> =
            list.iter().filter(|e| !e.is_official).collect();
        assert!(!studio_entries.is_empty());
        assert_eq!(studio_entries[0].version, "1.0.0");
        assert_eq!(studio_entries[0].vendor, "DuckyDeck Studio");
        assert!(studio_entries[0].supports_monochrome_icons);
        for entry in studio_entries {
            let bytes =
                std::fs::read(&entry.path).expect("Bundled firmware binary should be readable");
            let crc = compute_ieee_crc32(&bytes);
            assert_eq!(
                format!("{crc:x}"),
                entry.crc32,
                "CRC32 mismatch for {}",
                entry.filename
            );
        }
    }

    #[test]
    fn test_profile_rotation_fingerprint_and_unsynced_changes() {
        let base_profiles = vec![DuckyProfile {
            number: 1,
            name: "Default".to_string(),
            ..Default::default()
        }];
        let mut rotated_profiles = base_profiles.clone();
        rotated_profiles[0]
            .preserved_config_lines
            .push("IS_LANDSCAPE 1".to_string());

        let base_fp = compute_profiles_fingerprint(&base_profiles);
        let rotated_fp = compute_profiles_fingerprint(&rotated_profiles);
        assert_ne!(
            base_fp, rotated_fp,
            "Changing IS_LANDSCAPE rotation must change the profile sync fingerprint"
        );

        // Verify empty slot insertion does NOT change the fingerprint (fixes phantom unsynced state)
        let mut with_empty_slot = base_profiles.clone();
        with_empty_slot[0]
            .slots
            .insert(5, DuckyKeySlot::default());
        assert_eq!(
            compute_profiles_fingerprint(&with_empty_slot),
            base_fp,
            "An effectively empty DuckyKeySlot must not change the profile sync fingerprint"
        );

        // Verify Desktop Widget orientation (DW_ROTATION) does NOT require a microSD sync
        let mut widget_rot_profiles = base_profiles.clone();
        widget_rot_profiles[0].set_widget_rotation_degrees(90);
        assert_eq!(
            compute_profiles_fingerprint(&widget_rot_profiles),
            base_fp,
            "Changing Desktop Widget orientation (DW_ROTATION) must NOT require a microSD sync"
        );
        widget_rot_profiles[0].set_widget_rotation_degrees(270);
        assert_eq!(
            compute_profiles_fingerprint(&widget_rot_profiles),
            base_fp,
            "Changing Desktop Widget orientation to 270 must NOT require a microSD sync"
        );

        // Verify per-profile webhook (PROFILE_WEBHOOK) does NOT require a microSD sync
        widget_rot_profiles[0].set_profile_webhook(&ProfileWebhookConfig {
            enabled: true,
            destination: String::new(),
            method: "POST".to_string(),
            url: "http://127.0.0.1:8123/api/webhook/test".to_string(),
            headers: "Content-Type: application/json".to_string(),
            body: "{\"profile\":\"{{profile_name}}\"}".to_string(),
        });
        assert_eq!(
            compute_profiles_fingerprint(&widget_rot_profiles),
            base_fp,
            "Setting a per-profile webhook must NOT require a microSD sync"
        );

        let temp_sd = std::env::temp_dir().join(format!(
            "duckydeck_rotation_test_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&temp_sd); // ignore: best-effort test temp dir cleanup
        fs::create_dir_all(&temp_sd).expect("create temp sd dir");
        base_profiles[0]
            .save_to_sd_root(&temp_sd)
            .expect("save base profile");
        save_profile_order_to_sd(&temp_sd, &base_profiles).expect("save profile_info.txt");

        // Verify unsynced changes for widget_rot_profiles is 0 changes
        let wrot_diff_json = compute_unsynced_changes_json(
            "TEST_ROTATION_SERIAL",
            &widget_rot_profiles,
            Some(&temp_sd),
            "synced",
        );
        assert!(
            wrot_diff_json.contains("\"totalChanges\":0"),
            "Desktop widget rotation should produce 0 unsynced SD changes, got: {wrot_diff_json}"
        );

        let diff_json = compute_unsynced_changes_json(
            "TEST_ROTATION_SERIAL",
            &rotated_profiles,
            Some(&temp_sd),
            "unsynced",
        );
        assert!(
            diff_json.contains("is_landscape") && diff_json.contains("OLED Orientation: 0° → 90°"),
            "Unsynced changes should include is_landscape diff, got: {diff_json}"
        );

        rotated_profiles[0].set_oled_rotation_degrees(180);
        assert_eq!(rotated_profiles[0].oled_rotation_degrees(), 180);
        let diff_180_json = compute_unsynced_changes_json(
            "TEST_ROTATION_SERIAL",
            &rotated_profiles,
            Some(&temp_sd),
            "unsynced",
        );
        assert!(
            diff_180_json.contains("OLED Orientation: 0° → 180°"),
            "Unsynced changes should include 180° diff, got: {diff_180_json}"
        );

        rotated_profiles[0].set_oled_rotation_degrees(270);
        assert_eq!(rotated_profiles[0].oled_rotation_degrees(), 270);
        let diff_270_json = compute_unsynced_changes_json(
            "TEST_ROTATION_SERIAL",
            &rotated_profiles,
            Some(&temp_sd),
            "unsynced",
        );
        assert!(
            diff_270_json.contains("OLED Orientation: 0° → 270°"),
            "Unsynced changes should include 270° diff, got: {diff_270_json}"
        );

        // Test separate Desktop Widget orientation
        rotated_profiles[0].set_widget_rotation_degrees(90);
        assert_eq!(rotated_profiles[0].widget_rotation_degrees(), 90);
        assert_eq!(rotated_profiles[0].effective_widget_rotation_degrees(), 90);
        rotated_profiles[0].set_widget_rotation_degrees(-1);
        assert_eq!(rotated_profiles[0].widget_rotation_degrees(), -1);
        assert_eq!(rotated_profiles[0].effective_widget_rotation_degrees(), 270);

        let discard_res = discard_unsynced_change_in_profiles(
            "TEST_ROTATION_SERIAL",
            &mut rotated_profiles,
            Some(&temp_sd),
            1,
            "Default",
            "is_landscape",
        );
        assert!(
            discard_res.is_ok(),
            "Discarding is_landscape should succeed: {discard_res:?}"
        );
        assert_eq!(
            compute_profiles_fingerprint(&rotated_profiles),
            base_fp,
            "Fingerprint should match baseline after discarding rotation change"
        );

        // Verify built-in profile templates instantiate cleanly
        let templates = list_all_profile_templates();
        assert!(templates.len() >= 4);
        let inst = instantiate_profile_from_template("builtin_media_productivity", 2, "My Stream");
        assert_eq!(inst.number, 2);
        assert_eq!(inst.name, "My Stream");
        assert!(inst.slots.contains_key(&1));

        let _ = fs::remove_dir_all(&temp_sd); // ignore: best-effort test temp dir cleanup
    }
}


