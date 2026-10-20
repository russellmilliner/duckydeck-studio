use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppShortcut {
    pub action: String,
    pub shortcut: String,
    pub duckyscript: String,
    pub category: String,
    pub source: String,
    pub label_l1: String,
    pub label_l2: String,
}

/// Normalizes an application identifier, window class, or window title to a human-friendly name
pub fn normalize_app_name(raw: &str) -> String {
    let lower = raw.trim().to_lowercase();
    if lower.contains("minecraft")
        || lower.contains("prismlauncher")
        || lower.contains("lunarclient")
        || lower.contains("badlion")
        || lower.contains("modrinth")
        || lower.contains("atlauncher")
        || lower.contains("gdlauncher")
        || lower.contains("mojang")
    {
        return "Minecraft".to_string();
    }
    if lower.contains("code") || lower.contains("vscode") || lower.contains("vscodium") {
        return "VS Code".to_string();
    }
    if lower.contains("chrome") || lower.contains("chromium") {
        return "Google Chrome".to_string();
    }
    if lower.contains("firefox") || lower.contains("librewolf") || lower.contains("zen-browser") {
        return "Firefox".to_string();
    }
    if lower.contains("brave") {
        return "Brave Browser".to_string();
    }
    if lower.contains("dolphin") {
        return "Dolphin".to_string();
    }
    if lower.contains("blender") {
        return "Blender".to_string();
    }
    if lower.contains("gimp") {
        return "GIMP".to_string();
    }
    if lower.contains("krita") {
        return "Krita".to_string();
    }
    if lower.contains("inkscape") {
        return "Inkscape".to_string();
    }
    if lower.contains("vlc") {
        return "VLC".to_string();
    }
    if lower.contains("spotify") {
        return "Spotify".to_string();
    }
    if lower.contains("obs") {
        return "OBS Studio".to_string();
    }
    if lower.contains("discord") || lower.contains("vesktop") {
        return "Discord".to_string();
    }
    if lower.contains("slack") {
        return "Slack".to_string();
    }
    if lower.contains("terminal")
        || lower.contains("konsole")
        || lower.contains("kitty")
        || lower.contains("alacritty")
        || lower.contains("wezterm")
    {
        return "Terminal".to_string();
    }
    if lower.contains("kate") || lower.contains("kwrite") {
        return "Kate".to_string();
    }
    if lower.contains("libreoffice") || lower.contains("soffice") {
        return "LibreOffice".to_string();
    }
    if lower.contains("freecad") {
        return "FreeCAD".to_string();
    }
    if lower.contains("kicad") {
        return "KiCad".to_string();
    }
    if lower.contains("godot") {
        return "Godot".to_string();
    }
    if lower.contains("audacity") {
        return "Audacity".to_string();
    }
    if lower.contains("kdenlive") {
        return "Kdenlive".to_string();
    }
    if lower.contains("obsidian") {
        return "Obsidian".to_string();
    }
    if lower.contains("steam") {
        return "Steam".to_string();
    }

    // Strip reverse-DNS prefixes (e.g. "org.kde.dolphin" -> "Dolphin")
    let cleaned = raw.trim();
    if let Some(pos) = cleaned.rfind('.') {
        let name = &cleaned[pos + 1..];
        if !name.is_empty() && !name.chars().all(|c| c.is_ascii_digit()) {
            let mut c = name.chars();
            return match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            };
        }
    }

    raw.trim().to_string()
}

