use hidapi::{HidApi, HidDevice};
use std::fs;
use std::path::Path;

pub use crate::actions::app_launch::*;
pub use crate::actions::desktop_apps::*;
pub use crate::actions::web_api::*;
pub use crate::hardware::firmware::*;
pub use crate::hardware::msc::*;
pub use crate::hardware::profile::*;

pub use hidapi::HidDevice as DuckyHidDevice;

pub const DUCKYPAD_VID: u16 = 0x0483;
pub const PID_DUCKYPAD_PRO: u16 = 0xd11d;
pub const PID_DUCKYPAD_2020: u16 = 0xd11c;

pub const DP_MODEL_OG_DUCKYPAD: u8 = 20;
pub const DP_MODEL_DUCKYPAD_PRO: u8 = 24;

pub const HID_REPORT_ID_OUT: u8 = 0x05;
pub const HID_STATUS_ERROR: u8 = 1;
pub const HID_STATUS_EOF: u8 = 4;

pub const HID_CMD_QUERY_INFO: u8 = 0x00;
pub const HID_CMD_GOTO_PROFILE_BY_NUM: u8 = 0x01;
pub const HID_CMD_SET_LED_SINGLE: u8 = 0x04;
pub const HID_CMD_READ_FILE: u8 = 0x0B;
pub const HID_CMD_OPEN_FILE_FOR_WRITING: u8 = 0x0E;
pub const HID_CMD_WRITE_FILE: u8 = 0x0F;
pub const HID_CMD_CLOSE_FILE: u8 = 0x10;
pub const HID_CMD_DELETE_FILE: u8 = 0x11;
pub const HID_CMD_CREATE_DIR: u8 = 0x12;
pub const HID_CMD_DELETE_DIR: u8 = 0x13;
pub const HID_CMD_SOFTWARE_RESET: u8 = 0x14;
pub const HID_CMD_GOTO_PROFILE_BY_NAME: u8 = 0x17;
pub const HID_CMD_DUMP_PGV: u8 = 0x18;
pub const HID_CMD_WRITE_PGV: u8 = 0x19;
pub const HID_CMD_DUMP_SD: u8 = 0x20;
pub const HID_CMD_OPEN_FILE_FOR_READING: u8 = 0x21;

pub const HID_READ_FILE_PATH_SIZE_MAX: usize = 55;

pub const PGV_COUNT: usize = 32;
pub const MAX_INPUT_SLOTS: usize = 58;
pub const ONBOARD_KEY_COUNT_2020: usize = 15;
pub const ONBOARD_KEY_COUNT_PRO: usize = 20;
pub const ONBOARD_KEY_COUNT: usize = 20;

pub const fn onboard_key_count_for_model(hw_model: u8) -> usize {
    if hw_model == DP_MODEL_OG_DUCKYPAD {
        ONBOARD_KEY_COUNT_2020
    } else {
        ONBOARD_KEY_COUNT_PRO
    }
}

pub const fn onboard_key_count_for_pid(pid: u16) -> usize {
    if pid == PID_DUCKYPAD_2020 {
        ONBOARD_KEY_COUNT_2020
    } else {
        ONBOARD_KEY_COUNT_PRO
    }
}

pub const fn model_name_for_hw_model(hw_model: u8) -> &'static str {
    if hw_model == DP_MODEL_OG_DUCKYPAD {
        "duckyPad (2020)"
    } else {
        "duckyPad Pro"
    }
}

pub const fn model_name_for_pid(pid: u16) -> &'static str {
    if pid == PID_DUCKYPAD_2020 {
        "duckyPad (2020)"
    } else {
        "duckyPad Pro"
    }
}

pub fn normalize_duckypad_serial(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed == "-" {
        return String::new();
    }
    let after_prefix = if let Some((prefix, rest)) = trimmed.split_once('_') {
        let up_pre = prefix.to_ascii_uppercase();
        if up_pre == "DP24" || up_pre == "DP20" || up_pre == "DUCKYPAD" {
            rest.trim()
        } else {
            trimmed
        }
    } else {
        trimmed
    };
    let clean: String = after_prefix
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>()
        .to_ascii_uppercase();
    if clean.is_empty() || clean == "-" {
        String::new()
    } else {
        clean
    }
}

pub fn format_disk_label_for_device(hw_model: u8, serial: &str) -> String {
    let model_num = if hw_model == DP_MODEL_OG_DUCKYPAD {
        20u8
    } else {
        24u8
    };
    let clean = normalize_duckypad_serial(serial);
    if clean.len() >= 4 {
        let suffix = &clean[clean.len() - 4..];
        return format!("DP{model_num}_{suffix}");
    }
    String::new()
}

pub static HID_IO_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
pub static MSC_SYNC_IN_PROGRESS: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub static LAST_EVDEV_KEY_ACTIVITY_MS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub static LAST_EVDEV_KEYDOWN_MS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub static EVDEV_KEYS_HELD_COUNT: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);
pub static ACTIVE_PROFILE_HAS_PGV31_TRIGGER: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(true);

static SELECTED_DUCKYPAD_SERIAL: std::sync::OnceLock<std::sync::Mutex<Option<String>>> =
    std::sync::OnceLock::new();

pub fn get_selected_duckypad_serial() -> Option<String> {
    let mutex = SELECTED_DUCKYPAD_SERIAL.get_or_init(|| std::sync::Mutex::new(None));
    mutex.lock().ok().and_then(|g| g.clone())
}

pub fn set_selected_duckypad_serial(serial: Option<&str>) {
    let normalized = serial
        .map(normalize_duckypad_serial)
        .filter(|s| !s.is_empty());
    let mutex = SELECTED_DUCKYPAD_SERIAL.get_or_init(|| std::sync::Mutex::new(None));
    let mut changed = false;
    if let Ok(mut guard) = mutex.lock() {
        if *guard != normalized {
            *guard = normalized;
            changed = true;
        }
    }
    if changed {
        invalidate_cached_hid_device();
    }
}

static CACHED_HID_DEVICES: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<String, HidDevice>>,
> = std::sync::OnceLock::new();

pub fn invalidate_cached_hid_device() {
    crate::actions::web_api::invalidate_hardware_led_cache();
    if let Some(lock) = CACHED_HID_DEVICES.get() {
        if let Ok(mut guard) = lock.lock() {
            guard.clear();
        }
    }
}

