use hidapi::HidApi;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use zbus::{interface, Result as ZbusResult};

use crate::hardware::storage::{
    DuckyProfile, DUCKYPAD_VID, HID_CMD_GOTO_PROFILE_BY_NUM, HID_CMD_QUERY_INFO, HID_REPORT_ID_OUT,
    PID_DUCKYPAD_2020, PID_DUCKYPAD_PRO,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSwitchRule {
    pub enabled: bool,
    pub resource_class: String,
    pub title_contains: Option<String>,
    #[serde(default)]
    pub favicon_url: Option<String>,
    pub target_profile: u8,
}

#[derive(Debug, Clone)]
pub struct WindowChangeEvent {
    pub resource_class: String,
    pub resource_name: String,
    pub caption: String,
    pub matched_profile: Option<u8>,
    pub picked: bool,
}

fn default_tray_icon_mode() -> String {
    "auto".to_string()
}

fn default_app_theme_mode() -> String {
    "system".to_string()
}

fn default_true() -> bool {
    true
}

fn default_ui_text_scale() -> i32 {
    100
}

pub fn default_saved_colors() -> Vec<String> {
    vec![
        "#00aaff".to_string(),
        "#ff3b30".to_string(),
        "#ff9500".to_string(),
        "#ffcc00".to_string(),
        "#34c759".to_string(),
        "#00c7be".to_string(),
        "#5856d6".to_string(),
        "#af52de".to_string(),
        "#ff2d55".to_string(),
        "#ffffff".to_string(),
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedSwitcherConfig {
    pub enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    #[serde(default = "default_tray_icon_mode")]
    pub tray_icon_mode: String,
    #[serde(default = "default_app_theme_mode")]
    pub app_theme_mode: String,
    #[serde(default = "default_true")]
    pub close_to_tray: bool,
    #[serde(default = "default_ui_text_scale")]
    pub ui_text_scale: i32,
    #[serde(default = "default_ui_text_scale")]
    pub sidebar_text_scale: i32,
    #[serde(default = "default_ui_text_scale")]
    pub canvas_text_scale: i32,
    #[serde(default = "default_ui_text_scale")]
    pub inspector_text_scale: i32,
    #[serde(default = "default_ui_text_scale")]
    pub editor_text_scale: i32,
    #[serde(default = "default_ui_text_scale")]
    pub status_bar_text_scale: i32,
    #[serde(default = "default_ui_text_scale")]
    pub event_log_text_scale: i32,
    #[serde(default = "default_saved_colors")]
    pub saved_colors: Vec<String>,
}

pub struct SwitcherState {
    pub enabled: bool,
    pub paused_for_editing: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    pub tray_icon_mode: String,
    pub app_theme_mode: String,
    pub close_to_tray: bool,
    pub ui_text_scale: i32,
    pub sidebar_text_scale: i32,
    pub canvas_text_scale: i32,
    pub inspector_text_scale: i32,
    pub editor_text_scale: i32,
    pub status_bar_text_scale: i32,
    pub event_log_text_scale: i32,
    pub saved_colors: Vec<String>,
    pub last_switched_profile: u8,
    pub picker_armed: bool,
    pub loaded_from_disk: bool,
}

impl Default for SwitcherState {
    fn default() -> Self {
        if let Some(cfg) = Self::load_config_from_disk() {
            let mode = match cfg.tray_icon_mode.as_str() {
                "dark" | "light" | "auto" => cfg.tray_icon_mode,
                _ => "auto".to_string(),
            };
            let theme = match cfg.app_theme_mode.as_str() {
                "dark" | "light" | "system" => cfg.app_theme_mode,
                _ => "system".to_string(),
            };
            let saved_colors = if cfg.saved_colors.is_empty() {
                default_saved_colors()
            } else {
                cfg.saved_colors
            };
            return Self {
                enabled: cfg.enabled,
                paused_for_editing: false,
                default_profile: cfg.default_profile,
                rules: cfg.rules,
                tray_icon_mode: mode,
                app_theme_mode: theme,
                close_to_tray: cfg.close_to_tray,
                ui_text_scale: cfg.ui_text_scale.clamp(75, 160),
                sidebar_text_scale: cfg.sidebar_text_scale.clamp(75, 160),
                canvas_text_scale: cfg.canvas_text_scale.clamp(75, 160),
                inspector_text_scale: cfg.inspector_text_scale.clamp(75, 160),
                editor_text_scale: cfg.editor_text_scale.clamp(75, 160),
                status_bar_text_scale: cfg.status_bar_text_scale.clamp(75, 160),
                event_log_text_scale: cfg.event_log_text_scale.clamp(75, 160),
                saved_colors,
                last_switched_profile: 0,
                picker_armed: false,
                loaded_from_disk: true,
            };
        }

        Self {
            enabled: true,
            paused_for_editing: false,
            default_profile: Some(1),
            rules: vec![
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "org.kde.dolphin".into(),
                    title_contains: None,
                    favicon_url: None,
                    target_profile: 2,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "firefox".into(),
                    title_contains: None,
                    favicon_url: None,
                    target_profile: 3,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "code".into(),
                    title_contains: None,
                    favicon_url: None,
                    target_profile: 4,
                },
            ],
            tray_icon_mode: "auto".to_string(),
            app_theme_mode: "system".to_string(),
            close_to_tray: true,
            ui_text_scale: 100,
            sidebar_text_scale: 100,
            canvas_text_scale: 100,
            inspector_text_scale: 100,
            editor_text_scale: 100,
            status_bar_text_scale: 100,
            event_log_text_scale: 100,
            saved_colors: default_saved_colors(),
            last_switched_profile: 0,
            picker_armed: false,
            loaded_from_disk: false,
        }
    }
}

impl SwitcherState {
    pub fn config_file_path() -> PathBuf {
        let base = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                PathBuf::from(home).join(".config")
            });
        base.join("duckydeck-studio").join("kwin_rules.json")
    }

    pub fn load_config_from_disk() -> Option<PersistedSwitcherConfig> {
        let path = Self::config_file_path();
        let raw = fs::read_to_string(path).ok()?;
        serde_json::from_str(&raw).ok()
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        if std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1") {
            return Ok(());
        }
        let path = Self::config_file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed creating config dir: {e}"))?;
        }
        let payload = PersistedSwitcherConfig {
            enabled: self.enabled,
            default_profile: self.default_profile,
            rules: self.rules.clone(),
            tray_icon_mode: self.tray_icon_mode.clone(),
            app_theme_mode: self.app_theme_mode.clone(),
            close_to_tray: self.close_to_tray,
            ui_text_scale: self.ui_text_scale.clamp(75, 160),
            sidebar_text_scale: self.sidebar_text_scale.clamp(75, 160),
            canvas_text_scale: self.canvas_text_scale.clamp(75, 160),
            inspector_text_scale: self.inspector_text_scale.clamp(75, 160),
            editor_text_scale: self.editor_text_scale.clamp(75, 160),
            status_bar_text_scale: self.status_bar_text_scale.clamp(75, 160),
            event_log_text_scale: self.event_log_text_scale.clamp(75, 160),
            saved_colors: self.saved_colors.clone(),
        };
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("Failed serializing KWin rules: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("Failed writing {}: {e}", path.display()))
    }
}

pub const ANY_BROWSER_CLASS: &str = "*browser*";

pub fn is_any_browser_wildcard(rule_class: &str) -> bool {
    matches!(
        rule_class.trim().to_ascii_lowercase().as_str(),
        "*browser*"
            | "*browsers*"
            | "any-browser"
            | "all-browsers"
            | "web-browser"
            | "web-browsers"
            | "browser"
            | "browsers"
    )
}

pub fn is_web_browser_class(resource_class: &str) -> bool {
    let trimmed = resource_class.trim();
    if trimmed.is_empty() {
        return false;
    }
    if is_any_browser_wildcard(trimmed) {
        return true;
    }

    for part in trimmed.split([',', '|']) {
        let token = part.trim().to_ascii_lowercase();
        if token.is_empty() {
            continue;
        }
        if is_any_browser_wildcard(&token) {
            return true;
        }
        let short = token.rsplit('.').next().unwrap_or(token.as_str());
        let is_browser = matches!(
            short,
            "chrome"
                | "google-chrome"
                | "google-chrome-stable"
                | "google-chrome-beta"
                | "google-chrome-unstable"
                | "chromium"
                | "chromium-browser"
                | "ungoogled-chromium"
                | "thorium"
                | "thorium-browser"
                | "zen"
                | "zen-alpha"
                | "zen-beta"
                | "zen-twilight"
                | "zen-browser"
                | "zen_browser"
                | "firefox"
                | "firefox-esr"
                | "firefox-devedition"
                | "firefox-nightly"
                | "navigator"
                | "librewolf"
                | "librewolf-community"
                | "waterfox"
                | "floorp"
                | "tor-browser"
                | "icecat"
                | "palemoon"
                | "seamonkey"
                | "basilisk"
                | "mullvadbrowser"
                | "mercury"
                | "edge"
                | "microsoft-edge"
                | "microsoft-edge-stable"
                | "microsoft-edge-beta"
                | "microsoft-edge-dev"
                | "msedge"
                | "brave"
                | "brave-browser"
                | "brave-bin"
                | "opera"
                | "opera-beta"
                | "opera-developer"
                | "vivaldi"
                | "vivaldi-stable"
                | "vivaldi-snapshot"
                | "falkon"
                | "konqueror"
                | "epiphany"
                | "midori"
                | "qutebrowser"
                | "nyxt"
                | "min"
                | "arc"
                | "ladybird"
                | "whale"
                | "naver-whale"
                | "yandex-browser"
                | "maxthon"
                | "slimjet"
                | "wavebox"
                | "sidekick"
                | "dillo"
                | "netsurf"
                | "luakit"
                | "vieb"
        ) || token.contains("chrome")
            || token.contains("chromium")
            || token.contains("firefox")
            || token.contains("zen_browser")
            || token.contains("zen-browser")
            || token.contains("brave")
            || token.contains("microsoft-edge")
            || token.contains("vivaldi")
            || token.contains("opera")
            || token.contains("librewolf")
            || token.contains("waterfox")
            || token.contains("floorp");
        if is_browser {
            return true;
        }
    }
    false
}

