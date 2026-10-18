use std::fs;

/// Scrapes `~/.config/kglobalshortcutsrc` for KDE Plasma 6 application shortcuts
/// so macro template generation uses the user's actual local keybindings.
pub fn scrape_kde_shortcuts(app_filter: &str) -> Vec<(String, String)> {
    let config_path = crate::paths::xdg_config_home().join("kglobalshortcutsrc");
    let Ok(content) = fs::read_to_string(config_path) else {
        return Vec::new();
    };
    parse_kde_shortcuts_str(&content, app_filter)
}

/// Parses `kglobalshortcutsrc` content and returns all `(friendly_name, primary_shortcut)` matches.
pub fn parse_kde_shortcuts_str(content: &str, app_filter: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let filter_lower = app_filter.trim().to_lowercase();
    let mut in_matching_section = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section = &line[1..line.len() - 1];
            in_matching_section =
                filter_lower.is_empty() || section.to_lowercase().contains(&filter_lower);
            continue;
        }

        if in_matching_section && !line.is_empty() && !line.starts_with('#') {
            if let Some((action_name, rhs)) = line.split_once('=') {
                if action_name.trim() == "_k_friendly_name" {
                    continue;
                }
                // Format: Action=Shortcut,DefaultShortcut,FriendlyName
                let parts: Vec<&str> = rhs.splitn(3, ',').collect();
                if let Some(&shortcut_raw) = parts.first() {
                    let primary = shortcut_raw
                        .split(['\t', ';'])
                        .map(str::trim)
                        .find(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"));
                    if let Some(shortcut) = primary {
                        let friendly = parts
                            .get(2)
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .unwrap_or_else(|| action_name.trim());
                        results.push((friendly.to_string(), shortcut.to_string()));
                    }
                }
            }
        }
    }

    results
}

/// Converts a KDE `QKeySequence` shortcut string (e.g. `"Meta+Shift+S"`) into
/// duckyScript 3 key combo syntax (e.g. `"WINDOWS SHIFT s"`).
pub fn kde_shortcut_to_duckyscript(shortcut: &str) -> Option<String> {
    let primary = shortcut
        .split(['\t', ';', ','])
        .map(str::trim)
        .find(|s| !s.is_empty() && !s.eq_ignore_ascii_case("none"))?;

    let mut tokens: Vec<String> = Vec::new();
    for raw_part in primary.split('+') {
        let part = raw_part.trim();
        if part.is_empty() {
            continue;
        }
        let mapped = match part.to_ascii_lowercase().as_str() {
            "meta" | "super" | "win" | "windows" | "gui" => "WINDOWS".to_string(),
            "ctrl" | "control" => "CONTROL".to_string(),
            "shift" => "SHIFT".to_string(),
            "alt" => "ALT".to_string(),
            "altgr" => "ALTGR".to_string(),
            "return" | "enter" => "ENTER".to_string(),
            "esc" | "escape" => "ESCAPE".to_string(),
            "del" | "delete" => "DELETE".to_string(),
            "ins" | "insert" => "INSERT".to_string(),
            "backspace" | "bs" => "BACKSPACE".to_string(),
            "pgup" | "pageup" | "prior" => "PAGEUP".to_string(),
            "pgdown" | "pgdn" | "pagedown" | "next" => "PAGEDOWN".to_string(),
            "space" => "SPACE".to_string(),
            "tab" | "backtab" => "TAB".to_string(),
            "up" => "UP".to_string(),
            "down" => "DOWN".to_string(),
            "left" => "LEFT".to_string(),
            "right" => "RIGHT".to_string(),
            "home" => "HOME".to_string(),
            "end" => "END".to_string(),
            "print" | "sysreq" | "printscreen" => "PRINTSCREEN".to_string(),
            "capslock" => "CAPSLOCK".to_string(),
            "numlock" => "NUMLOCK".to_string(),
            "scrolllock" => "SCROLLLOCK".to_string(),
            "pause" => "PAUSE".to_string(),
            "menu" | "apps" => "MENU".to_string(),
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
                } else if part.len() == 1 {
                    part.to_ascii_lowercase()
                } else {
                    part.to_ascii_uppercase()
                }
            }
        };
        tokens.push(mapped);
    }

    if tokens.is_empty() {
        None
    } else {
        Some(tokens.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kde_shortcut_to_duckyscript_conversion() {
        assert_eq!(
            kde_shortcut_to_duckyscript("Meta+Shift+S").as_deref(),
            Some("WINDOWS SHIFT s")
        );
        assert_eq!(
            kde_shortcut_to_duckyscript("Ctrl+Shift+M").as_deref(),
            Some("CONTROL SHIFT m")
        );
        assert_eq!(
            kde_shortcut_to_duckyscript("Ctrl+Alt+Delete").as_deref(),
            Some("CONTROL ALT DELETE")
        );
        assert_eq!(
            kde_shortcut_to_duckyscript("Alt+F4").as_deref(),
            Some("ALT F4")
        );
        assert_eq!(
            kde_shortcut_to_duckyscript("Meta+Return\tCtrl+Enter").as_deref(),
            Some("WINDOWS ENTER")
        );
        assert_eq!(kde_shortcut_to_duckyscript("none"), None);
        assert_eq!(kde_shortcut_to_duckyscript(""), None);
    }

    #[test]
    fn test_parse_kde_shortcuts_str_returns_all_matches() {
        let sample = "\
[org.kde.spectacle.desktop]
_k_friendly_name=Spectacle,none,Spectacle
RectangularRegionScreenShot=Meta+Shift+S,Meta+Shift+S,Capture Rectangular Region
FullScreenScreenShot=Shift+Print,Shift+Print,Capture Entire Desktop
ActiveWindowScreenShot=Meta+Print,Meta+Print,Capture Active Window
";
        let matches = parse_kde_shortcuts_str(sample, "spectacle");
        assert_eq!(matches.len(), 3);
        assert_eq!(
            matches[0],
            (
                "Capture Rectangular Region".to_string(),
                "Meta+Shift+S".to_string()
            )
        );
        assert_eq!(
            kde_shortcut_to_duckyscript(&matches[0].1).as_deref(),
            Some("WINDOWS SHIFT s")
        );
    }
}
