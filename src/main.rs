pub mod bridge {
    pub mod device_controller;
}
pub mod hardware {
    pub mod permissions;
    pub mod storage;
}
pub mod kwin {
    pub mod auto_switcher;
}
pub mod ai {
    pub mod shortcut_scraper;
}
pub mod vm;

use std::env;

fn sanitize_xdg_env() {
    let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let default_data_home = format!("{home}/.local/share");
    let needs_fix = match env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let trimmed = val.trim();
            trimmed.is_empty()
                || trimmed.contains(':')
                || !trimmed.starts_with('/')
                || trimmed.starts_with("/var/")
                || trimmed.starts_with("/usr/")
        }
        Err(_) => true,
    };
    if needs_fix {
        env::set_var("XDG_DATA_HOME", &default_data_home);
    }

    let current_dirs = env::var("XDG_DATA_DIRS").unwrap_or_default();
    if !current_dirs.split(':').any(|p| p == "/usr/share") {
        let mut parts: Vec<String> = current_dirs
            .split(':')
            .filter(|s| !s.is_empty() && *s != ".")
            .map(|s| s.to_string())
            .collect();
        for std_dir in [
            format!("{home}/.local/share/flatpak/exports/share"),
            "/var/lib/flatpak/exports/share".to_string(),
            "/usr/local/share".to_string(),
            "/usr/share".to_string(),
        ] {
            if !parts.contains(&std_dir) {
                parts.push(std_dir);
            }
        }
        env::set_var("XDG_DATA_DIRS", parts.join(":"));
    }

    let quiet_rules = "union.*=false;qt.core.qobject.connect=false";
    match env::var("QT_LOGGING_RULES") {
        Ok(existing) if !existing.trim().is_empty() => {
            if !existing.contains("union.*=false") {
                env::set_var("QT_LOGGING_RULES", format!("{existing};{quiet_rules}"));
            }
        }
        _ => {
            env::set_var("QT_LOGGING_RULES", quiet_rules);
        }
    }
}

fn main() {
    sanitize_xdg_env();
    if env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1") {
        env::set_var("QT_QPA_PLATFORM", "xcb");
        env::set_var("QT_SCALE_FACTOR", "1");
        env::set_var("QT_AUTO_SCREEN_SCALE_FACTOR", "0");
        env::set_var("QT_SCREEN_SCALE_FACTORS", "1");
    }
    let start_minimized = env::args().any(|arg| arg == "--minimized" || arg == "--tray");
    let exit_code = bridge::device_controller::run_application(start_minimized);
    std::process::exit(exit_code);
}

