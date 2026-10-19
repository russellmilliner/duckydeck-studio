use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use zbus::{interface, Result as ZbusResult};

use crate::hardware::storage::{
    DuckyProfile, HID_CMD_GOTO_PROFILE_BY_NUM, HID_CMD_QUERY_INFO, HID_REPORT_ID_OUT,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSwitchRule {
    pub enabled: bool,
    pub resource_class: String,
    pub title_contains: Option<String>,
    #[serde(default)]
    pub title_is_regex: bool,
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

pub fn default_mouse_settle_ms() -> i32 {
    250
}

pub fn default_window_dwell_ms() -> i32 {
    150
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

pub fn default_rest_api_bind_ip() -> String {
    "127.0.0.1".to_string()
}

pub fn default_rest_api_port() -> u16 {
    28491
}

pub fn generate_rest_api_token() -> String {
    let mut buf = [0u8; 20];
    let mut filled = false;
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        use std::io::Read;
        if f.read_exact(&mut buf).is_ok() {
            filled = true;
        }
    }
    if !filled {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0x9e3779b97f4a7c15);
        let pid = std::process::id() as u128;
        let mut seed = nanos ^ (pid << 32) ^ 0x517cc1b727220a95;
        for b in &mut buf {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *b = (seed >> 56) as u8;
        }
    }
    let mut hex = String::with_capacity(44);
    hex.push_str("ddk_");
    for b in buf {
        hex.push_str(&format!("{b:02x}"));
    }
    hex
}

fn default_webhook_method() -> String {
    "POST".to_string()
}

fn default_webhook_headers() -> String {
    "Content-Type: application/json".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedSwitcherConfig {
    pub enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    #[serde(default = "default_mouse_settle_ms")]
    pub mouse_settle_ms: i32,
    #[serde(default = "default_window_dwell_ms")]
    pub window_dwell_ms: i32,
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
    #[serde(default)]
    pub rest_api_enabled: bool,
    #[serde(default = "default_rest_api_bind_ip")]
    pub rest_api_bind_ip: String,
    #[serde(default = "default_rest_api_port")]
    pub rest_api_port: u16,
    #[serde(default = "default_true")]
    pub rest_api_require_auth: bool,
    #[serde(default = "generate_rest_api_token")]
    pub rest_api_token: String,
    #[serde(default)]
    pub profile_webhook_enabled: bool,
    #[serde(default = "default_webhook_method")]
    pub profile_webhook_method: String,
    #[serde(default)]
    pub profile_webhook_url: String,
    #[serde(default = "default_webhook_headers")]
    pub profile_webhook_headers: String,
    #[serde(default)]
    pub profile_webhook_body: String,
    #[serde(default = "default_true")]
    pub autoswitch_enable_section_expanded: bool,
    #[serde(default = "default_true")]
    pub autoswitch_rule_editor_expanded: bool,
}

#[derive(Clone)]
pub struct SwitcherState {
    pub enabled: bool,
    pub paused_for_editing: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    pub mouse_settle_ms: i32,
    pub window_dwell_ms: i32,
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
    pub rest_api_enabled: bool,
    pub rest_api_bind_ip: String,
    pub rest_api_port: u16,
    pub rest_api_require_auth: bool,
    pub rest_api_token: String,
    pub profile_webhook_enabled: bool,
    pub profile_webhook_method: String,
    pub profile_webhook_url: String,
    pub profile_webhook_headers: String,
    pub profile_webhook_body: String,
    pub autoswitch_enable_section_expanded: bool,
    pub autoswitch_rule_editor_expanded: bool,
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
            let bind_ip = if cfg.rest_api_bind_ip.trim().is_empty() {
                default_rest_api_bind_ip()
            } else {
                cfg.rest_api_bind_ip.trim().to_string()
            };
            let port = if cfg.rest_api_port < 1024 {
                default_rest_api_port()
            } else {
                cfg.rest_api_port
            };
            let token = if cfg.rest_api_token.trim().is_empty() {
                generate_rest_api_token()
            } else {
                cfg.rest_api_token.trim().to_string()
            };
            let wh_method = if cfg.profile_webhook_method.trim().is_empty() {
                default_webhook_method()
            } else {
                cfg.profile_webhook_method.trim().to_ascii_uppercase()
            };
            return Self {
                enabled: cfg.enabled,
                paused_for_editing: false,
                default_profile: Some(cfg.default_profile.unwrap_or(1).max(1)),
                rules: cfg.rules,
                mouse_settle_ms: cfg.mouse_settle_ms.clamp(0, 5000),
                window_dwell_ms: cfg.window_dwell_ms.clamp(0, 5000),
                tray_icon_mode: mode,
                app_theme_mode: theme,
                close_to_tray: cfg.close_to_tray,
                ui_text_scale: cfg.ui_text_scale.clamp(75, 160),
                sidebar_text_scale: cfg.ui_text_scale.clamp(75, 160),
                canvas_text_scale: cfg.ui_text_scale.clamp(75, 160),
                inspector_text_scale: cfg.ui_text_scale.clamp(75, 160),
                editor_text_scale: cfg.ui_text_scale.clamp(75, 160),
                status_bar_text_scale: cfg.ui_text_scale.clamp(75, 160),
                event_log_text_scale: cfg.ui_text_scale.clamp(75, 160),
                saved_colors,
                rest_api_enabled: cfg.rest_api_enabled,
                rest_api_bind_ip: bind_ip,
                rest_api_port: port,
                rest_api_require_auth: cfg.rest_api_require_auth,
                rest_api_token: token,
                profile_webhook_enabled: cfg.profile_webhook_enabled,
                profile_webhook_method: wh_method,
                profile_webhook_url: cfg.profile_webhook_url.trim().to_string(),
                profile_webhook_headers: cfg.profile_webhook_headers.trim().to_string(),
                profile_webhook_body: cfg.profile_webhook_body,
                autoswitch_enable_section_expanded: cfg.autoswitch_enable_section_expanded,
                autoswitch_rule_editor_expanded: cfg.autoswitch_rule_editor_expanded,
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
                    title_is_regex: false,
                    favicon_url: None,
                    target_profile: 2,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "firefox".into(),
                    title_contains: None,
                    title_is_regex: false,
                    favicon_url: None,
                    target_profile: 3,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "code".into(),
                    title_contains: None,
                    title_is_regex: false,
                    favicon_url: None,
                    target_profile: 4,
                },
            ],
            mouse_settle_ms: default_mouse_settle_ms(),
            window_dwell_ms: default_window_dwell_ms(),
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
            rest_api_enabled: false,
            rest_api_bind_ip: default_rest_api_bind_ip(),
            rest_api_port: default_rest_api_port(),
            rest_api_require_auth: true,
            rest_api_token: generate_rest_api_token(),
            profile_webhook_enabled: false,
            profile_webhook_method: default_webhook_method(),
            profile_webhook_url: String::new(),
            profile_webhook_headers: default_webhook_headers(),
            profile_webhook_body: String::new(),
            autoswitch_enable_section_expanded: true,
            autoswitch_rule_editor_expanded: true,
            last_switched_profile: 0,
            picker_armed: false,
            loaded_from_disk: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdPersistedRule {
    pub enabled: bool,
    pub resource_class: String,
    #[serde(default)]
    pub title_contains: Option<String>,
    #[serde(default)]
    pub title_is_regex: bool,
    #[serde(default)]
    pub favicon_url: Option<String>,
    pub target_profile: u8,
    #[serde(default)]
    pub target_profile_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdPersistedRulesBundle {
    #[serde(default = "default_rules_bundle_version")]
    pub version: u32,
    pub enabled: bool,
    pub default_profile: Option<u8>,
    #[serde(default)]
    pub default_profile_name: String,
    pub rules: Vec<SdPersistedRule>,
}

fn default_rules_bundle_version() -> u32 {
    1
}

pub const SD_RULES_FILENAME: &str = "duckydeck_rules.json";

pub fn save_rules_bundle_to_dir(
    dir: &Path,
    enabled: bool,
    default_profile: Option<u8>,
    rules: &[AutoSwitchRule],
    profiles: &[DuckyProfile],
) -> Result<bool, String> {
    if !dir.is_dir() {
        return Ok(false);
    }
    let fb_num = default_profile.unwrap_or(1).max(1);
    let fb_name = profiles
        .iter()
        .find(|p| p.number == fb_num)
        .map(|p| p.name.clone())
        .unwrap_or_default();
    let sd_rules: Vec<SdPersistedRule> = rules
        .iter()
        .map(|r| {
            let p_name = profiles
                .iter()
                .find(|p| p.number == r.target_profile)
                .map(|p| p.name.clone())
                .unwrap_or_default();
            SdPersistedRule {
                enabled: r.enabled,
                resource_class: r.resource_class.clone(),
                title_contains: r.title_contains.clone(),
                title_is_regex: r.title_is_regex,
                favicon_url: r.favicon_url.clone(),
                target_profile: r.target_profile,
                target_profile_name: p_name,
            }
        })
        .collect();
    let bundle = SdPersistedRulesBundle {
        version: 1,
        enabled,
        default_profile: Some(fb_num),
        default_profile_name: fb_name,
        rules: sd_rules,
    };
    let json = serde_json::to_string_pretty(&bundle)
        .map_err(|e| format!("Failed serializing SD rules bundle: {e}"))?;
    let path = dir.join(SD_RULES_FILENAME);
    if path.is_file() {
        if let Ok(existing) = fs::read_to_string(&path) {
            if existing == json {
                return Ok(false);
            }
        }
    }
    fs::write(&path, json).map_err(|e| format!("Failed writing {}: {e}", path.display()))?;
    Ok(true)
}

pub fn load_rules_bundle_from_dir(
    dir: &Path,
    profiles: &[DuckyProfile],
) -> Option<(bool, Option<u8>, Vec<AutoSwitchRule>)> {
    let path = dir.join(SD_RULES_FILENAME);
    let raw = fs::read_to_string(&path).ok()?;
    let bundle: SdPersistedRulesBundle = serde_json::from_str(&raw).ok()?;
    if bundle.rules.is_empty() {
        return None;
    }
    let max_prof = profiles.len().max(1) as u8;
    let resolved_fb = if !bundle.default_profile_name.trim().is_empty() {
        profiles
            .iter()
            .find(|p| {
                p.name
                    .eq_ignore_ascii_case(bundle.default_profile_name.trim())
            })
            .map(|p| p.number)
            .or(bundle.default_profile)
    } else {
        bundle.default_profile
    }
    .unwrap_or(1)
    .clamp(1, max_prof.max(64));

    let mut rules = Vec::with_capacity(bundle.rules.len());
    for sr in bundle.rules {
        let target = if !sr.target_profile_name.trim().is_empty() {
            profiles
                .iter()
                .find(|p| p.name.eq_ignore_ascii_case(sr.target_profile_name.trim()))
                .map(|p| p.number)
                .unwrap_or(sr.target_profile)
        } else {
            sr.target_profile
        };
        rules.push(AutoSwitchRule {
            enabled: sr.enabled,
            resource_class: sr.resource_class,
            title_contains: sr.title_contains,
            title_is_regex: sr.title_is_regex,
            favicon_url: sr.favicon_url,
            target_profile: target.max(1),
        });
    }
    Some((bundle.enabled, Some(resolved_fb), rules))
}

pub fn sync_host_companion_configs_if_needed() {
    if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1")
        || std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1")
    {
        return;
    }
    let cfg_dir = crate::paths::config_dir();
    let host_dir = crate::paths::host_config_dir();
    if cfg_dir == host_dir || !host_dir.is_dir() {
        return;
    }
    let _ = fs::create_dir_all(&cfg_dir); // ignore: best-effort directory creation
    for fname in [
        "home_assistant.json",
        "web_api_destinations.json",
        "oled_widget_geometry.json",
    ] {
        let src = host_dir.join(fname);
        let dst = cfg_dir.join(fname);
        if src.is_file() && !dst.is_file() {
            let _ = fs::copy(&src, &dst); // ignore: best-effort companion config migration
        }
    }
}

impl SwitcherState {
    pub fn config_file_path() -> PathBuf {
        if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1") {
            return std::env::temp_dir()
                .join(format!("duckydeck-ui-test-suite-{}", std::process::id()))
                .join("kwin_rules.json");
        }
        crate::paths::config_dir().join("kwin_rules.json")
    }

    pub fn host_config_file_path() -> PathBuf {
        crate::paths::host_config_dir().join("kwin_rules.json")
    }

    pub fn load_config_from_disk() -> Option<PersistedSwitcherConfig> {
        sync_host_companion_configs_if_needed();
        let path = Self::config_file_path();
        let primary_cfg = fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str::<PersistedSwitcherConfig>(&raw).ok());

        if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1") {
            return primary_cfg;
        }

        let host_path = Self::host_config_file_path();
        let host_cfg = if host_path != path {
            fs::read_to_string(&host_path)
                .ok()
                .and_then(|raw| serde_json::from_str::<PersistedSwitcherConfig>(&raw).ok())
        } else {
            None
        };

        match (primary_cfg, host_cfg) {
            (Some(p_cfg), Some(h_cfg)) => {
                // If the Flatpak config only has the initial default/auto-aligned rules (<= 14)
                // while the host config has the user's richer rule set, adopt the host config!
                if h_cfg.rules.len() > p_cfg.rules.len() && p_cfg.rules.len() <= 14 {
                    if let Ok(json) = serde_json::to_string_pretty(&h_cfg) {
                        if let Some(parent) = path.parent() {
                            let _ = fs::create_dir_all(parent); // ignore: best-effort
                        }
                        let _ = fs::write(&path, json); // ignore: best-effort sync to Flatpak config
                    }
                    Some(h_cfg)
                } else {
                    Some(p_cfg)
                }
            }
            (None, Some(h_cfg)) => {
                if let Ok(json) = serde_json::to_string_pretty(&h_cfg) {
                    if let Some(parent) = path.parent() {
                        let _ = fs::create_dir_all(parent); // ignore: best-effort
                    }
                    let _ = fs::write(&path, json); // ignore: best-effort sync to Flatpak config
                }
                Some(h_cfg)
            }
            (Some(p_cfg), None) => Some(p_cfg),
            (None, None) => None,
        }
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        if std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1")
            && std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() != Ok("1")
        {
            return Ok(());
        }
        let path = Self::config_file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed creating config dir: {e}"))?;
        }
        let unified_scale = self.ui_text_scale.clamp(75, 160);
        let payload = PersistedSwitcherConfig {
            enabled: self.enabled,
            default_profile: self.default_profile,
            rules: self.rules.clone(),
            mouse_settle_ms: self.mouse_settle_ms.clamp(0, 5000),
            window_dwell_ms: self.window_dwell_ms.clamp(0, 5000),
            tray_icon_mode: self.tray_icon_mode.clone(),
            app_theme_mode: self.app_theme_mode.clone(),
            close_to_tray: self.close_to_tray,
            ui_text_scale: unified_scale,
            sidebar_text_scale: unified_scale,
            canvas_text_scale: unified_scale,
            inspector_text_scale: unified_scale,
            editor_text_scale: unified_scale,
            status_bar_text_scale: unified_scale,
            event_log_text_scale: unified_scale,
            saved_colors: self.saved_colors.clone(),
            rest_api_enabled: self.rest_api_enabled,
            rest_api_bind_ip: self.rest_api_bind_ip.clone(),
            rest_api_port: self.rest_api_port,
            rest_api_require_auth: self.rest_api_require_auth,
            rest_api_token: self.rest_api_token.clone(),
            profile_webhook_enabled: self.profile_webhook_enabled,
            profile_webhook_method: self.profile_webhook_method.clone(),
            profile_webhook_url: self.profile_webhook_url.clone(),
            profile_webhook_headers: self.profile_webhook_headers.clone(),
            profile_webhook_body: self.profile_webhook_body.clone(),
            autoswitch_enable_section_expanded: self.autoswitch_enable_section_expanded,
            autoswitch_rule_editor_expanded: self.autoswitch_rule_editor_expanded,
        };
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("Failed serializing KWin rules: {e}"))?;
        fs::write(&path, &json).map_err(|e| format!("Failed writing {}: {e}", path.display()))?;

        if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() != Ok("1") {
            let host_path = Self::host_config_file_path();
            if host_path != path {
                if let Some(h_parent) = host_path.parent() {
                    let _ = fs::create_dir_all(h_parent); // ignore: best-effort host mirror
                }
                let _ = fs::write(&host_path, &json); // ignore: best-effort host mirror
            }
        }
        Ok(())
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

