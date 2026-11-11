use serde::{Deserialize, Serialize};
use std::fs;
#[cfg(not(target_os = "windows"))]
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
#[cfg(not(target_os = "windows"))]
use zbus::{interface, Result as ZbusResult};
#[cfg(target_os = "windows")]
pub type ZbusResult<T> = std::result::Result<T, String>;

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

pub fn default_modifier_hold_delay_ms() -> i32 {
    200
}

pub fn default_modifier_oneshot_timeout_sec() -> i32 {
    6
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModifierLayerRule {
    pub enabled: bool,
    pub trigger_key: String,
    pub mode: String,
    #[serde(default = "default_modifier_hold_delay_ms")]
    pub hold_delay_ms: i32,
    #[serde(default = "default_modifier_oneshot_timeout_sec")]
    pub oneshot_timeout_sec: i32,
    pub target_profile: u8,
    #[serde(default)]
    pub resource_class: String,
    #[serde(default)]
    pub title_contains: Option<String>,
    #[serde(default)]
    pub title_is_regex: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierLayerTransition {
    Activated {
        target_profile: u8,
        trigger_key: String,
        mode: String,
    },
    Deactivated,
}

pub fn normalize_modifier_trigger_key(raw: &str) -> String {
    match raw.trim().to_ascii_uppercase().as_str() {
        "MENU" | "APPS" | "CONTEXT" | "COMPOSE" => "MENU".to_string(),
        "RCTRL" | "RIGHT_CTRL" | "RIGHTCTRL" => "RCTRL".to_string(),
        "RALT" | "RIGHT_ALT" | "RIGHTALT" | "ALTGR" => "RALT".to_string(),
        "RSHIFT" | "RIGHT_SHIFT" | "RIGHTSHIFT" => "RSHIFT".to_string(),
        "LCTRL" | "LEFT_CTRL" | "LEFTCTRL" | "CTRL" => "LCTRL".to_string(),
        "LALT" | "LEFT_ALT" | "LEFTALT" | "ALT" => "LALT".to_string(),
        "LSHIFT" | "LEFT_SHIFT" | "LEFTSHIFT" | "SHIFT" => "LSHIFT".to_string(),
        "SUPER" | "META" | "WIN" | "LWIN" | "RWIN" => "SUPER".to_string(),
        "CAPSLOCK" | "CAPS" | "CAPS_LOCK" => "CAPSLOCK".to_string(),
        "SCROLLLOCK" | "SCROLL_LOCK" | "SCRLK" => "SCROLLLOCK".to_string(),
        "PAUSE" | "BREAK" | "PAUSE_BREAK" => "PAUSE".to_string(),
        "FN" | "FUNCTION" => "FN".to_string(),
        "CTRL_ALT" | "CTRL+ALT" | "CTRL-ALT" => "CTRL_ALT".to_string(),
        "CTRL_SUPER" | "CTRL+SUPER" | "CTRL-SUPER" | "CTRL_META" => "CTRL_SUPER".to_string(),
        "RCTRL_RSHIFT"
        | "RCTRL+RSHIFT"
        | "RCTRL-RSHIFT"
        | "RIGHTCTRL_RIGHTSHIFT"
        | "RIGHTCTRL+RIGHTSHIFT"
        | "RIGHTCTRL-RIGHTSHIFT"
        | "RIGHT_CTRL_RIGHT_SHIFT"
        | "RIGHT_CTRL+RIGHT_SHIFT"
        | "RIGHT_CTRL-RIGHT_SHIFT" => "RCTRL_RSHIFT".to_string(),
        "MENU_RSHIFT"
        | "MENU+RSHIFT"
        | "MENU-RSHIFT"
        | "MENU_RIGHTSHIFT"
        | "MENU+RIGHTSHIFT"
        | "MENU-RIGHTSHIFT"
        | "MENU_RIGHT_SHIFT"
        | "MENU+RIGHT_SHIFT"
        | "MENU-RIGHT_SHIFT" => "MENU_RSHIFT".to_string(),
        "F13" => "F13".to_string(),
        "F14" => "F14".to_string(),
        "F15" => "F15".to_string(),
        "F16" => "F16".to_string(),
        "F17" => "F17".to_string(),
        "F18" => "F18".to_string(),
        "F19" => "F19".to_string(),
        "F20" => "F20".to_string(),
        "F21" => "F21".to_string(),
        "F22" => "F22".to_string(),
        "F23" => "F23".to_string(),
        "F24" => "F24".to_string(),
        _ => "MENU".to_string(),
    }
}

pub fn modifier_trigger_key_label(key: &str) -> &'static str {
    match normalize_modifier_trigger_key(key).as_str() {
        "MENU" => "Menu / App Key",
        "RCTRL" => "Right Ctrl",
        "RALT" => "Right Alt (AltGr)",
        "RSHIFT" => "Right Shift",
        "LCTRL" => "Left Ctrl",
        "LALT" => "Left Alt",
        "LSHIFT" => "Left Shift",
        "SUPER" => "Super / Meta (Win)",
        "CAPSLOCK" => "Caps Lock",
        "SCROLLLOCK" => "Scroll Lock",
        "PAUSE" => "Pause / Break",
        "FN" => "Fn (Function Key)",
        "CTRL_ALT" => "Ctrl + Alt Chord",
        "CTRL_SUPER" => "Ctrl + Super Chord",
        "RCTRL_RSHIFT" => "Right Ctrl + Right Shift Chord",
        "MENU_RSHIFT" => "Menu + Right Shift Chord",
        "F13" => "F13",
        "F14" => "F14",
        "F15" => "F15",
        "F16" => "F16",
        "F17" => "F17",
        "F18" => "F18",
        "F19" => "F19",
        "F20" => "F20",
        "F21" => "F21",
        "F22" => "F22",
        "F23" => "F23",
        "F24" => "F24",
        _ => "Menu / App Key",
    }
}

pub fn normalize_modifier_mode(raw: &str) -> String {
    match raw.trim().to_ascii_lowercase().as_str() {
        "hold" | "momentary" | "mo" => "hold".to_string(),
        "oneshot" | "one_shot" | "one-shot" | "osm" | "tap" => "oneshot".to_string(),
        "double_tap_hold" | "double-tap-hold" | "dth" => "double_tap_hold".to_string(),
        "toggle" | "tg" | "lock" => "toggle".to_string(),
        _ => "hold".to_string(),
    }
}

pub fn modifier_mode_label(mode: &str) -> &'static str {
    match normalize_modifier_mode(mode).as_str() {
        "hold" => "Hold (Momentary Shift)",
        "oneshot" => "One-Shot Tap (1 Key / Timeout)",
        "double_tap_hold" => "Double-Tap & Hold",
        "toggle" => "Toggle Lock",
        _ => "Hold (Momentary Shift)",
    }
}

pub fn trigger_key_requires_double_tap_for_toggle(trigger_key: &str) -> bool {
    matches!(
        normalize_modifier_trigger_key(trigger_key).as_str(),
        "LCTRL" | "RCTRL" | "LALT" | "RALT" | "LSHIFT" | "RSHIFT" | "SUPER" | "CAPSLOCK"
    )
}

pub fn trigger_key_evdev_codes(trigger_key: &str) -> &'static [u16] {
    match trigger_key {
        "MENU" => &[127, 139],
        "RCTRL" => &[97],
        "RALT" => &[100],
        "RSHIFT" => &[54],
        "LCTRL" => &[29],
        "LALT" => &[56],
        "LSHIFT" => &[42],
        "SUPER" => &[125, 126],
        "CAPSLOCK" => &[58],
        "SCROLLLOCK" => &[70],
        "PAUSE" => &[119],
        "FN" => &[464],
        "CTRL_ALT" => &[29, 97, 56, 100],
        "CTRL_SUPER" => &[29, 97, 125, 126],
        "RCTRL_RSHIFT" => &[97, 54],
        "MENU_RSHIFT" => &[54, 127, 139],
        "F13" => &[183],
        "F14" => &[184],
        "F15" => &[185],
        "F16" => &[186],
        "F17" => &[187],
        "F18" => &[188],
        "F19" => &[189],
        "F20" => &[190],
        "F21" => &[191],
        "F22" => &[192],
        "F23" => &[193],
        "F24" => &[194],
        _ => &[127, 139],
    }
}

pub fn is_modifier_trigger_pressed(
    trigger_key: &str,
    held_keys: &std::collections::HashSet<u16>,
) -> bool {
    match trigger_key {
        "CTRL_ALT" => {
            (held_keys.contains(&29) || held_keys.contains(&97))
                && (held_keys.contains(&56) || held_keys.contains(&100))
        }
        "CTRL_SUPER" => {
            (held_keys.contains(&29) || held_keys.contains(&97))
                && (held_keys.contains(&125) || held_keys.contains(&126))
        }
        "RCTRL_RSHIFT" => held_keys.contains(&97) && held_keys.contains(&54),
        "MENU_RSHIFT" => {
            (held_keys.contains(&127) || held_keys.contains(&139)) && held_keys.contains(&54)
        }
        other => trigger_key_evdev_codes(other)
            .iter()
            .any(|code| held_keys.contains(code)),
    }
}

pub fn has_non_trigger_key_interference(
    trigger_key: &str,
    held_keys: &std::collections::HashSet<u16>,
    newly_pressed_keys: &[u16],
) -> bool {
    let allowed = trigger_key_evdev_codes(trigger_key);
    held_keys.iter().any(|k| !allowed.contains(k))
        || newly_pressed_keys.iter().any(|k| !allowed.contains(k))
}

pub fn modifier_rule_specificity_for_window(
    rule: &ModifierLayerRule,
    active_window_class: &str,
    active_window_title: &str,
) -> Option<usize> {
    if !rule.enabled {
        return None;
    }
    let rule_class = rule.resource_class.trim();
    let is_global_class = rule_class.is_empty()
        || rule_class == "*"
        || rule_class.eq_ignore_ascii_case("any")
        || rule_class.eq_ignore_ascii_case("all");

    let class_score = if is_global_class {
        0usize
    } else {
        let lower_active = active_window_class.trim().to_ascii_lowercase();
        if lower_active.is_empty()
            || lower_active == "none"
            || lower_active == "listening for kwin..."
        {
            return None;
        }
        if !rule_matches_window_class(rule_class, &lower_active) {
            return None;
        }
        if is_any_browser_wildcard(rule_class) {
            1000usize
        } else {
            2000usize
        }
    };

    let rule_title = rule
        .title_contains
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let title_score = if let Some(sub) = rule_title {
        let resolved_caption =
            resolve_full_browser_caption(active_window_class, active_window_title);
        if resolved_caption.trim().is_empty() {
            return None;
        }
        let caption_lower = resolved_caption.to_lowercase();
        let matched_len = rule_matches_window_title(
            sub,
            rule.title_is_regex,
            &resolved_caption,
            &caption_lower,
        )?;
        4000usize + matched_len.min(999)
    } else {
        0usize
    };

    Some(class_score + title_score)
}

pub fn resolve_effective_modifier_rules_for_window(
    rules: &[ModifierLayerRule],
    active_window_class: &str,
    active_window_title: &str,
) -> Vec<ModifierLayerRule> {
    let mut scored: Vec<(String, String, usize, &ModifierLayerRule)> = Vec::new();
    let mut max_spec_per_trig: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for rule in rules {
        let Some(spec) =
            modifier_rule_specificity_for_window(rule, active_window_class, active_window_title)
        else {
            continue;
        };
        let trig = normalize_modifier_trigger_key(&rule.trigger_key);
        let mode = normalize_modifier_mode(&rule.mode);
        max_spec_per_trig
            .entry(trig.clone())
            .and_modify(|cur| {
                if spec > *cur {
                    *cur = spec;
                }
            })
            .or_insert(spec);
        scored.push((trig, mode, spec, rule));
    }

    let mut effective = Vec::new();
    let mut seen_trig_mode: std::collections::HashSet<(String, String)> =
        std::collections::HashSet::new();

    for (trig, mode, spec, rule) in scored {
        if max_spec_per_trig.get(&trig).copied() == Some(spec)
            && seen_trig_mode.insert((trig, mode))
        {
            effective.push(rule.clone());
        }
    }
    effective
}

#[derive(Debug, Clone, Default)]
struct ModifierRuleRuntimeState {
    was_pressed: bool,
    pressed_since_ms: Option<u64>,
    aborted_by_combo: bool,
    last_clean_tap_release_ms: Option<u64>,
    is_second_tap_hold: bool,
}

#[derive(Debug, Clone, Default)]
pub struct HostKeyboardModifierEngine {
    per_rule: std::collections::HashMap<(String, String), ModifierRuleRuntimeState>,
    active_rule_key: Option<(String, String)>,
    active_target_profile: Option<u8>,
    oneshot_activated_ms: u64,
    oneshot_expires_ms: u64,
    oneshot_saw_duckypad_keydown: bool,
}

