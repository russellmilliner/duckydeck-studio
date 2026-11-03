use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
#[allow(clippy::upper_case_acronyms)]
mod win32_input {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Once;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct POINT {
        pub x: i32,
        pub y: i32,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct KBDLLHOOKSTRUCT {
        pub vk_code: u32,
        pub scan_code: u32,
        pub flags: u32,
        pub time: u32,
        pub dw_extra_info: usize,
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct MSG {
        pub hwnd: isize,
        pub message: u32,
        pub w_param: usize,
        pub l_param: isize,
        pub time: u32,
        pub pt: POINT,
    }

    pub type LowLevelKeyboardProc =
        unsafe extern "system" fn(n_code: i32, w_param: usize, l_param: isize) -> isize;

    #[link(name = "user32")]
    extern "system" {
        pub fn GetAsyncKeyState(vKey: i32) -> i16;
        pub fn GetCursorPos(lpPoint: *mut POINT) -> i32;
        pub fn keybd_event(bVk: u8, bScan: u8, dwFlags: u32, dwExtraInfo: usize);
        pub fn MapVirtualKeyW(uCode: u32, uMapType: u32) -> u32;
        pub fn SetWindowsHookExW(
            idHook: i32,
            lpfn: LowLevelKeyboardProc,
            hMod: isize,
            dwThreadId: u32,
        ) -> isize;
        pub fn CallNextHookEx(hhk: isize, nCode: i32, wParam: usize, lParam: isize) -> isize;
        pub fn GetMessageW(
            lpMsg: *mut MSG,
            hWnd: isize,
            wMsgFilterMin: u32,
            wMsgFilterMax: u32,
        ) -> i32;
    }

    pub const KEYEVENTF_EXTENDEDKEY: u32 = 0x0001;
    pub const KEYEVENTF_KEYUP: u32 = 0x0002;
    pub const MAPVK_VK_TO_VSC: u32 = 0;
    const WH_KEYBOARD_LL: i32 = 13;
    const LLKHF_INJECTED: u32 = 0x0010;
    const WM_KEYDOWN: usize = 0x0100;
    const WM_KEYUP: usize = 0x0101;
    const WM_SYSKEYDOWN: usize = 0x0104;
    const WM_SYSKEYUP: usize = 0x0105;

    static PHYSICAL_VK_DOWN: [AtomicBool; 256] = [const { AtomicBool::new(false) }; 256];
    static SUPPRESSED_VK_MASK: [AtomicBool; 256] = [const { AtomicBool::new(false) }; 256];
    static LL_HOOK_INIT: Once = Once::new();

    unsafe extern "system" fn low_level_kbd_proc(
        n_code: i32,
        w_param: usize,
        l_param: isize,
    ) -> isize {
        if n_code >= 0 && l_param != 0 {
            let kb = &*(l_param as *const KBDLLHOOKSTRUCT);
            let vk = (kb.vk_code & 0xFF) as usize;
            let injected = (kb.flags & LLKHF_INJECTED) != 0;
            let is_down = w_param == WM_KEYDOWN || w_param == WM_SYSKEYDOWN;
            let is_up = w_param == WM_KEYUP || w_param == WM_SYSKEYUP;

            if !injected && vk < 256 {
                if is_down {
                    PHYSICAL_VK_DOWN[vk].store(true, Ordering::SeqCst);
                    // Swallow hardware typematic auto-repeat WM_KEYDOWN while this modifier
                    // key is suppressed for an active Hold / Double-Tap+Hold layer shift.
                    if SUPPRESSED_VK_MASK[vk].load(Ordering::SeqCst) {
                        return 1;
                    }
                } else if is_up {
                    PHYSICAL_VK_DOWN[vk].store(false, Ordering::SeqCst);
                    SUPPRESSED_VK_MASK[vk].store(false, Ordering::SeqCst);
                    match vk {
                        0xA2 | 0xA3 => SUPPRESSED_VK_MASK[0x11].store(false, Ordering::SeqCst),
                        0xA4 | 0xA5 => SUPPRESSED_VK_MASK[0x12].store(false, Ordering::SeqCst),
                        0xA0 | 0xA1 => SUPPRESSED_VK_MASK[0x10].store(false, Ordering::SeqCst),
                        _ => {}
                    }
                }
            }
        }
        CallNextHookEx(0, n_code, w_param, l_param)
    }

    pub fn ensure_ll_hook_started() {
        LL_HOOK_INIT.call_once(|| {
            let _ = std::thread::Builder::new()
                .name("win32-ll-kbd-hook".to_string())
                .spawn(|| unsafe {
                    let _hook = SetWindowsHookExW(WH_KEYBOARD_LL, low_level_kbd_proc, 0, 0);
                    let mut msg = MSG::default();
                    while GetMessageW(&mut msg, 0, 0, 0) > 0 {}
                });
        });
    }

    pub fn is_physical_vk_down(vk: usize) -> bool {
        vk < 256 && PHYSICAL_VK_DOWN[vk].load(Ordering::SeqCst)
    }

    pub fn set_physical_vk_down(vk: usize, down: bool) {
        if vk < 256 {
            PHYSICAL_VK_DOWN[vk].store(down, Ordering::SeqCst);
        }
    }

    pub fn is_vk_suppressed(vk: usize) -> bool {
        vk < 256 && SUPPRESSED_VK_MASK[vk].load(Ordering::SeqCst)
    }

    pub fn clear_suppressed_modifiers() {
        for flag in &SUPPRESSED_VK_MASK {
            flag.store(false, Ordering::SeqCst);
        }
    }

    /// Injects synthetic KEYUP events for the given trigger key and enables typematic repeat
    /// suppression in `WH_KEYBOARD_LL` so a held modifier key (`Ctrl`, `Alt`, `Shift`, `Win`)
    /// does not bleed into duckyPad macros (e.g. `ALT F4`) or prematurely deactivate the layer.
    pub fn suppress_held_modifier_trigger(trigger_key: &str) {
        ensure_ll_hook_started();
        let vks: &[(u8, bool)] = match crate::kwin::auto_switcher::normalize_modifier_trigger_key(
            trigger_key,
        )
        .as_str()
        {
            "LCTRL" => &[(0xA2, false), (0x11, false)],
            "RCTRL" => &[(0xA3, true), (0x11, false)],
            "LALT" => &[(0xA4, false), (0x12, false)],
            "RALT" => &[(0xA5, true), (0x12, false)],
            "LSHIFT" => &[(0xA0, false), (0x10, false)],
            "RSHIFT" => &[(0xA1, false), (0x10, false)],
            "SUPER" => &[(0x5B, true), (0x5C, true)],
            "CTRL_ALT" => &[
                (0xA2, false),
                (0xA3, true),
                (0x11, false),
                (0xA4, false),
                (0xA5, true),
                (0x12, false),
            ],
            "CTRL_SUPER" => &[
                (0xA2, false),
                (0xA3, true),
                (0x11, false),
                (0x5B, true),
                (0x5C, true),
            ],
            _ => &[],
        };
        for &(vk, _extended) in vks {
            SUPPRESSED_VK_MASK[vk as usize].store(true, Ordering::SeqCst);
        }
        for &(vk, extended) in vks {
            // SAFETY: MapVirtualKeyW and keybd_event are standard Win32 user32 input helpers.
            unsafe {
                let scan = MapVirtualKeyW(vk as u32, MAPVK_VK_TO_VSC) as u8;
                let flags = KEYEVENTF_KEYUP | if extended { KEYEVENTF_EXTENDEDKEY } else { 0 };
                keybd_event(vk, scan, flags, 0);
            }
        }
    }

    pub fn vk_to_duckyscript_key(vk: u8) -> Option<&'static str> {
        match vk {
            0x01 => Some("LMOUSE"),
            0x02 => Some("RMOUSE"),
            0x04 => Some("MMOUSE"),
            0x05 => Some("BMOUSE"),
            0x06 => Some("FMOUSE"),
            0x08 => Some("BACKSPACE"),
            0x09 => Some("TAB"),
            0x0D => Some("ENTER"),
            0x13 => Some("PAUSE"),
            0x14 => Some("CAPSLOCK"),
            0x1B => Some("ESC"),
            0x20 => Some("SPACE"),
            0x21 => Some("PAGEUP"),
            0x22 => Some("PAGEDOWN"),
            0x23 => Some("END"),
            0x24 => Some("HOME"),
            0x25 => Some("LEFT"),
            0x26 => Some("UP"),
            0x27 => Some("RIGHT"),
            0x28 => Some("DOWN"),
            0x2C => Some("PRINTSCREEN"),
            0x2D => Some("INSERT"),
            0x2E => Some("DELETE"),
            0x30 => Some("0"),
            0x31 => Some("1"),
            0x32 => Some("2"),
            0x33 => Some("3"),
            0x34 => Some("4"),
            0x35 => Some("5"),
            0x36 => Some("6"),
            0x37 => Some("7"),
            0x38 => Some("8"),
            0x39 => Some("9"),
            0x41 => Some("a"),
            0x42 => Some("b"),
            0x43 => Some("c"),
            0x44 => Some("d"),
            0x45 => Some("e"),
            0x46 => Some("f"),
            0x47 => Some("g"),
            0x48 => Some("h"),
            0x49 => Some("i"),
            0x4A => Some("j"),
            0x4B => Some("k"),
            0x4C => Some("l"),
            0x4D => Some("m"),
            0x4E => Some("n"),
            0x4F => Some("o"),
            0x50 => Some("p"),
            0x51 => Some("q"),
            0x52 => Some("r"),
            0x53 => Some("s"),
            0x54 => Some("t"),
            0x55 => Some("u"),
            0x56 => Some("v"),
            0x57 => Some("w"),
            0x58 => Some("x"),
            0x59 => Some("y"),
            0x5A => Some("z"),
            0x5B => Some("WINDOWS"),
            0x5C => Some("RWINDOWS"),
            0x5D => Some("MENU"),
            0x60 => Some("KP_0"),
            0x61 => Some("KP_1"),
            0x62 => Some("KP_2"),
            0x63 => Some("KP_3"),
            0x64 => Some("KP_4"),
            0x65 => Some("KP_5"),
            0x66 => Some("KP_6"),
            0x67 => Some("KP_7"),
            0x68 => Some("KP_8"),
            0x69 => Some("KP_9"),
            0x6A => Some("KP_ASTERISK"),
            0x6B => Some("KP_PLUS"),
            0x6D => Some("KP_MINUS"),
            0x6E => Some("KP_DOT"),
            0x6F => Some("KP_SLASH"),
            0x70 => Some("F1"),
            0x71 => Some("F2"),
            0x72 => Some("F3"),
            0x73 => Some("F4"),
            0x74 => Some("F5"),
            0x75 => Some("F6"),
            0x76 => Some("F7"),
            0x77 => Some("F8"),
            0x78 => Some("F9"),
            0x79 => Some("F10"),
            0x7A => Some("F11"),
            0x7B => Some("F12"),
            0x7C => Some("F13"),
            0x7D => Some("F14"),
            0x7E => Some("F15"),
            0x7F => Some("F16"),
            0x80 => Some("F17"),
            0x81 => Some("F18"),
            0x82 => Some("F19"),
            0x83 => Some("F20"),
            0x84 => Some("F21"),
            0x85 => Some("F22"),
            0x86 => Some("F23"),
            0x87 => Some("F24"),
            0x90 => Some("NUMLOCK"),
            0x91 => Some("SCROLLLOCK"),
            0xA0 => Some("SHIFT"),
            0xA1 => Some("RSHIFT"),
            0xA2 => Some("CTRL"),
            0xA3 => Some("RCTRL"),
            0xA4 => Some("ALT"),
            0xA5 => Some("RALT"),
            0xBA => Some(";"),
            0xBB => Some("="),
            0xBC => Some(","),
            0xBD => Some("-"),
            0xBE => Some("."),
            0xBF => Some("/"),
            0xC0 => Some("`"),
            0xDB => Some("["),
            0xDC => Some("\\"),
            0xDD => Some("]"),
            0xDE => Some("'"),
            _ => None,
        }
    }
}

pub struct DuckypadEvdevListener {
    files: Vec<(PathBuf, fs::File)>,
    #[cfg(unix)]
    hidraw_files: Vec<(PathBuf, fs::File)>,
    #[cfg(unix)]
    hidraw_keys_down: bool,
    last_scan: std::time::Instant,
    #[cfg(target_os = "windows")]
    prev_fkeys: u16,
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
            #[cfg(unix)]
            hidraw_files: Vec::new(),
            #[cfg(unix)]
            hidraw_keys_down: false,
            last_scan: std::time::Instant::now() - std::time::Duration::from_secs(10),
            #[cfg(target_os = "windows")]
            prev_fkeys: 0,
        };
        s.refresh_nodes();
        s
    }

    pub fn refresh_nodes(&mut self) {
        self.last_scan = std::time::Instant::now();
        self.files.retain(|(p, _)| p.exists());
        #[cfg(unix)]
        self.hidraw_files.retain(|(p, _)| p.exists());

        #[cfg(target_os = "windows")]
        {
            // Drain any stale GetAsyncKeyState LSB transition bits for F13..F24 on startup
            for i in 0..12 {
                // SAFETY: GetAsyncKeyState is a safe Win32 query.
                let state = unsafe { win32_input::GetAsyncKeyState(0x7C + i) as u16 };
                if (state & 0x8000) != 0 {
                    self.prev_fkeys |= 1u16 << i;
                }
            }
        }

        #[cfg(unix)]
        {
            use std::io::Read;
            use std::os::unix::fs::OpenOptionsExt;
            if let Ok(entries) = fs::read_dir("/sys/class/input") {
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

            // Fallback: also open duckyPad `/dev/hidraw*` nodes in O_RDONLY|O_NONBLOCK so users
            // without `input` group membership still detect F13..F24 Web API / App Launch triggers
            // and 1-key One-Shot Modifier Layer completion via Report ID 1/2/3 packets.
            if self.files.is_empty() {
                let mut drain = [0u8; 64];
                for (dev_path, _pid, _uniq) in
                    crate::hardware::storage::find_all_duckypad_hidraw_nodes()
                {
                    if self.hidraw_files.iter().any(|(p, _)| p == &dev_path) {
                        continue;
                    }
                    if let Ok(mut file) = fs::OpenOptions::new()
                        .read(true)
                        .custom_flags(0o4000) // O_NONBLOCK
                        .open(&dev_path)
                    {
                        while file.read(&mut drain).is_ok() {}
                        self.hidraw_files.push((dev_path, file));
                    }
                }
            }
        }
    }

    pub fn poll_fkey_triggers(&mut self) -> Vec<String> {
        #[cfg(target_os = "windows")]
        {
            let mut triggered = Vec::new();
            let mut held_count: u32 = 0;
            for i in 0..12 {
                // SAFETY: GetAsyncKeyState is a safe Win32 query.
                let state = unsafe { win32_input::GetAsyncKeyState(0x7C + i) as u16 };
                let is_down = (state & 0x8000) != 0;
                let was_pressed_since_last = (state & 0x0001) != 0;
                let was_down = (self.prev_fkeys & (1u16 << i)) != 0;
                if is_down {
                    held_count += 1;
                }
                if (is_down && !was_down) || (!is_down && !was_down && was_pressed_since_last) {
                    let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                    crate::hardware::storage::LAST_EVDEV_KEY_ACTIVITY_MS
                        .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                    crate::hardware::storage::LAST_EVDEV_KEYDOWN_MS
                        .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                    triggered.push(format!("F{}", 13 + i));
                } else if !is_down && was_down {
                    let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                    crate::hardware::storage::LAST_EVDEV_KEY_ACTIVITY_MS
                        .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                }
                if is_down {
                    self.prev_fkeys |= 1u16 << i;
                } else {
                    self.prev_fkeys &= !(1u16 << i);
                }
            }
            crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                .store(held_count, std::sync::atomic::Ordering::SeqCst);
            return triggered;
        }

        #[cfg(not(target_os = "windows"))]
        {
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
                            if ev_type == 1 {
                                let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                                crate::hardware::storage::LAST_EVDEV_KEY_ACTIVITY_MS
                                    .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                                if ev_value == 1 {
                                    crate::hardware::storage::LAST_EVDEV_KEYDOWN_MS
                                        .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                                    let cur = crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                                        .load(std::sync::atomic::Ordering::SeqCst);
                                    crate::hardware::storage::EVDEV_KEYS_HELD_COUNT.store(
                                        cur.saturating_add(1).min(16),
                                        std::sync::atomic::Ordering::SeqCst,
                                    );
                                } else if ev_value == 0 {
                                    let cur = crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                                        .load(std::sync::atomic::Ordering::SeqCst);
                                    crate::hardware::storage::EVDEV_KEYS_HELD_COUNT.store(
                                        cur.saturating_sub(1),
                                        std::sync::atomic::Ordering::SeqCst,
                                    );
                                }
                                // EV_KEY == 1, value == 1 (key press), KEY_F13 (183)..=KEY_F24 (194)
                                if ev_value == 1 && (183..=194).contains(&ev_code) {
                                    let f_num = 13 + (ev_code - 183);
                                    triggered.push(format!("F{f_num}"));
                                }
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

            if !drop_indices.is_empty() {
                crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                    .store(0, std::sync::atomic::Ordering::SeqCst);
            }
            for idx in drop_indices.into_iter().rev() {
                self.files.remove(idx);
            }

            if self.files.is_empty() && !self.hidraw_files.is_empty() {
                let mut hbuf = [0u8; 64];
                let mut drop_hidraw = Vec::new();
                for (idx, (_, file)) in self.hidraw_files.iter_mut().enumerate() {
                    loop {
                        match file.read(&mut hbuf) {
                            Ok(n) if n >= 2 => {
                                let report_id = hbuf[0];
                                // Report ID 1: Keyboard [1, mods, reserved, k0..k5]
                                // Report ID 2: Media/Consumer [2, bits]
                                // Report ID 3: Mouse [3, btns, dx, dy, wheel, hwheel]
                                if report_id == 1 && n >= 3 {
                                    let any_down = hbuf[1] != 0
                                        || hbuf[3..n.min(9)].iter().any(|&b| b != 0);
                                    let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                                    crate::hardware::storage::LAST_EVDEV_KEY_ACTIVITY_MS
                                        .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                                    if any_down {
                                        crate::hardware::storage::LAST_EVDEV_KEYDOWN_MS
                                            .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                                        crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                                            .store(1, std::sync::atomic::Ordering::SeqCst);
                                        self.hidraw_keys_down = true;
                                        for &u in &hbuf[3..n.min(9)] {
                                            if (0x68..=0x73).contains(&u) {
                                                let f_num = 13 + (u - 0x68);
                                                triggered.push(format!("F{f_num}"));
                                            }
                                        }
                                    } else if self.hidraw_keys_down {
                                        self.hidraw_keys_down = false;
                                        crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                                            .store(0, std::sync::atomic::Ordering::SeqCst);
                                    }
                                } else if report_id == 2 || report_id == 3 {
                                    let any_down = hbuf[1..n.min(6)].iter().any(|&b| b != 0);
                                    let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                                    crate::hardware::storage::LAST_EVDEV_KEY_ACTIVITY_MS
                                        .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                                    if any_down {
                                        crate::hardware::storage::LAST_EVDEV_KEYDOWN_MS
                                            .store(now_ms, std::sync::atomic::Ordering::SeqCst);
                                        crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                                            .store(1, std::sync::atomic::Ordering::SeqCst);
                                        self.hidraw_keys_down = true;
                                    } else if self.hidraw_keys_down {
                                        self.hidraw_keys_down = false;
                                        crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                                            .store(0, std::sync::atomic::Ordering::SeqCst);
                                    }
                                }
                            }
                            Ok(_) => break,
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                            Err(_) => {
                                drop_hidraw.push(idx);
                                break;
                            }
                        }
                    }
                }
                if !drop_hidraw.is_empty() {
                    self.hidraw_keys_down = false;
                    crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
                        .store(0, std::sync::atomic::Ordering::SeqCst);
                }
                for idx in drop_hidraw.into_iter().rev() {
                    self.hidraw_files.remove(idx);
                }
            }

            triggered
        }
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
        #[cfg(target_os = "windows")]
        {
            vec![
                "cmd.exe".to_string(),
                "/C".to_string(),
                full_cmd_display.clone(),
            ]
        }
        #[cfg(not(target_os = "windows"))]
        {
            vec!["sh".to_string(), "-c".to_string(), full_cmd_display.clone()]
        }
    } else if cfg!(target_os = "windows")
        && (raw_cmd.eq_ignore_ascii_case("xdg-open")
            || (raw_cmd.to_ascii_lowercase().ends_with(".lnk") && raw_args.is_empty()))
    {
        let target_arg = if raw_cmd.eq_ignore_ascii_case("xdg-open") {
            raw_args.to_string()
        } else {
            raw_cmd.to_string()
        };
        vec![
            "cmd.exe".to_string(),
            "/C".to_string(),
            "start".to_string(),
            String::new(),
            target_arg,
        ]
    } else if cfg!(target_os = "windows") && raw_cmd.eq_ignore_ascii_case("dolphin") {
        let mut v = vec!["explorer.exe".to_string()];
        if !raw_args.is_empty() {
            v.extend(split_shell_words(raw_args));
        }
        v
    } else if cfg!(target_os = "windows") && raw_cmd.eq_ignore_ascii_case("konsole") {
        if let Some((term_bin, _)) = find_terminal_emulator() {
            vec![term_bin]
        } else {
            vec!["cmd.exe".to_string()]
        }
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
    #[cfg(target_os = "windows")]
    prev_keys: [bool; 256],
    #[cfg(target_os = "windows")]
    last_cursor: Option<(i32, i32)>,
}

impl Default for GlobalMacroEvdevRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalMacroEvdevRecorder {
    pub fn new() -> Self {
        let mut s = Self {
            files: Vec::new(),
            #[cfg(target_os = "windows")]
            prev_keys: [false; 256],
            #[cfg(target_os = "windows")]
            last_cursor: None,
        };
        s.open_all_input_nodes();
        s
    }

    pub fn open_all_input_nodes(&mut self) {
        self.files.clear();
        #[cfg(target_os = "windows")]
        {
            for vk in 1..256usize {
                // SAFETY: GetAsyncKeyState is a safe Win32 query.
                let state = unsafe { win32_input::GetAsyncKeyState(vk as i32) as u16 };
                self.prev_keys[vk] = (state & 0x8000) != 0;
            }
            let mut pt = win32_input::POINT::default();
            // SAFETY: GetCursorPos writes to a valid stack struct.
            if unsafe { win32_input::GetCursorPos(&mut pt) } != 0 {
                self.last_cursor = Some((pt.x, pt.y));
            }
        }
        #[cfg(unix)]
        {
            use std::io::Read;
            use std::os::unix::fs::OpenOptionsExt;
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
    }

    pub fn device_count(&self) -> usize {
        #[cfg(target_os = "windows")]
        {
            2
        }
        #[cfg(not(target_os = "windows"))]
        {
            self.files.len()
        }
    }

    pub fn poll_events(&mut self) -> Vec<RecordedMacroInputEvent> {
        #[cfg(target_os = "windows")]
        {
            let mut out = Vec::new();
            let mut accum_dx: i32 = 0;
            let mut accum_dy: i32 = 0;

            let mut pt = win32_input::POINT::default();
            // SAFETY: GetCursorPos writes to a valid stack struct.
            if unsafe { win32_input::GetCursorPos(&mut pt) } != 0 {
                if let Some((lx, ly)) = self.last_cursor {
                    accum_dx = pt.x - lx;
                    accum_dy = pt.y - ly;
                }
                self.last_cursor = Some((pt.x, pt.y));
            }

            for vk in 1..256usize {
                let Some(duck_key) = win32_input::vk_to_duckyscript_key(vk as u8) else {
                    continue;
                };
                // SAFETY: GetAsyncKeyState is a safe Win32 query.
                let state = unsafe { win32_input::GetAsyncKeyState(vk as i32) as u16 };
                let is_down = (state & 0x8000) != 0;
                let was_down = self.prev_keys[vk];
                if is_down != was_down {
                    self.prev_keys[vk] = is_down;
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
                    let is_mouse_btn = (0x01..=0x06).contains(&vk);
                    out.push(RecordedMacroInputEvent {
                        kind: if is_mouse_btn {
                            if is_down {
                                "mouse_down".to_string()
                            } else {
                                "mouse_up".to_string()
                            }
                        } else if is_down {
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

            if accum_dx != 0 || accum_dy != 0 {
                out.push(RecordedMacroInputEvent {
                    kind: "mouse_move".to_string(),
                    key: String::new(),
                    dx: accum_dx,
                    dy: accum_dy,
                });
            }
            return out;
        }

        #[cfg(not(target_os = "windows"))]
        {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidrawKeyboardFormat {
    /// Standard 8-byte boot keyboard report without Report ID:
    /// `buf[0]` = modifier bitmask (0xE0..0xE7), `buf[2..]` = key usage array (0x00 = none).
    BootNoReportId,
    /// Report-ID prefixed 6KRO/array keyboard report:
    /// `buf[0] == report_id`, `buf[1]` = modifier bitmask, `buf[2..]` = key usage array (0x00 = none).
    ArrayWithReportId(u8),
    /// Report-ID prefixed NKRO bitmap keyboard report (e.g. QMK/VIA/Keychron shared EP):
    /// `buf[0] == report_id`, `buf[1]` = modifier bitmask, `buf[2..]` = bitfield of HID usages 0x00..0xEF.
    NkroBitmapWithReportId(u8),
}

pub fn detect_hidraw_keyboard_format(desc: &[u8]) -> Option<HidrawKeyboardFormat> {
    // Look for Usage Page (Generic Desktop 0x01), Usage (Keyboard 0x06), Collection (Application 0x01)
    let sig = [0x05u8, 0x01, 0x09, 0x06, 0xA1, 0x01];
    let pos = desc.windows(sig.len()).position(|w| w == sig)?;
    let tail = &desc[pos + sig.len()..];
    let section = &tail[..tail.len().min(64)];

    // Check if a Report ID tag (0x85 <id>) appears before the first Input item (0x81)
    let first_input_pos = section.iter().position(|&b| b == 0x81).unwrap_or(section.len());
    let pre_input = &section[..first_input_pos];
    let report_id = pre_input
        .windows(2)
        .find(|w| w[0] == 0x85 && w[1] != 0)
        .map(|w| w[1]);

    if let Some(id) = report_id {
        let is_nkro_bitmap = section.windows(4).any(|w| {
            (w[0] == 0x95 && w[1] >= 0x60 && w[2] == 0x75 && w[3] == 0x01)
                || (w[0] == 0x75 && w[1] == 0x01 && w[2] == 0x95 && w[3] >= 0x60)
        });
        if is_nkro_bitmap {
            Some(HidrawKeyboardFormat::NkroBitmapWithReportId(id))
        } else {
            Some(HidrawKeyboardFormat::ArrayWithReportId(id))
        }
    } else {
        Some(HidrawKeyboardFormat::BootNoReportId)
    }
}

pub fn hid_usage_to_evdev(usage: u8) -> Option<u16> {
    match usage {
        0x00..=0x03 => None, // No event / ErrorRollOver / POSTFail / ErrorUndefined
        0x39 => Some(58),    // Caps Lock -> KEY_CAPSLOCK
        0x47 => Some(70),    // Scroll Lock -> KEY_SCROLLLOCK
        0x48 => Some(119),   // Pause -> KEY_PAUSE
        0x65 | 0x76 => Some(127), // Keyboard Application / Menu -> KEY_COMPOSE
        0x68..=0x73 => Some(183 + (usage - 0x68) as u16), // F13..F24 -> 183..194
        0xE0 => Some(29),    // Left Control -> KEY_LEFTCTRL
        0xE1 => Some(42),    // Left Shift -> KEY_LEFTSHIFT
        0xE2 => Some(56),    // Left Alt -> KEY_LEFTALT
        0xE3 => Some(125),   // Left GUI -> KEY_LEFTMETA
        0xE4 => Some(97),    // Right Control -> KEY_RIGHTCTRL
        0xE5 => Some(54),    // Right Shift -> KEY_RIGHTSHIFT
        0xE6 => Some(100),   // Right Alt -> KEY_RIGHTALT
        0xE7 => Some(126),   // Right GUI -> KEY_RIGHTMETA
        other => Some(1000 + other as u16), // Distinct non-trigger typing key for chord guard
    }
}

fn insert_hid_modifier_bits(mod_bits: u8, out: &mut std::collections::HashSet<u16>) {
    if (mod_bits & 0x01) != 0 {
        out.insert(29); // LCTRL
    }
    if (mod_bits & 0x02) != 0 {
        out.insert(42); // LSHIFT
    }
    if (mod_bits & 0x04) != 0 {
        out.insert(56); // LALT
    }
    if (mod_bits & 0x08) != 0 {
        out.insert(125); // LMETA (SUPER)
    }
    if (mod_bits & 0x10) != 0 {
        out.insert(97); // RCTRL
    }
    if (mod_bits & 0x20) != 0 {
        out.insert(54); // RSHIFT
    }
    if (mod_bits & 0x40) != 0 {
        out.insert(100); // RALT
    }
    if (mod_bits & 0x80) != 0 {
        out.insert(126); // RMETA (SUPER)
    }
}

pub fn parse_hidraw_keyboard_report(
    format: HidrawKeyboardFormat,
    buf: &[u8],
) -> Option<std::collections::HashSet<u16>> {
    let mut keys = std::collections::HashSet::new();
    match format {
        HidrawKeyboardFormat::BootNoReportId => {
            if buf.len() < 3 {
                return None;
            }
            insert_hid_modifier_bits(buf[0], &mut keys);
            for &u in &buf[2..buf.len().min(16)] {
                if let Some(code) = hid_usage_to_evdev(u) {
                    keys.insert(code);
                }
            }
            Some(keys)
        }
        HidrawKeyboardFormat::ArrayWithReportId(report_id) => {
            if buf.len() < 3 || buf[0] != report_id {
                return None;
            }
            insert_hid_modifier_bits(buf[1], &mut keys);
            for &u in &buf[2..buf.len().min(16)] {
                if let Some(code) = hid_usage_to_evdev(u) {
                    keys.insert(code);
                }
            }
            Some(keys)
        }
        HidrawKeyboardFormat::NkroBitmapWithReportId(report_id) => {
            if buf.len() < 4 || buf[0] != report_id {
                return None;
            }
            insert_hid_modifier_bits(buf[1], &mut keys);
            let bitmap = &buf[2..buf.len().min(32)];
            for (byte_idx, &b) in bitmap.iter().enumerate() {
                if b == 0 {
                    continue;
                }
                for bit in 0..8u8 {
                    if (b & (1 << bit)) != 0 {
                        let usage = (byte_idx as u8).saturating_mul(8).saturating_add(bit);
                        if let Some(code) = hid_usage_to_evdev(usage) {
                            keys.insert(code);
                        }
                    }
                }
            }
            Some(keys)
        }
    }
}

#[cfg(unix)]
extern "C" {
    fn ioctl(fd: std::ffi::c_int, request: std::ffi::c_ulong, ...) -> std::ffi::c_int;
}

#[cfg(unix)]
struct UinputModifierSuppressor {
    file: fs::File,
}

#[cfg(unix)]
impl UinputModifierSuppressor {
    const UI_SET_EVBIT: std::ffi::c_ulong = 0x4004_5564;
    const UI_SET_KEYBIT: std::ffi::c_ulong = 0x4004_5565;
    const UI_DEV_SETUP: std::ffi::c_ulong = 0x405c_5503;
    const UI_DEV_CREATE: std::ffi::c_ulong = 0x5501;
    const UI_DEV_DESTROY: std::ffi::c_ulong = 0x5502;

    fn new() -> Option<Self> {
        use std::os::unix::fs::OpenOptionsExt;
        use std::os::unix::io::AsRawFd;

        let file = fs::OpenOptions::new()
            .write(true)
            .custom_flags(0o4000) // O_NONBLOCK
            .open("/dev/uinput")
            .ok()?;
        let fd = file.as_raw_fd();

        // SAFETY: Standard Linux uinput ioctl initialization on a valid /dev/uinput fd.
        unsafe {
            if ioctl(fd, Self::UI_SET_EVBIT, 1 as std::ffi::c_int) < 0 {
                return None;
            }
            // Register standard keyboard keybits (1..=127) plus F13..F24 (183..=194) so
            // systemd-udevd (`input_id`) tags this virtual device with `ID_INPUT_KEYBOARD=1`
            // and Wayland compositors (`libinput` / KWin) monitor its epoll fd.
            for code in 1..=127i32 {
                let _ = ioctl(fd, Self::UI_SET_KEYBIT, code as std::ffi::c_int);
            }
            for code in 183..=194i32 {
                let _ = ioctl(fd, Self::UI_SET_KEYBIT, code as std::ffi::c_int);
            }

            #[repr(C)]
            struct UinputSetup {
                bustype: u16,
                vendor: u16,
                product: u16,
                version: u16,
                name: [u8; 80],
                ff_effects_max: u32,
            }

            let mut setup = UinputSetup {
                bustype: 0x03, // BUS_USB
                vendor: 0x1209,
                product: 0xdd01,
                version: 1,
                name: [0u8; 80],
                ff_effects_max: 0,
            };
            let label = b"DuckyDeck Modifier Guard";
            setup.name[..label.len()].copy_from_slice(label);

            if ioctl(fd, Self::UI_DEV_SETUP, &setup as *const UinputSetup) < 0 {
                return None;
            }
            if ioctl(fd, Self::UI_DEV_CREATE) < 0 {
                return None;
            }
        }

        Some(Self { file })
    }

    fn emit_event(&mut self, ev_type: u16, ev_code: u16, ev_value: i32) {
        use std::io::Write;
        let mut pkt = [0u8; 24];
        pkt[16..18].copy_from_slice(&ev_type.to_ne_bytes());
        pkt[18..20].copy_from_slice(&ev_code.to_ne_bytes());
        pkt[20..24].copy_from_slice(&ev_value.to_ne_bytes());
        let _ = self.file.write_all(&pkt);
    }

    /// Synthesizes a momentary key-down + key-up release pulse on `/dev/uinput` so the Wayland/X11
    /// compositor (`libinput` / `xkbcommon`) clears the logical modifier bit (`Ctrl`, `Alt`, `Shift`,
    /// `Super`) while the user holds the physical modifier key to shift duckyPad layers.
    fn release_held_modifier_trigger(&mut self, trigger_key: &str) {
        let codes = trigger_key_evdev_codes(trigger_key);
        if codes.is_empty() {
            return;
        }
        for &code in codes {
            self.emit_event(1, code, 1); // EV_KEY press
        }
        self.emit_event(0, 0, 0); // EV_SYN / SYN_REPORT
        for &code in codes {
            self.emit_event(1, code, 0); // EV_KEY release
        }
        self.emit_event(0, 0, 0); // EV_SYN / SYN_REPORT
    }
}

#[cfg(unix)]
fn trigger_key_evdev_codes(trigger_key: &str) -> &'static [u16] {
    match crate::kwin::auto_switcher::normalize_modifier_trigger_key(trigger_key).as_str() {
        "LCTRL" => &[29],
        "RCTRL" => &[97],
        "LALT" => &[56],
        "RALT" => &[100],
        "LSHIFT" => &[42],
        "RSHIFT" => &[54],
        "SUPER" => &[125, 126],
        "CTRL_ALT" => &[29, 97, 56, 100],
        "CTRL_SUPER" => &[29, 97, 125, 126],
        _ => &[],
    }
}

/// Directly injects `EV_KEY` release (`value = 0`) + `SYN_REPORT` into the physical keyboard's
/// `/dev/input/eventN` kernel node (`drivers/input/evdev.c` -> `evdev_write` -> `input_inject_event`).
/// When `/dev/input/eventN` is writable (via `70-duckypad.rules` `uaccess` ACL or `input` group),
/// this clears the held modifier key inside `libinput` / `xkbcommon` across ALL Wayland and X11
/// compositors (COSMIC `cosmic-comp`, GNOME `mutter`, Linux Mint Cinnamon/MATE/Xfce, KWin, Sway, Hyprland)
/// while `/dev/hidraw*` continues tracking the physical USB/Bluetooth key state.
#[cfg(unix)]
fn try_inject_evdev_release_on_events(event_names: &[String], codes: &[u16]) -> bool {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;

    if event_names.is_empty() || codes.is_empty() {
        return false;
    }
    let mut any_ok = false;
    for ev in event_names {
        let dev_path = Path::new("/dev/input").join(ev);
        let Ok(mut file) = fs::OpenOptions::new()
            .write(true)
            .custom_flags(0o4000) // O_NONBLOCK
            .open(&dev_path)
        else {
            continue;
        };
        for &code in codes {
            let mut pkt = [0u8; 24];
            pkt[16..18].copy_from_slice(&1u16.to_ne_bytes()); // EV_KEY
            pkt[18..20].copy_from_slice(&code.to_ne_bytes());
            pkt[20..24].copy_from_slice(&0i32.to_ne_bytes()); // release (0)
            let _ = file.write_all(&pkt);
        }
        let mut syn = [0u8; 24];
        syn[16..18].copy_from_slice(&0u16.to_ne_bytes()); // EV_SYN / SYN_REPORT
        if file.write_all(&syn).is_ok() {
            any_ok = true;
        }
    }
    any_ok
}

/// On pure X11 sessions (e.g., Linux Mint Cinnamon/MATE/Xfce X11 or GNOME Xorg) where `/dev/input/event*`
/// is not directly writable, synthesizes a `KeyRelease` via `libXtst.so.6` (`XTestFakeKeyEvent`) on the
/// Xorg `Virtual core keyboard` master device (`x11_keycode = evdev_code + 8`).
#[cfg(unix)]
fn try_inject_x11_xtest_release(codes: &[u16]) {
    if codes.is_empty()
        || std::env::var_os("WAYLAND_DISPLAY").is_some()
        || std::env::var_os("DISPLAY").is_none()
    {
        return;
    }

    extern "C" {
        fn dlopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int) -> *mut std::ffi::c_void;
        fn dlsym(
            handle: *mut std::ffi::c_void,
            symbol: *const std::ffi::c_char,
        ) -> *mut std::ffi::c_void;
        fn dlclose(handle: *mut std::ffi::c_void) -> std::ffi::c_int;
    }

    // SAFETY: Dynamically loading standard X11/Xtst shared libraries when running in an X11 session.
    unsafe {
        let x11 = dlopen(c"libX11.so.6".as_ptr(), 1); // RTLD_LAZY = 1
        if x11.is_null() {
            return;
        }
        let xtst = dlopen(c"libXtst.so.6".as_ptr(), 1);
        if xtst.is_null() {
            let _ = dlclose(x11);
            return;
        }

        let open_sym = dlsym(x11, c"XOpenDisplay".as_ptr());
        let flush_sym = dlsym(x11, c"XFlush".as_ptr());
        let close_sym = dlsym(x11, c"XCloseDisplay".as_ptr());
        let fake_sym = dlsym(xtst, c"XTestFakeKeyEvent".as_ptr());

        if !open_sym.is_null()
            && !flush_sym.is_null()
            && !close_sym.is_null()
            && !fake_sym.is_null()
        {
            type XOpenDisplayFn =
                unsafe extern "C" fn(*const std::ffi::c_char) -> *mut std::ffi::c_void;
            type XFlushFn = unsafe extern "C" fn(*mut std::ffi::c_void) -> std::ffi::c_int;
            type XCloseDisplayFn = unsafe extern "C" fn(*mut std::ffi::c_void) -> std::ffi::c_int;
            type XTestFakeKeyEventFn = unsafe extern "C" fn(
                *mut std::ffi::c_void,
                std::ffi::c_uint,
                std::ffi::c_int,
                std::ffi::c_ulong,
            ) -> std::ffi::c_int;

            let x_open: XOpenDisplayFn = std::mem::transmute(open_sym);
            let x_flush: XFlushFn = std::mem::transmute(flush_sym);
            let x_close: XCloseDisplayFn = std::mem::transmute(close_sym);
            let x_fake_key: XTestFakeKeyEventFn = std::mem::transmute(fake_sym);

            let dpy = x_open(std::ptr::null());
            if !dpy.is_null() {
                for &code in codes {
                    let x11_keycode = (code as u32).saturating_add(8);
                    let _ = x_fake_key(dpy, x11_keycode, 0, 0); // is_press = False (0), delay = CurrentTime (0)
                }
                let _ = x_flush(dpy);
                let _ = x_close(dpy);
            }
        }

        let _ = dlclose(xtst);
        let _ = dlclose(x11);
    }
}

#[cfg(unix)]
impl Drop for UinputModifierSuppressor {
    fn drop(&mut self) {
        use std::os::unix::io::AsRawFd;
        // SAFETY: Destroying our own uinput device before closing the file descriptor.
        unsafe {
            let _ = ioctl(self.file.as_raw_fd(), Self::UI_DEV_DESTROY);
        }
    }
}

#[cfg(unix)]
struct KWinInputDeviceSuppressor {
    tx: std::sync::mpsc::SyncSender<(Vec<String>, bool, std::sync::mpsc::SyncSender<()>)>,
}

#[cfg(unix)]
impl KWinInputDeviceSuppressor {
    fn new() -> Option<Self> {
        let (tx, rx) =
            std::sync::mpsc::sync_channel::<(Vec<String>, bool, std::sync::mpsc::SyncSender<()>)>(
                16,
            );
        std::thread::Builder::new()
            .name("kwin-mod-guard".to_string())
            .spawn(move || {
                let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                else {
                    return;
                };
                rt.block_on(async move {
                    let mut conn_opt: Option<zbus::Connection> = None;
                    while let Ok((event_names, enabled, ack_tx)) = rx.recv() {
                        if conn_opt.is_none() {
                            conn_opt = zbus::Connection::session().await.ok();
                        }
                        if let Some(ref conn) = conn_opt {
                            for ev in &event_names {
                                let obj_path = format!("/org/kde/KWin/InputDevice/{ev}");
                                let res = conn
                                    .call_method(
                                        Some("org.kde.KWin"),
                                        obj_path.as_str(),
                                        Some("org.freedesktop.DBus.Properties"),
                                        "Set",
                                        &(
                                            "org.kde.KWin.InputDevice",
                                            "enabled",
                                            zbus::zvariant::Value::from(enabled),
                                        ),
                                    )
                                    .await;
                                if res.is_err() {
                                    conn_opt = None;
                                    break;
                                }
                            }
                        }
                        let _ = ack_tx.send(());
                    }
                });
            })
            .ok()?;
        Some(Self { tx })
    }

    fn set_events_enabled(&self, event_names: &[String], enabled: bool) {
        if event_names.is_empty() {
            return;
        }
        let (ack_tx, ack_rx) = std::sync::mpsc::sync_channel(1);
        if self
            .tx
            .try_send((event_names.to_vec(), enabled, ack_tx))
            .is_ok()
        {
            let _ = ack_rx.recv_timeout(std::time::Duration::from_millis(120));
        }
    }
}

#[cfg(unix)]
fn format_hyprland_device_name(raw_name: &str) -> String {
    raw_name
        .trim()
        .chars()
        .map(|c| {
            let lc = c.to_ascii_lowercase();
            if lc.is_ascii_alphanumeric() || lc == '_' || lc == '.' || lc == '-' {
                lc
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(unix)]
fn try_set_hyprland_events_enabled(event_sys_names: &[String], enabled: bool) -> bool {
    let Ok(sig) = std::env::var("HYPRLAND_INSTANCE_SIGNATURE") else {
        return false;
    };
    if sig.trim().is_empty() {
        return false;
    }
    let socket_path = std::env::var("XDG_RUNTIME_DIR")
        .ok()
        .map(|rt| PathBuf::from(rt).join("hypr").join(&sig).join(".socket.sock"))
        .filter(|p| p.exists())
        .or_else(|| {
            let fallback = PathBuf::from("/tmp/hypr").join(&sig).join(".socket.sock");
            fallback.exists().then_some(fallback)
        });
    let Some(sock_path) = socket_path else {
        return false;
    };

    let mut toggled_any = false;
    let mut seen_names = std::collections::HashSet::new();
    let val_str = if enabled { "true" } else { "false" };

    for ev in event_sys_names {
        let name_path = Path::new("/sys/class/input").join(ev).join("device/name");
        let Ok(raw_name) = fs::read_to_string(name_path) else {
            continue;
        };
        let hypr_name = format_hyprland_device_name(&raw_name);
        if hypr_name.is_empty() || !seen_names.insert(hypr_name.clone()) {
            continue;
        }
        if let Ok(mut stream) = std::os::unix::net::UnixStream::connect(&sock_path) {
            let _ = stream.set_write_timeout(Some(std::time::Duration::from_millis(40)));
            let _ = stream.set_read_timeout(Some(std::time::Duration::from_millis(40)));
            let cmd = format!("keyword device[{hypr_name}]:enabled {val_str}");
            if std::io::Write::write_all(&mut stream, cmd.as_bytes()).is_ok() {
                let mut resp = [0u8; 64];
                let _ = std::io::Read::read(&mut stream, &mut resp);
                toggled_any = true;
            }
        }
    }
    toggled_any
}

#[cfg(unix)]
fn is_standard_keyboard_key_caps(key_caps_raw: &str) -> bool {
    let Some(last_word) = key_caps_raw.split_whitespace().last() else {
        return false;
    };
    let Ok(low64) = u64::from_str_radix(last_word, 16) else {
        return false;
    };
    // Bit 29 is KEY_LEFTCTRL (29); standard keyboards set bits 1..31 (0xfffffffe).
    (low64 & (1u64 << 29)) != 0
}

#[cfg(unix)]
fn resolve_hidraw_keyboard_events_and_group(sys_dev_dir: &Path) -> (Vec<String>, String) {
    let uevent = fs::read_to_string(sys_dev_dir.join("uevent")).unwrap_or_default();
    let group_key = uevent
        .lines()
        .find_map(|line| line.strip_prefix("HID_ID="))
        .or_else(|| uevent.lines().find_map(|line| line.strip_prefix("HID_NAME=")))
        .unwrap_or("")
        .trim()
        .to_ascii_uppercase();

    let mut events = Vec::new();
    if let Ok(inputs) = fs::read_dir(sys_dev_dir.join("input")) {
        for inp in inputs.flatten() {
            let inp_path = inp.path();
            let dev_name_lower = fs::read_to_string(inp_path.join("name"))
                .unwrap_or_default()
                .trim()
                .to_ascii_lowercase();
            if dev_name_lower.ends_with(" mouse")
                || dev_name_lower.contains("duckypad")
                || dev_name_lower.contains("duckydeck")
            {
                continue;
            }
            let key_caps = fs::read_to_string(inp_path.join("capabilities/key"))
                .unwrap_or_default();
            if !is_standard_keyboard_key_caps(&key_caps) {
                continue;
            }
            if let Ok(sub_entries) = fs::read_dir(&inp_path) {
                for sub in sub_entries.flatten() {
                    let sname = sub.file_name().to_string_lossy().to_string();
                    if sname.starts_with("event") && !events.contains(&sname) {
                        events.push(sname);
                    }
                }
            }
        }
    }
    (events, group_key)
}

#[cfg(unix)]
struct HidrawKeyboardNode {
    path: PathBuf,
    file: fs::File,
    format: HidrawKeyboardFormat,
    held_keys: std::collections::HashSet<u16>,
    event_sys_names: Vec<String>,
    device_group_key: String,
}

pub struct HostKeyboardModifierListener {
    #[cfg(unix)]
    files: Vec<(PathBuf, fs::File)>,
    #[cfg(unix)]
    hidraw_nodes: Vec<HidrawKeyboardNode>,
    #[cfg(unix)]
    evdev_held_keys: std::collections::HashSet<u16>,
    #[cfg(unix)]
    uinput_suppressor: Option<UinputModifierSuppressor>,
    #[cfg(unix)]
    kwin_suppressor: Option<KWinInputDeviceSuppressor>,
    #[cfg(unix)]
    rescan_tick: u32,
    held_keys: std::collections::HashSet<u16>,
    last_suppressed_duckypad_keydown_ms: u64,
    #[cfg(target_os = "windows")]
    prev_vk_down: [bool; 256],
    pub engine: crate::kwin::auto_switcher::HostKeyboardModifierEngine,
}

impl Default for HostKeyboardModifierListener {
    fn default() -> Self {
        Self::new()
    }
}

impl HostKeyboardModifierListener {
    pub fn new() -> Self {
        let mut s = Self {
            #[cfg(unix)]
            files: Vec::new(),
            #[cfg(unix)]
            hidraw_nodes: Vec::new(),
            #[cfg(unix)]
            evdev_held_keys: std::collections::HashSet::new(),
            #[cfg(unix)]
            uinput_suppressor: UinputModifierSuppressor::new(),
            #[cfg(unix)]
            kwin_suppressor: KWinInputDeviceSuppressor::new(),
            #[cfg(unix)]
            rescan_tick: 0,
            held_keys: std::collections::HashSet::new(),
            last_suppressed_duckypad_keydown_ms: 0,
            #[cfg(target_os = "windows")]
            prev_vk_down: [false; 256],
            engine: crate::kwin::auto_switcher::HostKeyboardModifierEngine::new(),
        };
        s.refresh_keyboard_nodes();
        s
    }

    #[cfg(unix)]
    fn collect_holding_keyboard_event_names(&self, trigger_key: &str) -> Vec<String> {
        let norm_trig =
            crate::kwin::auto_switcher::normalize_modifier_trigger_key(trigger_key);
        let mut active_groups = std::collections::HashSet::new();
        let mut out = Vec::new();

        for node in &self.hidraw_nodes {
            if crate::kwin::auto_switcher::is_modifier_trigger_pressed(
                &norm_trig,
                &node.held_keys,
            ) {
                if !node.device_group_key.is_empty() {
                    active_groups.insert(node.device_group_key.clone());
                }
                for ev in &node.event_sys_names {
                    if !out.contains(ev) {
                        out.push(ev.clone());
                    }
                }
            }
        }

        if !active_groups.is_empty() {
            for node in &self.hidraw_nodes {
                if active_groups.contains(&node.device_group_key) {
                    for ev in &node.event_sys_names {
                        if !out.contains(ev) {
                            out.push(ev.clone());
                        }
                    }
                }
            }
        }

        if out.is_empty()
            && crate::kwin::auto_switcher::is_modifier_trigger_pressed(
                &norm_trig,
                &self.evdev_held_keys,
            )
        {
            for (path, _) in &self.files {
                if let Some(ev) = path.file_name().and_then(|s| s.to_str()) {
                    let ev_str = ev.to_string();
                    if !out.contains(&ev_str) {
                        out.push(ev_str);
                    }
                }
            }
        }

        out
    }

    pub fn suppress_active_modifier_bleed(&mut self, trigger_key: &str) {
        #[cfg(unix)]
        {
            if self.uinput_suppressor.is_none() {
                self.uinput_suppressor = UinputModifierSuppressor::new();
            }
            if self.kwin_suppressor.is_none() {
                self.kwin_suppressor = KWinInputDeviceSuppressor::new();
            }
            let codes = trigger_key_evdev_codes(trigger_key);
            let target_events = self.collect_holding_keyboard_event_names(trigger_key);

            // 1. Universal kernel evdev release injection (COSMIC, GNOME, Mint, Sway, Hyprland, KWin):
            //    When physical key state is tracked via `/dev/hidraw*` and `/dev/input/event*` is
            //    writable (via `70-duckypad.rules` uaccess ACL or `input` group), inject `EV_KEY=0`
            //    directly into the physical keyboard's evdev node.
            let evdev_injected = if !self.hidraw_nodes.is_empty() && !target_events.is_empty() {
                try_inject_evdev_release_on_events(&target_events, codes)
            } else {
                false
            };

            // 2. KWin Wayland or Omarchy / Hyprland zero-config IPC fallback when `/dev/input/event*` is not writable:
            let mut hyprland_toggled = false;
            if !evdev_injected && !target_events.is_empty() {
                if try_set_hyprland_events_enabled(&target_events, false) {
                    hyprland_toggled = true;
                } else if let Some(ref kwin_sup) = self.kwin_suppressor {
                    kwin_sup.set_events_enabled(&target_events, false);
                }
            }

            // 3. `/dev/uinput` release pulse (wakes libinput epoll fd & clears merged state):
            if let Some(ref mut sup) = self.uinput_suppressor {
                sup.release_held_modifier_trigger(trigger_key);
            }

            // 4. X11 / Xorg fallback (Linux Mint Cinnamon/MATE/Xfce X11, GNOME Xorg):
            if !evdev_injected {
                try_inject_x11_xtest_release(codes);
            }

            if !evdev_injected && !target_events.is_empty() {
                // Allow the compositor's libinput loop 15ms to drain the synthetic KEY_RELEASED
                // event woken by uinput before re-enabling the physical keyboard node.
                std::thread::sleep(std::time::Duration::from_millis(15));
                if hyprland_toggled {
                    let _ = try_set_hyprland_events_enabled(&target_events, true);
                } else if let Some(ref kwin_sup) = self.kwin_suppressor {
                    kwin_sup.set_events_enabled(&target_events, true);
                }
            }
        }
        #[cfg(target_os = "windows")]
        {
            win32_input::suppress_held_modifier_trigger(trigger_key);
        }
    }

    pub fn refresh_keyboard_nodes(&mut self) {
        #[cfg(unix)]
        {
            use std::io::Read;
            use std::os::unix::fs::OpenOptionsExt;

            let mut drain_buf = [0u8; 240];
            if let Ok(entries) = fs::read_dir("/sys/class/input") {
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

                    let sys_dev_dir = entry.path().join("device");
                    let uevent_upper = fs::read_to_string(sys_dev_dir.join("uevent"))
                        .unwrap_or_default()
                        .to_ascii_uppercase();
                    let dev_name_lower = fs::read_to_string(sys_dev_dir.join("name"))
                        .unwrap_or_default()
                        .to_ascii_lowercase();

                    // Explicitly exclude duckyPad USB HID nodes and our own uinput guard so duckyPad macros never loop back
                    if uevent_upper.contains("483/D11D")
                        || uevent_upper.contains("483/D11C")
                        || uevent_upper.contains("0483:D11D")
                        || uevent_upper.contains("0483:D11C")
                        || dev_name_lower.contains("duckypad")
                        || dev_name_lower.contains("duckydeck")
                    {
                        continue;
                    }

                    // Only open devices that report EV_KEY capability
                    let key_caps = fs::read_to_string(sys_dev_dir.join("capabilities/key"))
                        .unwrap_or_default();
                    let has_nonzero_key_caps = key_caps
                        .chars()
                        .any(|c| c.is_ascii_hexdigit() && c != '0');
                    if !has_nonzero_key_caps {
                        continue;
                    }

                    if let Ok(mut file) = fs::OpenOptions::new()
                        .read(true)
                        .custom_flags(0o4000) // O_NONBLOCK
                        .open(&dev_path)
                    {
                        while file.read(&mut drain_buf).is_ok() {}
                        self.files.push((dev_path, file));
                    }
                }
            }

            // Also discover readable USB HID keyboard `/dev/hidraw*` nodes so unprivileged users
            // without `input` group membership can use Main-Keyboard Modifier Layer Shifting zero-config.
            if let Ok(entries) = fs::read_dir("/sys/class/hidraw") {
                for entry in entries.flatten() {
                    let fname = entry.file_name();
                    let Some(name_str) = fname.to_str() else {
                        continue;
                    };
                    if !name_str.starts_with("hidraw") {
                        continue;
                    }
                    let dev_path = Path::new("/dev").join(name_str);
                    if self.hidraw_nodes.iter().any(|n| n.path == dev_path) {
                        continue;
                    }

                    let sys_dev_dir = entry.path().join("device");
                    let uevent_upper = fs::read_to_string(sys_dev_dir.join("uevent"))
                        .unwrap_or_default()
                        .to_ascii_uppercase();
                    if uevent_upper.contains("0483:0000D11D")
                        || uevent_upper.contains("0483:0000D11C")
                        || uevent_upper.contains("0483:D11D")
                        || uevent_upper.contains("0483:D11C")
                        || uevent_upper.contains("DUCKYPAD")
                        || uevent_upper.contains("1050:")
                        || uevent_upper.contains("YUBIKEY")
                    {
                        continue;
                    }

                    let Ok(desc) = fs::read(sys_dev_dir.join("report_descriptor")) else {
                        continue;
                    };
                    let Some(format) = detect_hidraw_keyboard_format(&desc) else {
                        continue;
                    };

                    if let Ok(mut file) = fs::OpenOptions::new()
                        .read(true)
                        .custom_flags(0o4000) // O_NONBLOCK
                        .open(&dev_path)
                    {
                        while file.read(&mut drain_buf).is_ok() {}
                        let (event_sys_names, device_group_key) =
                            resolve_hidraw_keyboard_events_and_group(&sys_dev_dir);
                        if !event_sys_names.is_empty() {
                            if let Some(ref kwin_sup) = self.kwin_suppressor {
                                kwin_sup.set_events_enabled(&event_sys_names, true);
                            }
                        }
                        self.hidraw_nodes.push(HidrawKeyboardNode {
                            path: dev_path,
                            file,
                            format,
                            held_keys: std::collections::HashSet::new(),
                            event_sys_names,
                            device_group_key,
                        });
                    }
                }
            }
        }
    }

    pub fn poll_modifier_layer(
        &mut self,
        modifier_switch_enabled: bool,
        paused_for_editing: bool,
        rules: &[crate::kwin::auto_switcher::ModifierLayerRule],
        active_window_class: &str,
        active_window_title: &str,
    ) -> Option<crate::kwin::auto_switcher::ModifierLayerTransition> {
        if !modifier_switch_enabled || rules.is_empty() || !rules.iter().any(|r| r.enabled) {
            self.held_keys.clear();
            #[cfg(unix)]
            {
                self.evdev_held_keys.clear();
                for node in &mut self.hidraw_nodes {
                    node.held_keys.clear();
                }
            }
            #[cfg(target_os = "windows")]
            {
                win32_input::clear_suppressed_modifiers();
            }
            return self.engine.force_deactivate();
        }

        let mut newly_pressed_keys: Vec<u16> = Vec::new();
        let duckypad_keys_held = crate::hardware::storage::EVDEV_KEYS_HELD_COUNT
            .load(std::sync::atomic::Ordering::SeqCst);
        let last_duckypad_keydown_ms = crate::hardware::storage::LAST_EVDEV_KEYDOWN_MS
            .load(std::sync::atomic::Ordering::SeqCst);
        let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
        #[cfg_attr(target_os = "windows", allow(unused_mut))]
        let mut intermediate_transition: Option<
            crate::kwin::auto_switcher::ModifierLayerTransition,
        > = None;

        #[cfg(unix)]
        {
            use std::io::{self, Read};

            self.rescan_tick = self.rescan_tick.wrapping_add(1);
            if (self.files.is_empty() && self.hidraw_nodes.is_empty())
                || self.rescan_tick.is_multiple_of(80)
            {
                self.refresh_keyboard_nodes();
            }

            let mut drop_indices = Vec::new();
            let mut raw = [0u8; 24];

            for (idx, (_path, file)) in self.files.iter_mut().enumerate() {
                loop {
                    match file.read_exact(&mut raw) {
                        Ok(()) => {
                            let ev_type = u16::from_ne_bytes([raw[16], raw[17]]);
                            let ev_code = u16::from_ne_bytes([raw[18], raw[19]]);
                            let ev_value =
                                i32::from_ne_bytes([raw[20], raw[21], raw[22], raw[23]]);
                            // EV_KEY == 1; ignore mouse buttons (0x110..0x120 = 272..288)
                            if ev_type == 1 && (ev_code <= 248 || ev_code == 464) {
                                if ev_value == 1 {
                                    self.evdev_held_keys.insert(ev_code);
                                    newly_pressed_keys.push(ev_code);
                                } else if ev_value == 0 {
                                    self.evdev_held_keys.remove(&ev_code);
                                }
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

            let mut drop_hidraw = Vec::new();
            let mut hbuf = [0u8; 64];
            for (idx, node) in self.hidraw_nodes.iter_mut().enumerate() {
                loop {
                    match node.file.read(&mut hbuf) {
                        Ok(n) if n > 0 => {
                            if let Some(new_set) =
                                parse_hidraw_keyboard_report(node.format, &hbuf[..n])
                            {
                                // If a key was pressed and then released within the same 50ms poll window,
                                // evaluate the press state first so sub-50ms taps are never missed.
                                let released_newly_pressed = node.held_keys.iter().any(|k| {
                                    !new_set.contains(k) && newly_pressed_keys.contains(k)
                                });
                                if released_newly_pressed {
                                    let mut snap = self.evdev_held_keys.clone();
                                    snap.extend(node.held_keys.iter().copied());
                                    if let Some(tr) = self.engine.evaluate_tick_for_window(
                                        modifier_switch_enabled,
                                        paused_for_editing,
                                        rules,
                                        active_window_class,
                                        active_window_title,
                                        &snap,
                                        &newly_pressed_keys,
                                        duckypad_keys_held,
                                        last_duckypad_keydown_ms,
                                        now_ms,
                                    ) {
                                        intermediate_transition = Some(tr);
                                    }
                                    newly_pressed_keys.clear();
                                }

                                for &k in &new_set {
                                    if !node.held_keys.contains(&k) {
                                        newly_pressed_keys.push(k);
                                    }
                                }
                                node.held_keys = new_set;
                            }
                        }
                        Ok(_) => break,
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(_) => {
                            drop_hidraw.push(idx);
                            break;
                        }
                    }
                }
            }
            for idx in drop_hidraw.into_iter().rev() {
                self.hidraw_nodes.remove(idx);
            }

            self.held_keys.clear();
            self.held_keys.extend(self.evdev_held_keys.iter().copied());
            for node in &self.hidraw_nodes {
                self.held_keys.extend(node.held_keys.iter().copied());
            }
        }

        #[cfg(target_os = "windows")]
        {
            win32_input::ensure_ll_hook_started();
            self.held_keys.clear();
            let vk_map: &[(i32, u16)] = &[
                (0x5D, 127), // VK_APPS -> MENU
                (0xA3, 97),  // VK_RCONTROL -> RCTRL
                (0xA2, 29),  // VK_LCONTROL -> LCTRL
                (0xA5, 100), // VK_RMENU -> RALT
                (0xA4, 56),  // VK_LMENU -> LALT
                (0xA1, 54),  // VK_RSHIFT -> RSHIFT
                (0xA0, 42),  // VK_LSHIFT -> LSHIFT
                (0x5B, 125), // VK_LWIN -> SUPER
                (0x5C, 126), // VK_RWIN -> SUPER
                (0x14, 58),  // VK_CAPITAL -> CAPSLOCK
                (0x91, 70),  // VK_SCROLL -> SCROLLLOCK
                (0x13, 119), // VK_PAUSE -> PAUSE
                (0x7C, 183), // VK_F13
                (0x7D, 184), // VK_F14
                (0x7E, 185), // VK_F15
                (0x7F, 186), // VK_F16
                (0x80, 187), // VK_F17
                (0x81, 188), // VK_F18
                (0x82, 189), // VK_F19
                (0x83, 190), // VK_F20
                (0x84, 191), // VK_F21
                (0x85, 192), // VK_F22
                (0x86, 193), // VK_F23
                (0x87, 194), // VK_F24
            ];
            for &(vk, ev_code) in vk_map {
                let vk_idx = vk as usize;
                let is_down = if win32_input::is_vk_suppressed(vk_idx) {
                    // While suppressed, GetAsyncKeyState is cleared by our synthetic KEYUP;
                    // rely on WH_KEYBOARD_LL's un-injected hardware state so the hold layer
                    // stays active until the user physically releases the key.
                    win32_input::is_physical_vk_down(vk_idx)
                } else {
                    // SAFETY: GetAsyncKeyState is a thread-safe Win32 query.
                    let async_down =
                        unsafe { (win32_input::GetAsyncKeyState(vk) as u16 & 0x8000) != 0 };
                    let phys_down = win32_input::is_physical_vk_down(vk_idx) || async_down;
                    if async_down {
                        win32_input::set_physical_vk_down(vk_idx, true);
                    }
                    phys_down
                };
                let was_down = self.prev_vk_down[vk_idx];
                self.prev_vk_down[vk_idx] = is_down;
                if is_down {
                    self.held_keys.insert(ev_code);
                    if !was_down {
                        newly_pressed_keys.push(ev_code);
                    }
                }
            }
            // Check standard typing keys (0x08..=0x7B, excluding modifiers) for chord guard
            for vk in 0x08..=0x7Bi32 {
                if matches!(
                    vk,
                    0x10 | 0x11 | 0x12 | 0x13 | 0x14 | 0x5B | 0x5C | 0x5D
                ) {
                    continue;
                }
                // SAFETY: GetAsyncKeyState is a thread-safe Win32 query.
                let is_down =
                    unsafe { (win32_input::GetAsyncKeyState(vk) as u16 & 0x8000) != 0 };
                let was_down = self.prev_vk_down[vk as usize];
                self.prev_vk_down[vk as usize] = is_down;
                if is_down {
                    self.held_keys.insert(30); // non-modifier typing key sentinel
                    if !was_down {
                        newly_pressed_keys.push(30);
                    }
                }
            }
        }

        let out_transition = self
            .engine
            .evaluate_tick_for_window(
                modifier_switch_enabled,
                paused_for_editing,
                rules,
                active_window_class,
                active_window_title,
                &self.held_keys,
                &newly_pressed_keys,
                duckypad_keys_held,
                last_duckypad_keydown_ms,
                now_ms,
            )
            .or(intermediate_transition);

        if let Some(crate::kwin::auto_switcher::ModifierLayerTransition::Activated {
            ref trigger_key,
            ref mode,
            ..
        }) = out_transition
        {
            if mode == "hold" || mode == "double_tap_hold" {
                self.suppress_active_modifier_bleed(trigger_key);
                self.last_suppressed_duckypad_keydown_ms = last_duckypad_keydown_ms;
            }
        } else if matches!(
            out_transition,
            Some(crate::kwin::auto_switcher::ModifierLayerTransition::Deactivated)
        ) {
            #[cfg(target_os = "windows")]
            {
                win32_input::clear_suppressed_modifiers();
            }
        } else if let Some((_prof, trig, mode)) = self.engine.active_override() {
            if (mode == "hold" || mode == "double_tap_hold")
                && last_duckypad_keydown_ms > self.last_suppressed_duckypad_keydown_ms
            {
                #[cfg(unix)]
                {
                    let codes = trigger_key_evdev_codes(&trig);
                    let target_events = self.collect_holding_keyboard_event_names(&trig);
                    if !self.hidraw_nodes.is_empty() && !target_events.is_empty() {
                        let _ = try_inject_evdev_release_on_events(&target_events, codes);
                    }
                }
                #[cfg(target_os = "windows")]
                {
                    self.suppress_active_modifier_bleed(&trig);
                }
                self.last_suppressed_duckypad_keydown_ms = last_duckypad_keydown_ms;
            }
        }

        out_transition
    }
}

#[cfg(test)]
mod hidraw_modifier_tests {
    use super::*;

    #[test]
    fn test_detect_and_parse_keychron_boot_and_nkro_reports() {
        // Keychron C3 Pro /dev/hidraw5 Boot Keyboard report descriptor
        let boot_desc: [u8; 28] = [
            0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x05, 0x07, 0x19, 0xE0, 0x29, 0xE7, 0x15, 0x00,
            0x25, 0x01, 0x95, 0x08, 0x75, 0x01, 0x81, 0x02, 0x95, 0x01, 0x75, 0x08, 0x81, 0x01,
        ];
        let fmt = detect_hidraw_keyboard_format(&boot_desc);
        assert_eq!(fmt, Some(HidrawKeyboardFormat::BootNoReportId));

        // Right Ctrl (0x10) held alone on Boot interface
        let rctrl_pkt = [0x10u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let keys = parse_hidraw_keyboard_report(HidrawKeyboardFormat::BootNoReportId, &rctrl_pkt)
            .expect("boot report");
        assert!(keys.contains(&97));
        assert_eq!(keys.len(), 1);

        // Right Ctrl (0x10) + 'C' (0x06) chord on Boot interface
        let rctrl_c_pkt = [0x10u8, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00];
        let chord_keys =
            parse_hidraw_keyboard_report(HidrawKeyboardFormat::BootNoReportId, &rctrl_c_pkt)
                .expect("boot chord report");
        assert!(chord_keys.contains(&97));
        assert!(chord_keys.contains(&1006));

        // Keychron C3 Pro /dev/hidraw7 composite NKRO report descriptor snippet (Report ID 0x06)
        let nkro_desc: [u8; 36] = [
            0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x85, 0x06, 0x05, 0x07, 0x19, 0xE0, 0x29, 0xE7,
            0x15, 0x00, 0x25, 0x01, 0x95, 0x08, 0x75, 0x01, 0x81, 0x02, 0x05, 0x07, 0x19, 0x00,
            0x29, 0xEF, 0x15, 0x00, 0x25, 0x01, 0x95, 0xF0,
        ];
        // Append 75 01 81 02
        let mut nkro_full = nkro_desc.to_vec();
        nkro_full.extend_from_slice(&[0x75, 0x01, 0x81, 0x02, 0xC0]);
        assert_eq!(
            detect_hidraw_keyboard_format(&nkro_full),
            Some(HidrawKeyboardFormat::NkroBitmapWithReportId(6))
        );

        let mut nkro_pkt = [0u8; 32];
        nkro_pkt[0] = 0x06; // Report ID 6
        nkro_pkt[1] = 0x10; // Right Ctrl
        let nkro_keys = parse_hidraw_keyboard_report(
            HidrawKeyboardFormat::NkroBitmapWithReportId(6),
            &nkro_pkt,
        )
        .expect("nkro report");
        assert!(nkro_keys.contains(&97));
        assert_eq!(nkro_keys.len(), 1);
    }

    #[test]
    fn test_held_modifier_bleed_suppression_on_activation() {
        #[cfg(unix)]
        {
            // Keychron C3 Pro keyboard capabilities vs mouse/consumer control subdevices
            assert!(is_standard_keyboard_key_caps(
                "1000000000007 ff9f207ac14057ff febeffdfffefffff fffffffffffffffe"
            ));
            assert!(!is_standard_keyboard_key_caps("ff0000 0 0 0 0"));
            assert!(!is_standard_keyboard_key_caps(
                "733fff 0 0 483ffff17aff32d bfd4444600000000 1 130ff38b17c000 677bfad9415fed 19ed68000004400 10000002"
            ));
            assert_eq!(
                format_hyprland_device_name("Keychron Keychron C3 Pro\n"),
                "keychron-keychron-c3-pro"
            );
        }
        let mut listener = HostKeyboardModifierListener::new();
        // Verify suppressing held modifiers (Right Ctrl, Left Ctrl, Alt, Shift, Super, Chords)
        // executes cleanly without panicking or looping back into held_keys.
        for key in [
            "RCTRL",
            "LCTRL",
            "RALT",
            "LALT",
            "RSHIFT",
            "LSHIFT",
            "SUPER",
            "CTRL_ALT",
            "CTRL_SUPER",
            "MENU",
        ] {
            listener.suppress_active_modifier_bleed(key);
        }
        assert!(listener.held_keys.is_empty());
    }
}