pub fn is_chromium_pwa_class(class_lower: &str) -> bool {
    for prefix in ["chrome-", "chromium-", "brave-", "msedge-"] {
        if let Some(rest) = class_lower.strip_prefix(prefix) {
            let app_id = rest.split('-').next().unwrap_or("");
            if (16..=40).contains(&app_id.len())
                && app_id.chars().all(|c| c.is_ascii_alphanumeric())
            {
                return true;
            }
        }
    }
    false
}

pub fn rule_matches_window_class(rule_class_raw: &str, active_window_class_lower: &str) -> bool {
    let trimmed = rule_class_raw.trim();
    if trimmed.is_empty() || active_window_class_lower.is_empty() {
        return false;
    }
    if trimmed == "*" || trimmed.eq_ignore_ascii_case("any") {
        return true;
    }
    if is_any_browser_wildcard(trimmed) {
        return is_web_browser_class(active_window_class_lower)
            || is_chromium_pwa_class(active_window_class_lower);
    }

    let is_pwa = is_chromium_pwa_class(active_window_class_lower);
    let norm_active: String = active_window_class_lower
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    for part in trimmed.split([',', '|']) {
        let token = part.trim().to_ascii_lowercase();
        if token.is_empty() {
            continue;
        }
        if token == "*" || token == "any" {
            return true;
        }
        if is_any_browser_wildcard(&token) {
            if is_web_browser_class(active_window_class_lower)
                || is_chromium_pwa_class(active_window_class_lower)
            {
                return true;
            }
            continue;
        }
        if active_window_class_lower == token {
            return true;
        }
        // Do not let generic browser rules (e.g. "chrome", "chromium") match standalone Chrome PWAs
        // like "chrome-hbmnbinfcdjocecbhmpfeapfnchehanm-Profile_1" unless the rule explicitly targets that PWA ID.
        if is_pwa && !is_chromium_pwa_class(&token) {
            continue;
        }
        let norm_token: String = token
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        if !norm_token.is_empty() && norm_active == norm_token {
            return true;
        }
        if active_window_class_lower
            .split(['.', '-', '_'])
            .any(|seg| seg == token || (!norm_token.is_empty() && seg == norm_token))
        {
            return true;
        }
        if token.len() >= 5 && active_window_class_lower.contains(&token) {
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
    suffixes
        .into_iter()
        .find(|&suf| trimmed.len() > suf.len() && lower.ends_with(&suf.to_lowercase()))
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
    let home_path = crate::paths::home_dir()?;
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
    profile_dirs.sort_by_key(|a| std::cmp::Reverse(a.0));

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
    let dir = crate::paths::cache_dir().join("site-icons");
    if let Err(e) = fs::create_dir_all(&dir) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "CONFIG",
            format!("Failed creating site icon cache dir {}: {e}", dir.display()),
        );
    }
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
        if let Err(e) = fs::write(&path, svg_content) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "CONFIG",
                format!("Failed writing cached site SVG {}: {e}", path.display()),
            );
        }
    }
    format!("file://{}", path.to_string_lossy())
}

