use super::{
    load_device_rules_for_serial, match_profile_with_rules, AutoSwitchRule, HidSwitchOutcome,
};
use crate::hardware::storage::{
    DuckyProfile, HID_CMD_GOTO_PROFILE_BY_NUM, HID_CMD_QUERY_INFO, HID_REPORT_ID_OUT,
};

#[derive(Debug, Clone)]
pub struct SecondaryDeviceRuntime {
    pub serial: String,
    pub pid: u16,
    pub hw_model: u8,
    pub model_name: String,
    pub firmware_version: String,
    pub disk_label: String,
    pub usb_connected: bool,
    pub active_profile: u8,
    pub profiles: Vec<DuckyProfile>,
    pub rules_enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
}

static SECONDARY_DEVICES_STATE: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<String, SecondaryDeviceRuntime>>,
> = std::sync::OnceLock::new();

pub fn get_secondary_devices_snapshot() -> Vec<SecondaryDeviceRuntime> {
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let Ok(guard) = mutex.lock() else {
        return Vec::new();
    };
    let mut list: Vec<SecondaryDeviceRuntime> = guard.values().cloned().collect();
    list.sort_by(|a, b| a.serial.cmp(&b.serial));
    list
}

pub fn get_secondary_device_runtime(serial: &str) -> Option<SecondaryDeviceRuntime> {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return None;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let guard = mutex.lock().ok()?;
    guard
        .values()
        .find(|d| crate::hardware::storage::serial_matches_target(&d.serial, &clean))
        .cloned()
}

pub fn update_secondary_device_profiles(serial: &str, profiles: Vec<DuckyProfile>) {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        if let Some(entry) = guard
            .values_mut()
            .find(|d| crate::hardware::storage::serial_matches_target(&d.serial, &clean))
        {
            entry.profiles = profiles;
        }
    }
}

pub fn update_secondary_device_runtime(runtime: SecondaryDeviceRuntime) {
    let clean = crate::hardware::storage::normalize_duckypad_serial(&runtime.serial);
    if clean.is_empty() {
        return;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        guard.insert(clean, runtime);
    }
}

pub fn update_secondary_device_active_profile(serial: &str, active_profile: u8) -> bool {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return false;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        if let Some(entry) = guard.get_mut(&clean) {
            if entry.active_profile != active_profile {
                entry.active_profile = active_profile;
                return true;
            }
        }
    }
    false
}

pub fn set_secondary_device_profile_rotation(serial: &str, degrees: u16) -> bool {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return false;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        if let Some(entry) = guard.get_mut(&clean) {
            let active_num = entry.active_profile;
            if let Some(prof) = entry.profiles.iter_mut().find(|p| p.number == active_num) {
                prof.set_oled_rotation_degrees(degrees as i32);
                let profiles_clone = entry.profiles.clone();
                crate::hardware::profile::sync_profiles_to_device_cache(&clean, &profiles_clone);
                return true;
            }
        }
    }
    false
}

