use crate::bridge::device_controller::{qobject, record_system_event};
use crate::kwin::auto_switcher::{
    is_web_browser_class, modifier_mode_label, modifier_trigger_key_label, normalize_modifier_mode,
    normalize_modifier_trigger_key, send_hid_switch_profile_checked, strip_browser_title_suffix,
    AutoSwitchRule, HidSwitchOutcome, ModifierLayerRule, ModifierLayerTransition, SwitcherState,
};
use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;

impl qobject::DeviceController {
    pub(crate) fn handle_modifier_layer_transition(
        mut self: Pin<&mut Self>,
        transition: ModifierLayerTransition,
    ) {
        match transition {
            ModifierLayerTransition::Activated {
                target_profile,
                trigger_key,
                mode,
            } => {
                let mut prev_prof = (*self.active_profile()).clamp(1, 64) as u8;
                let max_prof = self.rust().profiles.len().max(1) as u8;
                let clamped_target = target_profile.clamp(1, max_prof);
                let prof_name = self
                    .rust()
                    .profiles
                    .iter()
                    .find(|p| p.number == clamped_target)
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| format!("Profile {clamped_target}"));

                let key_lbl = modifier_trigger_key_label(&trigger_key);
                let mode_lbl = modifier_mode_label(&mode);

                let mut hid_rejected = false;
                {
                    let rust = self.as_mut().rust_mut().get_mut();
                    if let Ok(mut sw) = rust.switcher_state.lock() {
                        let was_already_overridden = sw.active_modifier_override.is_some();
                        crate::kwin::auto_switcher::LAST_OBSERVED_HW_PROFILE_BEFORE_SWITCH
                            .store(0, std::sync::atomic::Ordering::SeqCst);
                        sw.active_modifier_override =
                            Some((clamped_target, trigger_key.clone(), mode.clone()));
                        match send_hid_switch_profile_checked(clamped_target) {
                            HidSwitchOutcome::Switched => {
                                let hw_pre = crate::kwin::auto_switcher::LAST_OBSERVED_HW_PROFILE_BEFORE_SWITCH
                                    .load(std::sync::atomic::Ordering::SeqCst);
                                if !was_already_overridden {
                                    if hw_pre >= 1 {
                                        prev_prof = hw_pre.clamp(1, max_prof);
                                    } else if sw.last_switched_profile >= 1 {
                                        prev_prof = sw.last_switched_profile.clamp(1, max_prof);
                                    }
                                    sw.last_window_matched_profile = Some(prev_prof);
                                }
                                sw.last_switched_profile = clamped_target;
                            }
                            HidSwitchOutcome::NotLoadedOnHardware => {
                                if !was_already_overridden
                                    && sw.last_window_matched_profile.is_none()
                                {
                                    sw.last_window_matched_profile = Some(prev_prof);
                                }
                                hid_rejected = true;
                            }
                            HidSwitchOutcome::DeviceUnavailable => {
                                if !was_already_overridden
                                    && sw.last_window_matched_profile.is_none()
                                {
                                    sw.last_window_matched_profile = Some(prev_prof);
                                }
                            }
                        }
                    }
                }

                let badge_text =
                    format!("⌨️ {key_lbl} [{mode_lbl}] → #{clamped_target} {prof_name}");
                self.as_mut().set_modifier_layer_active(true);
                self.as_mut()
                    .set_modifier_layer_status(QString::from(&badge_text));

                if prev_prof != clamped_target {
                    self.as_mut().set_active_profile(clamped_target as i32);
                    self.as_mut().sync_active_profile_selection_only();
                    self.as_mut().dispatch_profile_change_webhooks_if_changed(
                        prev_prof,
                        clamped_target,
                        "modifier_layer_activate",
                    );
                } else {
                    self.as_mut().sync_tray_state();
                }

                let status_msg = if hid_rejected {
                    format!(
                        "⚠️ Modifier Layer ({key_lbl}): Profile #{clamped_target} ('{prof_name}') not yet synced to hardware"
                    )
                } else {
                    format!(
                        "⌨️ Modifier Layer Active ({key_lbl} • {mode_lbl}) → Profile #{clamped_target} ({prof_name})"
                    )
                };
                record_system_event("INFO", "AUTOSWITCH", &status_msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&status_msg));
            }
            ModifierLayerTransition::Deactivated => {
                let prev_prof = (*self.active_profile()).clamp(1, 64) as u8;
                let max_prof = self.rust().profiles.len().max(1) as u8;
                let active_class = self.active_window_class().to_string();
                let active_title = self.active_window_title().to_string();

                let mut restore_prof: u8 = prev_prof;
                {
                    let rust = self.as_mut().rust_mut().get_mut();
                    if let Ok(mut sw) = rust.switcher_state.lock() {
                        sw.active_modifier_override = None;
                        if let Some(win_prof) = sw.last_window_matched_profile {
                            restore_prof = win_prof.clamp(1, max_prof);
                        } else if !active_class.is_empty()
                            && active_class != "Listening for KWin..."
                        {
                            if let Some(matched) =
                                sw.match_profile_for_window(&active_class, &active_title)
                            {
                                restore_prof = matched.clamp(1, max_prof);
                            } else if let Some(fb) = sw.default_profile {
                                restore_prof = fb.clamp(1, max_prof);
                            }
                        } else if let Some(fb) = sw.default_profile {
                            restore_prof = fb.clamp(1, max_prof);
                        }

                        if let HidSwitchOutcome::Switched =
                            send_hid_switch_profile_checked(restore_prof)
                        {
                            sw.last_switched_profile = restore_prof;
                        }
                    }
                }

                self.as_mut().set_modifier_layer_active(false);
                self.as_mut().set_modifier_layer_status(QString::from(""));

                let prof_name = self
                    .rust()
                    .profiles
                    .iter()
                    .find(|p| p.number == restore_prof)
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| format!("Profile {restore_prof}"));

