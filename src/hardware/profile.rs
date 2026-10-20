use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::actions::web_api::{default_web_api_method, extract_web_api_config, has_web_api_config};
use crate::hardware::storage::{MAX_INPUT_SLOTS, ONBOARD_KEY_COUNT, PID_DUCKYPAD_PRO};

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
            } else if t == "IS_LANDSCAPE 1" && rot != 180 && rot != 270 {
                rot = 90;
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
                self.preserved_config_lines
                    .push("IS_LANDSCAPE 1".to_string());
            }
            180 => {
                self.preserved_config_lines
                    .push("IS_LANDSCAPE 2".to_string());
                self.preserved_config_lines
                    .push("OLED_ROTATION 180".to_string());
            }
            270 => {
                self.preserved_config_lines
                    .push("IS_LANDSCAPE 3".to_string());
                self.preserved_config_lines
                    .push("OLED_ROTATION 270".to_string());
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
        '💡' => Some(if compact { "o" } else { "(o)" }),
        '🔌' => Some(if compact { "=" } else { "=|" }),
        '🌡' => Some(if compact { "T" } else { "(T)" }),
        '❄' => Some(if compact { "*" } else { "{*}" }),
        '🛡' => Some(if compact { "V" } else { "[V]" }),
        '📹' => Some(if compact { "C" } else { "[C]" }),
        '🌀' => Some(if compact { "@" } else { "(@)" }),
        '🚪' => Some(if compact { "|" } else { "[|]" }),
        '🧹' => Some(if compact { "o" } else { "(o)" }),
        '💧' => Some(if compact { "." } else { "(.)" }),
        '🚗' => Some(if compact { "=" } else { "[=]" }),
        '🚨' => Some(if compact { "!" } else { "/!\\" }),
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
                        if let Err(e) = fs::write(&press_dsb_path, &compiled.bin_array) {
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "STORAGE",
                                format!("Failed writing {}: {e}", press_dsb_path.display()),
                            );
                        }
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
                        if let Err(e) = fs::write(&release_dsb_path, &compiled.bin_array) {
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "STORAGE",
                                format!("Failed writing {}: {e}", release_dsb_path.display()),
                            );
                        }
                    }
                }
            }
        }

        Ok(profile)
    }

    pub fn save_to_sd_root(&self, sd_root: &Path) -> io::Result<PathBuf> {
        self.save_to_sd_root_with_stats(sd_root)
            .map(|(dir, _, _)| dir)
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
                cfg.push_str(&format!(
                    "dw_z{} {}\n",
                    slot_idx,
                    slot.widget_label_line1.trim()
                ));
            }
            if !slot.widget_label_line2.trim().is_empty() {
                cfg.push_str(&format!(
                    "dw_x{} {}\n",
                    slot_idx,
                    slot.widget_label_line2.trim()
                ));
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
                        if let Err(e) = fs::remove_file(&press_path) {
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "STORAGE",
                                format!("Failed removing {}: {e}", press_path.display()),
                            );
                        }
                        record_write(true);
                    }
                    if press_dsb_path.exists() {
                        if let Err(e) = fs::remove_file(&press_dsb_path) {
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "STORAGE",
                                format!("Failed removing {}: {e}", press_dsb_path.display()),
                            );
                        }
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
                        if let Err(e) = fs::remove_file(&release_path) {
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "STORAGE",
                                format!("Failed removing {}: {e}", release_path.display()),
                            );
                        }
                        record_write(true);
                    }
                    if release_dsb_path.exists() {
                        if let Err(e) = fs::remove_file(&release_dsb_path) {
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "STORAGE",
                                format!("Failed removing {}: {e}", release_dsb_path.display()),
                            );
                        }
                        record_write(true);
                    }
                }
            } else {
                if press_path.exists() {
                    if let Err(e) = fs::remove_file(&press_path) {
                        crate::bridge::device_controller::record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed removing {}: {e}", press_path.display()),
                        );
                    }
                    record_write(true);
                }
                if press_dsb_path.exists() {
                    if let Err(e) = fs::remove_file(&press_dsb_path) {
                        crate::bridge::device_controller::record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed removing {}: {e}", press_dsb_path.display()),
                        );
                    }
                    record_write(true);
                }
                if release_path.exists() {
                    if let Err(e) = fs::remove_file(&release_path) {
                        crate::bridge::device_controller::record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed removing {}: {e}", release_path.display()),
                        );
                    }
                    record_write(true);
                }
                if release_dsb_path.exists() {
                    if let Err(e) = fs::remove_file(&release_dsb_path) {
                        crate::bridge::device_controller::record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed removing {}: {e}", release_dsb_path.display()),
                        );
                    }
                    record_write(true);
                }
            }
        }

        if files_written > 0 {
            if let Ok(dir_handle) = fs::File::open(&profile_dir) {
                if let Err(e) = dir_handle.sync_all() {
                    crate::bridge::device_controller::record_system_event(
                        "WARN",
                        "STORAGE",
                        format!("Failed syncing directory {}: {e}", profile_dir.display()),
                    );
                }
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

pub(crate) fn write_file_if_changed_and_sync(path: &Path, contents: &[u8]) -> io::Result<bool> {
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
            if mnt.starts_with("/run/media/")
                || mnt.starts_with("/media/")
                || mnt.starts_with("/mnt/")
                || mnt.starts_with("/run/host/run/media/")
                || mnt.starts_with("/run/host/media/")
            {
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
        candidate_roots.push(PathBuf::from(format!("/run/host/run/media/{user}")));
        candidate_roots.push(PathBuf::from(format!("/run/host/media/{user}")));
    }
    for base in [
        "/run/media",
        "/media",
        "/run/host/run/media",
        "/run/host/media",
    ] {
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

    let sd_rules_file = sd_root.join(crate::kwin::auto_switcher::SD_RULES_FILENAME);
    if sd_rules_file.is_file() {
        let global_cache = profile_cache_dir();
        if sd_root != global_cache {
            let _ = fs::create_dir_all(&global_cache); // ignore: best-effort cache dir creation
            let _ = fs::copy(
                &sd_rules_file,
                global_cache.join(crate::kwin::auto_switcher::SD_RULES_FILENAME),
            ); // ignore: best-effort SD rules cache copy
        }
    }

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
    write_file_if_changed_and_sync(&profile_info_path, content.as_bytes())?;
    Ok(())
}

pub fn profile_cache_dir() -> PathBuf {
    crate::paths::data_dir().join("profile_cache")
}

pub fn profile_cache_dir_for_serial(serial: &str) -> PathBuf {
    let clean: String = serial
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if clean.is_empty() || clean == "-" {
        profile_cache_dir()
    } else {
        crate::paths::data_dir().join("device_profiles").join(clean)
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
                        if let Err(e) = fs::remove_dir_all(&p) {
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "STORAGE",
                                format!("Failed removing stale cache dir {}: {e}", p.display()),
                            );
                        }
                    }
                }
            }
        }
    }
    if let Err(e) = save_profile_order_to_sd(cache_dir, profiles) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "STORAGE",
            format!(
                "Failed saving profile_info.txt in {}: {e}",
                cache_dir.display()
            ),
        );
    }
    for prof in profiles {
        if let Err(e) = prof.save_to_sd_root(cache_dir) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed caching profile '{}' in {}: {e}",
                    prof.name,
                    cache_dir.display()
                ),
            );
        }
    }
    if let Some(cfg) = crate::kwin::auto_switcher::SwitcherState::load_config_from_disk() {
        let _ = crate::kwin::auto_switcher::save_rules_bundle_to_dir(
            cache_dir,
            cfg.enabled,
            cfg.default_profile,
            &cfg.rules,
            profiles,
        ); // ignore: best-effort rules bundle cache write
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
    if let Err(e) = fs::create_dir_all(&global_dir) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "STORAGE",
            format!("Failed creating cache dir {}: {e}", global_dir.display()),
        );
    }
    if let Err(e) = fs::write(global_dir.join(".sd_synced_hash"), &fp) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "STORAGE",
            format!(
                "Failed writing .sd_synced_hash in {}: {e}",
                global_dir.display()
            ),
        );
    }
    if let Some(ref snap) = snapshot_json {
        if let Err(e) = fs::write(global_dir.join(".sd_synced_snapshot.json"), snap) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed writing .sd_synced_snapshot.json in {}: {e}",
                    global_dir.display()
                ),
            );
        }
    }

    let dev_dir = profile_cache_dir_for_serial(serial);
    if dev_dir != global_dir {
        if let Err(e) = fs::create_dir_all(&dev_dir) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed creating device cache dir {}: {e}",
                    dev_dir.display()
                ),
            );
        }
        if let Err(e) = fs::write(dev_dir.join(".sd_synced_hash"), &fp) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed writing .sd_synced_hash in {}: {e}",
                    dev_dir.display()
                ),
            );
        }
        if let Some(ref snap) = snapshot_json {
            if let Err(e) = fs::write(dev_dir.join(".sd_synced_snapshot.json"), snap) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "STORAGE",
                    format!(
                        "Failed writing .sd_synced_snapshot.json in {}: {e}",
                        dev_dir.display()
                    ),
                );
            }
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
    if let Err(e) = fs::create_dir_all(&global_dir) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "STORAGE",
            format!("Failed creating cache dir {}: {e}", global_dir.display()),
        );
    }
    if let Err(e) = fs::write(global_dir.join(".sd_space_info.json"), &json) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "STORAGE",
            format!(
                "Failed writing .sd_space_info.json in {}: {e}",
                global_dir.display()
            ),
        );
    }
    if dev_dir != global_dir {
        if let Err(e) = fs::create_dir_all(&dev_dir) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed creating device cache dir {}: {e}",
                    dev_dir.display()
                ),
            );
        }
        if let Err(e) = fs::write(dev_dir.join(".sd_space_info.json"), &json) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed writing .sd_space_info.json in {}: {e}",
                    dev_dir.display()
                ),
            );
        }
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

    if let Some(cfg) = crate::kwin::auto_switcher::SwitcherState::load_config_from_disk() {
        match crate::kwin::auto_switcher::save_rules_bundle_to_dir(
            sd_root,
            cfg.enabled,
            cfg.default_profile,
            &cfg.rules,
            profiles,
        ) {
            Ok(true) => summary.files_written += 1,
            Ok(false) => summary.files_verified_unchanged += 1,
            Err(e) => {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "STORAGE",
                    format!("Failed writing duckydeck_rules.json on SD: {e}"),
                );
            }
        }
    }

    if let Ok(entries) = fs::read_dir(sd_root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                    if parse_profile_folder_name(fname).is_some()
                        && !valid_folders_lower.contains(&fname.to_ascii_lowercase())
                        && fs::remove_dir_all(&p).is_ok()
                    {
                        summary.folders_removed += 1;
                    }
                }
            }
        }
    }

    if let Ok(root_dir_handle) = fs::File::open(sd_root) {
        if let Err(e) = root_dir_handle.sync_all() {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!("Failed syncing SD root {}: {e}", sd_root.display()),
            );
        }
    }
    if let Err(e) = crate::hardware::msc::run_storage_cmd("sync", &[]) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "STORAGE",
            format!("Failed running sync command: {e}"),
        );
    }
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