pub use crate::bridge::showcase::known_site_svg;

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

    let (scheme, rest) = with_scheme.split_once("://").unwrap_or(("https", trimmed));
    let host_port = rest.split('/').next().unwrap_or(rest).trim();
    let host_only = host_port
        .split(':')
        .next()
        .unwrap_or(host_port)
        .to_ascii_lowercase();

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
                    } else if bytes.starts_with(&[0x00, 0x00, 0x01, 0x00]) || ctype.contains("icon")
                    {
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
            if let Err(e) = fs::write(&failed_marker, b"1") {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "CONFIG",
                    format!(
                        "Failed writing favicon failure marker {}: {e}",
                        failed_marker.display()
                    ),
                );
            }
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
    let data_home = crate::paths::xdg_data_home();
    search_dirs.push(data_home.join("applications"));
    search_dirs.push(data_home.join("flatpak/exports/share/applications"));
    if let Some(home) = crate::paths::home_dir() {
        for candidate in [
            home.join(".local/share/applications"),
            home.join(".local/share/flatpak/exports/share/applications"),
        ] {
            if !search_dirs.contains(&candidate) {
                search_dirs.push(candidate);
            }
        }
    }
    search_dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
    search_dirs.push(PathBuf::from("/run/host/usr/local/share/applications"));
    search_dirs.push(PathBuf::from("/run/host/usr/share/applications"));
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
    crate::paths::xdg_config_home()
        .join("autostart")
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
    let data_home = crate::paths::xdg_data_home();

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
        let icon_file = icon_dir.join("io.github.duckydeck.Studio.svg");
        if let Err(e) = fs::write(&icon_file, svg_content) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "CONFIG",
                format!("Failed writing local icon {}: {e}", icon_file.display()),
            );
        }
    }

    // When running inside Flatpak, Flatpak itself exports io.github.duckydeck.Studio.desktop on the host
    if Path::new("/.flatpak-info").exists() {
        return;
    }

    let apps_dir = data_home.join("applications");
    if fs::create_dir_all(&apps_dir).is_ok() {
        let exe_path = if let Ok(appimage) = std::env::var("APPIMAGE") {
            appimage
        } else if let Some(local_bin) = crate::paths::home_dir()
            .map(|h| h.join(".local/bin/duckydeck-studio"))
            .filter(|p| p.exists())
        {
            local_bin.to_string_lossy().to_string()
        } else {
            std::env::current_exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "duckydeck-studio".to_string())
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
        let desktop_file = apps_dir.join("io.github.duckydeck.Studio.desktop");
        if let Err(e) = fs::write(&desktop_file, desktop_content) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "CONFIG",
                format!(
                    "Failed writing local desktop entry {}: {e}",
                    desktop_file.display()
                ),
            );
        }
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
        let exec_line = if Path::new("/.flatpak-info").exists() {
            "flatpak run io.github.duckydeck.Studio --autostart --minimized --foreground"
                .to_string()
        } else if let Ok(appimage) = std::env::var("APPIMAGE") {
            format!("\"{appimage}\" --autostart --minimized --foreground")
        } else {
            let exe_path = if let Some(local_bin) = crate::paths::home_dir()
                .map(|h| h.join(".local/bin/duckydeck-studio"))
                .filter(|p| p.exists())
            {
                local_bin.to_string_lossy().to_string()
            } else {
                std::env::current_exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "duckydeck-studio".to_string())
            };
            format!("\"{exe_path}\" --autostart --minimized --foreground")
        };
        let desktop_entry = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=DuckyDeck Studio\n\
             Comment=KDE Plasma 6 Configurator & Auto-Switcher for duckyPad Pro\n\
             Exec={exec_line}\n\
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