/// Generates a concise 2-line label for a 5-char duckyPad OLED key slot
pub fn make_smart_labels(action: &str, shortcut: &str) -> (String, String) {
    let act_lower = action.to_lowercase();
    let l1 = if act_lower.contains("save all") {
        "SvAll".to_string()
    } else if act_lower.contains("save hotbar") {
        "SvBar".to_string()
    } else if act_lower.contains("load hotbar") {
        "LdBar".to_string()
    } else if act_lower.contains("save") {
        "Save".to_string()
    } else if act_lower.contains("chunk border") || act_lower.contains("chunks") && act_lower.contains("border") {
        "Chunk".to_string()
    } else if act_lower.contains("reload chunk") {
        "RldCk".to_string()
    } else if act_lower.contains("hitbox") {
        "Hitbx".to_string()
    } else if act_lower.contains("coordinate") || act_lower.contains("copy /tp") {
        "Coord".to_string()
    } else if act_lower.contains("nbt") || act_lower.contains("block/entity") || act_lower.contains("entity data") {
        "NBT".to_string()
    } else if act_lower.contains("gamemode") {
        "GMode".to_string()
    } else if act_lower.contains("spectator") {
        "Spect".to_string()
    } else if act_lower.contains("tooltip") || act_lower.contains("durability") {
        "Tips".to_string()
    } else if act_lower.contains("resource") || act_lower.contains("texture") {
        "TexRl".to_string()
    } else if act_lower.contains("render dist") {
        "RDist".to_string()
    } else if act_lower.contains("clear chat") {
        "ClrCh".to_string()
    } else if act_lower.contains("debug help") || act_lower.contains("f3 help") {
        "F3Hlp".to_string()
    } else if act_lower.contains("debug") {
        "Debug".to_string()
    } else if act_lower.contains("inventory") {
        "Inv".to_string()
    } else if act_lower.contains("drop stack") || act_lower.contains("entire stack") {
        "Drop*".to_string()
    } else if act_lower.contains("drop") {
        "Drop".to_string()
    } else if act_lower.contains("offhand") || act_lower.contains("swap hand") {
        "Offhd".to_string()
    } else if act_lower.contains("perspective") || act_lower.contains("camera") || act_lower.contains("3rd person") {
        "Cam".to_string()
    } else if act_lower.contains("screenshot") {
        "Shot".to_string()
    } else if act_lower.contains("hide hud") || act_lower.contains("toggle hud") {
        "HUD".to_string()
    } else if act_lower.contains("advancement") {
        "Advnc".to_string()
    } else if act_lower.contains("player list") {
        "Plist".to_string()
    } else if act_lower.contains("command") && !act_lower.contains("palette") {
        "Cmd /".to_string()
    } else if act_lower.contains("open chat") || act_lower == "chat" {
        "Chat".to_string()
    } else if act_lower.contains("pick block") {
        "Pick".to_string()
    } else if act_lower.contains("sprint") {
        "Sprnt".to_string()
    } else if act_lower.contains("sneak") || act_lower.contains("crouch") {
        "Sneak".to_string()
    } else if act_lower.contains("jump") || act_lower.contains("fly") {
        "Jump".to_string()
    } else if act_lower.contains("terminal") {
        "Term".to_string()
    } else if act_lower.contains("palette") {
        "Cmd".to_string()
    } else if act_lower.contains("split") {
        "Split".to_string()
    } else if act_lower.contains("format") {
        "Fmt".to_string()
    } else if act_lower.contains("comment") {
        "//".to_string()
    } else if act_lower.contains("duplicate") {
        "Dup".to_string()
    } else if act_lower.contains("find in files") {
        "Find*".to_string()
    } else if act_lower.contains("find") || act_lower.contains("search") {
        "Find".to_string()
    } else if act_lower.contains("replace") {
        "Rplc".to_string()
    } else if act_lower.contains("new tab") {
        "Tab+".to_string()
    } else if act_lower.contains("close tab") {
        "Tab-".to_string()
    } else if act_lower.contains("reopen") {
        "ReOpn".to_string()
    } else if act_lower.contains("next tab") {
        "Tab>".to_string()
    } else if act_lower.contains("prev tab") || act_lower.contains("previous tab") {
        "<Tab".to_string()
    } else if act_lower.contains("reload") || act_lower.contains("refresh") {
        "Rld".to_string()
    } else if act_lower.contains("bookmark") {
        "Bkmk".to_string()
    } else if act_lower.contains("inspect") {
        "Dev".to_string()
    } else if act_lower.contains("play") || act_lower.contains("pause") {
        "Play".to_string()
    } else if act_lower.contains("next") {
        "Next".to_string()
    } else if act_lower.contains("prev") {
        "Prev".to_string()
    } else if act_lower.contains("deafen") {
        "Deaf".to_string()
    } else if act_lower.contains("mute") {
        "Mute".to_string()
    } else if act_lower.contains("volume up") {
        "Vol+".to_string()
    } else if act_lower.contains("volume down") {
        "Vol-".to_string()
    } else if act_lower.contains("fullscreen") {
        "Full".to_string()
    } else if act_lower.contains("record") {
        "Rec".to_string()
    } else if act_lower.contains("stream") {
        "Strm".to_string()
    } else if act_lower.contains("render") {
        "Rndr".to_string()
    } else if act_lower.contains("undo") {
        "Undo".to_string()
    } else if act_lower.contains("redo") {
        "Redo".to_string()
    } else if act_lower.contains("cut") {
        "Cut".to_string()
    } else if act_lower.contains("copy") {
        "Copy".to_string()
    } else if act_lower.contains("paste") {
        "Pste".to_string()
    } else if act_lower.contains("zoom in") {
        "Zm+".to_string()
    } else if act_lower.contains("zoom out") {
        "Zm-".to_string()
    } else if act_lower.contains("zoom") {
        "Zoom".to_string()
    } else {
        // Skip common filler verbs so the 5-char label is meaningful
        let meaningful = action
            .split_whitespace()
            .find(|w| {
                let wl = w.to_lowercase();
                !matches!(
                    wl.as_str(),
                    "open" | "toggle" | "show" | "hide" | "start" | "stop" | "the" | "a" | "an" | "to"
                )
            })
            .unwrap_or_else(|| action.split_whitespace().next().unwrap_or(action));
        meaningful
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '-')
            .take(5)
            .collect()
    };

    // Concise shortcut notation for Line 2 (max 5 chars)
    let l2 = shortcut
        .replace("Control + ", "^")
        .replace("Control+", "^")
        .replace("Ctrl + ", "^")
        .replace("Ctrl+", "^")
        .replace("Shift + ", "+")
        .replace("Shift+", "+")
        .replace("Alt + ", "@")
        .replace("Alt+", "@")
        .replace("Meta + ", "#")
        .replace("Meta+", "#")
        .replace("Super + ", "#")
        .replace("Super+", "#")
        .replace("Windows+", "#")
        .replace("Numpad ", "NP")
        .replace("Middle-click", "MMB")
        .replace("Left-click", "LMB")
        .replace("Right-click", "RMB")
        .replace("Escape", "Esc")
        .replace("Space", "Spc")
        .replace(" + ", "+")
        .chars()
        .take(5)
        .collect();

    (l1, l2)
}