pub fn check_profile_sync_status(
    serial: &str,
    profiles: &[DuckyProfile],
    sd_root: Option<&Path>,
    connected: bool,
    active_profile: u8,
) -> (String, String) {
    let mem_hash = compute_profiles_fingerprint(profiles);

    if connected
        && (profiles.is_empty()
            || (profiles.len() == 1 && profiles[0].name == "Blender Sculpt")
            || (active_profile as usize) > profiles.len())
    {
        return (
            "mismatch".to_string(),
            format!(
                "Profile mismatch: connected duckyPad is on Profile #{active_profile}, which does not match the {} loaded profile(s). Click to sync.",
                profiles.len()
            ),
        );
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
            let dev_snap_path =
                profile_cache_dir_for_serial(serial).join(".sd_synced_snapshot.json");
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
            let bk = base
                .slots
                .get(&slot_idx)
                .filter(|s| !s.is_effectively_empty());
            let ck = cur
                .slots
                .get(&slot_idx)
                .filter(|s| !s.is_effectively_empty());
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
                            format!(
                                "{} / {}",
                                b.widget_label_line1.trim(),
                                b.widget_label_line2.trim()
                            )
                        };
                        let new_wlbl = if c.widget_label_line2.trim().is_empty() {
                            c.widget_label_line1.trim().to_string()
                        } else {
                            format!(
                                "{} / {}",
                                c.widget_label_line1.trim(),
                                c.widget_label_line2.trim()
                            )
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
    if profiles.is_empty()
        || std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1")
        || std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1")
    {
        return;
    }
    write_profiles_to_cache_dir(&profile_cache_dir(), profiles);
}

pub fn sync_profiles_to_device_cache(serial: &str, profiles: &[DuckyProfile]) {
    if profiles.is_empty()
        || std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1")
        || std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1")
    {
        return;
    }
    write_profiles_to_cache_dir(&profile_cache_dir(), profiles);
    let dev_dir = profile_cache_dir_for_serial(serial);
    if dev_dir != profile_cache_dir() {
        write_profiles_to_cache_dir(&dev_dir, profiles);
    }
}

pub fn discover_dekunukem_profile_backups() -> Vec<(PathBuf, String, usize)> {
    let mut roots = Vec::new();
    let host_backups = crate::paths::dekunukem_config_dir().join("profile_backups");
    if host_backups.is_dir() {
        roots.push(host_backups);
    }
    let xdg_backups = crate::paths::xdg_data_home()
        .join("duckypad_config")
        .join("profile_backups");
    if xdg_backups.is_dir() && !roots.contains(&xdg_backups) {
        roots.push(xdg_backups);
    }

    let mut dirs: Vec<PathBuf> = Vec::new();
    for root in roots {
        if let Ok(entries) = fs::read_dir(&root) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() && is_duckypad_sd_root(&p) && !dirs.contains(&p) {
                    dirs.push(p);
                }
            }
        }
    }
    dirs.sort();
    dirs.reverse(); // newest first

    let mut out = Vec::new();
    for dir in dirs {
        let Some(fname) = dir.file_name().and_then(|s| s.to_str()) else {
            continue;
        };
        let count = fs::read_dir(&dir)
            .ok()
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|e| {
                        e.path().is_dir()
                            && e.file_name()
                                .to_str()
                                .and_then(parse_profile_folder_name)
                                .is_some()
                    })
                    .count()
            })
            .unwrap_or(0);
        if count > 0 {
            out.push((dir.clone(), fname.to_string(), count));
        }
    }
    out
}

