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

/// Normalizes an application identifier or window class to a human-friendly name
pub fn normalize_app_name(raw: &str) -> String {
    let lower = raw.trim().to_lowercase();
    if lower.contains("code") || lower.contains("vscode") {
        return "VS Code".to_string();
    }
    if lower.contains("chrome") {
        return "Google Chrome".to_string();
    }
    if lower.contains("firefox") {
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
    if lower.contains("discord") {
        return "Discord".to_string();
    }
    if lower.contains("slack") {
        return "Slack".to_string();
    }
    if lower.contains("terminal") || lower.contains("konsole") || lower.contains("kitty") || lower.contains("alacritty") {
        return "Terminal".to_string();
    }
    if lower.contains("kate") || lower.contains("kwrite") {
        return "Kate".to_string();
    }
    if lower.contains("libreoffice") {
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
    if lower.contains("steam") {
        return "Steam".to_string();
    }

    // Strip reverse-DNS prefixes (e.g. "org.kde.dolphin" -> "Dolphin")
    let cleaned = raw.trim();
    if let Some(pos) = cleaned.rfind('.') {
        let name = &cleaned[pos + 1..];
        if !name.is_empty() {
            let mut c = name.chars();
            return match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            };
        }
    }

    raw.trim().to_string()
}

/// Generates a concise 2-line label for a 16x16 / 10x10 key display
pub fn make_smart_labels(action: &str, shortcut: &str) -> (String, String) {
    let act_lower = action.to_lowercase();
    let l1 = if act_lower.contains("save all") {
        "SvAll".to_string()
    } else if act_lower.contains("save") {
        "Save".to_string()
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
    } else if act_lower.contains("prev tab") {
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
    } else {
        // Take first word or up to 5 chars
        action.split_whitespace().next().unwrap_or(action).chars().take(5).collect()
    };

    // Concise shortcut notation for Line 2
    let l2 = shortcut
        .replace("Control+", "^")
        .replace("Ctrl+", "^")
        .replace("Shift+", "+")
        .replace("Alt+", "@")
        .replace("Meta+", "#")
        .replace("Super+", "#")
        .replace("Windows+", "#")
        .chars()
        .take(5)
        .collect();

    (l1, l2)
}

/// Converts a shortcut string (e.g. "Ctrl+Shift+P") to standard DuckyScript 3
pub fn shortcut_to_duckyscript(shortcut: &str, action_hint: &str) -> String {
    let combo = crate::ai::shortcut_scraper::kde_shortcut_to_duckyscript(shortcut)
        .unwrap_or_else(|| {
            shortcut
                .replace("Ctrl+", "CONTROL ")
                .replace("Control+", "CONTROL ")
                .replace("Shift+", "SHIFT ")
                .replace("Alt+", "ALT ")
                .replace("Super+", "WINDOWS ")
                .replace("Meta+", "WINDOWS ")
        });

    let display_title = action_hint.chars().take(18).collect::<String>();
    format!(
        "// Macro for: {action_hint}\nDEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT {display_title}\nOLED_UPDATE\n{combo}\nDELAY 150\nOLED_RESTORE\n"
    )
}

/// Built-in curated database of shortcuts for top Linux applications
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

    if lower.contains("code") || lower.contains("vs") {
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
    } else if lower.contains("chrome") || lower.contains("firefox") || lower.contains("brave") || lower.contains("browser") {
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
    } else if lower.contains("gimp") || lower.contains("krita") {
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
    } else if lower.contains("vlc") || lower.contains("spotify") || lower.contains("media") {
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
    } else if lower.contains("obs") || lower.contains("stream") {
        add(&mut list, "Start/Stop Streaming", "Ctrl+F1", "Broadcast");
        add(&mut list, "Start/Stop Recording", "Ctrl+F2", "Broadcast");
        add(&mut list, "Pause Recording", "Ctrl+F3", "Broadcast");
        add(&mut list, "Studio Mode", "Ctrl+Shift+S", "View");
        add(&mut list, "Studio Transition", "Space", "Broadcast");
        add(&mut list, "Mute Mic", "Ctrl+Shift+M", "Audio");
        add(&mut list, "Screenshot Scene", "Ctrl+Shift+S", "Tools");
    } else if lower.contains("discord") || lower.contains("slack") {
        add(&mut list, "Quick Switcher", "Ctrl+K", "Navigation");
        add(&mut list, "Mute Mic", "Ctrl+Shift+M", "Audio");
        add(&mut list, "Deafen Audio", "Ctrl+Shift+D", "Audio");
        add(&mut list, "Answer Call", "Ctrl+Enter", "Tools");
        add(&mut list, "Decline Call", "Esc", "Tools");
        add(&mut list, "Mark as Read", "Esc", "General");
        add(&mut list, "Search Channel", "Ctrl+F", "Tools");
    } else if lower.contains("terminal") || lower.contains("konsole") || lower.contains("kitty") || lower.contains("alacritty") {
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
    } else {
        // General desktop default shortcuts
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
    }

    list
}

