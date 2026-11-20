use serde::{Deserialize, Serialize};

pub mod compiler;

pub const WEB_API_META_PREFIX: &str = "// @WEB_API ";
pub const APP_LAUNCH_META_PREFIX: &str = "// @DUCKYDECK_APP_LAUNCH: ";

pub fn default_web_api_method() -> String {
    "POST".to_string()
}

pub fn default_web_api_oled() -> String {
    String::new()
}

pub fn default_web_api_led_feedback() -> bool {
    true
}

pub fn default_web_api_trigger_mode() -> String {
    "pgv".to_string()
}

pub fn default_web_api_fkey() -> String {
    "F13".to_string()
}

pub fn default_fkey_for_slot(slot_num: usize) -> String {
    let idx = ((slot_num.saturating_sub(1)) % 12) + 13;
    format!("F{idx}")
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebApiConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub destination: String,
    #[serde(default = "default_web_api_method")]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: String,
    #[serde(default)]
    pub body: String,
    #[serde(default = "default_web_api_oled")]
    pub oled_text: String,
    #[serde(default = "default_web_api_led_feedback")]
    pub led_feedback: bool,
    #[serde(default = "default_web_api_trigger_mode")]
    pub trigger_mode: String,
    #[serde(default = "default_web_api_fkey")]
    pub fkey: String,
}

impl Default for WebApiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            destination: String::new(),
            method: default_web_api_method(),
            url: String::new(),
            headers: String::new(),
            body: String::new(),
            oled_text: String::new(),
            led_feedback: true,
            trigger_mode: default_web_api_trigger_mode(),
            fkey: default_web_api_fkey(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppLaunchConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub app_name: String,
    #[serde(default)]
    pub desktop_id: String,
    #[serde(default)]
    pub command: String,
    #[serde(default)]
    pub args: String,
    #[serde(default)]
    pub working_dir: String,
    #[serde(default)]
    pub run_in_terminal: bool,
    #[serde(default)]
    pub oled_text: String,
    #[serde(default = "default_web_api_led_feedback")]
    pub led_feedback: bool,
    #[serde(default = "default_web_api_trigger_mode")]
    pub trigger_mode: String,
    #[serde(default = "default_web_api_fkey")]
    pub fkey: String,
}

impl Default for AppLaunchConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            app_name: String::new(),
            desktop_id: String::new(),
            command: String::new(),
            args: String::new(),
            working_dir: String::new(),
            run_in_terminal: false,
            oled_text: String::new(),
            led_feedback: true,
            trigger_mode: default_web_api_trigger_mode(),
            fkey: default_web_api_fkey(),
        }
    }
}

pub fn extract_web_api_config(script: &str, slot_num: usize) -> WebApiConfig {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(WEB_API_META_PREFIX) {
            if let Ok(mut cfg) = serde_json::from_str::<WebApiConfig>(json_part.trim()) {
                cfg.method = cfg.method.trim().to_ascii_uppercase();
                if cfg.method.is_empty() {
                    cfg.method = "POST".to_string();
                }
                if cfg.trigger_mode != "fkey" {
                    cfg.trigger_mode = "pgv".to_string();
                }
                if cfg.fkey.trim().is_empty() {
                    cfg.fkey = default_fkey_for_slot(slot_num);
                }
                return cfg;
            }
        }
    }
    WebApiConfig {
        fkey: default_fkey_for_slot(slot_num),
        ..Default::default()
    }
}

pub fn extract_app_launch_config(script: &str, slot_num: usize) -> AppLaunchConfig {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(APP_LAUNCH_META_PREFIX) {
            if let Ok(mut cfg) = serde_json::from_str::<AppLaunchConfig>(json_part.trim()) {
                if !matches!(
                    cfg.trigger_mode.as_str(),
                    "pgv" | "fkey" | "krunner" | "guir"
                ) {
                    cfg.trigger_mode = "pgv".to_string();
                }
                if cfg.fkey.trim().is_empty() {
                    cfg.fkey = default_fkey_for_slot(slot_num);
                }
                return cfg;
            }
        }
    }
    AppLaunchConfig {
        fkey: default_fkey_for_slot(slot_num),
        ..Default::default()
    }
}