pub fn rule_matches_window_class(rule_class_raw: &str, active_window_class_lower: &str) -> bool {
    let trimmed = rule_class_raw.trim();
    if trimmed.is_empty() || active_window_class_lower.is_empty() {
        return false;
    }
    if is_any_browser_wildcard(trimmed) {
        return is_web_browser_class(active_window_class_lower);
    }

    for part in trimmed.split([',', '|']) {
        let token = part.trim().to_ascii_lowercase();
        if token.is_empty() {
            continue;
        }
        if is_any_browser_wildcard(&token) {
            if is_web_browser_class(active_window_class_lower) {
                return true;
            }
            continue;
        }
        if active_window_class_lower == token || active_window_class_lower.contains(&token) {
            return true;
        }
    }
    false
}

pub fn extract_browser_title_suffix(caption: &str) -> Option<&'static str> {
    let trimmed = caption.trim();
    let lower = trimmed.to_lowercase();
    let suffixes: [&'static str; 38] = [
        " — Zen Browser",
        " - Zen Browser",
        " — Zen Twilight",
        " - Zen Twilight",
        " — Zen",
        " — Mozilla Firefox",
        " - Mozilla Firefox",
        " — Firefox Developer Edition",
        " - Firefox Developer Edition",
        " — Firefox Nightly",
        " - Firefox Nightly",
        " — Firefox",
        " - Firefox",
        " - Google Chrome",
        " — Google Chrome",
        " - Chromium",
        " — Chromium",
        " - Thorium",
        " — Thorium",
        " - Brave",
        " — Brave",
        " - Microsoft\u{200b} Edge",
        " - Microsoft Edge",
        " — Microsoft Edge",
        " - Opera",
        " — Opera",
        " - Vivaldi",
        " — Vivaldi",
        " — LibreWolf",
        " - LibreWolf",
        " — Waterfox",
        " - Waterfox",
        " — Floorp",
        " - Floorp",
        " — Falkon",
        " - Falkon",
        " — qutebrowser",
        " - qutebrowser",
    ];
    for suf in suffixes {
        if trimmed.len() > suf.len() && lower.ends_with(&suf.to_lowercase()) {
            return Some(suf);
        }
    }
    None
}

pub fn strip_browser_title_suffix(caption: &str) -> String {
    let mut s = caption.trim().to_string();
    if let Some(suf) = extract_browser_title_suffix(&s) {
        let new_len = s.len() - suf.len();
        s.truncate(new_len);
        s = s.trim().to_string();
    }
    // Also strip leading unread notification badge like "(3) YouTube" -> "YouTube"
    if let Some(rest) = s.strip_prefix('(') {
        if let Some((digits, after)) = rest.split_once(')') {
            if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
                let cleaned = after.trim();
                if !cleaned.is_empty() {
                    s = cleaned.to_string();
                }
            }
        }
    }
    s
}

static RESOLVED_BROWSER_TITLES: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

static LAST_BROWSER_SITE_SUFFIX: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

fn decompress_mozlz4(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 12 || &data[..8] != b"mozLz40\0" {
        return None;
    }
    let uncomp_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
    if uncomp_len == 0 || uncomp_len > 64 * 1024 * 1024 {
        return None;
    }
    let src = &data[12..];
    let mut out = Vec::with_capacity(uncomp_len);
    let mut i = 0usize;
    let n = src.len();
    while i < n {
        let token = src[i] as usize;
        i += 1;
        let mut lit_len = token >> 4;
        if lit_len == 15 {
            while i < n {
                let b = src[i] as usize;
                i += 1;
                lit_len += b;
                if b != 255 {
                    break;
                }
            }
        }
        if i + lit_len > n {
            break;
        }
        out.extend_from_slice(&src[i..i + lit_len]);
        i += lit_len;
        if i >= n {
            break;
        }
        if i + 2 > n {
            break;
        }
        let offset = (src[i] as usize) | ((src[i + 1] as usize) << 8);
        i += 2;
        if offset == 0 || offset > out.len() {
            return None;
        }
        let mut match_len = (token & 0x0f) + 4;
        if (token & 0x0f) == 15 {
            while i < n {
                let b = src[i] as usize;
                i += 1;
                match_len += b;
                if b != 255 {
                    break;
                }
            }
        }
        let start = out.len() - offset;
        for j in 0..match_len {
            let b = out[start + j];
            out.push(b);
        }
    }
    Some(out)
}

fn clean_sqlite_extracted_title(raw: &str) -> String {
    let mut s = raw.trim().to_string();
    // In Firefox/Zen moz_places records, the `rev_host` column (e.g. `moc.elgoog.liam.`)
    // immediately follows `title` in the raw row payload.
    if let Some(pos) = s.find("moc.elgoog.liam.") {
        s.truncate(pos);
        if !s.to_lowercase().contains("gmail") {
            s.push_str(" - Gmail");
        }
        return s.trim().to_string();
    }
    for rev_marker in [
        "moc.elgoog.",
        "moc.ebotuoy.",
        "moc.kooltuo.",
        "moc.tfosorcim.",
        "moc.buhtig.",
    ] {
        if let Some(pos) = s.find(rev_marker) {
            s.truncate(pos);
            return s.trim().to_string();
        }
    }
    // Strip any generic trailing reversed hostname like `moc.elpmaxe.www.` if glued to "- Gmail"
    if let Some(gmail_end) = s.rfind(" - Gmail") {
        let after = &s[gmail_end + " - Gmail".len()..];
        if after.ends_with('.')
            && after
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-')
        {
            s.truncate(gmail_end + " - Gmail".len());
        }
    }
    s.trim().to_string()
}

fn lookup_full_title_in_browser_storage(search_prefix: &str) -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let home_path = PathBuf::from(&home);
    let roots = [
        home_path.join(".var/app/app.zen_browser.zen/.zen"),
        home_path.join(".zen"),
        home_path.join(".mozilla/firefox"),
        home_path.join(".var/app/org.mozilla.firefox/.mozilla/firefox"),
    ];

    let prefix_bytes = search_prefix.as_bytes();
    if prefix_bytes.len() < 32 {
        return None;
    }

    let mut profile_dirs: Vec<(std::time::SystemTime, PathBuf)> = Vec::new();
    for root in &roots {
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_dir() {
                continue;
            }
            let mtime = p
                .join("places.sqlite-wal")
                .metadata()
                .or_else(|_| p.join("places.sqlite").metadata())
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            profile_dirs.push((mtime, p));
        }
    }
    profile_dirs.sort_by(|a, b| b.0.cmp(&a.0));

    // 1. Check recovery.jsonlz4 first (live sessionstore with exact tab title & URL)
    for (_, prof_dir) in &profile_dirs {
        let recovery_path = prof_dir.join("sessionstore-backups/recovery.jsonlz4");
        if let Ok(compressed) = fs::read(&recovery_path) {
            if let Some(raw_json) = decompress_mozlz4(&compressed) {
                if let Some(pos) = raw_json
                    .windows(prefix_bytes.len())
                    .rposition(|w| w == prefix_bytes)
                {
                    let slice = &raw_json[pos..raw_json.len().min(pos + 1024)];
                    let mut end = 0usize;
                    while end < slice.len() {
                        if slice[end] == b'"' && (end == 0 || slice[end - 1] != b'\\') {
                            break;
                        }
                        if slice[end] < 0x20 {
                            break;
                        }
                        end += 1;
                    }
                    let extracted = String::from_utf8_lossy(&slice[..end]).to_string();
                    if extracted.len() > search_prefix.len() {
                        return Some(extracted);
                    }
                }
            }
        }
    }

    // 2. Check places.sqlite-wal and places.sqlite (contains full untruncated moz_places title & rev_host)
    for (_, prof_dir) in &profile_dirs {
        for fname in ["places.sqlite-wal", "places.sqlite"] {
            let db_path = prof_dir.join(fname);
            let Ok(data) = fs::read(&db_path) else {
                continue;
            };
            if let Some(pos) = data
                .windows(prefix_bytes.len())
                .rposition(|w| w == prefix_bytes)
            {
                let slice = &data[pos..data.len().min(pos + 1024)];
                let text = String::from_utf8_lossy(slice);
                let printable: String = text.chars().take_while(|&c| (c as u32) >= 0x20).collect();
                let cleaned = clean_sqlite_extracted_title(&printable);
                if cleaned.len() > search_prefix.len() {
                    return Some(cleaned);
                }
            }
        }
    }

    None
}