/// Queries Google search for cheatsheets and parses hotkey tables
pub fn query_google_shortcuts(app: &str) -> Vec<AppShortcut> {
    let clean_app = normalize_app_name(app);
    let query = format!("{clean_app} keyboard shortcuts cheatsheet");
    let url = format!(
        "https://www.google.com/search?q={}&hl=en",
        urlencoding::encode(&query)
    );

    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0")
        .build()
    {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let body = match client.get(&url).send() {
        Ok(resp) => match resp.text() {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        },
        Err(_) => return Vec::new(),
    };

    parse_google_html_shortcuts(&body)
}

/// Helper for URL encoding query parameters without an extra crate dependency
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
}

/// Parses HTML content extracted from search engines for action-to-hotkey patterns
pub fn parse_google_html_shortcuts(html: &str) -> Vec<AppShortcut> {
    let mut results = Vec::new();
    let mut seen_combos = HashSet::new();

    // Regex to match "Action - Ctrl+Shift+Key" or "Ctrl+Shift+Key : Action"
    // e.g. "Save All: Ctrl+Shift+S", "Duplicate Line — Shift+Alt+Down", "Ctrl+P: Quick Open"
    let re1 = regex::Regex::new(
        r#"(?i)([A-Za-z0-9][A-Za-z0-9 \t/&–—]{1,30})\s*[:\t|–—\-]+\s*((?:(?:Ctrl|Control|Alt|Shift|Super|Win|Meta)\s*\+\s*)+\S+)"#,
    ).ok();

    let re2 = regex::Regex::new(
        r#"(?i)((?:(?:Ctrl|Control|Alt|Shift|Super|Win|Meta)\s*\+\s*)+\S+)\s*[:\t|–—\-]+\s*([A-Za-z0-9][A-Za-z0-9 \t/&–—]{1,30})"#,
    ).ok();

    // Strips HTML tags
    let tag_re = regex::Regex::new(r#"<[^>]+>"#).ok();
    let text = if let Some(tr) = tag_re {
        tr.replace_all(html, " \n ").to_string()
    } else {
        html.to_string()
    };

    if let Some(re) = re1 {
        for cap in re.captures_iter(&text) {
            let act = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let sc = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("");
            if !act.is_empty() && !sc.is_empty() && !seen_combos.contains(sc) {
                seen_combos.insert(sc.to_string());
                let (l1, l2) = make_smart_labels(act, sc);
                let ds = shortcut_to_duckyscript(sc, act);
                results.push(AppShortcut {
                    action: act.to_string(),
                    shortcut: sc.to_string(),
                    duckyscript: ds,
                    category: "Google Search".to_string(),
                    source: "Google Search".to_string(),
                    label_l1: l1,
                    label_l2: l2,
                });
            }
        }
    }

    if let Some(re) = re2 {
        for cap in re.captures_iter(&text) {
            let sc = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            let act = cap.get(2).map(|m| m.as_str().trim()).unwrap_or("");
            if !act.is_empty() && !sc.is_empty() && !seen_combos.contains(sc) {
                seen_combos.insert(sc.to_string());
                let (l1, l2) = make_smart_labels(act, sc);
                let ds = shortcut_to_duckyscript(sc, act);
                results.push(AppShortcut {
                    action: act.to_string(),
                    shortcut: sc.to_string(),
                    duckyscript: ds,
                    category: "Google Search".to_string(),
                    source: "Google Search".to_string(),
                    label_l1: l1,
                    label_l2: l2,
                });
            }
        }
    }

    results
}

/// Unified entrypoint: queries Google (if permitted), KDE shortcuts, and curated catalogs
pub fn search_app_shortcuts(app: &str, allow_google: bool) -> Vec<AppShortcut> {
    let clean = normalize_app_name(app);
    let mut combined: Vec<AppShortcut> = Vec::new();
    let mut seen_keys: HashSet<String> = HashSet::new();

    // 1. If Google search is explicitly permitted by user, query Google
    if allow_google && !clean.is_empty() {
        let google_results = query_google_shortcuts(&clean);
        for item in google_results {
            let key = format!("{}:{}", item.action.to_lowercase(), item.shortcut.to_lowercase());
            if !seen_keys.contains(&key) {
                seen_keys.insert(key);
                combined.push(item);
            }
        }
    }

    // 2. Add local KDE Plasma 6 shortcuts scraped from ~/.config/kglobalshortcutsrc
    let kde_results = crate::ai::shortcut_scraper::scrape_kde_shortcuts(&clean);
    for (act, sc) in kde_results {
        let key = format!("{}:{}", act.to_lowercase(), sc.to_lowercase());
        if !seen_keys.contains(&key) {
            seen_keys.insert(key);
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

    // 3. Supplement with curated offline database
    let curated = get_curated_shortcuts(&clean);
    for item in curated {
        let key = format!("{}:{}", item.action.to_lowercase(), item.shortcut.to_lowercase());
        if !seen_keys.contains(&key) {
            seen_keys.insert(key);
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
    }

    #[test]
    fn test_make_smart_labels() {
        let (l1, l2) = make_smart_labels("Save All", "Ctrl+Shift+S");
        assert_eq!(l1, "SvAll");
        assert_eq!(l2, "^+S");

        let (l1, l2) = make_smart_labels("Command Palette", "Ctrl+Shift+P");
        assert_eq!(l1, "Cmd");
        assert_eq!(l2, "^+P");

        let (l1, l2) = make_smart_labels("New Tab", "Ctrl+T");
        assert_eq!(l1, "Tab+");
        assert_eq!(l2, "^T");
    }

    #[test]
    fn test_shortcut_to_duckyscript() {
        let ds = shortcut_to_duckyscript("Ctrl+T", "New Tab");
        assert!(ds.contains("CONTROL t") || ds.contains("CONTROL T"));

        let ds2 = shortcut_to_duckyscript("Ctrl+Shift+P", "Command Palette");
        assert!(ds2.contains("CONTROL SHIFT p") || ds2.contains("CONTROL SHIFT P"));
    }

    #[test]
    fn test_parse_google_html_shortcuts() {
        let html = r#"
            <div>
                <p>Save File: Ctrl+S</p>
                <p>New Window - Ctrl+Shift+N</p>
                <p>Ctrl+F: Find</p>
            </div>
        "#;
        let list = parse_google_html_shortcuts(html);
        assert!(!list.is_empty());
        let actions: Vec<&str> = list.iter().map(|s| s.action.as_str()).collect();
        assert!(actions.iter().any(|a| a.contains("Save File") || a.contains("Find") || a.contains("New Window")));
    }

    #[test]
    fn test_search_app_shortcuts_offline() {
        let list = search_app_shortcuts("VS Code", false);
        assert!(!list.is_empty());
        assert!(list.iter().any(|s| s.action == "Command Palette"));
        assert!(list.iter().any(|s| s.action == "Save All"));
    }
}