pub fn discover_dekunukem_autoswitcher_configs() -> Vec<(PathBuf, usize)> {
    let mut dirs = Vec::new();
    let host_dir = crate::paths::dekunukem_autoswitcher_dir();
    if host_dir.is_dir() {
        dirs.push(host_dir);
    }
    let xdg_dir = crate::paths::xdg_data_home().join("duckypad_autoswitcher");
    if xdg_dir.is_dir() && !dirs.contains(&xdg_dir) {
        dirs.push(xdg_dir);
    }

    let mut found: Vec<(PathBuf, usize)> = Vec::new();
    for dir in dirs {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
        paths.sort_by(|a, b| {
            let a_name = a.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let b_name = b.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let rank = |n: &str| -> u8 {
                if n.eq_ignore_ascii_case("config-LinuxPro.txt") {
                    0
                } else if n.eq_ignore_ascii_case("config-Pro.txt") {
                    1
                } else if n.eq_ignore_ascii_case("config.txt") {
                    2
                } else {
                    3
                }
            };
            rank(a_name)
                .cmp(&rank(b_name))
                .then_with(|| a_name.cmp(b_name))
        });
        for p in paths {
            if !p.is_file() {
                continue;
            }
            let Some(fname) = p.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if fname.eq_ignore_ascii_case("debug_log.txt") {
                continue;
            }
            let Ok(raw) = fs::read_to_string(&p) else {
                continue;
            };
            let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) else {
                continue;
            };
            if let Some(arr) = val.get("rules_list").and_then(|v| v.as_array()) {
                if !arr.is_empty() {
                    found.push((p, arr.len()));
                }
            }
        }
    }
    found
}

fn resolve_dekunukem_switch_to(
    switch_to: &serde_json::Value,
    profiles: &[DuckyProfile],
) -> Option<u8> {
    if let Some(num) = switch_to.as_u64() {
        if (1..=64).contains(&num) {
            return Some(num as u8);
        }
    }
    let s = switch_to.as_str()?.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(p) = profiles.iter().find(|p| p.name.eq_ignore_ascii_case(s)) {
        return Some(p.number);
    }
    let norm = |txt: &str| -> String {
        txt.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase()
    };
    let target_norm = norm(s);
    if !target_norm.is_empty() {
        if let Some(p) = profiles.iter().find(|p| norm(&p.name) == target_norm) {
            return Some(p.number);
        }
        // Common shorthand aliases used on 5-16 char duckyPad profile names
        let aliases: &[(&str, &[&str])] = &[
            ("ytmusic", &["youtubemusic", "ytmusic", "music"]),
            ("homeassist", &["homeassistant", "homeassist", "hass"]),
            ("msteams", &["microsoftteams", "msteams", "teams"]),
            ("gcalendar", &["googlecalendar", "gcalendar", "calendar"]),
            ("gmaps", &["googlemaps", "gmaps", "maps"]),
            (
                "ageofemiresii",
                &["ageofempiresii", "ageofemiresii", "aoe2"],
            ),
        ];
        for &(key, group) in aliases {
            if target_norm == key || group.contains(&target_norm.as_str()) {
                if let Some(p) = profiles
                    .iter()
                    .find(|p| group.contains(&norm(&p.name).as_str()))
                {
                    return Some(p.number);
                }
            }
        }
        if target_norm.len() >= 4 {
            if let Some(p) = profiles.iter().find(|p| {
                let pn = norm(&p.name);
                pn.len() >= 4 && (pn.starts_with(&target_norm) || target_norm.starts_with(&pn))
            }) {
                return Some(p.number);
            }
        }
    }
    if let Ok(num) = s.parse::<u8>() {
        if (1..=64).contains(&num) {
            return Some(num);
        }
    }
    None
}

pub fn parse_dekunukem_autoswitcher_file(
    path: &Path,
    profiles: &[DuckyProfile],
) -> Result<(bool, Option<u8>, Vec<AutoSwitchRule>), String> {
    let raw =
        fs::read_to_string(path).map_err(|e| format!("Failed reading {}: {e}", path.display()))?;
    let val: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed parsing {}: {e}", path.display()))?;
    let autoswitch_enabled = val
        .get("autoswitch_enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let arr = val
        .get("rules_list")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("No 'rules_list' array found in {}", path.display()))?;

    let mut default_profile: Option<u8> = None;
    let mut rules: Vec<AutoSwitchRule> = Vec::new();

    for item in arr {
        let app_raw = item
            .get("app_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let title_raw = item
            .get("window_title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let enabled = item
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let Some(switch_val) = item.get("switch_to") else {
            continue;
        };
        let Some(target_profile) = resolve_dekunukem_switch_to(switch_val, profiles) else {
            continue;
        };

        if app_raw.is_empty() && title_raw.is_empty() {
            if enabled {
                default_profile = Some(target_profile);
            }
            continue;
        }

        let app_lower = app_raw.to_ascii_lowercase();
        if app_lower == "kthreadd" || app_lower == "kded5" {
            continue;
        }

        let (resource_class, title_opt) = if app_raw.is_empty() {
            let cleaned_t = strip_browser_title_suffix(title_raw)
                .trim_matches(|c: char| c == '-' || c.is_whitespace())
                .to_string();
            let final_t = if cleaned_t.is_empty() {
                title_raw.to_string()
            } else {
                cleaned_t
            };
            (ANY_BROWSER_CLASS.to_string(), Some(final_t))
        } else {
            let norm_class = match app_lower.as_str() {
                "dolphin" => "org.kde.dolphin".to_string(),
                "yakuake" => "org.kde.yakuake".to_string(),
                "zen" => "app.zen_browser.zen".to_string(),
                "orcaslicer" => "OrcaSlicer".to_string(),
                "upnote" => "upnote".to_string(),
                _ => app_raw.to_string(),
            };
            let cleaned_t = if title_raw.is_empty() {
                String::new()
            } else if is_web_browser_class(&norm_class) {
                strip_browser_title_suffix(title_raw)
                    .trim_matches(|c: char| c == '-' || c.is_whitespace())
                    .to_string()
            } else {
                let tl = title_raw.to_ascii_lowercase();
                if (app_lower == "code" && tl.contains("visual studio code"))
                    || (app_lower == "codium" && tl.contains("vscodium"))
                    || (app_lower == "upnote" && tl == "upnote")
                {
                    String::new()
                } else {
                    title_raw.to_string()
                }
            };
            (
                norm_class,
                if cleaned_t.is_empty() {
                    None
                } else {
                    Some(cleaned_t)
                },
            )
        };

        let duplicate = rules.iter().any(|r| {
            r.resource_class.eq_ignore_ascii_case(&resource_class)
                && r.title_contains
                    .as_deref()
                    .unwrap_or("")
                    .eq_ignore_ascii_case(title_opt.as_deref().unwrap_or(""))
                && r.target_profile == target_profile
        });
        if !duplicate {
            rules.push(AutoSwitchRule {
                enabled,
                resource_class,
                title_contains: title_opt,
                title_is_regex: false,
                favicon_url: None,
                target_profile,
            });
        }
    }

    Ok((autoswitch_enabled, default_profile, rules))
}

pub fn build_auto_aligned_rules(profiles: &[DuckyProfile]) -> Vec<AutoSwitchRule> {
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
                title_is_regex: false,
                favicon_url: None,
                target_profile: prof.number,
            });
        }
    }
    aligned
}

