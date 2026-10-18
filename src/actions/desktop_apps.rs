use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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

pub(crate) fn expand_tilde_str(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed == "~" {
        if let Some(home) = crate::paths::home_dir() {
            return home.to_string_lossy().to_string();
        }
    } else if let Some(rest) = trimmed.strip_prefix("~/") {
        if let Some(home) = crate::paths::home_dir() {
            let h = home.to_string_lossy();
            return format!("{}/{rest}", h.trim_end_matches('/'));
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
    let data_home = crate::paths::xdg_data_home();
    search_dirs.push(data_home.join("applications"));
    search_dirs.push(data_home.join("flatpak/exports/share/applications"));
    if let Some(home) = crate::paths::home_dir() {
        for cand in [
            home.join(".local/share/applications"),
            home.join(".local/share/flatpak/exports/share/applications"),
        ] {
            if !search_dirs.contains(&cand) {
                search_dirs.push(cand);
            }
        }
    }
    for sys_dir in [
        "/var/lib/flatpak/exports/share/applications",
        "/var/lib/snapd/desktop/applications",
        "/run/host/usr/local/share/applications",
        "/run/host/usr/share/applications",
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

    apps.sort_by_key(|a| a.name.to_ascii_lowercase());
    apps
}

pub(crate) fn find_terminal_emulator() -> Option<(String, Vec<String>)> {
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