pub fn load_cached_profiles_for_serial(serial: &str) -> Option<Vec<DuckyProfile>> {
    let clean: String = serial
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if clean.is_empty() || clean == "-" {
        return None;
    }
    for dev_cache in [
        profile_cache_dir_for_serial(&clean),
        crate::paths::host_data_dir()
            .join("device_profiles")
            .join(&clean),
    ] {
        if dev_cache.is_dir() {
            if let Ok(profiles) = load_all_sd_profiles(&dev_cache) {
                if !profiles.is_empty() {
                    return Some(profiles);
                }
            }
        }
    }
    None
}

pub fn load_profiles_for_connected_device(pid: u16, serial: &str) -> Option<Vec<DuckyProfile>> {
    if let Some(profiles) = load_cached_profiles_for_serial(serial) {
        return Some(profiles);
    }

    for global_cache in [
        profile_cache_dir(),
        crate::paths::host_data_dir().join("profile_cache"),
    ] {
        if global_cache.is_dir() {
            if let Ok(profiles) = load_all_sd_profiles(&global_cache) {
                if !profiles.is_empty() {
                    sync_profiles_to_device_cache(serial, &profiles);
                    return Some(profiles);
                }
            }
        }
    }

    // Check if there is a backup from duckyPad-Configurator for this device model
    let model_prefix = if pid == PID_DUCKYPAD_PRO {
        "duckyPad_Pro_backup_"
    } else {
        "duckyPad_backup_"
    };

    for (backup_dir, fname, _) in discover_dekunukem_profile_backups() {
        if fname.starts_with(model_prefix) {
            if let Ok(profiles) = load_all_sd_profiles(&backup_dir) {
                if !profiles.is_empty() {
                    sync_profiles_to_device_cache(serial, &profiles);
                    return Some(profiles);
                }
            }
        }
    }

    load_cached_or_backup_profiles()
}