pub fn extract_inline_regex_pattern(raw: &str) -> Option<(&str, bool)> {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix('/') {
        if let Some(body) = rest.strip_suffix("/i") {
            if !body.is_empty() {
                return Some((body, true));
            }
        } else if let Some(body) = rest.strip_suffix("/s") {
            if !body.is_empty() {
                return Some((body, false));
            }
        } else if let Some(body) = rest.strip_suffix('/') {
            if !body.is_empty() {
                return Some((body, true));
            }
        }
    }
    None
}

pub fn validate_title_regex(pattern_raw: &str) -> Result<(), String> {
    let trimmed = pattern_raw.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    let (body, case_insensitive) = extract_inline_regex_pattern(trimmed).unwrap_or((trimmed, true));
    regex::RegexBuilder::new(body)
        .case_insensitive(case_insensitive)
        .size_limit(256 * 1024)
        .build()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

pub fn rule_matches_window_title(
    pattern_raw: &str,
    title_is_regex: bool,
    caption: &str,
    caption_lower: &str,
) -> Option<usize> {
    let sub = pattern_raw.trim();
    if sub.is_empty() {
        return Some(0);
    }

    if title_is_regex {
        let (regex_body, case_insensitive) =
            extract_inline_regex_pattern(sub).unwrap_or((sub, true));
        let sanitized = regex_body.trim_matches('|').trim();
        if sanitized.is_empty() {
            return None;
        }
        if let Ok(re) = regex::RegexBuilder::new(sanitized)
            .case_insensitive(case_insensitive)
            .size_limit(256 * 1024)
            .build()
        {
            if let Some(mat) = re.find(caption) {
                if !mat.is_empty() || sanitized == "^$" {
                    return Some(mat.len().max(sanitized.len()).max(1));
                }
            }
        }
        return None;
    }

    if caption_lower.contains(&sub.to_lowercase()) {
        return Some(sub.len());
    }

    None
}

impl SwitcherState {
    /// When SD profiles are loaded and the user hasn't saved custom rules yet (or only had auto-aligned defaults),
    /// first check if full rules exist in host config or SD/cache `duckydeck_rules.json`, and otherwise align default rules.
    pub fn align_rules_with_profiles(&mut self, profiles: &[DuckyProfile]) {
        if profiles.len() <= 1 {
            return;
        }

        // 1. Check if host config or SD card rules bundle has a fuller rule set than what is currently in memory
        if let Some(disk_cfg) = Self::load_config_from_disk() {
            if disk_cfg.rules.len() > self.rules.len() && self.rules.len() <= 14 {
                self.enabled = disk_cfg.enabled;
                self.default_profile = Some(disk_cfg.default_profile.unwrap_or(1).max(1));
                self.rules = disk_cfg.rules;
                self.loaded_from_disk = true;
                let _ = self.save_to_disk(); // ignore: best-effort persist
                return;
            }
        }

        for candidate_dir in [
            crate::hardware::profile::profile_cache_dir(),
            crate::paths::host_data_dir().join("profile_cache"),
        ] {
            if let Some((en, fb, sd_rules)) = load_rules_bundle_from_dir(&candidate_dir, profiles) {
                if !self.loaded_from_disk
                    || (sd_rules.len() > self.rules.len() && self.rules.len() <= 14)
                {
                    self.enabled = en;
                    if fb.is_some() {
                        self.default_profile = fb;
                    }
                    self.rules = sd_rules;
                    self.loaded_from_disk = true;
                    let _ = self.save_to_disk(); // ignore: best-effort persist
                    return;
                }
            }
        }

        if self.loaded_from_disk {
            return;
        }

        let aligned = build_auto_aligned_rules(profiles);
        if !aligned.is_empty() {
            self.rules = aligned;
            if let Err(e) = self.save_to_disk() {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "CONFIG",
                    format!("Failed saving aligned auto-switch rules: {e}"),
                );
            }
        }
    }

    pub fn match_profile_for_window(&self, resource_class: &str, caption: &str) -> Option<u8> {
        let lower_class = resource_class.trim().to_lowercase();
        if lower_class.is_empty() || lower_class == "none" {
            return self.default_profile;
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
                Some(sub) if !sub.is_empty() => match rule_matches_window_title(
                    sub,
                    rule.title_is_regex,
                    &resolved_caption,
                    &caption_lower,
                ) {
                    Some(spec) => spec,
                    None => continue,
                },
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

struct PendingWindowSwitch {
    raw_class: String,
    raw_name: String,
    raw_caption: String,
    window_entered_at: std::time::Instant,
    last_motion_at: std::time::Instant,
    settled_and_dispatched: bool,
}

impl Default for PendingWindowSwitch {
    fn default() -> Self {
        let now = std::time::Instant::now();
        Self {
            raw_class: String::new(),
            raw_name: String::new(),
            raw_caption: String::new(),
            window_entered_at: now,
            last_motion_at: now,
            settled_and_dispatched: true,
        }
    }
}

pub struct KWinBridgeService {
    state: Arc<Mutex<SwitcherState>>,
    ui_event_tx: mpsc::UnboundedSender<WindowChangeEvent>,
    pending: Arc<Mutex<PendingWindowSwitch>>,
    settle_notify: Arc<tokio::sync::Notify>,
}

#[interface(name = "io.github.duckydeck.KWinBridge")]
impl KWinBridgeService {
    fn notify_active_window(
        &mut self,
        resource_class: String,
        resource_name: String,
        caption: String,
    ) {
        let mut effective_class = resource_class.trim().to_string();
        if effective_class.is_empty() {
            effective_class = resource_name.trim().to_string();
        }
        if effective_class.is_empty() {
            effective_class = "plasmashell".to_string();
        }

        let lower_class = effective_class.to_lowercase();
        if lower_class.contains("duckydeck") {
            return;
        }

        let (picker_armed, settle_ms, dwell_ms) = if let Ok(state) = self.state.lock() {
            (
                state.picker_armed,
                state.mouse_settle_ms.clamp(0, 5000),
                state.window_dwell_ms.clamp(0, 5000),
            )
        } else {
            return;
        };

        if picker_armed {
            return;
        }

        let now = std::time::Instant::now();
        let mut window_changed = false;
        let mut already_settled = false;

        if let Ok(mut pending) = self.pending.lock() {
            window_changed = pending.raw_class != effective_class || pending.raw_caption != caption;
            if window_changed {
                pending.raw_class = effective_class.clone();
                pending.raw_name = resource_name.clone();
                pending.raw_caption = caption.clone();
                pending.window_entered_at = now;
                pending.last_motion_at = now;
                pending.settled_and_dispatched = false;
            } else {
                pending.last_motion_at = now;
                already_settled = pending.settled_and_dispatched;
            }

            if settle_ms <= 0 && dwell_ms <= 0 && !pending.settled_and_dispatched {
                pending.settled_and_dispatched = true;
            }
        }

        if already_settled {
            return;
        }

        if settle_ms <= 0 && dwell_ms <= 0 {
            let resolved_caption = resolve_full_browser_caption(&effective_class, &caption);
            let mut matched_profile: Option<u8> = None;

            if let Ok(mut state) = self.state.lock() {
                if !state.picker_armed && state.enabled && !state.paused_for_editing {
                    matched_profile =
                        state.match_profile_for_window(&effective_class, &resolved_caption);

                    if let Some(target) = matched_profile {
                        if send_hid_switch_profile(target) {
                            state.last_switched_profile = target;
                        }
                    }
                }
            }

            if let Err(e) = self.ui_event_tx.send(WindowChangeEvent {
                resource_class: effective_class,
                resource_name,
                caption: resolved_caption,
                matched_profile,
                picked: false,
            }) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "KWIN",
                    format!("Failed sending window change event to UI: {e}"),
                );
            }
            return;
        }

        if window_changed {
            let resolved_caption = resolve_full_browser_caption(&effective_class, &caption);
            if let Err(e) = self.ui_event_tx.send(WindowChangeEvent {
                resource_class: effective_class,
                resource_name,
                caption: resolved_caption,
                matched_profile: None,
                picked: false,
            }) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "KWIN",
                    format!("Failed sending window change preview event to UI: {e}"),
                );
            }
        }

        self.settle_notify.notify_one();
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
        crate::bridge::device_controller::qobject::duckydeck_restore_oled_widget_geometry();
        apply_kwin_oled_keep_above(is_pinned, true);
    }

    fn dispatch_ipc_command(&mut self, command: String) -> u32 {
        let qcmd = cxx_qt_lib::QString::from(&command);
        crate::bridge::device_controller::qobject::duckydeck_dispatch_ipc_command(&qcmd);
        std::process::id()
    }

    fn get_pid(&self) -> u32 {
        std::process::id()
    }
}

