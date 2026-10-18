use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::actions::desktop_apps::{
    expand_tilde_str, find_terminal_emulator, scan_installed_desktop_apps, split_shell_words,
};
use crate::actions::web_api::{default_fkey_for_slot, encode_pgv31_trigger};
pub use crate::vm::{AppLaunchConfig, APP_LAUNCH_META_PREFIX};

pub const APP_LAUNCH_BLOCK_BEGIN: &str = "// BEGIN_DUCKYDECK_APP_LAUNCH";
pub const APP_LAUNCH_BLOCK_END: &str = "// END_DUCKYDECK_APP_LAUNCH";

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
            "F13"
                | "F14"
                | "F15"
                | "F16"
                | "F17"
                | "F18"
                | "F19"
                | "F20"
                | "F21"
                | "F22"
                | "F23"
                | "F24"
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
        vec!["sh".to_string(), "-c".to_string(), full_cmd_display.clone()]
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
    let mut cmd = if Path::new("/.flatpak-info").exists() {
        let mut c = Command::new("flatpak-spawn");
        c.arg("--host").arg(&program);
        c
    } else {
        let mut c = Command::new(&program);
        if std::env::var_os("APPIMAGE").is_some() || std::env::var_os("APPDIR").is_some() {
            c.env_remove("LD_LIBRARY_PATH")
                .env_remove("QT_PLUGIN_PATH")
                .env_remove("QML2_IMPORT_PATH")
                .env_remove("QML_IMPORT_PATH")
                .env_remove("APPDIR")
                .env_remove("APPIMAGE");
        }
        c
    };
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
    } else if let Some(hp) = crate::paths::home_dir() {
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
                let _ = child.wait(); // ignore: reaping detached launcher child process
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