pub fn load_cached_or_backup_profiles() -> Option<Vec<DuckyProfile>> {
    for cache_dir in [
        profile_cache_dir(),
        crate::paths::host_data_dir().join("profile_cache"),
    ] {
        if cache_dir.is_dir() {
            if let Ok(profiles) = load_all_sd_profiles(&cache_dir) {
                if !profiles.is_empty() {
                    return Some(profiles);
                }
            }
        }
    }

    for (backup_dir, _, _) in discover_dekunukem_profile_backups() {
        if let Ok(profiles) = load_all_sd_profiles(&backup_dir) {
            if !profiles.is_empty() {
                sync_profiles_to_cache(&profiles);
                return Some(profiles);
            }
        }
    }

    None
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

pub(crate) fn parse_profile_folder_name(folder: &str) -> Option<(Option<u8>, String)> {
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

pub(crate) fn parse_indexed_directive<'a>(line: &'a str, prefix: &str) -> Option<(usize, &'a str)> {
    let rest = line.strip_prefix(prefix)?;
    let (idx_str, value) = rest.split_once(' ')?;
    let slot_idx: usize = idx_str.parse().ok()?;
    if (1..=MAX_INPUT_SLOTS).contains(&slot_idx) {
        Some((slot_idx, value))
    } else {
        None
    }
}

pub(crate) fn parse_rgb_triplet(s: &str) -> Option<[u8; 3]> {
    let mut parts = s.split_whitespace();
    let r: u8 = parts.next()?.parse().ok()?;
    let g: u8 = parts.next()?.parse().ok()?;
    let b: u8 = parts.next()?.parse().ok()?;
    Some([r, g, b])
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
    crate::paths::data_dir().join("profile_templates.json")
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
    add_slot(
        &mut media_slots,
        1,
        "Copy",
        "",
        "Copy",
        "Ctrl+C",
        Some([0, 180, 120]),
        "CONTROL c\n",
    );
    add_slot(
        &mut media_slots,
        2,
        "Paste",
        "",
        "Paste",
        "Ctrl+V",
        Some([0, 180, 120]),
        "CONTROL v\n",
    );
    add_slot(
        &mut media_slots,
        3,
        "Undo",
        "",
        "Undo",
        "Ctrl+Z",
        Some([240, 140, 0]),
        "CONTROL z\n",
    );
    add_slot(
        &mut media_slots,
        4,
        "Redo",
        "",
        "Redo",
        "Ctrl+Shift+Z",
        Some([240, 140, 0]),
        "CONTROL SHIFT z\n",
    );
    add_slot(
        &mut media_slots,
        5,
        "Prev",
        "Track",
        "Previous",
        "Media Track",
        Some([56, 189, 248]),
        "MK_PREV\n",
    );
    add_slot(
        &mut media_slots,
        6,
        "Play",
        "Pause",
        "Play / Pause",
        "Media",
        Some([34, 197, 94]),
        "MK_PP\n",
    );
    add_slot(
        &mut media_slots,
        7,
        "Next",
        "Track",
        "Next",
        "Media Track",
        Some([56, 189, 248]),
        "MK_NEXT\n",
    );
    add_slot(
        &mut media_slots,
        8,
        "Mute",
        "",
        "Mute Audio",
        "Toggle",
        Some([239, 68, 68]),
        "MK_MUTE\n",
    );
    add_slot(
        &mut media_slots,
        21,
        "Vol-",
        "",
        "Volume Down",
        "",
        None,
        "MK_VOLDOWN\n",
    );
    add_slot(
        &mut media_slots,
        22,
        "Vol+",
        "",
        "Volume Up",
        "",
        None,
        "MK_VOLUP\n",
    );
    add_slot(
        &mut media_slots,
        23,
        "Mute",
        "",
        "Mute",
        "",
        None,
        "MK_MUTE\n",
    );

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
    add_slot(
        &mut dev_slots,
        1,
        "Build",
        "",
        "Build Project",
        "Ctrl+B",
        Some([34, 197, 94]),
        "CONTROL b\n",
    );
    add_slot(
        &mut dev_slots,
        2,
        "Run",
        "",
        "Run Target",
        "Shift+F10",
        Some([16, 185, 129]),
        "SHIFT F10\n",
    );
    add_slot(
        &mut dev_slots,
        3,
        "Debug",
        "",
        "Start Debug",
        "Shift+F9",
        Some([245, 158, 11]),
        "SHIFT F9\n",
    );
    add_slot(
        &mut dev_slots,
        4,
        "Stop",
        "",
        "Stop Process",
        "Ctrl+F2",
        Some([239, 68, 68]),
        "CONTROL F2\n",
    );
    add_slot(
        &mut dev_slots,
        5,
        "Find",
        "All",
        "Find in Files",
        "Ctrl+Shift+F",
        Some([56, 189, 248]),
        "CONTROL SHIFT f\n",
    );
    add_slot(
        &mut dev_slots,
        6,
        "Fmt",
        "Code",
        "Format Code",
        "Ctrl+Shift+I",
        Some([168, 85, 247]),
        "CONTROL SHIFT i\n",
    );
    add_slot(
        &mut dev_slots,
        7,
        "Term",
        "",
        "Toggle Terminal",
        "Ctrl+`",
        Some([148, 163, 184]),
        "CONTROL `\n",
    );
    add_slot(
        &mut dev_slots,
        8,
        "Git",
        "Diff",
        "Git Commit/Diff",
        "Ctrl+K",
        Some([249, 115, 22]),
        "CONTROL k\n",
    );

    list.push(ProfileTemplate {
        id: "builtin_developer_ide".to_string(),
        name: "Developer / IDE".to_string(),
        description:
            "Build, Run, Debug, Stop, Find in Files, Format Code, Terminal, and Git workflow keys."
                .to_string(),
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
