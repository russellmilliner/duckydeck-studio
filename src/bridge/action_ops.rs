use crate::bridge::device_controller::{qobject, record_system_event};
use crate::hardware::storage::{
    apply_app_launch_config_to_script, apply_web_api_config_to_script,
    delete_web_api_destination_by_name, encode_pgv31_trigger, execute_app_launch,
    execute_profile_webhook_request, execute_web_api_request, extract_app_launch_config,
    extract_web_api_config, flash_slot_led_feedback, load_web_api_destinations,
    scan_installed_desktop_apps, set_single_key_led_hid, sync_profile_web_api_triggers,
    sync_profiles_to_cache, sync_profiles_to_device_cache, upsert_web_api_destination,
    AppLaunchConfig, GlobalMacroEvdevRecorder, ProfileWebhookConfig, WebApiConfig, MAX_INPUT_SLOTS,
};
use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use std::sync::{Mutex, OnceLock};

impl qobject::DeviceController {
    pub fn extract_script_web_api_json(&self, script: QString, slot_number: i32) -> QString {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let cfg = extract_web_api_config(&script.to_string(), slot_idx);
        let json_str = serde_json::to_string(&cfg).unwrap_or_else(|_| "{}".to_string());
        QString::from(&json_str)
    }

    pub fn apply_web_api_to_script(
        &self,
        existing_script: QString,
        config_json: QString,
        slot_number: i32,
        is_release: bool,
    ) -> QString {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        let cfg: WebApiConfig = serde_json::from_str(&config_json.to_string()).unwrap_or_default();
        let updated = apply_web_api_config_to_script(
            &existing_script.to_string(),
            &cfg,
            active_prof,
            slot_idx,
            is_release,
        );
        QString::from(&updated)
    }

    pub fn test_web_api_call(mut self: Pin<&mut Self>, config_json: QString, slot_number: i32) {
        if *self.web_api_test_busy() {
            return;
        }
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let cfg: WebApiConfig = match serde_json::from_str(&config_json.to_string()) {
            Ok(c) => c,
            Err(e) => {
                let err = format!("Invalid Web API configuration on Key {slot_idx}: {e}");
                record_system_event("ERROR", "WEB_API", &err);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_web_api_last_ok(false);
                self.as_mut().set_web_api_last_result(QString::from(&err));
                self.as_mut().set_status_message(QString::from(&err));
                return;
            }
        };

        let active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        let restore_rgb = self.lookup_slot_rgb(active_prof, slot_idx);
        let source_label = format!("Test Web API (Profile #{active_prof}, Key {slot_idx})");
        self.as_mut()
            .dispatch_web_api_execution(cfg, slot_idx, restore_rgb, &source_label);
    }

    pub fn reload_web_api_destinations(mut self: Pin<&mut Self>) -> QString {
        let list = load_web_api_destinations();
        let json_str = serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
        let qstr = QString::from(&json_str);
        self.as_mut().set_web_api_destinations_json(qstr.clone());
        qstr
    }

