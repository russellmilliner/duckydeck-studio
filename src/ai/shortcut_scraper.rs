use std::fs;

/// Scrapes `~/.config/kglobalshortcutsrc` for KDE Plasma 6 application shortcuts
/// so AI profile generation uses the user's actual local keybindings.
pub fn scrape_kde_shortcuts(app_filter: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let config_path = crate::paths::xdg_config_home().join("kglobalshortcutsrc");
    let Ok(content) = fs::read_to_string(config_path) else {
        return results;
    };

    let filter_lower = app_filter.to_lowercase();
    let mut in_matching_section = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section = &line[1..line.len() - 1];
            in_matching_section = section.to_lowercase().contains(&filter_lower);
            continue;
        }

        if in_matching_section && !line.is_empty() && !line.starts_with('#') {
            if let Some((action_name, rhs)) = line.split_once('=') {
                // Format: Action=Shortcut,DefaultShortcut,FriendlyName
                let parts: Vec<&str> = rhs.split(',').collect();
                if let Some(&shortcut) = parts.first() {
                    if !shortcut.is_empty() && shortcut != "none" {
                        let friendly = parts.get(2).copied().unwrap_or(action_name);
                        results.push((friendly.to_string(), shortcut.to_string()));
                    }
                }
            }
        }
    }

    results
}
