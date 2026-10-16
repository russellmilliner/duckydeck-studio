use hidapi::{HidApi, HidDevice};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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
                    let fw_version = format!("{}.{}.{}", rx[3], rx[4], rx[5]);
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

        if bytes_read < 8 || rx[2] != 0 {
            return Err(format!(
                "Invalid HID response for PGV dump at index {start_idx} (status byte: 0x{:02X})",
                rx[2]
            ));
        }

        let count_in_chunk = (PGV_COUNT - start_idx as usize).min(15);
        for offset in 0..count_in_chunk {
            let byte_pos = 4 + (offset * 4);
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

        device
            .write(&tx)
            .map_err(|e| format!("HID write failed for PGV batch #{chunk_seq}: {e}"))?;

        let mut rx = [0u8; 64];
        let _ = device
            .read_timeout(&mut rx, 500)
            .map_err(|e| format!("HID ACK timeout on PGV write batch #{chunk_seq}: {e}"))?;
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
    let _ = device.read_timeout(&mut rx, 200);
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckyKeySlot {
    pub slot: usize,
    pub label_line1: String,
    pub label_line2: String,
    pub custom_rgb: Option<[u8; 3]>,
    pub allow_abort: bool,
    pub disable_autorepeat: bool,
    pub on_press_script: String,
    pub on_release_script: String,
}

impl Default for DuckyKeySlot {
    fn default() -> Self {
        Self {
            slot: 1,
            label_line1: String::new(),
            label_line2: String::new(),
            custom_rgb: None,
            allow_abort: false,
            disable_autorepeat: false,
            on_press_script: String::new(),
            on_release_script: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckyProfile {
    pub number: u8,
    pub name: String,
    pub folder_name_override: Option<String>,
    pub bg_color: [u8; 3],
    pub keydown_color: Option<[u8; 3]>,
    pub dim_unused_keys: bool,
    pub slots: BTreeMap<usize, DuckyKeySlot>,
    pub preserved_config_lines: Vec<String>,
}

impl Default for DuckyProfile {
    fn default() -> Self {
        Self {
            number: 1,
            name: "Default".to_string(),
            folder_name_override: None,
            bg_color: [16, 48, 96],
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            slots: BTreeMap::new(),
            preserved_config_lines: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DuckySdConfig {
    pub last_profile: Option<u8>,
    pub fw_ver: Option<String>,
    pub serial_number: Option<String>,
}

pub fn read_dpp_config(sd_root: &Path) -> Option<DuckySdConfig> {
    let content = fs::read_to_string(sd_root.join("dpp_config.txt")).ok()?;
    let mut cfg = DuckySdConfig::default();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("last_profile ") {
            cfg.last_profile = rest.trim().parse::<u8>().ok();
        } else if let Some(rest) = trimmed.strip_prefix("fw_ver ") {
            let v = rest.trim();
            if !v.is_empty() {
                cfg.fw_ver = Some(v.to_string());
            }
        } else if let Some(rest) = trimmed.strip_prefix("serial_number ") {
            let s = rest.trim();
            if !s.is_empty() {
                cfg.serial_number = Some(s.to_string());
            }
        }
    }
    Some(cfg)
}

impl DuckyProfile {
    pub fn folder_name(&self) -> String {
        if let Some(ref existing) = self.folder_name_override {
            return existing.clone();
        }
        let sanitized: String = self
            .name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
            .take(24)
            .collect();
        format!("profile_{}", sanitized.trim())
    }

    pub fn load_from_dir(dir_path: &Path) -> io::Result<Self> {
        let folder_str = dir_path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid folder name"))?;

        let (number, name) = parse_profile_folder_name(folder_str).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Directory '{folder_str}' does not match 'profile_<Name>' or 'profile<N>_<Name>'"),
            )
        })?;

        Self::load_from_dir_with_meta(dir_path, number.unwrap_or(1), name)
    }

    pub fn load_from_dir_with_meta(dir_path: &Path, number: u8, name: String) -> io::Result<Self> {
        let folder_name_override = dir_path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());

        let mut profile = DuckyProfile {
            number,
            name,
            folder_name_override,
            keydown_color: None,
            ..Default::default()
        };

        let config_path = dir_path.join("config.txt");
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            for raw_line in content.lines() {
                let line = raw_line.trim();
                if line.is_empty() {
                    continue;
                }

                if let Some(rest) = line.strip_prefix("BG_COLOR ") {
                    if let Some(rgb) = parse_rgb_triplet(rest) {
                        profile.bg_color = rgb;
                    }
                } else if let Some(rest) = line.strip_prefix("KEYDOWN_COLOR ") {
                    if let Some(rgb) = parse_rgb_triplet(rest) {
                        profile.keydown_color = Some(rgb);
                    }
                } else if let Some(rest) = line.strip_prefix("DIM_UNUSED_KEYS ") {
                    profile.dim_unused_keys = rest.trim() != "0";
                } else if let Some(rest) = line.strip_prefix("SWCOLOR_") {
                    let mut parts = rest.split_whitespace();
                    if let Some(slot_str) = parts.next() {
                        if let Ok(slot_idx) = slot_str.parse::<usize>() {
                            let rgb_str = parts.collect::<Vec<_>>().join(" ");
                            if let Some(rgb) = parse_rgb_triplet(&rgb_str) {
                                let entry = profile.ensure_slot(slot_idx);
                                entry.custom_rgb = Some(rgb);
                            }
                        }
                    }
                } else if let Some((slot_idx, label1)) = parse_indexed_directive(line, "z") {
                    profile.ensure_slot(slot_idx).label_line1 = label1.to_string();
                } else if let Some((slot_idx, label2)) = parse_indexed_directive(line, "x") {
                    profile.ensure_slot(slot_idx).label_line2 = label2.to_string();
                } else if let Some(rest) = line.strip_prefix("ab ") {
                    if let Ok(slot_idx) = rest.trim().parse::<usize>() {
                        if (1..=MAX_INPUT_SLOTS).contains(&slot_idx) {
                            profile.ensure_slot(slot_idx).allow_abort = true;
                        }
                    }
                } else if let Some(rest) = line.strip_prefix("dr ") {
                    if let Ok(slot_idx) = rest.trim().parse::<usize>() {
                        if (1..=MAX_INPUT_SLOTS).contains(&slot_idx) {
                            profile.ensure_slot(slot_idx).disable_autorepeat = true;
                        }
                    }
                } else if let Some((slot_idx, val)) = parse_indexed_directive(line, "ab") {
                    profile.ensure_slot(slot_idx).allow_abort = val.trim() != "0";
                } else if let Some((slot_idx, val)) = parse_indexed_directive(line, "dr") {
                    profile.ensure_slot(slot_idx).disable_autorepeat = val.trim() != "0";
                } else {
                    profile.preserved_config_lines.push(line.to_string());
                }
            }
        }

        for slot_idx in 1..=MAX_INPUT_SLOTS {
            let press_path = dir_path.join(format!("key{slot_idx}.txt"));
            let press_dsb_path = dir_path.join(format!("key{slot_idx}.dsb"));
            let release_path = dir_path.join(format!("key{slot_idx}-release.txt"));
            let release_dsb_path = dir_path.join(format!("key{slot_idx}-release.dsb"));

            if press_path.exists() {
                let script = fs::read_to_string(&press_path)?;
                let (allow_abort, disable_autorepeat) = {
                    let slot = profile.ensure_slot(slot_idx);
                    slot.on_press_script = script.clone();
                    (slot.allow_abort, slot.disable_autorepeat)
                };
                // Auto-compile missing .dsb if keyN.txt exists without keyN.dsb
                if !press_dsb_path.exists() && !script.trim().is_empty() {
                    let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                        &script,
                        allow_abort,
                        disable_autorepeat,
                    );
                    if compiled.is_success {
                        let _ = fs::write(&press_dsb_path, &compiled.bin_array);
                    }
                }
            }
            if release_path.exists() {
                let script = fs::read_to_string(&release_path)?;
                let (allow_abort, disable_autorepeat) = {
                    let slot = profile.ensure_slot(slot_idx);
                    slot.on_release_script = script.clone();
                    (slot.allow_abort, slot.disable_autorepeat)
                };
                if !release_dsb_path.exists() && !script.trim().is_empty() {
                    let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                        &script,
                        allow_abort,
                        disable_autorepeat,
                    );
                    if compiled.is_success {
                        let _ = fs::write(&release_dsb_path, &compiled.bin_array);
                    }
                }
            }
        }

        Ok(profile)
    }

    pub fn save_to_sd_root(&self, sd_root: &Path) -> io::Result<PathBuf> {
        self.save_to_sd_root_with_stats(sd_root).map(|(dir, _, _)| dir)
    }

    pub fn save_to_sd_root_with_stats(
        &self,
        sd_root: &Path,
    ) -> io::Result<(PathBuf, usize, usize)> {
        let profile_dir = sd_root.join(self.folder_name());
        fs::create_dir_all(&profile_dir)?;

        let mut files_written = 0usize;
        let mut files_unchanged = 0usize;
        let mut record_write = |changed: bool| {
            if changed {
                files_written += 1;
            } else {
                files_unchanged += 1;
            }
        };

        let mut cfg = String::with_capacity(1024);
        for (&slot_idx, slot) in &self.slots {
            if !slot.label_line1.is_empty() {
                cfg.push_str(&format!("z{} {}\n", slot_idx, slot.label_line1));
            }
            if !slot.label_line2.is_empty() {
                cfg.push_str(&format!("x{} {}\n", slot_idx, slot.label_line2));
            }
        }

        cfg.push_str(&format!(
            "BG_COLOR {} {} {}\n",
            self.bg_color[0], self.bg_color[1], self.bg_color[2]
        ));

        if let Some(kd) = self.keydown_color {
            cfg.push_str(&format!("KEYDOWN_COLOR {} {} {}\n", kd[0], kd[1], kd[2]));
        }

        if !self.dim_unused_keys {
            cfg.push_str("DIM_UNUSED_KEYS 0\n");
        }

        for (&slot_idx, slot) in &self.slots {
            if slot_idx <= ONBOARD_KEY_COUNT {
                if let Some(rgb) = slot.custom_rgb {
                    cfg.push_str(&format!(
                        "SWCOLOR_{} {} {} {}\n",
                        slot_idx, rgb[0], rgb[1], rgb[2]
                    ));
                }
            }
            if slot.allow_abort {
                cfg.push_str(&format!("ab {}\n", slot_idx));
            }
            if slot.disable_autorepeat {
                cfg.push_str(&format!("dr {}\n", slot_idx));
            }
        }

        for extra in &self.preserved_config_lines {
            cfg.push_str(extra);
            cfg.push('\n');
        }

        record_write(write_file_if_changed_and_sync(
            &profile_dir.join("config.txt"),
            cfg.as_bytes(),
        )?);

        for slot_idx in 1..=MAX_INPUT_SLOTS {
            let press_path = profile_dir.join(format!("key{slot_idx}.txt"));
            let press_dsb_path = profile_dir.join(format!("key{slot_idx}.dsb"));
            let release_path = profile_dir.join(format!("key{slot_idx}-release.txt"));
            let release_dsb_path = profile_dir.join(format!("key{slot_idx}-release.dsb"));

            if let Some(slot) = self.slots.get(&slot_idx) {
                let has_press = !slot.on_press_script.trim().is_empty();
                let has_release = !slot.on_release_script.trim().is_empty();

                if has_press {
                    let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                        &slot.on_press_script,
                        slot.allow_abort,
                        slot.disable_autorepeat,
                    );
                    if !compiled.is_success {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "key{slot_idx}.txt compile error on line {}: {}",
                                compiled.error_line_number_starting_from_1, compiled.error_comment
                            ),
                        ));
                    }
                    record_write(write_file_if_changed_and_sync(
                        &press_path,
                        slot.on_press_script.as_bytes(),
                    )?);
                    record_write(write_file_if_changed_and_sync(
                        &press_dsb_path,
                        &compiled.bin_array,
                    )?);
                } else if has_release {
                    // When only On-Release script exists, write empty keyN.txt and 4-byte HALT keyN.dsb
                    let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                        "",
                        slot.allow_abort,
                        slot.disable_autorepeat,
                    );
                    if !compiled.is_success {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            compiled.error_comment,
                        ));
                    }
                    record_write(write_file_if_changed_and_sync(&press_path, b"")?);
                    record_write(write_file_if_changed_and_sync(
                        &press_dsb_path,
                        &compiled.bin_array,
                    )?);
                } else {
                    if press_path.exists() {
                        let _ = fs::remove_file(&press_path);
                        record_write(true);
                    }
                    if press_dsb_path.exists() {
                        let _ = fs::remove_file(&press_dsb_path);
                        record_write(true);
                    }
                }

                if has_release {
                    let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                        &slot.on_release_script,
                        slot.allow_abort,
                        slot.disable_autorepeat,
                    );
                    if !compiled.is_success {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "key{slot_idx}-release.txt compile error on line {}: {}",
                                compiled.error_line_number_starting_from_1, compiled.error_comment
                            ),
                        ));
                    }
                    record_write(write_file_if_changed_and_sync(
                        &release_path,
                        slot.on_release_script.as_bytes(),
                    )?);
                    record_write(write_file_if_changed_and_sync(
                        &release_dsb_path,
                        &compiled.bin_array,
                    )?);
                } else {
                    if release_path.exists() {
                        let _ = fs::remove_file(&release_path);
                        record_write(true);
                    }
                    if release_dsb_path.exists() {
                        let _ = fs::remove_file(&release_dsb_path);
                        record_write(true);
                    }
                }
            } else {
                if press_path.exists() {
                    let _ = fs::remove_file(&press_path);
                    record_write(true);
                }
                if press_dsb_path.exists() {
                    let _ = fs::remove_file(&press_dsb_path);
                    record_write(true);
                }
                if release_path.exists() {
                    let _ = fs::remove_file(&release_path);
                    record_write(true);
                }
                if release_dsb_path.exists() {
                    let _ = fs::remove_file(&release_dsb_path);
                    record_write(true);
                }
            }
        }

        if files_written > 0 {
            if let Ok(dir_handle) = fs::File::open(&profile_dir) {
                let _ = dir_handle.sync_all();
            }
        }

        Ok((profile_dir, files_written, files_unchanged))
    }

    pub fn ensure_slot(&mut self, slot_idx: usize) -> &mut DuckyKeySlot {
        self.slots.entry(slot_idx).or_insert_with(|| DuckyKeySlot {
            slot: slot_idx,
            ..Default::default()
        })
    }
}

