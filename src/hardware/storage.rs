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
    let _ = device.read_timeout(&mut rx, 200);
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckyKeySlot {
    pub slot: usize,
    pub label_line1: String,
    pub label_line2: String,
    #[serde(default)]
    pub widget_label_line1: String,
    #[serde(default)]
    pub widget_label_line2: String,
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
            widget_label_line1: String::new(),
            widget_label_line2: String::new(),
            custom_rgb: None,
            allow_abort: false,
            disable_autorepeat: false,
            on_press_script: String::new(),
            on_release_script: String::new(),
        }
    }
}

impl DuckyKeySlot {
    pub fn is_effectively_empty(&self) -> bool {
        self.label_line1.is_empty()
            && self.label_line2.is_empty()
            && self.widget_label_line1.trim().is_empty()
            && self.widget_label_line2.trim().is_empty()
            && self.custom_rgb.is_none()
            && !self.allow_abort
            && !self.disable_autorepeat
            && self.on_press_script.trim().is_empty()
            && self.on_release_script.trim().is_empty()
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

impl DuckyProfile {
    pub fn oled_rotation_degrees(&self) -> i32 {
        let mut rot = 0;
        for line in &self.preserved_config_lines {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("OLED_ROTATION ") {
                match rest.trim() {
                    "90" => rot = 90,
                    "180" => rot = 180,
                    "270" => rot = 270,
                    "0" => rot = 0,
                    _ => {}
                }
            } else if t == "IS_LANDSCAPE 3" || t == "IS_ROTATED_270 1" {
                rot = 270;
            } else if t == "IS_ROTATED_180 1" || t == "IS_LANDSCAPE 2" {
                rot = 180;
            } else if t == "IS_LANDSCAPE 1" {
                if rot != 180 && rot != 270 {
                    rot = 90;
                }
            }
        }
        rot
    }

    pub fn set_oled_rotation_degrees(&mut self, degrees: i32) {
        let normalized = match degrees {
            90 => 90,
            180 => 180,
            270 => 270,
            _ => 0,
        };
        let widget_rot = self.widget_rotation_degrees();
        self.preserved_config_lines.retain(|l| {
            let t = l.trim();
            !t.starts_with("IS_LANDSCAPE ")
                && !t.starts_with("OLED_ROTATION ")
                && !t.starts_with("IS_ROTATED_180 ")
                && !t.starts_with("IS_ROTATED_270 ")
                && !t.starts_with("DW_ROTATION ")
        });
        match normalized {
            90 => {
                self.preserved_config_lines.push("IS_LANDSCAPE 1".to_string());
            }
            180 => {
                self.preserved_config_lines.push("IS_LANDSCAPE 2".to_string());
                self.preserved_config_lines.push("OLED_ROTATION 180".to_string());
            }
            270 => {
                self.preserved_config_lines.push("IS_LANDSCAPE 3".to_string());
                self.preserved_config_lines.push("OLED_ROTATION 270".to_string());
            }
            _ => {}
        }
        if matches!(widget_rot, 0 | 90 | 180 | 270) {
            self.preserved_config_lines
                .push(format!("DW_ROTATION {widget_rot}"));
        }
    }

    /// Returns `-1` when Desktop Widget orientation follows the hardware OLED orientation,
    /// or `0`, `90`, `180`, `270` when configured independently for this profile.
    pub fn widget_rotation_degrees(&self) -> i32 {
        let mut wrot = -1;
        for line in &self.preserved_config_lines {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("DW_ROTATION ") {
                match rest.trim() {
                    "0" => wrot = 0,
                    "90" => wrot = 90,
                    "180" => wrot = 180,
                    "270" => wrot = 270,
                    "-1" | "AUTO" | "FOLLOW" => wrot = -1,
                    _ => {}
                }
            }
        }
        wrot
    }

    pub fn effective_widget_rotation_degrees(&self) -> i32 {
        let wrot = self.widget_rotation_degrees();
        if matches!(wrot, 0 | 90 | 180 | 270) {
            wrot
        } else {
            self.oled_rotation_degrees()
        }
    }

    pub fn set_widget_rotation_degrees(&mut self, degrees: i32) {
        let normalized = match degrees {
            0 => 0,
            90 => 90,
            180 => 180,
            270 => 270,
            _ => -1,
        };
        self.preserved_config_lines
            .retain(|l| !l.trim().starts_with("DW_ROTATION "));
        if normalized >= 0 {
            self.preserved_config_lines
                .push(format!("DW_ROTATION {normalized}"));
        }
    }

    pub fn is_upper_hs(&self) -> bool {
        self.preserved_config_lines.iter().any(|l| {
            let t = l.trim();
            t == "IS_UPPER_HS 1"
                || t == "UPPER_HS 1"
                || t.strip_prefix("IS_UPPER_HS ")
                    .map(|r| r.trim() != "0")
                    .unwrap_or(false)
                || t.strip_prefix("UPPER_HS ")
                    .map(|r| r.trim() != "0")
                    .unwrap_or(false)
        })
    }

    pub fn profile_webhook(&self) -> ProfileWebhookConfig {
        for line in &self.preserved_config_lines {
            let t = line.trim();
            if let Some(json_str) = t.strip_prefix("PROFILE_WEBHOOK ") {
                if let Ok(mut cfg) = serde_json::from_str::<ProfileWebhookConfig>(json_str.trim()) {
                    if cfg.method.trim().is_empty() {
                        cfg.method = "POST".to_string();
                    }
                    return cfg;
                }
            }
        }
        ProfileWebhookConfig::default()
    }

    pub fn set_profile_webhook(&mut self, cfg: &ProfileWebhookConfig) {
        self.preserved_config_lines
            .retain(|l| !l.trim().starts_with("PROFILE_WEBHOOK "));
        let has_content = cfg.enabled
            || !cfg.destination.trim().is_empty()
            || !cfg.url.trim().is_empty()
            || !cfg.body.trim().is_empty();
        if has_content {
            if let Ok(json_str) = serde_json::to_string(cfg) {
                self.preserved_config_lines
                    .push(format!("PROFILE_WEBHOOK {json_str}"));
            }
        }
    }

    pub fn canonical_extra_config_lines(&self) -> Vec<String> {
        self.preserved_config_lines
            .iter()
            .map(|l| l.trim().to_string())
            .filter(|t| {
                !t.is_empty()
                    && !t.starts_with("IS_LANDSCAPE ")
                    && !t.starts_with("OLED_ROTATION ")
                    && !t.starts_with("IS_ROTATED_180 ")
                    && !t.starts_with("IS_ROTATED_270 ")
                    && !t.starts_with("DW_ROTATION ")
                    && !t.starts_with("PROFILE_WEBHOOK ")
                    && !t.starts_with("IS_UPPER_HS ")
                    && !t.starts_with("UPPER_HS ")
                    && !t.starts_with("BG_COLOR ")
                    && !t.starts_with("KEYDOWN_COLOR ")
                    && !t.starts_with("DIM_UNUSED_KEYS ")
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileWebhookConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub destination: String,
    #[serde(default = "default_web_api_method")]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: String,
    #[serde(default)]
    pub body: String,
}

impl Default for ProfileWebhookConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            destination: String::new(),
            method: "POST".to_string(),
            url: String::new(),
            headers: "Content-Type: application/json".to_string(),
            body: String::new(),
        }
    }
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

pub fn oled_icon_char_to_ascii(ch: char, compact: bool) -> Option<&'static str> {
    match ch {
        '\u{FE0E}' | '\u{FE0F}' | '\u{200D}' => Some(""),
        '▶' => Some(if compact { ">" } else { "|>" }),
        '⏸' => Some("||"),
        '⏯' => Some(if compact { ">|" } else { "|>||" }),
        '⏹' => Some(if compact { "#" } else { "[#]" }),
        '⏭' => Some(if compact { ">|" } else { ">>|" }),
        '⏮' => Some(if compact { "|<" } else { "|<<" }),
        '⏩' => Some(">>"),
        '⏪' => Some("<<"),
        '🔊' => Some(if compact { "+" } else { "((+)" }),
        '🔉' => Some(if compact { "-" } else { "((-)" }),
        '🔇' => Some(if compact { "x" } else { "(x)" }),
        '🎤' => Some(if compact { "I" } else { "(I)" }),
        '🎧' => Some(if compact { "H" } else { "d-b" }),
        '🎵' | '🎶' => Some(if compact { "~" } else { "d~b" }),
        '⏺' => Some(if compact { "o" } else { "(o)" }),
        '↑' | '▲' => Some(if compact { "^" } else { "[^]" }),
        '↓' | '▼' => Some(if compact { "v" } else { "[v]" }),
        '←' | '◀' => Some(if compact { "<" } else { "<--" }),
        '→' => Some(if compact { ">" } else { "-->" }),
        '↖' => Some("<^"),
        '↘' => Some("v>"),
        '↩' | '↺' => Some(if compact { "<" } else { "<-'" }),
        '↪' | '↻' => Some(if compact { ">" } else { "'->" }),
        '⇄' => Some(if compact { "=" } else { "<->" }),
        '⇅' => Some(if compact { "|" } else { "^/v" }),
        '✂' => Some("8<"),
        '📋' => Some(if compact { "=" } else { "[=]" }),
        '📌' => Some(if compact { "*" } else { "[o]" }),
        '💾' => Some(if compact { "S" } else { "[S]" }),
        '📁' => Some(if compact { "/" } else { "[/]" }),
        '📄' => Some(if compact { "F" } else { "[F]" }),
        '🔍' => Some(if compact { "?" } else { "(?)" }),
        '🗑' => Some(if compact { "X" } else { "[X]" }),
        '✏' => Some(if compact { "/" } else { "[/]" }),
        '🔖' => Some(if compact { "*" } else { "[*]" }),
        '📷' => Some(if compact { "O" } else { "[O]" }),
        '🔗' => Some(if compact { "~" } else { "o-o" }),
        '⌨' => Some(">_"),
        '⚡' => Some(if compact { "!" } else { "/!/" }),
        '🐞' => Some(if compact { "*" } else { "{*}" }),
        '🚀' => Some(if compact { "^" } else { "/^\\" }),
        '🧪' => Some(if compact { "T" } else { "\\o/" }),
        '🔧' => Some(if compact { "+" } else { ">-<" }),
        '⚙' => Some(if compact { "*" } else { "(*)" }),
        '📦' => Some(if compact { "#" } else { "[#]" }),
        '🌿' => Some("Y-"),
        '☁' => Some(if compact { "~" } else { "(~)" }),
        '🤖' => Some(if compact { "@" } else { "[oo]" }),
        '⌘' => Some(if compact { "#" } else { "[#]" }),
        '🖥' => Some(if compact { "=" } else { "[=]" }),
        '🪟' => Some(if compact { "+" } else { "[+]" }),
        '🔒' => Some(if compact { "L" } else { "[L]" }),
        '🔓' => Some(if compact { "U" } else { "[U]" }),
        '🌙' => Some("(("),
        '☀' => Some(if compact { "*" } else { "\\o/" }),
        '🔋' => Some(if compact { "+" } else { "[=]+" }),
        '🌐' => Some(if compact { "W" } else { "(www)" }),
        '📡' => Some(if compact { "i" } else { "((i))" }),
        '⏱' => Some(if compact { "L" } else { "(L)" }),
        '🔔' => Some(if compact { "!" } else { "/_\\" }),
        '🏠' => Some(if compact { "H" } else { "/H\\" }),
        '🎥' => Some(if compact { ">" } else { "[>]" }),
        '📺' => Some(if compact { "T" } else { "[TV]" }),
        '💬' => Some(if compact { "\"" } else { "(\")" }),
        '🎮' => Some(if compact { "+" } else { "[+o]" }),
        '🎨' => Some(if compact { ":" } else { "(:)" }),
        '🧮' => Some(if compact { "+" } else { "[+-]" }),
        '📧' => Some(if compact { "M" } else { "[M]" }),
        '📅' => Some(if compact { "#" } else { "[31]" }),
        '★' | '☆' => Some(if compact { "*" } else { "[*]" }),
        '♥' => Some("<3"),
        '✔' => Some(if compact { "v" } else { "[v]" }),
        '✖' => Some(if compact { "x" } else { "[x]" }),
        '➕' => Some(if compact { "+" } else { "[+]" }),
        '➖' => Some(if compact { "-" } else { "[-]" }),
        '⚠' => Some(if compact { "!" } else { "/!\\" }),
        '🔥' => Some(if compact { "^" } else { "^^^" }),
        '💎' => Some("<>"),
        '🎯' => Some(if compact { "o" } else { "(o)" }),
        '∞' => Some("oo"),
        _ => None,
    }
}

/// Converts a key label into a 1–5 byte 7-bit printable ASCII string (`0x20..=0x7E`)
/// that the physical duckyPad / duckyPad Pro firmware (`Font_6x8` in `ssd1306_fonts.c`)
/// can render on its hardware OLED screen.
pub fn oled_label_to_hardware_ascii(label: &str) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.bytes().all(|b| (0x20..=0x7E).contains(&b)) {
        return trimmed.chars().take(5).collect();
    }
    let has_ascii_alnum = trimmed.chars().any(|c| c.is_ascii_alphanumeric());
    let mut out = String::with_capacity(8);
    for ch in trimmed.chars() {
        if (' '..='~').contains(&ch) {
            out.push(ch);
        } else if let Some(mapped) = oled_icon_char_to_ascii(ch, has_ascii_alnum) {
            out.push_str(mapped);
        } else if !ch.is_control() {
            out.push('*');
        }
    }
    let cleaned: String = out
        .trim()
        .chars()
        .filter(|c| (' '..='~').contains(c))
        .take(5)
        .collect();
    if cleaned.is_empty() {
        "[*]".to_string()
    } else {
        cleaned
    }
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
                } else if let Some((slot_idx, wlabel1)) = parse_indexed_directive(line, "dw_z") {
                    profile.ensure_slot(slot_idx).widget_label_line1 = wlabel1.to_string();
                } else if let Some((slot_idx, wlabel2)) = parse_indexed_directive(line, "dw_x") {
                    profile.ensure_slot(slot_idx).widget_label_line2 = wlabel2.to_string();
                } else if let Some((slot_idx, label1)) = parse_indexed_directive(line, "ui_z") {
                    profile.ensure_slot(slot_idx).label_line1 = label1.to_string();
                } else if let Some((slot_idx, label2)) = parse_indexed_directive(line, "ui_x") {
                    profile.ensure_slot(slot_idx).label_line2 = label2.to_string();
                } else if let Some((slot_idx, label1)) = parse_indexed_directive(line, "z") {
                    let slot = profile.ensure_slot(slot_idx);
                    if slot.label_line1.is_empty() {
                        slot.label_line1 = label1.to_string();
                    }
                } else if let Some((slot_idx, label2)) = parse_indexed_directive(line, "x") {
                    let slot = profile.ensure_slot(slot_idx);
                    if slot.label_line2.is_empty() {
                        slot.label_line2 = label2.to_string();
                    }
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
                let hw_l1 = oled_label_to_hardware_ascii(&slot.label_line1);
                if !hw_l1.is_empty() {
                    cfg.push_str(&format!("z{} {}\n", slot_idx, hw_l1));
                }
                if hw_l1 != slot.label_line1 {
                    cfg.push_str(&format!("ui_z{} {}\n", slot_idx, slot.label_line1));
                }
            }
            if !slot.label_line2.is_empty() {
                let hw_l2 = oled_label_to_hardware_ascii(&slot.label_line2);
                if !hw_l2.is_empty() {
                    cfg.push_str(&format!("x{} {}\n", slot_idx, hw_l2));
                }
                if hw_l2 != slot.label_line2 {
                    cfg.push_str(&format!("ui_x{} {}\n", slot_idx, slot.label_line2));
                }
            }
            if !slot.widget_label_line1.trim().is_empty() {
                cfg.push_str(&format!("dw_z{} {}\n", slot_idx, slot.widget_label_line1.trim()));
            }
            if !slot.widget_label_line2.trim().is_empty() {
                cfg.push_str(&format!("dw_x{} {}\n", slot_idx, slot.widget_label_line2.trim()));
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

pub fn merge_host_only_profile_settings(
    target_profiles: &mut [DuckyProfile],
    previous_profiles: &[DuckyProfile],
) {
    for target in target_profiles.iter_mut() {
        let prev_opt = previous_profiles
            .iter()
            .find(|p| p.number == target.number && p.name == target.name)
            .or_else(|| previous_profiles.iter().find(|p| p.name == target.name))
            .or_else(|| previous_profiles.iter().find(|p| p.number == target.number));
        if let Some(prev) = prev_opt {
            let prev_wrot = prev.widget_rotation_degrees();
            if prev_wrot >= 0 {
                target.set_widget_rotation_degrees(prev_wrot);
            }
            let prev_wh = prev.profile_webhook();
            if prev_wh.enabled
                || !prev_wh.url.trim().is_empty()
                || !prev_wh.destination.trim().is_empty()
            {
                target.set_profile_webhook(&prev_wh);
            }
        }
    }
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
        feed(&prof.oled_rotation_degrees().to_le_bytes());
        feed(&[u8::from(prof.is_upper_hs())]);
        for line in prof.canonical_extra_config_lines() {
            feed(line.as_bytes());
        }
        for slot_idx in 1..=MAX_INPUT_SLOTS {
            if let Some(slot) = prof.slots.get(&slot_idx) {
                if slot.is_effectively_empty() {
                    continue;
                }
                feed(&[slot_idx as u8]);
                feed(slot.label_line1.as_bytes());
                feed(slot.label_line2.as_bytes());
                feed(slot.widget_label_line1.trim().as_bytes());
                feed(slot.widget_label_line2.trim().as_bytes());
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistedSdSpaceSnapshot {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub baseline_cache_bytes: u64,
}

#[derive(Debug, Clone, Default)]
pub struct SdCardSpaceInfo {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub used_percent: i32,
    pub is_live_mount: bool,
    pub summary_text: String,
    pub tray_text: String,
    pub detail_tooltip: String,
}

pub fn format_storage_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= 10.0 * MB {
        format!("{:.1} MB", b / MB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.0} KB", (b / KB).max(1.0))
    } else {
        format!("{bytes} B")
    }
}

pub fn query_filesystem_space(path: &Path) -> Option<(u64, u64, u64)> {
    if !path.exists() {
        return None;
    }
    let out = std::process::Command::new("df")
        .args(["-B1", "--output=size,used,avail"])
        .arg(path)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let data_line = stdout.lines().nth(1)?.trim();
    let mut parts = data_line.split_whitespace();
    let total = parts.next()?.parse::<u64>().ok()?;
    let used = parts.next()?.parse::<u64>().ok()?;
    let avail = parts.next()?.parse::<u64>().ok()?;
    if total == 0 {
        return None;
    }
    Some((total, used, avail))
}

pub fn compute_dir_fat32_used_bytes(dir: &Path) -> u64 {
    const CLUSTER_SIZE: u64 = 4096;
    fn walk(p: &Path) -> u64 {
        let Ok(entries) = fs::read_dir(p) else {
            return 0;
        };
        let mut total = CLUSTER_SIZE; // 1 cluster for directory entry table
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with(".sd_") {
                continue;
            }
            let path = entry.path();
            if path.is_dir() {
                total = total.saturating_add(walk(&path));
            } else if let Ok(meta) = entry.metadata() {
                let len = meta.len();
                let clusters = if len == 0 {
                    1
                } else {
                    len.div_ceil(CLUSTER_SIZE)
                };
                total = total.saturating_add(clusters * CLUSTER_SIZE);
            }
        }
        total
    }
    if !dir.is_dir() {
        return 0;
    }
    walk(dir)
}

pub fn record_mounted_sd_space(serial: &str, mount_root: &Path) {
    let Some((total_bytes, used_bytes, free_bytes)) = query_filesystem_space(mount_root) else {
        return;
    };
    let dev_dir = profile_cache_dir_for_serial(serial);
    let cache_ref = if dev_dir.is_dir() {
        dev_dir.clone()
    } else {
        profile_cache_dir()
    };
    let baseline_cache_bytes = compute_dir_fat32_used_bytes(&cache_ref);
    let snap = PersistedSdSpaceSnapshot {
        total_bytes,
        used_bytes,
        free_bytes,
        baseline_cache_bytes,
    };
    let Ok(json) = serde_json::to_string_pretty(&snap) else {
        return;
    };
    let global_dir = profile_cache_dir();
    let _ = fs::create_dir_all(&global_dir);
    let _ = fs::write(global_dir.join(".sd_space_info.json"), &json);
    if dev_dir != global_dir {
        let _ = fs::create_dir_all(&dev_dir);
        let _ = fs::write(dev_dir.join(".sd_space_info.json"), &json);
    }
}

pub fn get_sd_card_space_info(serial: &str, sd_mount_path: Option<&Path>) -> SdCardSpaceInfo {
    // 1. Check live mounted SD card path or auto-detected duckyPad mount
    let live_mount = sd_mount_path
        .filter(|p| !p.as_os_str().is_empty() && p.is_dir())
        .map(|p| p.to_path_buf())
        .or_else(find_mounted_duckypad_sd);

    if let Some(ref mnt) = live_mount {
        if let Some((total_bytes, used_bytes, free_bytes)) = query_filesystem_space(mnt) {
            record_mounted_sd_space(serial, mnt);
            return build_sd_space_info(
                total_bytes,
                used_bytes,
                free_bytes,
                true,
                &format!("Mounted at {}", mnt.display()),
            );
        }
    }

    // 2. Check persisted SD space snapshot from most recent MSC sync / SD mount
    let dev_dir = profile_cache_dir_for_serial(serial);
    let global_dir = profile_cache_dir();
    let cache_dir = if dev_dir.is_dir() {
        &dev_dir
    } else {
        &global_dir
    };
    let current_cache_bytes = compute_dir_fat32_used_bytes(cache_dir);

    let snap_raw = fs::read_to_string(dev_dir.join(".sd_space_info.json"))
        .or_else(|_| fs::read_to_string(global_dir.join(".sd_space_info.json")))
        .ok();

    if let Some(raw) = snap_raw {
        if let Ok(snap) = serde_json::from_str::<PersistedSdSpaceSnapshot>(&raw) {
            if snap.total_bytes > 0 {
                let delta = current_cache_bytes as i64 - snap.baseline_cache_bytes as i64;
                let adjusted_used = (snap.used_bytes as i64 + delta)
                    .max(65536)
                    .min(snap.total_bytes as i64) as u64;
                let adjusted_free = snap.total_bytes.saturating_sub(adjusted_used);
                let src = if !serial.is_empty() && serial != "-" {
                    format!("duckyPad microSD (SN: {serial})")
                } else {
                    "duckyPad microSD cache".to_string()
                };
                return build_sd_space_info(
                    snap.total_bytes,
                    adjusted_used,
                    adjusted_free,
                    false,
                    &src,
                );
            }
        }
    }

    // 3. Fallback estimation for bundled 4 GB (3.74 GiB FAT32) duckyPad microSD card
    const DEFAULT_DUCKYPAD_SD_TOTAL_BYTES: u64 = 4_018_143_232; // ~3.74 GiB FAT32 volume on 7,864,320-sector 4GB card
    const FAT32_FS_OVERHEAD_BYTES: u64 = 98_304; // System Volume Information + root FAT cluster metadata
    let used_bytes = current_cache_bytes.saturating_add(FAT32_FS_OVERHEAD_BYTES);
    let total_bytes = DEFAULT_DUCKYPAD_SD_TOTAL_BYTES;
    let free_bytes = total_bytes.saturating_sub(used_bytes);
    let src = if !serial.is_empty() && serial != "-" {
        format!("duckyPad microSD (SN: {serial})")
    } else {
        "duckyPad microSD".to_string()
    };
    build_sd_space_info(total_bytes, used_bytes, free_bytes, false, &src)
}

fn build_sd_space_info(
    total_bytes: u64,
    used_bytes: u64,
    free_bytes: u64,
    is_live_mount: bool,
    source_desc: &str,
) -> SdCardSpaceInfo {
    let pct_f = if total_bytes > 0 {
        (used_bytes as f64 * 100.0) / (total_bytes as f64)
    } else {
        0.0
    };
    let used_percent = (pct_f.round() as i32).clamp(0, 100);
    let used_str = format_storage_bytes(used_bytes);
    let free_str = format_storage_bytes(free_bytes);
    let total_str = format_storage_bytes(total_bytes);

    let summary_text = format!("SD: {used_str} used • {free_str} free");
    let tray_text = format!("SD Card: {used_str} used • {free_str} free ({total_str})");
    let detail_tooltip = format!(
        "duckyPad microSD Card Capacity\n\
         • Used Space: {used_str} ({pct_f:.2}%)\n\
         • Free Space: {free_str} ({:.2}%)\n\
         • Total Capacity: {total_str}\n\
         • Source: {source_desc}",
        (100.0 - pct_f).max(0.0)
    );

    SdCardSpaceInfo {
        total_bytes,
        used_bytes,
        free_bytes,
        used_percent,
        is_live_mount,
        summary_text,
        tray_text,
        detail_tooltip,
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

pub fn find_duckypad_msc_block_device(disk_label: &str, allow_whole_disk: bool) -> Option<PathBuf> {
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

pub fn unmount_mounted_duckypad_sd(current_mount: Option<&Path>) -> Result<String, String> {
    let mount_root = current_mount
        .filter(|p| p.exists())
        .map(|p| p.to_path_buf())
        .or_else(find_mounted_duckypad_sd)
        .ok_or_else(|| "No mounted duckyPad microSD card detected".to_string())?;

    let block_dev = find_mount_block_device(&mount_root);
    unmount_duckypad_sd_cleanly(&mount_root, block_dev.as_deref())?;

    // If the duckyPad Pro was in USB Mass Storage mode, reset it back to normal HID mode
    if find_duckypad_msc_block_device("", true).is_some() {
        let _guard = HID_IO_LOCK.lock().ok();
        if let Ok(dev) = open_duckypad_hid() {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x15;
            tx[2] = HID_CMD_SOFTWARE_RESET;
            tx[3] = 0;
            let _ = dev.write(&tx);
        }
    }

    Ok(format!("Unmounted microSD card at {}", mount_root.display()))
}

pub fn detect_unmounted_duckypad_sd(disk_label: &str) -> Option<PathBuf> {
    if find_mounted_duckypad_sd().is_some() {
        return None;
    }
    if let Some(dev) = find_duckypad_msc_block_device(disk_label, true) {
        return Some(dev);
    }
    if let Ok(entries) = fs::read_dir("/sys/block") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("mmcblk") {
                let p1 = PathBuf::from(format!("/dev/{name}p1"));
                if p1.exists() {
                    return Some(p1);
                }
                let whole = PathBuf::from(format!("/dev/{name}"));
                if whole.exists() {
                    return Some(whole);
                }
            }
        }
    }
    None
}

pub fn mount_duckypad_sd_only(disk_label: &str) -> Result<PathBuf, String> {
    if let Some(mounted) = find_mounted_duckypad_sd() {
        return Ok(mounted);
    }
    if let Some(dev_path) = detect_unmounted_duckypad_sd(disk_label) {
        let out = std::process::Command::new("udisksctl")
            .args([
                "mount",
                "--no-user-interaction",
                "-b",
                &dev_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| format!("Failed invoking udisksctl mount: {e}"))?;
        std::thread::sleep(std::time::Duration::from_millis(350));
        if let Some(mounted) = find_mounted_duckypad_sd() {
            return Ok(mounted);
        }
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if !stderr.is_empty() {
            return Err(format!("udisksctl mount failed for {}: {stderr}", dev_path.display()));
        }
    }

    // If a duckyPad Pro is connected over USB HID, switch it into USB MSC mode and mount
    let entered_msc = {
        let _guard = HID_IO_LOCK.lock().ok();
        if let Ok(device) = open_duckypad_hid() {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x18;
            tx[2] = HID_CMD_SOFTWARE_RESET;
            tx[3] = 1; // reboot into USB MSC mode
            device.write(&tx).is_ok()
        } else {
            false
        }
    };

    if entered_msc {
        std::thread::sleep(std::time::Duration::from_millis(800));
        let (mounted, _dev) = wait_and_mount_duckypad_msc(disk_label)?;
        return Ok(mounted);
    }

    Err("No unmounted duckyPad microSD card or connected duckyPad Pro detected to mount".to_string())
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
            // Self-heal if baseline snapshot exists and canonical fingerprint or diff count is 0
            if let Some(baseline) = load_sd_synced_snapshot(serial, sd_root) {
                let base_hash = compute_profiles_fingerprint(&baseline);
                if base_hash == mem_hash {
                    mark_profiles_sd_synced(serial, profiles);
                    return (
                        "synced".to_string(),
                        format!(
                            "Validated & Synced: All {} profile(s) match the duckyPad microSD card (hash #{}).",
                            profiles.len(),
                            &mem_hash[..8.min(mem_hash.len())]
                        ),
                    );
                }
                let diff_raw = compute_unsynced_changes_json(serial, profiles, sd_root, "unsynced");
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&diff_raw) {
                    if val.get("hasBaseline").and_then(|v| v.as_bool()) == Some(true)
                        && val.get("totalChanges").and_then(|v| v.as_u64()) == Some(0)
                    {
                        mark_profiles_sd_synced(serial, profiles);
                        return (
                            "synced".to_string(),
                            format!(
                                "Validated & Synced: All {} profile(s) match the duckyPad microSD card (hash #{}).",
                                profiles.len(),
                                &mem_hash[..8.min(mem_hash.len())]
                            ),
                        );
                    }
                }
            }
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
                .filter(|k| !k.is_effectively_empty())
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
                .filter(|k| !k.is_effectively_empty())
                .count();
            total_changes += 1;
            let detail_text = format!(
                "Added with {} configured key(s), Default RGB ({}, {}, {})",
                configured_keys, cur.bg_color[0], cur.bg_color[1], cur.bg_color[2]
            );
            items.push(serde_json::json!({
                "kind": "added",
                "profileNumber": cur.number,
                "profileName": cur.name,
                "summary": format!("New Profile #{}: \"{}\"", cur.number, cur.name),
                "details": [ detail_text.clone() ],
                "detailItems": [
                    {
                        "changeKey": "profile_added",
                        "text": detail_text
                    }
                ]
            }));
            continue;
        };

        let mut details: Vec<String> = Vec::new();
        let mut detail_items: Vec<serde_json::Value> = Vec::new();
        let mut push_detail = |change_key: &str, text: String| {
            details.push(text.clone());
            detail_items.push(serde_json::json!({
                "changeKey": change_key,
                "text": text
            }));
        };

        if base.number != cur.number {
            push_detail(
                "order",
                format!("Reordered slot: #{} → #{}", base.number, cur.number),
            );
        }
        if base.name != cur.name {
            push_detail(
                "name",
                format!("Renamed profile: \"{}\" → \"{}\"", base.name, cur.name),
            );
        }
        if base.bg_color != cur.bg_color {
            push_detail(
                "bg_color",
                format!(
                    "Default LED Color (BG_COLOR): ({}, {}, {}) → ({}, {}, {})",
                    base.bg_color[0],
                    base.bg_color[1],
                    base.bg_color[2],
                    cur.bg_color[0],
                    cur.bg_color[1],
                    cur.bg_color[2]
                ),
            );
        }
        if base.keydown_color != cur.keydown_color {
            push_detail(
                "keydown_color",
                format!(
                    "Keydown Color: {:?} → {:?}",
                    base.keydown_color, cur.keydown_color
                ),
            );
        }
        if base.dim_unused_keys != cur.dim_unused_keys {
            push_detail(
                "dim_unused_keys",
                format!(
                    "Dim Unused Keys: {} → {}",
                    base.dim_unused_keys, cur.dim_unused_keys
                ),
            );
        }

        let base_rot = base.oled_rotation_degrees();
        let cur_rot = cur.oled_rotation_degrees();
        if base_rot != cur_rot {
            push_detail(
                "is_landscape",
                format!("OLED Orientation: {}° → {}°", base_rot, cur_rot),
            );
        }

        let base_upper_hs = base.is_upper_hs();
        let cur_upper_hs = cur.is_upper_hs();
        if base_upper_hs != cur_upper_hs {
            push_detail(
                "is_upper_hs",
                format!(
                    "Inverted Orientation (IS_UPPER_HS): {} → {}",
                    if base_upper_hs { "ON" } else { "OFF" },
                    if cur_upper_hs { "ON" } else { "OFF" }
                ),
            );
        }

        let base_other = base.canonical_extra_config_lines();
        let cur_other = cur.canonical_extra_config_lines();
        if base_other != cur_other {
            push_detail(
                "preserved_config",
                format!(
                    "Custom config.txt directives modified ({} → {} line(s))",
                    base_other.len(),
                    cur_other.len()
                ),
            );
        }

        // Compare slots 1..=MAX_INPUT_SLOTS
        for slot_idx in 1..=MAX_INPUT_SLOTS {
            let bk = base.slots.get(&slot_idx).filter(|s| !s.is_effectively_empty());
            let ck = cur.slots.get(&slot_idx).filter(|s| !s.is_effectively_empty());
            let slot_key = format!("slot:{slot_idx}");
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
                    if b.widget_label_line1.trim() != c.widget_label_line1.trim()
                        || b.widget_label_line2.trim() != c.widget_label_line2.trim()
                    {
                        let old_wlbl = if b.widget_label_line2.trim().is_empty() {
                            b.widget_label_line1.trim().to_string()
                        } else {
                            format!("{} / {}", b.widget_label_line1.trim(), b.widget_label_line2.trim())
                        };
                        let new_wlbl = if c.widget_label_line2.trim().is_empty() {
                            c.widget_label_line1.trim().to_string()
                        } else {
                            format!("{} / {}", c.widget_label_line1.trim(), c.widget_label_line2.trim())
                        };
                        key_diffs.push(format!("widget label \"{}\" → \"{}\"", old_wlbl, new_wlbl));
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
                    if b.disable_autorepeat != c.disable_autorepeat {
                        key_diffs.push(format!(
                            "No Repeat on Hold: {} → {}",
                            if b.disable_autorepeat { "ON" } else { "OFF" },
                            if c.disable_autorepeat { "ON" } else { "OFF" }
                        ));
                    }
                    if b.allow_abort != c.allow_abort {
                        key_diffs.push(format!(
                            "Allow Abort: {} → {}",
                            if b.allow_abort { "ON" } else { "OFF" },
                            if c.allow_abort { "ON" } else { "OFF" }
                        ));
                    }
                    if !key_diffs.is_empty() {
                        let key_title = if !c.label_line1.is_empty() {
                            format!("Key {} ({})", slot_idx, c.label_line1)
                        } else if !b.label_line1.is_empty() {
                            format!("Key {} ({})", slot_idx, b.label_line1)
                        } else {
                            format!("Key {}", slot_idx)
                        };
                        push_detail(
                            &slot_key,
                            format!("{}: {}", key_title, key_diffs.join(", ")),
                        );
                    }
                }
                (None, Some(c)) => {
                    push_detail(
                        &slot_key,
                        format!("Key {} added (\"{}\")", slot_idx, c.label_line1),
                    );
                }
                (Some(b), None) => {
                    push_detail(
                        &slot_key,
                        format!("Key {} cleared (was \"{}\")", slot_idx, b.label_line1),
                    );
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
                "details": details,
                "detailItems": detail_items
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
            let detail_text = format!(
                "Profile folder {} will be removed from microSD card",
                base.folder_name()
            );
            items.push(serde_json::json!({
                "kind": "deleted",
                "profileNumber": base.number,
                "profileName": base.name,
                "summary": format!("Deleted Profile #{}: \"{}\"", base.number, base.name),
                "details": [ detail_text.clone() ],
                "detailItems": [
                    {
                        "changeKey": "profile_deleted",
                        "text": detail_text
                    }
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

pub fn discard_unsynced_change_in_profiles(
    serial: &str,
    profiles: &mut Vec<DuckyProfile>,
    sd_root: Option<&Path>,
    profile_number: u8,
    profile_name: &str,
    change_key: &str,
) -> Result<String, String> {
    let baseline = load_sd_synced_snapshot(serial, sd_root)
        .ok_or_else(|| "No microSD sync baseline snapshot found to revert against.".to_string())?;

    if change_key == "profile_added" {
        let idx = profiles
            .iter()
            .position(|p| p.number == profile_number && p.name == profile_name)
            .or_else(|| profiles.iter().position(|p| p.number == profile_number))
            .ok_or_else(|| format!("Profile #{profile_number} not found."))?;
        let removed = profiles.remove(idx);
        for (i, p) in profiles.iter_mut().enumerate() {
            p.number = (i + 1) as u8;
        }
        return Ok(format!(
            "Discarded new Profile #{}: \"{}\"",
            removed.number, removed.name
        ));
    }

    if change_key == "profile_deleted" {
        let base = baseline
            .iter()
            .find(|b| b.number == profile_number && b.name == profile_name)
            .or_else(|| baseline.iter().find(|b| b.name == profile_name))
            .or_else(|| baseline.iter().find(|b| b.number == profile_number))
            .ok_or_else(|| format!("Baseline Profile #{profile_number} not found."))?;
        let insert_pos = ((base.number as usize).saturating_sub(1)).min(profiles.len());
        profiles.insert(insert_pos, base.clone());
        for (i, p) in profiles.iter_mut().enumerate() {
            p.number = (i + 1) as u8;
        }
        return Ok(format!(
            "Restored deleted Profile #{}: \"{}\"",
            base.number, base.name
        ));
    }

    let cur_idx = profiles
        .iter()
        .position(|p| p.number == profile_number && p.name == profile_name)
        .or_else(|| profiles.iter().position(|p| p.number == profile_number))
        .or_else(|| profiles.iter().position(|p| p.name == profile_name))
        .ok_or_else(|| format!("Profile #{profile_number} not found."))?;

    let cur_num = profiles[cur_idx].number;
    let cur_name = profiles[cur_idx].name.clone();

    let base = baseline
        .iter()
        .find(|b| b.number == cur_num)
        .or_else(|| baseline.iter().find(|b| b.name == cur_name))
        .ok_or_else(|| format!("No baseline found for Profile #{cur_num} (\"{cur_name}\")."))?;

    if change_key == "profile_all" {
        let mut restored = base.clone();
        restored.number = cur_num;
        profiles[cur_idx] = restored;
        return Ok(format!(
            "Reverted all unsynced changes in Profile #{cur_num}: \"{}\"",
            base.name
        ));
    }

    if change_key == "order" {
        // Restore profile ordering to match baseline
        for p in profiles.iter_mut() {
            if let Some(b) = baseline.iter().find(|b| b.name == p.name) {
                p.number = b.number;
            }
        }
        profiles.sort_by_key(|p| p.number);
        for (i, p) in profiles.iter_mut().enumerate() {
            p.number = (i + 1) as u8;
        }
        return Ok(format!(
            "Reverted profile order for \"{}\" back to #{}",
            base.name, base.number
        ));
    }

    let cur = &mut profiles[cur_idx];
    match change_key {
        "name" => {
            cur.name = base.name.clone();
            Ok(format!(
                "Reverted Profile #{cur_num} name to \"{}\"",
                base.name
            ))
        }
        "bg_color" => {
            cur.bg_color = base.bg_color;
            Ok(format!(
                "Reverted Default LED Color on Profile #{cur_num} (\"{}\")",
                cur.name
            ))
        }
        "keydown_color" => {
            cur.keydown_color = base.keydown_color;
            Ok(format!(
                "Reverted Keydown Color on Profile #{cur_num} (\"{}\")",
                cur.name
            ))
        }
        "dim_unused_keys" => {
            cur.dim_unused_keys = base.dim_unused_keys;
            Ok(format!(
                "Reverted Dim Unused Keys on Profile #{cur_num} (\"{}\")",
                cur.name
            ))
        }
        "is_landscape" | "widget_rotation" | "is_upper_hs" | "preserved_config" => {
            cur.preserved_config_lines = base.preserved_config_lines.clone();
            Ok(format!(
                "Reverted Orientation & Hardware Directives on Profile #{cur_num} (\"{}\")",
                cur.name
            ))
        }
        other if other.starts_with("slot:") => {
            let slot_idx: usize = other
                .strip_prefix("slot:")
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| format!("Invalid slot key: {other}"))?;
            if let Some(base_slot) = base.slots.get(&slot_idx) {
                cur.slots.insert(slot_idx, base_slot.clone());
            } else {
                cur.slots.remove(&slot_idx);
            }
            Ok(format!(
                "Reverted Key {} in Profile #{cur_num} (\"{}\")",
                slot_idx, cur.name
            ))
        }
        _ => Err(format!("Unknown change key: {change_key}")),
    }
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
            record_mounted_sd_space(serial, &already_mounted);
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

    let already_has_block_dev = find_duckypad_msc_block_device(disk_label, true).is_some();
    if !already_has_block_dev {
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
    } else {
        on_progress(
            20,
            "Step 1/4: Detected unmounted duckyPad microSD block device...",
            "• Mounting duckyPad microSD partition via udisksctl...",
        );
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
                    record_mounted_sd_space(serial, &mount_root);
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
        record_mounted_sd_space(serial, &already_mounted);
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
                record_mounted_sd_space(serial, &mount_root);
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

    let update_available = if is_duckydeck_studio_firmware_version(current_version) {
        remote_semver > (3, 3, 5)
    } else {
        match parse_semver(current_version) {
            Some(curr_semver) => remote_semver > curr_semver,
            None => false,
        }
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

fn default_true_bool() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareVersionEntry {
    pub version: String,
    pub filename: String,
    pub path: String,
    pub crc32: String,
    pub size_bytes: u64,
    pub vendor: String,
    pub is_official: bool,
    pub channel: String,
    pub release_date: String,
    pub notes: String,
    pub is_installed: bool,
    pub supports_monochrome_icons: bool,
    #[serde(default)]
    pub download_url: String,
    #[serde(default = "default_true_bool")]
    pub is_downloaded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRemoteFirmwareItem {
    pub version: String,
    pub filename: String,
    pub crc32: String,
    pub size_bytes: u64,
    pub download_url: String,
    pub release_date: String,
    pub notes: String,
}

pub fn official_firmware_cache_dir() -> PathBuf {
    data_home_dir()
        .join("duckydeck-studio")
        .join("firmware_official")
}

pub fn official_firmware_manifest_path() -> PathBuf {
    official_firmware_cache_dir().join("manifest.json")
}

pub fn load_cached_remote_firmwares() -> Vec<CachedRemoteFirmwareItem> {
    let path = official_firmware_manifest_path();
    let Ok(raw) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<CachedRemoteFirmwareItem>>(&raw).unwrap_or_default()
}

pub fn save_cached_remote_firmwares(items: &[CachedRemoteFirmwareItem]) {
    let dir = official_firmware_cache_dir();
    if fs::create_dir_all(&dir).is_ok() {
        if let Ok(json) = serde_json::to_string_pretty(items) {
            let _ = fs::write(official_firmware_manifest_path(), json);
        }
    }
}

pub fn is_duckydeck_studio_firmware_version(version: &str) -> bool {
    let trimmed = version.trim();
    if trimmed.is_empty() || trimmed == "-" {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    if lower.contains("official") || lower.contains("dekunukem") {
        return false;
    }
    if lower.contains("studio") || lower.contains("dds") {
        return true;
    }
    let clean = trimmed.trim_start_matches(['v', 'V']);
    clean == "1.0.0" || clean == "3.4.0"
}

pub fn compute_ieee_crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn firmware_release_metadata(
    version: &str,
    is_studio_custom: bool,
) -> (&'static str, &'static str, &'static str) {
    if is_studio_custom {
        return (
            "DuckyDeck Studio Custom (v1.0.0)",
            "Oct 17, 2026",
            "DuckyDeck Studio custom firmware v1.0.0 (built on duckyPad Pro 3.3.5 / DSVM v2): adds native 1-bit monochrome pixel-art OLED icons (16×16 full-cell, 10×10 top-icon + caption, and 7×10 / 6×8 inline glyphs), 0°/90°/180°/270° OLED orientation, UTF-8 key label decoding, and DuckyDeck Studio HID identification.",
        );
    }
    match version {
        "3.3.5" => (
            "Official Release",
            "Mar 15, 2026",
            "Official dekuNukem release: increased Anykey-to-Abort check frequency during script execution.",
        ),
        "3.3.4" => (
            "Official Release",
            "Mar 11, 2026",
            "Official dekuNukem release: adjusted PWM default level to reduce power consumption.",
        ),
        "3.3.3" => (
            "Official Release",
            "Feb 9, 2026",
            "Official dekuNukem release: disabled PWM output; stable public release.",
        ),
        "3.3.0" => (
            "Official Release",
            "Jan 2026",
            "Official dekuNukem release: DSVM v2 bytecode engine and expanded HID command support.",
        ),
        _ => (
            "Official dekuNukem Firmware",
            "Official Release",
            "Official dekuNukem duckyPad Pro ESP32-S3 OTA firmware binary from dekuNukem's repository.",
        ),
    }
}

pub fn list_available_duckypad_pro_firmwares(current_version: &str) -> Vec<FirmwareVersionEntry> {
    let mut candidate_dirs: Vec<(PathBuf, bool)> = Vec::new();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    candidate_dirs.push((manifest_dir.join("firmware/releases"), true));
    candidate_dirs.push((manifest_dir.join("firmware/duckyPad-Pro/firmware"), true));
    candidate_dirs.push((official_firmware_cache_dir(), false));
    if let Some(home) = std::env::var_os("HOME") {
        candidate_dirs.push((
            PathBuf::from(&home).join(".local/share/duckydeck-studio/firmware"),
            false,
        ));
    }

    let mut by_key: std::collections::BTreeMap<(u8, u32, u32, u32, String), FirmwareVersionEntry> =
        std::collections::BTreeMap::new();

    for (dir, is_bundled_studio_dir) in candidate_dirs {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(fname) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !fname.starts_with("DPP_FW_") || !fname.ends_with(".bin") {
                continue;
            }
            let inner = &fname["DPP_FW_".len()..fname.len() - ".bin".len()];
            let mut parts = inner.split('_');
            let Some(ver_str) = parts.next() else {
                continue;
            };
            let crc_str = parts.next().unwrap_or("").to_lowercase();
            let Some((maj, min, pat)) = parse_semver(ver_str) else {
                continue;
            };
            if ver_str == "3.4.0" {
                continue;
            }
            let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let is_studio_custom = is_bundled_studio_dir
                || matches!(crc_str.as_str(), "ad2acc81");
            let is_official = !is_studio_custom;
            let (channel, release_date, notes) =
                firmware_release_metadata(ver_str, is_studio_custom);
            let vendor = if is_official {
                "dekuNukem (Official)".to_string()
            } else {
                "DuckyDeck Studio".to_string()
            };
            let supports_monochrome_icons = is_studio_custom;
            let is_installed = if is_studio_custom {
                is_duckydeck_studio_firmware_version(current_version)
                    && (current_version.trim().trim_start_matches(['v', 'V']) == ver_str
                        || current_version.trim() == "3.4.0")
            } else {
                !is_duckydeck_studio_firmware_version(current_version)
                    && current_version.trim().starts_with(ver_str)
            };

            let studio_rank = if is_studio_custom { 1u8 } else { 0u8 };
            let key = (studio_rank, maj, min, pat, crc_str.clone());
            by_key.entry(key).or_insert_with(|| FirmwareVersionEntry {
                version: ver_str.to_string(),
                filename: fname.to_string(),
                path: path.to_string_lossy().to_string(),
                crc32: crc_str,
                size_bytes,
                vendor,
                is_official,
                channel: channel.to_string(),
                release_date: release_date.to_string(),
                notes: notes.to_string(),
                is_installed,
                supports_monochrome_icons,
                download_url: String::new(),
                is_downloaded: true,
            });
        }
    }

    // Merge remote official firmwares fetched from dekuNukem's site
    let official_dir = official_firmware_cache_dir();
    for remote in load_cached_remote_firmwares() {
        let Some((maj, min, pat)) = parse_semver(&remote.version) else {
            continue;
        };
        let crc_lower = remote.crc32.to_lowercase();
        let key = (0u8, maj, min, pat, crc_lower.clone());
        let local_path = official_dir.join(&remote.filename);
        let is_downloaded = local_path.is_file();
        let size_bytes = if is_downloaded {
            local_path.metadata().map(|m| m.len()).unwrap_or(remote.size_bytes)
        } else {
            remote.size_bytes
        };
        let (default_channel, default_date, default_notes) =
            firmware_release_metadata(&remote.version, false);
        let is_installed = !is_duckydeck_studio_firmware_version(current_version)
            && current_version.trim().starts_with(&remote.version);

        by_key
            .entry(key)
            .and_modify(|existing| {
                if existing.download_url.is_empty() && !remote.download_url.is_empty() {
                    existing.download_url = remote.download_url.clone();
                }
            })
            .or_insert_with(|| FirmwareVersionEntry {
                version: remote.version.clone(),
                filename: remote.filename.clone(),
                path: if is_downloaded {
                    local_path.to_string_lossy().to_string()
                } else {
                    String::new()
                },
                crc32: crc_lower,
                size_bytes,
                vendor: "dekuNukem (Official)".to_string(),
                is_official: true,
                channel: default_channel.to_string(),
                release_date: if remote.release_date.trim().is_empty() {
                    default_date.to_string()
                } else {
                    remote.release_date.clone()
                },
                notes: if remote.notes.trim().is_empty() {
                    default_notes.to_string()
                } else {
                    remote.notes.clone()
                },
                is_installed,
                supports_monochrome_icons: false,
                download_url: remote.download_url.clone(),
                is_downloaded,
            });
    }

    let mut out: Vec<FirmwareVersionEntry> = by_key.into_values().collect();
    out.reverse();
    out
}

pub async fn fetch_official_firmwares_from_site(
    current_version: &str,
) -> Result<Vec<FirmwareVersionEntry>, String> {
    let client = reqwest::Client::builder()
        .user_agent("DuckyDeck-Studio/0.1.0")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("HTTP client init failed: {e}"))?;

    let mut discovered: std::collections::BTreeMap<(u32, u32, u32, String), CachedRemoteFirmwareItem> =
        std::collections::BTreeMap::new();

    // Seed with existing cached entries so we never lose previously discovered items
    for item in load_cached_remote_firmwares() {
        if let Some((maj, min, pat)) = parse_semver(&item.version) {
            discovered.insert((maj, min, pat, item.crc32.to_lowercase()), item);
        }
    }

    let mut parse_and_insert = |fname: &str, dl_url: &str, size_bytes: u64, date_hint: &str, note_hint: &str| {
        if !fname.starts_with("DPP_FW_") || !fname.ends_with(".bin") {
            return;
        }
        let inner = &fname["DPP_FW_".len()..fname.len() - ".bin".len()];
        let mut parts = inner.split('_');
        let Some(ver_str) = parts.next() else {
            return;
        };
        let crc_str = parts.next().unwrap_or("").to_lowercase();
        if crc_str.is_empty() || crc_str == "ad2acc81" {
            return;
        }
        let Some((maj, min, pat)) = parse_semver(ver_str) else {
            return;
        };
        let (_, meta_date, meta_notes) = firmware_release_metadata(ver_str, false);
        let release_date = if !date_hint.trim().is_empty() {
            date_hint.trim().to_string()
        } else {
            meta_date.to_string()
        };
        let notes = if !note_hint.trim().is_empty() {
            note_hint.trim().to_string()
        } else {
            meta_notes.to_string()
        };
        discovered.insert(
            (maj, min, pat, crc_str.clone()),
            CachedRemoteFirmwareItem {
                version: ver_str.to_string(),
                filename: fname.to_string(),
                crc32: crc_str,
                size_bytes,
                download_url: dl_url.to_string(),
                release_date,
                notes,
            },
        );
    };

    // 1. Fetch active firmware directory from dekuNukem/duckyPad-Pro
    let contents_url = "https://api.github.com/repos/dekuNukem/duckyPad-Pro/contents/firmware?ref=master";
    let resp = client
        .get(contents_url)
        .send()
        .await
        .map_err(|e| format!("Failed reaching dekuNukem firmware repository: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Official firmware repository returned HTTP {}",
            resp.status()
        ));
    }

    if let Ok(entries) = resp.json::<serde_json::Value>().await {
        if let Some(arr) = entries.as_array() {
            for item in arr {
                let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if item_type != "file" {
                    continue;
                }
                let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let size = item.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
                let dl_url = item
                    .get("download_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !name.is_empty() && !dl_url.is_empty() {
                    parse_and_insert(name, dl_url, size, "", "");
                }
            }
        }
    }

    // 2. Also check GitHub Releases for any attached DPP_FW_*.bin assets
    let releases_url = "https://api.github.com/repos/dekuNukem/duckyPad-Pro/releases?per_page=20";
    if let Ok(rel_resp) = client.get(releases_url).send().await {
        if rel_resp.status().is_success() {
            if let Ok(rel_json) = rel_resp.json::<serde_json::Value>().await {
                if let Some(rel_arr) = rel_json.as_array() {
                    for rel in rel_arr {
                        let published = rel
                            .get("published_at")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.get(..10))
                            .unwrap_or("");
                        let body = rel
                            .get("body")
                            .and_then(|v| v.as_str())
                            .map(|s| s.lines().next().unwrap_or("").trim())
                            .unwrap_or("");
                        if let Some(assets) = rel.get("assets").and_then(|v| v.as_array()) {
                            for asset in assets {
                                let name = asset.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                let size = asset.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
                                let dl_url = asset
                                    .get("browser_download_url")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                if !name.is_empty() && !dl_url.is_empty() {
                                    parse_and_insert(name, dl_url, size, published, body);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Also inspect recent commits to `/firmware` in dekuNukem/duckyPad-Pro so historical versions in git history are listed
    let commits_url = "https://api.github.com/repos/dekuNukem/duckyPad-Pro/commits?path=firmware&per_page=10";
    if let Ok(com_resp) = client.get(commits_url).send().await {
        if com_resp.status().is_success() {
            if let Ok(com_json) = com_resp.json::<serde_json::Value>().await {
                if let Some(com_arr) = com_json.as_array() {
                    for com in com_arr.iter().take(6) {
                        let sha = com.get("sha").and_then(|v| v.as_str()).unwrap_or("");
                        if sha.is_empty() {
                            continue;
                        }
                        let commit_date = com
                            .get("commit")
                            .and_then(|c| c.get("committer"))
                            .and_then(|cm| cm.get("date"))
                            .and_then(|d| d.as_str())
                            .and_then(|s| s.get(..10))
                            .unwrap_or("");
                        let commit_msg = com
                            .get("commit")
                            .and_then(|c| c.get("message"))
                            .and_then(|m| m.as_str())
                            .map(|s| s.lines().next().unwrap_or("").trim())
                            .unwrap_or("");
                        let tree_url = format!(
                            "https://api.github.com/repos/dekuNukem/duckyPad-Pro/contents/firmware?ref={sha}"
                        );
                        if let Ok(t_resp) = client.get(&tree_url).send().await {
                            if t_resp.status().is_success() {
                                if let Ok(t_json) = t_resp.json::<serde_json::Value>().await {
                                    if let Some(t_arr) = t_json.as_array() {
                                        for item in t_arr {
                                            let name = item
                                                .get("name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("");
                                            let size = item
                                                .get("size")
                                                .and_then(|v| v.as_u64())
                                                .unwrap_or(0);
                                            let dl_url = item
                                                .get("download_url")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("");
                                            if !name.is_empty() && !dl_url.is_empty() {
                                                let note = if !commit_msg.is_empty() {
                                                    format!("Official dekuNukem release ({commit_msg})")
                                                } else {
                                                    String::new()
                                                };
                                                parse_and_insert(
                                                    name,
                                                    dl_url,
                                                    size,
                                                    commit_date,
                                                    &note,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let remote_list: Vec<CachedRemoteFirmwareItem> = discovered.into_values().collect();
    save_cached_remote_firmwares(&remote_list);

    Ok(list_available_duckypad_pro_firmwares(current_version))
}

pub async fn download_official_firmware_binary(
    filename: &str,
    download_url: &str,
) -> Result<PathBuf, String> {
    let clean_name = filename.trim();
    if !clean_name.starts_with("DPP_FW_") || !clean_name.ends_with(".bin") || clean_name.contains('/') || clean_name.contains('\\') {
        return Err(format!("Invalid official firmware filename: '{clean_name}'"));
    }
    let dir = official_firmware_cache_dir();
    fs::create_dir_all(&dir)
        .map_err(|e| format!("Failed creating firmware cache directory {}: {e}", dir.display()))?;
    let target_path = dir.join(clean_name);

    // Extract expected CRC32 from filename `DPP_FW_<ver>_<crc32>.bin`
    let inner = &clean_name["DPP_FW_".len()..clean_name.len() - ".bin".len()];
    let expected_crc_hex = inner.split('_').nth(1).unwrap_or("").to_lowercase();

    // If already downloaded and CRC matches, return immediately
    if target_path.is_file() {
        if let Ok(existing_bytes) = fs::read(&target_path) {
            if existing_bytes.len() > 64 * 1024 {
                let crc = format!("{:08x}", compute_ieee_crc32(&existing_bytes));
                if expected_crc_hex.is_empty() || crc == expected_crc_hex {
                    return Ok(target_path);
                }
            }
        }
    }

    let url = if !download_url.trim().is_empty() {
        download_url.trim().to_string()
    } else {
        format!(
            "https://raw.githubusercontent.com/dekuNukem/duckyPad-Pro/master/firmware/{clean_name}"
        )
    };

    let client = reqwest::Client::builder()
        .user_agent("DuckyDeck-Studio/0.1.0")
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed downloading {clean_name} from {url}: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Download of {clean_name} failed with HTTP {}",
            resp.status()
        ));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed reading firmware bytes: {e}"))?;

    if bytes.len() < 64 * 1024 {
        return Err(format!(
            "Downloaded firmware file is unexpectedly small ({} bytes)",
            bytes.len()
        ));
    }

    let actual_crc = format!("{:08x}", compute_ieee_crc32(&bytes));
    if !expected_crc_hex.is_empty() && actual_crc != expected_crc_hex {
        return Err(format!(
            "CRC32 verification failed for {clean_name}: expected {expected_crc_hex}, got {actual_crc}"
        ));
    }

    fs::write(&target_path, &bytes)
        .map_err(|e| format!("Failed saving firmware to {}: {e}", target_path.display()))?;

    Ok(target_path)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub is_builtin: bool,
    pub profile: DuckyProfile,
}

fn user_profile_templates_path() -> PathBuf {
    data_home_dir()
        .join("duckydeck-studio")
        .join("profile_templates.json")
}

pub fn builtin_profile_templates() -> Vec<ProfileTemplate> {
    let mut list = Vec::new();

    // 1. Blank Starter Template
    list.push(ProfileTemplate {
        id: "builtin_blank".to_string(),
        name: "Blank Profile".to_string(),
        description: "Clean empty 20-key profile with sky-blue default LED lighting and 0° portrait orientation.".to_string(),
        category: "Starter".to_string(),
        is_builtin: true,
        profile: DuckyProfile {
            number: 1,
            name: "New Profile".to_string(),
            bg_color: [16, 48, 96],
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            ..Default::default()
        },
    });

    // 2. Media & Productivity Template
    let mut media_slots = BTreeMap::new();
    let add_slot = |map: &mut BTreeMap<usize, DuckyKeySlot>,
                        slot: usize,
                        l1: &str,
                        l2: &str,
                        wl1: &str,
                        wl2: &str,
                        rgb: Option<[u8; 3]>,
                        script: &str| {
        map.insert(
            slot,
            DuckyKeySlot {
                slot,
                label_line1: l1.to_string(),
                label_line2: l2.to_string(),
                widget_label_line1: wl1.to_string(),
                widget_label_line2: wl2.to_string(),
                custom_rgb: rgb,
                allow_abort: false,
                disable_autorepeat: false,
                on_press_script: script.to_string(),
                on_release_script: String::new(),
            },
        );
    };
    add_slot(&mut media_slots, 1, "Copy", "", "Copy", "Ctrl+C", Some([0, 180, 120]), "CONTROL c\n");
    add_slot(&mut media_slots, 2, "Paste", "", "Paste", "Ctrl+V", Some([0, 180, 120]), "CONTROL v\n");
    add_slot(&mut media_slots, 3, "Undo", "", "Undo", "Ctrl+Z", Some([240, 140, 0]), "CONTROL z\n");
    add_slot(&mut media_slots, 4, "Redo", "", "Redo", "Ctrl+Shift+Z", Some([240, 140, 0]), "CONTROL SHIFT z\n");
    add_slot(&mut media_slots, 5, "Prev", "Track", "Previous", "Media Track", Some([56, 189, 248]), "MK_PREV\n");
    add_slot(&mut media_slots, 6, "Play", "Pause", "Play / Pause", "Media", Some([34, 197, 94]), "MK_PP\n");
    add_slot(&mut media_slots, 7, "Next", "Track", "Next", "Media Track", Some([56, 189, 248]), "MK_NEXT\n");
    add_slot(&mut media_slots, 8, "Mute", "", "Mute Audio", "Toggle", Some([239, 68, 68]), "MK_MUTE\n");
    add_slot(&mut media_slots, 21, "Vol-", "", "Volume Down", "", None, "MK_VOLDOWN\n");
    add_slot(&mut media_slots, 22, "Vol+", "", "Volume Up", "", None, "MK_VOLUP\n");
    add_slot(&mut media_slots, 23, "Mute", "", "Mute", "", None, "MK_MUTE\n");

    list.push(ProfileTemplate {
        id: "builtin_media_productivity".to_string(),
        name: "Media & Productivity".to_string(),
        description: "Clipboard shortcuts (Copy, Paste, Undo, Redo), media transport controls, and rotary volume knob.".to_string(),
        category: "Productivity".to_string(),
        is_builtin: true,
        profile: DuckyProfile {
            number: 1,
            name: "Productivity".to_string(),
            bg_color: [16, 48, 96],
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            slots: media_slots,
            ..Default::default()
        },
    });

    // 3. Developer / IDE Template
    let mut dev_slots = BTreeMap::new();
    add_slot(&mut dev_slots, 1, "Build", "", "Build Project", "Ctrl+B", Some([34, 197, 94]), "CONTROL b\n");
    add_slot(&mut dev_slots, 2, "Run", "", "Run Target", "Shift+F10", Some([16, 185, 129]), "SHIFT F10\n");
    add_slot(&mut dev_slots, 3, "Debug", "", "Start Debug", "Shift+F9", Some([245, 158, 11]), "SHIFT F9\n");
    add_slot(&mut dev_slots, 4, "Stop", "", "Stop Process", "Ctrl+F2", Some([239, 68, 68]), "CONTROL F2\n");
    add_slot(&mut dev_slots, 5, "Find", "All", "Find in Files", "Ctrl+Shift+F", Some([56, 189, 248]), "CONTROL SHIFT f\n");
    add_slot(&mut dev_slots, 6, "Fmt", "Code", "Format Code", "Ctrl+Shift+I", Some([168, 85, 247]), "CONTROL SHIFT i\n");
    add_slot(&mut dev_slots, 7, "Term", "", "Toggle Terminal", "Ctrl+`", Some([148, 163, 184]), "CONTROL `\n");
    add_slot(&mut dev_slots, 8, "Git", "Diff", "Git Commit/Diff", "Ctrl+K", Some([249, 115, 22]), "CONTROL k\n");

    list.push(ProfileTemplate {
        id: "builtin_developer_ide".to_string(),
        name: "Developer / IDE".to_string(),
        description: "Build, Run, Debug, Stop, Find in Files, Format Code, Terminal, and Git workflow keys.".to_string(),
        category: "Development".to_string(),
        is_builtin: true,
        profile: DuckyProfile {
            number: 1,
            name: "Dev IDE".to_string(),
            bg_color: [20, 36, 72],
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            slots: dev_slots,
            ..Default::default()
        },
    });

    // 4. Landscape 90° Starter Template
    let mut landscape_prof = DuckyProfile {
        number: 1,
        name: "Landscape 90".to_string(),
        bg_color: [16, 64, 96],
        keydown_color: Some([255, 255, 255]),
        dim_unused_keys: true,
        ..Default::default()
    };
    landscape_prof.set_oled_rotation_degrees(90);
    landscape_prof.set_widget_rotation_degrees(-1);

    list.push(ProfileTemplate {
        id: "builtin_landscape_90".to_string(),
        name: "Landscape 90° Starter".to_string(),
        description: "Pre-configured for a 90° rotated duckyPad Pro (5×4 landscape key grid on OLED and Desktop Widget).".to_string(),
        category: "Orientation".to_string(),
        is_builtin: true,
        profile: landscape_prof,
    });

    list
}

pub fn load_user_profile_templates() -> Vec<ProfileTemplate> {
    let path = user_profile_templates_path();
    let Ok(raw) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<ProfileTemplate>>(&raw).unwrap_or_default()
}

pub fn save_user_profile_templates(templates: &[ProfileTemplate]) -> Result<(), String> {
    let path = user_profile_templates_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed creating templates dir: {e}"))?;
    }
    let json = serde_json::to_string_pretty(templates)
        .map_err(|e| format!("Failed serializing profile templates: {e}"))?;
    fs::write(&path, json).map_err(|e| format!("Failed writing profile templates: {e}"))
}

pub fn list_all_profile_templates() -> Vec<ProfileTemplate> {
    let mut all = builtin_profile_templates();
    let user = load_user_profile_templates();
    all.extend(user);
    all
}

pub fn save_profile_as_user_template(
    profile: &DuckyProfile,
    template_name: &str,
    description: &str,
) -> Result<ProfileTemplate, String> {
    let clean_name = template_name.trim();
    if clean_name.is_empty() {
        return Err("Template name cannot be empty".to_string());
    }
    let slug: String = clean_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    let template_id = format!("user_{}", slug.trim_matches('_'));
    let mut prof_copy = profile.clone();
    prof_copy.number = 1;
    prof_copy.folder_name_override = None;

    let desc = if description.trim().is_empty() {
        let key_count = prof_copy
            .slots
            .values()
            .filter(|s| !s.is_effectively_empty())
            .count();
        format!(
            "Saved from Profile \"{}\" ({} configured key(s), OLED {}°)",
            profile.name,
            key_count,
            prof_copy.oled_rotation_degrees()
        )
    } else {
        description.trim().to_string()
    };

    let entry = ProfileTemplate {
        id: template_id.clone(),
        name: clean_name.to_string(),
        description: desc,
        category: "User Templates".to_string(),
        is_builtin: false,
        profile: prof_copy,
    };

    let mut user_list = load_user_profile_templates();
    if let Some(existing) = user_list
        .iter_mut()
        .find(|t| t.id == template_id || t.name.eq_ignore_ascii_case(clean_name))
    {
        *existing = entry.clone();
    } else {
        user_list.push(entry.clone());
    }
    save_user_profile_templates(&user_list)?;
    Ok(entry)
}

pub fn delete_user_profile_template(template_id: &str) -> Result<(), String> {
    let clean_id = template_id.trim();
    if clean_id.starts_with("builtin_") {
        return Err("Built-in starter templates cannot be deleted".to_string());
    }
    let mut user_list = load_user_profile_templates();
    let before = user_list.len();
    user_list.retain(|t| t.id != clean_id);
    if user_list.len() == before {
        return Err(format!("Template '{clean_id}' not found"));
    }
    save_user_profile_templates(&user_list)
}

pub fn instantiate_profile_from_template(
    template_id: &str,
    number: u8,
    profile_name: &str,
) -> DuckyProfile {
    let all = list_all_profile_templates();
    if let Some(tpl) = all.iter().find(|t| t.id == template_id) {
        let mut p = tpl.profile.clone();
        p.number = number;
        p.name = profile_name.to_string();
        p.folder_name_override = Some(format!("profile{number}_{profile_name}"));
        p
    } else {
        DuckyProfile {
            number,
            name: profile_name.to_string(),
            folder_name_override: Some(format!("profile{number}_{profile_name}")),
            ..Default::default()
        }
    }
}

fn write_and_verify_firmware_on_sd(
    sd_root: &Path,
    target_filename: &str,
    fw_bytes: &[u8],
    expected_crc: u32,
) -> Result<PathBuf, String> {
    if let Ok(rd) = fs::read_dir(sd_root) {
        for entry in rd.flatten() {
            let p = entry.path();
            if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("DPP_FW_") && name.ends_with(".bin") {
                    let _ = fs::remove_file(&p);
                }
            }
        }
    }

    let dest_path = sd_root.join(target_filename);
    {
        let mut f = fs::File::create(&dest_path).map_err(|e| {
            format!(
                "Failed creating firmware file {} on microSD: {e}",
                dest_path.display()
            )
        })?;
        f.write_all(fw_bytes).map_err(|e| {
            format!(
                "Failed writing firmware bytes to {}: {e}",
                dest_path.display()
            )
        })?;
        f.sync_all()
            .map_err(|e| format!("Failed flushing firmware file to microSD: {e}"))?;
    }

    let read_back = fs::read(&dest_path).map_err(|e| {
        format!(
            "Failed reading back staged firmware {} for verification: {e}",
            dest_path.display()
        )
    })?;
    if read_back.len() != fw_bytes.len() {
        return Err(format!(
            "Staged firmware size mismatch on microSD (expected {} bytes, got {} bytes)",
            fw_bytes.len(),
            read_back.len()
        ));
    }
    let read_crc = compute_ieee_crc32(&read_back);
    if read_crc != expected_crc {
        return Err(format!(
            "Staged firmware CRC32 mismatch on microSD (expected {:08x}, got {:08x})",
            expected_crc, read_crc
        ));
    }

    Ok(dest_path)
}

pub fn stage_duckypad_pro_firmware_via_msc_with_progress<F>(
    disk_label: &str,
    fw_source_path: &Path,
    mut on_progress: F,
) -> Result<String, String>
where
    F: FnMut(i32, &str, &str),
{
    let clean_str = fw_source_path
        .to_string_lossy()
        .trim()
        .trim_start_matches("file://")
        .to_string();
    let src_path = PathBuf::from(&clean_str);
    if !src_path.is_file() {
        return Err(format!(
            "Firmware binary file not found: {}",
            src_path.display()
        ));
    }

    let fw_bytes = fs::read(&src_path)
        .map_err(|e| format!("Failed reading firmware binary {}: {e}", src_path.display()))?;
    if fw_bytes.len() < 64 * 1024 {
        return Err(format!(
            "File {} is too small ({} bytes) to be a valid duckyPad Pro firmware image",
            src_path.display(),
            fw_bytes.len()
        ));
    }

    let crc = compute_ieee_crc32(&fw_bytes);
    let crc_hex = format!("{crc:x}");
    let raw_name = src_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("DPP_FW_custom.bin");

    let (ver_label, target_filename) = if raw_name.starts_with("DPP_FW_")
        && raw_name.ends_with(".bin")
        && raw_name.to_lowercase().contains(&crc_hex)
    {
        let ver = raw_name
            .strip_prefix("DPP_FW_")
            .and_then(|s| s.strip_suffix(".bin"))
            .and_then(|s| s.split('_').next())
            .unwrap_or("custom")
            .to_string();
        (ver, raw_name.to_string())
    } else {
        let ver = raw_name
            .strip_prefix("DPP_FW_")
            .and_then(|s| s.strip_suffix(".bin"))
            .and_then(|s| s.split('_').next())
            .unwrap_or("3.4.0")
            .to_string();
        (ver.clone(), format!("DPP_FW_{ver}_{crc_hex}.bin"))
    };

    on_progress(
        10,
        &format!("Validating duckyPad Pro firmware v{ver_label} (CRC32: {crc_hex})..."),
        &format!(
            "• Verified {} ({} KB, IEEE CRC32: 0x{crc_hex})\n• Target OTA image name: {target_filename}",
            src_path.display(),
            fw_bytes.len() / 1024
        ),
    );

    if let Some(already_mounted) = find_mounted_duckypad_sd() {
        on_progress(
            45,
            &format!("Staging {target_filename} to mounted microSD card..."),
            &format!(
                "• Found mounted microSD at {}\n• Removing old DPP_FW_*.bin and writing {target_filename}...",
                already_mounted.display()
            ),
        );
        let staged_path =
            write_and_verify_firmware_on_sd(&already_mounted, &target_filename, &fw_bytes, crc)?;
        let _ = std::process::Command::new("sync").status();
        let rebooted = reboot_duckypad().is_ok();
        let msg = if rebooted {
            format!(
                "Staged {target_filename} ({}) and rebooted duckyPad Pro — press ANY KEY on your duckyPad Pro to complete the v{ver_label} update!",
                staged_path.display()
            )
        } else {
            format!(
                "Staged {target_filename} to {} — safely eject/insert the microSD into your duckyPad Pro and press ANY KEY on the keypad to install v{ver_label}!",
                staged_path.display()
            )
        };
        on_progress(100, &msg, &format!("✓ {msg}"));
        return Ok(msg);
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
        20,
        "Step 1/5: Switching duckyPad Pro to USB Mass Storage mode...",
        &format!("• Sending USB MSC command (0x14) to duckyPad Pro ({disk_label})..."),
    );

    {
        let _guard = HID_IO_LOCK.lock().ok();
        let dev = open_duckypad_hid()?;
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x13;
        tx[2] = HID_CMD_SOFTWARE_RESET;
        tx[3] = 1;
        dev.write(&tx)
            .map_err(|e| format!("Failed to switch duckyPad Pro into USB storage mode: {e}"))?;
    }

    on_progress(
        38,
        "Step 2/5: Waiting for FAT32 partition & mounting microSD...",
        "• Waiting for duckyPad Pro microSD partition to enumerate on USB bus...",
    );

    let mount_res = wait_and_mount_duckypad_msc(disk_label);
    let stage_res: Result<PathBuf, String> = match mount_res {
        Ok((mount_root, real_dev)) => {
            on_progress(
                60,
                &format!("Step 3/5: Writing {target_filename} & verifying CRC32 on microSD..."),
                &format!(
                    "✓ Mounted microSD at {}\n• Writing {} bytes and verifying read-back CRC32 (0x{crc_hex})...",
                    mount_root.display(),
                    fw_bytes.len()
                ),
            );
            let mut res =
                write_and_verify_firmware_on_sd(&mount_root, &target_filename, &fw_bytes, crc);
            if res.is_ok() {
                on_progress(
                    80,
                    "Step 4/5: CRC32 verified! Flushing FAT32 buffers & unmounting...",
                    &format!("✓ Read-back CRC32 (0x{crc_hex}) verified on microSD; unmounting cleanly..."),
                );
            }
            if let Err(unmount_err) = unmount_duckypad_sd_cleanly(&mount_root, real_dev.as_deref()) {
                if res.is_ok() {
                    res = Err(unmount_err);
                }
            } else if res.is_ok() {
                on_progress(
                    92,
                    "Step 5/5: Rebooting duckyPad Pro into OTA Firmware Updater...",
                    "✓ Unmounted cleanly; rebooting duckyPad Pro into OTA update prompt...",
                );
            }
            res
        }
        Err(e) => Err(e),
    };

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

    drop(_msc_guard);

    match stage_res {
        Ok(_) => {
            let msg = format!(
                "Firmware v{ver_label} ({target_filename}) staged to duckyPad Pro! Press ANY KEY on your duckyPad Pro to confirm and install v{ver_label}."
            );
            on_progress(
                100,
                &format!("Ready on duckyPad Pro: Press ANY KEY on the device to install v{ver_label}!"),
                &format!("✓ {msg}"),
            );
            Ok(msg)
        }
        Err(e) => Err(e),
    }
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
pub struct WebApiDestination {
    pub name: String,
    #[serde(default = "default_web_api_method")]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: String,
}

pub fn web_api_destinations_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        });
    base.join("duckydeck-studio").join("web_api_destinations.json")
}

pub fn load_web_api_destinations() -> Vec<WebApiDestination> {
    let path = web_api_destinations_path();
    let Ok(raw) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let mut list: Vec<WebApiDestination> = serde_json::from_str(&raw).unwrap_or_default();
    for item in &mut list {
        item.name = item.name.trim().to_string();
        item.method = item.method.trim().to_ascii_uppercase();
        if item.method.is_empty() {
            item.method = "POST".to_string();
        }
        item.url = item.url.trim().to_string();
        item.headers = item.headers.trim().to_string();
    }
    list.retain(|d| !d.name.is_empty() && !d.url.is_empty());
    list
}

pub fn save_web_api_destinations(destinations: &[WebApiDestination]) -> Result<(), String> {
    let path = web_api_destinations_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed creating config dir {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(destinations)
        .map_err(|e| format!("Failed serializing Web API destinations: {e}"))?;
    fs::write(&path, json)
        .map_err(|e| format!("Failed writing {}: {e}", path.display()))
}

pub fn upsert_web_api_destination(
    name: &str,
    method: &str,
    url: &str,
    headers: &str,
) -> Result<Vec<WebApiDestination>, String> {
    let clean_name = name.trim();
    let clean_url = url.trim();
    if clean_name.is_empty() {
        return Err("Destination name cannot be empty.".to_string());
    }
    if clean_url.is_empty() {
        return Err("Destination URL cannot be empty.".to_string());
    }
    let clean_method = {
        let m = method.trim().to_ascii_uppercase();
        if m.is_empty() {
            "POST".to_string()
        } else {
            m
        }
    };
    let clean_headers = headers.trim().to_string();

    let mut list = load_web_api_destinations();
    if let Some(existing) = list
        .iter_mut()
        .find(|d| d.name.eq_ignore_ascii_case(clean_name))
    {
        existing.name = clean_name.to_string();
        existing.method = clean_method;
        existing.url = clean_url.to_string();
        existing.headers = clean_headers;
    } else {
        list.push(WebApiDestination {
            name: clean_name.to_string(),
            method: clean_method,
            url: clean_url.to_string(),
            headers: clean_headers,
        });
    }
    save_web_api_destinations(&list)?;
    Ok(list)
}

pub fn delete_web_api_destination_by_name(name: &str) -> Result<Vec<WebApiDestination>, String> {
    let clean_name = name.trim();
    let mut list = load_web_api_destinations();
    list.retain(|d| !d.name.eq_ignore_ascii_case(clean_name));
    save_web_api_destinations(&list)?;
    Ok(list)
}

pub fn seed_web_api_destinations_from_profiles(profiles: &[DuckyProfile]) -> Vec<WebApiDestination> {
    let path = web_api_destinations_path();
    let existing = load_web_api_destinations();
    if path.exists() || !existing.is_empty() {
        return existing;
    }
    let mut seeded: Vec<WebApiDestination> = Vec::new();
    for prof in profiles {
        for (&slot_num, slot) in &prof.slots {
            for script in [&slot.on_press_script, &slot.on_release_script] {
                if !has_web_api_config(script) {
                    continue;
                }
                let cfg = extract_web_api_config(script, slot_num);
                let url = cfg.url.trim();
                if url.is_empty() {
                    continue;
                }
                let already_have = seeded.iter().any(|d| {
                    d.url.eq_ignore_ascii_case(url)
                        && d.method.eq_ignore_ascii_case(&cfg.method)
                        && d.headers.trim() == cfg.headers.trim()
                });
                if already_have {
                    continue;
                }
                let base_name = if !cfg.destination.trim().is_empty() {
                    cfg.destination.trim().to_string()
                } else {
                    let lbl = format!("{} {}", slot.label_line1.trim(), slot.label_line2.trim())
                        .trim()
                        .to_string();
                    if !lbl.is_empty() {
                        format!("{} ({})", prof.name.trim(), lbl)
                    } else {
                        format!("{} — Key {}", prof.name.trim(), slot_num)
                    }
                };
                let mut candidate_name = base_name.clone();
                let mut suffix = 2usize;
                while seeded
                    .iter()
                    .any(|d| d.name.eq_ignore_ascii_case(&candidate_name))
                {
                    candidate_name = format!("{base_name} #{suffix}");
                    suffix += 1;
                }
                seeded.push(WebApiDestination {
                    name: candidate_name,
                    method: cfg.method.clone(),
                    url: url.to_string(),
                    headers: cfg.headers.trim().to_string(),
                });
            }
        }
    }
    if !seeded.is_empty() {
        let _ = save_web_api_destinations(&seeded);
    }
    seeded
}

pub fn resolve_web_api_config_destination(cfg: &mut WebApiConfig) {
    let dest_name = cfg.destination.trim();
    if dest_name.is_empty() {
        return;
    }
    let destinations = load_web_api_destinations();
    if let Some(found) = destinations
        .iter()
        .find(|d| d.name.eq_ignore_ascii_case(dest_name))
    {
        cfg.destination = found.name.clone();
        cfg.method = found.method.clone();
        cfg.url = found.url.clone();
        cfg.headers = found.headers.clone();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebApiConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub destination: String,
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
            destination: String::new(),
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
    pub route_note: Option<String>,
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
                resolve_web_api_config_destination(&mut cfg);
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
            if let Ok(mut cfg) = serde_json::from_str::<WebApiConfig>(json_part.trim()) {
                resolve_web_api_config_destination(&mut cfg);
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
    normalized.destination = normalized.destination.trim().to_string();
    resolve_web_api_config_destination(&mut normalized);
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

pub const APP_LAUNCH_META_PREFIX: &str = "// @DUCKYDECK_APP_LAUNCH: ";
pub const APP_LAUNCH_BLOCK_BEGIN: &str = "// BEGIN_DUCKYDECK_APP_LAUNCH";
pub const APP_LAUNCH_BLOCK_END: &str = "// END_DUCKYDECK_APP_LAUNCH";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppLaunchConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub app_name: String,
    #[serde(default)]
    pub desktop_id: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: String,
    #[serde(default)]
    pub working_dir: String,
    #[serde(default)]
    pub run_in_terminal: bool,
    #[serde(default)]
    pub oled_text: String,
    #[serde(default = "default_web_api_led_feedback")]
    pub led_feedback: bool,
    #[serde(default = "default_web_api_trigger_mode")]
    pub trigger_mode: String,
    #[serde(default = "default_web_api_fkey")]
    pub fkey: String,
}

impl Default for AppLaunchConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            app_name: String::new(),
            desktop_id: String::new(),
            command: String::new(),
            args: String::new(),
            working_dir: String::new(),
            run_in_terminal: false,
            oled_text: String::new(),
            led_feedback: true,
            trigger_mode: default_web_api_trigger_mode(),
            fkey: default_web_api_fkey(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledDesktopApp {
    pub name: String,
    pub desktop_id: String,
    pub exec_command: String,
    pub exec_args: String,
    pub icon: String,
    pub comment: String,
    pub categories: String,
    pub terminal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppLaunchExecutionResult {
    pub ok: bool,
    pub pid: u32,
    pub summary: String,
    pub command_line: String,
}

pub fn extract_app_launch_config(script: &str, slot_num: usize) -> AppLaunchConfig {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(APP_LAUNCH_META_PREFIX) {
            if let Ok(mut cfg) = serde_json::from_str::<AppLaunchConfig>(json_part.trim()) {
                if !matches!(
                    cfg.trigger_mode.as_str(),
                    "pgv" | "fkey" | "krunner" | "guir"
                ) {
                    cfg.trigger_mode = "pgv".to_string();
                }
                if cfg.fkey.trim().is_empty() {
                    cfg.fkey = default_fkey_for_slot(slot_num);
                }
                if cfg.command.trim().is_empty() && !cfg.desktop_id.trim().is_empty() {
                    let target_id = cfg.desktop_id.trim();
                    if let Some(found) = scan_installed_desktop_apps()
                        .into_iter()
                        .find(|a| a.desktop_id.eq_ignore_ascii_case(target_id))
                    {
                        cfg.command = found.exec_command;
                        if cfg.args.trim().is_empty() && !found.exec_args.trim().is_empty() {
                            cfg.args = found.exec_args;
                        }
                        if cfg.app_name.trim().is_empty() {
                            cfg.app_name = found.name;
                        }
                    } else {
                        cfg.command = format!("gtk-launch {target_id}");
                    }
                }
                return cfg;
            }
        }
    }
    AppLaunchConfig {
        fkey: default_fkey_for_slot(slot_num),
        ..Default::default()
    }
}

pub fn has_app_launch_config(script: &str) -> bool {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(APP_LAUNCH_META_PREFIX) {
            if let Ok(cfg) = serde_json::from_str::<AppLaunchConfig>(json_part.trim()) {
                if cfg.enabled
                    && (!cfg.command.trim().is_empty() || !cfg.desktop_id.trim().is_empty())
                {
                    return true;
                }
            }
        }
    }
    false
}

pub fn strip_app_launch_from_script(script: &str) -> String {
    let mut out_lines = Vec::new();
    let mut inside_block = false;
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(APP_LAUNCH_META_PREFIX) {
            continue;
        }
        if trimmed == APP_LAUNCH_BLOCK_BEGIN {
            inside_block = true;
            continue;
        }
        if trimmed == APP_LAUNCH_BLOCK_END {
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

pub fn apply_app_launch_config_to_script(
    existing_script: &str,
    cfg: &AppLaunchConfig,
    profile_num: u8,
    slot_num: usize,
    is_release: bool,
) -> String {
    let remainder = strip_app_launch_from_script(existing_script);
    if !cfg.enabled {
        return remainder;
    }

    let mut normalized = cfg.clone();
    normalized.command = normalized.command.trim().to_string();
    normalized.args = normalized.args.trim().to_string();
    normalized.working_dir = normalized.working_dir.trim().to_string();
    if !matches!(
        normalized.trigger_mode.as_str(),
        "pgv" | "fkey" | "krunner" | "guir"
    ) {
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
    out.push_str(&format!("{APP_LAUNCH_META_PREFIX}{meta_json}\n"));
    out.push_str(&format!("{APP_LAUNCH_BLOCK_BEGIN}\n"));

    let oled_clean = normalized.oled_text.trim();
    if !oled_clean.is_empty() {
        out.push_str("OLED_CLEAR\n");
        out.push_str(&format!("OLED_CPRINT {oled_clean}\n"));
        out.push_str("OLED_UPDATE\n");
    }

    let full_cmd = if normalized.args.is_empty() {
        normalized.command.clone()
    } else {
        format!("{} {}", normalized.command, normalized.args)
    };

    match normalized.trigger_mode.as_str() {
        "fkey" => {
            out.push_str(&format!("CONTROL ALT SHIFT {valid_fkey}\n"));
        }
        "krunner" => {
            out.push_str("ALT SPACE\nDELAY 250\n");
            if !full_cmd.is_empty() {
                out.push_str(&format!("STRINGLN {full_cmd}\n"));
            }
        }
        "guir" => {
            out.push_str("GUI r\nDELAY 250\n");
            if !full_cmd.is_empty() {
                out.push_str(&format!("STRINGLN {full_cmd}\n"));
            }
        }
        _ => {
            out.push_str(&format!("$_GV31 = 0x{trigger_code:08X}\n"));
        }
    }

    out.push_str(&format!("{APP_LAUNCH_BLOCK_END}\n"));
    if !remainder.trim().is_empty() {
        out.push_str(&remainder);
    }
    out
}

pub fn sync_profile_web_api_triggers(profiles: &mut [DuckyProfile]) -> bool {
    let mut changed = false;
    for prof in profiles.iter_mut() {
        let pnum = prof.number;
        for (&slot_num, slot) in prof.slots.iter_mut() {
            if has_web_api_config(&slot.on_press_script) {
                let cfg = extract_web_api_config(&slot.on_press_script, slot_num);
                let updated = apply_web_api_config_to_script(
                    &slot.on_press_script,
                    &cfg,
                    pnum,
                    slot_num,
                    false,
                );
                if updated != slot.on_press_script {
                    slot.on_press_script = updated;
                    changed = true;
                }
            }
            if has_web_api_config(&slot.on_release_script) {
                let cfg = extract_web_api_config(&slot.on_release_script, slot_num);
                let updated = apply_web_api_config_to_script(
                    &slot.on_release_script,
                    &cfg,
                    pnum,
                    slot_num,
                    true,
                );
                if updated != slot.on_release_script {
                    slot.on_release_script = updated;
                    changed = true;
                }
            }
            if has_app_launch_config(&slot.on_press_script) {
                let cfg = extract_app_launch_config(&slot.on_press_script, slot_num);
                let updated = apply_app_launch_config_to_script(
                    &slot.on_press_script,
                    &cfg,
                    pnum,
                    slot_num,
                    false,
                );
                if updated != slot.on_press_script {
                    slot.on_press_script = updated;
                    changed = true;
                }
            }
            if has_app_launch_config(&slot.on_release_script) {
                let cfg = extract_app_launch_config(&slot.on_release_script, slot_num);
                let updated = apply_app_launch_config_to_script(
                    &slot.on_release_script,
                    &cfg,
                    pnum,
                    slot_num,
                    true,
                );
                if updated != slot.on_release_script {
                    slot.on_release_script = updated;
                    changed = true;
                }
            }
        }
    }
    changed
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
    if bytes_read < 11 || rx[2] != 0 {
        return None;
    }

    // rx[0]=ReportID(4), rx[1]=Seq, rx[2]=Status(0), rx[3..7]=_GV30, rx[7..11]=_GV31
    let raw_gv31 = i32::from_le_bytes([rx[7], rx[8], rx[9], rx[10]]);
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

pub fn set_batch_key_leds_hid(updates: &[(usize, [u8; 3])]) -> bool {
    if updates.is_empty() {
        return true;
    }
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    let Ok(_guard) = HID_IO_LOCK.try_lock() else {
        return false;
    };
    let Ok(device) = open_duckypad_hid() else {
        return false;
    };
    let mut any_ok = false;
    for (idx, &(slot_num, rgb)) in updates.iter().enumerate() {
        if !(1..=ONBOARD_KEY_COUNT).contains(&slot_num) {
            continue;
        }
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x40u8.wrapping_add(idx as u8);
        tx[2] = HID_CMD_SET_LED_SINGLE;
        tx[3] = (slot_num - 1) as u8;
        tx[4] = rgb[0];
        tx[5] = rgb[1];
        tx[6] = rgb[2];
        if device.write(&tx).is_ok() {
            any_ok = true;
            let mut rx = [0u8; 64];
            let _ = device.read_timeout(&mut rx, 20);
        }
    }
    any_ok
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

fn resolve_home_assistant_service_url(url: &str, body: &str) -> (String, Option<String>) {
    let trimmed_body = body.trim();
    if trimmed_body.is_empty() {
        return (url.to_string(), None);
    }
    let Ok(json_val) = serde_json::from_str::<serde_json::Value>(trimmed_body) else {
        return (url.to_string(), None);
    };
    let Some(entity_val) = json_val.get("entity_id") else {
        return (url.to_string(), None);
    };

    let mut entity_ids: Vec<&str> = Vec::new();
    match entity_val {
        serde_json::Value::String(s) => entity_ids.push(s.as_str()),
        serde_json::Value::Array(arr) => {
            for item in arr {
                if let Some(s) = item.as_str() {
                    entity_ids.push(s);
                }
            }
        }
        _ => {}
    }
    if entity_ids.is_empty() {
        return (url.to_string(), None);
    }

    for &(src_segment, expected_prefix) in &[
        ("/api/services/switch/toggle", "switch."),
        ("/api/services/switch/turn_on", "switch."),
        ("/api/services/switch/turn_off", "switch."),
        ("/api/services/light/toggle", "light."),
        ("/api/services/light/turn_on", "light."),
        ("/api/services/light/turn_off", "light."),
    ] {
        if url.contains(src_segment)
            && entity_ids
                .iter()
                .any(|eid| eid.contains('.') && !eid.starts_with(expected_prefix))
        {
            let action = src_segment.rsplit('/').next().unwrap_or("toggle");
            let dst_segment = format!("/api/services/homeassistant/{action}");
            let new_url = url.replacen(src_segment, &dst_segment, 1);
            let note = format!(
                "Home Assistant service auto-routed '{src_segment}' -> '{dst_segment}' to match entity_id domain ({})",
                entity_ids.join(", ")
            );
            return (new_url, Some(note));
        }
    }

    (url.to_string(), None)
}

pub async fn execute_web_api_request(input_cfg: &WebApiConfig) -> WebApiExecutionResult {
    let mut resolved_cfg = input_cfg.clone();
    resolve_web_api_config_destination(&mut resolved_cfg);
    let cfg = &resolved_cfg;
    let raw_url = cfg.url.trim();
    if raw_url.is_empty() {
        return WebApiExecutionResult {
            ok: false,
            status_code: 0,
            elapsed_ms: 0,
            summary: "Error: URL is empty".to_string(),
            response_preview: String::new(),
            route_note: None,
        };
    }

    let (effective_url, route_note) = resolve_home_assistant_service_url(raw_url, &cfg.body);

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
                route_note,
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

    let mut req = client.request(method.clone(), &effective_url);
    let mut has_content_type = false;

    for raw_line in cfg.headers.lines() {
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
                route_note,
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
            }
            ;
            WebApiExecutionResult {
                ok: false,
                status_code: 0,
                elapsed_ms,
                summary: err_msg.clone(),
                response_preview: err_msg,
                route_note,
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

pub fn split_shell_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut escaped = false;

    for ch in input.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && !in_single {
            escaped = true;
            continue;
        }
        if ch == '\'' && !in_double {
            in_single = !in_single;
            continue;
        }
        if ch == '"' && !in_single {
            in_double = !in_double;
            continue;
        }
        if ch.is_whitespace() && !in_single && !in_double {
            if !current.is_empty() {
                words.push(std::mem::take(&mut current));
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn expand_tilde_str(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed == "~" {
        if let Ok(home) = std::env::var("HOME") {
            return home;
        }
    } else if let Some(rest) = trimmed.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/{rest}", home.trim_end_matches('/'));
        }
    }
    trimmed.to_string()
}

fn clean_desktop_exec(raw_exec: &str, desktop_id: &str) -> (String, String) {
    let tokens = split_shell_words(raw_exec);
    let mut filtered: Vec<String> = Vec::new();
    let mut skip_file_forwarding_block = false;
    for tok in tokens {
        if tok == "@@u" || tok == "@@" {
            skip_file_forwarding_block = !skip_file_forwarding_block;
            continue;
        }
        if tok.starts_with('%') && tok.len() == 2 {
            continue;
        }
        if !tok.is_empty() {
            filtered.push(tok);
        }
    }
    if filtered.is_empty() {
        return (String::new(), String::new());
    }

    let first_bin = Path::new(&filtered[0])
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(filtered[0].as_str());
    if first_bin == "flatpak" && filtered.get(1).map(|s| s.as_str()) == Some("run") {
        let app_id = desktop_id.trim_end_matches(".desktop");
        if !app_id.is_empty() && app_id.contains('.') {
            return ("flatpak".to_string(), format!("run {app_id}"));
        }
    }

    let cmd = filtered.remove(0);
    let args = filtered.join(" ");
    (cmd, args)
}

pub fn scan_installed_desktop_apps() -> Vec<InstalledDesktopApp> {
    let mut search_dirs: Vec<PathBuf> = Vec::new();
    if let Ok(home) = std::env::var("HOME") {
        search_dirs.push(PathBuf::from(&home).join(".local/share/applications"));
        search_dirs.push(PathBuf::from(&home).join(".local/share/flatpak/exports/share/applications"));
    }
    if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
        for part in xdg_data.split(':') {
            let p = part.trim();
            if !p.is_empty() && p != "." {
                let cand = PathBuf::from(p).join("applications");
                if !search_dirs.contains(&cand) {
                    search_dirs.push(cand);
                }
            }
        }
    }
    for sys_dir in [
        "/var/lib/flatpak/exports/share/applications",
        "/var/lib/snapd/desktop/applications",
        "/usr/local/share/applications",
        "/usr/share/applications",
    ] {
        let p = PathBuf::from(sys_dir);
        if !search_dirs.contains(&p) {
            search_dirs.push(p);
        }
    }

    let mut seen_ids = std::collections::HashSet::new();
    let mut seen_names = std::collections::HashSet::new();
    let mut apps = Vec::new();

    for dir in search_dirs {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if !fname.ends_with(".desktop") {
                continue;
            }
            let desktop_id = fname.to_string();
            if !seen_ids.insert(desktop_id.to_ascii_lowercase()) {
                continue;
            }

            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };

            let mut in_entry = false;
            let mut name = String::new();
            let mut exec_raw = String::new();
            let mut icon = String::new();
            let mut comment = String::new();
            let mut categories = String::new();
            let mut no_display = false;
            let mut hidden = false;
            let mut terminal = false;
            let mut entry_type = String::new();

            for line in content.lines() {
                let l = line.trim();
                if l.is_empty() || l.starts_with('#') {
                    continue;
                }
                if l.starts_with('[') {
                    if in_entry {
                        break;
                    }
                    in_entry = l == "[Desktop Entry]";
                    continue;
                }
                if !in_entry {
                    continue;
                }
                if let Some(v) = l.strip_prefix("Name=") {
                    if name.is_empty() {
                        name = v.trim().to_string();
                    }
                } else if let Some(v) = l.strip_prefix("Exec=") {
                    if exec_raw.is_empty() {
                        exec_raw = v.trim().to_string();
                    }
                } else if let Some(v) = l.strip_prefix("Icon=") {
                    if icon.is_empty() {
                        icon = v.trim().to_string();
                    }
                } else if let Some(v) = l.strip_prefix("Comment=") {
                    if comment.is_empty() {
                        comment = v.trim().to_string();
                    }
                } else if let Some(v) = l.strip_prefix("GenericName=") {
                    if comment.is_empty() {
                        comment = v.trim().to_string();
                    }
                } else if let Some(v) = l.strip_prefix("Categories=") {
                    categories = v.trim().to_string();
                } else if let Some(v) = l.strip_prefix("Type=") {
                    entry_type = v.trim().to_string();
                } else if let Some(v) = l.strip_prefix("NoDisplay=") {
                    no_display = v.trim().eq_ignore_ascii_case("true");
                } else if let Some(v) = l.strip_prefix("Hidden=") {
                    hidden = v.trim().eq_ignore_ascii_case("true");
                } else if let Some(v) = l.strip_prefix("Terminal=") {
                    terminal = v.trim().eq_ignore_ascii_case("true");
                }
            }

            if no_display || hidden || name.is_empty() || exec_raw.is_empty() {
                continue;
            }
            if !entry_type.is_empty() && !entry_type.eq_ignore_ascii_case("Application") {
                continue;
            }

            let (exec_command, exec_args) = clean_desktop_exec(&exec_raw, &desktop_id);
            if exec_command.is_empty() {
                continue;
            }

            let dedup_key = format!(
                "{}|{}|{}",
                name.to_ascii_lowercase(),
                exec_command.to_ascii_lowercase(),
                exec_args.to_ascii_lowercase()
            );
            if !seen_names.insert(dedup_key) {
                continue;
            }

            if icon.is_empty() {
                icon = "application-x-executable".to_string();
            }

            apps.push(InstalledDesktopApp {
                name,
                desktop_id,
                exec_command,
                exec_args,
                icon,
                comment,
                categories,
                terminal,
            });
        }
    }

    apps.sort_by(|a, b| a.name.to_ascii_lowercase().cmp(&b.name.to_ascii_lowercase()));
    apps
}

fn find_terminal_emulator() -> Option<(String, Vec<String>)> {
    let candidates: &[(&str, &[&str])] = &[
        ("konsole", &["-e"]),
        ("xdg-terminal-exec", &[]),
        ("ptyxis", &["--"]),
        ("kgx", &["--"]),
        ("gnome-terminal", &["--"]),
        ("alacritty", &["-e"]),
        ("kitty", &[]),
        ("wezterm", &["start", "--"]),
        ("xfce4-terminal", &["-x"]),
        ("xterm", &["-e"]),
    ];
    let path_var = std::env::var("PATH").unwrap_or_default();
    for &(bin, flags) in candidates {
        for dir in path_var.split(':') {
            if dir.is_empty() {
                continue;
            }
            let full = Path::new(dir).join(bin);
            if full.is_file() {
                return Some((
                    bin.to_string(),
                    flags.iter().map(|s| (*s).to_string()).collect(),
                ));
            }
        }
    }
    None
}

pub fn execute_app_launch(cfg: &AppLaunchConfig) -> AppLaunchExecutionResult {
    use std::process::{Command, Stdio};

    let mut effective_cmd = cfg.command.trim().to_string();
    let mut effective_args = cfg.args.trim().to_string();

    if effective_cmd.is_empty() && !cfg.desktop_id.trim().is_empty() {
        let target_id = cfg.desktop_id.trim();
        if let Some(found) = scan_installed_desktop_apps()
            .into_iter()
            .find(|a| a.desktop_id.eq_ignore_ascii_case(target_id))
        {
            effective_cmd = found.exec_command;
            if effective_args.is_empty() && !found.exec_args.trim().is_empty() {
                effective_args = found.exec_args;
            }
        } else {
            effective_cmd = format!("gtk-launch {target_id}");
        }
    }

    let raw_cmd = effective_cmd.trim();
    if raw_cmd.is_empty() {
        return AppLaunchExecutionResult {
            ok: false,
            pid: 0,
            summary: "Error: Application command is empty".to_string(),
            command_line: String::new(),
        };
    }

    let raw_args = effective_args.trim();
    let full_cmd_display = if raw_args.is_empty() {
        raw_cmd.to_string()
    } else {
        format!("{raw_cmd} {raw_args}")
    };

    let needs_shell = full_cmd_display.contains('|')
        || full_cmd_display.contains("&&")
        || full_cmd_display.contains("||")
        || full_cmd_display.contains(';')
        || full_cmd_display.contains('>')
        || full_cmd_display.contains('<')
        || full_cmd_display.contains('$');

    let mut tokens: Vec<String> = if needs_shell {
        vec![
            "sh".to_string(),
            "-c".to_string(),
            full_cmd_display.clone(),
        ]
    } else {
        let mut t = split_shell_words(raw_cmd);
        if !raw_args.is_empty() {
            t.extend(split_shell_words(raw_args));
        }
        t.into_iter().map(|tok| expand_tilde_str(&tok)).collect()
    };

    if tokens.is_empty() {
        return AppLaunchExecutionResult {
            ok: false,
            pid: 0,
            summary: "Error: Unable to parse command".to_string(),
            command_line: full_cmd_display,
        };
    }

    if cfg.run_in_terminal {
        if let Some((term_bin, mut term_flags)) = find_terminal_emulator() {
            term_flags.extend(tokens);
            let mut new_tokens = vec![term_bin];
            new_tokens.extend(term_flags);
            tokens = new_tokens;
        }
    }

    let program = tokens.remove(0);
    let mut cmd = Command::new(&program);
    cmd.args(&tokens)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let work_dir = expand_tilde_str(&cfg.working_dir);
    if !work_dir.is_empty() {
        let p = Path::new(&work_dir);
        if p.is_dir() {
            cmd.current_dir(p);
        } else {
            return AppLaunchExecutionResult {
                ok: false,
                pid: 0,
                summary: format!("Working directory does not exist: {work_dir}"),
                command_line: full_cmd_display,
            };
        }
    } else if let Ok(home) = std::env::var("HOME") {
        let hp = Path::new(&home);
        if hp.is_dir() {
            cmd.current_dir(hp);
        }
    }

    let display_name = if !cfg.app_name.trim().is_empty() {
        cfg.app_name.trim().to_string()
    } else {
        program.clone()
    };

    match cmd.spawn() {
        Ok(mut child) => {
            let pid = child.id();
            std::thread::spawn(move || {
                let _ = child.wait();
            });
            AppLaunchExecutionResult {
                ok: true,
                pid,
                summary: format!("Launched '{display_name}' (PID {pid})"),
                command_line: full_cmd_display,
            }
        }
        Err(e) => AppLaunchExecutionResult {
            ok: false,
            pid: 0,
            summary: format!("Failed to start '{display_name}': {e}"),
            command_line: full_cmd_display,
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedMacroInputEvent {
    pub kind: String, // "key_down", "key_up", "mouse_click", "mouse_move", "mouse_scroll"
    pub key: String,
    pub dx: i32,
    pub dy: i32,
}

pub fn evdev_code_to_duckyscript_key(code: u16) -> Option<&'static str> {
    match code {
        1 => Some("ESC"),
        2 => Some("1"),
        3 => Some("2"),
        4 => Some("3"),
        5 => Some("4"),
        6 => Some("5"),
        7 => Some("6"),
        8 => Some("7"),
        9 => Some("8"),
        10 => Some("9"),
        11 => Some("0"),
        12 => Some("-"),
        13 => Some("="),
        14 => Some("BACKSPACE"),
        15 => Some("TAB"),
        16 => Some("q"),
        17 => Some("w"),
        18 => Some("e"),
        19 => Some("r"),
        20 => Some("t"),
        21 => Some("y"),
        22 => Some("u"),
        23 => Some("i"),
        24 => Some("o"),
        25 => Some("p"),
        26 => Some("["),
        27 => Some("]"),
        28 => Some("ENTER"),
        29 => Some("LCTRL"),
        30 => Some("a"),
        31 => Some("s"),
        32 => Some("d"),
        33 => Some("f"),
        34 => Some("g"),
        35 => Some("h"),
        36 => Some("j"),
        37 => Some("k"),
        38 => Some("l"),
        39 => Some(";"),
        40 => Some("'"),
        41 => Some("`"),
        42 => Some("LSHIFT"),
        43 => Some("\\"),
        44 => Some("z"),
        45 => Some("x"),
        46 => Some("c"),
        47 => Some("v"),
        48 => Some("b"),
        49 => Some("n"),
        50 => Some("m"),
        51 => Some(","),
        52 => Some("."),
        53 => Some("/"),
        54 => Some("RSHIFT"),
        55 => Some("KP_ASTERISK"),
        56 => Some("LALT"),
        57 => Some("SPACE"),
        58 => Some("CAPSLOCK"),
        59 => Some("F1"),
        60 => Some("F2"),
        61 => Some("F3"),
        62 => Some("F4"),
        63 => Some("F5"),
        64 => Some("F6"),
        65 => Some("F7"),
        66 => Some("F8"),
        67 => Some("F9"),
        68 => Some("F10"),
        69 => Some("NUMLOCK"),
        70 => Some("SCROLLLOCK"),
        71 => Some("KP_7"),
        72 => Some("KP_8"),
        73 => Some("KP_9"),
        74 => Some("KP_MINUS"),
        75 => Some("KP_4"),
        76 => Some("KP_5"),
        77 => Some("KP_6"),
        78 => Some("KP_PLUS"),
        79 => Some("KP_1"),
        80 => Some("KP_2"),
        81 => Some("KP_3"),
        82 => Some("KP_0"),
        83 => Some("KP_DOT"),
        87 => Some("F11"),
        88 => Some("F12"),
        96 => Some("KP_ENTER"),
        97 => Some("RCTRL"),
        98 => Some("KP_SLASH"),
        99 => Some("PRINTSCREEN"),
        100 => Some("RALT"),
        102 => Some("HOME"),
        103 => Some("UP"),
        104 => Some("PAGEUP"),
        105 => Some("LEFT"),
        106 => Some("RIGHT"),
        107 => Some("END"),
        108 => Some("DOWN"),
        109 => Some("PAGEDOWN"),
        110 => Some("INSERT"),
        111 => Some("DELETE"),
        113 => Some("MUTE"),
        114 => Some("VOLUMEDOWN"),
        115 => Some("VOLUMEUP"),
        119 => Some("PAUSE"),
        125 => Some("LGUI"),
        126 => Some("RGUI"),
        127 => Some("MENU"),
        183 => Some("F13"),
        184 => Some("F14"),
        185 => Some("F15"),
        186 => Some("F16"),
        187 => Some("F17"),
        188 => Some("F18"),
        189 => Some("F19"),
        190 => Some("F20"),
        191 => Some("F21"),
        192 => Some("F22"),
        193 => Some("F23"),
        194 => Some("F24"),
        // Mouse buttons
        0x110 => Some("LMOUSE"),
        0x111 => Some("RMOUSE"),
        0x112 => Some("MMOUSE"),
        0x113 => Some("BMOUSE"),
        0x114 => Some("FMOUSE"),
        _ => None,
    }
}

pub struct GlobalMacroEvdevRecorder {
    files: Vec<(PathBuf, fs::File)>,
}

impl Default for GlobalMacroEvdevRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalMacroEvdevRecorder {
    pub fn new() -> Self {
        let mut s = Self { files: Vec::new() };
        s.open_all_input_nodes();
        s
    }

    pub fn open_all_input_nodes(&mut self) {
        use std::io::Read;
        use std::os::unix::fs::OpenOptionsExt;
        self.files.clear();
        let Ok(entries) = fs::read_dir("/sys/class/input") else {
            return;
        };
        let mut buf = [0u8; 240];
        for entry in entries.flatten() {
            let fname = entry.file_name();
            let Some(name_str) = fname.to_str() else {
                continue;
            };
            if !name_str.starts_with("event") {
                continue;
            }
            let dev_path = Path::new("/dev/input").join(name_str);
            if let Ok(mut file) = fs::OpenOptions::new()
                .read(true)
                .custom_flags(0o4000) // O_NONBLOCK
                .open(&dev_path)
            {
                // Drain any pending buffered bytes
                while file.read(&mut buf).is_ok() {}
                self.files.push((dev_path, file));
            }
        }
    }

    pub fn device_count(&self) -> usize {
        self.files.len()
    }

    pub fn poll_events(&mut self) -> Vec<RecordedMacroInputEvent> {
        use std::io::Read;
        let mut out = Vec::new();
        let mut buf = [0u8; 24];
        let mut accum_dx: i32 = 0;
        let mut accum_dy: i32 = 0;
        let mut accum_wheel_v: i32 = 0;
        let mut accum_wheel_h: i32 = 0;

        for (_, file) in self.files.iter_mut() {
            loop {
                match file.read_exact(&mut buf) {
                    Ok(()) => {
                        let ev_type = u16::from_ne_bytes([buf[16], buf[17]]);
                        let ev_code = u16::from_ne_bytes([buf[18], buf[19]]);
                        let ev_value = i32::from_ne_bytes([buf[20], buf[21], buf[22], buf[23]]);

                        if ev_type == 1 {
                            // EV_KEY: 1 = press, 0 = release (ignore 2 = autorepeat)
                            if ev_value == 1 || ev_value == 0 {
                                if let Some(duck_key) = evdev_code_to_duckyscript_key(ev_code) {
                                    // Flush pending mouse moves before key/click
                                    if accum_dx != 0 || accum_dy != 0 {
                                        out.push(RecordedMacroInputEvent {
                                            kind: "mouse_move".to_string(),
                                            key: String::new(),
                                            dx: accum_dx,
                                            dy: accum_dy,
                                        });
                                        accum_dx = 0;
                                        accum_dy = 0;
                                    }
                                    let is_mouse_btn = (0x110..=0x117).contains(&ev_code);
                                    out.push(RecordedMacroInputEvent {
                                        kind: if is_mouse_btn {
                                            if ev_value == 1 {
                                                "mouse_down".to_string()
                                            } else {
                                                "mouse_up".to_string()
                                            }
                                        } else if ev_value == 1 {
                                            "key_down".to_string()
                                        } else {
                                            "key_up".to_string()
                                        },
                                        key: duck_key.to_string(),
                                        dx: 0,
                                        dy: 0,
                                    });
                                }
                            }
                        } else if ev_type == 2 {
                            // EV_REL: 0 = REL_X, 1 = REL_Y, 6 = REL_HWHEEL, 8 = REL_WHEEL
                            match ev_code {
                                0 => accum_dx = accum_dx.saturating_add(ev_value),
                                1 => accum_dy = accum_dy.saturating_add(ev_value),
                                6 => accum_wheel_h = accum_wheel_h.saturating_add(ev_value),
                                8 => accum_wheel_v = accum_wheel_v.saturating_add(ev_value),
                                _ => {}
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(_) => break,
                }
            }
        }

        if accum_dx != 0 || accum_dy != 0 {
            out.push(RecordedMacroInputEvent {
                kind: "mouse_move".to_string(),
                key: String::new(),
                dx: accum_dx,
                dy: accum_dy,
            });
        }
        if accum_wheel_h != 0 || accum_wheel_v != 0 {
            out.push(RecordedMacroInputEvent {
                kind: "mouse_scroll".to_string(),
                key: String::new(),
                dx: accum_wheel_h,
                dy: accum_wheel_v,
            });
        }

        out
    }
}

// ============================================================================
// Home Assistant Web API Connection & Entity Discovery + Profile Change Webhook
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HomeAssistantConfig {
    #[serde(default, alias = "baseUrl")]
    pub base_url: String,
    #[serde(default, alias = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeAssistantEntity {
    #[serde(alias = "entityId")]
    pub entity_id: String,
    pub domain: String,
    #[serde(alias = "friendlyName")]
    pub friendly_name: String,
    pub state: String,
    pub icon: String,
    #[serde(alias = "areaOrDevice")]
    pub area_or_device: String,
    #[serde(alias = "suggestedService")]
    pub suggested_service: String,
}

pub fn home_assistant_config_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        });
    base.join("duckydeck-studio").join("home_assistant.json")
}

pub fn load_home_assistant_config() -> HomeAssistantConfig {
    let path = home_assistant_config_path();
    if let Ok(raw) = fs::read_to_string(&path) {
        if let Ok(mut cfg) = serde_json::from_str::<HomeAssistantConfig>(&raw) {
            cfg.base_url = cfg.base_url.trim().trim_end_matches('/').to_string();
            cfg.access_token = cfg.access_token.trim().to_string();
            if !cfg.base_url.is_empty() || !cfg.access_token.is_empty() {
                return cfg;
            }
        }
    }

    // Fallback: try to infer base_url and Bearer token from any saved Home Assistant WebApiDestination
    for dest in load_web_api_destinations() {
        let u = dest.url.trim();
        if let Some(idx) = u.find("/api/") {
            let base_url = u[..idx].trim_end_matches('/').to_string();
            let mut token = String::new();
            for line in dest.headers.lines() {
                let t = line.trim();
                if let Some((k, v)) = t.split_once(':') {
                    if k.trim().eq_ignore_ascii_case("authorization") {
                        let val = v.trim();
                        if let Some(rest) = val.strip_prefix("Bearer ").or_else(|| val.strip_prefix("bearer ")) {
                            token = rest.trim().to_string();
                        }
                    }
                }
            }
            if !base_url.is_empty() {
                return HomeAssistantConfig {
                    base_url,
                    access_token: token,
                };
            }
        }
    }

    HomeAssistantConfig {
        base_url: "http://homeassistant.local:8123".to_string(),
        access_token: String::new(),
    }
}

pub fn save_home_assistant_config(base_url: &str, access_token: &str) -> Result<HomeAssistantConfig, String> {
    let mut clean_url = base_url.trim().trim_end_matches('/').to_string();
    if let Some(idx) = clean_url.find("/api/") {
        clean_url = clean_url[..idx].trim_end_matches('/').to_string();
    } else if let Some(stripped) = clean_url.strip_suffix("/api") {
        clean_url = stripped.trim_end_matches('/').to_string();
    }
    let mut clean_token = access_token.trim().to_string();
    if let Some(rest) = clean_token
        .strip_prefix("Bearer ")
        .or_else(|| clean_token.strip_prefix("bearer "))
    {
        clean_token = rest.trim().to_string();
    }
    let cfg = HomeAssistantConfig {
        base_url: clean_url,
        access_token: clean_token,
    };
    let path = home_assistant_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed creating config dir: {e}"))?;
    }
    let json = serde_json::to_string_pretty(&cfg)
        .map_err(|e| format!("Failed serializing Home Assistant config: {e}"))?;
    fs::write(&path, json).map_err(|e| format!("Failed writing {}: {e}", path.display()))?;
    Ok(cfg)
}

pub fn suggested_ha_service_for_domain(domain: &str) -> &'static str {
    match domain {
        "light" | "switch" | "input_boolean" | "fan" | "cover" | "climate" | "humidifier" => "toggle",
        "media_player" => "media_play_pause",
        "scene" | "script" => "turn_on",
        "automation" => "trigger",
        "button" | "input_button" => "press",
        "lock" => "unlock",
        "vacuum" => "start",
        _ => "toggle",
    }
}

pub async fn fetch_and_search_home_assistant_entities(
    base_url: &str,
    access_token: &str,
    query: &str,
    domain_filter: &str,
) -> Result<Vec<HomeAssistantEntity>, String> {
    let saved = save_home_assistant_config(base_url, access_token)?;
    if saved.base_url.is_empty() {
        return Err("Home Assistant Base URL is required (e.g. http://homeassistant.local:8123)".to_string());
    }
    if saved.access_token.is_empty() {
        return Err("Home Assistant Long-Lived Access Token is required".to_string());
    }

    let states_url = format!("{}/api/states", saved.base_url);
    let client = reqwest::Client::builder()
        .user_agent("DuckyDeck-Studio/0.1.0")
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(&states_url)
        .header("Authorization", format!("Bearer {}", saved.access_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed connecting to Home Assistant at {states_url}: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Home Assistant returned HTTP {} from {}",
            resp.status(),
            states_url
        ));
    }

    let raw_states: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid JSON from Home Assistant /api/states: {e}"))?;

    let arr = raw_states
        .as_array()
        .ok_or_else(|| "Unexpected response format from /api/states".to_string())?;

    let q_lower = query.trim().to_ascii_lowercase();
    let dom_lower = domain_filter.trim().to_ascii_lowercase();

    let actionable_domains = [
        "light",
        "switch",
        "scene",
        "script",
        "automation",
        "button",
        "input_button",
        "input_boolean",
        "media_player",
        "fan",
        "cover",
        "climate",
        "lock",
        "vacuum",
        "siren",
        "humidifier",
        "water_heater",
        "remote",
        "select",
        "input_select",
        "number",
        "input_number",
    ];

    let mut results = Vec::new();
    for item in arr {
        let Some(entity_id) = item.get("entity_id").and_then(|v| v.as_str()) else {
            continue;
        };
        let domain = entity_id.split('.').next().unwrap_or("").to_ascii_lowercase();
        if !dom_lower.is_empty() && dom_lower != "all" {
            if domain != dom_lower {
                continue;
            }
        } else if q_lower.is_empty() && !actionable_domains.contains(&domain.as_str()) {
            // When browsing with no search query and "all" filter, prioritize controllable domains first
            continue;
        }

        let state_str = item
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let attrs = item.get("attributes");
        let friendly_name = attrs
            .and_then(|a| a.get("friendly_name"))
            .and_then(|v| v.as_str())
            .unwrap_or(entity_id)
            .to_string();
        let icon = attrs
            .and_then(|a| a.get("icon"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let device_class = attrs
            .and_then(|a| a.get("device_class"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if !q_lower.is_empty() {
            let matches_id = entity_id.to_ascii_lowercase().contains(&q_lower);
            let matches_name = friendly_name.to_ascii_lowercase().contains(&q_lower);
            let matches_class = device_class.to_ascii_lowercase().contains(&q_lower);
            let matches_state = state_str.to_ascii_lowercase().contains(&q_lower);
            if !matches_id && !matches_name && !matches_class && !matches_state {
                continue;
            }
        }

        let suggested_service = suggested_ha_service_for_domain(&domain).to_string();
        results.push(HomeAssistantEntity {
            entity_id: entity_id.to_string(),
            domain,
            friendly_name,
            state: state_str,
            icon,
            area_or_device: device_class,
            suggested_service,
        });
    }

    results.sort_by(|a, b| {
        a.domain
            .cmp(&b.domain)
            .then_with(|| a.friendly_name.to_ascii_lowercase().cmp(&b.friendly_name.to_ascii_lowercase()))
            .then_with(|| a.entity_id.cmp(&b.entity_id))
    });

    if results.len() > 300 {
        results.truncate(300);
    }

    Ok(results)
}

pub fn expand_profile_webhook_template(
    template: &str,
    profile_number: u8,
    profile_name: &str,
    previous_profile_number: u8,
    previous_profile_name: &str,
    source: &str,
    serial: &str,
) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let trimmed = template.trim();
    if trimmed.is_empty() {
        return serde_json::json!({
            "event": "profile_changed",
            "profile_number": profile_number,
            "profile_name": profile_name,
            "previous_profile_number": previous_profile_number,
            "previous_profile_name": previous_profile_name,
            "source": source,
            "device_serial": serial,
            "timestamp": ts
        })
        .to_string();
    }
    let escape_json = |s: &str| -> String {
        s.replace('\\', "\\\\").replace('"', "\\\"")
    };
    template
        .replace("{{profile_number}}", &profile_number.to_string())
        .replace("{{profile_name}}", &escape_json(profile_name))
        .replace("{{previous_profile_number}}", &previous_profile_number.to_string())
        .replace("{{previous_profile_name}}", &escape_json(previous_profile_name))
        .replace("{{source}}", &escape_json(source))
        .replace("{{device_serial}}", &escape_json(serial))
        .replace("{{timestamp}}", &ts.to_string())
}

pub async fn execute_profile_webhook_request(
    webhook: &ProfileWebhookConfig,
    profile_number: u8,
    profile_name: &str,
    previous_profile_number: u8,
    previous_profile_name: &str,
    source: &str,
    serial: &str,
) -> WebApiExecutionResult {
    let expanded_url = expand_profile_webhook_template(
        &webhook.url,
        profile_number,
        profile_name,
        previous_profile_number,
        previous_profile_name,
        source,
        serial,
    );
    let expanded_body = expand_profile_webhook_template(
        &webhook.body,
        profile_number,
        profile_name,
        previous_profile_number,
        previous_profile_name,
        source,
        serial,
    );
    let web_cfg = WebApiConfig {
        enabled: true,
        destination: webhook.destination.clone(),
        method: if webhook.method.trim().is_empty() {
            "POST".to_string()
        } else {
            webhook.method.clone()
        },
        url: expanded_url,
        headers: webhook.headers.clone(),
        body: expanded_body,
        oled_text: String::new(),
        led_feedback: false,
        trigger_mode: "pgv".to_string(),
        fkey: "F13".to_string(),
    };
    execute_web_api_request(&web_cfg).await
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
        let _ = fs::remove_dir_all(&temp_sd);
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

        let _ = fs::remove_dir_all(&temp_sd);
    }
}


