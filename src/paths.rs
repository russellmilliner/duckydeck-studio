use std::env;
use std::fs::DirBuilder;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::path::{Path, PathBuf};

const APP_DIR_NAME: &str = "duckydeck-studio";

pub fn is_flatpak() -> bool {
    Path::new("/.flatpak-info").exists() || env::var_os("FLATPAK_ID").is_some()
}

pub fn home_dir() -> Option<PathBuf> {
    for key in ["HOME", "USERPROFILE"] {
        if let Some(val) = env::var_os(key) {
            let p = PathBuf::from(&val);
            if !p.as_os_str().is_empty() && p.is_absolute() {
                return Some(p);
            }
        }
    }
    None
}

fn home_or_cwd() -> PathBuf {
    home_dir()
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn xdg_config_home() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("APPDATA") {
        let p = PathBuf::from(appdata);
        if p.is_absolute() {
            return p;
        }
    }
    if let Ok(val) = env::var("XDG_CONFIG_HOME") {
        let trimmed = val.trim();
        if !trimmed.is_empty() && trimmed.starts_with('/') && !trimmed.contains(':') {
            return PathBuf::from(trimmed);
        }
    }
    home_or_cwd().join(".config")
}

pub fn xdg_data_home() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("LOCALAPPDATA").or_else(|| env::var_os("APPDATA")) {
        let p = PathBuf::from(appdata);
        if p.is_absolute() {
            return p;
        }
    }
    if let Ok(val) = env::var("XDG_DATA_HOME") {
        let trimmed = val.trim();
        if !trimmed.is_empty()
            && !trimmed.contains(':')
            && trimmed.starts_with('/')
            && !trimmed.starts_with("/var/lib/")
            && !trimmed.starts_with("/usr/")
        {
            return PathBuf::from(trimmed);
        }
    }
    home_or_cwd().join(".local").join("share")
}

pub fn xdg_cache_home() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(localappdata) = env::var_os("LOCALAPPDATA") {
        let p = PathBuf::from(localappdata);
        if p.is_absolute() {
            return p.join("Temp");
        }
    }
    if let Ok(val) = env::var("XDG_CACHE_HOME") {
        let trimmed = val.trim();
        if !trimmed.is_empty() && trimmed.starts_with('/') && !trimmed.contains(':') {
            return PathBuf::from(trimmed);
        }
    }
    home_or_cwd().join(".cache")
}

pub fn config_dir() -> PathBuf {
    xdg_config_home().join(APP_DIR_NAME)
}

pub fn host_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return config_dir();
    }
    #[cfg(not(target_os = "windows"))]
    home_or_cwd().join(".config").join(APP_DIR_NAME)
}

pub fn data_dir() -> PathBuf {
    xdg_data_home().join(APP_DIR_NAME)
}

pub fn host_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return data_dir();
    }
    #[cfg(not(target_os = "windows"))]
    home_or_cwd()
        .join(".local")
        .join("share")
        .join(APP_DIR_NAME)
}

pub fn dekunukem_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("APPDATA") {
        return PathBuf::from(appdata).join("dekuNukem").join("duckypad_config");
    }
    home_or_cwd()
        .join(".local")
        .join("share")
        .join("duckypad_config")
}

pub fn dekunukem_autoswitcher_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("APPDATA") {
        return PathBuf::from(appdata)
            .join("dekuNukem")
            .join("duckypad_autoswitcher");
    }
    home_or_cwd()
        .join(".local")
        .join("share")
        .join("duckypad_autoswitcher")
}

pub fn cache_dir() -> PathBuf {
    xdg_cache_home().join(APP_DIR_NAME)
}

fn ensure_dir_0700(dir: &Path) -> bool {
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    builder.mode(0o700);
    if builder.create(dir).is_ok() {
        #[cfg(unix)]
        let _ = fs::set_permissions(dir, fs::Permissions::from_mode(0o700)); // ignore: best-effort mode enforcement after DirBuilder::mode(0o700)
        true
    } else {
        dir.is_dir()
    }
}

pub fn runtime_dir() -> PathBuf {
    if let Ok(val) = env::var("XDG_RUNTIME_DIR") {
        let trimmed = val.trim();
        if !trimmed.is_empty() && trimmed.starts_with('/') && !trimmed.contains(':') {
            let p = PathBuf::from(trimmed);
            if p.is_dir() || ensure_dir_0700(&p) {
                return p;
            }
        }
    }
    let fallback = data_dir().join("run");
    ensure_dir_0700(&fallback);
    fallback
}

pub fn kwin_runtime_dir() -> PathBuf {
    let dir = runtime_dir().join(APP_DIR_NAME);
    ensure_dir_0700(&dir);
    dir
}

pub fn sanitize_xdg_env() {
    let resolved_data_home = xdg_data_home();
    let needs_fix = match env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let trimmed = val.trim();
            trimmed.is_empty()
                || trimmed.contains(':')
                || !trimmed.starts_with('/')
                || trimmed.starts_with("/var/lib/")
                || trimmed.starts_with("/usr/")
        }
        Err(_) => true,
    };
    if needs_fix {
        env::set_var("XDG_DATA_HOME", &resolved_data_home);
    }

    let current_dirs = env::var("XDG_DATA_DIRS").unwrap_or_default();
    if !current_dirs.split(':').any(|p| p == "/usr/share") {
        let mut parts: Vec<String> = current_dirs
            .split(':')
            .filter(|s| !s.is_empty() && *s != ".")
            .map(|s| s.to_string())
            .collect();
        let flatpak_user_share = resolved_data_home
            .join("flatpak/exports/share")
            .to_string_lossy()
            .to_string();
        for std_dir in [
            flatpak_user_share,
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