fn write_file_if_changed_and_sync(path: &Path, contents: &[u8]) -> io::Result<bool> {
    if path.is_file() {
        if let Ok(existing) = fs::read(path) {
            if existing == contents {
                return Ok(false);
            }
        }
    }
    let mut file = fs::File::create(path)?;
    file.write_all(contents)?;
    file.sync_all()?;
    Ok(true)
}

pub fn find_mounted_duckypad_sd() -> Option<PathBuf> {
    if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
        for line in mounts.lines() {
            let mut parts = line.split_whitespace();
            let Some(_dev) = parts.next() else {
                continue;
            };
            let Some(mnt_raw) = parts.next() else {
                continue;
            };
            let mnt = mnt_raw.replace("\\040", " ");
            if mnt.starts_with("/run/media/") || mnt.starts_with("/media/") || mnt.starts_with("/mnt/") {
                let path = PathBuf::from(&mnt);
                if path.is_dir() && is_duckypad_sd_root(&path) {
                    return Some(path);
                }
            }
        }
    }

    let mut candidate_roots = Vec::new();
    if let Ok(user) = std::env::var("USER") {
        candidate_roots.push(PathBuf::from(format!("/run/media/{user}")));
        candidate_roots.push(PathBuf::from(format!("/media/{user}")));
    }
    for base in ["/run/media", "/media"] {
        candidate_roots.push(PathBuf::from(base));
        if let Ok(entries) = fs::read_dir(base) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() && !candidate_roots.contains(&p) {
                    candidate_roots.push(p);
                }
            }
        }
    }

    for root in &candidate_roots {
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && is_duckypad_sd_root(&path) {
                return Some(path);
            }
        }
    }

    None
}