pub fn invalidate_cached_hid_device_for_serial(serial: Option<&str>) {
    let key = serial
        .map(normalize_duckypad_serial)
        .filter(|s| !s.is_empty())
        .unwrap_or_default();
    if key.is_empty() {
        invalidate_cached_hid_device();
        return;
    }
    if let Some(lock) = CACHED_HID_DEVICES.get() {
        if let Ok(mut guard) = lock.lock() {
            guard.remove(&key);
            guard.remove("");
        }
    }
}

pub fn drain_pending_hid_reports(device: &HidDevice) {
    let mut flush_buf = [0u8; 64];
    for _ in 0..32 {
        match device.read_timeout(&mut flush_buf, 0) {
            Ok(n) if n > 0 => continue,
            _ => break,
        }
    }
}

pub fn read_hid_report4(
    device: &HidDevice,
    rx: &mut [u8; 64],
    timeout_ms: i32,
) -> Result<usize, String> {
    let deadline =
        std::time::Instant::now() + std::time::Duration::from_millis(timeout_ms.max(1) as u64);
    loop {
        let rem_ms = deadline
            .saturating_duration_since(std::time::Instant::now())
            .as_millis() as i32;
        if rem_ms <= 0 {
            return Ok(0);
        }
        rx.fill(0);
        let n = device
            .read_timeout(rx, rem_ms.max(1))
            .map_err(|e| e.to_string())?;
        if n == 0 {
            return Ok(0);
        }
        if rx[0] == 4 || rx[0] == 0 {
            return Ok(n);
        }
    }
}

pub fn with_cached_duckypad_hid_for_serial<T, F>(
    target_serial: Option<&str>,
    f: F,
) -> Result<T, String>
where
    F: FnOnce(&HidDevice) -> Result<T, String>,
{
    let effective_serial = target_serial
        .map(normalize_duckypad_serial)
        .filter(|s| !s.is_empty())
        .or_else(get_selected_duckypad_serial);
    let cache_key = effective_serial.clone().unwrap_or_default();
    let mutex =
        CACHED_HID_DEVICES.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let mut guard = mutex
        .lock()
        .map_err(|_| "HID cache lock poisoned".to_string())?;
    if !guard.contains_key(&cache_key) {
        let opened = match effective_serial.as_deref() {
            Some(sel) => open_duckypad_hid_for_serial(Some(sel))?,
            None => open_duckypad_hid()?,
        };
        guard.insert(cache_key.clone(), opened);
    }
    let res = {
        let dev = guard.get(&cache_key).unwrap();
        drain_pending_hid_reports(dev);
        f(dev)
    };
    if res.is_err() {
        guard.remove(&cache_key);
    }
    res
}

pub fn with_cached_duckypad_hid<T, F>(f: F) -> Result<T, String>
where
    F: FnOnce(&HidDevice) -> Result<T, String>,
{
    let target_serial = get_selected_duckypad_serial();
    with_cached_duckypad_hid_for_serial(target_serial.as_deref(), f)
}

pub fn is_duckypad_key_activity_recent(cooldown_ms: u64) -> bool {
    let now = crate::kwin::auto_switcher::current_epoch_ms();
    let held = EVDEV_KEYS_HELD_COUNT.load(std::sync::atomic::Ordering::SeqCst);
    if held > 0 {
        let last_down = LAST_EVDEV_KEYDOWN_MS.load(std::sync::atomic::Ordering::SeqCst);
        if now.saturating_sub(last_down) < 5000 {
            return true;
        }
        EVDEV_KEYS_HELD_COUNT.store(0, std::sync::atomic::Ordering::SeqCst);
    }
    let last = LAST_EVDEV_KEY_ACTIVITY_MS.load(std::sync::atomic::Ordering::SeqCst);
    if last == 0 {
        return false;
    }
    now.saturating_sub(last) < cooldown_ms
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConnectedDuckyPadInfo {
    pub pid: u16,
    pub hw_model: u8,
    pub fw_version: String,
    pub serial: String,
    pub disk_label: String,
    pub active_profile: u8,
    #[serde(default)]
    pub hid_path: String,
}

impl ConnectedDuckyPadInfo {
    pub fn model_name(&self) -> &'static str {
        if self.hw_model == DP_MODEL_OG_DUCKYPAD || self.pid == PID_DUCKYPAD_2020 {
            "duckyPad (2020)"
        } else {
            "duckyPad Pro"
        }
    }

    pub fn is_duckypad_2020(&self) -> bool {
        self.hw_model == DP_MODEL_OG_DUCKYPAD || self.pid == PID_DUCKYPAD_2020
    }

    pub fn onboard_key_count(&self) -> usize {
        if self.is_duckypad_2020() {
            ONBOARD_KEY_COUNT_2020
        } else {
            ONBOARD_KEY_COUNT_PRO
        }
    }
}

pub fn parse_hid_query_info_response(
    rx: &[u8],
    len: usize,
    fallback_pid: u16,
    hid_path: &str,
) -> Option<ConnectedDuckyPadInfo> {
    if len < 12 || rx[2] != 0 || rx[6] == 0 {
        return None;
    }
    let hw_model = rx[6];
    let pid = if hw_model == DP_MODEL_OG_DUCKYPAD {
        PID_DUCKYPAD_2020
    } else if hw_model == DP_MODEL_DUCKYPAD_PRO {
        PID_DUCKYPAD_PRO
    } else {
        fallback_pid
    };
    let raw_ver = (rx[3], rx[4], rx[5]);
    let has_dds_sig = len >= 23 && &rx[20..23] == b"DDS";
    let has_rtc_payload = len >= 20 && rx[16..20].iter().any(|&b| b != 0);
    let fw_version = if hw_model == DP_MODEL_DUCKYPAD_PRO
        && (raw_ver == (3, 4, 0) || (raw_ver == (1, 0, 0) && (has_dds_sig || has_rtc_payload)))
    {
        "1.0.0".to_string()
    } else if (hw_model == DP_MODEL_DUCKYPAD_PRO || hw_model == DP_MODEL_OG_DUCKYPAD)
        && (has_dds_sig || (raw_ver == (1, 0, 1) && has_rtc_payload))
    {
        format!("{}.{}.{}", rx[3], rx[4], rx[5])
    } else if hw_model == DP_MODEL_DUCKYPAD_PRO && raw_ver.0 == 1 && raw_ver.1 == 0 {
        format!("{}.{}.{} (Official)", rx[3], rx[4], rx[5])
    } else {
        format!("{}.{}.{}", rx[3], rx[4], rx[5])
    };
    let serial = format!("{:02X}{:02X}{:02X}{:02X}", rx[7], rx[8], rx[9], rx[10]);
    let disk_label = format!("DP{}_{:02X}{:02X}", hw_model, rx[9], rx[10]);
    let active_profile = rx[11].max(1);
    Some(ConnectedDuckyPadInfo {
        pid,
        hw_model,
        fw_version,
        serial,
        disk_label,
        active_profile,
        hid_path: hid_path.to_string(),
    })
}