pub fn sync_secondary_connected_devices(
    connected: &[crate::hardware::storage::ConnectedDuckyPadInfo],
    selected_serial: Option<&str>,
) {
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let Ok(mut guard) = mutex.lock() else {
        return;
    };
    let selected_norm = selected_serial
        .map(crate::hardware::storage::normalize_duckypad_serial)
        .filter(|s| !s.is_empty());

    for entry in guard.values_mut() {
        entry.usb_connected = false;
    }

    for dev in connected {
        let sn = crate::hardware::storage::normalize_duckypad_serial(&dev.serial);
        if sn.is_empty() {
            continue;
        }
        if let Some(ref sel) = selected_norm {
            if crate::hardware::storage::serial_matches_target(&sn, sel) {
                if let Some(existing) = guard.get_mut(&sn) {
                    existing.usb_connected = true;
                    existing.pid = dev.pid;
                    existing.hw_model = dev.hw_model;
                    existing.model_name = dev.model_name().to_string();
                    existing.firmware_version = dev.fw_version.clone();
                    existing.disk_label = dev.disk_label.clone();
                }
                continue;
            }
        }

        if let Some(existing) = guard.get_mut(&sn) {
            existing.usb_connected = true;
            existing.pid = dev.pid;
            existing.hw_model = dev.hw_model;
            existing.model_name = dev.model_name().to_string();
            existing.firmware_version = dev.fw_version.clone();
            existing.disk_label = dev.disk_label.clone();
            if dev.active_profile > 0 {
                existing.active_profile = dev.active_profile;
            }
            if existing.profiles.len() <= 1 {
                if let Some(cached) =
                    crate::hardware::profile::load_cached_profiles_for_serial_for_model(
                        &sn,
                        Some(dev.hw_model),
                    )
                {
                    if cached.len() > existing.profiles.len() {
                        existing.profiles = cached;
                    }
                }
            }
            continue;
        }

        let profiles = crate::hardware::profile::load_profiles_for_connected_device(dev.pid, &sn)
            .unwrap_or_else(|| {
                crate::hardware::profile::create_default_profiles_for_device(dev.pid, &sn)
            });
        let sd_root = crate::hardware::storage::find_mounted_duckypad_sd_for_device(
            Some(&sn),
            if dev.disk_label.is_empty() {
                None
            } else {
                Some(&dev.disk_label)
            },
            Some(dev.hw_model),
        );
        let (rules_enabled, default_profile, rules) =
            load_device_rules_for_serial(&sn, &profiles, sd_root.as_deref());
        let active_prof = if dev.active_profile > 0 {
            dev.active_profile
        } else {
            profiles.first().map(|p| p.number).unwrap_or(1)
        };
        guard.insert(
            sn.clone(),
            SecondaryDeviceRuntime {
                serial: sn,
                pid: dev.pid,
                hw_model: dev.hw_model,
                model_name: dev.model_name().to_string(),
                firmware_version: dev.fw_version.clone(),
                disk_label: dev.disk_label.clone(),
                usb_connected: true,
                active_profile: active_prof,
                profiles,
                rules_enabled,
                default_profile,
                rules,
            },
        );
    }
}

pub fn dispatch_window_switch_to_secondary_devices(
    effective_class: &str,
    resolved_caption: &str,
    paused_for_editing: bool,
) {
    if paused_for_editing {
        return;
    }
    let selected_norm = crate::hardware::storage::get_selected_duckypad_serial();
    let targets_to_switch: Vec<(String, u8)> = {
        let mutex = SECONDARY_DEVICES_STATE
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
        let Ok(guard) = mutex.lock() else {
            return;
        };
        let mut out = Vec::new();
        for dev in guard.values() {
            if !dev.usb_connected || !dev.rules_enabled {
                continue;
            }
            if let Some(ref sel) = selected_norm {
                if crate::hardware::storage::serial_matches_target(&dev.serial, sel) {
                    continue;
                }
            }
            if let Some(target) = match_profile_with_rules(
                &dev.rules,
                dev.default_profile,
                effective_class,
                resolved_caption,
            ) {
                if dev.profiles.iter().any(|p| p.number == target) && dev.active_profile != target {
                    out.push((dev.serial.clone(), target));
                }
            }
        }
        out
    };

    for (serial, target) in targets_to_switch {
        if send_hid_switch_profile_checked_for_serial(Some(&serial), target)
            == HidSwitchOutcome::Switched
        {
            update_secondary_device_active_profile(&serial, target);
        }
    }
}

pub static LAST_SW_PROFILE_SWITCH_MS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub static LAST_SW_PROFILE_TARGET: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);
pub static LAST_OBSERVED_HW_PROFILE_BEFORE_SWITCH: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);

pub fn current_epoch_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn send_hid_switch_profile(profile_number: u8) -> bool {
    send_hid_switch_profile_checked(profile_number) == HidSwitchOutcome::Switched
}

pub fn send_hid_switch_profile_for_serial(target_serial: Option<&str>, profile_number: u8) -> bool {
    send_hid_switch_profile_checked_for_serial(target_serial, profile_number)
        == HidSwitchOutcome::Switched
}