pub static LAST_SW_PROFILE_SWITCH_MS: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);
pub static LAST_SW_PROFILE_TARGET: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);

pub fn current_epoch_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn send_hid_switch_profile(profile_number: u8) -> bool {
    if crate::hardware::storage::MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    let clamped = profile_number.clamp(1, 64);
    LAST_SW_PROFILE_TARGET.store(clamped, std::sync::atomic::Ordering::SeqCst);
    LAST_SW_PROFILE_SWITCH_MS.store(current_epoch_ms(), std::sync::atomic::Ordering::SeqCst);

    let _guard = crate::hardware::storage::HID_IO_LOCK.lock().ok();
    crate::hardware::storage::with_cached_duckypad_hid(|device| {
        // Query current hardware profile first so we don't re-flash the OLED if already on `clamped`,
        // while still switching if the user manually changed profiles on the physical device.
        let mut qtx = [0u8; 64];
        qtx[0] = HID_REPORT_ID_OUT;
        qtx[1] = 0x76;
        qtx[2] = HID_CMD_QUERY_INFO;
        if device.write(&qtx).is_ok() {
            let mut qrx = [0u8; 64];
            if let Ok(n) = crate::hardware::storage::read_hid_report4(device, &mut qrx, 200) {
                if n >= 12 && qrx[2] == 0 && qrx[6] != 0 && qrx[11] == clamped {
                    LAST_SW_PROFILE_SWITCH_MS
                        .store(current_epoch_ms(), std::sync::atomic::Ordering::SeqCst);
                    return Ok(true);
                }
            }
        }

        for attempt in 0..6u8 {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x77u8.wrapping_add(attempt);
            tx[2] = HID_CMD_GOTO_PROFILE_BY_NUM;
            tx[3] = clamped;

            if device.write(&tx).is_ok() {
                let mut rx = [0u8; 64];
                if let Ok(n) = crate::hardware::storage::read_hid_report4(device, &mut rx, 500) {
                    if n >= 3 && rx[2] == 2 {
                        // HID_RESPONSE_BUSY: wait briefly and retry
                        std::thread::sleep(std::time::Duration::from_millis(75));
                        continue;
                    }
                }
                LAST_SW_PROFILE_SWITCH_MS
                    .store(current_epoch_ms(), std::sync::atomic::Ordering::SeqCst);
                return Ok(true);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Err("Failed switching profile over HID".to_string())
    })
    .unwrap_or(false)
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
    if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1") {
        return Ok(());
    }

    let pending = Arc::new(Mutex::new(PendingWindowSwitch::default()));
    let settle_notify = Arc::new(tokio::sync::Notify::new());

    {
        let state_bg = Arc::clone(&state);
        let pending_bg = Arc::clone(&pending);
        let notify_bg = Arc::clone(&settle_notify);
        let ui_tx_bg = ui_event_tx.clone();

        tokio::spawn(async move {
            loop {
                let maybe_deadline = {
                    let (settle_ms, dwell_ms) = if let Ok(st) = state_bg.lock() {
                        (
                            st.mouse_settle_ms.clamp(0, 5000) as u64,
                            st.window_dwell_ms.clamp(0, 5000) as u64,
                        )
                    } else {
                        (250, 150)
                    };

                    if let Ok(p) = pending_bg.lock() {
                        if p.settled_and_dispatched || p.raw_class.is_empty() {
                            None
                        } else {
                            let settle_dl =
                                p.last_motion_at + std::time::Duration::from_millis(settle_ms);
                            let dwell_dl =
                                p.window_entered_at + std::time::Duration::from_millis(dwell_ms);
                            Some(settle_dl.max(dwell_dl))
                        }
                    } else {
                        None
                    }
                };

                let Some(deadline) = maybe_deadline else {
                    notify_bg.notified().await;
                    continue;
                };

                let now = std::time::Instant::now();
                if now < deadline {
                    tokio::select! {
                        _ = tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)) => {}
                        _ = notify_bg.notified() => {
                            continue;
                        }
                    }
                    continue;
                }

                // Deadline reached without newer cursor motion or window change!
                let dispatch_target = if let Ok(mut p) = pending_bg.lock() {
                    if p.settled_and_dispatched || p.raw_class.is_empty() {
                        None
                    } else {
                        p.settled_and_dispatched = true;
                        Some((
                            p.raw_class.clone(),
                            p.raw_name.clone(),
                            p.raw_caption.clone(),
                        ))
                    }
                } else {
                    None
                };

                if let Some((effective_class, resource_name, raw_caption)) = dispatch_target {
                    let resolved_caption =
                        resolve_full_browser_caption(&effective_class, &raw_caption);
                    let mut matched_profile: Option<u8> = None;

                    if let Ok(mut st) = state_bg.lock() {
                        if !st.picker_armed && st.enabled && !st.paused_for_editing {
                            matched_profile =
                                st.match_profile_for_window(&effective_class, &resolved_caption);

                            if let Some(target) = matched_profile {
                                if send_hid_switch_profile(target) {
                                    st.last_switched_profile = target;
                                }
                            }
                        }
                    }

                    if let Err(e) = ui_tx_bg.send(WindowChangeEvent {
                        resource_class: effective_class,
                        resource_name,
                        caption: resolved_caption,
                        matched_profile,
                        picked: false,
                    }) {
                        crate::bridge::device_controller::record_system_event(
                            "WARN",
                            "KWIN",
                            format!("Failed sending settled window change event to UI: {e}"),
                        );
                    }
                }
            }
        });
    }

    let pending_check = Arc::clone(&pending);
    let service = KWinBridgeService {
        state,
        ui_event_tx,
        pending,
        settle_notify,
    };
    let conn = zbus::conn::Builder::session()?
        .name("io.github.duckydeck.Studio")?
        .serve_at("/KWinBridge", service)?
        .build()
        .await?;

    if let Ok(script_path) = write_kwin6_hook_script() {
        if let Ok(kwin_scripting) = zbus::Proxy::new(
            &conn,
            "org.kde.KWin",
            "/Scripting",
            "org.kde.kwin.Scripting",
        )
        .await
        {
            let plugin_name = "duckydeck_kwin6_autoswitcher";
            let fallback_name = format!("duckydeck_kwin6_autoswitcher_{}", std::process::id());

            for _attempt in 0..8 {
                {
                    let _guard = KWIN_SCRIPT_LOCK.lock().await;
                    let _: std::result::Result<bool, _> = kwin_scripting
                        .call("unloadScript", &("duckydeck_oled_keep_above_oneshot",))
                        .await;
                    let _: std::result::Result<bool, _> =
                        kwin_scripting.call("unloadScript", &(plugin_name,)).await;
                    let _: std::result::Result<bool, _> = kwin_scripting
                        .call("unloadScript", &(fallback_name.as_str(),))
                        .await;

                    let mut script_id: i32 = kwin_scripting
                        .call("loadScript", &(&script_path, plugin_name))
                        .await
                        .unwrap_or(-1);

                    if script_id < 0 {
                        script_id = kwin_scripting
                            .call("loadScript", &(&script_path, fallback_name.as_str()))
                            .await
                            .unwrap_or(-1);
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
                            if let Err(e) = kwin_script.call::<_, _, ()>("run", &()).await {
                                crate::bridge::device_controller::record_system_event(
                                    "WARN",
                                    "KWIN",
                                    format!("Failed running KWin auto-switcher script: {e}"),
                                );
                            }
                        }
                    }
                }

                // Wait up to 300ms for sendInitialTopWindow() to deliver the initial window over D-Bus.
                // If an external tool (such as kdotool) caused a transient KWin scripts.size() ID collision,
                // retry loading and running the hook script.
                let mut received_initial = false;
                for _ in 0..12 {
                    tokio::time::sleep(std::time::Duration::from_millis(25)).await;
                    if pending_check
                        .lock()
                        .map(|p| !p.raw_class.is_empty())
                        .unwrap_or(false)
                    {
                        received_initial = true;
                        break;
                    }
                }
                if received_initial {
                    break;
                }
            }
        }
    }

    std::future::pending::<()>().await;
    Ok(())
}