pub fn reverse_hex_byte_pairs_4b(serial: &str) -> Option<String> {
    let bytes = serial.as_bytes();
    if bytes.len() == 8 && bytes.iter().all(|b| b.is_ascii_hexdigit()) {
        let mut out = String::with_capacity(8);
        out.push_str(&serial[6..8]);
        out.push_str(&serial[4..6]);
        out.push_str(&serial[2..4]);
        out.push_str(&serial[0..2]);
        Some(out)
    } else {
        None
    }
}

pub fn serial_matches_target(candidate_serial_or_uniq: &str, target_serial: &str) -> bool {
    let target = normalize_duckypad_serial(target_serial);
    let cand = normalize_duckypad_serial(candidate_serial_or_uniq);
    if target.is_empty() || cand.is_empty() {
        return false;
    }
    if cand == target {
        return true;
    }
    if let Some(rev_cand) = reverse_hex_byte_pairs_4b(&cand) {
        if rev_cand == target {
            return true;
        }
        if target.len() >= 4 && (rev_cand.ends_with(&target) || target.ends_with(&rev_cand)) {
            return true;
        }
    }
    if let Some(rev_target) = reverse_hex_byte_pairs_4b(&target) {
        if cand == rev_target {
            return true;
        }
        if cand.len() >= 4 && (cand.ends_with(&rev_target) || rev_target.ends_with(&cand)) {
            return true;
        }
    }
    if cand.len() >= 4 && target.len() >= 4 && (cand.ends_with(&target) || target.ends_with(&cand))
    {
        return true;
    }
    false
}