/// Maps a single key token into a valid DSVM v2 key identifier
fn map_single_token_to_dsvm(raw_part: &str) -> Option<String> {
    let part = raw_part.trim();
    if part.is_empty() {
        return None;
    }
    let lower = part.to_ascii_lowercase();
    let mapped = match lower.as_str() {
        "meta" | "super" | "win" | "windows" | "gui" | "command" | "cmd" => "WINDOWS".to_string(),
        "ctrl" | "control" => "CONTROL".to_string(),
        "rctrl" | "rcontrol" => "RCONTROL".to_string(),
        "shift" => "SHIFT".to_string(),
        "rshift" => "RSHIFT".to_string(),
        "alt" | "option" => "ALT".to_string(),
        "ralt" | "altgr" => "RALT".to_string(),
        "return" | "enter" => "ENTER".to_string(),
        "esc" | "escape" => "ESCAPE".to_string(),
        "del" | "delete" => "DELETE".to_string(),
        "ins" | "insert" => "INSERT".to_string(),
        "backspace" | "bs" | "bksp" => "BACKSPACE".to_string(),
        "pgup" | "pageup" | "prior" | "page up" => "PAGEUP".to_string(),
        "pgdown" | "pgdn" | "pagedown" | "next" | "page down" => "PAGEDOWN".to_string(),
        "space" | "spacebar" => "SPACE".to_string(),
        "tab" | "backtab" => "TAB".to_string(),
        "up" | "uparrow" | "↑" => "UP".to_string(),
        "down" | "downarrow" | "↓" => "DOWN".to_string(),
        "left" | "leftarrow" | "←" => "LEFT".to_string(),
        "right" | "rightarrow" | "→" => "RIGHT".to_string(),
        "home" => "HOME".to_string(),
        "end" => "END".to_string(),
        "print" | "sysreq" | "printscreen" | "prtsc" => "PRINTSCREEN".to_string(),
        "capslock" | "caps" => "CAPSLOCK".to_string(),
        "numlock" => "NUMLOCK".to_string(),
        "scrolllock" => "SCROLLLOCK".to_string(),
        "pause" | "break" => "PAUSE".to_string(),
        "menu" | "apps" | "app" => "MENU".to_string(),
        "lmouse" | "left-click" | "left click" | "lmb" => "LMOUSE".to_string(),
        "rmouse" | "right-click" | "right click" | "rmb" => "RMOUSE".to_string(),
        "mmouse" | "middle-click" | "middle click" | "mmb" => "MMOUSE".to_string(),
        "numpad 0" | "num 0" | "kp 0" | "kp_0" => "KP_0".to_string(),
        "numpad 1" | "num 1" | "kp 1" | "kp_1" => "KP_1".to_string(),
        "numpad 2" | "num 2" | "kp 2" | "kp_2" => "KP_2".to_string(),
        "numpad 3" | "num 3" | "kp 3" | "kp_3" => "KP_3".to_string(),
        "numpad 4" | "num 4" | "kp 4" | "kp_4" => "KP_4".to_string(),
        "numpad 5" | "num 5" | "kp 5" | "kp_5" => "KP_5".to_string(),
        "numpad 6" | "num 6" | "kp 6" | "kp_6" => "KP_6".to_string(),
        "numpad 7" | "num 7" | "kp 7" | "kp_7" => "KP_7".to_string(),
        "numpad 8" | "num 8" | "kp 8" | "kp_8" => "KP_8".to_string(),
        "numpad 9" | "num 9" | "kp 9" | "kp_9" => "KP_9".to_string(),
        "numpad +" | "kp +" | "kp_plus" => "KP_PLUS".to_string(),
        "numpad -" | "kp -" | "kp_minus" => "KP_MINUS".to_string(),
        "numpad *" | "kp *" | "kp_asterisk" => "KP_ASTERISK".to_string(),
        "numpad /" | "kp /" | "kp_slash" => "KP_SLASH".to_string(),
        "numpad ." | "kp ." | "kp_dot" => "KP_DOT".to_string(),
        "numpad enter" | "kp enter" | "kp_enter" => "KP_ENTER".to_string(),
        "plus" => "+".to_string(),
        "minus" => "-".to_string(),
        "comma" => ",".to_string(),
        "period" => ".".to_string(),
        "slash" => "/".to_string(),
        "backslash" => "\\".to_string(),
        other => {
            if let Some(num) = other
                .strip_prefix('f')
                .and_then(|n| n.parse::<u8>().ok())
                .filter(|&n| (1..=24).contains(&n))
            {
                format!("F{num}")
            } else if part.len() == 1 && part.chars().all(|c| c.is_ascii_graphic()) {
                part.to_ascii_lowercase()
            } else if crate::vm::compiler::hid_keyname_lookup(&part.to_ascii_uppercase()).is_some() {
                part.to_ascii_uppercase()
            } else {
                return None;
            }
        }
    };
    Some(mapped)
}