    pub fn save_web_api_destination(
        mut self: Pin<&mut Self>,
        name: QString,
        method: QString,
        url: QString,
        headers: QString,
    ) -> QString {
        let name_str = name.to_string();
        let method_str = method.to_string();
        let url_str = url.to_string();
        let headers_str = headers.to_string();

        match upsert_web_api_destination(&name_str, &method_str, &url_str, &headers_str) {
            Ok(list) => {
                let json_str = serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
                let qstr = QString::from(&json_str);
                self.as_mut().set_web_api_destinations_json(qstr.clone());

                // Propagate updated destination method/url/headers to any slots referencing this destination
                let any_updated = {
                    let mut rust = self.as_mut().rust_mut();
                    sync_profile_web_api_triggers(&mut rust.profiles)
                };
                if any_updated {
                    let sn = self.serial_number().to_string();
                    let profiles_clone = self.rust().profiles.clone();
                    if sn != "-" && !sn.is_empty() {
                        sync_profiles_to_device_cache(&sn, &profiles_clone);
                    } else {
                        sync_profiles_to_cache(&profiles_clone);
                    }
                    self.as_mut().sync_active_profile_state();
                }

                let msg = format!(
                    "Saved reusable Web API destination '{}' ({} {})",
                    name_str.trim(),
                    method_str.trim().to_ascii_uppercase(),
                    url_str.trim()
                );
                record_system_event("INFO", "WEB_API", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                qstr
            }
            Err(e) => {
                let msg = format!("Failed saving Web API destination: {e}");
                record_system_event("ERROR", "WEB_API", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                self.web_api_destinations_json().clone()
            }
        }
    }

    pub fn delete_web_api_destination(mut self: Pin<&mut Self>, name: QString) -> QString {
        let name_str = name.to_string();
        match delete_web_api_destination_by_name(&name_str) {
            Ok(list) => {
                let json_str = serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
                let qstr = QString::from(&json_str);
                self.as_mut().set_web_api_destinations_json(qstr.clone());
                let msg = format!("Deleted Web API destination '{}'", name_str.trim());
                record_system_event("INFO", "WEB_API", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                qstr
            }
            Err(e) => {
                let msg = format!("Failed deleting Web API destination: {e}");
                record_system_event("ERROR", "WEB_API", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                self.web_api_destinations_json().clone()
            }
        }
    }

    pub fn refresh_installed_apps(mut self: Pin<&mut Self>) -> QString {
        let apps = scan_installed_desktop_apps();
        let json_str = serde_json::to_string(&apps).unwrap_or_else(|_| "[]".to_string());
        let qstr = QString::from(&json_str);
        self.as_mut().set_installed_apps_json(qstr.clone());
        qstr
    }

    pub fn extract_script_app_launch_json(&self, script: QString, slot_number: i32) -> QString {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let cfg = extract_app_launch_config(&script.to_string(), slot_idx);
        let json_str = serde_json::to_string(&cfg).unwrap_or_else(|_| "{}".to_string());
        QString::from(&json_str)
    }

    pub fn apply_app_launch_to_script(
        &self,
        existing_script: QString,
        config_json: QString,
        slot_number: i32,
        is_release: bool,
    ) -> QString {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        let cfg: AppLaunchConfig =
            serde_json::from_str(&config_json.to_string()).unwrap_or_default();
        if cfg.enabled {
            let mut trimmed = cfg.clone();
            trimmed.command = trimmed.command.trim().to_string();
            trimmed.args = trimmed.args.trim().to_string();
            trimmed.working_dir = trimmed.working_dir.trim().to_string();
            crate::actions::app_launch::approve_app_launch_config(&trimmed);
        }
        let updated = apply_app_launch_config_to_script(
            &existing_script.to_string(),
            &cfg,
            active_prof,
            slot_idx,
            is_release,
        );
        QString::from(&updated)
    }

    pub fn test_app_launch(mut self: Pin<&mut Self>, config_json: QString, slot_number: i32) {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let cfg: AppLaunchConfig = match serde_json::from_str(&config_json.to_string()) {
            Ok(c) => c,
            Err(e) => {
                let err = format!("Invalid App Launch configuration on Key {slot_idx}: {e}");
                record_system_event("ERROR", "APP_LAUNCH", &err);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_app_launch_last_ok(false);
                self.as_mut()
                    .set_app_launch_last_result(QString::from(&err));
                self.as_mut().set_status_message(QString::from(&err));
                return;
            }
        };

        let active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        let restore_rgb = self.lookup_slot_rgb(active_prof, slot_idx);
        let source_label = format!("Test Launch (Profile #{active_prof}, Key {slot_idx})");
        crate::actions::app_launch::approve_app_launch_config(&cfg);
        self.as_mut()
            .dispatch_app_launch_execution(cfg, slot_idx, restore_rgb, &source_label);
    }

    pub(crate) fn lookup_slot_rgb(&self, profile_num: u8, slot_idx: usize) -> [u8; 3] {
        let rust = self.rust();
        if let Some(prof) = rust
            .profiles
            .iter()
            .find(|p| p.number == profile_num)
            .or_else(|| rust.profiles.first())
        {
            if let Some(slot) = prof.slots.get(&slot_idx) {
                return slot.custom_rgb.unwrap_or(prof.bg_color);
            }
            return prof.bg_color;
        }
        [0, 170, 255]
    }

    pub(crate) fn handle_hardware_pgv_web_api_trigger(
        mut self: Pin<&mut Self>,
        encoded_prof: u8,
        slot_num: usize,
        is_release: bool,
    ) {
        let active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        let phase = if is_release { "Release" } else { "Press" };
        let trigger_hex = format!(
            "0x{:08X}",
            encode_pgv31_trigger(encoded_prof, slot_num, is_release)
        );
        let mut found_web: Option<(u8, String, String, WebApiConfig, [u8; 3])> = None;
        let mut found_app: Option<(u8, String, String, AppLaunchConfig, [u8; 3])> = None;

        {
            let rust = self.rust();
            let candidate_profiles = if encoded_prof > 0 && encoded_prof != active_prof {
                vec![active_prof, encoded_prof]
            } else {
                vec![active_prof]
            };

            for pnum in candidate_profiles {
                if let Some(prof) = rust.profiles.iter().find(|p| p.number == pnum) {
                    if let Some(slot) = prof.slots.get(&slot_num) {
                        let script = if is_release {
                            &slot.on_release_script
                        } else {
                            &slot.on_press_script
                        };
                        let rgb = slot.custom_rgb.unwrap_or(prof.bg_color);
                        let label = format!("{} {}", slot.label_line1, slot.label_line2)
                            .trim()
                            .to_string();
                        let web_cfg = extract_web_api_config(script, slot_num);
                        if found_web.is_none() && web_cfg.enabled && !web_cfg.url.trim().is_empty()
                        {
                            found_web =
                                Some((prof.number, prof.name.clone(), label.clone(), web_cfg, rgb));
                        }
                        let app_cfg = extract_app_launch_config(script, slot_num);
                        if found_app.is_none()
                            && app_cfg.enabled
                            && app_cfg.trigger_mode == "pgv"
                            && (!app_cfg.command.trim().is_empty()
                                || !app_cfg.desktop_id.trim().is_empty())
                        {
                            found_app = Some((prof.number, prof.name.clone(), label, app_cfg, rgb));
                        }
                        if found_web.is_some() || found_app.is_some() {
                            break;
                        }
                    }
                }
            }

            if found_web.is_none() && found_app.is_none() {
                for prof in &rust.profiles {
                    if let Some(slot) = prof.slots.get(&slot_num) {
                        let script = if is_release {
                            &slot.on_release_script
                        } else {
                            &slot.on_press_script
                        };
                        if script.contains(&trigger_hex) {
                            let rgb = slot.custom_rgb.unwrap_or(prof.bg_color);
                            let label = format!("{} {}", slot.label_line1, slot.label_line2)
                                .trim()
                                .to_string();
                            let web_cfg = extract_web_api_config(script, slot_num);
                            if web_cfg.enabled && !web_cfg.url.trim().is_empty() {
                                found_web = Some((
                                    prof.number,
                                    prof.name.clone(),
                                    label.clone(),
                                    web_cfg,
                                    rgb,
                                ));
                            }
                            let app_cfg = extract_app_launch_config(script, slot_num);
                            if app_cfg.enabled
                                && app_cfg.trigger_mode == "pgv"
                                && (!app_cfg.command.trim().is_empty()
                                    || !app_cfg.desktop_id.trim().is_empty())
                            {
                                found_app =
                                    Some((prof.number, prof.name.clone(), label, app_cfg, rgb));
                            }
                            if found_web.is_some() || found_app.is_some() {
                                break;
                            }
                        }
                    }
                }
            }
        }

        let mut handled = false;
        if let Some((pnum, pname, label, app_cfg, restore_rgb)) = found_app {
            handled = true;
            let source_label = if label.is_empty() {
                format!("Profile #{pnum} '{pname}' Key {slot_num} ({phase})")
            } else {
                format!("Profile #{pnum} '{pname}' Key {slot_num} '{label}' ({phase})")
            };
            record_system_event(
                "INFO",
                "APP_LAUNCH",
                format!(
                    "Hardware PGV trigger received (_GV31 = {trigger_hex}) -> matched {source_label}"
                ),
            );
            self.as_mut().sync_event_log_properties();
            self.as_mut().dispatch_app_launch_execution(
                app_cfg,
                slot_num,
                restore_rgb,
                &source_label,
            );
        }

        if let Some((pnum, pname, label, cfg, restore_rgb)) = found_web {
            handled = true;
            let source_label = if label.is_empty() {
                format!("Profile #{pnum} '{pname}' Key {slot_num} ({phase})")
            } else {
                format!("Profile #{pnum} '{pname}' Key {slot_num} '{label}' ({phase})")
            };
            record_system_event(
                "INFO",
                "WEB_API",
                format!(
                    "Hardware PGV trigger received (_GV31 = {trigger_hex}) -> matched {source_label}"
                ),
            );
            self.as_mut().sync_event_log_properties();
            self.as_mut()
                .dispatch_web_api_execution(cfg, slot_num, restore_rgb, &source_label);
        }

        if !handled {
            record_system_event(
                "WARN",
                "APP_LAUNCH",
                format!(
                    "Hardware PGV trigger received (_GV31 = {trigger_hex}, Key {slot_num} {phase}, active Profile #{active_prof}, encoded Profile #{encoded_prof}) — no enabled App Launch or Web API found on this slot"
                ),
            );
            self.as_mut().sync_event_log_properties();
        }
    }

    pub(crate) fn handle_hardware_fkey_web_api_trigger(mut self: Pin<&mut Self>, fkey_name: &str) {
        let active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        let mut found_web: Option<(u8, String, usize, String, WebApiConfig, [u8; 3])> = None;
        let mut found_app: Option<(u8, String, usize, String, AppLaunchConfig, [u8; 3])> = None;

        {
            let rust = self.rust();
            if let Some(prof) = rust
                .profiles
                .iter()
                .find(|p| p.number == active_prof)
                .or_else(|| rust.profiles.first())
            {
                for (&slot_num, slot) in &prof.slots {
                    for script in [&slot.on_press_script, &slot.on_release_script] {
                        let rgb = slot.custom_rgb.unwrap_or(prof.bg_color);
                        let label = format!("{} {}", slot.label_line1, slot.label_line2)
                            .trim()
                            .to_string();
                        if found_web.is_none() {
                            let cfg = extract_web_api_config(script, slot_num);
                            if cfg.enabled
                                && cfg.trigger_mode == "fkey"
                                && cfg.fkey.eq_ignore_ascii_case(fkey_name)
                                && !cfg.url.trim().is_empty()
                            {
                                found_web = Some((
                                    prof.number,
                                    prof.name.clone(),
                                    slot_num,
                                    label.clone(),
                                    cfg,
                                    rgb,
                                ));
                            }
                        }
                        if found_app.is_none() {
                            let app_cfg = extract_app_launch_config(script, slot_num);
                            if app_cfg.enabled
                                && app_cfg.trigger_mode == "fkey"
                                && app_cfg.fkey.eq_ignore_ascii_case(fkey_name)
                                && (!app_cfg.command.trim().is_empty()
                                    || !app_cfg.desktop_id.trim().is_empty())
                            {
                                found_app = Some((
                                    prof.number,
                                    prof.name.clone(),
                                    slot_num,
                                    label,
                                    app_cfg,
                                    rgb,
                                ));
                            }
                        }
                    }
                    if found_web.is_some() || found_app.is_some() {
                        break;
                    }
                }
            }
        }

        if let Some((pnum, pname, slot_num, label, app_cfg, restore_rgb)) = found_app {
            let source_label = if label.is_empty() {
                format!("Profile #{pnum} '{pname}' Key {slot_num} ({fkey_name})")
            } else {
                format!("Profile #{pnum} '{pname}' Key {slot_num} '{label}' ({fkey_name})")
            };
            record_system_event(
                "INFO",
                "APP_LAUNCH",
                format!("Hardware sentinel key {fkey_name} intercepted -> matched {source_label}"),
            );
            self.as_mut().sync_event_log_properties();
            self.as_mut().dispatch_app_launch_execution(
                app_cfg,
                slot_num,
                restore_rgb,
                &source_label,
            );
        }

        if let Some((pnum, pname, slot_num, label, cfg, restore_rgb)) = found_web {
            let source_label = if label.is_empty() {
                format!("Profile #{pnum} '{pname}' Key {slot_num} ({fkey_name})")
            } else {
                format!("Profile #{pnum} '{pname}' Key {slot_num} '{label}' ({fkey_name})")
            };
            record_system_event(
                "INFO",
                "WEB_API",
                format!("Hardware sentinel key {fkey_name} intercepted -> matched {source_label}"),
            );
            self.as_mut().sync_event_log_properties();
            self.as_mut()
                .dispatch_web_api_execution(cfg, slot_num, restore_rgb, &source_label);
        }
    }

    pub(crate) fn dispatch_app_launch_execution(
        mut self: Pin<&mut Self>,
        cfg: AppLaunchConfig,
        slot_num: usize,
        restore_rgb: [u8; 3],
        source_label: &str,
    ) {
        let result = execute_app_launch(&cfg);
        let ok = result.ok;
        let log_lvl = if ok { "INFO" } else { "ERROR" };
        record_system_event(
            log_lvl,
            "APP_LAUNCH",
            format!(
                "{source_label} -> {} [cmd: {}]",
                result.summary, result.command_line
            ),
        );
        let summary_msg = if ok {
            format!("🚀 {source_label}: {}", result.summary)
        } else {
            format!("⚠ {source_label}: {}", result.summary)
        };
        self.as_mut().set_app_launch_last_ok(ok);
        self.as_mut()
            .set_app_launch_last_result(QString::from(&summary_msg));
        self.as_mut()
            .set_status_message(QString::from(&summary_msg));
        self.as_mut().sync_event_log_properties();
        if !ok {
            crate::kwin::auto_switcher::send_kde_notification(
                "actionExecuted",
                "duckyPad App Launch Error",
                &summary_msg,
                1,
            );
        }

        if cfg.led_feedback && (1..=20).contains(&slot_num) {
            crate::actions::web_api::spawn_shared_async(async move {
                flash_slot_led_feedback(slot_num, ok, restore_rgb).await;
            });
        }
    }

    pub(crate) fn dispatch_web_api_execution(
        mut self: Pin<&mut Self>,
        cfg: WebApiConfig,
        slot_num: usize,
        restore_rgb: [u8; 3],
        source_label: &str,
    ) {
        let method = cfg.method.trim().to_ascii_uppercase();
        let url = cfg.url.trim().to_string();
        let body_len = cfg.body.trim().len();
        let start_msg = format!("🌐 {source_label}: {method} {url} …");
        record_system_event(
            "INFO",
            "WEB_API",
            format!("Dispatching {method} {url} [source: {source_label}, body: {body_len} B]"),
        );
        self.as_mut().sync_event_log_properties();
        self.as_mut().set_web_api_test_busy(true);
        self.as_mut()
            .set_web_api_last_result(QString::from(&start_msg));
        self.as_mut().set_status_message(QString::from(&start_msg));

        let led_feedback = cfg.led_feedback;
        let source_owned = source_label.to_string();
        let method_owned = method.clone();
        let url_owned = url.clone();
        let qt_thread = self.qt_thread();

        crate::actions::web_api::spawn_shared_async(async move {
            if led_feedback && (1..=20).contains(&slot_num) {
                set_single_key_led_hid(slot_num, [0, 210, 255]);
            }
            let result = execute_web_api_request(&cfg).await;
            let ok = result.ok;
            if let Some(ref note) = result.route_note {
                record_system_event("INFO", "WEB_API", note);
            }
            let log_lvl = if ok { "INFO" } else { "ERROR" };
            record_system_event(
                log_lvl,
                "WEB_API",
                format!(
                    "{source_owned} -> {method_owned} {url_owned}: {}",
                    result.summary
                ),
            );
            let summary_msg = if ok {
                format!("✓ {source_owned}: {}", result.summary)
            } else {
                format!("⚠ {source_owned}: {}", result.summary)
            };

            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_web_api_test_busy(false);
                qobject.as_mut().set_web_api_last_ok(ok);
                qobject
                    .as_mut()
                    .set_web_api_last_result(QString::from(&summary_msg));
                qobject
                    .as_mut()
                    .set_status_message(QString::from(&summary_msg));
                qobject.as_mut().sync_event_log_properties();
                if !ok {
                    crate::kwin::auto_switcher::send_kde_notification(
                        "actionExecuted",
                        "duckyPad Web API Error",
                        &summary_msg,
                        1,
                    );
                }
            });

            if led_feedback && (1..=20).contains(&slot_num) {
                flash_slot_led_feedback(slot_num, ok, restore_rgb).await;
            }
        });
    }

    pub fn start_global_macro_capture(self: Pin<&mut Self>) -> i32 {
        let recorder = GlobalMacroEvdevRecorder::new();
        let count = recorder.device_count() as i32;
        if let Ok(mut guard) = global_macro_recorder_store().lock() {
            *guard = Some(recorder);
        }
        count
    }

    pub fn stop_global_macro_capture(self: Pin<&mut Self>) {
        if let Ok(mut guard) = global_macro_recorder_store().lock() {
            *guard = None;
        }
    }

    pub fn poll_global_macro_events_json(self: Pin<&mut Self>) -> QString {
        if let Ok(mut guard) = global_macro_recorder_store().lock() {
            if let Some(ref mut recorder) = *guard {
                let evts = recorder.poll_events();
                if !evts.is_empty() {
                    if let Ok(s) = serde_json::to_string(&evts) {
                        return QString::from(&s);
                    }
                }
            }
        }
        QString::from("[]")
    }

    pub fn get_global_profile_webhook_json(&self) -> QString {
        let rust = self.rust();
        if let Ok(sw) = rust.switcher_state.lock() {
            let obj = serde_json::json!({
                "enabled": sw.profile_webhook_enabled,
                "method": sw.profile_webhook_method,
                "url": sw.profile_webhook_url,
                "headers": sw.profile_webhook_headers,
                "body": sw.profile_webhook_body,
            });
            return QString::from(&obj.to_string());
        }
        let def = ProfileWebhookConfig::default();
        QString::from(&serde_json::to_string(&def).unwrap_or_else(|_| "{}".to_string()))
    }

    pub fn save_global_profile_webhook_json(
        mut self: Pin<&mut Self>,
        config_json: QString,
    ) -> bool {
        let raw = config_json.to_string();
        let Ok(cfg) = serde_json::from_str::<ProfileWebhookConfig>(&raw) else {
            return false;
        };
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.profile_webhook_enabled = cfg.enabled;
                sw.profile_webhook_method = if cfg.method.trim().is_empty() {
                    "POST".to_string()
                } else {
                    cfg.method.trim().to_ascii_uppercase()
                };
                sw.profile_webhook_url = cfg.url.trim().to_string();
                sw.profile_webhook_headers = cfg.headers;
                sw.profile_webhook_body = cfg.body;
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving global profile webhook config: {e}"),
                    );
                }
            }
        }
        let msg = if cfg.enabled {
            format!("✓ Saved Global Profile Change Webhook ({})", cfg.url.trim())
        } else {
            "✓ Updated Global Profile Change Webhook settings (disabled)".to_string()
        };
        record_system_event("INFO", "WEBHOOK", &msg);
        self.as_mut().sync_event_log_properties();
        self.as_mut().set_status_message(QString::from(&msg));
        true
    }

    pub fn test_profile_webhook_json(
        mut self: Pin<&mut Self>,
        config_json: QString,
        profile_number: i32,
    ) {
        let raw = config_json.to_string();
        let Ok(mut cfg) = serde_json::from_str::<ProfileWebhookConfig>(&raw) else {
            self.as_mut()
                .set_status_message(QString::from("Invalid webhook configuration JSON"));
            return;
        };
        cfg.enabled = true;
        if cfg.url.trim().is_empty() {
            self.as_mut()
                .set_status_message(QString::from("Enter a webhook URL before testing"));
            return;
        }
        let target_num = if profile_number > 0 {
            profile_number as u8
        } else {
            (*self.active_profile()).clamp(1, 64) as u8
        };
        let prof_name = self
            .rust()
            .profiles
            .iter()
            .find(|p| p.number == target_num)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| format!("Profile {target_num}"));
        let serial = self.serial_number().to_string();

        self.as_mut().set_web_api_test_busy(true);
        self.as_mut().set_status_message(QString::from(&format!(
            "Testing profile change webhook → {}...",
            cfg.url.trim()
        )));

        let qt_thread = self.qt_thread();
        crate::actions::web_api::spawn_shared_async(async move {
            let res = execute_profile_webhook_request(
                &cfg,
                target_num,
                &prof_name,
                target_num,
                &prof_name,
                "test_button",
                &serial,
            )
            .await;
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_web_api_test_busy(false);
                qobject.as_mut().set_web_api_last_ok(res.ok);
                qobject
                    .as_mut()
                    .set_web_api_last_result(QString::from(&res.summary));
                let lvl = if res.ok { "INFO" } else { "WARN" };
                let msg = format!(
                    "Profile Webhook test (HTTP {}): {}",
                    res.status_code, res.summary
                );
                record_system_event(lvl, "WEBHOOK", &msg);
                qobject.as_mut().sync_event_log_properties();
                qobject.as_mut().set_status_message(QString::from(&msg));
            });
        });
    }

    pub fn dispatch_profile_change_webhooks_if_changed(
        self: Pin<&mut Self>,
        previous_profile: u8,
        new_profile: u8,
        reason: &str,
    ) {
        if previous_profile == new_profile || new_profile == 0 {
            return;
        }
        let (prev_name, new_name, per_prof_webhook, global_webhook) = {
            let rust = self.rust();
            let prev_name = rust
                .profiles
                .iter()
                .find(|p| p.number == previous_profile)
                .map(|p| p.name.clone())
                .unwrap_or_else(|| format!("Profile {previous_profile}"));
            let new_prof_opt = rust.profiles.iter().find(|p| p.number == new_profile);
            let new_name = new_prof_opt
                .map(|p| p.name.clone())
                .unwrap_or_else(|| format!("Profile {new_profile}"));
            let per_prof_wh = new_prof_opt
                .map(|p| p.profile_webhook())
                .filter(|w| w.enabled && !w.url.trim().is_empty());
            let global_wh = rust
                .switcher_state
                .lock()
                .ok()
                .map(|sw| ProfileWebhookConfig {
                    enabled: sw.profile_webhook_enabled,
                    destination: String::new(),
                    method: sw.profile_webhook_method.clone(),
                    url: sw.profile_webhook_url.clone(),
                    headers: sw.profile_webhook_headers.clone(),
                    body: sw.profile_webhook_body.clone(),
                })
                .filter(|w| w.enabled && !w.url.trim().is_empty());
            (prev_name, new_name, per_prof_wh, global_wh)
        };

        if per_prof_webhook.is_none() && global_webhook.is_none() {
            return;
        }

        let serial = self.serial_number().to_string();
        let reason_str = reason.to_string();
        let qt_thread = self.qt_thread();

        crate::actions::web_api::spawn_shared_async(async move {
            let mut logs: Vec<(bool, String)> = Vec::new();
            if let Some(ref g_cfg) = global_webhook {
                let res = execute_profile_webhook_request(
                    g_cfg,
                    new_profile,
                    &new_name,
                    previous_profile,
                    &prev_name,
                    &reason_str,
                    &serial,
                )
                .await;
                logs.push((
                    res.ok,
                    format!(
                        "Global profile webhook → #{} ({}) [HTTP {}]: {}",
                        new_profile, new_name, res.status_code, res.summary
                    ),
                ));
            }
            if let Some(ref p_cfg) = per_prof_webhook {
                let res = execute_profile_webhook_request(
                    p_cfg,
                    new_profile,
                    &new_name,
                    previous_profile,
                    &prev_name,
                    &reason_str,
                    &serial,
                )
                .await;
                logs.push((
                    res.ok,
                    format!(
                        "Profile #{} ({}) webhook [HTTP {}]: {}",
                        new_profile, new_name, res.status_code, res.summary
                    ),
                ));
            }
            if !logs.is_empty() {
                let _ = qt_thread.queue(move |mut qobject| {
                    for (ok, msg) in logs {
                        record_system_event(if ok { "INFO" } else { "WARN" }, "WEBHOOK", &msg);
                    }
                    qobject.as_mut().sync_event_log_properties();
                });
            }
        });
    }
}

static GLOBAL_MACRO_RECORDER: OnceLock<Mutex<Option<GlobalMacroEvdevRecorder>>> = OnceLock::new();

fn global_macro_recorder_store() -> &'static Mutex<Option<GlobalMacroEvdevRecorder>> {
    GLOBAL_MACRO_RECORDER.get_or_init(|| Mutex::new(None))
}