pub fn find_all_duckypad_hidraw_nodes() -> Vec<(std::path::PathBuf, u16, Option<String>)> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir("/sys/class/hidraw") else {
        return out;
    };
    for entry in entries.flatten() {
        if let Ok(uevent) = fs::read_to_string(entry.path().join("device/uevent")) {
            let upper = uevent.to_ascii_uppercase();
            let pid = if upper.contains("00000483:0000D11D") {
                Some(PID_DUCKYPAD_PRO)
            } else if upper.contains("00000483:0000D11C") {
                Some(PID_DUCKYPAD_2020)
            } else {
                None
            };
            if let Some(pid) = pid {
                let dev_node = Path::new("/dev").join(entry.file_name());
                if dev_node.exists() {
                    let uniq = uevent
                        .lines()
                        .find_map(|l| l.strip_prefix("HID_UNIQ="))
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .or_else(|| {
                            for rel in ["device/../../serial", "device/../../../serial"] {
                                if let Ok(s) = fs::read_to_string(entry.path().join(rel)) {
                                    let trimmed = s.trim().to_string();
                                    if !trimmed.is_empty() {
                                        return Some(trimmed);
                                    }
                                }
                            }
                            None
                        });
                    out.push((dev_node, pid, uniq));
                }
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

pub fn find_duckypad_hidraw_node_for_serial(
    target_serial: Option<&str>,
) -> Option<(std::path::PathBuf, u16, Option<String>)> {
    let nodes = find_all_duckypad_hidraw_nodes();
    if nodes.is_empty() {
        return None;
    }
    if let Some(req) = target_serial
        .map(normalize_duckypad_serial)
        .filter(|s| !s.is_empty())
    {
        if let Some(matched) = nodes.iter().find(|(_, _, uniq)| {
            uniq.as_deref()
                .map(|u| serial_matches_target(u, &req))
                .unwrap_or(false)
        }) {
            return Some(matched.clone());
        }
        // If all nodes have explicit HID_UNIQ values and none matched `req`, do not return a wrong device
        if nodes.iter().all(|(_, _, uniq)| uniq.is_some()) {
            return None;
        }
    }
    nodes.into_iter().next()
}

pub fn find_duckypad_hidraw_node() -> Option<(std::path::PathBuf, u16, Option<String>)> {
    let selected = get_selected_duckypad_serial();
    find_duckypad_hidraw_node_for_serial(selected.as_deref())
        .or_else(|| find_duckypad_hidraw_node_for_serial(None))
}

fn query_info_from_open_hid_device(
    device: &HidDevice,
    pid: u16,
    sysfs_uniq: Option<String>,
    hid_path: &str,
) -> ConnectedDuckyPadInfo {
    let default_model = if pid == PID_DUCKYPAD_PRO {
        DP_MODEL_DUCKYPAD_PRO
    } else {
        DP_MODEL_OG_DUCKYPAD
    };
    let mut fallback_serial = "-".to_string();
    let mut fallback_disk_label = String::new();
    let serial_source = sysfs_uniq.or_else(|| device.get_serial_number_string().ok().flatten());
    if let Some(usb_serial) = serial_source {
        let norm = normalize_duckypad_serial(&usb_serial);
        if !norm.is_empty() {
            fallback_disk_label = format_disk_label_for_device(default_model, &norm);
            fallback_serial = norm;
        }
    }

    drain_pending_hid_reports(device);
    for attempt in 0..3u8 {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x01u8.wrapping_add(attempt);
        tx[2] = HID_CMD_QUERY_INFO;

        if device.write(&tx).is_ok() {
            let mut rx = [0u8; 64];
            if let Ok(len) = read_hid_report4(device, &mut rx, 350) {
                if len >= 12 && rx[2] == 2 {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    continue;
                }
                if let Some(info) = parse_hid_query_info_response(&rx, len, pid, hid_path) {
                    return info;
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(35));
    }

    ConnectedDuckyPadInfo {
        pid,
        hw_model: default_model,
        fw_version: "-".to_string(),
        serial: fallback_serial,
        disk_label: fallback_disk_label,
        active_profile: 1,
        hid_path: hid_path.to_string(),
    }
}

/// Fast, zero-HID-I/O count of connected duckyPad custom-HID interfaces.
pub fn count_connected_duckypad_hid_nodes() -> usize {
    #[cfg(target_os = "linux")]
    {
        let nodes = find_all_duckypad_hidraw_nodes();
        let mut unique_serials = std::collections::HashSet::new();
        let mut unnamed_count = 0usize;
        for (_path, _pid, uniq) in nodes {
            let clean = uniq
                .as_deref()
                .map(normalize_duckypad_serial)
                .unwrap_or_default();
            if !clean.is_empty() && clean != "-" {
                unique_serials.insert(clean);
            } else {
                unnamed_count += 1;
            }
        }
        unique_serials.len() + unnamed_count
    }
    #[cfg(not(target_os = "linux"))]
    {
        let Ok(api) = HidApi::new() else {
            return 0;
        };
        let mut unique_serials = std::collections::HashSet::new();
        let mut unnamed_count = 0usize;
        for dev_info in api.device_list() {
            if dev_info.vendor_id() != DUCKYPAD_VID {
                continue;
            }
            let pid = dev_info.product_id();
            if pid != PID_DUCKYPAD_PRO && pid != PID_DUCKYPAD_2020 {
                continue;
            }
            #[cfg(target_os = "windows")]
            {
                let usage = dev_info.usage();
                if usage != 0 && usage != 58 {
                    continue;
                }
            }
            let clean = dev_info
                .serial_number()
                .map(normalize_duckypad_serial)
                .unwrap_or_default();
            if !clean.is_empty() && clean != "-" {
                unique_serials.insert(clean);
            } else {
                unnamed_count += 1;
            }
        }
        unique_serials.len() + unnamed_count
    }
}

#[allow(deprecated)]
pub fn list_connected_duckypads() -> Vec<ConnectedDuckyPadInfo> {
    let _guard = HID_IO_LOCK.lock().ok();
    let mut results: Vec<ConnectedDuckyPadInfo> = Vec::new();
    let mut seen_paths: std::collections::HashSet<String> = std::collections::HashSet::new();

    for (dev_node, pid, uniq) in find_all_duckypad_hidraw_nodes() {
        let path_str = dev_node.to_string_lossy().to_string();
        if let Ok(c_path) = std::ffi::CString::new(path_str.as_bytes()) {
            if let Ok(api) = HidApi::new_without_enumerate() {
                if let Ok(dev) = api.open_path(c_path.as_c_str()) {
                    seen_paths.insert(path_str.clone());
                    let info = query_info_from_open_hid_device(&dev, pid, uniq, &path_str);
                    results.push(info);
                }
            }
        }
    }

    if results.is_empty() {
        if let Ok(api) = HidApi::new() {
            for dev_info in api.device_list() {
                if dev_info.vendor_id() != DUCKYPAD_VID {
                    continue;
                }
                let pid = dev_info.product_id();
                if pid != PID_DUCKYPAD_PRO && pid != PID_DUCKYPAD_2020 {
                    continue;
                }
                #[cfg(target_os = "windows")]
                {
                    let usage = dev_info.usage();
                    if usage != 0 && usage != 58 {
                        continue;
                    }
                }
                let path_cstr = dev_info.path();
                let path_str = path_cstr.to_string_lossy().to_string();
                if seen_paths.contains(&path_str) {
                    continue;
                }
                if let Ok(dev) = api.open_path(path_cstr) {
                    seen_paths.insert(path_str.clone());
                    let uniq = dev_info.serial_number().map(|s| s.to_string());
                    let info = query_info_from_open_hid_device(&dev, pid, uniq, &path_str);
                    results.push(info);
                }
            }
        }
    }

    // Deduplicate by serial number (preferring entries with valid firmware version)
    let mut deduped: Vec<ConnectedDuckyPadInfo> = Vec::new();
    for item in results {
        if item.serial != "-" && !item.serial.is_empty() {
            if let Some(existing) = deduped
                .iter_mut()
                .find(|e| e.serial.eq_ignore_ascii_case(&item.serial))
            {
                if existing.fw_version == "-" && item.fw_version != "-" {
                    *existing = item;
                }
                continue;
            }
        }
        deduped.push(item);
    }

    deduped.sort_by(|a, b| {
        a.serial
            .cmp(&b.serial)
            .then_with(|| a.pid.cmp(&b.pid))
            .then_with(|| a.hid_path.cmp(&b.hid_path))
    });
    deduped
}

#[allow(deprecated)]
pub fn open_duckypad_hid_for_serial(target_serial: Option<&str>) -> Result<HidDevice, String> {
    let req_serial = target_serial
        .map(normalize_duckypad_serial)
        .filter(|s| !s.is_empty());

    if let Some(ref req) = req_serial {
        // 1. Check Linux hidraw nodes matching HID_UNIQ first
        let nodes = find_all_duckypad_hidraw_nodes();
        for (dev_node, _pid, uniq) in &nodes {
            if uniq
                .as_deref()
                .map(|u| serial_matches_target(u, req))
                .unwrap_or(false)
            {
                if let Ok(c_path) = std::ffi::CString::new(dev_node.to_string_lossy().as_bytes()) {
                    if let Ok(api) = HidApi::new_without_enumerate() {
                        if let Ok(device) = api.open_path(c_path.as_c_str()) {
                            return Ok(device);
                        }
                    }
                }
            }
        }
        // 2. Probe any hidraw nodes without a conflicting HID_UNIQ
        for (dev_node, pid, uniq) in &nodes {
            if uniq
                .as_deref()
                .map(|u| !serial_matches_target(u, req))
                .unwrap_or(false)
            {
                continue;
            }
            let path_str = dev_node.to_string_lossy().to_string();
            if let Ok(c_path) = std::ffi::CString::new(path_str.as_bytes()) {
                if let Ok(api) = HidApi::new_without_enumerate() {
                    if let Ok(device) = api.open_path(c_path.as_c_str()) {
                        let info =
                            query_info_from_open_hid_device(&device, *pid, uniq.clone(), &path_str);
                        if serial_matches_target(&info.serial, req)
                            || (nodes.len() == 1 && (info.serial == "-" || info.serial.is_empty()))
                        {
                            return Ok(device);
                        }
                    }
                }
            }
        }
        // 3. Check hidapi enumeration matching serial_number() or queried hardware serial
        if let Ok(api) = HidApi::new() {
            for dev_info in api.device_list() {
                if dev_info.vendor_id() != DUCKYPAD_VID {
                    continue;
                }
                let pid = dev_info.product_id();
                if pid != PID_DUCKYPAD_PRO && pid != PID_DUCKYPAD_2020 {
                    continue;
                }
                #[cfg(target_os = "windows")]
                {
                    let usage = dev_info.usage();
                    if usage != 0 && usage != 58 {
                        continue;
                    }
                }
                if let Some(sn) = dev_info.serial_number() {
                    if serial_matches_target(sn, req) {
                        if let Ok(device) = api.open_path(dev_info.path()) {
                            return Ok(device);
                        }
                    }
                }
            }
            for dev_info in api.device_list() {
                if dev_info.vendor_id() != DUCKYPAD_VID {
                    continue;
                }
                let pid = dev_info.product_id();
                if pid != PID_DUCKYPAD_PRO && pid != PID_DUCKYPAD_2020 {
                    continue;
                }
                #[cfg(target_os = "windows")]
                {
                    let usage = dev_info.usage();
                    if usage != 0 && usage != 58 {
                        continue;
                    }
                }
                if let Ok(device) = api.open_path(dev_info.path()) {
                    let path_str = dev_info.path().to_string_lossy().to_string();
                    let info = query_info_from_open_hid_device(
                        &device,
                        pid,
                        dev_info.serial_number().map(|s| s.to_string()),
                        &path_str,
                    );
                    if serial_matches_target(&info.serial, req) {
                        return Ok(device);
                    }
                }
            }
        }
        return Err(format!(
            "Requested duckyPad (SN: {req}) is not currently connected on USB HID"
        ));
    }

    if let Some((dev_node, _, _)) = find_duckypad_hidraw_node_for_serial(None) {
        if let Ok(c_path) = std::ffi::CString::new(dev_node.to_string_lossy().as_bytes()) {
            if let Ok(api) = HidApi::new_without_enumerate() {
                if let Ok(device) = api.open_path(c_path.as_c_str()) {
                    return Ok(device);
                }
            }
        }
    }
    let api = HidApi::new().map_err(|e| format!("Failed to initialize hidapi: {e}"))?;
    for dev_info in api.device_list() {
        if dev_info.vendor_id() == DUCKYPAD_VID
            && (dev_info.product_id() == PID_DUCKYPAD_PRO
                || dev_info.product_id() == PID_DUCKYPAD_2020)
        {
            #[cfg(target_os = "windows")]
            {
                let usage = dev_info.usage();
                if usage != 0 && usage != 58 {
                    continue;
                }
            }
            if let Ok(device) = api.open_path(dev_info.path()) {
                return Ok(device);
            }
        }
    }
    for pid in [PID_DUCKYPAD_PRO, PID_DUCKYPAD_2020] {
        if let Ok(device) = api.open(DUCKYPAD_VID, pid) {
            return Ok(device);
        }
    }
    Err("No duckyPad Pro (0xd11d) or duckyPad (0xd11c) found on USB bus".into())
}

pub fn open_duckypad_hid() -> Result<HidDevice, String> {
    let selected = get_selected_duckypad_serial();
    if let Some(ref sel) = selected {
        return open_duckypad_hid_for_serial(Some(sel));
    }
    open_duckypad_hid_for_serial(None)
}

pub fn is_duckypad_hid_present() -> bool {
    if !find_all_duckypad_hidraw_nodes().is_empty() {
        return true;
    }
    #[cfg(target_os = "windows")]
    if let Ok(api) = HidApi::new() {
        if api.device_list().any(|d| {
            d.vendor_id() == DUCKYPAD_VID
                && (d.product_id() == PID_DUCKYPAD_PRO || d.product_id() == PID_DUCKYPAD_2020)
                && (d.usage() == 0 || d.usage() == 58)
        }) {
            return true;
        }
    }
    invalidate_cached_hid_device();
    false
}

pub fn query_connected_duckypad_info_for_serial(
    target_serial: Option<&str>,
) -> Result<ConnectedDuckyPadInfo, String> {
    let all = list_connected_duckypads();
    if all.is_empty() {
        return Err("No duckyPad Pro (0xd11d) or duckyPad 2020 (0xd11c) found".to_string());
    }
    if let Some(req) = target_serial
        .map(normalize_duckypad_serial)
        .filter(|s| !s.is_empty())
    {
        if let Some(matched) = all
            .iter()
            .find(|info| serial_matches_target(&info.serial, &req))
        {
            return Ok(matched.clone());
        }
    }
    Ok(all.into_iter().next().unwrap())
}

pub fn query_connected_duckypad_info() -> Result<ConnectedDuckyPadInfo, String> {
    let selected = get_selected_duckypad_serial();
    query_connected_duckypad_info_for_serial(selected.as_deref())
}

pub fn poll_hardware_active_profile_for_serial(target_serial: Option<&str>) -> Option<u8> {
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst)
        || is_duckypad_key_activity_recent(280)
    {
        return None;
    }
    let _guard = HID_IO_LOCK.try_lock().ok()?;
    with_cached_duckypad_hid_for_serial(target_serial, |device| {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x2E;
        tx[2] = HID_CMD_QUERY_INFO;

        device.write(&tx).map_err(|e| e.to_string())?;

        let mut rx = [0u8; 64];
        let len = read_hid_report4(device, &mut rx, 120)?;
        if len >= 12 && rx[2] == 0 && rx[6] != 0 && rx[11] > 0 {
            return Ok(Some(rx[11]));
        }
        Ok(None)
    })
    .ok()
    .flatten()
}

pub fn poll_hardware_active_profile() -> Option<u8> {
    let selected = get_selected_duckypad_serial();
    poll_hardware_active_profile_for_serial(selected.as_deref())
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
        let list = list_available_duckypad_pro_firmwares("1.1.2 (DuckyDeck Studio)");
        assert!(
            !list.is_empty(),
            "Expected at least 1 DuckyDeck Studio firmware release, got {}",
            list.len()
        );
        // First entry should be v1.1.2 (DuckyDeck Studio Firmware)
        let studio_entries: Vec<&FirmwareVersionEntry> =
            list.iter().filter(|e| !e.is_official).collect();
        assert!(!studio_entries.is_empty());
        assert_eq!(studio_entries[0].version, "1.1.2");
        assert_eq!(studio_entries[0].vendor, "DuckyDeck Studio");
        assert!(studio_entries[0].supports_monochrome_icons);
        assert!(
            studio_entries[0].filename.starts_with("DDS_FW_"),
            "Custom firmware should use DDS_FW_ prefix: {}",
            studio_entries[0].filename
        );
        for entry in studio_entries {
            let bytes =
                std::fs::read(&entry.path).expect("Bundled firmware binary should be readable");
            let crc = compute_ieee_crc32(&bytes);
            assert_eq!(
                format!("{crc:08x}"),
                entry.crc32,
                "CRC32 mismatch for {}",
                entry.filename
            );
        }

        // Verify duckyPad (2020) custom firmware catalog as well
        let dp20_list = list_available_duckypad_2020_firmwares("1.1.2");
        let dp20_studio: Vec<&FirmwareVersionEntry> =
            dp20_list.iter().filter(|e| !e.is_official).collect();
        assert!(
            !dp20_studio.is_empty(),
            "Expected duckyPad (2020) custom firmware in catalog"
        );
        assert_eq!(dp20_studio[0].version, "1.1.2");
        assert_eq!(dp20_studio[0].filename, "DDS20_FW_1.1.2_3bd4beef.dfu");
        assert!(dp20_studio[0].supports_monochrome_icons);
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
        with_empty_slot[0].slots.insert(5, DuckyKeySlot::default());
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

        let temp_sd =
            std::env::temp_dir().join(format!("duckydeck_rotation_test_{}", std::process::id()));
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

    #[test]
    fn test_duckypad_2020_hid_info_and_slot_conversions() {
        // 1. Verify HID GET_INFO response parsing for duckyPad (2020) (model 20) and duckyPad Pro (model 24)
        let mut rx_2020 = [0u8; 64];
        rx_2020[3] = 2; // fw major
        rx_2020[4] = 1; // fw minor
        rx_2020[5] = 0; // fw patch
        rx_2020[6] = DP_MODEL_OG_DUCKYPAD; // 20
        rx_2020[7..11].copy_from_slice(&0xA1B2C3D4u32.to_be_bytes());
        rx_2020[11] = 3; // current profile
        rx_2020[12] = 0; // awake

        let info_2020 =
            parse_hid_query_info_response(&rx_2020, 64, PID_DUCKYPAD_2020, "/dev/hidraw0")
                .expect("parse dp20");
        assert_eq!(info_2020.hw_model, DP_MODEL_OG_DUCKYPAD);
        assert_eq!(info_2020.model_name(), "duckyPad (2020)");
        assert!(info_2020.is_duckypad_2020());
        assert_eq!(info_2020.serial, "A1B2C3D4");
        assert_eq!(info_2020.disk_label, "DP20_C3D4");
        assert_eq!(info_2020.fw_version, "2.1.0");
        assert_eq!(info_2020.active_profile, 3);
        assert_eq!(onboard_key_count_for_model(info_2020.hw_model), 15);
        assert_eq!(onboard_key_count_for_pid(PID_DUCKYPAD_2020), 15);

        let mut rx_pro = [0u8; 64];
        rx_pro[3] = 1;
        rx_pro[4] = 1;
        rx_pro[5] = 1;
        rx_pro[6] = DP_MODEL_DUCKYPAD_PRO; // 24
        rx_pro[7..11].copy_from_slice(&0x11223344u32.to_be_bytes());
        rx_pro[11] = 1;
        rx_pro[20..23].copy_from_slice(b"DDS");
        let info_pro = parse_hid_query_info_response(&rx_pro, 64, PID_DUCKYPAD_PRO, "/dev/hidraw1")
            .expect("parse dp24");
        assert_eq!(info_pro.hw_model, DP_MODEL_DUCKYPAD_PRO);
        assert_eq!(info_pro.model_name(), "duckyPad Pro");
        assert!(!info_pro.is_duckypad_2020());
        assert_eq!(info_pro.serial, "11223344");
        assert_eq!(info_pro.disk_label, "DP24_3344");
        assert_eq!(info_pro.fw_version, "1.1.1");
        assert_eq!(onboard_key_count_for_model(info_pro.hw_model), 20);

        // 2. Verify 3x5 <-> 4x5 physical row-preserving key slot conversion
        let mut dp20_profile = DuckyProfile {
            number: 1,
            name: "OGPad".to_string(),
            ..Default::default()
        };
        for s in 1..=15usize {
            dp20_profile.slots.insert(
                s,
                DuckyKeySlot {
                    slot: s,
                    label_line1: format!("K{s}"),
                    on_press_script: format!("STRING key_{s}\n"),
                    ..Default::default()
                },
            );
        }
        let mut converted_dp24 = dp20_profile.clone();
        convert_profile_slots_dp20_to_dp24(&mut converted_dp24);
        // Row 0: 1,2,3 -> 1,2,3; Row 1: 4,5,6 -> 5,6,7; Row 4: 13,14,15 -> 17,18,19
        assert_eq!(converted_dp24.slots.get(&1).unwrap().label_line1, "K1");
        assert_eq!(converted_dp24.slots.get(&5).unwrap().label_line1, "K4");
        assert_eq!(converted_dp24.slots.get(&19).unwrap().label_line1, "K15");
        assert!(!converted_dp24.slots.contains_key(&4)); // 4th column left unassigned

        let mut roundtrip_dp20 = converted_dp24.clone();
        convert_profile_slots_dp24_to_dp20(&mut roundtrip_dp20);
        for s in 1..=15usize {
            assert_eq!(
                roundtrip_dp20.slots.get(&s).unwrap().label_line1,
                format!("K{s}"),
                "Round-trip dp20 -> dp24 -> dp20 should preserve slot {s}"
            );
        }

        // 3. Verify HID file command packet builder for duckyPad (2020)
        let pkt =
            build_hid_file_command_packet(0x42, HID_CMD_OPEN_FILE_FOR_WRITING, b"profile_info.txt");
        assert_eq!(pkt[0], HID_REPORT_ID_OUT);
        assert_eq!(pkt[1], 0x42);
        assert_eq!(pkt[2], HID_CMD_OPEN_FILE_FOR_WRITING);
        assert_eq!(&pkt[3..19], b"profile_info.txt");
        assert_eq!(pkt[19], 0);
    }

    #[test]
    fn test_multiple_duckypads_and_multiple_mounted_sd_cards_selection() {
        let base_temp =
            std::env::temp_dir().join(format!("duckydeck_multi_sd_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&base_temp); // ignore: best-effort test temp dir cleanup

        // Create 3 simulated mounted SD cards under base_temp:
        // 1) DP24_1111 (duckyPad Pro #1, serial AAAA1111)
        // 2) DP24_2222 (duckyPad Pro #2, serial BBBB2222)
        // 3) DP20_3333 (duckyPad 2020 #1, serial CCCC3333, legacy profile1_Default folder)
        let sd_pro_1 = base_temp.join("DP24_1111");
        let sd_pro_2 = base_temp.join("DP24_2222");
        let sd_2020_1 = base_temp.join("DP20_3333");

        fs::create_dir_all(&sd_pro_1).expect("create sd_pro_1");
        fs::create_dir_all(&sd_pro_2).expect("create sd_pro_2");
        fs::create_dir_all(&sd_2020_1).expect("create sd_2020_1");

        // Populate sd_pro_1 via write_profiles_to_sd_root_for_model
        let mut p_pro_1 = DuckyProfile {
            number: 1,
            name: "ProOne".to_string(),
            ..Default::default()
        };
        p_pro_1.slots.insert(
            1,
            DuckyKeySlot {
                slot: 1,
                label_line1: "PRO1".to_string(),
                on_press_script: "STRING pro1\n".to_string(),
                ..Default::default()
            },
        );
        let meta_pro_1 = DuckySdDeviceMeta {
            hw_model: DP_MODEL_DUCKYPAD_PRO,
            model_name: "duckyPad Pro".to_string(),
            serial_number: "AAAA1111".to_string(),
            disk_label: "DP24_1111".to_string(),
            firmware_version: "1.1.1".to_string(),
        };
        write_profiles_to_sd_root_for_model(
            &sd_pro_1,
            &[p_pro_1],
            DP_MODEL_DUCKYPAD_PRO,
            Some(&meta_pro_1),
        )
        .expect("write sd_pro_1");

        // Populate sd_pro_2
        let mut p_pro_2 = DuckyProfile {
            number: 1,
            name: "ProTwo".to_string(),
            ..Default::default()
        };
        p_pro_2.slots.insert(
            20,
            DuckyKeySlot {
                slot: 20,
                label_line1: "PRO2".to_string(),
                on_press_script: "STRING pro2\n".to_string(),
                ..Default::default()
            },
        );
        let meta_pro_2 = DuckySdDeviceMeta {
            hw_model: DP_MODEL_DUCKYPAD_PRO,
            model_name: "duckyPad Pro".to_string(),
            serial_number: "BBBB2222".to_string(),
            disk_label: "DP24_2222".to_string(),
            firmware_version: "1.1.1".to_string(),
        };
        write_profiles_to_sd_root_for_model(
            &sd_pro_2,
            &[p_pro_2],
            DP_MODEL_DUCKYPAD_PRO,
            Some(&meta_pro_2),
        )
        .expect("write sd_pro_2");

        // Populate sd_2020_1 with both a valid key15 and an out-of-range key18 + encoder slot 21
        // to verify duckyPad (2020) clamps physical SD output to keys 1..=15
        let mut p_2020 = DuckyProfile {
            number: 1,
            name: "Legacy2020".to_string(),
            ..Default::default()
        };
        p_2020.slots.insert(
            15,
            DuckyKeySlot {
                slot: 15,
                label_line1: "KEY15".to_string(),
                on_press_script: "STRING key15\n".to_string(),
                ..Default::default()
            },
        );
        p_2020.slots.insert(
            18,
            DuckyKeySlot {
                slot: 18,
                label_line1: "KEY18".to_string(),
                on_press_script: "STRING key18\n".to_string(),
                ..Default::default()
            },
        );
        p_2020.slots.insert(
            21,
            DuckyKeySlot {
                slot: 21,
                label_line1: "ENC1".to_string(),
                on_press_script: "STRING enc1\n".to_string(),
                ..Default::default()
            },
        );
        let meta_2020 = DuckySdDeviceMeta {
            hw_model: DP_MODEL_OG_DUCKYPAD,
            model_name: "duckyPad (2020)".to_string(),
            serial_number: "CCCC3333".to_string(),
            disk_label: "DP20_3333".to_string(),
            firmware_version: "2.1.0".to_string(),
        };
        write_profiles_to_sd_root_for_model(
            &sd_2020_1,
            &[p_2020],
            DP_MODEL_OG_DUCKYPAD,
            Some(&meta_2020),
        )
        .expect("write sd_2020_1");
        fs::write(
            sd_2020_1.join("dp_stats.txt"),
            "fw 2.1.0\nserial CCCC3333\n",
        )
        .expect("write dp_stats.txt");

        // Verify duckyPad (2020) SD card only wrote key15.txt and did NOT write key18.txt or key21.txt
        let dp20_prof_dir = sd_2020_1.join("profile_Legacy2020");
        assert!(dp20_prof_dir.join("key15.txt").is_file());
        assert!(
            !dp20_prof_dir.join("key18.txt").exists(),
            "duckyPad (2020) must clamp physical onboard keys to 1..=15"
        );
        assert!(
            !dp20_prof_dir.join("key21.txt").exists(),
            "duckyPad (2020) has no rotary encoders (slots 21..=26)"
        );

        // Discover all 3 mounted SD cards simultaneously
        let mounted = list_mounted_duckypad_sds_in_roots(std::slice::from_ref(&base_temp));
        assert_eq!(
            mounted.len(),
            3,
            "Should discover all 3 mounted duckyPad SD cards, got: {mounted:?}"
        );

        // Verify exact selection for duckyPad Pro #1 (serial AAAA1111 / DP24_1111)
        let match_pro_1 = find_mounted_duckypad_sd_for_device_in_list(
            &mounted,
            Some("AAAA1111"),
            Some("DP24_1111"),
            Some(DP_MODEL_DUCKYPAD_PRO),
        )
        .expect("should match sd_pro_1");
        assert_eq!(match_pro_1.path, sd_pro_1);
        assert_eq!(match_pro_1.hw_model, DP_MODEL_DUCKYPAD_PRO);

        // Verify exact selection for duckyPad Pro #2 (serial BBBB2222 / DP24_2222)
        let match_pro_2 = find_mounted_duckypad_sd_for_device_in_list(
            &mounted,
            Some("BBBB2222"),
            Some("DP24_2222"),
            Some(DP_MODEL_DUCKYPAD_PRO),
        )
        .expect("should match sd_pro_2");
        assert_eq!(match_pro_2.path, sd_pro_2);

        // Verify exact selection for duckyPad (2020) (serial CCCC3333 / DP20_3333)
        let match_2020 = find_mounted_duckypad_sd_for_device_in_list(
            &mounted,
            Some("CCCC3333"),
            Some("DP20_3333"),
            Some(DP_MODEL_OG_DUCKYPAD),
        )
        .expect("should match sd_2020_1");
        assert_eq!(match_2020.path, sd_2020_1);
        assert_eq!(match_2020.hw_model, DP_MODEL_OG_DUCKYPAD);

        // Verify a 4th connected duckyPad (serial DDDD4444 / DP24_4444) whose SD card is NOT mounted
        // does NOT accidentally grab sd_pro_1, sd_pro_2, or sd_2020_1!
        let match_unmounted = find_mounted_duckypad_sd_for_device_in_list(
            &mounted,
            Some("DDDD4444"),
            Some("DP24_4444"),
            Some(DP_MODEL_DUCKYPAD_PRO),
        );
        assert!(
            match_unmounted.is_none(),
            "Must NOT select a foreign duckyPad's SD card when the target device's SD card is not mounted: {match_unmounted:?}"
        );

        // Verify selected serial state management
        let prev_sel = get_selected_duckypad_serial();
        set_selected_duckypad_serial(Some("cccc3333"));
        assert_eq!(get_selected_duckypad_serial().as_deref(), Some("CCCC3333"));
        assert!(serial_matches_target("CCCC3333", "3333"));
        assert!(!serial_matches_target("CCCC3333", "1111"));
        set_selected_duckypad_serial(prev_sel.as_deref());

        let _ = fs::remove_dir_all(&base_temp); // ignore: best-effort test temp dir cleanup
    }

    #[test]
    fn test_multi_device_profile_sync_isolation_and_simultaneous_rules() {
        use crate::hardware::profile::{
            check_profile_sync_status, create_default_profiles_for_device, DuckyProfile,
        };
        use crate::kwin::auto_switcher::{
            get_secondary_devices_snapshot, match_profile_with_rules,
            set_secondary_device_profile_rotation, update_secondary_device_active_profile,
            update_secondary_device_runtime, AutoSwitchRule, SecondaryDeviceRuntime,
        };

        // 1. Default profile generation per device model
        let pro_defaults =
            create_default_profiles_for_device(PID_DUCKYPAD_PRO, "UNITTEST_PRO_9901");
        assert_eq!(pro_defaults.len(), 1);
        assert_eq!(pro_defaults[0].number, 1);

        let og_defaults =
            create_default_profiles_for_device(PID_DUCKYPAD_2020, "UNITTEST_2020_9902");
        assert_eq!(og_defaults.len(), 1);
        assert_eq!(og_defaults[0].number, 1);

        // 2. Newly connected second device with no SD snapshot yet reports "synced" (no false unsynced alert)
        let test_serial = "UNITTEST_NEW_SECOND_DUCKY_9903";
        let (state, _detail) = check_profile_sync_status(test_serial, &og_defaults, None, true, 1);
        assert_eq!(
            state, "synced",
            "Newly plugged-in second duckyPad must not trigger a false unsynced alert"
        );

        // Once the user edits a profile on that second duckyPad, it accurately transitions to "unsynced"
        let mut edited_og = og_defaults.clone();
        edited_og[0].name = "EditedSecondDucky".to_string();
        let (edited_state, _edited_detail) =
            check_profile_sync_status(test_serial, &edited_og, None, true, 1);
        assert_eq!(
            edited_state, "unsynced",
            "Editing a profile on the second duckyPad must report unsynced for that device"
        );
        let _ = fs::remove_dir_all(crate::hardware::profile::profile_cache_dir_for_serial(
            test_serial,
        ));

        // 3. Independent Auto-Switch rules per duckyPad evaluated simultaneously
        let dev1_rules = vec![
            AutoSwitchRule {
                enabled: true,
                resource_class: "firefox".to_string(),
                title_contains: None,
                title_is_regex: false,
                favicon_url: None,
                target_profile: 2,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "code".to_string(),
                title_contains: None,
                title_is_regex: false,
                favicon_url: None,
                target_profile: 4,
            },
        ];
        let dev2_rules = vec![
            AutoSwitchRule {
                enabled: true,
                resource_class: "firefox".to_string(),
                title_contains: None,
                title_is_regex: false,
                favicon_url: None,
                target_profile: 5,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "obs".to_string(),
                title_contains: None,
                title_is_regex: false,
                favicon_url: None,
                target_profile: 3,
            },
        ];

        // When Firefox is focused, Device 1 switches to Profile 2 while Device 2 switches to Profile 5
        assert_eq!(
            match_profile_with_rules(&dev1_rules, Some(1), "firefox", "Mozilla Firefox"),
            Some(2)
        );
        assert_eq!(
            match_profile_with_rules(&dev2_rules, Some(1), "firefox", "Mozilla Firefox"),
            Some(5)
        );

        // When VS Code is focused, Device 1 switches to Profile 4 while Device 2 falls back to its own default (Profile 1)
        assert_eq!(
            match_profile_with_rules(&dev1_rules, Some(1), "code", "main.rs - VS Code"),
            Some(4)
        );
        assert_eq!(
            match_profile_with_rules(&dev2_rules, Some(1), "code", "main.rs - VS Code"),
            Some(1)
        );

        // 4. Secondary device runtime state & per-device widget rotation/profile tracking
        let dev2_profiles = vec![
            DuckyProfile {
                number: 1,
                name: "Stream".to_string(),
                ..Default::default()
            },
            DuckyProfile {
                number: 5,
                name: "Browser".to_string(),
                ..Default::default()
            },
        ];
        update_secondary_device_runtime(SecondaryDeviceRuntime {
            serial: "UNITTEST_2020_9902".to_string(),
            pid: PID_DUCKYPAD_2020,
            hw_model: DP_MODEL_OG_DUCKYPAD,
            model_name: "duckyPad (2020)".to_string(),
            firmware_version: "2.1.0".to_string(),
            disk_label: "DP20_9902".to_string(),
            usb_connected: true,
            active_profile: 1,
            profiles: dev2_profiles,
            rules_enabled: true,
            default_profile: Some(1),
            rules: dev2_rules,
        });
        update_secondary_device_active_profile("UNITTEST_2020_9902", 5);
        assert!(set_secondary_device_profile_rotation(
            "UNITTEST_2020_9902",
            90
        ));

        let snap = get_secondary_devices_snapshot();
        let entry = snap
            .iter()
            .find(|d| d.serial == "UNITTEST_2020_9902")
            .expect("secondary runtime should contain UNITTEST_2020_9902");
        assert_eq!(entry.active_profile, 5);
        assert_eq!(entry.hw_model, DP_MODEL_OG_DUCKYPAD);
        let p5 = entry
            .profiles
            .iter()
            .find(|p| p.number == 5)
            .expect("profile 5 should exist");
        assert_eq!(p5.effective_widget_rotation_degrees(), 90);
    }
}