/// Resolves the full untruncated title of a browser window when the browser (e.g. Zen Browser / Firefox)
/// truncates `<title>` at 255/256 UTF-8 bytes before appending the site name (such as `- russell.milliner@gmail.com - Gmail`).
pub fn resolve_full_browser_caption(resource_class: &str, caption: &str) -> String {
    if !is_web_browser_class(resource_class) {
        return caption.to_string();
    }
    let stripped = strip_browser_title_suffix(caption);
    let browser_suffix = extract_browser_title_suffix(caption).unwrap_or("");
    let lower_class = resource_class.trim().to_ascii_lowercase();

    // If this normal (non-truncated) browser title explicitly shows Gmail, remember that this browser window
    // is currently on a Gmail tab in case the user clicks an email with a 256+ byte subject next.
    if stripped.len() < 200 {
        if let Ok(mut last_map) = LAST_BROWSER_SITE_SUFFIX.lock() {
            if stripped.to_ascii_lowercase().contains("gmail") {
                last_map.insert(lower_class, " - Gmail".to_string());
            } else if !stripped.is_empty()
                && !stripped.eq_ignore_ascii_case("zen browser")
                && !stripped.eq_ignore_ascii_case("mozilla firefox")
            {
                last_map.remove(&lower_class);
            }
        }
        return caption.to_string();
    }

    // Build a stable 96-character search prefix from the truncated title
    let search_prefix: String = stripped.chars().take(96).collect();
    if search_prefix.len() < 40 {
        return caption.to_string();
    }

    if let Ok(cache) = RESOLVED_BROWSER_TITLES.lock() {
        if let Some(full_stripped) = cache.get(&search_prefix) {
            return format!("{full_stripped}{browser_suffix}");
        }
    }

    if let Some(full_stripped) = lookup_full_title_in_browser_storage(&search_prefix) {
        if let Ok(mut cache) = RESOLVED_BROWSER_TITLES.lock() {
            cache.insert(search_prefix, full_stripped.clone());
        }
        if full_stripped.to_ascii_lowercase().contains("gmail") {
            if let Ok(mut last_map) = LAST_BROWSER_SITE_SUFFIX.lock() {
                last_map.insert(lower_class, " - Gmail".to_string());
            }
        }
        return format!("{full_stripped}{browser_suffix}");
    }

    // Fallback for brand-new long email clicked while already inside Gmail before SQLite WAL flushes to disk:
    // Gecko truncates long `<title>` strings at ~254-256 UTF-8 bytes.
    if stripped.len() >= 230 {
        if let Ok(last_map) = LAST_BROWSER_SITE_SUFFIX.lock() {
            if let Some(site_suf) = last_map.get(&lower_class) {
                let augmented = format!("{stripped} …{site_suf}");
                if let Ok(mut cache) = RESOLVED_BROWSER_TITLES.lock() {
                    cache.insert(search_prefix, augmented.clone());
                }
                return format!("{augmented}{browser_suffix}");
            }
        }
    }

    caption.to_string()
}

fn site_icon_cache_dir() -> PathBuf {
    let base = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".cache")
        });
    let dir = base.join("duckydeck-studio").join("site-icons");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn write_cached_site_svg(slug: &str, svg_content: &str) -> String {
    let dir = site_icon_cache_dir();
    let path = dir.join(format!("{slug}.svg"));
    let needs_write = match fs::read_to_string(&path) {
        Ok(existing) => existing != svg_content,
        Err(_) => true,
    };
    if needs_write {
        let _ = fs::write(&path, svg_content);
    }
    format!("file://{}", path.to_string_lossy())
}

