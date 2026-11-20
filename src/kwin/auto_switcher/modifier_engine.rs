use serde::{Deserialize, Serialize};

use super::{
    is_any_browser_wildcard, resolve_full_browser_caption, rule_matches_window_class,
    rule_matches_window_title,
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
        "MENU_RSHIFT" | "MENU+RSHIFT" | "MENU-RSHIFT" | "MENU_RIGHTSHIFT" | "MENU+RIGHTSHIFT"
        | "MENU-RIGHTSHIFT" | "MENU_RIGHT_SHIFT" | "MENU+RIGHT_SHIFT" | "MENU-RIGHT_SHIFT" => {
            "MENU_RSHIFT".to_string()
        }
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
        let matched_len =
            rule_matches_window_title(sub, rule.title_is_regex, &resolved_caption, &caption_lower)?;
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