fn write_kwin6_hook_script() -> std::io::Result<String> {
    let initial_rect_js = match load_saved_oled_geometry_tuple() {
        Some((gx, gy, gw, gh)) => {
            format!("{{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }}")
        }
        None => "null".to_string(),
    };
    let js_code = format!(
        r#"
var trackedWindow = null;
var hookedWindows = [];
var lastCursorKey = "";
var lastMotionSentMs = 0;
var initialSavedOledRect = {initial_rect_js};

function isWindowHooked(win) {{
    for (var i = 0; i < hookedWindows.length; ++i) {{
        if (hookedWindows[i] === win) {{
            return true;
        }}
    }}
    return false;
}}

function isOledWidgetWindow(win) {{
    if (!win) return false;
    var cap = win.caption ? win.caption.toString() : "";
    return cap.indexOf("duckyPad OLED —") === 0;
}}

function isDuckyDeckMainWindow(win) {{
    if (!win || isOledWidgetWindow(win)) return false;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var name = (win.resourceName ? win.resourceName.toString() : "").toLowerCase();
    return cls.indexOf("duckydeck") !== -1 || name.indexOf("duckydeck") !== -1;
}}

function checkOledKeepAbove(win) {{
    if (!win) return;
    if (isOledWidgetWindow(win)) {{
        var cap = win.caption ? win.caption.toString() : "";
        var isPinned = (cap.indexOf("[Unpinned]") === -1);
        win.keepAbove = isPinned;
        win.skipTaskbar = isPinned;
        win.skipPager = isPinned;
        if (isPinned && win.minimized) {{
            win.minimized = false;
        }}
        if (!isWindowHooked(win)) {{
            hookedWindows.push(win);
            try {{
                if (initialSavedOledRect) {{
                    win.frameGeometry = {{
                        x: initialSavedOledRect.x,
                        y: initialSavedOledRect.y,
                        width: initialSavedOledRect.width,
                        height: initialSavedOledRect.height
                    }};
                    initialSavedOledRect = null;
                }}
                callDBus(
                    "io.github.duckydeck.Studio",
                    "/KWinBridge",
                    "io.github.duckydeck.KWinBridge",
                    "NotifyOledWidgetMapped",
                    isPinned
                );
                if (win.minimizedChanged) {{
                    win.minimizedChanged.connect(function() {{
                        var c = win.caption ? win.caption.toString() : "";
                        var pinnedNow = (c.indexOf("[Unpinned]") === -1);
                        if (pinnedNow && win.minimized) {{
                            win.minimized = false;
                            win.keepAbove = true;
                        }}
                    }});
                }}
                if (win.captionChanged) {{
                    win.captionChanged.connect(function() {{
                        checkOledKeepAbove(win);
                    }});
                }}
                if (win.frameGeometryChanged) {{
                    win.frameGeometryChanged.connect(function() {{
                        if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 180 && win.frameGeometry.height >= 220) {{
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
                        }}
                    }});
                }}
            }} catch (e) {{}}
        }}
        return;
    }}
"#
    ) + r#"
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

function isDuckyDeckWindow(win) {
    if (!win) return false;
    checkOledKeepAbove(win);
    if (isOledWidgetWindow(win) || isDuckyDeckMainWindow(win)) return true;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var name = (win.resourceName ? win.resourceName.toString() : "").toLowerCase();
    return cls.indexOf("duckydeck") !== -1 || name.indexOf("duckydeck") !== -1;
}

function isDesktopOrPanelWindow(win) {
    if (!win) return true;
    if (win.desktopWindow || win.dock) return true;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var name = (win.resourceName ? win.resourceName.toString() : "").toLowerCase();
    return cls === "plasmashell" || cls === "org.kde.plasmashell" || name === "plasmashell";
}

function effectiveWindowClass(win) {
    if (!win || isDesktopOrPanelWindow(win)) return "plasmashell";
    var cls = win.resourceClass ? win.resourceClass.toString().trim() : "";
    if (cls !== "") return cls;
    var name = win.resourceName ? win.resourceName.toString().trim() : "";
    if (name !== "") return name;
    var df = win.desktopFileName ? win.desktopFileName.toString().trim() : "";
    if (df !== "") return df;
    return "unmatched-window";
}

function disconnectTrackedWindow() {
    if (trackedWindow) {
        try {
            if (trackedWindow.captionChanged) {
                trackedWindow.captionChanged.disconnect(onTrackedCaptionChanged);
            }
        } catch (e) {}
        trackedWindow = null;
    }
}

function sendDesktop() {
    lastCursorKey = "plasmashell::Desktop";
    callDBus(
        "io.github.duckydeck.Studio",
        "/KWinBridge",
        "io.github.duckydeck.KWinBridge",
        "NotifyActiveWindow",
        "plasmashell",
        "plasmashell",
        "Desktop"
    );
}

function sendWindow(win) {
    if (!win || isDesktopOrPanelWindow(win)) {
        sendDesktop();
        return;
    }
    var cls = effectiveWindowClass(win);
    var rname = win.resourceName ? win.resourceName.toString() : cls;
    var cap = win.caption ? win.caption.toString() : "";
    lastCursorKey = cls + "::" + cap;
    callDBus(
        "io.github.duckydeck.Studio",
        "/KWinBridge",
        "io.github.duckydeck.KWinBridge",
        "NotifyActiveWindow",
        cls,
        rname,
        cap
    );
}

function onTrackedCaptionChanged() {
    if (workspace.activeWindow && isDuckyDeckWindow(workspace.activeWindow)) {
        return;
    }
    if (trackedWindow && !isDuckyDeckWindow(trackedWindow)) {
        sendWindow(trackedWindow);
    }
}

function onWindowActivated(win) {
    if (!win) {
        disconnectTrackedWindow();
        sendDesktop();
        return;
    }
    if (isDuckyDeckWindow(win)) {
        disconnectTrackedWindow();
        lastCursorKey = "__duckydeck__";
        return;
    }
    if (isDesktopOrPanelWindow(win)) {
        disconnectTrackedWindow();
        sendDesktop();
        return;
    }
    if (trackedWindow !== win) {
        if (trackedWindow) {
            disconnectTrackedWindow();
        }
        trackedWindow = win;
        try {
            if (trackedWindow.captionChanged) {
                trackedWindow.captionChanged.connect(onTrackedCaptionChanged);
            }
        } catch (e) {}
    }
    sendWindow(win);
}

function isWindowOnCurrentDesktop(w) {
    if (!w) return false;
    if (w.onAllDesktops || w.desktopWindow || w.dock) return true;
    if (!workspace.currentDesktop || !w.desktops || w.desktops.length === 0) return true;
    for (var d = 0; d < w.desktops.length; ++d) {
        if (w.desktops[d] === workspace.currentDesktop) {
            return true;
        }
    }
    return false;
}

function topWindowAtPoint(pt) {
    if (!pt) return null;
    var order = workspace.stackingOrder || (workspace.windowList ? workspace.windowList() : null);
    if (!order || !order.length) return null;
    for (var i = order.length - 1; i >= 0; --i) {
        var w = order[i];
        if (!w || w.minimized || w.hidden || w.deleted) continue;
        if (w.tooltip || w.notification || w.criticalNotification || w.onScreenDisplay || w.dndIcon || w.splash) continue;
        if (!isWindowOnCurrentDesktop(w)) continue;
        var fg = w.frameGeometry;
        if (!fg || fg.width <= 0 || fg.height <= 0) continue;
        if (pt.x >= fg.x && pt.x < (fg.x + fg.width) && pt.y >= fg.y && pt.y < (fg.y + fg.height)) {
            return w;
        }
    }
    return null;
}

function onCursorPosChanged() {
    var pt = workspace.cursorPos;
    if (!pt) return;
    var now = Date.now();
    var topWin = topWindowAtPoint(pt);
    if (!topWin || isDesktopOrPanelWindow(topWin)) {
        if (lastCursorKey !== "plasmashell::Desktop" || (now - lastMotionSentMs) >= 50) {
            lastMotionSentMs = now;
            disconnectTrackedWindow();
            sendDesktop();
        }
        return;
    }
    if (isDuckyDeckWindow(topWin)) {
        lastCursorKey = "__duckydeck__";
        return;
    }
    var cls = effectiveWindowClass(topWin);
    var cap = topWin.caption ? topWin.caption.toString() : "";
    var key = cls + "::" + cap;
    if (lastCursorKey !== key || (now - lastMotionSentMs) >= 50) {
        lastMotionSentMs = now;
        onWindowActivated(topWin);
    }
}

function sendInitialTopWindow() {
    var order = workspace.stackingOrder || (workspace.windowList ? workspace.windowList() : null);
    if (order && order.length) {
        for (var j = 0; j < order.length; ++j) {
            checkOledKeepAbove(order[j]);
        }
    }
    if (workspace.cursorPos) {
        var underCursor = topWindowAtPoint(workspace.cursorPos);
        if (!underCursor || isDesktopOrPanelWindow(underCursor)) {
            sendDesktop();
            return;
        }
        if (!isDuckyDeckWindow(underCursor)) {
            onWindowActivated(underCursor);
            return;
        }
    }
    if (workspace.activeWindow && !isDuckyDeckWindow(workspace.activeWindow)) {
        onWindowActivated(workspace.activeWindow);
        return;
    }
    if (order && order.length) {
        for (var i = order.length - 1; i >= 0; --i) {
            var w = order[i];
            if (!isDuckyDeckWindow(w) && (!w.minimized)) {
                onWindowActivated(w);
                return;
            }
        }
    }
    sendDesktop();
}

if (workspace.windowAdded) {
    workspace.windowAdded.connect(checkOledKeepAbove);
}
if (workspace.cursorPosChanged) {
    workspace.cursorPosChanged.connect(onCursorPosChanged);
}
if (workspace.windowActivated) {
    workspace.windowActivated.connect(onWindowActivated);
    sendInitialTopWindow();
} else if (workspace.clientActivated) {
    workspace.clientActivated.connect(onWindowActivated);
    onWindowActivated(workspace.activeClient);
}
"#;

    let file_path = crate::paths::kwin_runtime_dir()
        .join("duckydeck-kwin6-hook.js")
        .to_string_lossy()
        .to_string();

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(js_code.as_bytes())?;
    Ok(file_path)
}

fn load_saved_oled_geometry_tuple() -> Option<(i32, i32, i32, i32)> {
    let path = crate::paths::config_dir().join("oled_widget_geometry.json");
    let raw = fs::read_to_string(path).ok()?;
    let val: serde_json::Value = serde_json::from_str(&raw).ok()?;
    if !val
        .get("has_position")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return None;
    }
    let x = val.get("x").and_then(|v| v.as_i64())? as i32;
    let y = val.get("y").and_then(|v| v.as_i64())? as i32;
    let w = (val.get("width").and_then(|v| v.as_i64()).unwrap_or(284) as i32).clamp(184, 1600);
    let h = (val.get("height").and_then(|v| v.as_i64()).unwrap_or(356) as i32).clamp(240, 1600);
    Some((x, y, w, h))
}