impl HostKeyboardModifierEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active_override(&self) -> Option<(u8, String, String)> {
        let (ref trig, ref mode) = self.active_rule_key.as_ref()?;
        let prof = self.active_target_profile?;
        Some((prof, trig.clone(), mode.clone()))
    }

    pub fn force_deactivate(&mut self) -> Option<ModifierLayerTransition> {
        if self.active_rule_key.is_some() {
            self.active_rule_key = None;
            self.active_target_profile = None;
            self.oneshot_activated_ms = 0;
            self.oneshot_expires_ms = 0;
            self.oneshot_saw_duckypad_keydown = false;
            return Some(ModifierLayerTransition::Deactivated);
        }
        None
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_tick(
        &mut self,
        modifier_switch_enabled: bool,
        paused_for_editing: bool,
        rules: &[ModifierLayerRule],
        held_keys: &std::collections::HashSet<u16>,
        newly_pressed_keys: &[u16],
        duckypad_keys_held: u32,
        last_duckypad_keydown_ms: u64,
        now_ms: u64,
    ) -> Option<ModifierLayerTransition> {
        self.evaluate_tick_for_window(
            modifier_switch_enabled,
            paused_for_editing,
            rules,
            "",
            "",
            held_keys,
            newly_pressed_keys,
            duckypad_keys_held,
            last_duckypad_keydown_ms,
            now_ms,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate_tick_for_window(
        &mut self,
        modifier_switch_enabled: bool,
        paused_for_editing: bool,
        rules: &[ModifierLayerRule],
        active_window_class: &str,
        active_window_title: &str,
        held_keys: &std::collections::HashSet<u16>,
        newly_pressed_keys: &[u16],
        duckypad_keys_held: u32,
        last_duckypad_keydown_ms: u64,
        now_ms: u64,
    ) -> Option<ModifierLayerTransition> {
        if !modifier_switch_enabled || paused_for_editing {
            self.per_rule.clear();
            return self.force_deactivate();
        }

        let effective_rules = resolve_effective_modifier_rules_for_window(
            rules,
            active_window_class,
            active_window_title,
        );

        let mut transition: Option<ModifierLayerTransition> = None;

        // 1. Check active oneshot expiration or 1-key completion on duckyPad (or window scope change)
        if let Some((ref act_trig, ref act_mode)) = self.active_rule_key.clone() {
            let matching_active = effective_rules.iter().find(|r| {
                r.enabled
                    && normalize_modifier_trigger_key(&r.trigger_key) == *act_trig
                    && normalize_modifier_mode(&r.mode) == *act_mode
            });
            let Some(matched_rule) = matching_active else {
                return self.force_deactivate();
            };

            if self.active_target_profile != Some(matched_rule.target_profile) {
                self.active_target_profile = Some(matched_rule.target_profile);
                transition = Some(ModifierLayerTransition::Activated {
                    target_profile: matched_rule.target_profile,
                    trigger_key: act_trig.clone(),
                    mode: act_mode.clone(),
                });
            }

            if act_mode == "oneshot" {
                if last_duckypad_keydown_ms > self.oneshot_activated_ms || duckypad_keys_held > 0 {
                    self.oneshot_saw_duckypad_keydown = true;
                }
                if (self.oneshot_saw_duckypad_keydown && duckypad_keys_held == 0)
                    || (self.oneshot_expires_ms > 0 && now_ms >= self.oneshot_expires_ms)
                {
                    transition = self.force_deactivate();
                }
            }
        }

        // 2. Evaluate each effective rule for the active window in priority order
        for rule in &effective_rules {
            if !rule.enabled {
                continue;
            }
            let trig = normalize_modifier_trigger_key(&rule.trigger_key);
            let mode = normalize_modifier_mode(&rule.mode);
            let key_id = (trig.clone(), mode.clone());
            let is_pressed = is_modifier_trigger_pressed(&trig, held_keys);
            let combo_interference =
                has_non_trigger_key_interference(&trig, held_keys, newly_pressed_keys);
            let hold_delay = rule.hold_delay_ms.clamp(0, 2000) as u64;
            let oneshot_timeout_ms =
                (rule.oneshot_timeout_sec.clamp(1, 60) as u64).saturating_mul(1000);

            let rt = self.per_rule.entry(key_id.clone()).or_default();

            if is_pressed {
                if !rt.was_pressed {
                    rt.was_pressed = true;
                    rt.pressed_since_ms = Some(now_ms);
                    rt.aborted_by_combo = combo_interference;
                    rt.is_second_tap_hold = rt
                        .last_clean_tap_release_ms
                        .map(|rel_ms| now_ms.saturating_sub(rel_ms) <= 400)
                        .unwrap_or(false);
                } else if combo_interference {
                    rt.aborted_by_combo = true;
                }

                let held_ms = rt
                    .pressed_since_ms
                    .map(|t| now_ms.saturating_sub(t))
                    .unwrap_or(0);

                match mode.as_str() {
                    "hold" => {
                        let is_this_active = self.active_rule_key.as_ref() == Some(&key_id);
                        if !is_this_active && !rt.aborted_by_combo && held_ms >= hold_delay {
                            self.active_rule_key = Some(key_id);
                            self.active_target_profile = Some(rule.target_profile);
                            return Some(ModifierLayerTransition::Activated {
                                target_profile: rule.target_profile,
                                trigger_key: trig,
                                mode,
                            });
                        }
                    }
                    "double_tap_hold" => {
                        let is_this_active = self.active_rule_key.as_ref() == Some(&key_id);
                        if !is_this_active
                            && !rt.aborted_by_combo
                            && rt.is_second_tap_hold
                            && held_ms >= 50
                        {
                            self.active_rule_key = Some(key_id);
                            self.active_target_profile = Some(rule.target_profile);
                            return Some(ModifierLayerTransition::Activated {
                                target_profile: rule.target_profile,
                                trigger_key: trig,
                                mode,
                            });
                        }
                    }
                    _ => {}
                }
            } else if rt.was_pressed {
                let held_ms = rt
                    .pressed_since_ms
                    .map(|t| now_ms.saturating_sub(t))
                    .unwrap_or(0);
                let was_clean_tap = !rt.aborted_by_combo && held_ms <= 400;
                let prev_tap_ms = rt.last_clean_tap_release_ms;

                rt.was_pressed = false;
                rt.pressed_since_ms = None;
                rt.aborted_by_combo = false;
                rt.is_second_tap_hold = false;

                let is_this_active = self.active_rule_key.as_ref() == Some(&key_id);

                match mode.as_str() {
                    "hold" | "double_tap_hold" => {
                        if was_clean_tap {
                            rt.last_clean_tap_release_ms = Some(now_ms);
                        } else {
                            rt.last_clean_tap_release_ms = None;
                        }
                        if is_this_active {
                            return self.force_deactivate();
                        }
                    }
                    "oneshot" => {
                        if was_clean_tap {
                            let needs_double = trigger_key_requires_double_tap_for_toggle(&trig);
                            let is_double_tap = prev_tap_ms
                                .map(|t| now_ms.saturating_sub(t) <= 400)
                                .unwrap_or(false);
                            if is_this_active {
                                rt.last_clean_tap_release_ms = None;
                                return self.force_deactivate();
                            } else if !needs_double || is_double_tap {
                                rt.last_clean_tap_release_ms = None;
                                self.active_rule_key = Some(key_id);
                                self.active_target_profile = Some(rule.target_profile);
                                self.oneshot_activated_ms = now_ms;
                                self.oneshot_expires_ms = now_ms.saturating_add(oneshot_timeout_ms);
                                self.oneshot_saw_duckypad_keydown = false;
                                return Some(ModifierLayerTransition::Activated {
                                    target_profile: rule.target_profile,
                                    trigger_key: trig,
                                    mode,
                                });
                            } else {
                                rt.last_clean_tap_release_ms = Some(now_ms);
                            }
                        } else {
                            rt.last_clean_tap_release_ms = None;
                        }
                    }
                    "toggle" => {
                        if was_clean_tap {
                            let needs_double = trigger_key_requires_double_tap_for_toggle(&trig);
                            let is_double_tap = prev_tap_ms
                                .map(|t| now_ms.saturating_sub(t) <= 400)
                                .unwrap_or(false);
                            if !needs_double || is_double_tap {
                                rt.last_clean_tap_release_ms = None;
                                if is_this_active {
                                    return self.force_deactivate();
                                } else {
                                    self.active_rule_key = Some(key_id);
                                    self.active_target_profile = Some(rule.target_profile);
                                    return Some(ModifierLayerTransition::Activated {
                                        target_profile: rule.target_profile,
                                        trigger_key: trig,
                                        mode,
                                    });
                                }
                            } else {
                                rt.last_clean_tap_release_ms = Some(now_ms);
                            }
                        } else {
                            rt.last_clean_tap_release_ms = None;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Clean up any runtime state for trigger keys that are no longer physically pressed
        // (e.g., if the active window changed while a key was held).
        for ((trig, _mode), rt) in &mut self.per_rule {
            if rt.was_pressed && !is_modifier_trigger_pressed(trig, held_keys) {
                rt.was_pressed = false;
                rt.pressed_since_ms = None;
                rt.aborted_by_combo = false;
                rt.is_second_tap_hold = false;
            }
        }

        transition
    }
}

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
                        let _ = crate::paths::write_private_file_atomic(&path, json.as_bytes()); // ignore: best-effort sync to Flatpak config
                    }
                    Some(h_cfg)
                } else {
                    Some(p_cfg)
                }
            }
            (None, Some(h_cfg)) => {
                if let Ok(json) = serde_json::to_string_pretty(&h_cfg) {
                    let _ = crate::paths::write_private_file_atomic(&path, json.as_bytes()); // ignore: best-effort sync to Flatpak config
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
                let _ = crate::paths::write_private_file_atomic(&host_path, json.as_bytes()); // ignore: best-effort host mirror
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
    std::sync::Mutex<std::collections::HashMap<String, Option<String>>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

const MAX_BROWSER_RECOVERY_COMPRESSED_BYTES: u64 = 8 * 1024 * 1024;
const MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES: usize = 16 * 1024 * 1024;
const MAX_BROWSER_SQLITE_SCAN_BYTES: u64 = 16 * 1024 * 1024;
const MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES: usize = 256;

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

    // 1. Check recovery.jsonlz4 first (live sessionstore with exact tab title & URL),
    //    checking the 12-byte mozLz40 header first so multi-tens-of-MB session files never spike RSS.
    for (_, prof_dir) in &profile_dirs {
        let recovery_path = prof_dir.join("sessionstore-backups/recovery.jsonlz4");
        let Ok(mut file) = fs::File::open(&recovery_path) else {
            continue;
        };
        let Ok(meta) = file.metadata() else {
            continue;
        };
        if meta.len() < 12 || meta.len() > MAX_BROWSER_RECOVERY_COMPRESSED_BYTES {
            continue;
        }
        let mut hdr = [0u8; 12];
        if std::io::Read::read_exact(&mut file, &mut hdr).is_err() || &hdr[..8] != b"mozLz40\0" {
            continue;
        }
        let uncomp_len = u32::from_le_bytes([hdr[8], hdr[9], hdr[10], hdr[11]]) as usize;
        if uncomp_len == 0 || uncomp_len > MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES {
            continue;
        }
        let mut compressed = Vec::with_capacity(meta.len() as usize);
        compressed.extend_from_slice(&hdr);
        if std::io::Read::read_to_end(&mut file, &mut compressed).is_ok() {
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

    // 2. Check places.sqlite-wal and places.sqlite (bounded to <= 16 MB to prevent large heap spikes)
    for (_, prof_dir) in &profile_dirs {
        for fname in ["places.sqlite-wal", "places.sqlite"] {
            let db_path = prof_dir.join(fname);
            let Ok(meta) = fs::metadata(&db_path) else {
                continue;
            };
            if meta.len() == 0 || meta.len() > MAX_BROWSER_SQLITE_SCAN_BYTES {
                continue;
            }
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
        if let Some(cached_entry) = cache.get(&search_prefix) {
            return match cached_entry {
                Some(full_stripped) => format!("{full_stripped}{browser_suffix}"),
                None => caption.to_string(),
            };
        }
    }

    if let Some(full_stripped) = lookup_full_title_in_browser_storage(&search_prefix) {
        if let Ok(mut cache) = RESOLVED_BROWSER_TITLES.lock() {
            if cache.len() >= MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES {
                cache.clear();
            }
            cache.insert(search_prefix, Some(full_stripped.clone()));
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
                    if cache.len() >= MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES {
                        cache.clear();
                    }
                    cache.insert(search_prefix, Some(augmented.clone()));
                }
                return format!("{augmented}{browser_suffix}");
            }
        }
    }

    if let Ok(mut cache) = RESOLVED_BROWSER_TITLES.lock() {
        if cache.len() >= MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES {
            cache.clear();
        }
        cache.insert(search_prefix, None);
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
    crate::paths::path_to_file_uri(&path)
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
            return Some(crate::paths::path_to_file_uri(&existing));
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
                return Some(crate::paths::path_to_file_uri(&out_path));
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
    static RULE_ICON_CACHE: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, String>>,
    > = std::sync::OnceLock::new();

    let cache_key = format!(
        "{}\x1f{}\x1f{}\x1f{}",
        resource_class.trim(),
        title_contains.unwrap_or("").trim(),
        favicon_url.unwrap_or("").trim(),
        target_profile_name.unwrap_or("").trim()
    );
    let cache =
        RULE_ICON_CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(guard) = cache.lock() {
        if let Some(cached) = guard.get(&cache_key) {
            return cached.clone();
        }
    }

    let resolved = resolve_rule_icon_uncached(
        resource_class,
        title_contains,
        favicon_url,
        target_profile_name,
    );
    if let Ok(mut guard) = cache.lock() {
        guard.insert(cache_key, resolved.clone());
    }
    resolved
}

fn resolve_rule_icon_uncached(
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
    static APP_ICON_CACHE: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, String>>,
    > = std::sync::OnceLock::new();

    let trimmed = resource_class.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if is_any_browser_wildcard(trimmed) {
        return resolve_rule_icon(trimmed, None, None, None);
    }
    let lower = trimmed.to_ascii_lowercase();
    let cache =
        APP_ICON_CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(guard) = cache.lock() {
        if let Some(cached) = guard.get(&lower) {
            return cached.clone();
        }
    }

    let resolved = resolve_app_icon_for_class_uncached(trimmed, &lower);
    if let Ok(mut guard) = cache.lock() {
        guard.insert(lower, resolved.clone());
    }
    resolved
}

fn resolve_app_icon_for_class_uncached(trimmed: &str, lower: &str) -> String {
    let short_lower = lower.rsplit('.').next().unwrap_or(lower).to_string();

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
    #[cfg(target_os = "windows")]
    {
        return win32_window::is_registry_autostart_enabled();
    }

    #[cfg(not(target_os = "windows"))]
    {
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
    #[cfg(target_os = "windows")]
    {
        return win32_window::set_registry_autostart_enabled(enabled);
    }

    #[cfg(not(target_os = "windows"))]
    {
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
        match_profile_with_rules(&self.rules, self.default_profile, resource_class, caption)
    }
}

pub fn match_profile_with_rules(
    rules: &[AutoSwitchRule],
    default_profile: Option<u8>,
    resource_class: &str,
    caption: &str,
) -> Option<u8> {
    let lower_class = resource_class.trim().to_lowercase();
    if lower_class.is_empty() || lower_class == "none" {
        return default_profile;
    }
    let resolved_caption = resolve_full_browser_caption(resource_class, caption);
    let caption_lower = resolved_caption.to_lowercase();

    let mut best_match: Option<(usize, u8)> = None;

    for rule in rules {
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

    default_profile
}

#[derive(Debug, Clone)]
pub struct SecondaryDeviceRuntime {
    pub serial: String,
    pub pid: u16,
    pub hw_model: u8,
    pub model_name: String,
    pub firmware_version: String,
    pub disk_label: String,
    pub usb_connected: bool,
    pub active_profile: u8,
    pub profiles: Vec<DuckyProfile>,
    pub rules_enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
}

static SECONDARY_DEVICES_STATE: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<String, SecondaryDeviceRuntime>>,
> = std::sync::OnceLock::new();

pub fn get_secondary_devices_snapshot() -> Vec<SecondaryDeviceRuntime> {
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let Ok(guard) = mutex.lock() else {
        return Vec::new();
    };
    let mut list: Vec<SecondaryDeviceRuntime> = guard.values().cloned().collect();
    list.sort_by(|a, b| a.serial.cmp(&b.serial));
    list
}

pub fn get_secondary_device_runtime(serial: &str) -> Option<SecondaryDeviceRuntime> {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return None;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let guard = mutex.lock().ok()?;
    guard
        .values()
        .find(|d| crate::hardware::storage::serial_matches_target(&d.serial, &clean))
        .cloned()
}

pub fn update_secondary_device_profiles(serial: &str, profiles: Vec<DuckyProfile>) {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        if let Some(entry) = guard
            .values_mut()
            .find(|d| crate::hardware::storage::serial_matches_target(&d.serial, &clean))
        {
            entry.profiles = profiles;
        }
    }
}

pub fn update_secondary_device_runtime(runtime: SecondaryDeviceRuntime) {
    let clean = crate::hardware::storage::normalize_duckypad_serial(&runtime.serial);
    if clean.is_empty() {
        return;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        guard.insert(clean, runtime);
    }
}

pub fn update_secondary_device_active_profile(serial: &str, active_profile: u8) -> bool {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return false;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        if let Some(entry) = guard.get_mut(&clean) {
            if entry.active_profile != active_profile {
                entry.active_profile = active_profile;
                return true;
            }
        }
    }
    false
}

pub fn set_secondary_device_profile_rotation(serial: &str, degrees: u16) -> bool {
    let clean = crate::hardware::storage::normalize_duckypad_serial(serial);
    if clean.is_empty() {
        return false;
    }
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(mut guard) = mutex.lock() {
        if let Some(entry) = guard.get_mut(&clean) {
            let active_num = entry.active_profile;
            if let Some(prof) = entry.profiles.iter_mut().find(|p| p.number == active_num) {
                prof.set_oled_rotation_degrees(degrees as i32);
                let profiles_clone = entry.profiles.clone();
                crate::hardware::profile::sync_profiles_to_device_cache(&clean, &profiles_clone);
                return true;
            }
        }
    }
    false
}

pub fn sync_secondary_connected_devices(
    connected: &[crate::hardware::storage::ConnectedDuckyPadInfo],
    selected_serial: Option<&str>,
) {
    let mutex = SECONDARY_DEVICES_STATE
        .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let Ok(mut guard) = mutex.lock() else {
        return;
    };
    let selected_norm = selected_serial
        .map(crate::hardware::storage::normalize_duckypad_serial)
        .filter(|s| !s.is_empty());

    for entry in guard.values_mut() {
        entry.usb_connected = false;
    }

    for dev in connected {
        let sn = crate::hardware::storage::normalize_duckypad_serial(&dev.serial);
        if sn.is_empty() {
            continue;
        }
        if let Some(ref sel) = selected_norm {
            if crate::hardware::storage::serial_matches_target(&sn, sel) {
                if let Some(existing) = guard.get_mut(&sn) {
                    existing.usb_connected = true;
                    existing.pid = dev.pid;
                    existing.hw_model = dev.hw_model;
                    existing.model_name = dev.model_name().to_string();
                    existing.firmware_version = dev.fw_version.clone();
                    existing.disk_label = dev.disk_label.clone();
                }
                continue;
            }
        }

        if let Some(existing) = guard.get_mut(&sn) {
            existing.usb_connected = true;
            existing.pid = dev.pid;
            existing.hw_model = dev.hw_model;
            existing.model_name = dev.model_name().to_string();
            existing.firmware_version = dev.fw_version.clone();
            existing.disk_label = dev.disk_label.clone();
            if dev.active_profile > 0 {
                existing.active_profile = dev.active_profile;
            }
            if existing.profiles.len() <= 1 {
                if let Some(cached) =
                    crate::hardware::profile::load_cached_profiles_for_serial_for_model(
                        &sn,
                        Some(dev.hw_model),
                    )
                {
                    if cached.len() > existing.profiles.len() {
                        existing.profiles = cached;
                    }
                }
            }
            continue;
        }

        let profiles = crate::hardware::profile::load_profiles_for_connected_device(dev.pid, &sn)
            .unwrap_or_else(|| {
                crate::hardware::profile::create_default_profiles_for_device(dev.pid, &sn)
            });
        let sd_root = crate::hardware::storage::find_mounted_duckypad_sd_for_device(
            Some(&sn),
            if dev.disk_label.is_empty() {
                None
            } else {
                Some(&dev.disk_label)
            },
            Some(dev.hw_model),
        );
        let (rules_enabled, default_profile, rules) =
            load_device_rules_for_serial(&sn, &profiles, sd_root.as_deref());
        let active_prof = if dev.active_profile > 0 {
            dev.active_profile
        } else {
            profiles.first().map(|p| p.number).unwrap_or(1)
        };
        guard.insert(
            sn.clone(),
            SecondaryDeviceRuntime {
                serial: sn,
                pid: dev.pid,
                hw_model: dev.hw_model,
                model_name: dev.model_name().to_string(),
                firmware_version: dev.fw_version.clone(),
                disk_label: dev.disk_label.clone(),
                usb_connected: true,
                active_profile: active_prof,
                profiles,
                rules_enabled,
                default_profile,
                rules,
            },
        );
    }
}

pub fn dispatch_window_switch_to_secondary_devices(
    effective_class: &str,
    resolved_caption: &str,
    paused_for_editing: bool,
) {
    if paused_for_editing {
        return;
    }
    let selected_norm = crate::hardware::storage::get_selected_duckypad_serial();
    let targets_to_switch: Vec<(String, u8)> = {
        let mutex = SECONDARY_DEVICES_STATE
            .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
        let Ok(guard) = mutex.lock() else {
            return;
        };
        let mut out = Vec::new();
        for dev in guard.values() {
            if !dev.usb_connected || !dev.rules_enabled {
                continue;
            }
            if let Some(ref sel) = selected_norm {
                if crate::hardware::storage::serial_matches_target(&dev.serial, sel) {
                    continue;
                }
            }
            if let Some(target) = match_profile_with_rules(
                &dev.rules,
                dev.default_profile,
                effective_class,
                resolved_caption,
            ) {
                if dev.profiles.iter().any(|p| p.number == target) && dev.active_profile != target {
                    out.push((dev.serial.clone(), target));
                }
            }
        }
        out
    };

    for (serial, target) in targets_to_switch {
        if send_hid_switch_profile_checked_for_serial(Some(&serial), target)
            == HidSwitchOutcome::Switched
        {
            update_secondary_device_active_profile(&serial, target);
        }
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

impl KWinBridgeService {
    pub(crate) fn handle_active_window(
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

        record_last_bridge_window(&effective_class, &resource_name, &caption);

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
            let mut hid_rejected_not_on_device = false;
            let mut paused = false;

            if let Ok(mut state) = self.state.lock() {
                paused = state.picker_armed || state.paused_for_editing;
                if !state.picker_armed {
                    state.current_window_class = effective_class.clone();
                    state.current_window_title = resolved_caption.clone();
                }
                if !state.picker_armed && state.enabled && !state.paused_for_editing {
                    let window_prof =
                        state.match_profile_for_window(&effective_class, &resolved_caption);
                    if let Some(wp) = window_prof {
                        state.last_window_matched_profile = Some(wp);
                    } else if state.active_modifier_override.is_none() {
                        state.last_window_matched_profile = None;
                    }

                    if state.active_modifier_override.is_none() {
                        matched_profile = window_prof;
                        if let Some(target) = matched_profile {
                            match send_hid_switch_profile_checked(target) {
                                HidSwitchOutcome::Switched => {
                                    state.last_switched_profile = target;
                                }
                                HidSwitchOutcome::NotLoadedOnHardware => {
                                    hid_rejected_not_on_device = true;
                                }
                                HidSwitchOutcome::DeviceUnavailable => {}
                            }
                        }
                    }
                }
            }

            dispatch_window_switch_to_secondary_devices(
                &effective_class,
                &resolved_caption,
                paused,
            );

            if let Err(e) = self.ui_event_tx.send(WindowChangeEvent {
                resource_class: effective_class,
                resource_name,
                caption: resolved_caption,
                matched_profile,
                picked: false,
                hid_rejected_not_on_device,
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
                hid_rejected_not_on_device: false,
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
}

#[cfg(not(target_os = "windows"))]
pub static KWIN_NOTIFY_SEQ: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

#[cfg(not(target_os = "windows"))]
pub static KWIN_HOOK_REGISTERED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(not(target_os = "windows"))]
#[interface(name = "io.github.duckydeck.KWinBridge")]
impl KWinBridgeService {
    fn notify_active_window(
        &mut self,
        resource_class: String,
        resource_name: String,
        caption: String,
    ) {
        KWIN_NOTIFY_SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.handle_active_window(resource_class, resource_name, caption);
    }

    fn notify_main_window_minimized(&mut self) {
        crate::bridge::device_controller::qobject::duckydeck_hide_main_window();
    }

    fn notify_main_window_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        crate::bridge::device_controller::qobject::duckydeck_record_main_geometry(
            x, y, width, height,
        );
    }

    fn notify_oled_widget_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        crate::bridge::device_controller::qobject::duckydeck_record_oled_geometry(
            x, y, width, height,
        );
    }

    fn notify_oled_widget_mapped(&mut self, is_pinned: bool) {
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
pub static LAST_OBSERVED_HW_PROFILE_BEFORE_SWITCH: std::sync::atomic::AtomicU8 =
    std::sync::atomic::AtomicU8::new(0);

pub fn current_epoch_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn send_hid_switch_profile(profile_number: u8) -> bool {
    send_hid_switch_profile_checked(profile_number) == HidSwitchOutcome::Switched
}

pub fn send_hid_switch_profile_for_serial(target_serial: Option<&str>, profile_number: u8) -> bool {
    send_hid_switch_profile_checked_for_serial(target_serial, profile_number)
        == HidSwitchOutcome::Switched
}

pub fn send_hid_switch_profile_checked(profile_number: u8) -> HidSwitchOutcome {
    let selected = crate::hardware::storage::get_selected_duckypad_serial();
    send_hid_switch_profile_checked_for_serial(selected.as_deref(), profile_number)
}

pub fn send_hid_switch_profile_checked_for_serial(
    target_serial: Option<&str>,
    profile_number: u8,
) -> HidSwitchOutcome {
    if crate::hardware::storage::MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return HidSwitchOutcome::DeviceUnavailable;
    }
    let clamped = profile_number.clamp(1, 64);
    let is_selected_target = match (
        target_serial
            .map(crate::hardware::storage::normalize_duckypad_serial)
            .filter(|s| !s.is_empty()),
        crate::hardware::storage::get_selected_duckypad_serial(),
    ) {
        (Some(req), Some(sel)) => crate::hardware::storage::serial_matches_target(&req, &sel),
        (None, _) | (_, None) => true,
    };

    let _guard = crate::hardware::storage::HID_IO_LOCK.lock().ok();
    crate::hardware::storage::with_cached_duckypad_hid_for_serial(target_serial, |device| {
        // Query current hardware profile first so we don't re-flash the OLED if already on `clamped`,
        // while still switching if the user manually changed profiles on the physical device.
        let mut qtx = [0u8; 64];
        qtx[0] = HID_REPORT_ID_OUT;
        qtx[1] = 0x76;
        qtx[2] = HID_CMD_QUERY_INFO;
        if device.write(&qtx).is_ok() {
            let mut qrx = [0u8; 64];
            if let Ok(n) = crate::hardware::storage::read_hid_report4(device, &mut qrx, 200) {
                if n >= 12 && qrx[2] == 0 && qrx[6] != 0 {
                    if is_selected_target && qrx[11] >= 1 {
                        LAST_OBSERVED_HW_PROFILE_BEFORE_SWITCH
                            .store(qrx[11], std::sync::atomic::Ordering::SeqCst);
                    }
                    if qrx[11] == clamped {
                        if is_selected_target {
                            LAST_SW_PROFILE_TARGET
                                .store(clamped, std::sync::atomic::Ordering::SeqCst);
                            LAST_SW_PROFILE_SWITCH_MS
                                .store(current_epoch_ms(), std::sync::atomic::Ordering::SeqCst);
                        }
                        return Ok(HidSwitchOutcome::Switched);
                    }
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
                    if n >= 3 && rx[2] != 0 {
                        // HID_RESPONSE_INVALID_ARG (3) / HID_RESPONSE_NO_PROFILE (4) / HID_RESPONSE_ERROR (1):
                        // Profile does not exist on the duckyPad SD card!
                        return Ok(HidSwitchOutcome::NotLoadedOnHardware);
                    }
                }
                if is_selected_target {
                    LAST_SW_PROFILE_TARGET.store(clamped, std::sync::atomic::Ordering::SeqCst);
                    LAST_SW_PROFILE_SWITCH_MS
                        .store(current_epoch_ms(), std::sync::atomic::Ordering::SeqCst);
                }
                return Ok(HidSwitchOutcome::Switched);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        Err("Failed switching profile over HID".to_string())
    })
    .unwrap_or(HidSwitchOutcome::DeviceUnavailable)
}

#[cfg(target_os = "windows")]
pub fn is_windows_pid_duckydeck(pid: u32) -> bool {
    win32_window::is_pid_duckydeck(pid)
}

#[cfg(target_os = "windows")]
mod win32_window {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::path::Path;

    #[repr(C)]
    #[derive(Clone, Copy, Default)]
    pub struct POINT {
        pub x: i32,
        pub y: i32,
    }

    #[link(name = "user32")]
    extern "system" {
        pub fn GetForegroundWindow() -> isize;
        pub fn FindWindowExW(
            hWndParent: isize,
            hWndChildAfter: isize,
            lpszClass: *const u16,
            lpszWindow: *const u16,
        ) -> isize;
        pub fn SetWindowPos(
            hWnd: isize,
            hWndInsertAfter: isize,
            X: i32,
            Y: i32,
            cx: i32,
            cy: i32,
            uFlags: u32,
        ) -> i32;
        pub fn GetWindowTextW(hWnd: isize, lpString: *mut u16, nMaxCount: i32) -> i32;
        pub fn GetClassNameW(hWnd: isize, lpClassName: *mut u16, nMaxCount: i32) -> i32;
        pub fn GetWindowThreadProcessId(hWnd: isize, lpdwProcessId: *mut u32) -> u32;
        pub fn GetCursorPos(lpPoint: *mut POINT) -> i32;
        pub fn WindowFromPoint(Point: POINT) -> isize;
        pub fn GetAncestor(hwnd: isize, gaFlags: u32) -> isize;
        pub fn GetAsyncKeyState(vKey: i32) -> i16;
        pub fn GetWindowLongPtrW(hWnd: isize, nIndex: i32) -> isize;
        pub fn SetWindowLongPtrW(hWnd: isize, nIndex: i32, dwNewLong: isize) -> isize;
    }

    #[link(name = "kernel32")]
    extern "system" {
        pub fn OpenProcess(dwDesiredAccess: u32, bInheritHandle: i32, dwProcessId: u32) -> isize;
        pub fn QueryFullProcessImageNameW(
            hProcess: isize,
            dwFlags: u32,
            lpExeName: *mut u16,
            lpdwSize: *mut u32,
        ) -> i32;
        pub fn CloseHandle(hObject: isize) -> i32;
    }

    #[link(name = "advapi32")]
    extern "system" {
        pub fn RegOpenKeyExW(
            hKey: isize,
            lpSubKey: *const u16,
            ulOptions: u32,
            samDesired: u32,
            phkResult: *mut isize,
        ) -> i32;
        pub fn RegQueryValueExW(
            hKey: isize,
            lpValueName: *const u16,
            lpReserved: *const u32,
            lpType: *mut u32,
            lpData: *mut u8,
            lpcbData: *mut u32,
        ) -> i32;
        pub fn RegSetValueExW(
            hKey: isize,
            lpValueName: *const u16,
            Reserved: u32,
            dwType: u32,
            lpData: *const u8,
            cbData: u32,
        ) -> i32;
        pub fn RegDeleteValueW(hKey: isize, lpValueName: *const u16) -> i32;
        pub fn RegCloseKey(hKey: isize) -> i32;
    }

    pub const GA_ROOT: u32 = 2;
    pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    pub const VK_LBUTTON: i32 = 0x01;
    pub const VK_ESCAPE: i32 = 0x1B;
    const HKEY_CURRENT_USER: isize = -2147483647isize; // 0x80000001
    const KEY_READ: u32 = 0x20019;
    const KEY_SET_VALUE: u32 = 0x0002;
    const REG_SZ: u32 = 1;

    fn wide_null(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub fn is_registry_autostart_enabled() -> bool {
        let subkey = wide_null("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let val_name = wide_null("DuckyDeckStudio");
        // SAFETY: RegOpenKeyExW / RegQueryValueExW / RegCloseKey use valid null-terminated UTF-16 buffers.
        unsafe {
            let mut hkey: isize = 0;
            if RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_READ, &mut hkey) != 0 {
                return false;
            }
            let mut data_len: u32 = 0;
            let status = RegQueryValueExW(
                hkey,
                val_name.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut data_len,
            );
            RegCloseKey(hkey);
            status == 0 && data_len > 2
        }
    }

    pub fn set_registry_autostart_enabled(enabled: bool) -> Result<std::path::PathBuf, String> {
        let subkey = wide_null("Software\\Microsoft\\Windows\\CurrentVersion\\Run");
        let val_name = wide_null("DuckyDeckStudio");
        let exe_path = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("duckydeck-studio.exe"));
        // SAFETY: Registry calls operate on HKCU\...\Run with valid UTF-16 buffers.
        unsafe {
            let mut hkey: isize = 0;
            let open_rc = RegOpenKeyExW(
                HKEY_CURRENT_USER,
                subkey.as_ptr(),
                0,
                KEY_SET_VALUE,
                &mut hkey,
            );
            if open_rc != 0 {
                return Err(format!("Failed opening HKCU Run key (code {open_rc})"));
            }
            if enabled {
                let cmd = format!("\"{}\" --minimized", exe_path.to_string_lossy());
                let cmd_wide = wide_null(&cmd);
                let byte_len = (cmd_wide.len() * 2) as u32;
                let set_rc = RegSetValueExW(
                    hkey,
                    val_name.as_ptr(),
                    0,
                    REG_SZ,
                    cmd_wide.as_ptr() as *const u8,
                    byte_len,
                );
                RegCloseKey(hkey);
                if set_rc != 0 {
                    return Err(format!("Failed setting Run registry value (code {set_rc})"));
                }
            } else {
                let _ = RegDeleteValueW(hkey, val_name.as_ptr());
                RegCloseKey(hkey);
            }
        }
        Ok(exe_path)
    }

    pub fn apply_oled_topmost_and_geometry(
        keep_above: bool,
        on_all_desktops: bool,
        geom: Option<(i32, i32, i32, i32)>,
    ) {
        const HWND_TOPMOST: isize = -1;
        const HWND_NOTOPMOST: isize = -2;
        const SWP_NOSIZE: u32 = 0x0001;
        const SWP_NOMOVE: u32 = 0x0002;
        const SWP_NOACTIVATE: u32 = 0x0010;
        const SWP_FRAMECHANGED: u32 = 0x0020;
        const GWL_EXSTYLE: i32 = -20;
        const WS_EX_TOOLWINDOW: isize = 0x0000_0080;

        let self_pid = std::process::id();
        // SAFETY: FindWindowExW / GetWindowThreadProcessId / GetWindowTextW / SetWindowPos are standard Win32 user32 calls.
        unsafe {
            let mut hwnd: isize = 0;
            loop {
                hwnd = FindWindowExW(0, hwnd, std::ptr::null(), std::ptr::null());
                if hwnd == 0 {
                    break;
                }
                let mut pid: u32 = 0;
                GetWindowThreadProcessId(hwnd, &mut pid);
                if pid != self_pid {
                    continue;
                }
                let mut title_buf = [0u16; 256];
                let len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32);
                if len <= 0 {
                    continue;
                }
                let title = OsString::from_wide(&title_buf[..len as usize])
                    .to_string_lossy()
                    .to_string();
                if title.starts_with("duckyPad OLED") {
                    let want_all_desktops = on_all_desktops && !title.contains("[SingleDesktop]");
                    let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
                    let next_ex = if want_all_desktops {
                        ex_style | WS_EX_TOOLWINDOW
                    } else {
                        ex_style & !WS_EX_TOOLWINDOW
                    };
                    if next_ex != ex_style {
                        SetWindowLongPtrW(hwnd, GWL_EXSTYLE, next_ex);
                    }
                    let insert_after = if keep_above && !title.contains("[Unpinned]") {
                        HWND_TOPMOST
                    } else {
                        HWND_NOTOPMOST
                    };
                    if let Some((gx, gy, gw, gh)) = geom {
                        SetWindowPos(
                            hwnd,
                            insert_after,
                            gx,
                            gy,
                            gw,
                            gh,
                            SWP_NOACTIVATE | SWP_FRAMECHANGED,
                        );
                    } else {
                        SetWindowPos(
                            hwnd,
                            insert_after,
                            0,
                            0,
                            0,
                            0,
                            SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE | SWP_FRAMECHANGED,
                        );
                    }
                }
            }
        }
    }

    pub fn is_pid_duckydeck(pid: u32) -> bool {
        if pid <= 1 {
            return false;
        }
        // SAFETY: OpenProcess / QueryFullProcessImageNameW / CloseHandle use a stack buffer.
        unsafe {
            let hproc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if hproc == 0 {
                return false;
            }
            let mut path_buf = [0u16; 1024];
            let mut sz = path_buf.len() as u32;
            let ok =
                QueryFullProcessImageNameW(hproc, 0, path_buf.as_mut_ptr(), &mut sz) != 0 && sz > 0;
            CloseHandle(hproc);
            if !ok {
                return false;
            }
            let full = OsString::from_wide(&path_buf[..sz as usize])
                .to_string_lossy()
                .to_ascii_lowercase();
            full.contains("duckydeck-studio")
        }
    }

    pub fn inspect_hwnd(hwnd: isize) -> Option<(String, String, String)> {
        static HWND_CLASS_CACHE: std::sync::OnceLock<
            std::sync::Mutex<std::collections::HashMap<(isize, u32), (String, String)>>,
        > = std::sync::OnceLock::new();

        if hwnd == 0 {
            return None;
        }
        // SAFETY: Win32 window queries use stack buffers with explicit max lengths.
        unsafe {
            let root = {
                let r = GetAncestor(hwnd, GA_ROOT);
                if r != 0 {
                    r
                } else {
                    hwnd
                }
            };
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(root, &mut pid);
            if pid == std::process::id() {
                return None;
            }

            let mut title_buf = [0u16; 512];
            let title_len = GetWindowTextW(root, title_buf.as_mut_ptr(), title_buf.len() as i32);
            let caption = if title_len > 0 {
                OsString::from_wide(&title_buf[..title_len as usize])
                    .to_string_lossy()
                    .to_string()
            } else {
                String::new()
            };

            let cache = HWND_CLASS_CACHE
                .get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
            if let Ok(guard) = cache.lock() {
                if let Some((rc, rn)) = guard.get(&(root, pid)) {
                    if rc.is_empty() && caption.is_empty() {
                        return None;
                    }
                    return Some((rc.clone(), rn.clone(), caption));
                }
            }

            let mut class_buf = [0u16; 256];
            let class_len = GetClassNameW(root, class_buf.as_mut_ptr(), class_buf.len() as i32);
            let win_class = if class_len > 0 {
                OsString::from_wide(&class_buf[..class_len as usize])
                    .to_string_lossy()
                    .to_string()
            } else {
                String::new()
            };

            let mut exe_stem = String::new();
            if pid != 0 {
                let hproc = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
                if hproc != 0 {
                    let mut path_buf = [0u16; 1024];
                    let mut sz = path_buf.len() as u32;
                    if QueryFullProcessImageNameW(hproc, 0, path_buf.as_mut_ptr(), &mut sz) != 0
                        && sz > 0
                    {
                        let full = OsString::from_wide(&path_buf[..sz as usize])
                            .to_string_lossy()
                            .to_string();
                        if let Some(stem) = Path::new(&full).file_stem().and_then(|s| s.to_str()) {
                            exe_stem = stem.to_string();
                        }
                    }
                    CloseHandle(hproc);
                }
            }

            let resource_class = if !exe_stem.is_empty() {
                exe_stem.clone()
            } else {
                win_class.clone()
            };
            let resource_name = if !win_class.is_empty() {
                win_class
            } else {
                resource_class.clone()
            };
            if let Ok(mut guard) = cache.lock() {
                if guard.len() >= 256 {
                    guard.clear();
                }
                guard.insert((root, pid), (resource_class.clone(), resource_name.clone()));
            }
            if resource_class.is_empty() && caption.is_empty() {
                return None;
            }
            Some((resource_class, resource_name, caption))
        }
    }
}

#[cfg(target_os = "windows")]
pub async fn pick_window_via_kwin_dbus() -> std::result::Result<WindowChangeEvent, String> {
    // Wait up to 15 seconds for user to click any window (or press Escape to cancel)
    for _ in 0..300 {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        // SAFETY: GetAsyncKeyState and GetCursorPos/WindowFromPoint are thread-safe Win32 queries.
        let picked = unsafe {
            if (win32_window::GetAsyncKeyState(win32_window::VK_ESCAPE) as u16 & 0x8000) != 0 {
                return Err("Window picker canceled".to_string());
            }
            if (win32_window::GetAsyncKeyState(win32_window::VK_LBUTTON) as u16 & 0x8000) != 0 {
                let mut pt = win32_window::POINT::default();
                let hwnd = if win32_window::GetCursorPos(&mut pt) != 0 {
                    win32_window::WindowFromPoint(pt)
                } else {
                    win32_window::GetForegroundWindow()
                };
                win32_window::inspect_hwnd(hwnd)
            } else {
                None
            }
        };
        if let Some((resource_class, resource_name, raw_caption)) = picked {
            let caption = resolve_full_browser_caption(&resource_class, &raw_caption);
            return Ok(WindowChangeEvent {
                resource_class,
                resource_name,
                caption,
                matched_profile: None,
                picked: true,
                hid_rejected_not_on_device: false,
            });
        }
    }
    Err("Window picker timed out".to_string())
}

static LAST_BRIDGE_WINDOW: std::sync::OnceLock<Mutex<(String, String, String, u64)>> =
    std::sync::OnceLock::new();

fn record_last_bridge_window(resource_class: &str, resource_name: &str, caption: &str) {
    let lock = LAST_BRIDGE_WINDOW.get_or_init(|| Mutex::new((String::new(), String::new(), String::new(), 0)));
    if let Ok(mut guard) = lock.lock() {
        let next_seq = guard.3.wrapping_add(1);
        *guard = (
            resource_class.to_string(),
            resource_name.to_string(),
            caption.to_string(),
            next_seq,
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn latest_bridge_window() -> Option<(String, String, String, u64)> {
    let lock = LAST_BRIDGE_WINDOW.get()?;
    let guard = lock.lock().ok()?;
    if guard.3 == 0 || guard.0.is_empty() {
        None
    } else {
        Some(guard.clone())
    }
}

#[cfg(not(target_os = "windows"))]
mod linux_desktop_window {
    use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
    use std::fs;
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;
    use std::path::{Path, PathBuf};
    use std::sync::{Mutex, OnceLock};
    use std::time::Duration;

    pub type WindowTuple = (String, String, String);

    // =========================================================================
    // 1. Omarchy / Hyprland Backend ($XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE)
    // =========================================================================

    pub fn is_hyprland_session() -> bool {
        find_hyprland_socket_path().is_some()
    }

    pub fn find_hyprland_socket_path() -> Option<PathBuf> {
        let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
        let sig = sig.trim();
        if sig.is_empty() {
            return None;
        }
        if let Ok(rt) = std::env::var("XDG_RUNTIME_DIR") {
            let p = PathBuf::from(rt).join("hypr").join(sig).join(".socket.sock");
            if p.exists() {
                return Some(p);
            }
        }
        let fallback = PathBuf::from("/tmp/hypr").join(sig).join(".socket.sock");
        if fallback.exists() {
            return Some(fallback);
        }
        None
    }

    fn hyprland_ipc_request(sock_path: &Path, command: &str) -> Option<String> {
        let mut stream = UnixStream::connect(sock_path).ok()?;
        let _ = stream.set_write_timeout(Some(Duration::from_millis(50)));
        let _ = stream.set_read_timeout(Some(Duration::from_millis(50)));
        stream.write_all(command.as_bytes()).ok()?;
        let mut out = String::new();
        stream.read_to_string(&mut out).ok()?;
        Some(out)
    }

    pub fn parse_hyprland_active_window_json(raw_json: &str) -> Option<WindowTuple> {
        let val: serde_json::Value = serde_json::from_str(raw_json.trim()).ok()?;
        let obj = val.as_object()?;
        let class = obj
            .get("class")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let initial_class = obj
            .get("initialClass")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let title = obj
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();

        let effective_class = if !class.is_empty() {
            class.clone()
        } else {
            initial_class.clone()
        };
        if effective_class.is_empty() {
            return None;
        }
        let lower = effective_class.to_ascii_lowercase();
        if lower.contains("duckydeck") || title.starts_with("duckyPad OLED —") {
            return None;
        }
        let res_name = if !initial_class.is_empty() {
            initial_class
        } else {
            effective_class.clone()
        };
        Some((effective_class, res_name, title))
    }

    pub fn parse_hyprland_cursorpos_json(raw_json: &str) -> Option<(i64, i64)> {
        let val: serde_json::Value = serde_json::from_str(raw_json.trim()).ok()?;
        let obj = val.as_object()?;
        let x = obj.get("x")?.as_i64()?;
        let y = obj.get("y")?.as_i64()?;
        Some((x, y))
    }

    pub fn find_hyprland_client_at_cursor_json(
        clients_json: &str,
        active_workspace_id: Option<i64>,
        cx: i64,
        cy: i64,
    ) -> Option<WindowTuple> {
        let val: serde_json::Value = serde_json::from_str(clients_json.trim()).ok()?;
        let arr = val.as_array()?;
        let mut best: Option<(bool, i64, WindowTuple)> = None;

        for item in arr {
            let Some(obj) = item.as_object() else {
                continue;
            };
            let mapped = obj.get("mapped").and_then(|v| v.as_bool()).unwrap_or(true);
            let hidden = obj.get("hidden").and_then(|v| v.as_bool()).unwrap_or(false);
            if !mapped || hidden {
                continue;
            }
            let pinned = obj.get("pinned").and_then(|v| v.as_bool()).unwrap_or(false);
            let ws_id = obj
                .get("workspace")
                .and_then(|w| w.get("id"))
                .and_then(|v| v.as_i64());
            if let (Some(active_ws), Some(client_ws)) = (active_workspace_id, ws_id) {
                if !pinned && client_ws != active_ws && client_ws > 0 {
                    continue;
                }
            }
            let Some(at) = obj.get("at").and_then(|v| v.as_array()) else {
                continue;
            };
            let Some(size) = obj.get("size").and_then(|v| v.as_array()) else {
                continue;
            };
            if at.len() < 2 || size.len() < 2 {
                continue;
            }
            let x = at[0].as_i64().unwrap_or(0);
            let y = at[1].as_i64().unwrap_or(0);
            let w = size[0].as_i64().unwrap_or(0);
            let h = size[1].as_i64().unwrap_or(0);
            if w <= 0 || h <= 0 || cx < x || cx >= x + w || cy < y || cy >= y + h {
                continue;
            }

            let class = obj
                .get("class")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let initial_class = obj
                .get("initialClass")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let title = obj
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let effective_class = if !class.is_empty() {
                class
            } else {
                initial_class.clone()
            };
            if effective_class.is_empty() {
                continue;
            }
            if effective_class.to_ascii_lowercase().contains("duckydeck")
                || title.starts_with("duckyPad OLED —")
            {
                continue;
            }
            let res_name = if !initial_class.is_empty() {
                initial_class
            } else {
                effective_class.clone()
            };
            let floating = obj
                .get("floating")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let focus_history = obj
                .get("focusHistoryID")
                .and_then(|v| v.as_i64())
                .unwrap_or(9999);

            let candidate = (effective_class, res_name, title);
            let replace = match &best {
                None => true,
                Some((best_floating, best_hist, _)) => {
                    if floating != *best_floating {
                        floating
                    } else {
                        focus_history < *best_hist
                    }
                }
            };
            if replace {
                best = Some((floating, focus_history, candidate));
            }
        }

        best.map(|(_, _, win)| win)
    }

    pub fn query_hyprland_window_and_hover(
        last_cursor: &mut Option<(i64, i64)>,
    ) -> Option<(Option<WindowTuple>, Option<WindowTuple>)> {
        let sock = find_hyprland_socket_path()?;
        Some(query_hyprland_window_and_hover_from_socket(&sock, last_cursor))
    }

    pub fn query_hyprland_window_and_hover_from_socket(
        sock: &Path,
        last_cursor: &mut Option<(i64, i64)>,
    ) -> (Option<WindowTuple>, Option<WindowTuple>) {
        let active_win = hyprland_ipc_request(sock, "j/activewindow")
            .and_then(|raw| parse_hyprland_active_window_json(&raw));

        let mut hover_win = None;
        if let Some(cursor_raw) = hyprland_ipc_request(sock, "j/cursorpos") {
            if let Some(cur_xy) = parse_hyprland_cursorpos_json(&cursor_raw) {
                if *last_cursor != Some(cur_xy) {
                    *last_cursor = Some(cur_xy);
                    let active_ws = hyprland_ipc_request(sock, "j/activeworkspace")
                        .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw.trim()).ok())
                        .and_then(|v| v.get("id").and_then(|id| id.as_i64()));
                    if let Some(clients_raw) = hyprland_ipc_request(sock, "j/clients") {
                        hover_win = find_hyprland_client_at_cursor_json(
                            &clients_raw,
                            active_ws,
                            cur_xy.0,
                            cur_xy.1,
                        );
                    }
                }
            }
        }
        (active_win, hover_win)
    }

    pub fn query_hyprland_window_at_current_cursor() -> Option<WindowTuple> {
        let sock = find_hyprland_socket_path()?;
        let cursor_raw = hyprland_ipc_request(&sock, "j/cursorpos")?;
        let (cx, cy) = parse_hyprland_cursorpos_json(&cursor_raw)?;
        let active_ws = hyprland_ipc_request(&sock, "j/activeworkspace")
            .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw.trim()).ok())
            .and_then(|v| v.get("id").and_then(|id| id.as_i64()));
        let clients_raw = hyprland_ipc_request(&sock, "j/clients")?;
        find_hyprland_client_at_cursor_json(&clients_raw, active_ws, cx, cy)
    }

    // =========================================================================
    // 2. COSMIC (`cosmic-comp`) / wlroots / Sway / Niri Wayland Toplevel Watcher
    //    (Pure-Rust Wayland wire protocol client for `zwlr_foreign_toplevel_manager_v1`
    //     and `zcosmic_toplevel_info_v1` / `ext_foreign_toplevel_list_v1`)
    // =========================================================================

    static WAYLAND_ACTIVE_TOPLEVEL: OnceLock<Mutex<(String, String, String, u64)>> =
        OnceLock::new();
    static WAYLAND_WATCHER_STARTED: OnceLock<()> = OnceLock::new();

    #[derive(Clone, Default)]
    struct ToplevelEntry {
        is_cosmic: bool,
        app_id: String,
        title: String,
        activated: bool,
    }

    pub fn parse_wayland_state_has_activated(state_bytes: &[u8]) -> bool {
        // In both `zwlr_foreign_toplevel_handle_v1::state` and `zcosmic_toplevel_handle_v1::state`,
        // `STATE_ACTIVATED = 2` is encoded as a 32-bit little-endian integer in the `wl_array`.
        state_bytes
            .as_chunks::<4>()
            .0
            .iter()
            .any(|chunk| u32::from_ne_bytes(*chunk) == 2)
    }

    fn encode_wl_str(s: &str) -> Vec<u8> {
        let bytes = s.as_bytes();
        let len_with_nul = (bytes.len() + 1) as u32;
        let mut out = Vec::with_capacity(4 + ((len_with_nul as usize + 3) & !3));
        out.extend_from_slice(&len_with_nul.to_ne_bytes());
        out.extend_from_slice(bytes);
        out.push(0);
        while out.len() % 4 != 0 {
            out.push(0);
        }
        out
    }

    fn send_wl_msg(stream: &mut UnixStream, sender_id: u32, opcode: u16, payload: &[u8]) -> bool {
        let total_size = (8 + payload.len()) as u16;
        let size_and_opcode = ((total_size as u32) << 16) | (opcode as u32);
        let mut buf = Vec::with_capacity(8 + payload.len());
        buf.extend_from_slice(&sender_id.to_ne_bytes());
        buf.extend_from_slice(&size_and_opcode.to_ne_bytes());
        buf.extend_from_slice(payload);
        stream.write_all(&buf).is_ok()
    }

    fn read_wl_str(payload: &[u8], offset: usize) -> Option<(String, usize)> {
        if offset + 4 > payload.len() {
            return None;
        }
        let len = u32::from_ne_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
        ]) as usize;
        let padded = (len + 3) & !3;
        let start = offset + 4;
        if len == 0 || start + padded > payload.len() {
            return Some((String::new(), start + padded.min(payload.len().saturating_sub(start))));
        }
        let raw = &payload[start..start + len.saturating_sub(1)];
        let s = String::from_utf8_lossy(raw).trim().to_string();
        Some((s, start + padded))
    }

    fn read_wl_array(payload: &[u8], offset: usize) -> Option<(&[u8], usize)> {
        if offset + 4 > payload.len() {
            return None;
        }
        let len = u32::from_ne_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
        ]) as usize;
        let padded = (len + 3) & !3;
        let start = offset + 4;
        if start + padded > payload.len() {
            return None;
        }
        Some((&payload[start..start + len], start + padded))
    }

    fn record_wayland_active_toplevel(app_id: &str, title: &str) {
        let cls = app_id.trim();
        if cls.is_empty() {
            return;
        }
        if cls.to_ascii_lowercase().contains("duckydeck")
            || title.starts_with("duckyPad OLED —")
        {
            return;
        }
        let lock = WAYLAND_ACTIVE_TOPLEVEL
            .get_or_init(|| Mutex::new((String::new(), String::new(), String::new(), 0)));
        if let Ok(mut guard) = lock.lock() {
            if guard.0 != cls || guard.2 != title {
                let next_seq = guard.3.wrapping_add(1);
                *guard = (
                    cls.to_string(),
                    cls.to_string(),
                    title.to_string(),
                    next_seq,
                );
            }
        }
    }

    pub fn ensure_wayland_toplevel_watcher_started() {
        WAYLAND_WATCHER_STARTED.get_or_init(|| {
            std::thread::Builder::new()
                .name("duckydeck-wl-toplevel".to_string())
                .spawn(|| {
                    run_wayland_toplevel_client();
                })
                .ok();
        });
    }

    pub fn latest_wayland_active_window() -> Option<(String, String, String, u64)> {
        ensure_wayland_toplevel_watcher_started();
        let lock = WAYLAND_ACTIVE_TOPLEVEL.get()?;
        let guard = lock.lock().ok()?;
        if guard.3 == 0 || guard.0.is_empty() {
            None
        } else {
            Some(guard.clone())
        }
    }

    fn run_wayland_toplevel_client() {
        let Ok(wl_disp) = std::env::var("WAYLAND_DISPLAY") else {
            return;
        };
        if wl_disp.trim().is_empty() {
            return;
        }
        let sock_path = if Path::new(&wl_disp).is_absolute() {
            PathBuf::from(&wl_disp)
        } else if let Ok(rt) = std::env::var("XDG_RUNTIME_DIR") {
            PathBuf::from(rt).join(&wl_disp)
        } else {
            return;
        };
        let Ok(stream) = UnixStream::connect(&sock_path) else {
            return;
        };
        run_wayland_toplevel_client_on_stream(stream);
    }

    pub fn run_wayland_toplevel_client_on_stream(mut stream: UnixStream) {
        // Object IDs: 1 = wl_display, 2 = wl_registry
        let mut next_client_id: u32 = 3;
        let registry_id: u32 = 2;
        if !send_wl_msg(&mut stream, 1, 1, &registry_id.to_ne_bytes()) {
            return;
        }

        let mut zwlr_mgr_id: Option<u32> = None;
        let mut cosmic_info_id: Option<u32> = None;
        let mut toplevels: std::collections::HashMap<u32, ToplevelEntry> =
            std::collections::HashMap::new();

        let mut hdr = [0u8; 8];
        loop {
            if stream.read_exact(&mut hdr).is_err() {
                break;
            }
            let sender_id = u32::from_ne_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
            let size_and_op = u32::from_ne_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]);
            let size = (size_and_op >> 16) as usize;
            let opcode = (size_and_op & 0xFFFF) as u16;
            if size < 8 {
                break;
            }
            let mut payload = vec![0u8; size - 8];
            if !payload.is_empty() && stream.read_exact(&mut payload).is_err() {
                break;
            }

            if sender_id == registry_id && opcode == 0 && payload.len() >= 8 {
                // wl_registry::global(name: u32, interface: str, version: u32)
                let name = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
                if let Some((iface, next_off)) = read_wl_str(&payload, 4) {
                    if next_off + 4 <= payload.len() {
                        let version = u32::from_ne_bytes([
                            payload[next_off],
                            payload[next_off + 1],
                            payload[next_off + 2],
                            payload[next_off + 3],
                        ]);
                        if iface == "zwlr_foreign_toplevel_manager_v1" && zwlr_mgr_id.is_none() {
                            let new_id = next_client_id;
                            next_client_id += 1;
                            let mut bind_payload = Vec::new();
                            bind_payload.extend_from_slice(&name.to_ne_bytes());
                            bind_payload.extend_from_slice(&encode_wl_str(&iface));
                            bind_payload.extend_from_slice(&version.clamp(1, 3).to_ne_bytes());
                            bind_payload.extend_from_slice(&new_id.to_ne_bytes());
                            if send_wl_msg(&mut stream, registry_id, 0, &bind_payload) {
                                zwlr_mgr_id = Some(new_id);
                            }
                        } else if iface == "zcosmic_toplevel_info_v1" && cosmic_info_id.is_none() {
                            let new_id = next_client_id;
                            next_client_id += 1;
                            let mut bind_payload = Vec::new();
                            bind_payload.extend_from_slice(&name.to_ne_bytes());
                            bind_payload.extend_from_slice(&encode_wl_str(&iface));
                            bind_payload.extend_from_slice(&1u32.to_ne_bytes());
                            bind_payload.extend_from_slice(&new_id.to_ne_bytes());
                            if send_wl_msg(&mut stream, registry_id, 0, &bind_payload) {
                                cosmic_info_id = Some(new_id);
                            }
                        }
                    }
                }
            } else if (Some(sender_id) == zwlr_mgr_id || Some(sender_id) == cosmic_info_id)
                && opcode == 0
                && payload.len() >= 4
            {
                // `toplevel(id: new_id)` event from manager
                let handle_id =
                    u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
                toplevels.insert(
                    handle_id,
                    ToplevelEntry {
                        is_cosmic: Some(sender_id) == cosmic_info_id,
                        ..ToplevelEntry::default()
                    },
                );
            } else if let Some(entry) = toplevels.get_mut(&sender_id) {
                if entry.is_cosmic {
                    // `zcosmic_toplevel_handle_v1` (bound at v1) event opcodes:
                    // 0 = closed(), 1 = done(), 2 = title(string), 3 = app_id(string), 8 = state(wl_array)
                    match opcode {
                        0 => {
                            toplevels.remove(&sender_id);
                        }
                        1 => {
                            if entry.activated && !entry.app_id.is_empty() {
                                let app_id = entry.app_id.clone();
                                let title = entry.title.clone();
                                record_wayland_active_toplevel(&app_id, &title);
                            }
                        }
                        2 => {
                            if let Some((title, _)) = read_wl_str(&payload, 0) {
                                entry.title = title;
                            }
                        }
                        3 => {
                            if let Some((app_id, _)) = read_wl_str(&payload, 0) {
                                entry.app_id = app_id;
                            }
                        }
                        8 => {
                            if let Some((arr, _)) = read_wl_array(&payload, 0) {
                                entry.activated = parse_wayland_state_has_activated(arr);
                            }
                        }
                        _ => {}
                    }
                } else {
                    // `zwlr_foreign_toplevel_handle_v1` event opcodes:
                    // 0 = title(string), 1 = app_id(string), 4 = state(wl_array), 5 = done(), 6 = closed()
                    match opcode {
                        0 => {
                            if let Some((title, _)) = read_wl_str(&payload, 0) {
                                entry.title = title;
                            }
                        }
                        1 => {
                            if let Some((app_id, _)) = read_wl_str(&payload, 0) {
                                entry.app_id = app_id;
                            }
                        }
                        4 => {
                            if let Some((arr, _)) = read_wl_array(&payload, 0) {
                                entry.activated = parse_wayland_state_has_activated(arr);
                            }
                        }
                        5 => {
                            if entry.activated && !entry.app_id.is_empty() {
                                let app_id = entry.app_id.clone();
                                let title = entry.title.clone();
                                record_wayland_active_toplevel(&app_id, &title);
                            }
                        }
                        6 => {
                            toplevels.remove(&sender_id);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // =========================================================================
    // 3. Cinnamon (Linux Mint) / MATE / Xfce / Xorg Backend (`libX11.so.6`)
    // =========================================================================

    type XErrorHandler = unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int;

    unsafe extern "C" fn ignore_x11_errors(_dpy: *mut c_void, _err: *mut c_void) -> c_int {
        0
    }

    struct X11Api {
        dpy: *mut c_void,
        root: c_ulong,
        atom_net_active_window: c_ulong,
        atom_net_wm_name: c_ulong,
        atom_utf8_string: c_ulong,
        atom_wm_class: c_ulong,
        atom_wm_name: c_ulong,
        atom_string: c_ulong,
        x_get_window_property: unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            c_ulong,
            c_long,
            c_long,
            c_int,
            c_ulong,
            *mut c_ulong,
            *mut c_int,
            *mut c_ulong,
            *mut c_ulong,
            *mut *mut c_uchar,
        ) -> c_int,
        x_free: unsafe extern "C" fn(*mut c_void) -> c_int,
        x_query_pointer: unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            *mut c_ulong,
            *mut c_ulong,
            *mut c_int,
            *mut c_int,
            *mut c_int,
            *mut c_int,
            *mut c_uint,
        ) -> c_int,
        x_query_tree: unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            *mut c_ulong,
            *mut c_ulong,
            *mut *mut c_ulong,
            *mut c_uint,
        ) -> c_int,
    }

    // SAFETY: Accessed exclusively through a global `Mutex<Option<X11Api>>`.
    unsafe impl Send for X11Api {}

    static X11_API: OnceLock<Mutex<Option<X11Api>>> = OnceLock::new();

    unsafe extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }

    const RTLD_LAZY: c_int = 1;

    #[allow(clippy::missing_transmute_annotations)]
    unsafe fn init_x11_api() -> Option<X11Api> {
        std::env::var_os("DISPLAY")?;
        let handle = dlopen(c"libX11.so.6".as_ptr(), RTLD_LAZY);
        if handle.is_null() {
            return None;
        }

        let sym_open = dlsym(handle, c"XOpenDisplay".as_ptr());
        let sym_root = dlsym(handle, c"XDefaultRootWindow".as_ptr());
        let sym_intern = dlsym(handle, c"XInternAtom".as_ptr());
        let sym_get_prop = dlsym(handle, c"XGetWindowProperty".as_ptr());
        let sym_free = dlsym(handle, c"XFree".as_ptr());
        let sym_qp = dlsym(handle, c"XQueryPointer".as_ptr());
        let sym_qt = dlsym(handle, c"XQueryTree".as_ptr());
        let sym_err = dlsym(handle, c"XSetErrorHandler".as_ptr());

        if sym_open.is_null()
            || sym_root.is_null()
            || sym_intern.is_null()
            || sym_get_prop.is_null()
            || sym_free.is_null()
            || sym_qp.is_null()
            || sym_qt.is_null()
        {
            return None;
        }

        if !sym_err.is_null() {
            let x_set_error_handler: unsafe extern "C" fn(XErrorHandler) -> *mut c_void =
                std::mem::transmute(sym_err);
            x_set_error_handler(ignore_x11_errors);
        }

        let x_open_display: unsafe extern "C" fn(*const c_char) -> *mut c_void =
            std::mem::transmute(sym_open);
        let x_default_root: unsafe extern "C" fn(*mut c_void) -> c_ulong =
            std::mem::transmute(sym_root);
        let x_intern_atom: unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_ulong =
            std::mem::transmute(sym_intern);

        let dpy = x_open_display(std::ptr::null());
        if dpy.is_null() {
            return None;
        }
        let root = x_default_root(dpy);
        let atom_net_active_window = x_intern_atom(dpy, c"_NET_ACTIVE_WINDOW".as_ptr(), 0);
        let atom_net_wm_name = x_intern_atom(dpy, c"_NET_WM_NAME".as_ptr(), 0);
        let atom_utf8_string = x_intern_atom(dpy, c"UTF8_STRING".as_ptr(), 0);
        let atom_wm_class = x_intern_atom(dpy, c"WM_CLASS".as_ptr(), 0);
        let atom_wm_name = x_intern_atom(dpy, c"WM_NAME".as_ptr(), 0);
        let atom_string = x_intern_atom(dpy, c"STRING".as_ptr(), 0);

        Some(X11Api {
            dpy,
            root,
            atom_net_active_window,
            atom_net_wm_name,
            atom_utf8_string,
            atom_wm_class,
            atom_wm_name,
            atom_string,
            x_get_window_property: std::mem::transmute(sym_get_prop),
            x_free: std::mem::transmute(sym_free),
            x_query_pointer: std::mem::transmute(sym_qp),
            x_query_tree: std::mem::transmute(sym_qt),
        })
    }

    pub fn parse_wm_class_bytes(bytes: &[u8]) -> Option<(String, String)> {
        let parts: Vec<&[u8]> = bytes
            .split(|&b| b == 0)
            .filter(|s| !s.is_empty())
            .collect();
        if parts.is_empty() {
            return None;
        }
        let res_name = String::from_utf8_lossy(parts[0]).trim().to_string();
        let res_class = if parts.len() >= 2 {
            String::from_utf8_lossy(parts[1]).trim().to_string()
        } else {
            res_name.clone()
        };
        let effective_class = if !res_class.is_empty() {
            res_class
        } else {
            res_name.clone()
        };
        if effective_class.is_empty() {
            None
        } else {
            Some((effective_class, res_name))
        }
    }

    unsafe fn x11_get_property_bytes(
        api: &X11Api,
        win: c_ulong,
        prop: c_ulong,
        req_type: c_ulong,
    ) -> Option<Vec<u8>> {
        if win == 0 {
            return None;
        }
        let mut actual_type: c_ulong = 0;
        let mut actual_format: c_int = 0;
        let mut nitems: c_ulong = 0;
        let mut bytes_after: c_ulong = 0;
        let mut prop_ptr: *mut c_uchar = std::ptr::null_mut();

        let status = (api.x_get_window_property)(
            api.dpy,
            win,
            prop,
            0,
            1024,
            0,
            req_type,
            &mut actual_type,
            &mut actual_format,
            &mut nitems,
            &mut bytes_after,
            &mut prop_ptr,
        );
        if status != 0 || prop_ptr.is_null() {
            return None;
        }
        let byte_len = match actual_format {
            8 => nitems as usize,
            16 => (nitems as usize) * 2,
            32 => (nitems as usize) * std::mem::size_of::<c_ulong>(),
            _ => 0,
        };
        let slice = if byte_len > 0 {
            std::slice::from_raw_parts(prop_ptr, byte_len).to_vec()
        } else {
            Vec::new()
        };
        (api.x_free)(prop_ptr.cast());
        if slice.is_empty() {
            None
        } else {
            Some(slice)
        }
    }

    unsafe fn x11_read_window_title(api: &X11Api, win: c_ulong) -> String {
        if let Some(bytes) =
            x11_get_property_bytes(api, win, api.atom_net_wm_name, api.atom_utf8_string)
                .or_else(|| x11_get_property_bytes(api, win, api.atom_net_wm_name, 0))
        {
            let s = String::from_utf8_lossy(&bytes)
                .trim_end_matches('\0')
                .trim()
                .to_string();
            if !s.is_empty() {
                return s;
            }
        }
        if let Some(bytes) = x11_get_property_bytes(api, win, api.atom_wm_name, api.atom_string)
            .or_else(|| x11_get_property_bytes(api, win, api.atom_wm_name, 0))
        {
            let s = String::from_utf8_lossy(&bytes)
                .trim_end_matches('\0')
                .trim()
                .to_string();
            if !s.is_empty() {
                return s;
            }
        }
        String::new()
    }

    unsafe fn x11_inspect_window_recursive(
        api: &X11Api,
        win: c_ulong,
        depth: u8,
    ) -> Option<WindowTuple> {
        if win == 0 || win == api.root {
            return None;
        }
        if let Some(bytes) = x11_get_property_bytes(api, win, api.atom_wm_class, api.atom_string)
            .or_else(|| x11_get_property_bytes(api, win, api.atom_wm_class, 0))
        {
            if let Some((cls, rname)) = parse_wm_class_bytes(&bytes) {
                let title = x11_read_window_title(api, win);
                if !cls.to_ascii_lowercase().contains("duckydeck")
                    && !title.starts_with("duckyPad OLED —")
                {
                    return Some((cls, rname, title));
                }
                return None;
            }
        }
        if depth >= 3 {
            return None;
        }
        let mut root_ret: c_ulong = 0;
        let mut parent_ret: c_ulong = 0;
        let mut children_ptr: *mut c_ulong = std::ptr::null_mut();
        let mut nchildren: c_uint = 0;
        let ok = (api.x_query_tree)(
            api.dpy,
            win,
            &mut root_ret,
            &mut parent_ret,
            &mut children_ptr,
            &mut nchildren,
        );
        if ok == 0 || children_ptr.is_null() {
            return None;
        }
        let children = std::slice::from_raw_parts(children_ptr, nchildren as usize).to_vec();
        (api.x_free)(children_ptr.cast());
        for &child in children.iter().rev() {
            if let Some(found) = x11_inspect_window_recursive(api, child, depth + 1) {
                return Some(found);
            }
        }
        None
    }

    pub fn query_x11_window_and_hover(
        last_cursor: &mut Option<(i32, i32)>,
        inspect_cursor_always: bool,
    ) -> (Option<WindowTuple>, Option<WindowTuple>, bool) {
        let lock = X11_API.get_or_init(|| Mutex::new(unsafe { init_x11_api() }));
        let Ok(guard) = lock.lock() else {
            return (None, None, false);
        };
        let Some(ref api) = *guard else {
            return (None, None, false);
        };

        unsafe {
            let mut active_win = None;
            if let Some(bytes) =
                x11_get_property_bytes(api, api.root, api.atom_net_active_window, 0)
            {
                if bytes.len() >= std::mem::size_of::<c_ulong>() {
                    let mut arr = [0u8; std::mem::size_of::<c_ulong>()];
                    arr.copy_from_slice(&bytes[..std::mem::size_of::<c_ulong>()]);
                    let win_id = c_ulong::from_ne_bytes(arr);
                    active_win = x11_inspect_window_recursive(api, win_id, 0);
                }
            }

            let mut root_ret: c_ulong = 0;
            let mut child_ret: c_ulong = 0;
            let mut root_x: c_int = 0;
            let mut root_y: c_int = 0;
            let mut win_x: c_int = 0;
            let mut win_y: c_int = 0;
            let mut mask: c_uint = 0;
            let mut hover_win = None;
            let mut button1_down = false;

            if (api.x_query_pointer)(
                api.dpy,
                api.root,
                &mut root_ret,
                &mut child_ret,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask,
            ) != 0
            {
                // Button1Mask in X11 is (1 << 8) = 256
                button1_down = (mask & (1 << 8)) != 0;
                let cur_xy = (root_x, root_y);
                let moved = *last_cursor != Some(cur_xy);
                if moved {
                    *last_cursor = Some(cur_xy);
                }
                if (moved || inspect_cursor_always || button1_down) && child_ret != 0 {
                    hover_win = x11_inspect_window_recursive(api, child_ret, 0);
                }
            }

            (active_win, hover_win, button1_down)
        }
    }

    #[cfg(test)]
    pub fn inspect_x11_window_id(win: c_ulong) -> Option<WindowTuple> {
        let lock = X11_API.get_or_init(|| Mutex::new(unsafe { init_x11_api() }));
        let guard = lock.lock().ok()?;
        let api = guard.as_ref()?;
        unsafe { x11_inspect_window_recursive(api, win, 0) }
    }

    // =========================================================================
    // 4. GNOME (`mutter` Wayland & Xorg) Auto-Installed Shell Extension + Introspect
    // =========================================================================

    static GNOME_EXT_INSTALLED: OnceLock<()> = OnceLock::new();

    pub fn is_gnome_session() -> bool {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_default()
            .to_ascii_lowercase();
        let session = std::env::var("DESKTOP_SESSION")
            .unwrap_or_default()
            .to_ascii_lowercase();
        desktop.contains("gnome") || session.contains("gnome") || desktop.contains("ubuntu")
    }

    pub async fn ensure_gnome_shell_extension_installed(conn: &zbus::Connection) {
        if !is_gnome_session() {
            return;
        }
        let mut first_time = false;
        GNOME_EXT_INSTALLED.get_or_init(|| {
            first_time = true;
            if let Some(home) = std::env::var_os("HOME") {
                let ext_uuid = "duckydeck-autoswitcher@duckydeck.github.io";
                let ext_dir = PathBuf::from(home)
                    .join(".local/share/gnome-shell/extensions")
                    .join(ext_uuid);
                if fs::create_dir_all(&ext_dir).is_ok() {
                    let metadata_json = r#"{
  "uuid": "duckydeck-autoswitcher@duckydeck.github.io",
  "name": "DuckyDeck Studio Auto-Switcher Bridge",
  "description": "Reports active window focus changes to DuckyDeck Studio over session D-Bus",
  "shell-version": ["42", "43", "44", "45", "46", "47", "48", "49"],
  "version": 1
}
"#;
                    let extension_js = r#"import Gio from 'gi://Gio';
import GLib from 'gi://GLib';

export default class DuckyDeckAutoSwitcherExtension {
    enable() {
        this._focusSig = global.display.connect('notify::focus-window', () => {
            this._sendFocusWindow();
        });
        this._sendFocusWindow();
    }

    _sendFocusWindow() {
        try {
            const win = global.display.focus_window;
            if (!win) return;
            const wmClass = win.get_wm_class() || '';
            const wmInstance = win.get_wm_class_instance() || wmClass;
            const title = win.get_title() || '';
            Gio.DBus.session.call(
                'io.github.duckydeck.Studio',
                '/KWinBridge',
                'io.github.duckydeck.KWinBridge',
                'NotifyActiveWindow',
                new GLib.Variant('(sss)', [wmClass, wmInstance, title]),
                null,
                Gio.DBusCallFlags.NONE,
                -1,
                null,
                null
            );
        } catch (e) {}
    }

    disable() {
        if (this._focusSig) {
            global.display.disconnect(this._focusSig);
            this._focusSig = null;
        }
    }
}
"#;
                    let _ = fs::write(ext_dir.join("metadata.json"), metadata_json);
                    let _ = fs::write(ext_dir.join("extension.js"), extension_js);
                }
            }
        });

        if first_time {
            if let Ok(proxy) = super::build_uncached_proxy(
                conn,
                "org.gnome.Shell.Extensions",
                "/org/gnome/Shell/Extensions",
                "org.gnome.Shell.Extensions",
            )
            .await
            {
                let _: std::result::Result<bool, _> = proxy
                    .call(
                        "EnableExtension",
                        &("duckydeck-autoswitcher@duckydeck.github.io",),
                    )
                    .await;
            }
        }
    }

    pub async fn query_gnome_introspect_active_window(
        conn: &zbus::Connection,
    ) -> Option<WindowTuple> {
        if !is_gnome_session() {
            return None;
        }
        let proxy = super::build_uncached_proxy(
            conn,
            "org.gnome.Shell.Introspect",
            "/org/gnome/Shell/Introspect",
            "org.gnome.Shell.Introspect",
        )
        .await
        .ok()?;

        let windows: std::collections::HashMap<
            u64,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        > = proxy.call("GetWindows", &()).await.ok()?;

        for props in windows.values() {
            let focused = props
                .get("focus")
                .and_then(|v| bool::try_from(v).ok())
                .unwrap_or(false);
            if !focused {
                continue;
            }
            let extract_str = |key: &str| -> String {
                props
                    .get(key)
                    .and_then(|v| <&str>::try_from(v).ok().map(|s| s.trim().to_string()))
                    .unwrap_or_default()
            };
            let wm_class = extract_str("wm-class");
            let wm_instance = extract_str("wm-class-instance");
            let app_id = extract_str("app-id").trim_end_matches(".desktop").to_string();
            let title = extract_str("title");

            let effective_class = if !wm_class.is_empty() {
                wm_class
            } else if !app_id.is_empty() {
                app_id
            } else {
                wm_instance.clone()
            };
            if effective_class.is_empty()
                || effective_class.to_ascii_lowercase().contains("duckydeck")
                || title.starts_with("duckyPad OLED —")
            {
                continue;
            }
            let res_name = if !wm_instance.is_empty() {
                wm_instance
            } else {
                effective_class.clone()
            };
            return Some((effective_class, res_name, title));
        }
        None
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) async fn build_uncached_proxy<'a>(
    conn: &'a zbus::Connection,
    dest: &'a str,
    path: &'a str,
    iface: &'a str,
) -> zbus::Result<zbus::Proxy<'a>> {
    zbus::proxy::Builder::new(conn)
        .destination(dest)?
        .path(path)?
        .interface(iface)?
        .cache_properties(zbus::proxy::CacheProperties::No)
        .build()
        .await
}

static KWIN_HOOK_RELOAD_REQUESTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(not(target_os = "windows"))]
static KWIN_HOOK_RELOAD_NOTIFY: std::sync::OnceLock<tokio::sync::Notify> =
    std::sync::OnceLock::new();

#[cfg(not(target_os = "windows"))]
fn kwin_hook_reload_notify() -> &'static tokio::sync::Notify {
    KWIN_HOOK_RELOAD_NOTIFY.get_or_init(tokio::sync::Notify::new)
}

pub fn request_kwin6_hook_reload() {
    KWIN_HOOK_RELOAD_REQUESTED.store(true, std::sync::atomic::Ordering::SeqCst);
    #[cfg(not(target_os = "windows"))]
    {
        kwin_hook_reload_notify().notify_one();
    }
}

fn read_unfocused_hover_enabled(state: &Arc<Mutex<SwitcherState>>) -> bool {
    state
        .lock()
        .map(|s| s.auto_switch_unfocused_hover)
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
pub async fn pick_window_via_kwin_dbus() -> std::result::Result<WindowChangeEvent, String> {
    let conn = zbus::Connection::session()
        .await
        .map_err(|e| format!("D-Bus session error: {e}"))?;

    // 1. Try native KWin interactive crosshair picker first (KDE Plasma 6 Wayland & X11)
    if let Ok(kwin_proxy) =
        build_uncached_proxy(&conn, "org.kde.KWin", "/KWin", "org.kde.KWin").await
    {
        if let Ok(info) = kwin_proxy
            .call::<_, _, std::collections::HashMap<String, zbus::zvariant::OwnedValue>>(
                "queryWindowInfo",
                &(),
            )
            .await
        {
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

            return Ok(WindowChangeEvent {
                resource_class: effective_class,
                resource_name,
                caption,
                matched_profile: None,
                picked: true,
                hid_rejected_not_on_device: false,
            });
        }
    }

    // 2. Multi-Desktop Linux Window Picker fallback for:
    //    - Omarchy / Hyprland (`HYPRLAND_INSTANCE_SIGNATURE`)
    //    - COSMIC (`cosmic-comp` Wayland `zwlr_foreign_toplevel_manager_v1` / `zcosmic_toplevel_info_v1`)
    //    - GNOME (`mutter` Wayland & Xorg via D-Bus Bridge / Introspect)
    //    - Cinnamon / Linux Mint / MATE / Xfce (X11 `_NET_ACTIVE_WINDOW` & `XQueryPointer` click)
    linux_desktop_window::ensure_gnome_shell_extension_installed(&conn).await;
    linux_desktop_window::ensure_wayland_toplevel_watcher_started();

    let baseline_bridge_seq = latest_bridge_window().map(|w| w.3).unwrap_or(0);
    let baseline_wayland_seq = linux_desktop_window::latest_wayland_active_window()
        .map(|w| w.3)
        .unwrap_or(0);
    let mut dummy_hypr_cursor = None;
    let baseline_hypr = linux_desktop_window::query_hyprland_window_and_hover(&mut dummy_hypr_cursor)
        .and_then(|(act, _)| act);
    let mut dummy_x11_cursor = None;
    let (baseline_x11, _, _) =
        linux_desktop_window::query_x11_window_and_hover(&mut dummy_x11_cursor, false);
    let baseline_gnome = linux_desktop_window::query_gnome_introspect_active_window(&conn).await;

    // Wait up to 15 seconds (300 * 50ms) for the user to click/focus a target window.
    for tick in 0..300u32 {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut picked: Option<(String, String, String)> = None;

        // A. Check D-Bus Bridge notification (e.g. GNOME Shell extension or external notifier)
        if let Some((cls, rname, cap, seq)) = latest_bridge_window() {
            if seq != baseline_bridge_seq {
                picked = Some((cls, rname, cap));
            }
        }

        // B. Check Omarchy / Hyprland active window change or cursor click
        if picked.is_none() && linux_desktop_window::is_hyprland_session() {
            let mut cur = None;
            if let Some(( Some(act), _)) =
                linux_desktop_window::query_hyprland_window_and_hover(&mut cur)
            {
                if Some(&act) != baseline_hypr.as_ref() {
                    picked = Some(act);
                }
            }
            // Also check X11 mouse click state on XWayland + Hyprland cursor hit-test
            let (_, _, b1_down) =
                linux_desktop_window::query_x11_window_and_hover(&mut dummy_x11_cursor, true);
            if picked.is_none() && b1_down && tick >= 3 {
                picked = linux_desktop_window::query_hyprland_window_at_current_cursor();
            }
        }

        // C. Check COSMIC / Wayland foreign-toplevel activation
        if picked.is_none() {
            if let Some((cls, rname, cap, seq)) =
                linux_desktop_window::latest_wayland_active_window()
            {
                if seq != baseline_wayland_seq {
                    picked = Some((cls, rname, cap));
                }
            }
        }

        // D. Check Cinnamon / Linux Mint / X11 cursor click (`Button1Mask`) or `_NET_ACTIVE_WINDOW` change
        if picked.is_none() {
            let (act_x11, hover_x11, b1_down) =
                linux_desktop_window::query_x11_window_and_hover(&mut dummy_x11_cursor, true);
            if b1_down && tick >= 3 {
                picked = hover_x11.or(act_x11);
            } else if act_x11.is_some() && act_x11 != baseline_x11 {
                picked = act_x11;
            }
        }

        // E. Check GNOME Shell Introspect focus change (every 150ms to keep D-Bus chatter minimal)
        if picked.is_none() && tick.is_multiple_of(3) {
            if let Some(act_gnome) =
                linux_desktop_window::query_gnome_introspect_active_window(&conn).await
            {
                if Some(&act_gnome) != baseline_gnome.as_ref() {
                    picked = Some(act_gnome);
                }
            }
        }

        if let Some((resource_class, resource_name, raw_caption)) = picked {
            let caption = resolve_full_browser_caption(&resource_class, &raw_caption);
            return Ok(WindowChangeEvent {
                resource_class,
                resource_name,
                caption,
                matched_profile: None,
                picked: true,
                hid_rejected_not_on_device: false,
            });
        }
    }

    Err("Window picker timed out (click the target window to focus it)".to_string())
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
                    let mut hid_rejected_not_on_device = false;
                    let mut paused = false;

                    if let Ok(mut st) = state_bg.lock() {
                        paused = st.picker_armed || st.paused_for_editing;
                        if !st.picker_armed {
                            st.current_window_class = effective_class.clone();
                            st.current_window_title = resolved_caption.clone();
                        }
                        if !st.picker_armed && st.enabled && !st.paused_for_editing {
                            let window_prof =
                                st.match_profile_for_window(&effective_class, &resolved_caption);
                            if let Some(wp) = window_prof {
                                st.last_window_matched_profile = Some(wp);
                            } else if st.active_modifier_override.is_none() {
                                st.last_window_matched_profile = None;
                            }

                            if st.active_modifier_override.is_none() {
                                matched_profile = window_prof;
                                if let Some(target) = matched_profile {
                                    match send_hid_switch_profile_checked(target) {
                                        HidSwitchOutcome::Switched => {
                                            st.last_switched_profile = target;
                                        }
                                        HidSwitchOutcome::NotLoadedOnHardware => {
                                            hid_rejected_not_on_device = true;
                                        }
                                        HidSwitchOutcome::DeviceUnavailable => {}
                                    }
                                }
                            }
                        }
                    }

                    dispatch_window_switch_to_secondary_devices(
                        &effective_class,
                        &resolved_caption,
                        paused,
                    );

                    if let Err(e) = ui_tx_bg.send(WindowChangeEvent {
                        resource_class: effective_class,
                        resource_name,
                        caption: resolved_caption,
                        matched_profile,
                        picked: false,
                        hid_rejected_not_on_device,
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

    #[cfg(target_os = "windows")]
    {
        let state_for_hover = Arc::clone(&state);
        let mut service = KWinBridgeService {
            state,
            ui_event_tx,
            pending,
            settle_notify,
        };
        let mut last_fg: Option<(String, String, String)> = None;
        let mut last_pt: Option<(i32, i32)> = None;
        let ipc_file = crate::paths::runtime_dir().join("duckydeck-ipc.cmd");
        let mut tick_counter: u32 = 0;
        loop {
            tick_counter = tick_counter.wrapping_add(1);
            if tick_counter.is_multiple_of(4) && ipc_file.is_file() {
                if let Ok(raw_cmd) = fs::read_to_string(&ipc_file) {
                    let _ = fs::remove_file(&ipc_file);
                    let trimmed = raw_cmd.trim();
                    if !trimmed.is_empty() {
                        let qcmd = cxx_qt_lib::QString::from(trimmed);
                        crate::bridge::device_controller::qobject::duckydeck_dispatch_ipc_command(
                            &qcmd,
                        );
                    }
                }
            }
            let unfocused_hover = read_unfocused_hover_enabled(&state_for_hover);
            // Poll active foreground window (and optional cursor hover window) on Windows every 50ms
            // SAFETY: GetForegroundWindow, GetCursorPos, and WindowFromPoint are thread-safe Win32 queries.
            let (fg_info, cursor_moved_info) = unsafe {
                let fg = win32_window::inspect_hwnd(win32_window::GetForegroundWindow());
                let mut hover = None;
                if unfocused_hover {
                    let mut pt = win32_window::POINT::default();
                    if win32_window::GetCursorPos(&mut pt) != 0 {
                        let cur_xy = (pt.x, pt.y);
                        if last_pt != Some(cur_xy) {
                            last_pt = Some(cur_xy);
                            hover = win32_window::inspect_hwnd(win32_window::WindowFromPoint(pt));
                        }
                    }
                } else {
                    last_pt = None;
                }
                (fg, hover)
            };
            if let Some((cls, rname, cap)) = cursor_moved_info {
                service.handle_active_window(cls, rname, cap);
            } else if fg_info != last_fg {
                last_fg = fg_info.clone();
                if let Some((cls, rname, cap)) = fg_info {
                    service.handle_active_window(cls, rname, cap);
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let pending_check = Arc::clone(&pending);
        let state_for_hook = Arc::clone(&state);
        let mut fallback_service = KWinBridgeService {
            state: Arc::clone(&state),
            ui_event_tx: ui_event_tx.clone(),
            pending: Arc::clone(&pending),
            settle_notify: Arc::clone(&settle_notify),
        };
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

        let mut active_kwin_hook: Option<(String, i32)> = None;
        let mut kwin_scripting_opt: Option<zbus::Proxy<'_>> = None;
        let mut kwin_introspect_opt: Option<zbus::Proxy<'_>> = None;

        if !linux_desktop_window::is_hyprland_session() {
            let initial_unfocused_hover = read_unfocused_hover_enabled(&state_for_hook);
            if let Ok(script_path) = write_kwin6_hook_script(initial_unfocused_hover) {
                if let (Ok(kwin_scripting), Ok(kwin_introspect)) = (
                    build_uncached_proxy(
                        &conn,
                        "org.kde.KWin",
                        "/Scripting",
                        "org.kde.kwin.Scripting",
                    )
                    .await,
                    build_uncached_proxy(
                        &conn,
                        "org.kde.KWin",
                        "/Scripting",
                        "org.freedesktop.DBus.Introspectable",
                    )
                    .await,
                ) {
                    KWIN_HOOK_REGISTERED.store(true, std::sync::atomic::Ordering::SeqCst);
                    active_kwin_hook = register_kwin6_hook_script(
                        &conn,
                        &kwin_scripting,
                        &kwin_introspect,
                        &script_path,
                        &pending_check,
                        true,
                        false,
                    )
                    .await;
                    if active_kwin_hook.is_none() {
                        KWIN_HOOK_REGISTERED.store(false, std::sync::atomic::Ordering::SeqCst);
                    }
                    kwin_scripting_opt = Some(kwin_scripting);
                    kwin_introspect_opt = Some(kwin_introspect);
                }
            }
        }

        if let (
            Some((mut active_plugin, mut active_script_id)),
            Some(kwin_scripting),
            Some(kwin_introspect),
        ) = (active_kwin_hook, kwin_scripting_opt, kwin_introspect_opt)
        {
            let notify = kwin_hook_reload_notify();
            loop {
                if !KWIN_HOOK_RELOAD_REQUESTED.load(std::sync::atomic::Ordering::SeqCst) {
                    tokio::select! {
                        _ = tokio::time::sleep(std::time::Duration::from_millis(30_000)) => {}
                        _ = notify.notified() => {}
                    }
                }
                let reload_requested =
                    KWIN_HOOK_RELOAD_REQUESTED.swap(false, std::sync::atomic::Ordering::SeqCst);

                let (is_loaded, node_alive) = if reload_requested {
                    (true, true)
                } else {
                    let loaded = kwin_scripting
                        .call::<_, _, bool>("isScriptLoaded", &(active_plugin.as_str(),))
                        .await
                        .unwrap_or(false);
                    let current_ids = introspect_kwin_script_ids_with_proxy(&kwin_introspect).await;
                    (loaded, current_ids.contains(&active_script_id))
                };

                if reload_requested || !is_loaded || !node_alive {
                    {
                        let _guard = KWIN_SCRIPT_LOCK.lock().await;
                        stop_and_unload_kwin_script(
                            &conn,
                            &kwin_scripting,
                            &active_plugin,
                            Some(active_script_id),
                        )
                        .await;
                    }
                    let unfocused_hover = read_unfocused_hover_enabled(&state_for_hook);
                    if let Ok(script_path) = write_kwin6_hook_script(unfocused_hover) {
                        if let Some((new_plugin, new_id)) = register_kwin6_hook_script(
                            &conn,
                            &kwin_scripting,
                            &kwin_introspect,
                            &script_path,
                            &pending_check,
                            false,
                            reload_requested,
                        )
                        .await
                        {
                            active_plugin = new_plugin;
                            active_script_id = new_id;
                        }
                    }
                }
            }
        }

        // Multi-Desktop Linux Auto-Switcher Loop for:
        // - Omarchy / Hyprland (`HYPRLAND_INSTANCE_SIGNATURE`)
        // - COSMIC (`cosmic-comp` Wayland `zwlr_foreign_toplevel_manager_v1` / `zcosmic_toplevel_info_v1`)
        // - GNOME (`mutter` Wayland & Xorg via auto-installed Shell extension + Introspect)
        // - Cinnamon / Linux Mint / MATE / Xfce (X11 `_NET_ACTIVE_WINDOW` & `XQueryPointer`)
        linux_desktop_window::ensure_gnome_shell_extension_installed(&conn).await;
        linux_desktop_window::ensure_wayland_toplevel_watcher_started();

        crate::bridge::device_controller::record_system_event(
            "INFO",
            "KWIN",
            "Multi-desktop Linux Auto-Switcher active (Omarchy/Hyprland, COSMIC, GNOME, Cinnamon/X11)",
        );

        let mut last_fg: Option<(String, String, String)> = None;
        let mut last_hypr_cursor: Option<(i64, i64)> = None;
        let mut last_x11_cursor: Option<(i32, i32)> = None;
        let mut last_wayland_seq: u64 = 0;
        let mut tick_counter: u32 = 0;

        loop {
            tick_counter = tick_counter.wrapping_add(1);
            let unfocused_hover = read_unfocused_hover_enabled(&state_for_hook);
            if !unfocused_hover {
                last_hypr_cursor = None;
                last_x11_cursor = None;
            }
            let mut fg_info: Option<(String, String, String)> = None;
            let mut hover_info: Option<(String, String, String)> = None;

            // 1. Omarchy / Hyprland IPC socket (`j/activewindow` + optional `j/cursorpos` + `j/clients`)
            if let Some((hypr_act, hypr_hover)) =
                linux_desktop_window::query_hyprland_window_and_hover(&mut last_hypr_cursor)
            {
                fg_info = hypr_act;
                if unfocused_hover {
                    hover_info = hypr_hover;
                }
            }

            // 2. COSMIC / wlroots / Sway / Niri Wayland foreign-toplevel watcher
            if fg_info.is_none() {
                if let Some((cls, rname, cap, seq)) =
                    linux_desktop_window::latest_wayland_active_window()
                {
                    if seq != last_wayland_seq {
                        last_wayland_seq = seq;
                    }
                    fg_info = Some((cls, rname, cap));
                }
            }

            // 3. Cinnamon / Linux Mint / MATE / Xfce / Xorg (`_NET_ACTIVE_WINDOW` + optional `XQueryPointer`)
            if fg_info.is_none() || (unfocused_hover && hover_info.is_none()) {
                let (x11_act, x11_hover, _) =
                    linux_desktop_window::query_x11_window_and_hover(&mut last_x11_cursor, false);
                if fg_info.is_none() {
                    fg_info = x11_act;
                }
                if unfocused_hover && hover_info.is_none() {
                    hover_info = x11_hover;
                }
            }

            // 4. GNOME Shell Introspect fallback (every 150ms when not already reported)
            if fg_info.is_none() && tick_counter.is_multiple_of(3) {
                fg_info = linux_desktop_window::query_gnome_introspect_active_window(&conn).await;
            }

            if let Some((cls, rname, cap)) = hover_info {
                fallback_service.handle_active_window(cls, rname, cap);
            } else if fg_info != last_fg {
                last_fg = fg_info.clone();
                if let Some((cls, rname, cap)) = fg_info {
                    fallback_service.handle_active_window(cls, rname, cap);
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn write_kwin6_hook_script(unfocused_hover: bool) -> std::io::Result<String> {
    let initial_rect_js = match load_saved_oled_geometry_tuple() {
        Some((gx, gy, gw, gh)) => {
            format!("{{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }}")
        }
        None => "null".to_string(),
    };
    let initial_main_rect_js = match load_saved_main_window_geometry_tuple() {
        Some((gx, gy, gw, gh)) => {
            format!("{{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }}")
        }
        None => "null".to_string(),
    };
    let js_code = generate_kwin6_hook_js(unfocused_hover, &initial_rect_js, &initial_main_rect_js);
    let file_path = crate::paths::kwin_runtime_dir()
        .join("duckydeck-kwin6-hook.js")
        .to_string_lossy()
        .to_string();

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(js_code.as_bytes())?;
    Ok(file_path)
}

#[cfg(not(target_os = "windows"))]
fn generate_kwin6_hook_js(
    unfocused_hover: bool,
    initial_rect_js: &str,
    initial_main_rect_js: &str,
) -> String {
    let hover_functions_js = if unfocused_hover {
        r#"
var lastMotionSentMs = 0;

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
    var now = Date.now();
    if ((now - lastMotionSentMs) < 120) return;
    lastMotionSentMs = now;
    var pt = workspace.cursorPos;
    if (!pt) return;
    var topWin = topWindowAtPoint(pt);
    if (!topWin || isDesktopOrPanelWindow(topWin)) {
        if (lastCursorKey !== "plasmashell::Desktop") {
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
    if (lastCursorKey !== key) {
        onWindowActivated(topWin);
    }
}
"#
    } else {
        ""
    };

    let initial_hover_check_js = if unfocused_hover {
        r#"    if (workspace.cursorPos) {
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
"#
    } else {
        ""
    };

    let cursor_connect_js = if unfocused_hover {
        r#"if (workspace.cursorPosChanged) {
    workspace.cursorPosChanged.connect(onCursorPosChanged);
}
"#
    } else {
        ""
    };

    format!(
        r#"
var trackedWindow = null;
var hookedWindows = [];
var lastCursorKey = "";
var initialSavedOledRect = {initial_rect_js};
var initialSavedMainRect = {initial_main_rect_js};

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
    if (win.dialog || win.modal || win.transient || win.utility || win.splash || win.tooltip || win.notification) return false;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var name = (win.resourceName ? win.resourceName.toString() : "").toLowerCase();
    if (cls.indexOf("duckydeck") === -1 && name.indexOf("duckydeck") === -1) return false;
    var cap = win.caption ? win.caption.toString() : "";
    return cap.indexOf("DuckyDeck Studio v") === 0 || cap === "DuckyDeck Studio";
}}

function checkOledKeepAbove(win) {{
    if (!win) return;
    if (isOledWidgetWindow(win)) {{
        var cap = win.caption ? win.caption.toString() : "";
        var isPinned = (cap.indexOf("[Unpinned]") === -1);
        var isAllDesktops = (cap.indexOf("[SingleDesktop]") === -1);
        win.keepAbove = isPinned;
        win.onAllDesktops = isAllDesktops;
        win.skipTaskbar = isPinned;
        win.skipPager = isPinned;
        if (isPinned && win.minimized) {{
            win.minimized = false;
        }}
        if (!isWindowHooked(win)) {{
            hookedWindows.push(win);
            try {{
                if (win.closed) {{
                    win.closed.connect(function() {{
                        var idx = hookedWindows.indexOf(win);
                        if (idx !== -1) {{
                            hookedWindows.splice(idx, 1);
                        }}
                    }});
                }}
                var trackedOledX = initialSavedOledRect ? initialSavedOledRect.x : Math.round(win.frameGeometry.x);
                var trackedOledTopY = initialSavedOledRect ? initialSavedOledRect.y : Math.round(win.frameGeometry.y);
                var trackedOledW = initialSavedOledRect ? initialSavedOledRect.width : Math.round(win.frameGeometry.width);
                var trackedOledH = initialSavedOledRect ? initialSavedOledRect.height : Math.round(win.frameGeometry.height);
                var isAdjustingOledGeometry = false;
                if (initialSavedOledRect) {{
                    isAdjustingOledGeometry = true;
                    try {{
                        win.frameGeometry = {{
                            x: initialSavedOledRect.x,
                            y: initialSavedOledRect.y,
                            width: initialSavedOledRect.width,
                            height: initialSavedOledRect.height
                        }};
                    }} catch (e) {{}}
                    isAdjustingOledGeometry = false;
                    trackedOledX = initialSavedOledRect.x;
                    trackedOledTopY = initialSavedOledRect.y;
                    trackedOledW = initialSavedOledRect.width;
                    trackedOledH = initialSavedOledRect.height;
                }}
                if (isAllDesktops) {{
                    win.onAllDesktops = false;
                    win.onAllDesktops = true;
                }} else {{
                    win.onAllDesktops = false;
                }}
                win.keepAbove = isPinned;
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
                if (win.moveResizedChanged) {{
                    win.moveResizedChanged.connect(function() {{
                        if (win.frameGeometry && (win.move || win.resize)) {{
                            trackedOledX = Math.round(win.frameGeometry.x);
                            trackedOledTopY = Math.round(win.frameGeometry.y);
                            trackedOledW = Math.round(win.frameGeometry.width);
                            trackedOledH = Math.round(win.frameGeometry.height);
                        }}
                    }});
                }}
                if (win.frameGeometryChanged) {{
                    win.frameGeometryChanged.connect(function() {{
                        if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 180 && win.frameGeometry.height >= 220) {{
                            if (isAdjustingOledGeometry) return;

                            var curX = Math.round(win.frameGeometry.x);
                            var curY = Math.round(win.frameGeometry.y);
                            var curW = Math.round(win.frameGeometry.width);
                            var curH = Math.round(win.frameGeometry.height);

                            var onlyHeightChanged = (curX === trackedOledX && curW === trackedOledW && curH !== trackedOledH);
                            if (win.move || win.resize || !onlyHeightChanged) {{
                                trackedOledTopY = curY;
                            }} else if (trackedOledTopY !== null && Math.abs(curY - trackedOledTopY) >= 1) {{
                                isAdjustingOledGeometry = true;
                                try {{
                                    win.frameGeometry = {{
                                        x: curX,
                                        y: trackedOledTopY,
                                        width: curW,
                                        height: curH
                                    }};
                                    curY = trackedOledTopY;
                                }} catch (e) {{}}
                                isAdjustingOledGeometry = false;
                            }} else if (trackedOledTopY === null) {{
                                trackedOledTopY = curY;
                            }}

                            trackedOledX = curX;
                            trackedOledW = curW;
                            trackedOledH = curH;
                            initialSavedOledRect = {{
                                x: curX,
                                y: curY,
                                width: curW,
                                height: curH
                            }};

                            callDBus(
                                "io.github.duckydeck.Studio",
                                "/KWinBridge",
                                "io.github.duckydeck.KWinBridge",
                                "NotifyOledWidgetGeometry",
                                curX,
                                curY,
                                curW,
                                curH
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
                if (win.closed) {
                    win.closed.connect(function() {
                        var idx = hookedWindows.indexOf(win);
                        if (idx !== -1) {
                            hookedWindows.splice(idx, 1);
                        }
                    });
                }
                var mainMappedAtMs = Date.now();
                var isAdjustingMainGeometry = false;
                if (initialSavedMainRect && win.frameGeometry) {
                    isAdjustingMainGeometry = true;
                    try {
                        var curW0 = Math.round(win.frameGeometry.width);
                        var curH0 = Math.round(win.frameGeometry.height);
                        var targetW0 = curW0 >= 600 ? curW0 : initialSavedMainRect.width;
                        var targetH0 = (initialSavedMainRect.height && initialSavedMainRect.height >= 680)
                            ? initialSavedMainRect.height
                            : (curH0 >= 400 ? curH0 : 800);
                        win.frameGeometry = {
                            x: initialSavedMainRect.x,
                            y: initialSavedMainRect.y,
                            width: targetW0,
                            height: targetH0
                        };
                    } catch (e) {}
                    isAdjustingMainGeometry = false;
                } else if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 600 && win.frameGeometry.height >= 400) {
                    var initX = Math.round(win.frameGeometry.x);
                    var initY = Math.round(win.frameGeometry.y);
                    var initW = Math.round(win.frameGeometry.width);
                    var initH = Math.round(win.frameGeometry.height);
                    initialSavedMainRect = {
                        x: initX,
                        y: initY,
                        width: initW,
                        height: initH
                    };
                    callDBus(
                        "io.github.duckydeck.Studio",
                        "/KWinBridge",
                        "io.github.duckydeck.KWinBridge",
                        "NotifyMainWindowGeometry",
                        initX,
                        initY,
                        initW,
                        initH
                    );
                }
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
                            mainMappedAtMs = Date.now();
                            if (initialSavedMainRect && win.frameGeometry) {
                                isAdjustingMainGeometry = true;
                                try {
                                    win.frameGeometry = {
                                        x: initialSavedMainRect.x,
                                        y: initialSavedMainRect.y,
                                        width: Math.round(win.frameGeometry.width),
                                        height: Math.round(win.frameGeometry.height)
                                    };
                                } catch (e) {}
                                isAdjustingMainGeometry = false;
                            }
                        }
                    });
                }
                if (win.frameGeometryChanged) {
                    win.frameGeometryChanged.connect(function() {
                        if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 600 && win.frameGeometry.height >= 400) {
                            if (isAdjustingMainGeometry) return;

                            var curX = Math.round(win.frameGeometry.x);
                            var curY = Math.round(win.frameGeometry.y);
                            var curW = Math.round(win.frameGeometry.width);
                            var curH = Math.round(win.frameGeometry.height);

                            if (!win.move && !win.resize && initialSavedMainRect && (Date.now() - mainMappedAtMs) < 800) {
                                if (Math.abs(curX - initialSavedMainRect.x) >= 1 || Math.abs(curY - initialSavedMainRect.y) >= 1) {
                                    isAdjustingMainGeometry = true;
                                    try {
                                        win.frameGeometry = {
                                            x: initialSavedMainRect.x,
                                            y: initialSavedMainRect.y,
                                            width: curW,
                                            height: curH
                                        };
                                        curX = initialSavedMainRect.x;
                                        curY = initialSavedMainRect.y;
                                    } catch (e) {}
                                    isAdjustingMainGeometry = false;
                                }
                            }

                            initialSavedMainRect = {
                                x: curX,
                                y: curY,
                                width: curW,
                                height: curH
                            };

                            callDBus(
                                "io.github.duckydeck.Studio",
                                "/KWinBridge",
                                "io.github.duckydeck.KWinBridge",
                                "NotifyMainWindowGeometry",
                                curX,
                                curY,
                                curW,
                                curH
                            );
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
"# + hover_functions_js
        + r#"
function sendInitialTopWindow() {
    var order = workspace.stackingOrder || (workspace.windowList ? workspace.windowList() : null);
    if (order && order.length) {
        for (var j = 0; j < order.length; ++j) {
            checkOledKeepAbove(order[j]);
        }
    }
"# + initial_hover_check_js
        + r#"    if (workspace.activeWindow && !isDuckyDeckWindow(workspace.activeWindow)) {
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

function onWindowAdded(win) {
    if (win) {
        var idx = hookedWindows.indexOf(win);
        if (idx !== -1) {
            hookedWindows.splice(idx, 1);
        }
    }
    checkOledKeepAbove(win);
}

if (workspace.windowAdded) {
    workspace.windowAdded.connect(onWindowAdded);
}
"# + cursor_connect_js
        + r#"if (workspace.windowActivated) {
    workspace.windowActivated.connect(onWindowActivated);
    sendInitialTopWindow();
} else if (workspace.clientActivated) {
    workspace.clientActivated.connect(onWindowActivated);
    onWindowActivated(workspace.activeClient);
}
"#
}

fn load_saved_main_window_geometry_tuple() -> Option<(i32, i32, i32, i32)> {
    let path = crate::paths::config_dir().join("main_window_geometry.json");
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
    let w = (val.get("width").and_then(|v| v.as_i64()).unwrap_or(1060) as i32).clamp(680, 3840);
    let h = (val.get("height").and_then(|v| v.as_i64()).unwrap_or(800) as i32).clamp(680, 2160);
    Some((x, y, w, h))
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

fn load_saved_oled_on_all_desktops() -> bool {
    let path = crate::paths::config_dir().join("oled_widget_geometry.json");
    let Ok(raw) = fs::read_to_string(path) else {
        return true;
    };
    let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return true;
    };
    val.get("on_all_desktops")
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

#[cfg(not(target_os = "windows"))]
static KWIN_SCRIPT_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

#[cfg(not(target_os = "windows"))]
static ACTIVE_KWIN_HOOK_SLOT: std::sync::LazyLock<std::sync::Mutex<Option<(String, i32)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

#[cfg(not(target_os = "windows"))]
fn kwin_hook_sid_path() -> PathBuf {
    crate::paths::kwin_runtime_dir().join("duckydeck-kwin6-hook.sid")
}

#[cfg(not(target_os = "windows"))]
fn load_persisted_kwin_hook_sid() -> Option<(String, i32)> {
    let raw = fs::read_to_string(kwin_hook_sid_path()).ok()?;
    let line = raw.lines().next()?.trim();
    let (plugin, id_str) = line.split_once('\t')?;
    let plugin = plugin.trim();
    if plugin.is_empty() {
        return None;
    }
    let id = id_str.trim().parse::<i32>().ok()?;
    if id < 0 {
        return None;
    }
    Some((plugin.to_string(), id))
}

#[cfg(not(target_os = "windows"))]
fn save_persisted_kwin_hook_sid(plugin_name: &str, script_id: i32) {
    if let Ok(mut guard) = ACTIVE_KWIN_HOOK_SLOT.lock() {
        *guard = Some((plugin_name.to_string(), script_id));
    }
    let sid_path = kwin_hook_sid_path();
    let payload = format!("{plugin_name}\t{script_id}\n");
    let _ = fs::write(sid_path, payload.as_bytes());
}

#[cfg(not(target_os = "windows"))]
async fn stop_and_unload_kwin_script(
    conn: &zbus::Connection,
    kwin_scripting: &zbus::Proxy<'_>,
    plugin_name: &str,
    script_id: Option<i32>,
) {
    if let Some(id) = script_id.filter(|&i| i >= 0) {
        let script_obj_path = format!("/Scripting/Script{id}");
        if let Ok(script_proxy) = build_uncached_proxy(
            conn,
            "org.kde.KWin",
            script_obj_path.as_str(),
            "org.kde.kwin.Script",
        )
        .await
        {
            let _: std::result::Result<(), _> = script_proxy.call("stop", &()).await;
        }
    }
    let _: std::result::Result<bool, _> = kwin_scripting
        .call("unloadScript", &(plugin_name,))
        .await;
}

#[cfg(not(target_os = "windows"))]
async fn introspect_kwin_script_ids_with_proxy(
    kwin_introspect: &zbus::Proxy<'_>,
) -> std::collections::BTreeSet<i32> {
    let mut ids = std::collections::BTreeSet::new();
    let Ok(xml) = kwin_introspect.call::<_, _, String>("Introspect", &()).await else {
        return ids;
    };
    for line in xml.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("<node name=\"Script") {
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(id) = digits.parse::<i32>() {
                ids.insert(id);
            }
        }
    }
    ids
}

#[cfg(not(target_os = "windows"))]
async fn register_kwin6_hook_script(
    conn: &zbus::Connection,
    kwin_scripting: &zbus::Proxy<'_>,
    kwin_introspect: &zbus::Proxy<'_>,
    script_path: &str,
    pending_check: &Arc<Mutex<PendingWindowSwitch>>,
    initial_startup: bool,
    reload_requested: bool,
) -> Option<(String, i32)> {
    let base_plugin_name = "duckydeck_kwin6_autoswitcher";
    let pid_fallback_name = format!("duckydeck_kwin6_autoswitcher_{}", std::process::id());

    // Cleanly stop & unload any active/persisted DuckyDeck KWin hook script first without
    // firing 24 blind `_slot_{1..23}` unloadScript D-Bus calls unless a collision actually occurs.
    {
        let _guard = KWIN_SCRIPT_LOCK.lock().await;
        let mut prior_slots: Vec<(String, i32)> = Vec::new();
        if let Ok(guard) = ACTIVE_KWIN_HOOK_SLOT.lock() {
            if let Some(entry) = guard.clone() {
                prior_slots.push(entry);
            }
        }
        if let Some(persisted) = load_persisted_kwin_hook_sid() {
            if !prior_slots.iter().any(|(p, id)| p == &persisted.0 && *id == persisted.1) {
                prior_slots.push(persisted);
            }
        }
        let mut unloaded_names: Vec<String> = Vec::new();
        for (prev_plugin, prev_id) in prior_slots {
            stop_and_unload_kwin_script(conn, kwin_scripting, &prev_plugin, Some(prev_id)).await;
            unloaded_names.push(prev_plugin);
        }
        for known_name in [
            "duckydeck_oled_keep_above_oneshot",
            base_plugin_name,
            pid_fallback_name.as_str(),
        ] {
            if unloaded_names.iter().any(|n| n == known_name) {
                continue;
            }
            if kwin_scripting
                .call::<_, _, bool>("isScriptLoaded", &(known_name,))
                .await
                .unwrap_or(false)
            {
                let _: std::result::Result<bool, _> =
                    kwin_scripting.call("unloadScript", &(known_name,)).await;
            }
        }
    }

    if let Ok(mut p) = pending_check.lock() {
        p.raw_class.clear();
        p.settled_and_dispatched = false;
    }

    for attempt in 0..24 {
        let candidate_name = if attempt == 0 {
            base_plugin_name.to_string()
        } else {
            format!("{base_plugin_name}_slot_{attempt}")
        };

        let seq_before = KWIN_NOTIFY_SEQ.load(std::sync::atomic::Ordering::SeqCst);
        let mut run_succeeded = false;
        let mut assigned_id: i32 = -1;
        {
            let _guard = KWIN_SCRIPT_LOCK.lock().await;
            if attempt > 0
                && kwin_scripting
                    .call::<_, _, bool>("isScriptLoaded", &(candidate_name.as_str(),))
                    .await
                    .unwrap_or(false)
            {
                let _: std::result::Result<bool, _> = kwin_scripting
                    .call("unloadScript", &(candidate_name.as_str(),))
                    .await;
            }
            let pre_ids = introspect_kwin_script_ids_with_proxy(kwin_introspect).await;
            let script_id: i32 = kwin_scripting
                .call("loadScript", &(script_path, candidate_name.as_str()))
                .await
                .unwrap_or(-1);

            if script_id >= 0 {
                assigned_id = script_id;
                if pre_ids.contains(&script_id) {
                    // `/Scripting/Script{script_id}` was already owned by another script before `loadScript`!
                    // Keep `candidate_name` in KWin's `scripts` list as an inert gap-filler to advance
                    // `scripts.size()`, and do NOT call `.run()` on the foreign D-Bus object.
                    continue;
                }
                let script_obj_path = format!("/Scripting/Script{script_id}");
                if let Ok(kwin_script) = build_uncached_proxy(
                    conn,
                    "org.kde.KWin",
                    script_obj_path.as_str(),
                    "org.kde.kwin.Script",
                )
                .await
                {
                    match kwin_script.call::<_, _, ()>("run", &()).await {
                        Ok(()) => {
                            run_succeeded = true;
                        }
                        Err(e) => {
                            let _: std::result::Result<(), _> =
                                kwin_script.call("stop", &()).await;
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "KWIN",
                                format!(
                                    "KWin Script{script_id} slot collision on attempt {attempt} ({e}); advancing to next slot"
                                ),
                            );
                        }
                    }
                }
            }
        }

        if !run_succeeded || assigned_id < 0 {
            continue;
        }

        // Wait up to 400ms for sendInitialTopWindow() to deliver the initial window over D-Bus.
        let mut received_initial = false;
        for _ in 0..16 {
            tokio::time::sleep(std::time::Duration::from_millis(25)).await;
            let seq_now = KWIN_NOTIFY_SEQ.load(std::sync::atomic::Ordering::SeqCst);
            let has_class = pending_check
                .lock()
                .map(|p| !p.raw_class.is_empty())
                .unwrap_or(false);
            if seq_now > seq_before || has_class {
                received_initial = true;
                break;
            }
        }
        if received_initial {
            save_persisted_kwin_hook_sid(&candidate_name, assigned_id);
            let msg = if initial_startup {
                format!(
                    "KWin 6 Auto-Switcher hook active (plugin '{candidate_name}', Script{assigned_id})"
                )
            } else if reload_requested {
                format!(
                    "KWin 6 Auto-Switcher hook reloaded after settings update (plugin '{candidate_name}', Script{assigned_id})"
                )
            } else {
                format!(
                    "KWin 6 Auto-Switcher hook self-healed & re-registered (plugin '{candidate_name}', Script{assigned_id})"
                )
            };
            crate::bridge::device_controller::record_system_event("INFO", "KWIN", msg);
            return Some((candidate_name, assigned_id));
        }
    }
    None
}

pub fn apply_kwin_oled_keep_above(keep_above: bool, restore_geometry: bool) {
    let on_all_desktops = load_saved_oled_on_all_desktops();
    #[cfg(target_os = "windows")]
    {
        let geom_opt = if restore_geometry {
            load_saved_oled_geometry_tuple()
        } else {
            None
        };
        win32_window::apply_oled_topmost_and_geometry(keep_above, on_all_desktops, geom_opt);
        return;
    }
    #[cfg(not(target_os = "windows"))]
    {
        if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        std::thread::spawn(move || {
            if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
                return;
            }
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
                let Ok(kwin_scripting) = build_uncached_proxy(
                    &conn,
                    "org.kde.KWin",
                    "/Scripting",
                    "org.kde.kwin.Scripting",
                )
                .await
                else {
                    return;
                };

                // If our persistent KWin 6 hook script is already loaded, it automatically enforces
                // keepAbove, onAllDesktops, skipTaskbar, skipPager, and saved OLED geometry via
                // workspace.windowAdded and win.captionChanged without creating oneshot Script{id} churn.
                if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst)
                    || kwin_scripting
                        .call::<_, _, bool>("isScriptLoaded", &("duckydeck_kwin6_autoswitcher",))
                        .await
                        .unwrap_or(false)
                {
                    return;
                }
                if let Ok(guard) = ACTIVE_KWIN_HOOK_SLOT.lock() {
                    if let Some((ref active_name, _)) = *guard {
                        if kwin_scripting
                            .call::<_, _, bool>("isScriptLoaded", &(active_name.as_str(),))
                            .await
                            .unwrap_or(false)
                        {
                            return;
                        }
                    }
                }

                let script_path = crate::paths::kwin_runtime_dir()
                    .join("duckydeck-kwin6-oled-pin.js")
                    .to_string_lossy()
                    .to_string();
                let flag_str = if keep_above { "true" } else { "false" };
                let all_desk_str = if on_all_desktops { "true" } else { "false" };
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
            var isPinned = (cap.indexOf("[Unpinned]") === -1) && {flag_str};
            var isAllDesktops = (cap.indexOf("[SingleDesktop]") === -1) && {all_desk_str};
            if (isPinned && w.minimized) {{
                w.minimized = false;
            }}
            {geom_js}
            if (isAllDesktops) {{
                w.onAllDesktops = false;
                w.onAllDesktops = true;
            }} else {{
                w.onAllDesktops = false;
            }}
            w.keepAbove = isPinned;
            w.skipTaskbar = isPinned;
            w.skipPager = isPinned;
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
                    if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
                        return;
                    }
                    let _guard = KWIN_SCRIPT_LOCK.lock().await;
                    if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
                        return;
                    }
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
                            let script_proxy_opt = build_uncached_proxy(
                                &conn,
                                "org.kde.KWin",
                                obj_path.as_str(),
                                "org.kde.kwin.Script",
                            )
                            .await
                            .ok();
                            if let Some(ref script_proxy) = script_proxy_opt {
                                if let Err(e) = script_proxy.call::<_, _, ()>("run", &()).await {
                                    crate::bridge::device_controller::record_system_event(
                                        "WARN",
                                        "KWIN",
                                        format!("Failed running KWin OLED keep-above script: {e}"),
                                    );
                                }
                            }
                            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                            if let Some(ref script_proxy) = script_proxy_opt {
                                let _: std::result::Result<(), _> =
                                    script_proxy.call("stop", &()).await;
                            }
                            let _: std::result::Result<bool, _> =
                                kwin_scripting.call("unloadScript", &(plugin_name,)).await;
                        }
                    }
                }
            });
        });
    }
}

#[cfg(test)]
mod modifier_engine_tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_hold_mode_activates_after_delay_and_restores_on_release() {
        let mut engine = HostKeyboardModifierEngine::new();
        let rules = vec![ModifierLayerRule {
            enabled: true,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 8,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        }];

        let mut held = HashSet::new();
        held.insert(97u16); // KEY_RIGHTCTRL

        // Press at t=1000 -> pending hold
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[97], 0, 0, 1000),
            None
        );
        // At t=1150 (< 200ms) -> still waiting
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1150),
            None
        );
        // At t=1210 (>= 200ms) -> activates Profile #8
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1210),
            Some(ModifierLayerTransition::Activated {
                target_profile: 8,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
            })
        );

        // Release RCTRL at t=1500 -> deactivates
        held.clear();
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1500),
            Some(ModifierLayerTransition::Deactivated)
        );
    }

    #[test]
    fn test_hold_mode_chord_guard_cancels_when_another_key_pressed() {
        let mut engine = HostKeyboardModifierEngine::new();
        let rules = vec![ModifierLayerRule {
            enabled: true,
            trigger_key: "LCTRL".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 8,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        }];

        let mut held = HashSet::new();
        held.insert(29u16); // KEY_LEFTCTRL

        // Press LCTRL at t=1000
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[29], 0, 0, 1000),
            None
        );

        // User presses 'C' (evdev 46) at t=1080 for Ctrl+C
        held.insert(46u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[46], 0, 0, 1080),
            None
        );

        // Release 'C' at t=1140 while still holding LCTRL past 200ms -> must NOT activate!
        held.remove(&46u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1250),
            None
        );
    }

    #[test]
    fn test_oneshot_mode_activates_on_tap_and_completes_after_duckypad_key() {
        let mut engine = HostKeyboardModifierEngine::new();
        let rules = vec![ModifierLayerRule {
            enabled: true,
            trigger_key: "MENU".to_string(),
            mode: "oneshot".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 5,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        }];

        let mut held = HashSet::new();
        held.insert(127u16); // KEY_COMPOSE (Menu)

        // Press Menu at t=2000
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[127], 0, 0, 2000),
            None
        );

        // Release Menu cleanly at t=2090 -> One-Shot activates!
        held.clear();
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 2090),
            Some(ModifierLayerTransition::Activated {
                target_profile: 5,
                trigger_key: "MENU".to_string(),
                mode: "oneshot".to_string(),
            })
        );

        // User presses a duckyPad key at t=3000 (duckypad_keys_held = 1, last_duckypad_keydown_ms = 3000)
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 1, 3000, 3000),
            None
        );

        // User releases the duckyPad key at t=3100 (duckypad_keys_held = 0) -> One-Shot completes & restores!
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 3000, 3100),
            Some(ModifierLayerTransition::Deactivated)
        );
    }

    #[test]
    fn test_toggle_mode_locks_and_unlocks() {
        let mut engine = HostKeyboardModifierEngine::new();
        let rules = vec![ModifierLayerRule {
            enabled: true,
            trigger_key: "MENU".to_string(),
            mode: "toggle".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 4,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        }];

        let mut held = HashSet::new();
        held.insert(127u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[127], 0, 0, 1000),
            None
        );
        held.clear();
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1080),
            Some(ModifierLayerTransition::Activated {
                target_profile: 4,
                trigger_key: "MENU".to_string(),
                mode: "toggle".to_string(),
            })
        );

        // Tap Menu again at t=5000 to unlock
        held.insert(127u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[127], 0, 0, 5000),
            None
        );
        held.clear();
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 5080),
            Some(ModifierLayerTransition::Deactivated)
        );
    }

    #[test]
    fn test_app_and_title_scoped_modifier_rules_override_global_all_rule() {
        let mut engine = HostKeyboardModifierEngine::new();
        let rules = vec![
            // 1. Global ("All Applications") fallback for RCTRL -> Profile #2
            ModifierLayerRule {
                enabled: true,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
                hold_delay_ms: 200,
                oneshot_timeout_sec: 6,
                target_profile: 2,
                resource_class: String::new(),
                title_contains: None,
                title_is_regex: false,
            },
            // 2. All Web Browsers (*browser*) rule for RCTRL -> Profile #4
            ModifierLayerRule {
                enabled: true,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
                hold_delay_ms: 200,
                oneshot_timeout_sec: 6,
                target_profile: 4,
                resource_class: "*browser*".to_string(),
                title_contains: None,
                title_is_regex: false,
            },
            // 3. Specific App ("firefox") rule for RCTRL -> Profile #5
            ModifierLayerRule {
                enabled: true,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
                hold_delay_ms: 200,
                oneshot_timeout_sec: 6,
                target_profile: 5,
                resource_class: "firefox".to_string(),
                title_contains: None,
                title_is_regex: false,
            },
            // 4. Specific App + Title ("firefox" + "YouTube") rule for RCTRL -> Profile #6
            ModifierLayerRule {
                enabled: true,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
                hold_delay_ms: 200,
                oneshot_timeout_sec: 6,
                target_profile: 6,
                resource_class: "firefox".to_string(),
                title_contains: Some("YouTube".to_string()),
                title_is_regex: false,
            },
        ];

        let mut held = HashSet::new();
        held.insert(97u16); // KEY_RIGHTCTRL

        // A. In Firefox on YouTube -> activates Profile #6 (App + Title specificity)
        assert_eq!(
            engine.evaluate_tick_for_window(
                true,
                false,
                &rules,
                "firefox",
                "Watch Rust Tutorials - YouTube — Mozilla Firefox",
                &held,
                &[97],
                0,
                0,
                1000,
            ),
            None
        );
        assert_eq!(
            engine.evaluate_tick_for_window(
                true,
                false,
                &rules,
                "firefox",
                "Watch Rust Tutorials - YouTube — Mozilla Firefox",
                &held,
                &[],
                0,
                0,
                1220,
            ),
            Some(ModifierLayerTransition::Activated {
                target_profile: 6,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
            })
        );

        // B. While still holding RCTRL, user switches tab in Firefox to GitHub -> shifts to Profile #5!
        assert_eq!(
            engine.evaluate_tick_for_window(
                true,
                false,
                &rules,
                "firefox",
                "duckydeck-studio — GitHub — Mozilla Firefox",
                &held,
                &[],
                0,
                0,
                1300,
            ),
            Some(ModifierLayerTransition::Activated {
                target_profile: 5,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
            })
        );

        // C. While still holding RCTRL, user focuses Chrome -> shifts to Profile #4 (*browser*)!
        assert_eq!(
            engine.evaluate_tick_for_window(
                true,
                false,
                &rules,
                "google-chrome",
                "Docs — Google Chrome",
                &held,
                &[],
                0,
                0,
                1400,
            ),
            Some(ModifierLayerTransition::Activated {
                target_profile: 4,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
            })
        );

        // D. While still holding RCTRL, user focuses Konsole -> falls back to Profile #2 (All Apps)!
        assert_eq!(
            engine.evaluate_tick_for_window(
                true,
                false,
                &rules,
                "org.kde.konsole",
                "millinr@eos-russdesk:~",
                &held,
                &[],
                0,
                0,
                1500,
            ),
            Some(ModifierLayerTransition::Activated {
                target_profile: 2,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
            })
        );

        // Release RCTRL -> deactivates
        held.clear();
        assert_eq!(
            engine.evaluate_tick_for_window(
                true,
                false,
                &rules,
                "org.kde.konsole",
                "millinr@eos-russdesk:~",
                &held,
                &[],
                0,
                0,
                1600,
            ),
            Some(ModifierLayerTransition::Deactivated)
        );
    }

    #[test]
    fn test_rctrl_rshift_and_menu_rshift_modifier_chords() {
        assert_eq!(
            normalize_modifier_trigger_key("rightctrl-rightshift"),
            "RCTRL_RSHIFT"
        );
        assert_eq!(normalize_modifier_trigger_key("menu-rightshift"), "MENU_RSHIFT");
        assert_eq!(
            modifier_trigger_key_label("RCTRL_RSHIFT"),
            "Right Ctrl + Right Shift Chord"
        );
        assert_eq!(
            modifier_trigger_key_label("MENU_RSHIFT"),
            "Menu + Right Shift Chord"
        );

        let mut engine = HostKeyboardModifierEngine::new();
        let rules = vec![
            ModifierLayerRule {
                enabled: true,
                trigger_key: "RCTRL".to_string(),
                mode: "hold".to_string(),
                hold_delay_ms: 200,
                oneshot_timeout_sec: 6,
                target_profile: 2,
                resource_class: String::new(),
                title_contains: None,
                title_is_regex: false,
            },
            ModifierLayerRule {
                enabled: true,
                trigger_key: "RCTRL_RSHIFT".to_string(),
                mode: "hold".to_string(),
                hold_delay_ms: 200,
                oneshot_timeout_sec: 6,
                target_profile: 10,
                resource_class: String::new(),
                title_contains: None,
                title_is_regex: false,
            },
            ModifierLayerRule {
                enabled: true,
                trigger_key: "MENU_RSHIFT".to_string(),
                mode: "hold".to_string(),
                hold_delay_ms: 200,
                oneshot_timeout_sec: 6,
                target_profile: 11,
                resource_class: String::new(),
                title_contains: None,
                title_is_regex: false,
            },
        ];

        // 1. Press Right Ctrl (97) at t=1000, then Right Shift (54) at t=1030
        let mut held = HashSet::new();
        held.insert(97u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[97], 0, 0, 1000),
            None
        );
        held.insert(54u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[54], 0, 0, 1030),
            None
        );
        // At t=1240 (>= 200ms after chord completed), RCTRL_RSHIFT activates Profile #10 (and RCTRL alone did not!)
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1240),
            Some(ModifierLayerTransition::Activated {
                target_profile: 10,
                trigger_key: "RCTRL_RSHIFT".to_string(),
                mode: "hold".to_string(),
            })
        );
        // Release Right Shift (54) at t=1500 while Right Ctrl (97) is still briefly held -> deactivates cleanly
        held.remove(&54u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1500),
            Some(ModifierLayerTransition::Deactivated)
        );
        held.clear();
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1530),
            None
        );

        // 2. Press Menu (127) + Right Shift (54) at t=2000 -> activates Profile #11 after 200ms
        held.insert(127u16);
        held.insert(54u16);
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[127, 54], 0, 0, 2000),
            None
        );
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 2210),
            Some(ModifierLayerTransition::Activated {
                target_profile: 11,
                trigger_key: "MENU_RSHIFT".to_string(),
                mode: "hold".to_string(),
            })
        );
        held.clear();
        assert_eq!(
            engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 2500),
            Some(ModifierLayerTransition::Deactivated)
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_hyprland_omarchy_active_window_and_cursor_hit_parsing() {
        let active_json = r#"{
            "address": "0x55d8f0a0",
            "mapped": true,
            "hidden": false,
            "at": [20, 48],
            "size": [1880, 1012],
            "workspace": {"id": 1, "name": "1"},
            "floating": false,
            "class": "Alacritty",
            "title": "nvim ~/Builds/duckydeck-studio",
            "initialClass": "Alacritty",
            "initialTitle": "Alacritty",
            "pid": 14322
        }"#;
        assert_eq!(
            linux_desktop_window::parse_hyprland_active_window_json(active_json),
            Some((
                "Alacritty".to_string(),
                "Alacritty".to_string(),
                "nvim ~/Builds/duckydeck-studio".to_string()
            ))
        );

        let clients_json = r#"[
            {
                "address": "0x1",
                "mapped": true,
                "hidden": false,
                "at": [0, 0],
                "size": [960, 1080],
                "workspace": {"id": 1, "name": "1"},
                "floating": false,
                "focusHistoryID": 1,
                "class": "firefox",
                "initialClass": "Navigator",
                "title": "GitHub — Mozilla Firefox"
            },
            {
                "address": "0x2",
                "mapped": true,
                "hidden": false,
                "at": [200, 200],
                "size": [500, 400],
                "workspace": {"id": 1, "name": "1"},
                "floating": true,
                "focusHistoryID": 2,
                "class": "org.gnome.Calculator",
                "initialClass": "gnome-calculator",
                "title": "Calculator"
            }
        ]"#;
        // Cursor at (300, 300) overlaps both tiled Firefox and floating Calculator; floating wins:
        assert_eq!(
            linux_desktop_window::find_hyprland_client_at_cursor_json(
                clients_json,
                Some(1),
                300,
                300
            ),
            Some((
                "org.gnome.Calculator".to_string(),
                "gnome-calculator".to_string(),
                "Calculator".to_string()
            ))
        );
        // Cursor at (100, 100) hits only tiled Firefox:
        assert_eq!(
            linux_desktop_window::find_hyprland_client_at_cursor_json(
                clients_json,
                Some(1),
                100,
                100
            ),
            Some((
                "firefox".to_string(),
                "Navigator".to_string(),
                "GitHub — Mozilla Firefox".to_string()
            ))
        );
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_x11_wm_class_and_wayland_state_parsing() {
        let wm_class_raw = b"Navigator\0firefox\0";
        assert_eq!(
            linux_desktop_window::parse_wm_class_bytes(wm_class_raw),
            Some(("firefox".to_string(), "Navigator".to_string()))
        );

        let mut state_bytes = Vec::new();
        state_bytes.extend_from_slice(&0u32.to_ne_bytes()); // STATE_MAXIMIZED
        assert!(!linux_desktop_window::parse_wayland_state_has_activated(&state_bytes));
        state_bytes.extend_from_slice(&2u32.to_ne_bytes()); // STATE_ACTIVATED
        assert!(linux_desktop_window::parse_wayland_state_has_activated(&state_bytes));
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_e2e_cosmic_and_zwlr_wayland_socket_protocol() {
        use std::io::{Read, Write};
        use std::os::unix::net::UnixStream;

        fn wl_str(s: &str) -> Vec<u8> {
            let b = s.as_bytes();
            let len = (b.len() + 1) as u32;
            let mut v = Vec::new();
            v.extend_from_slice(&len.to_ne_bytes());
            v.extend_from_slice(b);
            v.push(0);
            while v.len() % 4 != 0 {
                v.push(0);
            }
            v
        }

        fn wl_msg(sender_id: u32, opcode: u16, payload: &[u8]) -> Vec<u8> {
            let size = (8 + payload.len()) as u16;
            let size_op = ((size as u32) << 16) | (opcode as u32);
            let mut v = Vec::with_capacity(8 + payload.len());
            v.extend_from_slice(&sender_id.to_ne_bytes());
            v.extend_from_slice(&size_op.to_ne_bytes());
            v.extend_from_slice(payload);
            v
        }

        let (mut server, client) = UnixStream::pair().expect("UnixStream::pair");
        server
            .set_read_timeout(Some(std::time::Duration::from_millis(500)))
            .unwrap();

        let client_thread = std::thread::spawn(move || {
            linux_desktop_window::run_wayland_toplevel_client_on_stream(client);
        });

        // 1. Read client's `wl_display.get_registry(2)` request (object 1, opcode 1, new_id = 2)
        let mut req = [0u8; 12];
        server.read_exact(&mut req).expect("read get_registry");
        assert_eq!(u32::from_ne_bytes([req[0], req[1], req[2], req[3]]), 1);
        assert_eq!(u32::from_ne_bytes([req[8], req[9], req[10], req[11]]), 2);

        // 2. Server advertises both `zcosmic_toplevel_info_v1` (global name 10) and `zwlr_foreign_toplevel_manager_v1` (global name 20)
        let mut g1 = Vec::new();
        g1.extend_from_slice(&10u32.to_ne_bytes());
        g1.extend_from_slice(&wl_str("zcosmic_toplevel_info_v1"));
        g1.extend_from_slice(&2u32.to_ne_bytes());
        server.write_all(&wl_msg(2, 0, &g1)).unwrap();

        let mut g2 = Vec::new();
        g2.extend_from_slice(&20u32.to_ne_bytes());
        g2.extend_from_slice(&wl_str("zwlr_foreign_toplevel_manager_v1"));
        g2.extend_from_slice(&3u32.to_ne_bytes());
        server.write_all(&wl_msg(2, 0, &g2)).unwrap();

        // Read the two `wl_registry.bind` requests from client (bound object IDs 3 and 4)
        let mut hdr = [0u8; 8];
        for expected_id in [3u32, 4u32] {
            server.read_exact(&mut hdr).expect("read bind hdr");
            let sz = (u32::from_ne_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]) >> 16) as usize;
            let mut body = vec![0u8; sz - 8];
            server.read_exact(&mut body).expect("read bind body");
            let bound_id = u32::from_ne_bytes([
                body[body.len() - 4],
                body[body.len() - 3],
                body[body.len() - 2],
                body[body.len() - 1],
            ]);
            assert_eq!(bound_id, expected_id);
        }

        // 3. Emit a COSMIC toplevel on `cosmic_info_id` (3):
        //    - manager opcode 0: `toplevel(new_id = 0xFF000001)`
        //    - handle opcode 2: `title("COSMIC Terminal — ~/Builds")`
        //    - handle opcode 3: `app_id("com.system76.CosmicTerm")`
        //    - handle opcode 8: `state([2u32])` (`STATE_ACTIVATED = 2`)
        //    - handle opcode 1: `done()`
        let cosmic_handle = 0xFF00_0001u32;
        server
            .write_all(&wl_msg(3, 0, &cosmic_handle.to_ne_bytes()))
            .unwrap();
        server
            .write_all(&wl_msg(
                cosmic_handle,
                2,
                &wl_str("COSMIC Terminal — ~/Builds"),
            ))
            .unwrap();
        server
            .write_all(&wl_msg(
                cosmic_handle,
                3,
                &wl_str("com.system76.CosmicTerm"),
            ))
            .unwrap();
        let mut state_arr = Vec::new();
        state_arr.extend_from_slice(&4u32.to_ne_bytes());
        state_arr.extend_from_slice(&2u32.to_ne_bytes());
        server
            .write_all(&wl_msg(cosmic_handle, 8, &state_arr))
            .unwrap();
        server.write_all(&wl_msg(cosmic_handle, 1, &[])).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(30));
        let active_cosmic = linux_desktop_window::latest_wayland_active_window()
            .expect("COSMIC active window should be recorded");
        assert_eq!(active_cosmic.0, "com.system76.CosmicTerm");
        assert_eq!(active_cosmic.2, "COSMIC Terminal — ~/Builds");

        // 4. Emit a `zwlr_foreign_toplevel_manager_v1` toplevel on `zwlr_mgr_id` (4):
        //    - manager opcode 0: `toplevel(new_id = 0xFF000002)`
        //    - handle opcode 0: `title("GitHub — Mozilla Firefox")`
        //    - handle opcode 1: `app_id("firefox")`
        //    - handle opcode 4: `state([2u32])` (`STATE_ACTIVATED = 2`)
        //    - handle opcode 5: `done()`
        let zwlr_handle = 0xFF00_0002u32;
        server
            .write_all(&wl_msg(4, 0, &zwlr_handle.to_ne_bytes()))
            .unwrap();
        server
            .write_all(&wl_msg(
                zwlr_handle,
                0,
                &wl_str("GitHub — Mozilla Firefox"),
            ))
            .unwrap();
        server
            .write_all(&wl_msg(zwlr_handle, 1, &wl_str("firefox")))
            .unwrap();
        server
            .write_all(&wl_msg(zwlr_handle, 4, &state_arr))
            .unwrap();
        server.write_all(&wl_msg(zwlr_handle, 5, &[])).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(30));
        let active_zwlr = linux_desktop_window::latest_wayland_active_window()
            .expect("zwlr active window should be recorded");
        assert_eq!(active_zwlr.0, "firefox");
        assert_eq!(active_zwlr.2, "GitHub — Mozilla Firefox");
        assert!(active_zwlr.3 > active_cosmic.3);

        drop(server);
        let _ = client_thread.join();
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_e2e_omarchy_hyprland_unix_socket_ipc_and_hover_hit_test() {
        use std::io::{Read, Write};
        use std::os::unix::net::UnixListener;

        let tmp_dir = std::env::temp_dir().join(format!(
            "duckydeck-hypr-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&tmp_dir);
        std::fs::create_dir_all(&tmp_dir).unwrap();
        let sock_path = tmp_dir.join(".socket.sock");
        let listener = UnixListener::bind(&sock_path).expect("bind mock Hyprland socket");

        let server_thread = std::thread::spawn(move || {
            for _ in 0..4 {
                if let Ok((mut stream, _)) = listener.accept() {
                    let mut buf = [0u8; 128];
                    let n = stream.read(&mut buf).unwrap_or(0);
                    let cmd = String::from_utf8_lossy(&buf[..n]);
                    let resp = match cmd.as_ref() {
                        "j/activewindow" => {
                            r#"{"class":"ghostty","initialClass":"com.mitchellh.ghostty","title":"omarchy — zsh"}"#
                        }
                        "j/cursorpos" => r#"{"x":1400,"y":500}"#,
                        "j/activeworkspace" => r#"{"id":2,"name":"2"}"#,
                        "j/clients" => {
                            r#"[
                                {"mapped":true,"hidden":false,"at":[0,0],"size":[960,1080],"workspace":{"id":2},"floating":false,"focusHistoryID":0,"class":"ghostty","initialClass":"com.mitchellh.ghostty","title":"omarchy — zsh"},
                                {"mapped":true,"hidden":false,"at":[960,0],"size":[960,1080],"workspace":{"id":2},"floating":false,"focusHistoryID":1,"class":"chromium","initialClass":"chromium","title":"Omarchy Manual — Chromium"}
                            ]"#
                        }
                        _ => "ok",
                    };
                    let _ = stream.write_all(resp.as_bytes());
                }
            }
        });

        let mut last_cursor = None;
        let (active_win, hover_win) =
            linux_desktop_window::query_hyprland_window_and_hover_from_socket(
                &sock_path,
                &mut last_cursor,
            );

        assert_eq!(
            active_win,
            Some((
                "ghostty".to_string(),
                "com.mitchellh.ghostty".to_string(),
                "omarchy — zsh".to_string()
            ))
        );
        assert_eq!(
            hover_win,
            Some((
                "chromium".to_string(),
                "chromium".to_string(),
                "Omarchy Manual — Chromium".to_string()
            ))
        );
        assert_eq!(last_cursor, Some((1400, 500)));

        let _ = server_thread.join();
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_e2e_cinnamon_x11_live_display_window_inspection() {
        use std::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

        if std::env::var_os("DISPLAY").is_none() {
            return;
        }
        unsafe extern "C" {
            fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
            fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
        }
        unsafe {
            let handle = dlopen(c"libX11.so.6".as_ptr(), 1);
            if handle.is_null() {
                return;
            }
            let sym_open = dlsym(handle, c"XOpenDisplay".as_ptr());
            let sym_root = dlsym(handle, c"XDefaultRootWindow".as_ptr());
            let sym_create = dlsym(handle, c"XCreateSimpleWindow".as_ptr());
            let sym_intern = dlsym(handle, c"XInternAtom".as_ptr());
            let sym_change = dlsym(handle, c"XChangeProperty".as_ptr());
            let sym_flush = dlsym(handle, c"XFlush".as_ptr());
            let sym_destroy = dlsym(handle, c"XDestroyWindow".as_ptr());
            let sym_close = dlsym(handle, c"XCloseDisplay".as_ptr());
            if sym_open.is_null()
                || sym_root.is_null()
                || sym_create.is_null()
                || sym_intern.is_null()
                || sym_change.is_null()
                || sym_flush.is_null()
                || sym_destroy.is_null()
                || sym_close.is_null()
            {
                return;
            }

            type OpenFn = unsafe extern "C" fn(*const c_char) -> *mut c_void;
            type RootFn = unsafe extern "C" fn(*mut c_void) -> c_ulong;
            type CreateFn = unsafe extern "C" fn(
                *mut c_void,
                c_ulong,
                c_int,
                c_int,
                c_uint,
                c_uint,
                c_uint,
                c_ulong,
                c_ulong,
            ) -> c_ulong;
            type InternFn = unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_ulong;
            type ChangeFn = unsafe extern "C" fn(
                *mut c_void,
                c_ulong,
                c_ulong,
                c_ulong,
                c_int,
                c_int,
                *const c_uchar,
                c_int,
            ) -> c_int;
            type FlushFn = unsafe extern "C" fn(*mut c_void) -> c_int;
            type DestroyFn = unsafe extern "C" fn(*mut c_void, c_ulong) -> c_int;
            type CloseFn = unsafe extern "C" fn(*mut c_void) -> c_int;

            let x_open: OpenFn = std::mem::transmute(sym_open);
            let x_root: RootFn = std::mem::transmute(sym_root);
            let x_create: CreateFn = std::mem::transmute(sym_create);
            let x_intern: InternFn = std::mem::transmute(sym_intern);
            let x_change: ChangeFn = std::mem::transmute(sym_change);
            let x_flush: FlushFn = std::mem::transmute(sym_flush);
            let x_destroy: DestroyFn = std::mem::transmute(sym_destroy);
            let x_close: CloseFn = std::mem::transmute(sym_close);

            let dpy = x_open(std::ptr::null());
            if dpy.is_null() {
                return;
            }
            let root = x_root(dpy);
            let win = x_create(dpy, root, 0, 0, 100, 100, 0, 0, 0);
            let atom_wm_class = x_intern(dpy, c"WM_CLASS".as_ptr(), 0);
            let atom_string = x_intern(dpy, c"STRING".as_ptr(), 0);
            let atom_net_wm_name = x_intern(dpy, c"_NET_WM_NAME".as_ptr(), 0);
            let atom_utf8 = x_intern(dpy, c"UTF8_STRING".as_ptr(), 0);

            let wm_class_bytes = b"nemo\0Nemo\0";
            x_change(
                dpy,
                win,
                atom_wm_class,
                atom_string,
                8,
                0,
                wm_class_bytes.as_ptr(),
                wm_class_bytes.len() as c_int,
            );
            let title_bytes = b"Home \xE2\x80\x94 Nemo File Manager";
            x_change(
                dpy,
                win,
                atom_net_wm_name,
                atom_utf8,
                8,
                0,
                title_bytes.as_ptr(),
                title_bytes.len() as c_int,
            );
            x_flush(dpy);

            let inspected = linux_desktop_window::inspect_x11_window_id(win);
            x_destroy(dpy, win);
            x_flush(dpy);
            x_close(dpy);

            assert_eq!(
                inspected,
                Some((
                    "Nemo".to_string(),
                    "nemo".to_string(),
                    "Home — Nemo File Manager".to_string()
                ))
            );
        }
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_e2e_gnome_bridge_notification_sequence_tracking() {
        let before_seq = latest_bridge_window().map(|w| w.3).unwrap_or(0);
        record_last_bridge_window(
            "org.gnome.Nautilus",
            "org.gnome.Nautilus",
            "Downloads — Files",
        );
        let after = latest_bridge_window().expect("bridge window should be recorded");
        assert_eq!(after.0, "org.gnome.Nautilus");
        assert_eq!(after.1, "org.gnome.Nautilus");
        assert_eq!(after.2, "Downloads — Files");
        assert!(after.3 > before_seq);
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_kwin6_hook_js_omits_cursor_pos_changed_by_default() {
        let js_default = generate_kwin6_hook_js(false, "null", "null");
        assert!(!js_default.contains("cursorPosChanged"));
        assert!(!js_default.contains("onCursorPosChanged"));
        assert!(!js_default.contains("topWindowAtPoint"));
        assert!(js_default.contains("workspace.windowActivated.connect(onWindowActivated)"));
        assert!(js_default.contains("workspace.activeWindow && !isDuckyDeckWindow(workspace.activeWindow)"));

        let js_hover = generate_kwin6_hook_js(true, "null", "null");
        assert!(js_hover.contains("workspace.cursorPosChanged.connect(onCursorPosChanged)"));
        assert!(js_hover.contains("function onCursorPosChanged()"));
        assert!(js_hover.contains("topWindowAtPoint(workspace.cursorPos)"));
    }

    #[test]
    fn test_persisted_switcher_config_unfocused_hover_defaults_false() {
        let json_without_field = r#"{
            "enabled": true,
            "default_profile": 1,
            "rules": []
        }"#;
        let parsed: PersistedSwitcherConfig =
            serde_json::from_str(json_without_field).expect("deserialize legacy config");
        assert!(!parsed.auto_switch_unfocused_hover);
    }
}
