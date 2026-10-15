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

pub fn open_duckypad_hid() -> Result<HidDevice, String> {
    let api = HidApi::new().map_err(|e| format!("Failed to initialize hidapi: {e}"))?;
    for pid in [PID_DUCKYPAD_PRO, PID_DUCKYPAD_2020] {
        if let Ok(device) = api.open(DUCKYPAD_VID, pid) {
            return Ok(device);
        }
    }
    Err("No duckyPad Pro (0xd11d) or duckyPad (0xd11c) found on USB bus".into())
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
        let profile_dir = sd_root.join(self.folder_name());
        fs::create_dir_all(&profile_dir)?;

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

        let mut config_file = fs::File::create(profile_dir.join("config.txt"))?;
        config_file.write_all(cfg.as_bytes())?;
        config_file.sync_all()?;

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
                    fs::write(&press_path, &slot.on_press_script)?;
                    fs::write(&press_dsb_path, &compiled.bin_array)?;
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
                    fs::write(&press_path, "")?;
                    fs::write(&press_dsb_path, &compiled.bin_array)?;
                } else {
                    if press_path.exists() {
                        let _ = fs::remove_file(&press_path);
                    }
                    if press_dsb_path.exists() {
                        let _ = fs::remove_file(&press_dsb_path);
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
                    fs::write(&release_path, &slot.on_release_script)?;
                    fs::write(&release_dsb_path, &compiled.bin_array)?;
                } else {
                    if release_path.exists() {
                        let _ = fs::remove_file(&release_path);
                    }
                    if release_dsb_path.exists() {
                        let _ = fs::remove_file(&release_dsb_path);
                    }
                }
            } else {
                if press_path.exists() {
                    let _ = fs::remove_file(&press_path);
                }
                if press_dsb_path.exists() {
                    let _ = fs::remove_file(&press_dsb_path);
                }
                if release_path.exists() {
                    let _ = fs::remove_file(&release_path);
                }
                if release_dsb_path.exists() {
                    let _ = fs::remove_file(&release_dsb_path);
                }
            }
        }

        Ok(profile_dir)
    }

    pub fn ensure_slot(&mut self, slot_idx: usize) -> &mut DuckyKeySlot {
        self.slots.entry(slot_idx).or_insert_with(|| DuckyKeySlot {
            slot: slot_idx,
            ..Default::default()
        })
    }
}

pub fn find_mounted_duckypad_sd() -> Option<PathBuf> {
    let mut candidate_roots = Vec::new();

    if let Ok(user) = std::env::var("USER") {
        candidate_roots.push(PathBuf::from(format!("/run/media/{user}")));
        candidate_roots.push(PathBuf::from(format!("/media/{user}")));
    }
    candidate_roots.push(PathBuf::from("/run/media"));
    candidate_roots.push(PathBuf::from("/media"));

    for root in candidate_roots {
        let Ok(entries) = fs::read_dir(&root) else {
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
        }
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
    fs::write(profile_info_path, content)
}

fn is_duckypad_sd_root(dir: &Path) -> bool {
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