static KWIN_SCRIPT_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

pub fn apply_kwin_oled_keep_above(keep_above: bool, restore_geometry: bool) {
    std::thread::spawn(move || {
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

            let script_path = crate::paths::kwin_runtime_dir()
                .join("duckydeck-kwin6-oled-pin.js")
                .to_string_lossy()
                .to_string();
            let flag_str = if keep_above { "true" } else { "false" };
            let geom_js = if let Some((gx, gy, gw, gh)) = geom_opt {
                format!("w.frameGeometry = {{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }};")
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
            let plugin_name = "duckydeck_oled_keep_above_oneshot";
            let delays_ms: &[u64] = if geom_opt.is_some() {
                &[0, 60, 180, 420]
            } else {
                &[0]
            };
            for &delay in delays_ms {
                if delay > 0 {
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
                let _guard = KWIN_SCRIPT_LOCK.lock().await;
                if std::fs::write(&script_path, js.as_bytes()).is_err() {
                    return;
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
                            if let Err(e) = script_proxy.call::<_, _, ()>("run", &()).await {
                                crate::bridge::device_controller::record_system_event(
                                    "WARN",
                                    "KWIN",
                                    format!("Failed running KWin OLED keep-above script: {e}"),
                                );
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                        let _: std::result::Result<bool, _> =
                            kwin_scripting.call("unloadScript", &(plugin_name,)).await;
                    }
                }
            }
        });
    });
}