pub fn send_hid_switch_profile_checked(profile_number: u8) -> HidSwitchOutcome {
    let selected = crate::hardware::storage::get_selected_duckypad_serial();
    send_hid_switch_profile_checked_for_serial(selected.as_deref(), profile_number)
}

pub fn send_hid_switch_profile_checked_for_serial(
    target_serial: Option<&str>,
    profile_number: u8,
) -> HidSwitchOutcome {
    if crate::hardware::storage::MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return HidSwitchOutcome::DeviceUnavailable;
    }
    let clamped = profile_number.clamp(1, 64);
    let is_selected_target = match (
        target_serial
            .map(crate::hardware::storage::normalize_duckypad_serial)
            .filter(|s| !s.is_empty()),
        crate::hardware::storage::get_selected_duckypad_serial(),
    ) {
        (Some(req), Some(sel)) => crate::hardware::storage::serial_matches_target(&req, &sel),
        (None, _) | (_, None) => true,
    };

    let _guard = crate::hardware::storage::HID_IO_LOCK.lock().ok();
    crate::hardware::storage::with_cached_duckypad_hid_for_serial(target_serial, |device| {
        // Query current hardware profile first so we don't re-flash the OLED if already on `clamped`,
        // while still switching if the user manually changed profiles on the physical device.
        let mut qtx = [0u8; 64];
        qtx[0] = HID_REPORT_ID_OUT;
        qtx[1] = 0x76;
        qtx[2] = HID_CMD_QUERY_INFO;
        if device.write(&qtx).is_ok() {
            let mut qrx = [0u8; 64];
            if let Ok(n) = crate::hardware::storage::read_hid_report4(device, &mut qrx, 200) {
                if n >= 12 && qrx[2] == 0 && qrx[6] != 0 {
                    if is_selected_target && qrx[11] >= 1 {
                        LAST_OBSERVED_HW_PROFILE_BEFORE_SWITCH
                            .store(qrx[11], std::sync::atomic::Ordering::SeqCst);
                    }
                    if qrx[11] == clamped {
                        if is_selected_target {
                            LAST_SW_PROFILE_TARGET
                                .store(clamped, std::sync::atomic::Ordering::SeqCst);
                            LAST_SW_PROFILE_SWITCH_MS
                                .store(current_epoch_ms(), std::sync::atomic::Ordering::SeqCst);
                        }
                        return Ok(HidSwitchOutcome::Switched);
                    }
                }
            }
        }

        for attempt in 0..6u8 {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x77u8.wrapping_add(attempt);
            tx[2] = HID_CMD_GOTO_PROFILE_BY_NUM;
            tx[3] = clamped;

            if device.write(&tx).is_ok() {
                let mut rx = [0u8; 64];
                if let Ok(n) = crate::hardware::storage::read_hid_report4(device, &mut rx, 500) {
                    if n >= 3 && rx[2] == 2 {
                        // HID_RESPONSE_BUSY: wait briefly and retry
                        std::thread::sleep(std::time::Duration::from_millis(75));
                        continue;
                    }
                    if n >= 3 && rx[2] != 0 {
                        // HID_RESPONSE_INVALID_ARG (3) / HID_RESPONSE_NO_PROFILE (4) / HID_RESPONSE_ERROR (1):
                        // Profile does not exist on the duckyPad SD card!
                        return Ok(HidSwitchOutcome::NotLoadedOnHardware);
                    }
                }
                if is_selected_target {
                    LAST_SW_PROFILE_TARGET.store(clamped, std::sync::atomic::Ordering::SeqCst);
                    LAST_SW_PROFILE_SWITCH_MS
                        .store(current_epoch_ms(), std::sync::atomic::Ordering::SeqCst);
                }
                return Ok(HidSwitchOutcome::Switched);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Err("Failed switching profile over HID".to_string())
    })
    .unwrap_or(HidSwitchOutcome::DeviceUnavailable)
}