/// Converts a shortcut string (e.g. "Ctrl+Shift+P", "F3+G", "E") to valid DuckyScript 3
pub fn shortcut_to_duckyscript(shortcut: &str, action_hint: &str) -> String {
    let primary = shortcut
        .split(['\t', ';', ','])
        .map(str::trim)
        .find(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"))
        .unwrap_or(shortcut.trim());

    // Split by '+' while preserving '+' if the shortcut ends with '+' or '++' (e.g. "Ctrl++")
    let mut raw_tokens: Vec<String> = Vec::new();
    let mut rest = primary;
    while !rest.is_empty() {
        if let Some(pos) = rest.find('+') {
            let before = rest[..pos].trim();
            if !before.is_empty() {
                raw_tokens.push(before.to_string());
                rest = rest[pos + 1..].trim();
            } else {
                // Token itself is '+'
                raw_tokens.push("+".to_string());
                rest = rest[pos + 1..].trim();
            }
        } else {
            // Check for chorded sequences like "Ctrl+K S" -> split by space if not "Numpad X"
            let trimmed = rest.trim();
            if !trimmed.to_ascii_lowercase().starts_with("numpad")
                && !trimmed.to_ascii_lowercase().starts_with("page ")
                && !trimmed.to_ascii_lowercase().starts_with("left ")
                && !trimmed.to_ascii_lowercase().starts_with("right ")
                && !trimmed.to_ascii_lowercase().starts_with("middle ")
                && trimmed.contains(' ')
            {
                for sub in trimmed.split_whitespace() {
                    raw_tokens.push(sub.to_string());
                }
            } else {
                raw_tokens.push(trimmed.to_string());
            }
            break;
        }
    }

    let mapped_tokens: Vec<String> = raw_tokens
        .iter()
        .filter_map(|t| map_single_token_to_dsvm(t))
        .collect();

    let body = if mapped_tokens.is_empty() {
        "DELAY 50".to_string()
    } else if mapped_tokens.len() == 1 {
        let tok = &mapped_tokens[0];
        if crate::vm::compiler::hid_keyname_lookup(tok).is_some() {
            tok.clone()
        } else {
            format!("KEYDOWN {tok}\nDELAY 25\nKEYUP {tok}")
        }
    } else {
        let first = &mapped_tokens[0];
        if crate::vm::compiler::hid_keyname_lookup(first).is_some()
            && mapped_tokens.len() <= 6
            && mapped_tokens
                .iter()
                .all(|k| crate::vm::compiler::hid_keyname_lookup(k).is_some() || k.len() == 1)
        {
            mapped_tokens.join(" ")
        } else {
            let mut lines = Vec::new();
            for k in &mapped_tokens {
                lines.push(format!("KEYDOWN {k}"));
            }
            lines.push("DELAY 25".to_string());
            for k in mapped_tokens.iter().rev() {
                lines.push(format!("KEYUP {k}"));
            }
            lines.join("\n")
        }
    };

    let display_title: String = action_hint
        .chars()
        .filter(|c| c.is_ascii_graphic() || *c == ' ')
        .take(18)
        .collect();
    let safe_title = if display_title.trim().is_empty() {
        "Shortcut".to_string()
    } else {
        display_title.trim().to_string()
    };

    format!(
        "// Macro for: {safe_title}\nDEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT {safe_title}\nOLED_UPDATE\n{body}\nDELAY 150\nOLED_RESTORE\n"
    )
}

/// Built-in curated database of shortcuts for top Linux & gaming applications
pub fn get_curated_shortcuts(app: &str) -> Vec<AppShortcut> {
    let lower = app.to_lowercase();
    let mut list = Vec::new();

    let add = |list: &mut Vec<AppShortcut>, act: &str, sc: &str, cat: &str| {
        let (l1, l2) = make_smart_labels(act, sc);
        let ds = shortcut_to_duckyscript(sc, act);
        list.push(AppShortcut {
            action: act.to_string(),
            shortcut: sc.to_string(),
            duckyscript: ds,
            category: cat.to_string(),
            source: "App Catalog".to_string(),
            label_l1: l1,
            label_l2: l2,
        });
    };

    if lower.contains("minecraft")
        || lower.contains("prism")
        || lower.contains("lunar")
        || lower.contains("badlion")
        || lower.contains("modrinth")
        || lower.contains("mojang")
    {
        // Gameplay, Inventory & Camera
        add(&mut list, "Open Inventory", "E", "Inventory");
        add(&mut list, "Drop Single Item", "Q", "Inventory");
        add(&mut list, "Drop Entire Stack", "Ctrl+Q", "Inventory");
        add(&mut list, "Swap Offhand Item", "F", "Inventory");
        add(&mut list, "Pick Block (Middle Click)", "Middle-click", "Gameplay");
        add(&mut list, "Toggle Perspective (1st/3rd)", "F5", "Camera");
        add(&mut list, "Take Screenshot", "F2", "Camera");
        add(&mut list, "Hide HUD Overlay", "F1", "Camera");
        add(&mut list, "Toggle Fullscreen", "F11", "View");
        add(&mut list, "Open Chat", "T", "Multiplayer");
        add(&mut list, "Open Command (/ )", "/", "Multiplayer");
        add(&mut list, "Player List", "Tab", "Multiplayer");
        add(&mut list, "Advancements", "L", "Gameplay");
        add(&mut list, "Social Interactions", "P", "Multiplayer");
        add(&mut list, "Sprint", "Ctrl", "Movement");
        add(&mut list, "Sneak / Crouch", "Shift", "Movement");
        add(&mut list, "Jump / Fly", "Space", "Movement");
        add(&mut list, "Save Creative Hotbar 1", "C+1", "Creative");
        add(&mut list, "Load Creative Hotbar 1", "X+1", "Creative");
        // F3 Debug Combos (huge value on a macro pad!)
        add(&mut list, "Debug Overlay (Coords/FPS)", "F3", "F3 Debug");
        add(&mut list, "Show Chunk Borders", "F3+G", "F3 Debug");
        add(&mut list, "Show Entity Hitboxes", "F3+B", "F3 Debug");
        add(&mut list, "Advanced Item Tooltips", "F3+H", "F3 Debug");
        add(&mut list, "Reload All Chunks", "F3+A", "F3 Debug");
        add(&mut list, "Copy Coordinates (/tp)", "F3+C", "F3 Debug");
        add(&mut list, "Copy Block/Entity NBT", "F3+I", "F3 Debug");
        add(&mut list, "Gamemode Switcher", "F3+F4", "F3 Debug");
        add(&mut list, "Cycle Spectator Mode", "F3+N", "F3 Debug");
        add(&mut list, "Clear Chat History", "F3+D", "F3 Debug");
        add(&mut list, "Cycle Render Distance", "F3+F", "F3 Debug");
        add(&mut list, "Reload Textures & Packs", "F3+T", "F3 Debug");
        add(&mut list, "Toggle Auto-Pause on Blur", "F3+P", "F3 Debug");
        add(&mut list, "Show F3 Debug Help", "F3+Q", "F3 Debug");
        add(&mut list, "Pause Without Menu", "F3+Esc", "F3 Debug");
        add(&mut list, "Profiler Pie Chart", "Shift+F3", "F3 Debug");
        add(&mut list, "FPS Frame-Time Graph", "Alt+F3", "F3 Debug");
        // Hotbar slots
        add(&mut list, "Hotbar Slot 1", "1", "Hotbar");
        add(&mut list, "Hotbar Slot 2", "2", "Hotbar");
        add(&mut list, "Hotbar Slot 3", "3", "Hotbar");
        add(&mut list, "Hotbar Slot 4", "4", "Hotbar");
        add(&mut list, "Hotbar Slot 5", "5", "Hotbar");
        add(&mut list, "Hotbar Slot 6", "6", "Hotbar");
        add(&mut list, "Hotbar Slot 7", "7", "Hotbar");
        add(&mut list, "Hotbar Slot 8", "8", "Hotbar");
        add(&mut list, "Hotbar Slot 9", "9", "Hotbar");
    } else if lower.contains("code") || lower.contains("vs") {
        add(&mut list, "Command Palette", "Ctrl+Shift+P", "Tools");
        add(&mut list, "Quick Open File", "Ctrl+P", "Navigation");
        add(&mut list, "Toggle Terminal", "Ctrl+`", "Tools");
        add(&mut list, "Toggle Sidebar", "Ctrl+B", "View");
        add(&mut list, "Split Editor", "Ctrl+\\", "View");
        add(&mut list, "Format Document", "Ctrl+Shift+I", "Edit");
        add(&mut list, "Toggle Comment", "Ctrl+/", "Edit");
        add(&mut list, "Duplicate Line Down", "Shift+Alt+Down", "Edit");
        add(&mut list, "Find in Files", "Ctrl+Shift+F", "Navigation");
        add(&mut list, "Replace in Files", "Ctrl+Shift+H", "Edit");
        add(&mut list, "Save All", "Ctrl+K S", "File");
        add(&mut list, "Close Editor", "Ctrl+W", "File");
        add(&mut list, "Reopen Closed Tab", "Ctrl+Shift+T", "File");
        add(&mut list, "Go to Symbol", "Ctrl+Shift+O", "Navigation");
        add(&mut list, "Go to Line", "Ctrl+G", "Navigation");
        add(&mut list, "Trigger Suggestion", "Ctrl+Space", "Edit");
        add(&mut list, "Multi-Cursor Up", "Ctrl+Alt+Up", "Edit");
        add(&mut list, "Multi-Cursor Down", "Ctrl+Alt+Down", "Edit");
        add(&mut list, "Zoom In", "Ctrl+=", "View");
        add(&mut list, "Zoom Out", "Ctrl+-", "View");
    } else if lower.contains("chrome")
        || lower.contains("firefox")
        || lower.contains("brave")
        || lower.contains("browser")
    {
        add(&mut list, "New Tab", "Ctrl+T", "Navigation");
        add(&mut list, "Close Tab", "Ctrl+W", "Navigation");
        add(&mut list, "Reopen Closed Tab", "Ctrl+Shift+T", "Navigation");
        add(&mut list, "Next Tab", "Ctrl+Tab", "Navigation");
        add(&mut list, "Previous Tab", "Ctrl+Shift+Tab", "Navigation");
        add(&mut list, "Focus Address Bar", "Ctrl+L", "Navigation");
        add(&mut list, "Reload Page", "Ctrl+R", "View");
        add(&mut list, "Hard Reload", "Ctrl+Shift+R", "View");
        add(&mut list, "Bookmark Page", "Ctrl+D", "File");
        add(&mut list, "Bookmarks Bar", "Ctrl+Shift+B", "View");
        add(&mut list, "Downloads", "Ctrl+J", "Tools");
        add(&mut list, "History", "Ctrl+H", "Tools");
        add(&mut list, "Inspect Elements", "Ctrl+Shift+I", "Tools");
        add(&mut list, "Find on Page", "Ctrl+F", "Tools");
        add(&mut list, "Zoom In", "Ctrl++", "View");
        add(&mut list, "Zoom Out", "Ctrl+-", "View");
        add(&mut list, "Reset Zoom", "Ctrl+0", "View");
        add(&mut list, "Private Window", "Ctrl+Shift+P", "File");
    } else if lower.contains("dolphin") || lower.contains("nautilus") || lower.contains("file") {
        add(&mut list, "New Tab", "Ctrl+T", "Navigation");
        add(&mut list, "Close Tab", "Ctrl+W", "Navigation");
        add(&mut list, "New Folder", "F10", "File");
        add(&mut list, "Split View", "F3", "View");
        add(&mut list, "Terminal Panel", "F4", "Tools");
        add(&mut list, "Show Hidden Files", "Ctrl+H", "View");
        add(&mut list, "Search Files", "Ctrl+F", "Tools");
        add(&mut list, "Properties", "Alt+Return", "File");
        add(&mut list, "Go Back", "Alt+Left", "Navigation");
        add(&mut list, "Go Forward", "Alt+Right", "Navigation");
        add(&mut list, "Go Up", "Alt+Up", "Navigation");
        add(&mut list, "Home Folder", "Alt+Home", "Navigation");
    } else if lower.contains("blender") {
        add(&mut list, "Search Operator", "F3", "Tools");
        add(&mut list, "Render Still Image", "F12", "Tools");
        add(&mut list, "Render Animation", "Ctrl+F12", "Tools");
        add(&mut list, "Play Animation", "Space", "Playback");
        add(&mut list, "Top Ortho View", "Numpad 7", "View");
        add(&mut list, "Front Ortho View", "Numpad 1", "View");
        add(&mut list, "Right Ortho View", "Numpad 3", "View");
        add(&mut list, "Camera View", "Numpad 0", "View");
        add(&mut list, "Subdivide Surface", "Ctrl+1", "Edit");
        add(&mut list, "Join Selected", "Ctrl+J", "Edit");
        add(&mut list, "Separate Selection", "P", "Edit");
        add(&mut list, "Extrude Region", "E", "Tools");
        add(&mut list, "Inset Faces", "I", "Tools");
        add(&mut list, "Bevel Edges", "Ctrl+B", "Tools");
    } else if lower.contains("gimp") || lower.contains("krita") || lower.contains("inkscape") {
        add(&mut list, "New Layer", "Ctrl+Shift+N", "Layer");
        add(&mut list, "Duplicate Layer", "Ctrl+Shift+D", "Layer");
        add(&mut list, "Invert Selection", "Ctrl+I", "Select");
        add(&mut list, "Deselect All", "Ctrl+Shift+A", "Select");
        add(&mut list, "Export As...", "Ctrl+Shift+E", "File");
        add(&mut list, "Fit to Window", "Ctrl+Shift+J", "View");
        add(&mut list, "Brush Tool", "B", "Tools");
        add(&mut list, "Eraser Tool", "Shift+E", "Tools");
        add(&mut list, "Color Picker", "O", "Tools");
        add(&mut list, "Zoom 100%", "Ctrl+1", "View");
    } else if lower.contains("vlc") || lower.contains("spotify") || lower.contains("media") || lower.contains("audacity") {
        add(&mut list, "Play / Pause", "Space", "Playback");
        add(&mut list, "Next Track", "Ctrl+Right", "Playback");
        add(&mut list, "Previous Track", "Ctrl+Left", "Playback");
        add(&mut list, "Volume Up", "Ctrl+Up", "Audio");
        add(&mut list, "Volume Down", "Ctrl+Down", "Audio");
        add(&mut list, "Mute Audio", "M", "Audio");
        add(&mut list, "Fullscreen", "F", "View");
        add(&mut list, "Subtitles Toggle", "V", "Tools");
        add(&mut list, "Audio Track", "B", "Audio");
        add(&mut list, "Speed Up", "]", "Playback");
        add(&mut list, "Slow Down", "[", "Playback");
    } else if lower.contains("obs") || lower.contains("stream") || lower.contains("kdenlive") {
        add(&mut list, "Start/Stop Streaming", "Ctrl+F1", "Broadcast");
        add(&mut list, "Start/Stop Recording", "Ctrl+F2", "Broadcast");
        add(&mut list, "Pause Recording", "Ctrl+F3", "Broadcast");
        add(&mut list, "Studio Mode", "Ctrl+Shift+S", "View");
        add(&mut list, "Studio Transition", "Space", "Broadcast");
        add(&mut list, "Mute Mic", "Ctrl+Shift+M", "Audio");
        add(&mut list, "Screenshot Scene", "Ctrl+Shift+P", "Tools");
    } else if lower.contains("discord") || lower.contains("slack") {
        add(&mut list, "Quick Switcher", "Ctrl+K", "Navigation");
        add(&mut list, "Mute Mic", "Ctrl+Shift+M", "Audio");
        add(&mut list, "Deafen Audio", "Ctrl+Shift+D", "Audio");
        add(&mut list, "Answer Call", "Ctrl+Enter", "Tools");
        add(&mut list, "Decline Call", "Esc", "Tools");
        add(&mut list, "Mark as Read", "Esc", "General");
        add(&mut list, "Search Channel", "Ctrl+F", "Tools");
    } else if lower.contains("terminal")
        || lower.contains("konsole")
        || lower.contains("kitty")
        || lower.contains("alacritty")
    {
        add(&mut list, "New Tab", "Ctrl+Shift+T", "Navigation");
        add(&mut list, "Close Tab", "Ctrl+Shift+W", "Navigation");
        add(&mut list, "Copy Selection", "Ctrl+Shift+C", "Edit");
        add(&mut list, "Paste Clipboard", "Ctrl+Shift+V", "Edit");
        add(&mut list, "Clear Terminal", "Ctrl+L", "Tools");
        add(&mut list, "Interrupt Process", "Ctrl+C", "Tools");
        add(&mut list, "Send to Background", "Ctrl+Z", "Tools");
        add(&mut list, "Reverse History Search", "Ctrl+R", "Navigation");
        add(&mut list, "Split Horizontal", "Ctrl+Shift+O", "View");
        add(&mut list, "Split Vertical", "Ctrl+Shift+E", "View");
    } else if lower.contains("freecad") || lower.contains("kicad") || lower.contains("godot") {
        add(&mut list, "Fit All in View", "V", "View");
        add(&mut list, "Isometric View", "0", "View");
        add(&mut list, "Front View", "1", "View");
        add(&mut list, "Top View", "2", "View");
        add(&mut list, "Right View", "3", "View");
        add(&mut list, "Toggle Wireframe", "Shift+W", "View");
        add(&mut list, "Run / Play Scene", "F5", "Tools");
        add(&mut list, "Toggle Fullscreen", "F11", "View");
        add(&mut list, "Save Project", "Ctrl+S", "File");
        add(&mut list, "Undo", "Ctrl+Z", "Edit");
        add(&mut list, "Redo", "Ctrl+Y", "Edit");
    } else if lower.contains("steam") {
        add(&mut list, "Steam Overlay", "Shift+Tab", "Overlay");
        add(&mut list, "Take Screenshot", "F12", "Overlay");
        add(&mut list, "Toggle Fullscreen", "Alt+Enter", "View");
        add(&mut list, "Friends List", "Ctrl+F", "Navigation");
    }

    list
}

/// Default OS shortcuts used only as a fallback when no app-specific shortcuts were found
pub fn get_default_os_shortcuts() -> Vec<AppShortcut> {
    let mut list = Vec::new();
    let add = |list: &mut Vec<AppShortcut>, act: &str, sc: &str, cat: &str| {
        let (l1, l2) = make_smart_labels(act, sc);
        let ds = shortcut_to_duckyscript(sc, act);
        list.push(AppShortcut {
            action: act.to_string(),
            shortcut: sc.to_string(),
            duckyscript: ds,
            category: cat.to_string(),
            source: "Standard OS".to_string(),
            label_l1: l1,
            label_l2: l2,
        });
    };
    add(&mut list, "Save", "Ctrl+S", "File");
    add(&mut list, "Open", "Ctrl+O", "File");
    add(&mut list, "New", "Ctrl+N", "File");
    add(&mut list, "Print", "Ctrl+P", "File");
    add(&mut list, "Undo", "Ctrl+Z", "Edit");
    add(&mut list, "Redo", "Ctrl+Shift+Z", "Edit");
    add(&mut list, "Cut", "Ctrl+X", "Edit");
    add(&mut list, "Copy", "Ctrl+C", "Edit");
    add(&mut list, "Paste", "Ctrl+V", "Edit");
    add(&mut list, "Select All", "Ctrl+A", "Edit");
    add(&mut list, "Find", "Ctrl+F", "Tools");
    add(&mut list, "Close Window", "Ctrl+W", "File");
    add(&mut list, "Quit Application", "Ctrl+Q", "File");
    add(&mut list, "Fullscreen", "F11", "View");
    add(&mut list, "Help", "F1", "General");
    list
}

/// Queries online search (DuckDuckGo HTML + top cheatsheet pages) for application shortcuts
pub fn query_google_shortcuts(app: &str) -> Vec<AppShortcut> {
    let clean_app = normalize_app_name(app);
    let query = format!("{clean_app} keyboard shortcuts hotkeys cheatsheet");
    let ddg_url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        urlencoding::encode(&query)
    );

    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(4))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0")
        .build()
    {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut all_results: Vec<AppShortcut> = Vec::new();
    let mut seen_combos: HashSet<String> = HashSet::new();

    if let Ok(resp) = client.get(&ddg_url).send() {
        if let Ok(ddg_html) = resp.text() {
            // 1. Parse any shortcuts directly visible in search snippets
            for item in parse_google_html_shortcuts(&ddg_html) {
                let k = format!("{}:{}", item.action.to_lowercase(), item.shortcut.to_lowercase());
                if seen_combos.insert(k) {
                    all_results.push(item);
                }
            }

            // 2. Follow up to 2 top result cheatsheet URLs (uddg=...) to extract full HTML shortcut tables
            let target_urls = extract_ddg_result_urls(&ddg_html, 2);
            for page_url in target_urls {
                if let Ok(page_resp) = client.get(&page_url).send() {
                    if let Ok(page_html) = page_resp.text() {
                        for item in parse_google_html_shortcuts(&page_html) {
                            let k = format!(
                                "{}:{}",
                                item.action.to_lowercase(),
                                item.shortcut.to_lowercase()
                            );
                            if seen_combos.insert(k) {
                                all_results.push(item);
                            }
                        }
                    }
                }
                if all_results.len() >= 40 {
                    break;
                }
            }
        }
    }

    all_results
}