fn known_site_svg(query_raw: &str) -> Option<(&'static str, String)> {
    let cleaned = strip_browser_title_suffix(query_raw);
    let q = cleaned.trim().to_ascii_lowercase();
    if q.is_empty() {
        return None;
    }

    let has = |kw: &str| -> bool { q.contains(kw) };

    if has("youtube") || has("youtu.be") {
        return Some((
            "youtube",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="12" width="56" height="40" rx="12" fill="#FF0000"/>
  <polygon points="26,21 26,43 45,32" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("gmail") || has("mail.google") || has("google mail") {
        return Some((
            "gmail",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="10" width="56" height="44" rx="8" fill="#FFFFFF" stroke="#E0E0E0" stroke-width="2"/>
  <path d="M8 16 L8 50 L18 50 L18 28 L32 39 L46 28 L46 50 L56 50 L56 16 L46 16 L32 27 L18 16 Z" fill="#EA4335"/>
  <path d="M8 16 L18 24 L18 50 L10 50 C8.9 50 8 49.1 8 48 Z" fill="#4285F4"/>
  <path d="M56 16 L46 24 L46 50 L54 50 C55.1 50 56 49.1 56 48 Z" fill="#34A853"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("twitch") {
        return Some((
            "twitch",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <path d="M12 8 L8 18 L8 50 L20 50 L20 58 L28 58 L36 50 L46 50 L56 40 L56 8 Z" fill="#9146FF"/>
  <path d="M18 14 L50 14 L50 37 L43 44 L33 44 L26 51 L26 44 L18 44 Z" fill="#FFFFFF"/>
  <rect x="27" y="21" width="5" height="14" fill="#9146FF"/>
  <rect x="38" y="21" width="5" height="14" fill="#9146FF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("github") {
        return Some((
            "github",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#24292F"/>
  <path d="M32 12 C20.95 12 12 20.95 12 32 C12 40.84 17.73 48.33 25.68 50.98 C26.68 51.16 27.05 50.55 27.05 50.02 C27.05 49.55 27.03 48.0 27.02 46.2 C21.45 47.41 20.28 43.8 20.28 43.8 C19.37 41.49 18.05 40.88 18.05 40.88 C16.23 39.64 18.19 39.66 18.19 39.66 C20.2 39.8 21.26 41.73 21.26 41.73 C23.05 44.79 25.95 43.91 27.1 43.4 C27.28 42.1 27.8 41.22 28.37 40.72 C23.93 40.22 19.26 38.5 19.26 30.83 C19.26 28.64 20.04 26.86 21.32 25.46 C21.11 24.96 20.43 22.92 21.52 20.16 C21.52 20.16 23.2 19.62 27.02 22.21 C28.62 21.77 30.32 21.55 32.02 21.54 C33.72 21.55 35.42 21.77 37.02 22.21 C40.84 19.62 42.52 20.16 42.52 20.16 C43.61 22.92 42.93 24.96 42.72 25.46 C44.0 26.86 44.78 28.64 44.78 30.83 C44.78 38.52 40.1 40.21 35.64 40.7 C36.36 41.32 37.0 42.54 37.0 44.41 C37.0 47.09 36.98 49.25 36.98 50.02 C36.98 50.55 37.35 51.17 38.36 50.98 C46.3 48.32 52 40.84 52 32 C52 20.95 43.05 12 32 12 Z" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("gitlab") {
        return Some((
            "gitlab",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#292961"/>
  <polygon points="32,52 44,24 20,24" fill="#E24329"/>
  <polygon points="32,52 20,24 10,24" fill="#FC6D26"/>
  <polygon points="10,24 15,10 20,24" fill="#FCA326"/>
  <polygon points="32,52 44,24 54,24" fill="#FC6D26"/>
  <polygon points="54,24 49,10 44,24" fill="#FCA326"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("reddit") {
        return Some((
            "reddit",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#FF4500"/>
  <ellipse cx="32" cy="37" rx="16" ry="11" fill="#FFFFFF"/>
  <circle cx="17" cy="31" r="4.5" fill="#FFFFFF"/>
  <circle cx="47" cy="31" r="4.5" fill="#FFFFFF"/>
  <circle cx="26" cy="35" r="3" fill="#FF4500"/>
  <circle cx="38" cy="35" r="3" fill="#FF4500"/>
  <path d="M25 42 Q32 46 39 42" stroke="#FF4500" stroke-width="2.5" fill="none" stroke-linecap="round"/>
  <path d="M32 26 L36 16 L44 18" stroke="#FFFFFF" stroke-width="2.5" fill="none" stroke-linecap="round"/>
  <circle cx="45" cy="18" r="3.5" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("netflix") {
        return Some((
            "netflix",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="6" y="6" width="52" height="52" rx="12" fill="#141414"/>
  <rect x="18" y="13" width="8" height="38" fill="#B1060F"/>
  <rect x="38" y="13" width="8" height="38" fill="#B1060F"/>
  <polygon points="18,13 26,13 46,51 38,51" fill="#E50914"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("spotify") {
        return Some((
            "spotify",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#1DB954"/>
  <path d="M18 25 Q32 20 46 26" stroke="#121212" stroke-width="4.5" fill="none" stroke-linecap="round"/>
  <path d="M20 34 Q32 30 43 35" stroke="#121212" stroke-width="4" fill="none" stroke-linecap="round"/>
  <path d="M22 42 Q32 39 40 43" stroke="#121212" stroke-width="3.5" fill="none" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("discord") {
        return Some((
            "discord",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="8" width="56" height="48" rx="14" fill="#5865F2"/>
  <path d="M22 20 C26 18 38 18 42 20 C47 26 49 34 47 42 C43 45 39 46 37 44 L35 41 C29 41 29 41 27 44 C25 46 21 45 17 42 C15 34 17 26 22 20 Z" fill="#FFFFFF"/>
  <circle cx="26" cy="32" r="3.5" fill="#5865F2"/>
  <circle cx="38" cy="32" r="3.5" fill="#5865F2"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("slack") {
        return Some((
            "slack",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#4A154B"/>
  <rect x="14" y="25" width="15" height="6" rx="3" fill="#36C5F0"/>
  <rect x="25" y="14" width="6" height="15" rx="3" fill="#2EB67D"/>
  <rect x="35" y="33" width="15" height="6" rx="3" fill="#ECB22E"/>
  <rect x="33" y="35" width="6" height="15" rx="3" fill="#E01E5A"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("chatgpt") || has("openai") || has("chat.com") {
        return Some((
            "chatgpt",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#10A37F"/>
  <circle cx="32" cy="32" r="14" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <circle cx="32" cy="22" r="8" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <circle cx="23" cy="37" r="8" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <circle cx="41" cy="37" r="8" fill="none" stroke="#FFFFFF" stroke-width="3"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("claude") || has("anthropic") {
        return Some((
            "claude",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#D97757"/>
  <g stroke="#FFF7F0" stroke-width="4" stroke-linecap="round">
    <line x1="32" y1="12" x2="32" y2="52"/>
    <line x1="12" y1="32" x2="52" y2="32"/>
    <line x1="18" y1="18" x2="46" y2="46"/>
    <line x1="46" y1="18" x2="18" y2="46"/>
  </g>
</svg>"##
                .to_string(),
        ));
    }
    if has("gemini") || has("aistudio.google") {
        return Some((
            "gemini",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#1A1F35"/>
  <path d="M32 10 Q34 30 54 32 Q34 34 32 54 Q30 34 10 32 Q30 30 32 10 Z" fill="#669DF6"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google docs") || has("docs.google") {
        return Some((
            "google-docs",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="12" y="6" width="40" height="52" rx="6" fill="#4285F4"/>
  <rect x="20" y="22" width="24" height="4" rx="2" fill="#FFFFFF"/>
  <rect x="20" y="31" width="24" height="4" rx="2" fill="#FFFFFF"/>
  <rect x="20" y="40" width="16" height="4" rx="2" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google sheets") || has("sheets.google") {
        return Some((
            "google-sheets",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="12" y="6" width="40" height="52" rx="6" fill="#0F9D58"/>
  <rect x="19" y="22" width="26" height="22" rx="2" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="19" y1="33" x2="45" y2="33" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="30" y1="22" x2="30" y2="44" stroke="#FFFFFF" stroke-width="3"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google slides") || has("slides.google") {
        return Some((
            "google-slides",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="12" y="6" width="40" height="52" rx="6" fill="#F4B400"/>
  <rect x="19" y="23" width="26" height="18" rx="2" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google drive") || has("drive.google") {
        return Some((
            "google-drive",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <polygon points="24,10 40,10 56,38 40,38" fill="#FBBC04"/>
  <polygon points="8,38 24,10 32,24 16,52" fill="#0F9D58"/>
  <polygon points="16,52 50,52 56,38 24,38" fill="#4285F4"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google calendar") || has("calendar.google") {
        return Some((
            "google-calendar",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="8" y="8" width="48" height="48" rx="10" fill="#4285F4"/>
  <rect x="16" y="20" width="32" height="28" rx="4" fill="#FFFFFF"/>
  <text x="32" y="41" font-family="sans-serif" font-size="18" font-weight="bold" text-anchor="middle" fill="#1A73E8">31</text>
</svg>"##
                .to_string(),
        ));
    }
    if has("google meet") || has("meet.google") {
        return Some((
            "google-meet",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="6" y="14" width="36" height="36" rx="8" fill="#00897B"/>
  <polygon points="42,24 58,14 58,50 42,40" fill="#34A853"/>
  <rect x="14" y="22" width="20" height="20" rx="4" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google maps") || has("maps.google") {
        return Some((
            "google-maps",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <path d="M32 6 C20 6 12 15 12 26 C12 41 32 58 32 58 C32 58 52 41 52 26 C52 15 44 6 32 6 Z" fill="#EA4335"/>
  <circle cx="32" cy="26" r="8" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("printables") || has("prusa") {
        return Some((
            "printables",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#FA6831"/>
  <polygon points="32,12 50,22 50,42 32,52 14,42 14,22" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <polyline points="14,22 32,32 50,22" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <line x1="32" y1="32" x2="32" y2="52" stroke="#FFFFFF" stroke-width="4"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("makerworld") || has("bambu") {
        return Some((
            "makerworld",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#00AE42"/>
  <path d="M16 46 L16 18 L32 32 L48 18 L48 46" fill="none" stroke="#FFFFFF" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("thingiverse") {
        return Some((
            "thingiverse",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#248BFB"/>
  <circle cx="32" cy="32" r="21" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <path d="M21 22 L43 22 M32 22 L32 45" stroke="#FFFFFF" stroke-width="6" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("onshape") || has("tinkercad") {
        return Some((
            "onshape",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#1B5AA0"/>
  <polygon points="32,12 50,22 50,42 32,52 14,42 14,22" fill="#4CAF50" stroke="#FFFFFF" stroke-width="3"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("octoprint") || has("mainsail") || has("fluidd") || has("klipper") {
        return Some((
            "klipper-web",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#D32F2F"/>
  <polygon points="22,14 42,14 42,30 35,40 29,40 22,30" fill="#FFFFFF"/>
  <rect x="16" y="46" width="32" height="5" rx="2.5" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("home assistant") || has("homeassistant") {
        return Some((
            "home-assistant",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#18BCF2"/>
  <polygon points="32,12 10,32 18,32 18,52 46,52 46,32 54,32" fill="#FFFFFF"/>
  <circle cx="32" cy="36" r="5" fill="#18BCF2"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("proxmox") {
        return Some((
            "proxmox",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#222222"/>
  <path d="M16 16 L32 32 L16 48 M48 16 L32 32 L48 48" stroke="#E57000" stroke-width="6" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("plex") {
        return Some((
            "plex",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#1F2326"/>
  <polygon points="22,14 34,14 46,32 34,50 22,50 34,32" fill="#E5A00D"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("jellyfin") {
        return Some((
            "jellyfin",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#101828"/>
  <path d="M32 12 L50 48 L14 48 Z" fill="none" stroke="#AA5CC3" stroke-width="5" stroke-linejoin="round"/>
  <path d="M32 25 L40 41 L24 41 Z" fill="#00A4DC"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("figma") {
        return Some((
            "figma",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#1E1E1E"/>
  <circle cx="26" cy="18" r="7" fill="#F24E1E"/>
  <circle cx="38" cy="18" r="7" fill="#FF7262"/>
  <circle cx="26" cy="32" r="7" fill="#A259FF"/>
  <circle cx="38" cy="32" r="7" fill="#1ABCFE"/>
  <circle cx="26" cy="46" r="7" fill="#0ACF83"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("notion") {
        return Some((
            "notion",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="6" y="6" width="52" height="52" rx="10" fill="#FFFFFF" stroke="#191919" stroke-width="4"/>
  <path d="M22 44 L22 20 L42 44 L42 20" fill="none" stroke="#191919" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("stack overflow") || has("stackoverflow") {
        return Some((
            "stackoverflow",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#222426"/>
  <path d="M18 38 L18 50 L46 50 L46 38" fill="none" stroke="#BCBBBB" stroke-width="4"/>
  <rect x="23" y="41" width="18" height="4" fill="#F48024"/>
  <rect x="24" y="33" width="18" height="4" transform="rotate(10 33 35)" fill="#F48024"/>
  <rect x="26" y="25" width="18" height="4" transform="rotate(24 35 27)" fill="#F48024"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("wikipedia") {
        return Some((
            "wikipedia",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#F8F9FA" stroke="#54595D" stroke-width="3"/>
  <text x="32" y="42" font-family="serif" font-size="30" font-weight="bold" text-anchor="middle" fill="#202122">W</text>
</svg>"##
                .to_string(),
        ));
    }
    if has("amazon") {
        return Some((
            "amazon",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#232F3E"/>
  <text x="32" y="38" font-family="sans-serif" font-size="30" font-weight="bold" text-anchor="middle" fill="#FFFFFF">a</text>
  <path d="M16 45 Q32 54 48 44" stroke="#FF9900" stroke-width="4" fill="none" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("twitter") || has("x.com") || q == "x" {
        return Some((
            "x-twitter",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0F1419"/>
  <path d="M18 16 L28 16 L46 48 L36 48 Z M45 16 L19 48" stroke="#FFFFFF" stroke-width="4" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("bluesky") || has("bsky") {
        return Some((
            "bluesky",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0285FF"/>
  <path d="M32 28 C25 16 12 12 12 24 C12 34 24 36 28 36 C20 38 14 44 20 50 C25 54 30 46 32 40 C34 46 39 54 44 50 C50 44 44 38 36 36 C40 36 52 34 52 24 C52 12 39 16 32 28 Z" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("linkedin") {
        return Some((
            "linkedin",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0A66C2"/>
  <text x="32" y="44" font-family="sans-serif" font-size="32" font-weight="bold" text-anchor="middle" fill="#FFFFFF">in</text>
</svg>"##
                .to_string(),
        ));
    }
    if has("whatsapp") {
        return Some((
            "whatsapp",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#25D366"/>
  <circle cx="32" cy="32" r="18" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <path d="M25 26 C25 34 30 39 38 39" stroke="#FFFFFF" stroke-width="5" fill="none" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("telegram") {
        return Some((
            "telegram",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#26A5E4"/>
  <polygon points="14,31 48,17 41,47 30,38 24,43 26,35" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("linear") {
        return Some((
            "linear",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#5E6AD2"/>
  <circle cx="32" cy="32" r="18" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <line x1="20" y1="44" x2="44" y2="20" stroke="#FFFFFF" stroke-width="4"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("jira") || has("atlassian") {
        return Some((
            "jira",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#0052CC"/>
  <polygon points="32,14 48,30 32,46 24,38 32,30 24,22" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("trello") {
        return Some((
            "trello",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0079BF"/>
  <rect x="14" y="14" width="14" height="34" rx="4" fill="#FFFFFF"/>
  <rect x="36" y="14" width="14" height="22" rx="4" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("soundcloud") {
        return Some((
            "soundcloud",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#FF5500"/>
  <path d="M16 42 L48 42 C53 42 54 33 48 32 C47 23 35 21 32 29 C28 28 24 31 24 35 L16 35 Z" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }

    None
}

fn generate_custom_site_badge_svg(site_hint: &str) -> (String, String) {
    let cleaned = strip_browser_title_suffix(site_hint);
    let candidate = cleaned
        .rsplit(['-', '—', '|', ':'])
        .map(|s| s.trim())
        .find(|s| !s.is_empty())
        .unwrap_or_else(|| cleaned.trim());

    let mut hash: u32 = 2166136261;
    for b in candidate.to_ascii_lowercase().bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }

    let palette = [
        "#2563EB", "#0891B2", "#059669", "#7C3AED", "#DB2777", "#EA580C", "#DC2626", "#4F46E5",
    ];
    let bg = palette[(hash as usize) % palette.len()];

    let monogram: String = candidate
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase();
    let label = if monogram.is_empty() {
        "W".to_string()
    } else {
        monogram
    };

    let slug = format!("site_{hash:08x}");
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="{bg}"/>
  <circle cx="32" cy="32" r="22" fill="none" stroke="#FFFFFF" stroke-opacity="0.22" stroke-width="2"/>
  <ellipse cx="32" cy="32" rx="11" ry="22" fill="none" stroke="#FFFFFF" stroke-opacity="0.22" stroke-width="2"/>
  <line x1="10" y1="32" x2="54" y2="32" stroke="#FFFFFF" stroke-opacity="0.22" stroke-width="2"/>
  <text x="32" y="40" font-family="sans-serif" font-size="23" font-weight="bold" text-anchor="middle" fill="#FFFFFF">{label}</text>
</svg>"##
    );
    (slug, svg)
}

fn resolve_favicon_for_url(raw_url: &str) -> Option<String> {
    let trimmed = raw_url.trim();
    if trimmed.is_empty() {
        return None;
    }

    let dir = site_icon_cache_dir();
    let mut hash: u32 = 2166136261;
    for b in trimmed.to_ascii_lowercase().bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }
    let stem = format!("fav_{hash:08x}");

    for ext in ["png", "ico", "svg", "jpg", "webp"] {
        let existing = dir.join(format!("{stem}.{ext}"));
        if existing.is_file() {
            return Some(format!("file://{}", existing.to_string_lossy()));
        }
    }

    let with_scheme = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    };

    let (scheme, rest) = with_scheme
        .split_once("://")
        .unwrap_or(("https", trimmed));
    let host_port = rest.split('/').next().unwrap_or(rest).trim();
    let host_only = host_port.split(':').next().unwrap_or(host_port).to_ascii_lowercase();

    let mut candidates = Vec::new();
    let lower_url = with_scheme.to_ascii_lowercase();
    if lower_url.ends_with(".ico")
        || lower_url.ends_with(".png")
        || lower_url.ends_with(".svg")
        || lower_url.ends_with(".jpg")
        || lower_url.ends_with(".jpeg")
        || lower_url.ends_with(".webp")
    {
        candidates.push(with_scheme.clone());
    }

    let is_local_or_lan = host_only == "localhost"
        || host_only.ends_with(".local")
        || host_only.ends_with(".lan")
        || host_only.starts_with("127.")
        || host_only.starts_with("10.")
        || host_only.starts_with("192.168.")
        || host_only.starts_with("172.");

    if !host_only.is_empty() {
        if !is_local_or_lan && host_only.contains('.') {
            candidates.push(format!(
                "https://www.google.com/s2/favicons?domain={host_only}&sz=64"
            ));
        }
        candidates.push(format!("{scheme}://{host_port}/favicon.ico"));
        candidates.push(format!("{scheme}://{host_port}/favicon.png"));
    }

    let failed_marker = dir.join(format!("{stem}.failed"));
    if !failed_marker.is_file() && !candidates.is_empty() {
        if let Ok(rt) = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            let fetched_path: Option<PathBuf> = rt.block_on(async {
                let client = reqwest::Client::builder()
                    .connect_timeout(std::time::Duration::from_millis(1200))
                    .timeout(std::time::Duration::from_millis(2200))
                    .user_agent("DuckyDeck-Studio/1.0")
                    .build()
                    .ok()?;
                for cand in &candidates {
                    let Ok(resp) = client.get(cand).send().await else {
                        continue;
                    };
                    if !resp.status().is_success() {
                        continue;
                    }
                    let ctype = resp
                        .headers()
                        .get(reqwest::header::CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("")
                        .to_ascii_lowercase();
                    if ctype.contains("text/html") {
                        continue;
                    }
                    let Ok(bytes) = resp.bytes().await else {
                        continue;
                    };
                    if bytes.len() < 24 || bytes.len() > 2_000_000 {
                        continue;
                    }
                    let prefix_ascii =
                        String::from_utf8_lossy(&bytes[..bytes.len().min(64)]).to_ascii_lowercase();
                    if prefix_ascii.contains("<!doctype") || prefix_ascii.contains("<html") {
                        continue;
                    }
                    let ext = if ctype.contains("svg") || prefix_ascii.contains("<svg") {
                        "svg"
                    } else if bytes.starts_with(b"\x89PNG") || ctype.contains("png") {
                        "png"
                    } else if bytes.starts_with(&[0x00, 0x00, 0x01, 0x00]) || ctype.contains("icon") {
                        "ico"
                    } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) || ctype.contains("jpeg") {
                        "jpg"
                    } else {
                        "png"
                    };
                    let out_path = dir.join(format!("{stem}.{ext}"));
                    if fs::write(&out_path, &bytes).is_ok() {
                        return Some(out_path);
                    }
                }
                None
            });
            if let Some(out_path) = fetched_path {
                return Some(format!("file://{}", out_path.to_string_lossy()));
            }
            let _ = fs::write(&failed_marker, b"1");
        }
    }

    if let Some((slug, svg)) = known_site_svg(trimmed) {
        return Some(write_cached_site_svg(slug, &svg));
    }
    let (slug, svg) = generate_custom_site_badge_svg(if !host_only.is_empty() {
        &host_only
    } else {
        trimmed
    });
    Some(write_cached_site_svg(&slug, &svg))
}

pub fn resolve_rule_icon(
    resource_class: &str,
    title_contains: Option<&str>,
    favicon_url: Option<&str>,
    target_profile_name: Option<&str>,
) -> String {
    if let Some(raw_fav) = favicon_url {
        let fav_trimmed = raw_fav.trim();
        if !fav_trimmed.is_empty() {
            if let Some(icon_uri) = resolve_favicon_for_url(fav_trimmed) {
                return icon_uri;
            }
        }
    }

    let trimmed_class = resource_class.trim();
    if is_web_browser_class(trimmed_class) {
        let title_str = title_contains.unwrap_or("").trim();
        let cleaned_title = strip_browser_title_suffix(title_str);

        // 1. If the browser rule specifies a title substring, always resolve a site icon!
        if !cleaned_title.is_empty() {
            if let Some((slug, svg)) = known_site_svg(&cleaned_title) {
                return write_cached_site_svg(slug, &svg);
            }
            if cleaned_title.contains('.') && !cleaned_title.contains(' ') {
                if let Some(uri) = resolve_favicon_for_url(&cleaned_title) {
                    return uri;
                }
            }
            let (slug, svg) = generate_custom_site_badge_svg(&cleaned_title);
            return write_cached_site_svg(&slug, &svg);
        }

        // 2. If title_contains is empty, check if the target profile name matches a known site
        if let Some(prof_name) = target_profile_name {
            let p_trim = prof_name.trim();
            if !p_trim.is_empty() && !is_web_browser_class(p_trim) {
                if let Some((slug, svg)) = known_site_svg(p_trim) {
                    return write_cached_site_svg(slug, &svg);
                }
            }
        }

        // 3. If the rule matches all browsers (*browser*) and has no site title, show a multi-browser globe icon
        if is_any_browser_wildcard(trimmed_class) {
            let all_browsers_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#0284C7"/>
  <circle cx="32" cy="32" r="21" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <ellipse cx="32" cy="32" rx="11" ry="21" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="11" y1="32" x2="53" y2="32" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="15" y1="21" x2="49" y2="21" stroke="#FFFFFF" stroke-width="2.5"/>
  <line x1="15" y1="43" x2="49" y2="43" stroke="#FFFFFF" stroke-width="2.5"/>
</svg>"##;
            return write_cached_site_svg("all-browsers", all_browsers_svg);
        }
    }

    resolve_app_icon_for_class(trimmed_class)
}

pub fn resolve_app_icon_for_class(resource_class: &str) -> String {
    let trimmed = resource_class.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if is_any_browser_wildcard(trimmed) {
        return resolve_rule_icon(trimmed, None, None, None);
    }
    let lower = trimmed.to_ascii_lowercase();
    let short_lower = lower
        .rsplit('.')
        .next()
        .unwrap_or(lower.as_str())
        .to_string();

    let mut search_dirs = Vec::new();
    if let Ok(home) = std::env::var("HOME") {
        search_dirs.push(PathBuf::from(&home).join(".local/share/applications"));
        search_dirs
            .push(PathBuf::from(&home).join(".local/share/flatpak/exports/share/applications"));
    }
    if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
        for part in xdg_data.split(':') {
            let p = part.trim();
            if !p.is_empty() && p != "." {
                let candidate = PathBuf::from(p).join("applications");
                if !search_dirs.contains(&candidate) {
                    search_dirs.push(candidate);
                }
            }
        }
    }
    search_dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
    search_dirs.push(PathBuf::from("/usr/local/share/applications"));
    search_dirs.push(PathBuf::from("/usr/share/applications"));

    fn extract_icon_from_desktop(content: &str) -> Option<String> {
        let mut in_entry = false;
        for line in content.lines() {
            let l = line.trim();
            if l.starts_with('[') {
                in_entry = l == "[Desktop Entry]";
                continue;
            }
            if in_entry {
                if let Some(icon) = l.strip_prefix("Icon=") {
                    let val = icon.trim();
                    if !val.is_empty() {
                        return Some(val.to_string());
                    }
                }
            }
        }
        None
    }

    // 1. Direct desktop filename match
    for dir in &search_dirs {
        for cand in [
            format!("{trimmed}.desktop"),
            format!("{lower}.desktop"),
            format!("org.kde.{short_lower}.desktop"),
        ] {
            let p = dir.join(&cand);
            if p.is_file() {
                if let Ok(content) = fs::read_to_string(&p) {
                    if let Some(icon) = extract_icon_from_desktop(&content) {
                        return icon;
                    }
                }
            }
        }
    }

    // 2. Scan desktop files for StartupWMClass, filename suffix, or Exec binary match
    for dir in &search_dirs {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if !fname.ends_with(".desktop") {
                continue;
            };
            let stem_lower = fname.trim_end_matches(".desktop").to_ascii_lowercase();
            let stem_matches = stem_lower == lower
                || stem_lower == short_lower
                || stem_lower.ends_with(&format!(".{short_lower}"));

            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };

            if stem_matches {
                if let Some(icon) = extract_icon_from_desktop(&content) {
                    return icon;
                }
            }

            let mut wm_match = false;
            let mut exec_match = false;
            let mut in_entry = false;
            for line in content.lines() {
                let l = line.trim();
                if l.starts_with('[') {
                    in_entry = l == "[Desktop Entry]";
                    continue;
                }
                if !in_entry {
                    continue;
                }
                if let Some(wm) = l.strip_prefix("StartupWMClass=") {
                    let wm_l = wm.trim().to_ascii_lowercase();
                    if wm_l == lower || wm_l == short_lower {
                        wm_match = true;
                        break;
                    }
                } else if let Some(exec) = l.strip_prefix("Exec=") {
                    let first_tok = exec
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .trim_matches('"');
                    let bin = std::path::Path::new(first_tok)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_ascii_lowercase();
                    if !bin.is_empty() && (bin == lower || bin == short_lower) {
                        exec_match = true;
                    }
                }
            }

            if wm_match || exec_match {
                if let Some(icon) = extract_icon_from_desktop(&content) {
                    return icon;
                }
            }
        }
    }

    // 3. Fallback common icon names
    match short_lower.as_str() {
        "dolphin" => "system-file-manager".to_string(),
        "konsole" | "alacritty" | "kitty" | "wezterm" => "utilities-terminal".to_string(),
        "code" | "code-oss" | "codium" | "vscodium" => "com.visualstudio.code".to_string(),
        "firefox" | "navigator" => "firefox".to_string(),
        "chrome" | "google-chrome" => "google-chrome".to_string(),
        "chromium" => "chromium".to_string(),
        _ => short_lower,
    }
}

pub fn autostart_desktop_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".config")
        });
    base.join("autostart")
        .join("io.github.duckydeck.Studio.desktop")
}

pub fn is_kde_autostart_enabled() -> bool {
    let path = autostart_desktop_path();
    let Ok(content) = fs::read_to_string(&path) else {
        return false;
    };
    for line in content.lines() {
        let t = line.trim();
        if t.eq_ignore_ascii_case("Hidden=true")
            || t.eq_ignore_ascii_case("X-GNOME-Autostart-enabled=false")
        {
            return false;
        }
    }
    true
}

pub fn sync_local_desktop_and_icon(use_light_on_dark_icon: bool) {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let fallback_data_home = PathBuf::from(&home).join(".local").join("share");
    let data_home = match std::env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let trimmed = val.trim();
            if trimmed.is_empty()
                || trimmed.contains(':')
                || !trimmed.starts_with('/')
                || trimmed.starts_with("/var/")
                || trimmed.starts_with("/usr/")
            {
                fallback_data_home
            } else {
                PathBuf::from(trimmed)
            }
        }
        Err(_) => fallback_data_home,
    };

    let icon_dir = data_home
        .join("icons")
        .join("hicolor")
        .join("scalable")
        .join("apps");
    if fs::create_dir_all(&icon_dir).is_ok() {
        let svg_content = if use_light_on_dark_icon {
            include_str!("../qml/icons/tray-icon-dark.svg")
        } else {
            include_str!("../qml/icons/tray-icon-light.svg")
        };
        let _ = fs::write(icon_dir.join("io.github.duckydeck.Studio.svg"), svg_content);
    }

    let apps_dir = data_home.join("applications");
    if fs::create_dir_all(&apps_dir).is_ok() {
        let exe_path = {
            let home = std::env::var("HOME").unwrap_or_default();
            let local_bin = PathBuf::from(&home).join(".local/bin/duckydeck-studio");
            if !home.is_empty() && local_bin.exists() {
                local_bin.to_string_lossy().to_string()
            } else {
                std::env::current_exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "duckydeck-studio".to_string())
            }
        };
        let desktop_content = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=DuckyDeck Studio\n\
             Comment=KDE Plasma 6 Configurator & Auto-Switcher for duckyPad Pro\n\
             Exec=\"{exe_path}\"\n\
             Icon=io.github.duckydeck.Studio\n\
             Terminal=false\n\
             Categories=Utility;HardwareSettings;Qt;KDE;\n\
             StartupWMClass=io.github.duckydeck.Studio\n"
        );
        let _ = fs::write(
            apps_dir.join("io.github.duckydeck.Studio.desktop"),
            desktop_content,
        );
    }
}

pub fn set_kde_autostart_enabled(enabled: bool) -> Result<PathBuf, String> {
    let path = autostart_desktop_path();
    if enabled {
        sync_local_desktop_and_icon(true);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed creating autostart directory: {e}"))?;
        }
        let exe_path = {
            let home = std::env::var("HOME").unwrap_or_default();
            let local_bin = PathBuf::from(&home).join(".local/bin/duckydeck-studio");
            if !home.is_empty() && local_bin.exists() {
                local_bin.to_string_lossy().to_string()
            } else {
                std::env::current_exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "duckydeck-studio".to_string())
            }
        };
        let desktop_entry = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=DuckyDeck Studio\n\
             Comment=KDE Plasma 6 Configurator & Auto-Switcher for duckyPad Pro\n\
             Exec=\"{exe_path}\" --autostart --minimized --foreground\n\
             Icon=io.github.duckydeck.Studio\n\
             Terminal=false\n\
             Categories=Utility;HardwareSettings;Qt;KDE;\n\
             StartupWMClass=io.github.duckydeck.Studio\n\
             X-GNOME-Autostart-enabled=true\n\
             X-KDE-StartupNotify=false\n\
             X-KDE-AutostartScript=false\n\
             X-KDE-autostart-after=panel\n"
        );
        fs::write(&path, desktop_entry)
            .map_err(|e| format!("Failed writing {}: {e}", path.display()))?;
        Ok(path)
    } else {
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed removing {}: {e}", path.display()))?;
        }
        Ok(path)
    }
}

impl SwitcherState {

    /// When SD profiles are loaded for the first time and the user hasn't saved custom rules yet,
    /// automatically align default KWin rules with matching profile names on their duckyPad.
    pub fn align_rules_with_profiles(&mut self, profiles: &[DuckyProfile]) {
        if self.loaded_from_disk || profiles.len() <= 1 {
            return;
        }

        let known_mappings: &[(&str, &str, Option<&str>)] = &[
            ("Firefox", "firefox", None),
            ("Chrome", "chrome", None),
            ("Dolphin", "org.kde.dolphin", None),
            ("Yakuake", "org.kde.yakuake", None),
            ("VS Code", "code", None),
            ("OBS", "com.obsproject.Studio", None),
            ("Discord", "discord", None),
            ("Slack", "Slack", None),
            ("Zoom", "zoom", None),
            ("OrcaSlicer", "OrcaSlicer", None),
            ("UpNote", "upnote", None),
            ("YouTube", ANY_BROWSER_CLASS, Some("YouTube")),
            ("Gmail", ANY_BROWSER_CLASS, Some("Gmail")),
        ];

        let mut aligned = Vec::new();
        for &(prof_name_pat, class_name, title_sub) in known_mappings {
            if let Some(prof) = profiles
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(prof_name_pat))
            {
                aligned.push(AutoSwitchRule {
                    enabled: true,
                    resource_class: class_name.to_string(),
                    title_contains: title_sub.map(|s| s.to_string()),
                    favicon_url: None,
                    target_profile: prof.number,
                });
            }
        }

        if !aligned.is_empty() {
            self.rules = aligned;
            let _ = self.save_to_disk();
        }
    }

    pub fn match_profile_for_window(&self, resource_class: &str, caption: &str) -> Option<u8> {
        let lower_class = resource_class.trim().to_lowercase();
        if lower_class.is_empty() || lower_class == "none" {
            return None;
        }
        let resolved_caption = resolve_full_browser_caption(resource_class, caption);
        let caption_lower = resolved_caption.to_lowercase();

        let mut best_match: Option<(usize, u8)> = None;

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            if !rule_matches_window_class(&rule.resource_class, &lower_class) {
                continue;
            }

            let title_specificity = match rule.title_contains.as_ref().map(|s| s.trim()) {
                Some(sub) if !sub.is_empty() => {
                    if caption_lower.contains(&sub.to_lowercase()) {
                        sub.len()
                    } else {
                        continue;
                    }
                }
                _ => 0,
            };

            if best_match.is_none_or(|(best_len, _)| title_specificity > best_len) {
                best_match = Some((title_specificity, rule.target_profile));
            }
        }

        if let Some((_, target)) = best_match {
            return Some(target);
        }

        self.default_profile
    }
}

pub struct KWinBridgeService {
    state: Arc<Mutex<SwitcherState>>,
    ui_event_tx: mpsc::UnboundedSender<WindowChangeEvent>,
}

#[interface(name = "io.github.duckydeck.KWinBridge")]
impl KWinBridgeService {
    fn notify_active_window(
        &mut self,
        resource_class: String,
        resource_name: String,
        caption: String,
    ) {
        let lower_class = resource_class.to_lowercase();
        if resource_class.is_empty()
            || lower_class == "plasmashell"
            || lower_class.contains("duckydeck")
        {
            return;
        }

        let resolved_caption = resolve_full_browser_caption(&resource_class, &caption);
        let mut matched_profile: Option<u8> = None;

        if let Ok(mut state) = self.state.lock() {
            if state.picker_armed {
                state.picker_armed = false;
                let _ = self.ui_event_tx.send(WindowChangeEvent {
                    resource_class,
                    resource_name,
                    caption: resolved_caption,
                    matched_profile: None,
                    picked: true,
                });
                return;
            }

            if state.enabled && !state.paused_for_editing {
                matched_profile =
                    state.match_profile_for_window(&resource_class, &resolved_caption);

                if let Some(target) = matched_profile {
                    if send_hid_switch_profile(target) {
                        state.last_switched_profile = target;
                    }
                }
            }
        }

        let _ = self.ui_event_tx.send(WindowChangeEvent {
            resource_class,
            resource_name,
            caption: resolved_caption,
            matched_profile,
            picked: false,
        });
    }

    fn notify_main_window_minimized(&mut self) {
        crate::bridge::device_controller::qobject::duckydeck_hide_main_window();
    }

    fn notify_oled_widget_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        crate::bridge::device_controller::qobject::duckydeck_record_oled_geometry(
            x, y, width, height,
        );
    }

    fn notify_oled_widget_mapped(&mut self, is_pinned: bool) {
        apply_kwin_oled_keep_above(is_pinned, true);
    }
}

pub fn send_hid_switch_profile(profile_number: u8) -> bool {
    if crate::hardware::storage::MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    let _guard = crate::hardware::storage::HID_IO_LOCK.lock().ok();
    let Ok(api) = HidApi::new() else {
        return false;
    };

    let clamped = profile_number.clamp(1, 64);

    for pid in [PID_DUCKYPAD_PRO, PID_DUCKYPAD_2020] {
        if let Ok(device) = api.open(DUCKYPAD_VID, pid) {
            // Query current hardware profile first so we don't re-flash the OLED if already on `clamped`,
            // while still switching if the user manually changed profiles on the physical device.
            let mut qtx = [0u8; 64];
            qtx[0] = HID_REPORT_ID_OUT;
            qtx[1] = 0x76;
            qtx[2] = HID_CMD_QUERY_INFO;
            if device.write(&qtx).is_ok() {
                let mut qrx = [0u8; 64];
                if let Ok(n) = device.read_timeout(&mut qrx, 200) {
                    if n >= 12 && qrx[2] == 0 && qrx[6] != 0 && qrx[11] == clamped {
                        return true;
                    }
                }
            }

            for attempt in 0..2u8 {
                let mut tx = [0u8; 64];
                tx[0] = HID_REPORT_ID_OUT;
                tx[1] = 0x77u8.wrapping_add(attempt);
                tx[2] = HID_CMD_GOTO_PROFILE_BY_NUM;
                tx[3] = clamped;

                if device.write(&tx).is_ok() {
                    let mut rx = [0u8; 64];
                    if let Ok(n) = device.read_timeout(&mut rx, 500) {
                        if n >= 3 && rx[2] == 2 {
                            // HID_RESPONSE_BUSY: wait briefly and retry once
                            std::thread::sleep(std::time::Duration::from_millis(50));
                            continue;
                        }
                    }
                    return true;
                }
            }
        }
    }
    false
}

pub async fn pick_window_via_kwin_dbus() -> std::result::Result<WindowChangeEvent, String> {
    let conn = zbus::Connection::session()
        .await
        .map_err(|e| format!("D-Bus session error: {e}"))?;

    let kwin_proxy = zbus::Proxy::new(&conn, "org.kde.KWin", "/KWin", "org.kde.KWin")
        .await
        .map_err(|e| format!("KWin proxy error: {e}"))?;

    let info: std::collections::HashMap<String, zbus::zvariant::OwnedValue> = kwin_proxy
        .call("queryWindowInfo", &())
        .await
        .map_err(|e| format!("Window picker canceled or failed: {e}"))?;

    let extract_str = |key: &str| -> String {
        info.get(key)
            .and_then(|v| <&str>::try_from(v).ok().map(|s| s.to_string()))
            .unwrap_or_default()
    };

    let resource_class = extract_str("resourceClass");
    let resource_name = extract_str("resourceName");
    let raw_caption = extract_str("caption");

    if resource_class.is_empty() && resource_name.is_empty() {
        return Err("Window picker canceled".to_string());
    }

    let effective_class = if !resource_class.is_empty() {
        resource_class
    } else {
        resource_name.clone()
    };
    let caption = resolve_full_browser_caption(&effective_class, &raw_caption);

    Ok(WindowChangeEvent {
        resource_class: effective_class,
        resource_name,
        caption,
        matched_profile: None,
        picked: true,
    })
}

pub async fn run_kwin_auto_switcher(
    state: Arc<Mutex<SwitcherState>>,
    ui_event_tx: mpsc::UnboundedSender<WindowChangeEvent>,
) -> ZbusResult<()> {
    let service = KWinBridgeService { state, ui_event_tx };
    let conn = zbus::conn::Builder::session()?
        .name("io.github.duckydeck.Studio")?
        .serve_at("/KWinBridge", service)?
        .build()
        .await?;

    let script_path = write_kwin6_hook_script()
        .map_err(|e| zbus::Error::Failure(format!("Failed to write KWin JS hook: {e}")))?;

    let kwin_scripting = zbus::Proxy::new(
        &conn,
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
    )
    .await?;

    let plugin_name = "duckydeck_kwin6_autoswitcher";
    let _: std::result::Result<bool, _> =
        kwin_scripting.call("unloadScript", &(plugin_name,)).await;

    let mut script_id: i32 = kwin_scripting
        .call("loadScript", &(&script_path, plugin_name))
        .await?;

    if script_id < 0 {
        let fallback_name = format!("duckydeck_kwin6_autoswitcher_{}", std::process::id());
        let _: std::result::Result<bool, _> = kwin_scripting
            .call("unloadScript", &(fallback_name.as_str(),))
            .await;
        script_id = kwin_scripting
            .call("loadScript", &(&script_path, fallback_name.as_str()))
            .await?;
    }

    if script_id >= 0 {
        let script_obj_path = format!("/Scripting/Script{script_id}");
        if let Ok(kwin_script) = zbus::Proxy::new(
            &conn,
            "org.kde.KWin",
            script_obj_path.as_str(),
            "org.kde.kwin.Script",
        )
        .await
        {
            let _ = kwin_script.call_noreply("run", &()).await;
        }
    }

    std::future::pending::<()>().await;
    Ok(())
}

fn write_kwin6_hook_script() -> std::io::Result<String> {
    let js_code = r#"
var trackedWindow = null;
var hookedWindows = [];

function isWindowHooked(win) {
    for (var i = 0; i < hookedWindows.length; ++i) {
        if (hookedWindows[i] === win) {
            return true;
        }
    }
    return false;
}

function isOledWidgetWindow(win) {
    if (!win) return false;
    var cap = win.caption ? win.caption.toString() : "";
    return cap.indexOf("duckyPad OLED —") === 0;
}

function isDuckyDeckMainWindow(win) {
    if (!win || isOledWidgetWindow(win)) return false;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var cap = win.caption ? win.caption.toString() : "";
    return cls.indexOf("duckydeck") !== -1 || cap.indexOf("DuckyDeck Studio") === 0;
}

function checkOledKeepAbove(win) {
    if (!win) return;
    if (isOledWidgetWindow(win)) {
        var cap = win.caption ? win.caption.toString() : "";
        var isPinned = (cap.indexOf("[Unpinned]") === -1);
        win.keepAbove = isPinned;
        win.skipTaskbar = isPinned;
        win.skipPager = isPinned;
        if (isPinned && win.minimized) {
            win.minimized = false;
        }
        if (!isWindowHooked(win)) {
            hookedWindows.push(win);
            try {
                callDBus(
                    "io.github.duckydeck.Studio",
                    "/KWinBridge",
                    "io.github.duckydeck.KWinBridge",
                    "NotifyOledWidgetMapped",
                    isPinned
                );
                if (win.minimizedChanged) {
                    win.minimizedChanged.connect(function() {
                        var c = win.caption ? win.caption.toString() : "";
                        var pinnedNow = (c.indexOf("[Unpinned]") === -1);
                        if (pinnedNow && win.minimized) {
                            win.minimized = false;
                            win.keepAbove = true;
                        }
                    });
                }
                if (win.captionChanged) {
                    win.captionChanged.connect(function() {
                        checkOledKeepAbove(win);
                    });
                }
                if (win.frameGeometryChanged) {
                    win.frameGeometryChanged.connect(function() {
                        if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 180 && win.frameGeometry.height >= 220) {
                            callDBus(
                                "io.github.duckydeck.Studio",
                                "/KWinBridge",
                                "io.github.duckydeck.KWinBridge",
                                "NotifyOledWidgetGeometry",
                                Math.round(win.frameGeometry.x),
                                Math.round(win.frameGeometry.y),
                                Math.round(win.frameGeometry.width),
                                Math.round(win.frameGeometry.height)
                            );
                        }
                    });
                }
            } catch (e) {}
        }
        return;
    }

    if (isDuckyDeckMainWindow(win)) {
        if (!isWindowHooked(win)) {
            hookedWindows.push(win);
            try {
                if (win.minimizedChanged) {
                    win.minimizedChanged.connect(function() {
                        if (win.minimized) {
                            win.skipTaskbar = true;
                            win.skipPager = true;
                            win.skipSwitcher = true;
                            callDBus(
                                "io.github.duckydeck.Studio",
                                "/KWinBridge",
                                "io.github.duckydeck.KWinBridge",
                                "NotifyMainWindowMinimized"
                            );
                        } else {
                            win.skipTaskbar = false;
                            win.skipPager = false;
                            win.skipSwitcher = false;
                        }
                    });
                }
            } catch (e) {}
        }
        if (win.minimized) {
            win.skipTaskbar = true;
            win.skipPager = true;
            win.skipSwitcher = true;
            callDBus(
                "io.github.duckydeck.Studio",
                "/KWinBridge",
                "io.github.duckydeck.KWinBridge",
                "NotifyMainWindowMinimized"
            );
        } else {
            win.skipTaskbar = false;
            win.skipPager = false;
            win.skipSwitcher = false;
        }
    }
}

function isIgnoredWindow(win) {
    if (!win) return true;
    checkOledKeepAbove(win);
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    return cls === "" || cls === "plasmashell" || cls.indexOf("duckydeck") !== -1;
}

function sendWindow(win) {
    if (!win) return;
    callDBus(
        "io.github.duckydeck.Studio",
        "/KWinBridge",
        "io.github.duckydeck.KWinBridge",
        "NotifyActiveWindow",
        win.resourceClass ? win.resourceClass.toString() : "",
        win.resourceName ? win.resourceName.toString() : "",
        win.caption ? win.caption.toString() : ""
    );
}

function onTrackedCaptionChanged() {
    if (trackedWindow && !isIgnoredWindow(trackedWindow)) {
        sendWindow(trackedWindow);
    }
}

function onWindowActivated(win) {
    if (!win || isIgnoredWindow(win)) return;
    if (trackedWindow && trackedWindow !== win) {
        try {
            if (trackedWindow.captionChanged) {
                trackedWindow.captionChanged.disconnect(onTrackedCaptionChanged);
            }
        } catch (e) {}
    }
    trackedWindow = win;
    try {
        if (trackedWindow.captionChanged) {
            trackedWindow.captionChanged.connect(onTrackedCaptionChanged);
        }
    } catch (e) {}
    sendWindow(win);
}

function sendInitialTopWindow() {
    var order = workspace.stackingOrder || workspace.windowList();
    if (order && order.length) {
        for (var j = 0; j < order.length; ++j) {
            checkOledKeepAbove(order[j]);
        }
    }
    if (workspace.activeWindow && !isIgnoredWindow(workspace.activeWindow)) {
        onWindowActivated(workspace.activeWindow);
        return;
    }
    if (order && order.length) {
        for (var i = order.length - 1; i >= 0; --i) {
            var w = order[i];
            if (!isIgnoredWindow(w) && (!w.minimized)) {
                onWindowActivated(w);
                return;
            }
        }
    }
}

if (workspace.windowAdded) {
    workspace.windowAdded.connect(checkOledKeepAbove);
}
if (workspace.windowActivated) {
    workspace.windowActivated.connect(onWindowActivated);
    sendInitialTopWindow();
} else if (workspace.clientActivated) {
    workspace.clientActivated.connect(onWindowActivated);
    onWindowActivated(workspace.activeClient);
}
"#;

    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let file_path = format!("{runtime_dir}/duckydeck-kwin6-hook.js");

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(js_code.as_bytes())?;
    Ok(file_path)
}

fn load_saved_oled_geometry_tuple() -> Option<(i32, i32, i32, i32)> {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".config")
        });
    let path = base
        .join("duckydeck-studio")
        .join("oled_widget_geometry.json");
    let raw = fs::read_to_string(path).ok()?;
    let val: serde_json::Value = serde_json::from_str(&raw).ok()?;
    if !val.get("has_position").and_then(|v| v.as_bool()).unwrap_or(false) {
        return None;
    }
    let x = val.get("x").and_then(|v| v.as_i64())? as i32;
    let y = val.get("y").and_then(|v| v.as_i64())? as i32;
    let w = (val.get("width").and_then(|v| v.as_i64()).unwrap_or(284) as i32).clamp(184, 1600);
    let h = (val.get("height").and_then(|v| v.as_i64()).unwrap_or(356) as i32).clamp(240, 1600);
    Some((x, y, w, h))
}

static OLED_PIN_SCRIPT_LOCK: std::sync::LazyLock<std::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(()));

pub fn apply_kwin_oled_keep_above(keep_above: bool, restore_geometry: bool) {
    std::thread::spawn(move || {
        let _guard = OLED_PIN_SCRIPT_LOCK.lock().ok();
        let geom_opt = if restore_geometry {
            load_saved_oled_geometry_tuple()
        } else {
            None
        };
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(_) => return,
        };
        rt.block_on(async move {
            let Ok(conn) = zbus::Connection::session().await else {
                return;
            };
            let Ok(kwin_scripting) = zbus::Proxy::new(
                &conn,
                "org.kde.KWin",
                "/Scripting",
                "org.kde.kwin.Scripting",
            )
            .await
            else {
                return;
            };

            let runtime_dir =
                std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
            let script_path = format!("{runtime_dir}/duckydeck-kwin6-oled-pin.js");
            let flag_str = if keep_above { "true" } else { "false" };
            let geom_js = if let Some((gx, gy, gw, gh)) = geom_opt {
                format!(
                    "w.frameGeometry = {{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }};"
                )
            } else {
                String::new()
            };
            let js = format!(
                r#"
var list = workspace.stackingOrder || workspace.windowList();
if (list) {{
    for (var i = 0; i < list.length; ++i) {{
        var w = list[i];
        var cap = w && w.caption ? w.caption.toString() : "";
        if (cap.indexOf("duckyPad OLED —") === 0) {{
            w.keepAbove = {flag_str};
            w.skipTaskbar = {flag_str};
            w.skipPager = {flag_str};
            if ({flag_str} && w.minimized) {{
                w.minimized = false;
            }}
            {geom_js}
        }}
    }}
}}
"#
            );
            if std::fs::write(&script_path, js.as_bytes()).is_err() {
                return;
            }
            let plugin_name = "duckydeck_oled_keep_above_oneshot";
            let delays_ms: &[u64] = if geom_opt.is_some() {
                &[0, 50, 140]
            } else {
                &[0]
            };
            for &delay in delays_ms {
                if delay > 0 {
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
                let _: std::result::Result<bool, _> =
                    kwin_scripting.call("unloadScript", &(plugin_name,)).await;
                if let Ok(script_id) = kwin_scripting
                    .call::<_, _, i32>("loadScript", &(&script_path, plugin_name))
                    .await
                {
                    if script_id >= 0 {
                        let obj_path = format!("/Scripting/Script{script_id}");
                        if let Ok(script_proxy) = zbus::Proxy::new(
                            &conn,
                            "org.kde.KWin",
                            obj_path.as_str(),
                            "org.kde.kwin.Script",
                        )
                        .await
                        {
                            let _ = script_proxy.call_noreply("run", &()).await;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(35)).await;
                        let _: std::result::Result<bool, _> =
                            kwin_scripting.call("unloadScript", &(plugin_name,)).await;
                    }
                }
            }
        });
    });
}