                if prev_prof != restore_prof {
                    self.as_mut().set_active_profile(restore_prof as i32);
                    self.as_mut().sync_active_profile_selection_only();
                    self.as_mut().dispatch_profile_change_webhooks_if_changed(
                        prev_prof,
                        restore_prof,
                        "modifier_layer_restore",
                    );
                } else {
                    self.as_mut().sync_tray_state();
                }

                let msg = format!(
                    "⌨️ Modifier Layer Released → Restored Profile #{restore_prof} ({prof_name})"
                );
                record_system_event("INFO", "AUTOSWITCH", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
            }
        }
    }

    pub fn set_script_editor_open(mut self: Pin<&mut Self>, open: bool) {
        let was_paused = {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let prev = sw.paused_for_editing;
                sw.paused_for_editing = open;
                prev
            } else {
                false
            }
        };
        if was_paused && !open {
            self.as_mut().apply_auto_switch_for_current_window();
        }
        self.as_mut().sync_tray_state();
    }

    pub fn set_auto_switch_config(mut self: Pin<&mut Self>, enabled: bool, default_profile: i32) {
        let cur_serial = self.serial_number().to_string();
        let effective_fb = {
            let rust = self.as_mut().rust_mut().get_mut();
            let max_prof = rust.profiles.len().max(1) as u8;
            let mut chosen_fb = 1u8;
            if let Ok(mut sw) = rust.switcher_state.lock() {
                chosen_fb = if (1..=64).contains(&default_profile) {
                    (default_profile as u8).clamp(1, max_prof)
                } else {
                    sw.default_profile.unwrap_or(1).clamp(1, max_prof)
                };
                sw.enabled = enabled;
                sw.default_profile = Some(chosen_fb);
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving auto-switch config: {e}"),
                    );
                }
                crate::kwin::auto_switcher::save_device_rules_for_serial(
                    &cur_serial,
                    sw.enabled,
                    sw.default_profile,
                    &sw.rules,
                    &rust.profiles,
                    rust.sd_root_path.as_deref(),
                );
            }
            chosen_fb as i32
        };

        self.as_mut().set_auto_switch_enabled(enabled);
        self.as_mut().set_default_fallback_profile(effective_fb);
        self.as_mut().apply_auto_switch_for_current_window();
        self.as_mut().sync_active_profile_state();
    }

    pub fn set_auto_switch_timings(mut self: Pin<&mut Self>, settle_ms: i32, dwell_ms: i32) {
        let clamped_settle = settle_ms.clamp(0, 5000);
        let clamped_dwell = dwell_ms.clamp(0, 5000);

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.mouse_settle_ms = clamped_settle;
                sw.window_dwell_ms = clamped_dwell;
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving auto-switch timings: {e}"),
                    );
                }
            }
        }

        self.as_mut().set_auto_switch_settle_ms(clamped_settle);
        self.as_mut().set_auto_switch_dwell_ms(clamped_dwell);
        self.as_mut().set_status_message(QString::from(&format!(
            "Auto-Switcher timings set: mouse settle = {} ms, window dwell = {} ms",
            clamped_settle, clamped_dwell
        )));
    }

    pub fn update_auto_switch_unfocused_hover(mut self: Pin<&mut Self>, enabled: bool) {
        let mut changed = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if sw.auto_switch_unfocused_hover != enabled {
                    sw.auto_switch_unfocused_hover = enabled;
                    changed = true;
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving unfocused hover setting: {e}"),
                        );
                    }
                }
            }
        }

        self.as_mut().set_auto_switch_unfocused_hover(enabled);
        if changed {
            crate::kwin::auto_switcher::request_kwin6_hook_reload();
            let msg = if enabled {
                "Auto-Switcher unfocused window hover polling enabled (Click-to-Focus hover mode)"
            } else {
                "Auto-Switcher unfocused window hover polling disabled (focus-only, 0 Hz cursor overhead)"
            };
            record_system_event("INFO", "AUTOSWITCH", msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(msg));
            self.as_mut().apply_auto_switch_for_current_window();
        }
    }

    pub fn set_auto_switch_sections_expanded(
        mut self: Pin<&mut Self>,
        enable_expanded: bool,
        rule_editor_expanded: bool,
    ) {
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if sw.autoswitch_enable_section_expanded != enable_expanded
                    || sw.autoswitch_rule_editor_expanded != rule_editor_expanded
                {
                    sw.autoswitch_enable_section_expanded = enable_expanded;
                    sw.autoswitch_rule_editor_expanded = rule_editor_expanded;
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving auto-switch section states: {e}"),
                        );
                    }
                }
            }
        }
        self.as_mut()
            .set_autoswitch_enable_section_expanded(enable_expanded);
        self.as_mut()
            .set_autoswitch_rule_editor_expanded(rule_editor_expanded);
    }

    pub fn reload_auto_switch_sections_from_disk(mut self: Pin<&mut Self>) {
        let (enable_exp, rule_exp, unfocused_hover) = SwitcherState::load_config_from_disk()
            .map(|c| {
                (
                    c.autoswitch_enable_section_expanded,
                    c.autoswitch_rule_editor_expanded,
                    c.auto_switch_unfocused_hover,
                )
            })
            .unwrap_or((true, true, false));
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.autoswitch_enable_section_expanded = enable_exp;
                sw.autoswitch_rule_editor_expanded = rule_exp;
                sw.auto_switch_unfocused_hover = unfocused_hover;
            }
        }
        self.as_mut()
            .set_autoswitch_enable_section_expanded(enable_exp);
        self.as_mut().set_autoswitch_rule_editor_expanded(rule_exp);
        self.as_mut()
            .set_auto_switch_unfocused_hover(unfocused_hover);
    }

    pub fn arm_window_picker(mut self: Pin<&mut Self>, bind_to_active_profile: bool) {
        let switcher_state = {
            let rust = self.as_mut().rust_mut().get_mut();
            rust.picker_bind_to_active_profile = bind_to_active_profile;
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.picker_armed = true;
            }
            rust.switcher_state.clone()
        };
        self.as_mut()
            .set_last_picked_window_class(QString::from(""));
        self.as_mut()
            .set_last_picked_window_title(QString::from(""));
        self.as_mut().set_window_picker_armed(true);
        self.as_mut().set_status_message(QString::from(
            "🎯 Window Picker Armed — Click any open window with the crosshair cursor",
        ));

        let qt_thread = self.qt_thread();
        crate::actions::web_api::spawn_shared_async(async move {
            let result = crate::kwin::auto_switcher::pick_window_via_kwin_dbus().await;
            let _ = qt_thread.queue(move |mut qobject| {
                if let Ok(mut sw) = switcher_state.lock() {
                    sw.picker_armed = false;
                }
                qobject.as_mut().set_window_picker_armed(false);

                match result {
                    Ok(evt) => {
                        let picked_title = if is_web_browser_class(&evt.resource_class) {
                            strip_browser_title_suffix(&evt.caption)
                        } else {
                            evt.caption.clone()
                        };
                        qobject
                            .as_mut()
                            .set_active_window_class(QString::from(&evt.resource_class));
                        qobject
                            .as_mut()
                            .set_active_window_title(QString::from(&evt.caption));
                        qobject
                            .as_mut()
                            .set_last_picked_window_class(QString::from(&evt.resource_class));
                        qobject
                            .as_mut()
                            .set_last_picked_window_title(QString::from(&picked_title));

                        let bind_direct = qobject.rust().picker_bind_to_active_profile;
                        if bind_direct {
                            let target_prof = *qobject.active_profile();
                            qobject.as_mut().add_or_update_auto_switch_rule(
                                -1,
                                true,
                                QString::from(&evt.resource_class),
                                QString::from(""),
                                target_prof,
                            );
                        } else {
                            qobject.as_mut().set_status_message(QString::from(&format!(
                                "Picked window: '{}' ({})",
                                evt.resource_class, picked_title
                            )));
                        }
                    }
                    Err(err) => {
                        qobject.as_mut().set_status_message(QString::from(&err));
                    }
                }
            });
        });
    }

    pub fn add_or_update_auto_switch_rule(
        self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
        resource_class: QString,
        title_contains: QString,
        target_profile: i32,
    ) -> bool {
        self.save_auto_switch_rule_with_favicon(
            rule_index,
            enabled,
            resource_class,
            title_contains,
            QString::default(),
            target_profile,
        )
    }

    pub fn save_auto_switch_rule_with_favicon(
        self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
        resource_class: QString,
        title_contains: QString,
        favicon_url: QString,
        target_profile: i32,
    ) -> bool {
        self.save_auto_switch_rule_full(
            rule_index,
            enabled,
            resource_class,
            title_contains,
            false,
            favicon_url,
            target_profile,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save_auto_switch_rule_full(
        mut self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
        resource_class: QString,
        title_contains: QString,
        title_is_regex: bool,
        favicon_url: QString,
        target_profile: i32,
    ) -> bool {
        let class_str = resource_class.to_string().trim().to_string();
        if class_str.is_empty() {
            return false;
        }
        let title_str = title_contains.to_string().trim().to_string();
        let title_opt = if title_str.is_empty() {
            None
        } else {
            Some(title_str)
        };
        let is_regex = if title_opt.is_none() {
            false
        } else {
            title_is_regex
        };
        let fav_str = favicon_url.to_string().trim().to_string();
        let fav_opt = if fav_str.is_empty() {
            None
        } else {
            Some(fav_str)
        };
        let target = target_profile.clamp(1, 64) as u8;

        let new_rule = AutoSwitchRule {
            enabled,
            resource_class: class_str.clone(),
            title_contains: title_opt,
            title_is_regex: is_regex,
            favicon_url: fav_opt,
            target_profile: target,
        };

        let cur_serial = self.serial_number().to_string();
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if rule_index >= 0 && (rule_index as usize) < sw.rules.len() {
                    sw.rules[rule_index as usize] = new_rule;
                } else if let Some(existing) = sw.rules.iter_mut().find(|r| {
                    r.resource_class.eq_ignore_ascii_case(&class_str)
                        && r.title_contains == new_rule.title_contains
                }) {
                    *existing = new_rule;
                } else {
                    sw.rules.push(new_rule);
                }
                sw.loaded_from_disk = true;
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving auto-switch rule: {e}"),
                    );
                }
                crate::kwin::auto_switcher::save_device_rules_for_serial(
                    &cur_serial,
                    sw.enabled,
                    sw.default_profile,
                    &sw.rules,
                    &rust.profiles,
                    rust.sd_root_path.as_deref(),
                );
            }
        }

        self.as_mut().apply_auto_switch_for_current_window();
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "Saved Auto-Switcher rule: '{class_str}' → Profile #{target}"
        )));
        true
    }

    pub fn validate_title_regex_pattern(&self, pattern: QString) -> QString {
        let raw = pattern.to_string();
        match crate::kwin::auto_switcher::validate_title_regex(&raw) {
            Ok(()) => QString::default(),
            Err(err) => QString::from(&err),
        }
    }

    pub fn remove_auto_switch_rule(mut self: Pin<&mut Self>, rule_index: i32) -> bool {
        let cur_serial = self.serial_number().to_string();
        let mut removed = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let idx = rule_index as usize;
                if rule_index >= 0 && idx < sw.rules.len() {
                    sw.rules.remove(idx);
                    sw.loaded_from_disk = true;
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving after removing rule: {e}"),
                        );
                    }
                    crate::kwin::auto_switcher::save_device_rules_for_serial(
                        &cur_serial,
                        sw.enabled,
                        sw.default_profile,
                        &sw.rules,
                        &rust.profiles,
                        rust.sd_root_path.as_deref(),
                    );
                    removed = true;
                }
            }
        }

        if removed {
            self.as_mut().apply_auto_switch_for_current_window();
            self.as_mut().sync_active_profile_state();
            self.as_mut()
                .set_status_message(QString::from("Removed Auto-Switcher rule"));
        }
        removed
    }

    pub fn toggle_auto_switch_rule(
        mut self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
    ) -> bool {
        let cur_serial = self.serial_number().to_string();
        let mut updated = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let idx = rule_index as usize;
                if rule_index >= 0 && idx < sw.rules.len() {
                    sw.rules[idx].enabled = enabled;
                    sw.loaded_from_disk = true;
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving after toggling rule: {e}"),
                        );
                    }
                    crate::kwin::auto_switcher::save_device_rules_for_serial(
                        &cur_serial,
                        sw.enabled,
                        sw.default_profile,
                        &sw.rules,
                        &rust.profiles,
                        rust.sd_root_path.as_deref(),
                    );
                    updated = true;
                }
            }
        }

        if updated {
            self.as_mut().apply_auto_switch_for_current_window();
            self.as_mut().sync_active_profile_state();
        }
        updated
    }

    pub fn update_modifier_switch_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        let mut had_active_override = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.modifier_switch_enabled = enabled;
                if !enabled && sw.active_modifier_override.is_some() {
                    sw.active_modifier_override = None;
                    had_active_override = true;
                }
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving modifier switch state: {e}"),
                    );
                }
            }
        }
        self.as_mut().set_modifier_switch_enabled(enabled);
        if had_active_override {
            self.as_mut().set_modifier_layer_active(false);
            self.as_mut().set_modifier_layer_status(QString::from(""));
            self.as_mut().apply_auto_switch_for_current_window();
        }
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(if enabled {
            "⌨️ Main-Keyboard Modifier Layer Switcher enabled"
        } else {
            "⌨️ Main-Keyboard Modifier Layer Switcher disabled"
        }));
    }

    pub fn update_modifier_section_expanded(mut self: Pin<&mut Self>, expanded: bool) {
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if sw.modifier_section_expanded != expanded {
                    sw.modifier_section_expanded = expanded;
                    let _ = sw.save_to_disk();
                }
            }
        }
        self.as_mut().set_modifier_section_expanded(expanded);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save_modifier_rule(
        mut self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
        trigger_key: QString,
        mode: QString,
        hold_delay_ms: i32,
        oneshot_timeout_sec: i32,
        target_profile: i32,
        resource_class: QString,
        title_contains: QString,
        title_is_regex: bool,
    ) -> bool {
        let norm_key = normalize_modifier_trigger_key(&trigger_key.to_string());
        let norm_mode = normalize_modifier_mode(&mode.to_string());
        let clamped_delay = hold_delay_ms.clamp(0, 2000);
        let clamped_timeout = oneshot_timeout_sec.clamp(1, 60);
        let max_prof = self.rust().profiles.len().max(1) as i32;
        let target = target_profile.clamp(1, max_prof.max(1)) as u8;

        let class_raw = resource_class.to_string().trim().to_string();
        let norm_class = if class_raw == "*"
            || class_raw.eq_ignore_ascii_case("any")
            || class_raw.eq_ignore_ascii_case("all")
        {
            String::new()
        } else {
            class_raw
        };
        let title_str = title_contains.to_string().trim().to_string();
        let title_opt = if title_str.is_empty() {
            None
        } else {
            Some(title_str)
        };
        let is_regex = if title_opt.is_none() {
            false
        } else {
            title_is_regex
        };

        let new_rule = ModifierLayerRule {
            enabled,
            trigger_key: norm_key.clone(),
            mode: norm_mode.clone(),
            hold_delay_ms: clamped_delay,
            oneshot_timeout_sec: clamped_timeout,
            target_profile: target,
            resource_class: norm_class.clone(),
            title_contains: title_opt,
            title_is_regex: is_regex,
        };

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if rule_index >= 0 && (rule_index as usize) < sw.modifier_rules.len() {
                    sw.modifier_rules[rule_index as usize] = new_rule;
                } else if let Some(existing) = sw.modifier_rules.iter_mut().find(|r| {
                    r.trigger_key.eq_ignore_ascii_case(&norm_key)
                        && r.resource_class.trim().eq_ignore_ascii_case(&norm_class)
                        && r.title_contains == new_rule.title_contains
                }) {
                    *existing = new_rule;
                } else {
                    sw.modifier_rules.push(new_rule);
                }
                sw.loaded_from_disk = true;
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving modifier rule: {e}"),
                    );
                }
            }
        }

        let key_lbl = modifier_trigger_key_label(&norm_key);
        let mode_lbl = modifier_mode_label(&norm_mode);
        let scope_lbl = if norm_class.is_empty() {
            "All Apps".to_string()
        } else {
            norm_class
        };
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "Saved Modifier Layer rule [{scope_lbl}]: {key_lbl} ({mode_lbl}) → Profile #{target}"
        )));
        true
    }

    pub fn remove_modifier_rule(mut self: Pin<&mut Self>, rule_index: i32) -> bool {
        let mut removed = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let idx = rule_index as usize;
                if rule_index >= 0 && idx < sw.modifier_rules.len() {
                    sw.modifier_rules.remove(idx);
                    sw.loaded_from_disk = true;
                    let _ = sw.save_to_disk();
                    removed = true;
                }
            }
        }
        if removed {
            self.as_mut().sync_active_profile_state();
            self.as_mut()
                .set_status_message(QString::from("Removed Modifier Layer rule"));
        }
        removed
    }

    pub fn toggle_modifier_rule(mut self: Pin<&mut Self>, rule_index: i32, enabled: bool) -> bool {
        let mut updated = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let idx = rule_index as usize;
                if rule_index >= 0 && idx < sw.modifier_rules.len() {
                    sw.modifier_rules[idx].enabled = enabled;
                    sw.loaded_from_disk = true;
                    let _ = sw.save_to_disk();
                    updated = true;
                }
            }
        }
        if updated {
            self.as_mut().sync_active_profile_state();
        }
        updated
    }

    pub fn reorder_auto_switch_rule(
        mut self: Pin<&mut Self>,
        from_index: i32,
        to_index: i32,
    ) -> bool {
        if from_index < 0 || to_index < 0 || from_index == to_index {
            return false;
        }
        let cur_serial = self.serial_number().to_string();
        let from_idx = from_index as usize;
        let to_idx = to_index as usize;
        let mut moved_class = String::new();
        let mut reordered = false;

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if from_idx < sw.rules.len() && to_idx < sw.rules.len() {
                    let rule = sw.rules.remove(from_idx);
                    moved_class = rule.resource_class.clone();
                    sw.rules.insert(to_idx, rule);
                    sw.loaded_from_disk = true;
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving after reordering rule: {e}"),
                        );
                    }
                    crate::kwin::auto_switcher::save_device_rules_for_serial(
                        &cur_serial,
                        sw.enabled,
                        sw.default_profile,
                        &sw.rules,
                        &rust.profiles,
                        rust.sd_root_path.as_deref(),
                    );
                    reordered = true;
                }
            }
        }

        if reordered {
            self.as_mut().apply_auto_switch_for_current_window();
            self.as_mut().sync_active_profile_state();
            self.as_mut().set_status_message(QString::from(&format!(
                "Reordered Auto-Switcher rule '{moved_class}' to position #{}",
                to_idx + 1
            )));
        }
        reordered
    }

    pub fn is_browser_window_class(&self, resource_class: QString) -> bool {
        is_web_browser_class(&resource_class.to_string())
    }

    pub fn clean_browser_window_title(&self, title: QString) -> QString {
        QString::from(&strip_browser_title_suffix(&title.to_string()))
    }
}