fn extract_ddg_result_urls(ddg_html: &str, max_urls: usize) -> Vec<String> {
    let mut urls = Vec::new();
    let Ok(re) = regex::Regex::new(r#"uddg=(https?%3A%2F%2F[^"&>\s]+)"#) else {
        return urls;
    };
    for cap in re.captures_iter(ddg_html) {
        if let Some(encoded) = cap.get(1) {
            let decoded = urlencoding::decode(encoded.as_str());
            if decoded.starts_with("http")
                && !decoded.contains("youtube.com")
                && !decoded.contains("reddit.com")
                && !decoded.contains(".pdf")
                && !urls.contains(&decoded)
            {
                urls.push(decoded);
                if urls.len() >= max_urls {
                    break;
                }
            }
        }
    }
    urls
}

/// Helper for URL encoding/decoding query parameters without an extra crate dependency
mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut out = String::new();
        for b in s.bytes() {
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                out.push(b as char);
            } else if b == b' ' {
                out.push('+');
            } else {
                out.push_str(&format!("%{:02X}", b));
            }
        }
        out
    }

    pub fn decode(s: &str) -> String {
        let mut bytes = Vec::with_capacity(s.len());
        let raw = s.as_bytes();
        let mut i = 0;
        while i < raw.len() {
            if raw[i] == b'%' && i + 2 < raw.len() {
                if let Ok(hex) = std::str::from_utf8(&raw[i + 1..i + 3]) {
                    if let Ok(val) = u8::from_str_radix(hex, 16) {
                        bytes.push(val);
                        i += 3;
                        continue;
                    }
                }
            } else if raw[i] == b'+' {
                bytes.push(b' ');
                i += 1;
                continue;
            }
            bytes.push(raw[i]);
            i += 1;
        }
        String::from_utf8_lossy(&bytes).to_string()
    }
}