pub fn load_all_sd_profiles(sd_root: &Path) -> io::Result<Vec<DuckyProfile>> {
    let mut profiles = Vec::new();
    let mut loaded_dirs = std::collections::HashSet::new();

    // 1. First, check duckyPad Pro's `profile_info.txt` (maps `<number> <ProfileName>` -> `profile_<ProfileName>`)
    let profile_info_path = sd_root.join("profile_info.txt");
    if profile_info_path.exists() {
        if let Ok(info_content) = fs::read_to_string(&profile_info_path) {
            for raw_line in info_content.lines() {
                let line = raw_line.trim();
                if line.is_empty() {
                    continue;
                }
                if let Some((num_str, prof_name)) = line.split_once(char::is_whitespace) {
                    if let Ok(number) = num_str.trim().parse::<u8>() {
                        let name = prof_name.trim().to_string();
                        let candidate_dpp = sd_root.join(format!("profile_{name}"));
                        let candidate_legacy = sd_root.join(format!("profile{number}_{name}"));
                        let target_dir = if candidate_dpp.is_dir() {
                            Some(candidate_dpp)
                        } else if candidate_legacy.is_dir() {
                            Some(candidate_legacy)
                        } else {
                            None
                        };

                        if let Some(dir_path) = target_dir {
                            if let Ok(profile) =
                                DuckyProfile::load_from_dir_with_meta(&dir_path, number, name)
                            {
                                if let Some(fname) = dir_path.file_name().and_then(|s| s.to_str()) {
                                    loaded_dirs.insert(fname.to_string());
                                }
                                profiles.push(profile);
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Also scan directory entries for any legacy `profile<N>_<Name>` or unlisted `profile_<Name>` folders
    let mut next_auto_num = profiles
        .iter()
        .map(|p| p.number)
        .max()
        .unwrap_or(0)
        .saturating_add(1);

    let mut extra_dirs: Vec<PathBuf> = fs::read_dir(sd_root)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    extra_dirs.sort();

    for path in extra_dirs {
        let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        if loaded_dirs.contains(fname) {
            continue;
        };
        if let Some((maybe_num, name)) = parse_profile_folder_name(fname) {
            let num = maybe_num.unwrap_or_else(|| {
                let n = next_auto_num.clamp(1, 64);
                next_auto_num = next_auto_num.saturating_add(1);
                n
            });
            if let Ok(profile) = DuckyProfile::load_from_dir_with_meta(&path, num, name) {
                loaded_dirs.insert(fname.to_string());
                profiles.push(profile);
            }
        }
    }

    profiles.sort_by_key(|p| p.number);
    Ok(profiles)
}

pub fn save_profile_order_to_sd(sd_root: &Path, profiles: &[DuckyProfile]) -> io::Result<()> {
    let profile_info_path = sd_root.join("profile_info.txt");
    let mut content = String::new();
    for prof in profiles {
        content.push_str(&format!("{} {}\n", prof.number, prof.name));
    }
    let _ = write_file_if_changed_and_sync(&profile_info_path, content.as_bytes())?;
    Ok(())
}

fn data_home_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let fallback = PathBuf::from(&home).join(".local").join("share");
    match std::env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let trimmed = val.trim();
            if trimmed.is_empty()
                || trimmed.contains(':')
                || !trimmed.starts_with('/')
                || trimmed.starts_with("/var/")
                || trimmed.starts_with("/usr/")
            {
                fallback
            } else {
                PathBuf::from(trimmed)
            }
        }
        Err(_) => fallback,
    }
}

pub fn profile_cache_dir() -> PathBuf {
    data_home_dir().join("duckydeck-studio").join("profile_cache")
}

pub fn profile_cache_dir_for_serial(serial: &str) -> PathBuf {
    let clean: String = serial
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if clean.is_empty() || clean == "-" {
        profile_cache_dir()
    } else {
        data_home_dir()
            .join("duckydeck-studio")
            .join("device_profiles")
            .join(clean)
    }
}

fn write_profiles_to_cache_dir(cache_dir: &Path, profiles: &[DuckyProfile]) {
    if fs::create_dir_all(cache_dir).is_err() {
        return;
    }
    let valid_folders: std::collections::HashSet<String> =
        profiles.iter().map(|p| p.folder_name()).collect();
    if let Ok(entries) = fs::read_dir(cache_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                    if parse_profile_folder_name(fname).is_some() && !valid_folders.contains(fname)
                    {
                        let _ = fs::remove_dir_all(&p);
                    }
                }
            }
        }
    }
    let _ = save_profile_order_to_sd(cache_dir, profiles);
    for prof in profiles {
        let _ = prof.save_to_sd_root(cache_dir);
    }
}

pub fn compute_profiles_fingerprint(profiles: &[DuckyProfile]) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut feed = |bytes: &[u8]| {
        for &b in bytes {
            hash ^= b as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    };

    for prof in profiles {
        feed(&[prof.number]);
        feed(prof.name.as_bytes());
        feed(&prof.bg_color);
        if let Some(kd) = prof.keydown_color {
            feed(&[1, kd[0], kd[1], kd[2]]);
        } else {
            feed(&[0]);
        }
        feed(&[u8::from(prof.dim_unused_keys)]);
        for slot_idx in 1..=MAX_INPUT_SLOTS {
            if let Some(slot) = prof.slots.get(&slot_idx) {
                feed(&[slot_idx as u8]);
                feed(slot.label_line1.as_bytes());
                feed(slot.label_line2.as_bytes());
                if let Some(c) = slot.custom_rgb {
                    feed(&[1, c[0], c[1], c[2]]);
                } else {
                    feed(&[0]);
                }
                feed(&[
                    u8::from(slot.allow_abort),
                    u8::from(slot.disable_autorepeat),
                ]);
                feed(slot.on_press_script.trim().as_bytes());
                feed(slot.on_release_script.trim().as_bytes());
            }
        }
    }
    format!("{hash:016X}")
}

pub fn mark_profiles_sd_synced(serial: &str, profiles: &[DuckyProfile]) {
    if profiles.is_empty() {
        return;
    }
    let fp = compute_profiles_fingerprint(profiles);
    let snapshot_json = serde_json::to_string(profiles).ok();
    let global_dir = profile_cache_dir();
    let _ = fs::create_dir_all(&global_dir);
    let _ = fs::write(global_dir.join(".sd_synced_hash"), &fp);
    if let Some(ref snap) = snapshot_json {
        let _ = fs::write(global_dir.join(".sd_synced_snapshot.json"), snap);
    }

    let dev_dir = profile_cache_dir_for_serial(serial);
    if dev_dir != global_dir {
        let _ = fs::create_dir_all(&dev_dir);
        let _ = fs::write(dev_dir.join(".sd_synced_hash"), &fp);
        if let Some(ref snap) = snapshot_json {
            let _ = fs::write(dev_dir.join(".sd_synced_snapshot.json"), snap);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SdWriteSummary {
    pub profiles_count: usize,
    pub files_written: usize,
    pub files_verified_unchanged: usize,
    pub folders_removed: usize,
    pub validated_fingerprint: String,
}

pub fn write_profiles_to_sd_root(
    sd_root: &Path,
    profiles: &[DuckyProfile],
) -> Result<SdWriteSummary, String> {
    if !sd_root.is_dir() {
        return Err(format!("SD root {} does not exist", sd_root.display()));
    }

    let mut summary = SdWriteSummary {
        profiles_count: profiles.len(),
        ..Default::default()
    };

    let valid_folders_lower: std::collections::HashSet<String> = profiles
        .iter()
        .map(|p| p.folder_name().to_ascii_lowercase())
        .collect();

    for prof in profiles {
        let (_, written, unchanged) = prof
            .save_to_sd_root_with_stats(sd_root)
            .map_err(|e| format!("Failed saving profile '{}': {e}", prof.name))?;
        summary.files_written += written;
        summary.files_verified_unchanged += unchanged;
    }

    let profile_info_path = sd_root.join("profile_info.txt");
    let mut info_content = String::new();
    for prof in profiles {
        info_content.push_str(&format!("{} {}\n", prof.number, prof.name));
    }
    match write_file_if_changed_and_sync(&profile_info_path, info_content.as_bytes()) {
        Ok(true) => summary.files_written += 1,
        Ok(false) => summary.files_verified_unchanged += 1,
        Err(e) => return Err(format!("Failed writing profile_info.txt: {e}")),
    }

    if let Ok(entries) = fs::read_dir(sd_root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                    if parse_profile_folder_name(fname).is_some()
                        && !valid_folders_lower.contains(&fname.to_ascii_lowercase())
                    {
                        if fs::remove_dir_all(&p).is_ok() {
                            summary.folders_removed += 1;
                        }
                    }
                }
            }
        }
    }

    if let Ok(root_dir_handle) = fs::File::open(sd_root) {
        let _ = root_dir_handle.sync_all();
    }
    let _ = std::process::Command::new("sync").output();
    // Allow USB 1.1 Full-Speed MSC and microSD controller to commit FAT clusters before read-back
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Read-back validation directly from the SD root before declaring success
    let read_back = load_all_sd_profiles(sd_root)
        .map_err(|e| format!("Read-back validation error on microSD: {e}"))?;
    let expected_fp = compute_profiles_fingerprint(profiles);
    let actual_fp = compute_profiles_fingerprint(&read_back);
    if read_back.len() != profiles.len() || actual_fp != expected_fp {
        return Err(format!(
            "Read-back validation mismatch on microSD: expected {} profile(s) [#{}], read back {} profile(s) [#{}]",
            profiles.len(),
            &expected_fp[..8.min(expected_fp.len())],
            read_back.len(),
            &actual_fp[..8.min(actual_fp.len())]
        ));
    }

    summary.validated_fingerprint = actual_fp;
    Ok(summary)
}

fn find_mount_block_device(mount_point: &Path) -> Option<PathBuf> {
    let mounts = fs::read_to_string("/proc/mounts").ok()?;
    let target_canon = fs::canonicalize(mount_point).unwrap_or_else(|_| mount_point.to_path_buf());
    for line in mounts.lines() {
        let mut parts = line.split_whitespace();
        let Some(dev) = parts.next() else {
            continue;
        };
        let Some(mnt_raw) = parts.next() else {
            continue;
        };
        let mnt = PathBuf::from(mnt_raw.replace("\\040", " "));
        let mnt_canon = fs::canonicalize(&mnt).unwrap_or(mnt.clone());
        if mnt == mount_point || mnt_canon == target_canon {
            return Some(PathBuf::from(dev));
        }
    }
    None
}

fn is_mount_point_mounted(mount_point: &Path, dev_path: Option<&Path>) -> bool {
    let Ok(mounts) = fs::read_to_string("/proc/mounts") else {
        return false;
    };
    let dev_canon = dev_path.and_then(|d| fs::canonicalize(d).ok().or_else(|| Some(d.to_path_buf())));
    for line in mounts.lines() {
        let mut parts = line.split_whitespace();
        let Some(dev) = parts.next() else {
            continue;
        };
        let Some(mnt_raw) = parts.next() else {
            continue;
        };
        let mnt = PathBuf::from(mnt_raw.replace("\\040", " "));
        if mnt == mount_point {
            return true;
        }
        if let Some(ref dc) = dev_canon {
            let line_dev = PathBuf::from(dev);
            let line_dev_canon = fs::canonicalize(&line_dev).unwrap_or(line_dev);
            if &line_dev_canon == dc {
                return true;
            }
        }
    }
    false
}

fn find_duckypad_msc_block_device(disk_label: &str, allow_whole_disk: bool) -> Option<PathBuf> {
    if !disk_label.is_empty() {
        let by_label = PathBuf::from(format!("/dev/disk/by-label/{disk_label}"));
        if by_label.exists() {
            if let Ok(canon) = fs::canonicalize(&by_label) {
                return Some(canon);
            }
        }
    }

    if let Ok(entries) = fs::read_dir("/dev/disk/by-label") {
        for entry in entries.flatten() {
            let label = entry.file_name().to_string_lossy().to_string();
            if label.starts_with("DP24_")
                || label.starts_with("DP20_")
                || label.starts_with("DUCKYPAD")
            {
                if let Ok(canon) = fs::canonicalize(entry.path()) {
                    return Some(canon);
                }
            }
        }
    }

    if let Ok(entries) = fs::read_dir("/dev/disk/by-id") {
        let mut whole_disk_fallback: Option<PathBuf> = None;
        for entry in entries.flatten() {
            let id_name = entry.file_name().to_string_lossy().to_string();
            if id_name.contains("TinyUSB_Flash_Storage") || id_name.contains("duckyPad") {
                if let Ok(canon) = fs::canonicalize(entry.path()) {
                    if id_name.contains("-part") {
                        return Some(canon);
                    }
                    // Check if kernel has already created the first partition node (e.g. /dev/sdh1)
                    let part1 = PathBuf::from(format!("{}1", canon.display()));
                    if part1.exists() {
                        return Some(part1);
                    }
                    whole_disk_fallback = Some(canon);
                }
            }
        }
        if allow_whole_disk && whole_disk_fallback.is_some() {
            return whole_disk_fallback;
        }
    }

    None
}

fn wait_and_mount_duckypad_msc(disk_label: &str) -> Result<(PathBuf, Option<PathBuf>), String> {
    let mut last_udisks_err = String::new();
    let mut settled_dev_once = false;

    for attempt in 0..48 {
        if let Some(mounted) = find_mounted_duckypad_sd() {
            std::thread::sleep(std::time::Duration::from_millis(350));
            let dev = find_mount_block_device(&mounted)
                .or_else(|| find_duckypad_msc_block_device(disk_label, true));
            return Ok((mounted, dev));
        }

        // Only allow superfloppy whole-disk fallback after ~4.5s (18 * 250ms) if no partition table exists
        let allow_whole_disk = attempt >= 18;
        if let Some(dev_path) = find_duckypad_msc_block_device(disk_label, allow_whole_disk) {
            if !settled_dev_once {
                settled_dev_once = true;
                // Give udev and udisksd a moment to finish probing the FAT32 partition before mounting
                std::thread::sleep(std::time::Duration::from_millis(350));
                if let Some(mounted) = find_mounted_duckypad_sd() {
                    let dev = find_mount_block_device(&mounted).or(Some(dev_path));
                    return Ok((mounted, dev));
                }
            }

            if let Ok(out) = std::process::Command::new("udisksctl")
                .args([
                    "mount",
                    "--no-user-interaction",
                    "-b",
                    &dev_path.to_string_lossy(),
                ])
                .output()
            {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                if !stderr.is_empty() && !stderr.contains("AlreadyMounted") {
                    last_udisks_err = format!("{} ({})", stderr, dev_path.display());
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(300));
            if let Some(mounted) = find_mounted_duckypad_sd() {
                std::thread::sleep(std::time::Duration::from_millis(300));
                let dev = find_mount_block_device(&mounted).or(Some(dev_path));
                return Ok((mounted, dev));
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    }

    if last_udisks_err.is_empty() {
        Err("Timed out waiting for duckyPad microSD partition to appear and mount in USB Mass Storage mode".to_string())
    } else {
        Err(format!(
            "Timed out mounting duckyPad microSD in USB Mass Storage mode: {last_udisks_err}"
        ))
    }
}

fn unmount_duckypad_sd_cleanly(mount_root: &Path, known_dev: Option<&Path>) -> Result<(), String> {
    let block_dev = known_dev
        .map(|p| p.to_path_buf())
        .or_else(|| find_mount_block_device(mount_root));

    // Flush Linux filesystem buffers and allow USB 1.1 MSC pipeline to drain before unmounting
    let _ = std::process::Command::new("sync").output();
    std::thread::sleep(std::time::Duration::from_millis(600));

    for attempt in 0..24 {
        if !is_mount_point_mounted(mount_root, block_dev.as_deref()) {
            let _ = std::process::Command::new("sync").output();
            // Give the STM32 TinyUSB MSC driver and SD card flash controller 1 full second
            // to finish internal FAT table / wear-leveling commits before software reset
            std::thread::sleep(std::time::Duration::from_millis(1000));
            return Ok(());
        }

        if let Some(ref dev_path) = block_dev {
            let _ = std::process::Command::new("udisksctl")
                .args([
                    "unmount",
                    "--no-user-interaction",
                    "-b",
                    &dev_path.to_string_lossy(),
                ])
                .output();
        }

        if attempt >= 3 && is_mount_point_mounted(mount_root, block_dev.as_deref()) {
            let _ = std::process::Command::new("umount")
                .arg(mount_root.as_os_str())
                .output();
        }

        let _ = std::process::Command::new("sync").output();
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    if !is_mount_point_mounted(mount_root, block_dev.as_deref()) {
        let _ = std::process::Command::new("sync").output();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    } else {
        Err(format!(
            "Volume {} is still busy and could not be unmounted cleanly",
            mount_root.display()
        ))
    }
}

pub fn check_profile_sync_status(
    serial: &str,
    profiles: &[DuckyProfile],
    sd_root: Option<&Path>,
    connected: bool,
    active_profile: u8,
) -> (String, String) {
    let mem_hash = compute_profiles_fingerprint(profiles);

    if connected {
        if profiles.is_empty()
            || (profiles.len() == 1 && profiles[0].name == "Blender Sculpt")
            || (active_profile as usize) > profiles.len()
        {
            return (
                "mismatch".to_string(),
                format!(
                    "Profile mismatch: connected duckyPad is on Profile #{active_profile}, which does not match the {} loaded profile(s). Click to sync.",
                    profiles.len()
                ),
            );
        }
    }

    if let Some(sd_dir) = sd_root {
        if is_duckypad_sd_root(sd_dir) {
            if let Ok(sd_profiles) = load_all_sd_profiles(sd_dir) {
                let sd_hash = compute_profiles_fingerprint(&sd_profiles);
                if sd_hash != mem_hash {
                    return (
                        "mismatch".to_string(),
                        "Loaded profiles differ from the mounted microSD card. Click to sync or reload.".to_string(),
                    );
                } else {
                    mark_profiles_sd_synced(serial, profiles);
                    return (
                        "synced".to_string(),
                        format!(
                            "Validated & Synced: All {} profile(s) match the mounted duckyPad microSD card (hash #{}).",
                            profiles.len(),
                            &mem_hash[..8.min(mem_hash.len())]
                        ),
                    );
                }
            }
        }
    }

    let dev_hash_path = profile_cache_dir_for_serial(serial).join(".sd_synced_hash");
    let global_hash_path = profile_cache_dir().join(".sd_synced_hash");
    let saved_hash = fs::read_to_string(&dev_hash_path)
        .or_else(|_| fs::read_to_string(&global_hash_path))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if let Some(ref sd_hash) = saved_hash {
        if *sd_hash == mem_hash {
            let dev_snap_path = profile_cache_dir_for_serial(serial).join(".sd_synced_snapshot.json");
            let global_snap_path = profile_cache_dir().join(".sd_synced_snapshot.json");
            if !dev_snap_path.exists() && !global_snap_path.exists() {
                mark_profiles_sd_synced(serial, profiles);
            }
            return (
                "synced".to_string(),
                format!(
                    "Validated & Synced: All {} profile(s) match the duckyPad microSD card (hash #{}).",
                    profiles.len(),
                    &mem_hash[..8.min(mem_hash.len())]
                ),
            );
        } else {
            return (
                "unsynced".to_string(),
                "You have unsynced profile changes in DuckyDeck Studio that have not been written to the duckyPad microSD card yet. Click to sync.".to_string(),
            );
        }
    }

    if connected {
        (
            "mismatch".to_string(),
            "Loaded profiles have not been verified against this duckyPad's microSD card yet. Click to sync or reload from device.".to_string(),
        )
    } else {
        (
            "unsynced".to_string(),
            "Offline profile cache (connect duckyPad or mount microSD card to sync).".to_string(),
        )
    }
}

pub fn load_sd_synced_snapshot(serial: &str, sd_root: Option<&Path>) -> Option<Vec<DuckyProfile>> {
    if let Some(root) = sd_root {
        if root.exists() {
            if let Ok(sd_profiles) = load_all_sd_profiles(root) {
                if !sd_profiles.is_empty() {
                    return Some(sd_profiles);
                }
            }
        }
    }

    let dev_snap_path = profile_cache_dir_for_serial(serial).join(".sd_synced_snapshot.json");
    let global_snap_path = profile_cache_dir().join(".sd_synced_snapshot.json");
    let raw = fs::read_to_string(&dev_snap_path)
        .or_else(|_| fs::read_to_string(&global_snap_path))
        .ok()?;
    serde_json::from_str::<Vec<DuckyProfile>>(&raw).ok()
}

pub fn compute_unsynced_changes_json(
    serial: &str,
    current_profiles: &[DuckyProfile],
    sd_root: Option<&Path>,
    sync_state: &str,
) -> String {
    let baseline_opt = load_sd_synced_snapshot(serial, sd_root);
    let Some(baseline) = baseline_opt else {
        // No snapshot yet; summarize current profiles that will be synced
        let mut items = Vec::new();
        for p in current_profiles {
            let configured_keys = p
                .slots
                .values()
                .filter(|k| {
                    !k.label_line1.is_empty()
                        || !k.label_line2.is_empty()
                        || !k.on_press_script.trim().is_empty()
                        || !k.on_release_script.trim().is_empty()
                })
                .count();
            items.push(serde_json::json!({
                "kind": "unverified",
                "profileNumber": p.number,
                "profileName": p.name,
                "summary": format!("Profile #{}: \"{}\" ({} configured keys)", p.number, p.name, configured_keys),
                "details": [
                    format!("Default RGB: ({}, {}, {})", p.bg_color[0], p.bg_color[1], p.bg_color[2]),
                    format!("{} active key(s) ready to write to microSD card", configured_keys)
                ]
            }));
        }
        return serde_json::json!({
            "hasBaseline": false,
            "syncState": sync_state,
            "totalChanges": items.len(),
            "headline": if sync_state == "synced" {
                "All profiles are in sync with the duckyPad microSD card."
            } else {
                "No prior microSD sync baseline found — syncing will write all current profiles below to the duckyPad microSD card:"
            },
            "items": items
        })
        .to_string();
    };

    let mut items = Vec::new();
    let mut total_changes: usize = 0;

    // Match profiles by (number, or fallback by name if reordered)
    for cur in current_profiles {
        let base_by_num = baseline.iter().find(|b| b.number == cur.number);
        let base_by_name = baseline.iter().find(|b| b.name == cur.name);
        let base_opt = base_by_num.or(base_by_name);

        let Some(base) = base_opt else {
            let configured_keys = cur
                .slots
                .values()
                .filter(|k| !k.label_line1.is_empty() || !k.on_press_script.trim().is_empty())
                .count();
            total_changes += 1;
            items.push(serde_json::json!({
                "kind": "added",
                "profileNumber": cur.number,
                "profileName": cur.name,
                "summary": format!("New Profile #{}: \"{}\"", cur.number, cur.name),
                "details": [
                    format!("Added with {} configured key(s), Default RGB ({}, {}, {})", configured_keys, cur.bg_color[0], cur.bg_color[1], cur.bg_color[2])
                ]
            }));
            continue;
        };

        let mut details: Vec<String> = Vec::new();
        if base.number != cur.number {
            details.push(format!("Reordered slot: #{} → #{}", base.number, cur.number));
        }
        if base.name != cur.name {
            details.push(format!("Renamed profile: \"{}\" → \"{}\"", base.name, cur.name));
        }
        if base.bg_color != cur.bg_color {
            details.push(format!(
                "Default LED Color (BG_COLOR): ({}, {}, {}) → ({}, {}, {})",
                base.bg_color[0], base.bg_color[1], base.bg_color[2],
                cur.bg_color[0], cur.bg_color[1], cur.bg_color[2]
            ));
        }
        if base.keydown_color != cur.keydown_color {
            details.push(format!(
                "Keydown Color: {:?} → {:?}",
                base.keydown_color, cur.keydown_color
            ));
        }
        if base.dim_unused_keys != cur.dim_unused_keys {
            details.push(format!(
                "Dim Unused Keys: {} → {}",
                base.dim_unused_keys, cur.dim_unused_keys
            ));
        }

        // Compare slots 1..=MAX_INPUT_SLOTS
        for slot_idx in 1..=MAX_INPUT_SLOTS {
            let bk = base.slots.get(&slot_idx);
            let ck = cur.slots.get(&slot_idx);
            match (bk, ck) {
                (Some(b), Some(c)) => {
                    let mut key_diffs: Vec<String> = Vec::new();
                    if b.label_line1 != c.label_line1 || b.label_line2 != c.label_line2 {
                        let old_lbl = if b.label_line2.is_empty() {
                            b.label_line1.clone()
                        } else {
                            format!("{} / {}", b.label_line1, b.label_line2)
                        };
                        let new_lbl = if c.label_line2.is_empty() {
                            c.label_line1.clone()
                        } else {
                            format!("{} / {}", c.label_line1, c.label_line2)
                        };
                        key_diffs.push(format!("label \"{}\" → \"{}\"", old_lbl, new_lbl));
                    }
                    if b.custom_rgb != c.custom_rgb {
                        let old_c = b
                            .custom_rgb
                            .map(|rgb| format!("({}, {}, {})", rgb[0], rgb[1], rgb[2]))
                            .unwrap_or_else(|| "Profile Default".to_string());
                        let new_c = c
                            .custom_rgb
                            .map(|rgb| format!("({}, {}, {})", rgb[0], rgb[1], rgb[2]))
                            .unwrap_or_else(|| "Profile Default".to_string());
                        key_diffs.push(format!("RGB {} → {}", old_c, new_c));
                    }
                    let b_api = has_web_api_config(&b.on_press_script);
                    let c_api = has_web_api_config(&c.on_press_script);
                    if b_api != c_api {
                        if c_api {
                            let w = extract_web_api_config(&c.on_press_script, slot_idx);
                            key_diffs.push(format!("Web API added ({} {})", w.method, w.url));
                        } else {
                            key_diffs.push("Web API removed".to_string());
                        }
                    } else if b_api && c_api {
                        let w1 = extract_web_api_config(&b.on_press_script, slot_idx);
                        let w2 = extract_web_api_config(&c.on_press_script, slot_idx);
                        if serde_json::to_string(&w1).ok() != serde_json::to_string(&w2).ok() {
                            key_diffs.push(format!(
                                "Web API updated ({} {} → {} {})",
                                w1.method, w1.url, w2.method, w2.url
                            ));
                        }
                    }
                    if b.on_press_script.trim() != c.on_press_script.trim() {
                        let old_lines = b.on_press_script.trim().lines().count();
                        let new_lines = c.on_press_script.trim().lines().count();
                        key_diffs.push(format!(
                            "on-press script modified ({} → {} line(s))",
                            old_lines, new_lines
                        ));
                    }
                    if b.on_release_script.trim() != c.on_release_script.trim() {
                        let old_lines = b.on_release_script.trim().lines().count();
                        let new_lines = c.on_release_script.trim().lines().count();
                        key_diffs.push(format!(
                            "on-release script modified ({} → {} line(s))",
                            old_lines, new_lines
                        ));
                    }
                    if b.allow_abort != c.allow_abort || b.disable_autorepeat != c.disable_autorepeat {
                        key_diffs.push("execution flags changed".to_string());
                    }
                    if !key_diffs.is_empty() {
                        let key_title = if !c.label_line1.is_empty() {
                            format!("Key {} ({})", slot_idx, c.label_line1)
                        } else if !b.label_line1.is_empty() {
                            format!("Key {} ({})", slot_idx, b.label_line1)
                        } else {
                            format!("Key {}", slot_idx)
                        };
                        details.push(format!("{}: {}", key_title, key_diffs.join(", ")));
                    }
                }
                (None, Some(c)) => {
                    if !c.label_line1.is_empty()
                        || !c.on_press_script.trim().is_empty()
                        || !c.on_release_script.trim().is_empty()
                        || c.custom_rgb.is_some()
                    {
                        details.push(format!("Key {} added (\"{}\")", slot_idx, c.label_line1));
                    }
                }
                (Some(b), None) => {
                    if !b.label_line1.is_empty()
                        || !b.on_press_script.trim().is_empty()
                        || !b.on_release_script.trim().is_empty()
                        || b.custom_rgb.is_some()
                    {
                        details.push(format!("Key {} cleared (was \"{}\")", slot_idx, b.label_line1));
                    }
                }
                (None, None) => {}
            }
        }

        if !details.is_empty() {
            total_changes += details.len();
            items.push(serde_json::json!({
                "kind": "modified",
                "profileNumber": cur.number,
                "profileName": cur.name,
                "summary": format!("Profile #{}: \"{}\" ({} change{})", cur.number, cur.name, details.len(), if details.len() == 1 { "" } else { "s" }),
                "details": details
            }));
        }
    }

    // Check deleted profiles
    for base in &baseline {
        let still_exists = current_profiles
            .iter()
            .any(|c| c.number == base.number || c.name == base.name);
        if !still_exists {
            total_changes += 1;
            items.push(serde_json::json!({
                "kind": "deleted",
                "profileNumber": base.number,
                "profileName": base.name,
                "summary": format!("Deleted Profile #{}: \"{}\"", base.number, base.name),
                "details": [
                    format!("Profile folder {} will be removed from microSD card", base.folder_name())
                ]
            }));
        }
    }

    let headline = if items.is_empty() {
        "All profiles and keys match the last synced duckyPad microSD card state.".to_string()
    } else {
        format!(
            "{} unsynced change(s) across {} profile(s) waiting to be written to the duckyPad microSD card:",
            total_changes,
            items.len()
        )
    };

    serde_json::json!({
        "hasBaseline": true,
        "syncState": sync_state,
        "totalChanges": total_changes,
        "headline": headline,
        "items": items
    })
    .to_string()
}

pub fn sync_profiles_to_cache(profiles: &[DuckyProfile]) {
    if profiles.is_empty() {
        return;
    }
    write_profiles_to_cache_dir(&profile_cache_dir(), profiles);
}

pub fn sync_profiles_to_device_cache(serial: &str, profiles: &[DuckyProfile]) {
    if profiles.is_empty() {
        return;
    }
    write_profiles_to_cache_dir(&profile_cache_dir(), profiles);
    let dev_dir = profile_cache_dir_for_serial(serial);
    if dev_dir != profile_cache_dir() {
        write_profiles_to_cache_dir(&dev_dir, profiles);
    }
}

pub fn load_cached_profiles_for_serial(serial: &str) -> Option<Vec<DuckyProfile>> {
    let clean: String = serial
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if clean.is_empty() || clean == "-" {
        return None;
    }
    let dev_cache = profile_cache_dir_for_serial(&clean);
    if dev_cache.is_dir() {
        if let Ok(profiles) = load_all_sd_profiles(&dev_cache) {
            if !profiles.is_empty() {
                return Some(profiles);
            }
        }
    }
    None
}

pub fn load_profiles_for_connected_device(pid: u16, serial: &str) -> Option<Vec<DuckyProfile>> {
    if let Some(profiles) = load_cached_profiles_for_serial(serial) {
        return Some(profiles);
    }

    // Check if there is a backup from duckyPad-Configurator for this device model
    let backups_root = data_home_dir().join("duckypad_config").join("profile_backups");
    let model_prefix = if pid == PID_DUCKYPAD_PRO {
        "duckyPad_Pro_backup_"
    } else {
        "duckyPad_backup_"
    };

    let mut latest_backup: Option<PathBuf> = None;
    if backups_root.is_dir() {
        if let Ok(entries) = fs::read_dir(&backups_root) {
            let mut matching: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.is_dir()
                        && p.file_name()
                            .and_then(|s| s.to_str())
                            .map(|n| n.starts_with(model_prefix))
                            .unwrap_or(false)
                        && is_duckypad_sd_root(p)
                })
                .collect();
            matching.sort();
            latest_backup = matching.pop();
        }
    }

    if let Some(backup_dir) = latest_backup {
        if let Ok(profiles) = load_all_sd_profiles(&backup_dir) {
            if !profiles.is_empty() {
                sync_profiles_to_device_cache(serial, &profiles);
                return Some(profiles);
            }
        }
    }

    let global_cache = profile_cache_dir();
    if global_cache.is_dir() {
        if let Ok(profiles) = load_all_sd_profiles(&global_cache) {
            if !profiles.is_empty() {
                sync_profiles_to_device_cache(serial, &profiles);
                return Some(profiles);
            }
        }
    }

    load_cached_or_backup_profiles()
}

pub fn load_cached_or_backup_profiles() -> Option<Vec<DuckyProfile>> {
    let cache_dir = profile_cache_dir();
    if cache_dir.is_dir() {
        if let Ok(profiles) = load_all_sd_profiles(&cache_dir) {
            if !profiles.is_empty() {
                return Some(profiles);
            }
        }
    }

    let backups_root = data_home_dir().join("duckypad_config").join("profile_backups");
    if backups_root.is_dir() {
        if let Ok(entries) = fs::read_dir(&backups_root) {
            let mut dirs: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.is_dir() && is_duckypad_sd_root(p))
                .collect();
            dirs.sort();
            if let Some(latest_backup) = dirs.last() {
                if let Ok(profiles) = load_all_sd_profiles(latest_backup) {
                    if !profiles.is_empty() {
                        sync_profiles_to_cache(&profiles);
                        return Some(profiles);
                    }
                }
            }
        }
    }

    None
}

pub fn read_duckypad_pro_profiles_via_msc(
    disk_label: &str,
    serial: &str,
    restore_profile: u8,
) -> Result<Vec<DuckyProfile>, String> {
    read_duckypad_pro_profiles_via_msc_with_progress(
        disk_label,
        serial,
        restore_profile,
        |_, _, _| {},
    )
}

pub fn read_duckypad_pro_profiles_via_msc_with_progress<F>(
    disk_label: &str,
    serial: &str,
    restore_profile: u8,
    mut on_progress: F,
) -> Result<Vec<DuckyProfile>, String>
where
    F: FnMut(i32, &str, &str),
{
    if let Some(already_mounted) = find_mounted_duckypad_sd() {
        on_progress(
            35,
            "Reading profiles from mounted duckyPad microSD...",
            &format!("• Found mounted microSD at {}", already_mounted.display()),
        );
        let profiles = load_all_sd_profiles(&already_mounted)
            .map_err(|e| format!("Failed reading mounted microSD: {e}"))?;
        if !profiles.is_empty() {
            let fp = compute_profiles_fingerprint(&profiles);
            sync_profiles_to_device_cache(serial, &profiles);
            mark_profiles_sd_synced(serial, &profiles);
            on_progress(
                100,
                "Validated: Profiles loaded from duckyPad microSD!",
                &format!(
                    "✓ Loaded & verified {} profile(s) from {} [hash #{}]",
                    profiles.len(),
                    already_mounted.display(),
                    &fp[..8.min(fp.len())]
                ),
            );
            return Ok(profiles);
        }
    }

    struct MscGuard;
    impl Drop for MscGuard {
        fn drop(&mut self) {
            MSC_SYNC_IN_PROGRESS.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
    MSC_SYNC_IN_PROGRESS.store(true, std::sync::atomic::Ordering::SeqCst);
    let _msc_guard = MscGuard;

    on_progress(
        15,
        "Step 1/4: Switching duckyPad Pro to USB Mass Storage mode...",
        &format!("• Sending USB MSC command (0x14) to duckyPad Pro ({disk_label})..."),
    );

    // 1. Reboot duckyPad Pro into USB Mass Storage mode (0x14, tx[3] = 1)
    {
        let _guard = HID_IO_LOCK.lock().ok();
        let dev = open_duckypad_hid()?;
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x11;
        tx[2] = HID_CMD_SOFTWARE_RESET;
        tx[3] = 1;
        dev.write(&tx)
            .map_err(|e| format!("Failed to enter USB storage mode: {e}"))?;
    }

    on_progress(
        35,
        "Step 2/4: Waiting for microSD block device & mounting...",
        "• Waiting for duckyPad Pro microSD card to enumerate and mount...",
    );

    // 2. Wait for block device and mount
    let mount_res = wait_and_mount_duckypad_msc(disk_label);
    let loaded_profiles: Result<Vec<DuckyProfile>, String> = match mount_res {
        Ok((mount_root, real_dev)) => {
            let dev_str = real_dev
                .as_ref()
                .map(|d| d.display().to_string())
                .unwrap_or_else(|| disk_label.to_string());
            on_progress(
                60,
                "Step 3/4: Reading profiles & scripts from microSD...",
                &format!("✓ Mounted microSD ({dev_str}) at {}", mount_root.display()),
            );

            let res = match load_all_sd_profiles(&mount_root) {
                Ok(profiles) if !profiles.is_empty() => {
                    let fp = compute_profiles_fingerprint(&profiles);
                    sync_profiles_to_device_cache(serial, &profiles);
                    mark_profiles_sd_synced(serial, &profiles);
                    on_progress(
                        80,
                        "Step 4/4: Flushing & unmounting microSD before returning to HID mode...",
                        &format!(
                            "✓ Read {} profile(s) from microSD [hash #{}]",
                            profiles.len(),
                            &fp[..8.min(fp.len())]
                        ),
                    );
                    Ok(profiles)
                }
                Ok(_) => Err(format!(
                    "No profile folders found on {}",
                    mount_root.display()
                )),
                Err(e) => Err(format!("Error reading profiles from SD card: {e}")),
            };

            // 3. Cleanly unmount and verify in /proc/mounts before resetting
            let _ = unmount_duckypad_sd_cleanly(&mount_root, real_dev.as_deref());
            res
        }
        Err(e) => Err(e),
    };

    // 4. Reset duckyPad Pro back to normal HID keypad mode (0x14, tx[3] = 0)
    for _ in 0..20 {
        let reset_ok = {
            let _guard = HID_IO_LOCK.lock().ok();
            if let Ok(dev) = open_duckypad_hid() {
                let mut tx = [0u8; 64];
                tx[0] = HID_REPORT_ID_OUT;
                tx[1] = 0x12;
                tx[2] = HID_CMD_SOFTWARE_RESET;
                tx[3] = 0;
                dev.write(&tx).is_ok()
            } else {
                false
            }
        };
        if reset_ok {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(180));
    }

    // 5. Wait for normal HID mode to re-enumerate and restore active profile
    std::thread::sleep(std::time::Duration::from_millis(1200));
    for _ in 0..15 {
        let ready = {
            let _guard = HID_IO_LOCK.lock().ok();
            open_duckypad_hid().is_ok()
        };
        if ready {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    drop(_msc_guard);
    if restore_profile > 0 {
        let _ = crate::kwin::auto_switcher::send_hid_switch_profile(restore_profile);
    }

    if let Ok(ref profs) = loaded_profiles {
        on_progress(
            100,
            "Validated: Profiles reloaded from duckyPad Pro!",
            &format!(
                "✓ Cleanly unmounted microSD and restored duckyPad Pro to HID mode ({} profiles loaded)",
                profs.len()
            ),
        );
    }

    loaded_profiles
}

pub fn write_duckypad_pro_profiles_via_msc(
    disk_label: &str,
    serial: &str,
    profiles: &[DuckyProfile],
    restore_profile: u8,
) -> Result<SdWriteSummary, String> {
    write_duckypad_pro_profiles_via_msc_with_progress(
        disk_label,
        serial,
        profiles,
        restore_profile,
        |_, _, _| {},
    )
}

pub fn write_duckypad_pro_profiles_via_msc_with_progress<F>(
    disk_label: &str,
    serial: &str,
    profiles: &[DuckyProfile],
    restore_profile: u8,
    mut on_progress: F,
) -> Result<SdWriteSummary, String>
where
    F: FnMut(i32, &str, &str),
{
    if let Some(already_mounted) = find_mounted_duckypad_sd() {
        on_progress(
            30,
            "Step 1/2: Writing profile changes & .dsb bytecode to mounted microSD...",
            &format!("• Found mounted microSD at {}", already_mounted.display()),
        );
        let summary = write_profiles_to_sd_root(&already_mounted, profiles)?;
        sync_profiles_to_device_cache(serial, profiles);
        mark_profiles_sd_synced(serial, profiles);
        let short_fp = &summary.validated_fingerprint[..8.min(summary.validated_fingerprint.len())];
        on_progress(
            100,
            "Validated: All changes saved to mounted duckyPad microSD!",
            &format!(
                "✓ Wrote {} updated file(s) ({} unchanged skipped) & verified {} profile(s) [hash #{short_fp}]",
                summary.files_written, summary.files_verified_unchanged, summary.profiles_count
            ),
        );
        return Ok(summary);
    }

    struct MscGuard;
    impl Drop for MscGuard {
        fn drop(&mut self) {
            MSC_SYNC_IN_PROGRESS.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
    MSC_SYNC_IN_PROGRESS.store(true, std::sync::atomic::Ordering::SeqCst);
    let _msc_guard = MscGuard;

    on_progress(
        12,
        "Step 1/5: Switching duckyPad Pro to USB Mass Storage mode...",
        &format!("• Sending USB MSC command (0x14) to duckyPad Pro ({disk_label})..."),
    );

    // 1. Reboot duckyPad Pro into USB Mass Storage mode (0x14, tx[3] = 1)
    {
        let _guard = HID_IO_LOCK.lock().ok();
        let dev = open_duckypad_hid()?;
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x13;
        tx[2] = HID_CMD_SOFTWARE_RESET;
        tx[3] = 1;
        dev.write(&tx)
            .map_err(|e| format!("Failed to enter USB storage mode: {e}"))?;
    }

    on_progress(
        30,
        "Step 2/5: Waiting for FAT32 partition & mounting microSD...",
        "• Waiting for duckyPad Pro microSD partition to enumerate on USB bus...",
    );

    // 2. Wait for block device and mount
    let mount_res = wait_and_mount_duckypad_msc(disk_label);
    let write_res: Result<SdWriteSummary, String> = match mount_res {
        Ok((mount_root, real_dev)) => {
            let dev_str = real_dev
                .as_ref()
                .map(|d| d.display().to_string())
                .unwrap_or_else(|| disk_label.to_string());
            on_progress(
                52,
                "Step 3/5: Writing changed profile files & compiled .dsb bytecode...",
                &format!("✓ Mounted microSD ({dev_str}) at {}", mount_root.display()),
            );

            let mut res = write_profiles_to_sd_root(&mount_root, profiles);
            if let Ok(ref summary) = res {
                sync_profiles_to_device_cache(serial, profiles);
                mark_profiles_sd_synced(serial, profiles);
                let short_fp =
                    &summary.validated_fingerprint[..8.min(summary.validated_fingerprint.len())];
                on_progress(
                    76,
                    "Step 4/5: Read-back validation passed! Flushing FAT buffers & unmounting...",
                    &format!(
                        "✓ Wrote {} updated file(s) ({} unchanged verified) & validated {} profile(s) on microSD [hash #{short_fp}]",
                        summary.files_written,
                        summary.files_verified_unchanged,
                        summary.profiles_count
                    ),
                );
            }

            // 3. Cleanly unmount and verify in /proc/mounts BEFORE resetting back to HID mode
            if let Err(unmount_err) = unmount_duckypad_sd_cleanly(&mount_root, real_dev.as_deref()) {
                if res.is_ok() {
                    res = Err(unmount_err);
                }
            } else if res.is_ok() {
                on_progress(
                    90,
                    "Step 5/5: Rebooting duckyPad Pro back to normal HID mode...",
                    "✓ FAT32 filesystem flushed, unmounted, and settled; rebooting duckyPad Pro...",
                );
            }
            res
        }
        Err(e) => Err(e),
    };

    // 4. Reset duckyPad Pro back to normal HID keypad mode (0x14, tx[3] = 0)
    for _ in 0..20 {
        let reset_ok = {
            let _guard = HID_IO_LOCK.lock().ok();
            if let Ok(dev) = open_duckypad_hid() {
                let mut tx = [0u8; 64];
                tx[0] = HID_REPORT_ID_OUT;
                tx[1] = 0x14;
                tx[2] = HID_CMD_SOFTWARE_RESET;
                tx[3] = 0;
                dev.write(&tx).is_ok()
            } else {
                false
            }
        };
        if reset_ok {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(180));
    }

    // 5. Wait for normal HID mode to re-enumerate and restore active profile
    std::thread::sleep(std::time::Duration::from_millis(1200));
    for _ in 0..15 {
        let ready = {
            let _guard = HID_IO_LOCK.lock().ok();
            open_duckypad_hid().is_ok()
        };
        if ready {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    drop(_msc_guard);
    if restore_profile > 0 {
        let _ = crate::kwin::auto_switcher::send_hid_switch_profile(restore_profile);
    }

    if let Ok(ref summary) = write_res {
        let short_fp = &summary.validated_fingerprint[..8.min(summary.validated_fingerprint.len())];
        on_progress(
            100,
            "Validated: All changes successfully saved to duckyPad!",
            &format!(
                "✓ duckyPad Pro reconnected in HID mode (Profile #{restore_profile}) — {} profile(s) verified [hash #{short_fp}]",
                summary.profiles_count
            ),
        );
    }

    write_res
}

pub fn reboot_duckypad() -> Result<(), String> {
    let _guard = HID_IO_LOCK
        .lock()
        .map_err(|_| "HID lock poisoned".to_string())?;
    let dev = open_duckypad_hid()?;
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x14;
    tx[2] = HID_CMD_SOFTWARE_RESET;
    tx[3] = 0;
    dev.write(&tx)
        .map_err(|e| format!("Failed sending software reset to duckyPad: {e}"))?;
    Ok(())
}

pub fn is_duckypad_sd_root(dir: &Path) -> bool {
    if dir.join("dpp_config.txt").exists()
        || dir.join("profile_info.txt").exists()
        || dir.join("dp_stats.txt").exists()
        || dir.join("keymaps").is_dir()
    {
        return true;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if parse_profile_folder_name(name).is_some() {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn parse_profile_folder_name(folder: &str) -> Option<(Option<u8>, String)> {
    // Support both duckyPad Pro `profile_<Name>` and duckyPad (2020) `profile<N>_<Name>`
    if let Some(name_str) = folder.strip_prefix("profile_") {
        let trimmed = name_str.trim();
        if !trimmed.is_empty() {
            return Some((None, trimmed.to_string()));
        }
    }
    let rest = folder.strip_prefix("profile")?;
    let (num_str, name_str) = rest.split_once('_')?;
    let number: u8 = num_str.parse().ok()?;
    if (1..=64).contains(&number) {
        Some((Some(number), name_str.to_string()))
    } else {
        None
    }
}

fn parse_indexed_directive<'a>(line: &'a str, prefix: &str) -> Option<(usize, &'a str)> {
    let rest = line.strip_prefix(prefix)?;
    let (idx_str, value) = rest.split_once(' ')?;
    let slot_idx: usize = idx_str.parse().ok()?;
    if (1..=MAX_INPUT_SLOTS).contains(&slot_idx) {
        Some((slot_idx, value))
    } else {
        None
    }
}

fn parse_rgb_triplet(s: &str) -> Option<[u8; 3]> {
    let mut parts = s.split_whitespace();
    let r: u8 = parts.next()?.parse().ok()?;
    let g: u8 = parts.next()?.parse().ok()?;
    let b: u8 = parts.next()?.parse().ok()?;
    Some([r, g, b])
}

#[derive(Debug, Clone)]
pub struct FirmwareUpdateCheckResult {
    pub is_pro: bool,
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub download_url: String,
    pub guide_url: String,
}

fn parse_semver(v: &str) -> Option<(u32, u32, u32)> {
    let clean = v.trim().trim_start_matches(['v', 'V']);
    let mut parts = clean.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next()?.parse().ok()?;
    let patch: u32 = parts.next()?.parse().ok()?;
    Some((major, minor, patch))
}

pub async fn check_latest_duckypad_firmware(
    is_pro: bool,
    current_version: &str,
) -> Result<FirmwareUpdateCheckResult, String> {
    let (api_url, guide_url) = if is_pro {
        (
            "https://api.github.com/repos/dekuNukem/duckyPad-Pro/contents/firmware?ref=master",
            "https://dekuNukem.github.io/duckyPad-Pro/doc/fw_update.html",
        )
    } else {
        (
            "https://api.github.com/repos/dekuNukem/duckyPad/contents/firmware?ref=master",
            "https://github.com/dekuNukem/duckyPad/blob/master/firmware_updates_and_version_history.md",
        )
    };

    let client = reqwest::Client::builder()
        .user_agent("DuckyDeck-Studio/0.1.0")
        .timeout(std::time::Duration::from_secs(6))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(api_url)
        .send()
        .await
        .map_err(|e| format!("Failed to reach GitHub firmware repository: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API returned status {}", resp.status()));
    }

    let entries: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid GitHub API response: {e}"))?;

    let arr = entries
        .as_array()
        .ok_or_else(|| "Unexpected GitHub firmware listing format".to_string())?;

    let mut best: Option<((u32, u32, u32), String, String)> = None;

    for item in arr {
        let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
        if item_type != "file" {
            continue;
        }
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let dl_url = item
            .get("download_url")
            .and_then(|v| v.as_str())
            .or_else(|| item.get("html_url").and_then(|v| v.as_str()))
            .unwrap_or(guide_url);

        let ver_opt = if is_pro {
            if name.starts_with("DPP_FW_") && name.ends_with(".bin") {
                name.strip_prefix("DPP_FW_")
                    .and_then(|s| s.strip_suffix(".bin"))
                    .and_then(|s| s.split('_').next())
            } else {
                None
            }
        } else if name.starts_with("duckypad_v") && name.ends_with(".dfu") {
            name.strip_prefix("duckypad_v")
                .and_then(|s| s.strip_suffix(".dfu"))
        } else {
            None
        };

        if let Some(ver_str) = ver_opt {
            if let Some(semver) = parse_semver(ver_str) {
                let replace = match &best {
                    Some((best_ver, _, _)) => semver > *best_ver,
                    None => true,
                };
                if replace {
                    best = Some((semver, ver_str.to_string(), dl_url.to_string()));
                }
            }
        }
    }

    let (remote_semver, latest_version, download_url) =
        best.ok_or_else(|| "No firmware image found in repository".to_string())?;

    let update_available = match parse_semver(current_version) {
        Some(curr_semver) => remote_semver > curr_semver,
        None => false,
    };

    Ok(FirmwareUpdateCheckResult {
        is_pro,
        current_version: current_version.to_string(),
        latest_version,
        update_available,
        download_url,
        guide_url: guide_url.to_string(),
    })
}

// ============================================================================
// Web API Key Action Builder, Hardware Trigger Poller (_GV31 + evdev F13-F24)
// ============================================================================

pub const WEB_API_META_PREFIX: &str = "// @WEB_API ";
pub const WEB_API_BLOCK_BEGIN: &str = "// --- BEGIN DUCKYDECK WEB API TRIGGER ---";
pub const WEB_API_BLOCK_END: &str = "// --- END DUCKYDECK WEB API TRIGGER ---";
pub const WEB_API_PGV_MAGIC: u32 = 0x444B_0000;

fn default_web_api_method() -> String {
    "POST".to_string()
}

fn default_web_api_oled() -> String {
    String::new()
}

fn default_web_api_led_feedback() -> bool {
    true
}

fn default_web_api_trigger_mode() -> String {
    "pgv".to_string()
}

fn default_web_api_fkey() -> String {
    "F13".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebApiConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_web_api_method")]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: String,
    #[serde(default)]
    pub body: String,
    #[serde(default = "default_web_api_oled")]
    pub oled_text: String,
    #[serde(default = "default_web_api_led_feedback")]
    pub led_feedback: bool,
    #[serde(default = "default_web_api_trigger_mode")]
    pub trigger_mode: String,
    #[serde(default = "default_web_api_fkey")]
    pub fkey: String,
}

impl Default for WebApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: default_web_api_method(),
            url: String::new(),
            headers: String::new(),
            body: String::new(),
            oled_text: String::new(),
            led_feedback: true,
            trigger_mode: default_web_api_trigger_mode(),
            fkey: default_web_api_fkey(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebApiExecutionResult {
    pub ok: bool,
    pub status_code: u16,
    pub elapsed_ms: u64,
    pub summary: String,
    pub response_preview: String,
}

pub fn default_fkey_for_slot(slot_num: usize) -> String {
    let idx = ((slot_num.saturating_sub(1)) % 12) + 13;
    format!("F{idx}")
}

pub fn extract_web_api_config(script: &str, slot_num: usize) -> WebApiConfig {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(WEB_API_META_PREFIX) {
            if let Ok(mut cfg) = serde_json::from_str::<WebApiConfig>(json_part.trim()) {
                cfg.method = cfg.method.trim().to_ascii_uppercase();
                if cfg.method.is_empty() {
                    cfg.method = "POST".to_string();
                }
                if cfg.trigger_mode != "fkey" {
                    cfg.trigger_mode = "pgv".to_string();
                }
                if cfg.fkey.trim().is_empty() {
                    cfg.fkey = default_fkey_for_slot(slot_num);
                }
                return cfg;
            }
        }
    }
    WebApiConfig {
        fkey: default_fkey_for_slot(slot_num),
        ..Default::default()
    }
}

pub fn has_web_api_config(script: &str) -> bool {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(WEB_API_META_PREFIX) {
            if let Ok(cfg) = serde_json::from_str::<WebApiConfig>(json_part.trim()) {
                if cfg.enabled && !cfg.url.trim().is_empty() {
                    return true;
                }
            }
        }
    }
    false
}

pub fn strip_web_api_from_script(script: &str) -> String {
    let mut out_lines = Vec::new();
    let mut inside_block = false;
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(WEB_API_META_PREFIX) {
            continue;
        }
        if trimmed == WEB_API_BLOCK_BEGIN {
            inside_block = true;
            continue;
        }
        if trimmed == WEB_API_BLOCK_END {
            inside_block = false;
            continue;
        }
        if !inside_block {
            out_lines.push(line);
        }
    }
    let joined = out_lines.join("\n");
    let clean = joined.trim_start_matches('\n');
    if clean.is_empty() {
        String::new()
    } else if clean.ends_with('\n') {
        clean.to_string()
    } else {
        format!("{clean}\n")
    }
}

pub fn encode_pgv31_trigger(profile_num: u8, slot_num: usize, is_release: bool) -> u32 {
    let rel_bit = if is_release { 1u32 << 15 } else { 0u32 };
    let prof_bits = ((profile_num as u32) & 0x7F) << 8;
    let slot_bits = (slot_num as u32) & 0xFF;
    WEB_API_PGV_MAGIC | rel_bit | prof_bits | slot_bits
}

pub fn decode_pgv31_trigger(raw_val: i32) -> Option<(u8, usize, bool)> {
    let uval = raw_val as u32;
    if (uval & 0xFFFF_0000) != WEB_API_PGV_MAGIC {
        return None;
    }
    let is_release = ((uval >> 15) & 1) != 0;
    let profile_num = ((uval >> 8) & 0x7F) as u8;
    let slot_num = (uval & 0xFF) as usize;
    if (1..=MAX_INPUT_SLOTS).contains(&slot_num) {
        Some((profile_num, slot_num, is_release))
    } else {
        None
    }
}

pub fn apply_web_api_config_to_script(
    existing_script: &str,
    cfg: &WebApiConfig,
    profile_num: u8,
    slot_num: usize,
    is_release: bool,
) -> String {
    let remainder = strip_web_api_from_script(existing_script);
    if !cfg.enabled {
        return remainder;
    }

    let mut normalized = cfg.clone();
    normalized.method = normalized.method.trim().to_ascii_uppercase();
    if normalized.method.is_empty() {
        normalized.method = "POST".to_string();
    }
    if normalized.trigger_mode != "fkey" {
        normalized.trigger_mode = "pgv".to_string();
    }
    let valid_fkey = {
        let up = normalized.fkey.trim().to_ascii_uppercase();
        if matches!(
            up.as_str(),
            "F13" | "F14" | "F15" | "F16" | "F17" | "F18" | "F19" | "F20" | "F21" | "F22" | "F23" | "F24"
        ) {
            up
        } else {
            default_fkey_for_slot(slot_num)
        }
    };
    normalized.fkey = valid_fkey.clone();

    let meta_json = serde_json::to_string(&normalized).unwrap_or_else(|_| "{}".to_string());
    let trigger_code = encode_pgv31_trigger(profile_num, slot_num, is_release);

    let mut out = String::new();
    out.push_str(&format!("{WEB_API_META_PREFIX}{meta_json}\n"));
    out.push_str(&format!("{WEB_API_BLOCK_BEGIN}\n"));

    let oled_clean = normalized.oled_text.trim();
    if !oled_clean.is_empty() {
        out.push_str("OLED_CLEAR\n");
        out.push_str(&format!("OLED_CPRINT {oled_clean}\n"));
        out.push_str("OLED_UPDATE\n");
    }

    if normalized.trigger_mode == "fkey" {
        out.push_str(&format!("CONTROL ALT SHIFT {valid_fkey}\n"));
    } else {
        out.push_str(&format!("$_GV31 = 0x{trigger_code:08X}\n"));
    }

    out.push_str(&format!("{WEB_API_BLOCK_END}\n"));
    if !remainder.trim().is_empty() {
        out.push_str(&remainder);
    }
    out
}

pub fn poll_and_clear_pgv31_trigger() -> Option<(u8, usize, bool)> {
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return None;
    }
    let _guard = HID_IO_LOCK.try_lock().ok()?;
    let device = open_duckypad_hid().ok()?;

    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x3F;
    tx[2] = HID_CMD_DUMP_PGV;
    tx[3] = 30; // Dump _GV30 and _GV31 in a single packet

    if device.write(&tx).is_err() {
        return None;
    }

    let mut rx = [0u8; 64];
    let bytes_read = device.read_timeout(&mut rx, 120).ok()?;
    if bytes_read < 12 || rx[2] != 0 {
        return None;
    }

    // _GV30 is at rx[4..8], _GV31 is at rx[8..12]
    let raw_gv31 = i32::from_le_bytes([rx[8], rx[9], rx[10], rx[11]]);
    if let Some(decoded) = decode_pgv31_trigger(raw_gv31) {
        let _ = write_single_pgv(&device, 31, 0);
        return Some(decoded);
    }
    None
}

pub fn set_single_key_led_hid(slot_num: usize, rgb: [u8; 3]) -> bool {
    if !(1..=ONBOARD_KEY_COUNT).contains(&slot_num) {
        return false;
    }
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    let Ok(_guard) = HID_IO_LOCK.lock() else {
        return false;
    };
    let Ok(device) = open_duckypad_hid() else {
        return false;
    };
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x33;
    tx[2] = HID_CMD_SET_LED_SINGLE;
    tx[3] = (slot_num - 1) as u8;
    tx[4] = rgb[0];
    tx[5] = rgb[1];
    tx[6] = rgb[2];
    let ok = device.write(&tx).is_ok();
    if ok {
        let mut rx = [0u8; 64];
        let _ = device.read_timeout(&mut rx, 120);
    }
    ok
}

pub async fn flash_slot_led_feedback(slot_num: usize, success: bool, restore_rgb: [u8; 3]) {
    if !(1..=ONBOARD_KEY_COUNT).contains(&slot_num) {
        return;
    }
    if success {
        set_single_key_led_hid(slot_num, [0, 255, 80]);
        tokio::time::sleep(std::time::Duration::from_millis(650)).await;
        set_single_key_led_hid(slot_num, restore_rgb);
    } else {
        set_single_key_led_hid(slot_num, [255, 30, 30]);
        tokio::time::sleep(std::time::Duration::from_millis(260)).await;
        set_single_key_led_hid(slot_num, [40, 0, 0]);
        tokio::time::sleep(std::time::Duration::from_millis(120)).await;
        set_single_key_led_hid(slot_num, [255, 30, 30]);
        tokio::time::sleep(std::time::Duration::from_millis(320)).await;
        set_single_key_led_hid(slot_num, restore_rgb);
    }
}

pub async fn execute_web_api_request(cfg: &WebApiConfig) -> WebApiExecutionResult {
    let url = cfg.url.trim();
    if url.is_empty() {
        return WebApiExecutionResult {
            ok: false,
            status_code: 0,
            elapsed_ms: 0,
            summary: "Error: URL is empty".to_string(),
            response_preview: String::new(),
        };
    }

    let start = std::time::Instant::now();
    let client = match reqwest::Client::builder()
        .user_agent("DuckyDeck-Studio/0.1.0")
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return WebApiExecutionResult {
                ok: false,
                status_code: 0,
                elapsed_ms: 0,
                summary: format!("HTTP client init failed: {e}"),
                response_preview: String::new(),
            };
        }
    };

    let method_str = cfg.method.trim().to_ascii_uppercase();
    let method = match method_str.as_str() {
        "GET" => reqwest::Method::GET,
        "PUT" => reqwest::Method::PUT,
        "PATCH" => reqwest::Method::PATCH,
        "DELETE" => reqwest::Method::DELETE,
        "HEAD" => reqwest::Method::HEAD,
        _ => reqwest::Method::POST,
    };

    let mut req = client.request(method.clone(), url);
    let mut has_content_type = false;

    for raw_line in cfg.headers.split(['\n', ';']) {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            let val = v.trim();
            if key.eq_ignore_ascii_case("content-type") {
                has_content_type = true;
            }
            if !key.is_empty() {
                req = req.header(key, val);
            }
        } else if line.to_ascii_lowercase().starts_with("bearer ") {
            req = req.header("Authorization", line);
        }
    }

    let body_trimmed = cfg.body.trim();
    if !body_trimmed.is_empty() && method != reqwest::Method::GET && method != reqwest::Method::HEAD
    {
        if !has_content_type
            && (body_trimmed.starts_with('{') || body_trimmed.starts_with('['))
        {
            req = req.header("Content-Type", "application/json");
        }
        req = req.body(cfg.body.clone());
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status();
            let code = status.as_u16();
            let ok = status.is_success();
            let body_text = resp.text().await.unwrap_or_default();
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let single_line: String = body_text
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            let preview: String = if single_line.chars().count() > 180 {
                let trunc: String = single_line.chars().take(180).collect();
                format!("{trunc}…")
            } else {
                single_line
            };
            let reason = status.canonical_reason().unwrap_or("");
            let summary = if preview.is_empty() {
                format!("HTTP {code} {reason} ({elapsed_ms} ms)").trim().to_string()
            } else {
                format!("HTTP {code} {reason} ({elapsed_ms} ms) — {preview}")
            };
            WebApiExecutionResult {
                ok,
                status_code: code,
                elapsed_ms,
                summary,
                response_preview: preview,
            }
        }
        Err(e) => {
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let err_msg = if e.is_timeout() {
                format!("Request timed out after {elapsed_ms} ms")
            } else if e.is_connect() {
                format!("Connection error: {e}")
            } else {
                format!("Request error: {e}")
            };
            WebApiExecutionResult {
                ok: false,
                status_code: 0,
                elapsed_ms,
                summary: err_msg.clone(),
                response_preview: err_msg,
            }
        }
    }
}

pub struct DuckypadEvdevListener {
    files: Vec<(PathBuf, fs::File)>,
    last_scan: std::time::Instant,
}

impl Default for DuckypadEvdevListener {
    fn default() -> Self {
        Self::new()
    }
}

impl DuckypadEvdevListener {
    pub fn new() -> Self {
        let mut s = Self {
            files: Vec::new(),
            last_scan: std::time::Instant::now() - std::time::Duration::from_secs(10),
        };
        s.refresh_nodes();
        s
    }

    pub fn refresh_nodes(&mut self) {
        use std::os::unix::fs::OpenOptionsExt;
        self.last_scan = std::time::Instant::now();
        self.files.retain(|(p, _)| p.exists());

        let Ok(entries) = fs::read_dir("/sys/class/input") else {
            return;
        };
        for entry in entries.flatten() {
            let fname = entry.file_name();
            let Some(name_str) = fname.to_str() else {
                continue;
            };
            if !name_str.starts_with("event") {
                continue;
            }
            let dev_path = Path::new("/dev/input").join(name_str);
            if self.files.iter().any(|(p, _)| p == &dev_path) {
                continue;
            }
            if let Ok(uevent) = fs::read_to_string(entry.path().join("device/uevent")) {
                let upper = uevent.to_ascii_uppercase();
                // Match USB VID 0483 and PID D11D / D11C
                if upper.contains("483/D11D")
                    || upper.contains("483/D11C")
                    || upper.contains("0483:D11D")
                    || upper.contains("0483:D11C")
                {
                    if let Ok(file) = fs::OpenOptions::new()
                        .read(true)
                        .custom_flags(0o4000) // O_NONBLOCK
                        .open(&dev_path)
                    {
                        self.files.push((dev_path, file));
                    }
                }
            }
        }
    }

    pub fn poll_fkey_triggers(&mut self) -> Vec<String> {
        use std::io::Read;
        if self.last_scan.elapsed() >= std::time::Duration::from_secs(3) {
            self.refresh_nodes();
        }
        let mut triggered = Vec::new();
        let mut buf = [0u8; 24]; // sizeof(struct input_event) on 64-bit Linux
        let mut drop_indices = Vec::new();

        for (idx, (_, file)) in self.files.iter_mut().enumerate() {
            loop {
                match file.read_exact(&mut buf) {
                    Ok(()) => {
                        let ev_type = u16::from_ne_bytes([buf[16], buf[17]]);
                        let ev_code = u16::from_ne_bytes([buf[18], buf[19]]);
                        let ev_value = i32::from_ne_bytes([buf[20], buf[21], buf[22], buf[23]]);
                        // EV_KEY == 1, value == 1 (key press), KEY_F13 (183)..=KEY_F24 (194)
                        if ev_type == 1 && ev_value == 1 && (183..=194).contains(&ev_code) {
                            let f_num = 13 + (ev_code - 183);
                            triggered.push(format!("F{f_num}"));
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(_) => {
                        drop_indices.push(idx);
                        break;
                    }
                }
            }
        }

        for idx in drop_indices.into_iter().rev() {
            self.files.remove(idx);
        }
        triggered
    }
}


