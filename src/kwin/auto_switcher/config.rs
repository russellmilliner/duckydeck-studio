use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use super::{
    build_auto_aligned_rules, default_modifier_hold_delay_ms, default_modifier_oneshot_timeout_sec,
    AutoSwitchRule, ModifierLayerRule,
};
use crate::hardware::storage::DuckyProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HidSwitchOutcome {
    Switched,
    NotLoadedOnHardware,
    DeviceUnavailable,
}

#[derive(Debug, Clone)]
pub struct WindowChangeEvent {
    pub resource_class: String,
    pub resource_name: String,
    pub caption: String,
    pub matched_profile: Option<u8>,
    pub picked: bool,
    pub hid_rejected_not_on_device: bool,
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

pub fn default_multi_click_interval_ms() -> i32 {
    250
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
    crate::paths::fill_os_random_bytes(&mut buf)
        .expect("OS CSPRNG unavailable for security token generation");
    let mut hex = String::with_capacity(44);
    hex.push_str("ddk_");
    for b in buf {
        use std::fmt::Write as _;
        let _ = write!(&mut hex, "{b:02x}");
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
    #[serde(default)]
    pub auto_switch_unfocused_hover: bool,
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
    pub saved_color_names: std::collections::HashMap<String, String>,
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
    #[serde(default = "default_true")]
    pub modifier_switch_enabled: bool,
    #[serde(default = "default_true")]
    pub modifier_section_expanded: bool,
    #[serde(default)]
    pub modifier_rules: Vec<ModifierLayerRule>,
    #[serde(default)]
    pub google_search_shortcuts_permitted: bool,
    #[serde(default = "default_multi_click_interval_ms")]
    pub multi_click_interval_ms: i32,
    #[serde(default = "default_true")]
    pub check_updates_on_startup: bool,
}

#[derive(Clone)]
pub struct SwitcherState {
    pub enabled: bool,
    pub paused_for_editing: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    pub auto_switch_unfocused_hover: bool,
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
    pub saved_color_names: std::collections::HashMap<String, String>,
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
    pub modifier_switch_enabled: bool,
    pub modifier_section_expanded: bool,
    pub modifier_rules: Vec<ModifierLayerRule>,
    pub active_modifier_override: Option<(u8, String, String)>,
    pub last_window_matched_profile: Option<u8>,
    pub current_window_class: String,
    pub current_window_title: String,
    pub google_search_shortcuts_permitted: bool,
    pub multi_click_interval_ms: i32,
    pub check_updates_on_startup: bool,
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
                auto_switch_unfocused_hover: cfg.auto_switch_unfocused_hover,
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
                saved_color_names: cfg.saved_color_names,
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
                modifier_switch_enabled: cfg.modifier_switch_enabled,
                modifier_section_expanded: cfg.modifier_section_expanded,
                modifier_rules: cfg.modifier_rules,
                active_modifier_override: None,
                last_window_matched_profile: None,
                current_window_class: String::new(),
                current_window_title: String::new(),
                google_search_shortcuts_permitted: cfg.google_search_shortcuts_permitted,
                multi_click_interval_ms: cfg.multi_click_interval_ms.clamp(80, 1000),
                check_updates_on_startup: cfg.check_updates_on_startup,
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
            auto_switch_unfocused_hover: false,
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
            saved_color_names: std::collections::HashMap::new(),
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
            modifier_switch_enabled: true,
            modifier_section_expanded: true,
            modifier_rules: Vec::new(),
            active_modifier_override: None,
            last_window_matched_profile: None,
            current_window_class: String::new(),
            current_window_title: String::new(),
            google_search_shortcuts_permitted: false,
            multi_click_interval_ms: default_multi_click_interval_ms(),
            check_updates_on_startup: true,
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
pub struct SdPersistedModifierRule {
    pub enabled: bool,
    pub trigger_key: String,
    pub mode: String,
    #[serde(default = "default_modifier_hold_delay_ms")]
    pub hold_delay_ms: i32,
    #[serde(default = "default_modifier_oneshot_timeout_sec")]
    pub oneshot_timeout_sec: i32,
    pub target_profile: u8,
    #[serde(default)]
    pub target_profile_name: String,
    #[serde(default)]
    pub resource_class: String,
    #[serde(default)]
    pub title_contains: Option<String>,
    #[serde(default)]
    pub title_is_regex: bool,
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
    #[serde(default = "default_true")]
    pub modifier_switch_enabled: bool,
    #[serde(default)]
    pub modifier_rules: Vec<SdPersistedModifierRule>,
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

    let (mod_enabled, mod_rules) = SwitcherState::load_config_from_disk()
        .map(|cfg| {
            let m_rules: Vec<SdPersistedModifierRule> = cfg
                .modifier_rules
                .into_iter()
                .map(|mr| {
                    let p_name = profiles
                        .iter()
                        .find(|p| p.number == mr.target_profile)
                        .map(|p| p.name.clone())
                        .unwrap_or_default();
                    SdPersistedModifierRule {
                        enabled: mr.enabled,
                        trigger_key: mr.trigger_key,
                        mode: mr.mode,
                        hold_delay_ms: mr.hold_delay_ms,
                        oneshot_timeout_sec: mr.oneshot_timeout_sec,
                        target_profile: mr.target_profile,
                        target_profile_name: p_name,
                        resource_class: mr.resource_class,
                        title_contains: mr.title_contains,
                        title_is_regex: mr.title_is_regex,
                    }
                })
                .collect();
            (cfg.modifier_switch_enabled, m_rules)
        })
        .unwrap_or((true, Vec::new()));

    let bundle = SdPersistedRulesBundle {
        version: 1,
        enabled,
        default_profile: Some(fb_num),
        default_profile_name: fb_name,
        rules: sd_rules,
        modifier_switch_enabled: mod_enabled,
        modifier_rules: mod_rules,
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

pub fn save_device_rules_for_serial(
    serial: &str,
    enabled: bool,
    default_profile: Option<u8>,
    rules: &[AutoSwitchRule],
    profiles: &[DuckyProfile],
    sd_root: Option<&Path>,
) {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if !clean.is_empty() {
        let dev_dir = crate::hardware::profile::profile_cache_dir_for_serial(&clean);
        let _ = save_rules_bundle_to_dir(&dev_dir, enabled, default_profile, rules, profiles);
    }
    if let Some(sd) = sd_root {
        if sd.is_dir() {
            let _ = save_rules_bundle_to_dir(sd, enabled, default_profile, rules, profiles);
        }
    }
}

pub fn load_device_rules_for_serial(
    serial: &str,
    profiles: &[DuckyProfile],
    sd_root: Option<&Path>,
) -> (bool, Option<u8>, Vec<AutoSwitchRule>) {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if !clean.is_empty() {
        for dev_dir in [
            crate::hardware::profile::profile_cache_dir_for_serial(&clean),
            crate::paths::host_data_dir()
                .join("device_profiles")
                .join(&clean),
        ] {
            if let Some(loaded) = load_rules_bundle_from_dir(&dev_dir, profiles) {
                return loaded;
            }
        }
    }
    if let Some(sd) = sd_root {
        if let Some(loaded) = load_rules_bundle_from_dir(sd, profiles) {
            if !clean.is_empty() {
                save_device_rules_for_serial(
                    &clean,
                    loaded.0,
                    loaded.1,
                    &loaded.2,
                    profiles,
                    Some(sd),
                );
            }
            return loaded;
        }
    }
    if clean.is_empty() || !crate::hardware::profile::has_other_cached_devices(&clean) {
        if let Some(cfg) = SwitcherState::load_config_from_disk() {
            let fb = Some(cfg.default_profile.unwrap_or(1).max(1));
            if !clean.is_empty() {
                save_device_rules_for_serial(
                    &clean,
                    cfg.enabled,
                    fb,
                    &cfg.rules,
                    profiles,
                    sd_root,
                );
            }
            return (cfg.enabled, fb, cfg.rules);
        }
        if let Some(loaded) =
            load_rules_bundle_from_dir(&crate::hardware::profile::profile_cache_dir(), profiles)
        {
            return loaded;
        }
    }
    let aligned = build_auto_aligned_rules(profiles);
    let default_prof = profiles.first().map(|p| p.number).or(Some(1));
    if !clean.is_empty() {
        save_device_rules_for_serial(&clean, true, default_prof, &aligned, profiles, sd_root);
    }
    (true, default_prof, aligned)
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
                        let _ = crate::paths::write_private_file_atomic(&path, json.as_bytes());
                        // ignore: best-effort sync to Flatpak config
                    }
                    Some(h_cfg)
                } else {
                    Some(p_cfg)
                }
            }
            (None, Some(h_cfg)) => {
                if let Ok(json) = serde_json::to_string_pretty(&h_cfg) {
                    let _ = crate::paths::write_private_file_atomic(&path, json.as_bytes());
                    // ignore: best-effort sync to Flatpak config
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
            auto_switch_unfocused_hover: self.auto_switch_unfocused_hover,
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
            saved_color_names: self.saved_color_names.clone(),
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
            modifier_switch_enabled: self.modifier_switch_enabled,
            modifier_section_expanded: self.modifier_section_expanded,
            modifier_rules: self.modifier_rules.clone(),
            google_search_shortcuts_permitted: self.google_search_shortcuts_permitted,
            multi_click_interval_ms: self.multi_click_interval_ms.clamp(80, 1000),
            check_updates_on_startup: self.check_updates_on_startup,
        };
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("Failed serializing KWin rules: {e}"))?;
        crate::paths::write_private_file_atomic(&path, json.as_bytes())
            .map_err(|e| format!("Failed writing {}: {e}", path.display()))?;

        if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() != Ok("1") {
            let host_path = Self::host_config_file_path();
            if host_path != path {
                let _ = crate::paths::write_private_file_atomic(&host_path, json.as_bytes());
                // ignore: best-effort host mirror
            }
        }
        Ok(())
    }
}