fn clean_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
        .replace("&nbsp;", " ")
        .replace("&#8211;", "-")
        .replace("&#8212;", "-")
        .trim()
        .to_string()
}

fn looks_like_shortcut(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() || trimmed.len() > 26 {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    // Single function key or special key
    if matches!(
        lower.as_str(),
        "f1" | "f2"
            | "f3"
            | "f4"
            | "f5"
            | "f6"
            | "f7"
            | "f8"
            | "f9"
            | "f10"
            | "f11"
            | "f12"
            | "esc"
            | "escape"
            | "tab"
            | "space"
            | "enter"
            | "backspace"
            | "delete"
            | "insert"
            | "home"
            | "end"
            | "pageup"
            | "pagedown"
            | "middle-click"
    ) {
        return true;
    }
    // Single letter/symbol key (in a table `<kbd>` cell)
    if trimmed.len() == 1 && trimmed.chars().all(|c| c.is_ascii_uppercase() || c == '/') {
        return true;
    }
    // Combination with '+' such as "Ctrl+S", "F3 + G", "Alt+F4", "Shift+F3"
    if trimmed.contains('+') {
        let has_mod_or_fn = lower.contains("ctrl")
            || lower.contains("control")
            || lower.contains("alt")
            || lower.contains("shift")
            || lower.contains("super")
            || lower.contains("meta")
            || lower.contains("win")
            || lower.contains("f1")
            || lower.contains("f2")
            || lower.contains("f3")
            || lower.contains("f4")
            || lower.contains("f5")
            || lower.contains("f6")
            || lower.contains("f7")
            || lower.contains("f8")
            || lower.contains("f9");
        return has_mod_or_fn;
    }
    false
}

fn looks_like_action(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.len() < 3 || trimmed.len() > 55 {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    if matches!(
        lower.as_str(),
        "action" | "key" | "shortcut" | "command" | "description" | "press" | "i want to…" | "function"
    ) {
        return false;
    }
    trimmed.chars().any(|c| c.is_ascii_alphabetic()) && !looks_like_shortcut(trimmed)
}

/// Parses HTML content (tables, `<kbd>` elements, and text snippets) for action-to-hotkey pairs
pub fn parse_google_html_shortcuts(html: &str) -> Vec<AppShortcut> {
    let mut results = Vec::new();
    let mut seen_combos = HashSet::new();
    let tag_re = regex::Regex::new(r#"<[^>]+>"#).ok();

    let strip_tags = |raw: &str| -> String {
        let s = if let Some(ref tr) = tag_re {
            tr.replace_all(raw, " ").to_string()
        } else {
            raw.to_string()
        };
        clean_html_entities(&s)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };

    // 1. Parse HTML `<tr>` rows with two `<td>` cells (supports both [Action | Key] and [Key | Action])
    if let Ok(tr_re) =
        regex::Regex::new(r#"(?is)<tr[^>]*>\s*<td[^>]*>(.*?)</td>\s*<td[^>]*>(.*?)</td>\s*</tr>"#)
    {
        for cap in tr_re.captures_iter(html) {
            let c1 = strip_tags(cap.get(1).map(|m| m.as_str()).unwrap_or(""));
            let c2 = strip_tags(cap.get(2).map(|m| m.as_str()).unwrap_or(""));

            let pair = if looks_like_shortcut(&c1) && looks_like_action(&c2) {
                Some((c2, c1))
            } else if looks_like_action(&c1) && looks_like_shortcut(&c2) {
                Some((c1, c2))
            } else {
                None
            };

            if let Some((act, sc)) = pair {
                let norm_sc = sc.replace(" + ", "+");
                if !seen_combos.contains(&norm_sc) {
                    seen_combos.insert(norm_sc.clone());
                    let (l1, l2) = make_smart_labels(&act, &norm_sc);
                    let ds = shortcut_to_duckyscript(&norm_sc, &act);
                    results.push(AppShortcut {
                        action: act,
                        shortcut: norm_sc,
                        duckyscript: ds,
                        category: "Web Search".to_string(),
                        source: "Web Search".to_string(),
                        label_l1: l1,
                        label_l2: l2,
                    });
                }
            }
        }
    }

    // 2. Regex to match inline text "Action - Ctrl+Shift+Key" or "Ctrl+Shift+Key : Action" (including F3+Key)
    let re1 = regex::Regex::new(
        r#"(?i)([A-Za-z0-9][A-Za-z0-9 \t/&–—]{1,30})\s*[:\t|–—\-]+\s*((?:(?:Ctrl|Control|Alt|Shift|Super|Win|Meta|F[1-9]|F1[0-2])\s*\+\s*)+\S+)"#,
    )
    .ok();

    let re2 = regex::Regex::new(
        r#"(?i)((?:(?:Ctrl|Control|Alt|Shift|Super|Win|Meta|F[1-9]|F1[0-2])\s*\+\s*)+\S+)\s*[:\t|–—\-]+\s*([A-Za-z0-9][A-Za-z0-9 \t/&–—]{1,30})"#,
    )
    .ok();

    let text = if let Some(ref tr) = tag_re {
        clean_html_entities(&tr.replace_all(html, " \n "))
    } else {
        clean_html_entities(html)
    };

    if let Some(re) = re1 {
        for cap in re.captures_iter(&text) {
            let act = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let sc = cap
                .get(2)
                .map(|m| m.as_str().trim().trim_end_matches([',', '.', ';', ')']))
                .unwrap_or("")
                .replace(" + ", "+");
            if looks_like_action(act) && !sc.is_empty() && !seen_combos.contains(&sc) {
                seen_combos.insert(sc.clone());
                let (l1, l2) = make_smart_labels(act, &sc);
                let ds = shortcut_to_duckyscript(&sc, act);
                results.push(AppShortcut {
                    action: act.to_string(),
                    shortcut: sc,
                    duckyscript: ds,
                    category: "Web Search".to_string(),
                    source: "Web Search".to_string(),
                    label_l1: l1,
                    label_l2: l2,
                });
            }
        }
    }

    if let Some(re) = re2 {
        for cap in re.captures_iter(&text) {
            let sc = cap
                .get(1)
                .map(|m| m.as_str().trim().trim_end_matches([',', '.', ';', ')']))
                .unwrap_or("")
                .replace(" + ", "+");
            let act = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("");
            if looks_like_action(act) && !sc.is_empty() && !seen_combos.contains(&sc) {
                seen_combos.insert(sc.clone());
                let (l1, l2) = make_smart_labels(act, &sc);
                let ds = shortcut_to_duckyscript(&sc, act);
                results.push(AppShortcut {
                    action: act.to_string(),
                    shortcut: sc,
                    duckyscript: ds,
                    category: "Web Search".to_string(),
                    source: "Web Search".to_string(),
                    label_l1: l1,
                    label_l2: l2,
                });
            }
        }
    }

    results
}

/// Unified entrypoint: queries local KDE shortcuts, curated catalogs, and online search (if permitted)
pub fn search_app_shortcuts(app: &str, allow_google: bool) -> Vec<AppShortcut> {
    let clean = normalize_app_name(app);
    let mut combined: Vec<AppShortcut> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();

    // 1. Add curated app-specific catalog first so core actions are always present & well-labeled
    let curated = get_curated_shortcuts(&clean);
    for item in curated {
        let key = format!(
            "{}:{}",
            item.action.to_lowercase(),
            item.shortcut.to_lowercase()
        );
        if seen_keys.insert(key) {
            combined.push(item);
        }
    }

    // 2. Add local KDE Plasma 6 shortcuts scraped from ~/.config/kglobalshortcutsrc
    if !clean.is_empty() {
        let kde_results = crate::ai::shortcut_scraper::scrape_kde_shortcuts(&clean);
        for (act, sc) in kde_results {
            let key = format!("{}:{}", act.to_lowercase(), sc.to_lowercase());
            if seen_keys.insert(key) {
                let (l1, l2) = make_smart_labels(&act, &sc);
                let ds = shortcut_to_duckyscript(&sc, &act);
                combined.push(AppShortcut {
                    action: act,
                    shortcut: sc,
                    duckyscript: ds,
                    category: "KDE Plasma".to_string(),
                    source: "Local KDE".to_string(),
                    label_l1: l1,
                    label_l2: l2,
                });
            }
        }
    }

    // 3. If web search is permitted by user, query online cheatsheets
    if allow_google && !clean.is_empty() {
        let web_results = query_google_shortcuts(&clean);
        for item in web_results {
            let key = format!(
                "{}:{}",
                item.action.to_lowercase(),
                item.shortcut.to_lowercase()
            );
            if seen_keys.insert(key) {
                combined.push(item);
            }
        }
    }

    // 4. Only fall back to default OS shortcuts if nothing app-specific was found
    if combined.is_empty() {
        for item in get_default_os_shortcuts() {
            combined.push(item);
        }
    }

    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_app_name() {
        assert_eq!(normalize_app_name("org.kde.dolphin"), "Dolphin");
        assert_eq!(normalize_app_name("code-oss"), "VS Code");
        assert_eq!(normalize_app_name("google-chrome"), "Google Chrome");
        assert_eq!(normalize_app_name("firefox-developer-edition"), "Firefox");
        assert_eq!(normalize_app_name("blender"), "Blender");
        assert_eq!(normalize_app_name("Minecraft* 1.21.4"), "Minecraft");
        assert_eq!(normalize_app_name("org.prismlauncher.PrismLauncher"), "Minecraft");
    }

    #[test]
    fn test_make_smart_labels() {
        let (l1, l2) = make_smart_labels("Save All", "Ctrl+Shift+S");
        assert_eq!(l1, "SvAll");
        assert_eq!(l2, "^+S");

        let (l1, l2) = make_smart_labels("Command Palette", "Ctrl+Shift+P");
        assert_eq!(l1, "Cmd");
        assert_eq!(l2, "^+P");

        let (l1, l2) = make_smart_labels("Show Chunk Borders", "F3+G");
        assert_eq!(l1, "Chunk");
        assert_eq!(l2, "F3+G");
    }

    #[test]
    fn test_shortcut_to_duckyscript_and_dsvm_compilation() {
        let ds = shortcut_to_duckyscript("Ctrl+T", "New Tab");
        assert!(ds.contains("CONTROL t") || ds.contains("CONTROL T"));

        // Verify every curated shortcut across all supported apps compiles cleanly with DSVM v2!
        for app in [
            "Minecraft",
            "VS Code",
            "Firefox",
            "Dolphin",
            "Blender",
            "GIMP",
            "VLC",
            "OBS Studio",
            "Discord",
            "Terminal",
            "FreeCAD",
            "Steam",
        ] {
            let items = search_app_shortcuts(app, false);
            assert!(!items.is_empty(), "Expected shortcuts for {app}");
            for item in items {
                let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                    &item.duckyscript,
                    false,
                    false,
                );
                assert!(
                    compiled.is_success,
                    "Shortcut '{}' ({}) for {} failed DSVM compile: {} | script:\n{}",
                    item.action,
                    item.shortcut,
                    app,
                    compiled.error_comment,
                    item.duckyscript
                );
            }
        }
    }

    #[test]
    fn test_parse_google_html_shortcuts() {
        let html = r#"
            <table>
                <tr><td>Show coordinates / FPS</td><td><kbd>F3</kbd></td></tr>
                <tr><td><kbd>F3 + G</kbd></td><td>Chunk borders</td></tr>
            </table>
            <div>
                <p>Save File: Ctrl+S</p>
                <p>New Window - Ctrl+Shift+N</p>
                <p>Ctrl+F: Find</p>
            </div>
        "#;
        let list = parse_google_html_shortcuts(html);
        assert!(list.len() >= 4);
        assert!(list.iter().any(|s| s.shortcut == "F3+G"));
        assert!(list.iter().any(|s| s.shortcut == "F3"));
    }

    #[test]
    fn test_search_app_shortcuts_offline() {
        let list = search_app_shortcuts("VS Code", false);
        assert!(!list.is_empty());
        assert!(list.iter().any(|s| s.action == "Command Palette"));
        assert!(list.iter().any(|s| s.action == "Save All"));

        let mc = search_app_shortcuts("Minecraft", false);
        assert!(mc.len() >= 30);
        assert!(mc.iter().any(|s| s.shortcut == "F3+G"));
        assert!(mc.iter().any(|s| s.action == "Open Inventory"));
    }
}
