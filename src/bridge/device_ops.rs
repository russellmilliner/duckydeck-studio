use crate::bridge::device_controller::{
    is_demo_showcase_env, is_ui_test_suite_env, qobject, record_system_event, DeviceControllerRust,
};
use crate::hardware::storage::{
    format_disk_label_for_device, get_selected_duckypad_serial, list_manageable_duckypad_targets,
    load_profiles_for_connected_device, model_name_for_hw_model, normalize_duckypad_serial,
    onboard_key_count_for_model, query_connected_duckypad_info, read_duckypad_pro_profiles_via_msc,
    set_selected_duckypad_serial, sync_profiles_to_device_cache_with_meta,
    try_auto_mount_duckypad_sd_for_device, DuckySdDeviceMeta, DP_MODEL_DUCKYPAD_PRO,
    DP_MODEL_OG_DUCKYPAD, PID_DUCKYPAD_2020, PID_DUCKYPAD_PRO,
};
use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use serde_json::json;

impl qobject::DeviceController {
    pub(crate) fn update_hardware_model_properties(mut self: Pin<&mut Self>, hw_model: u8) {
        let normalized = if hw_model == DP_MODEL_OG_DUCKYPAD {
            DP_MODEL_OG_DUCKYPAD
        } else {
            DP_MODEL_DUCKYPAD_PRO
        };
        let is_dp20 = normalized == DP_MODEL_OG_DUCKYPAD;
        let key_count = onboard_key_count_for_model(normalized) as i32;
        if *self.active_hw_model() != normalized as i32 {
            self.as_mut().set_active_hw_model(normalized as i32);
        }
        if *self.is_duckypad_2020() != is_dp20 {
            self.as_mut().set_is_duckypad_2020(is_dp20);
        }
        if *self.onboard_key_count() != key_count {
            self.as_mut().set_onboard_key_count(key_count);
        }
    }

    pub(crate) fn sync_multi_device_oled_states(mut self: Pin<&mut Self>) {
        let mut map = serde_json::Map::new();
        let active_serial = normalize_duckypad_serial(&self.serial_number().to_string());
        let active_hw = (*self.active_hw_model()).clamp(20, 24) as u8;
        let active_prof_num = (*self.active_profile()).clamp(1, 64) as u8;
        let active_connected = *self.connected();
        let active_model_name = self.model_name().to_string();

        {
            let rust = self.rust();
            let prof_opt = rust
                .profiles
                .iter()
                .find(|p| p.number == active_prof_num)
                .or_else(|| rust.profiles.first());
            let (prof_name, hw_rot, eff_rot, slots_val) = if let Some(p) = prof_opt {
                let slots_raw = DeviceControllerRust::serialize_profile_slots(p);
                let parsed_slots =
                    serde_json::from_str::<serde_json::Value>(&slots_raw).unwrap_or(json!([]));
                (
                    p.name.clone(),
                    p.oled_rotation_degrees(),
                    p.effective_widget_rotation_degrees(),
                    parsed_slots,
                )
            } else {
                ("Default".to_string(), 0, 0, json!([]))
            };
            let key_id = if !active_serial.is_empty() {
                active_serial.clone()
            } else {
                "SELECTED".to_string()
            };
            map.insert(
                key_id.clone(),
                json!({
                    "id": key_id,
                    "serialNumber": if active_serial.is_empty() { "-".to_string() } else { active_serial.clone() },
                    "modelName": active_model_name,
                    "hwModel": active_hw,
                    "isDuckyPad2020": active_hw == DP_MODEL_OG_DUCKYPAD,
                    "onboardKeyCount": onboard_key_count_for_model(active_hw),
                    "connected": active_connected,
                    "isSelected": true,
                    "activeProfile": active_prof_num,
                    "activeProfileName": prof_name,
                    "profileCount": rust.profiles.len(),
                    "oledRotation": hw_rot,
                    "effectiveWidgetRotation": eff_rot,
                    "slots": slots_val,
                }),
            );
        }

        for sec in crate::kwin::auto_switcher::get_secondary_devices_snapshot() {
            let clean_sn = normalize_duckypad_serial(&sec.serial);
            if clean_sn.is_empty() || map.contains_key(&clean_sn) {
                continue;
            }
            let prof_opt = sec
                .profiles
                .iter()
                .find(|p| p.number == sec.active_profile)
                .or_else(|| sec.profiles.first());
            let (prof_num, prof_name, hw_rot, eff_rot, slots_val) = if let Some(p) = prof_opt {
                let slots_raw = DeviceControllerRust::serialize_profile_slots(p);
                let parsed_slots =
                    serde_json::from_str::<serde_json::Value>(&slots_raw).unwrap_or(json!([]));
                (
                    p.number,
                    p.name.clone(),
                    p.oled_rotation_degrees(),
                    p.effective_widget_rotation_degrees(),
                    parsed_slots,
                )
            } else {
                (
                    sec.active_profile.max(1),
                    "Default".to_string(),
                    0,
                    0,
                    json!([]),
                )
            };
            map.insert(
                clean_sn.clone(),
                json!({
                    "id": clean_sn,
                    "serialNumber": sec.serial,
                    "modelName": sec.model_name,
                    "hwModel": sec.hw_model,
                    "isDuckyPad2020": sec.hw_model == DP_MODEL_OG_DUCKYPAD,
                    "onboardKeyCount": onboard_key_count_for_model(sec.hw_model),
                    "connected": sec.usb_connected,
                    "isSelected": false,
                    "activeProfile": prof_num,
                    "activeProfileName": prof_name,
                    "profileCount": sec.profiles.len(),
                    "oledRotation": hw_rot,
                    "effectiveWidgetRotation": eff_rot,
                    "slots": slots_val,
                }),
            );
        }

        let serialized = serde_json::Value::Object(map).to_string();
        if self.multi_device_oled_states_json().to_string() != serialized {
            self.as_mut()
                .set_multi_device_oled_states_json(QString::from(&serialized));
        }
    }

    pub fn switch_profile_for_device(
        mut self: Pin<&mut Self>,
        device_id: QString,
        profile_number: i32,
    ) -> bool {
        let raw_id = normalize_duckypad_serial(&device_id.to_string());
        let active_serial = normalize_duckypad_serial(&self.serial_number().to_string());
        if raw_id.is_empty()
            || raw_id == "SELECTED"
            || (!active_serial.is_empty()
                && crate::hardware::storage::serial_matches_target(&raw_id, &active_serial))
        {
            return self.switch_profile(profile_number);
        }

        let clamped = profile_number.clamp(1, 64) as u8;
        let _ =
            crate::kwin::auto_switcher::send_hid_switch_profile_for_serial(Some(&raw_id), clamped);
        crate::kwin::auto_switcher::update_secondary_device_active_profile(&raw_id, clamped);
        self.as_mut().sync_multi_device_oled_states();
        let _ = self.as_mut().refresh_available_devices_state();
        true
    }

    pub fn set_device_profile_widget_rotation(
        mut self: Pin<&mut Self>,
        device_id: QString,
        degrees: i32,
    ) -> bool {
        let raw_id = normalize_duckypad_serial(&device_id.to_string());
        let active_serial = normalize_duckypad_serial(&self.serial_number().to_string());
        if raw_id.is_empty()
            || raw_id == "SELECTED"
            || (!active_serial.is_empty()
                && crate::hardware::storage::serial_matches_target(&raw_id, &active_serial))
        {
            return self.set_active_profile_widget_rotation(degrees);
        }

        let norm_deg = match degrees {
            90 => 90u16,
            180 => 180u16,
            270 => 270u16,
            _ => 0u16,
        };
        let ok =
            crate::kwin::auto_switcher::set_secondary_device_profile_rotation(&raw_id, norm_deg);
        self.as_mut().sync_multi_device_oled_states();
        ok
    }

    pub(crate) fn sync_esp32_ha_state(mut self: Pin<&mut Self>) {
        if is_demo_showcase_env() {
            return;
        }
        let active_serial = normalize_duckypad_serial(&self.serial_number().to_string());
        let active_custom_name =
            crate::hardware::profile::get_custom_duckypad_name(&active_serial).unwrap_or_default();
        if self.custom_device_name().to_string() != active_custom_name {
            self.as_mut()
                .set_custom_device_name(QString::from(&active_custom_name));
        }

        let active_is_esp32 = *self.connected()
            && crate::hardware::profile::is_esp32_ha_configured_profiles(&self.rust().profiles);

        let mut detected_serial: Option<String> = if active_is_esp32 && !active_serial.is_empty() {
            Some(active_serial)
        } else {
            None
        };

        if detected_serial.is_none() {
            for sec in crate::kwin::auto_switcher::get_secondary_devices_snapshot() {
                if sec.usb_connected
                    && crate::hardware::profile::is_esp32_ha_configured_profiles(&sec.profiles)
                {
                    detected_serial = Some(sec.serial.clone());
                    break;
                }
            }
        }

        if detected_serial.is_none() {
            if let Ok(targets) = serde_json::from_str::<
                Vec<crate::hardware::profile::ManageableDuckyPadTarget>,
            >(&self.available_devices_json().to_string())
            {
                if let Some(found) = targets
                    .iter()
                    .find(|t| t.is_esp32_ha_configured && t.usb_connected)
                {
                    detected_serial = Some(found.serial_number.clone());
                }
            }
        }

        let detected = detected_serial.is_some();
        let serial_str = detected_serial.unwrap_or_default();
        if *self.esp32_ha_detected() != detected {
            self.as_mut().set_esp32_ha_detected(detected);
        }
        if self.esp32_ha_device_serial().to_string() != serial_str {
            self.as_mut()
                .set_esp32_ha_device_serial(QString::from(&serial_str));
        }

        if !detected {
            if crate::actions::esp32_ha_server::is_esp32_ha_server_running() {
                crate::actions::esp32_ha_server::stop_esp32_ha_server();
                record_system_event(
                    "INFO",
                    "WEB_API",
                    "ESP32-HA Web Server stopped (no ESP32-HA duckyPad connected)",
                );
            }
        } else if !crate::actions::esp32_ha_server::is_esp32_ha_server_running() {
            let esp32_cfg_exists = crate::paths::config_dir()
                .join("esp32_ha_bridge.json")
                .exists();
            let esp32_cfg = crate::actions::esp32_ha_server::load_esp32_ha_bridge_config();
            if esp32_cfg.enabled || !esp32_cfg_exists {
                let _ = crate::actions::esp32_ha_server::start_esp32_ha_web_server(
                    esp32_cfg.port.max(1024),
                    if serial_str.is_empty() {
                        None
                    } else {
                        Some(serial_str.clone())
                    },
                );
            }
        }

        let running = crate::actions::esp32_ha_server::is_esp32_ha_server_running();
        let port = crate::actions::esp32_ha_server::esp32_ha_server_port() as i32;
        let url = crate::actions::esp32_ha_server::esp32_ha_server_url();
        if *self.esp32_ha_server_running() != running {
            self.as_mut().set_esp32_ha_server_running(running);
        }
        if port > 0 && *self.esp32_ha_server_port() != port {
            self.as_mut().set_esp32_ha_server_port(port);
        }
        if self.esp32_ha_server_url().to_string() != url {
            self.as_mut().set_esp32_ha_server_url(QString::from(&url));
        }
    }

    pub fn assign_custom_device_name(
        mut self: Pin<&mut Self>,
        serial: &QString,
        name: &QString,
    ) -> bool {
        let req_serial = normalize_duckypad_serial(&serial.to_string());
        let target_serial = if !req_serial.is_empty() {
            req_serial
        } else {
            normalize_duckypad_serial(&self.serial_number().to_string())
        };
        if target_serial.is_empty() || target_serial == "-" {
            let msg = "No duckyPad serial number available to name";
            self.as_mut().set_status_message(QString::from(msg));
            return false;
        }
        match crate::hardware::profile::set_custom_duckypad_name(&target_serial, &name.to_string())
        {
            Ok(saved) => {
                let _ = self.as_mut().refresh_available_devices_state();
                let msg = if saved.is_empty() {
                    format!("Cleared custom name for duckyPad (SN: {target_serial})")
                } else {
                    format!("Named duckyPad (SN: {target_serial}) → \"{saved}\"")
                };
                record_system_event("INFO", "DEVICE", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                true
            }
            Err(e) => {
                let msg = format!("Failed naming duckyPad: {e}");
                record_system_event("WARN", "DEVICE", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                false
            }
        }
    }

    pub fn start_esp32_ha_web_server(mut self: Pin<&mut Self>, port: i32) -> bool {
        let target_port = if (1024..=65535).contains(&port) {
            port as u16
        } else {
            let cur = *self.esp32_ha_server_port();
            if (1024..=65535).contains(&cur) {
                cur as u16
            } else {
                crate::actions::esp32_ha_server::DEFAULT_ESP32_HA_WEB_PORT
            }
        };
        let target_serial = {
            let s = self.esp32_ha_device_serial().to_string();
            if !s.trim().is_empty() {
                Some(s)
            } else {
                let cur_sn = normalize_duckypad_serial(&self.serial_number().to_string());
                if !cur_sn.is_empty() {
                    Some(cur_sn)
                } else {
                    None
                }
            }
        };
        match crate::actions::esp32_ha_server::start_esp32_ha_web_server(target_port, target_serial)
        {
            Ok(url) => {
                self.as_mut().sync_esp32_ha_state();
                let msg = format!("ESP32-HA Web Server started at {url}");
                record_system_event("INFO", "WEB_API", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                true
            }
            Err(err) => {
                self.as_mut().sync_esp32_ha_state();
                let msg = format!("Failed starting ESP32-HA Web Server: {err}");
                record_system_event("ERROR", "WEB_API", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                false
            }
        }
    }

    pub fn stop_esp32_ha_web_server(mut self: Pin<&mut Self>) -> bool {
        crate::actions::esp32_ha_server::stop_esp32_ha_web_server();
        self.as_mut().sync_esp32_ha_state();
        let msg = "ESP32-HA Web Server stopped";
        record_system_event("INFO", "WEB_API", msg);
        self.as_mut().sync_event_log_properties();
        self.as_mut().set_status_message(QString::from(msg));
        true
    }

    pub fn toggle_esp32_ha_web_server(mut self: Pin<&mut Self>) -> bool {
        if crate::actions::esp32_ha_server::is_esp32_ha_server_running() {
            self.as_mut().stop_esp32_ha_web_server()
        } else {
            let port = *self.esp32_ha_server_port();
            self.as_mut().start_esp32_ha_web_server(port)
        }
    }

    pub fn open_esp32_ha_web_ui(mut self: Pin<&mut Self>) -> bool {
        if !crate::actions::esp32_ha_server::is_esp32_ha_server_running() {
            let port = *self.esp32_ha_server_port();
            if !self.as_mut().start_esp32_ha_web_server(port) {
                return false;
            }
        }
        let url = crate::actions::esp32_ha_server::esp32_ha_server_url();
        if let Err(e) = crate::paths::open_url_or_path_in_os(&url) {
            let msg = format!("Failed opening ESP32-HA Web UI ({url}): {e}");
            record_system_event("WARN", "WEB_API", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            return false;
        }
        let msg = format!("Opened ESP32-HA Web UI ({url})");
        self.as_mut().set_status_message(QString::from(&msg));
        true
    }

    pub(crate) fn refresh_available_devices_state(mut self: Pin<&mut Self>) -> String {
        if is_demo_showcase_env() {
            self.as_mut().sync_multi_device_oled_states();
            return self.available_devices_json().to_string();
        }
        let targets = list_manageable_duckypad_targets();
        let connected_count = targets.iter().filter(|t| t.usb_connected).count() as i32;
        let json_str = serde_json::to_string(&targets).unwrap_or_else(|_| "[]".to_string());
        if *self.connected_device_count() != connected_count {
            self.as_mut().set_connected_device_count(connected_count);
        }
        if self.available_devices_json().to_string() != json_str {
            self.as_mut()
                .set_available_devices_json(QString::from(&json_str));
        }
        self.as_mut().sync_multi_device_oled_states();
        self.as_mut().sync_esp32_ha_state();
        json_str
    }

    pub fn scan_available_devices(mut self: Pin<&mut Self>) -> QString {
        let json_str = self.as_mut().refresh_available_devices_state();
        QString::from(&json_str)
    }

    pub fn set_simulated_device_model(mut self: Pin<&mut Self>, hw_model: i32) -> bool {
        let target_model = if hw_model == DP_MODEL_OG_DUCKYPAD as i32 {
            DP_MODEL_OG_DUCKYPAD
        } else {
            DP_MODEL_DUCKYPAD_PRO
        };
        self.as_mut().update_hardware_model_properties(target_model);
        let model_label = model_name_for_hw_model(target_model);
        if !*self.connected() {
            if !self.sd_mount_path().is_empty() {
                self.as_mut()
                    .set_model_name(QString::from(&format!("{model_label} (SD Mode)")));
            } else {
                self.as_mut().set_model_name(QString::from(model_label));
            }
        } else {
            self.as_mut().set_model_name(QString::from(model_label));
        }
        let msg = format!(
            "Switched hardware layout to {} ({} mechanical keys{})",
            model_label,
            onboard_key_count_for_model(target_model),
            if target_model == DP_MODEL_OG_DUCKYPAD {
                ", 3×5 grid"
            } else {
                " + 2 rotary encoders, 4×5 grid"
            }
        );
        record_system_event("INFO", "HARDWARE", &msg);
        self.as_mut().sync_event_log_properties();
        self.as_mut().set_status_message(QString::from(&msg));
        self.as_mut().sync_active_profile_state();
        true
    }

    pub fn select_managed_device(mut self: Pin<&mut Self>, device_id: QString) -> bool {
        let raw_id = device_id.to_string().trim().to_string();
        if raw_id.is_empty() {
            return false;
        }

        // Save current profiles and auto-switch rules to previous device cache before switching
        let prev_serial = self.serial_number().to_string();
        let prev_hw_model = (*self.active_hw_model()).clamp(20, 24) as u8;
        let prev_fw = self.firmware_version().to_string();
        let prev_active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        if !is_demo_showcase_env() && !prev_serial.is_empty() && prev_serial != "-" {
            let meta = DuckySdDeviceMeta {
                serial_number: prev_serial.clone(),
                hw_model: prev_hw_model,
                model_name: model_name_for_hw_model(prev_hw_model).to_string(),
                firmware_version: prev_fw.clone(),
                disk_label: format_disk_label_for_device(prev_hw_model, &prev_serial),
            };
            sync_profiles_to_device_cache_with_meta(
                &prev_serial,
                &self.rust().profiles,
                Some(&meta),
            );
            if let Ok(sw) = self.rust().switcher_state.lock() {
                crate::kwin::auto_switcher::save_device_rules_for_serial(
                    &prev_serial,
                    sw.enabled,
                    sw.default_profile,
                    &sw.rules,
                    &self.rust().profiles,
                    self.rust().sd_root_path.as_deref(),
                );
                crate::kwin::auto_switcher::update_secondary_device_runtime(
                    crate::kwin::auto_switcher::SecondaryDeviceRuntime {
                        serial: prev_serial.clone(),
                        pid: if prev_hw_model == DP_MODEL_OG_DUCKYPAD {
                            crate::hardware::storage::PID_DUCKYPAD_2020
                        } else {
                            crate::hardware::storage::PID_DUCKYPAD_PRO
                        },
                        hw_model: prev_hw_model,
                        model_name: model_name_for_hw_model(prev_hw_model).to_string(),
                        firmware_version: prev_fw.clone(),
                        disk_label: format_disk_label_for_device(prev_hw_model, &prev_serial),
                        usb_connected: *self.connected(),
                        active_profile: prev_active_prof,
                        profiles: self.rust().profiles.clone(),
                        rules_enabled: sw.enabled,
                        default_profile: sw.default_profile,
                        rules: sw.rules.clone(),
                    },
                );
            }
        }
        self.as_mut()
            .set_auto_switch_error_message(QString::from(""));

        if is_demo_showcase_env() {
            let is_dp20 = raw_id.contains("A042B910") || raw_id.contains("DP20");
            let new_model = if is_dp20 {
                DP_MODEL_OG_DUCKYPAD
            } else {
                DP_MODEL_DUCKYPAD_PRO
            };
            self.as_mut().update_hardware_model_properties(new_model);
            self.as_mut()
                .set_model_name(QString::from(model_name_for_hw_model(new_model)));
            self.as_mut().set_serial_number(QString::from(if is_dp20 {
                "DP20-A042B910"
            } else {
                "DP24-8F4A1C9E"
            }));
            self.as_mut()
                .set_firmware_version(QString::from("1.1.2 (DuckyDeck Studio)"));
            let updated_demo_targets = serde_json::json!([
                {
                    "id": "8F4A1C9E",
                    "serialNumber": "8F4A1C9E",
                    "hwModel": 24,
                    "pid": 53533,
                    "modelName": "duckyPad Pro",
                    "firmwareVersion": "1.1.2",
                    "diskLabel": "DP24_1C9E",
                    "usbConnected": true,
                    "activeProfile": 1,
                    "isBusy": false,
                    "sdMounted": true,
                    "sdMountPath": "/run/media/ducky/DP24_1C9E",
                    "cachedProfileCount": 8,
                    "isSelected": !is_dp20
                },
                {
                    "id": "A042B910",
                    "serialNumber": "A042B910",
                    "hwModel": 20,
                    "pid": 53532,
                    "modelName": "duckyPad (2020)",
                    "firmwareVersion": "1.1.2",
                    "diskLabel": "DP20_B910",
                    "usbConnected": true,
                    "activeProfile": 1,
                    "isBusy": false,
                    "sdMounted": false,
                    "sdMountPath": "",
                    "cachedProfileCount": 4,
                    "isSelected": is_dp20
                }
            ])
            .to_string();
            self.as_mut()
                .set_available_devices_json(QString::from(&updated_demo_targets));
            self.as_mut().set_status_message(QString::from(&format!(
                "Now managing {} ({})",
                model_name_for_hw_model(new_model),
                if is_dp20 {
                    "DP20-A042B910"
                } else {
                    "DP24-8F4A1C9E"
                }
            )));
            self.as_mut().sync_active_profile_state();
            return true;
        }

        if let Some(sd_path_str) = raw_id.strip_prefix("SD:") {
            set_selected_duckypad_serial(Some(&raw_id));
            let ok = self.as_mut().load_sd_directory(QString::from(sd_path_str));
            let _ = self.as_mut().refresh_available_devices_state();
            return ok;
        }

        let targets = list_manageable_duckypad_targets();
        let matched_target = targets
            .iter()
            .find(|t| {
                t.id.eq_ignore_ascii_case(&raw_id)
                    || t.serial_number.eq_ignore_ascii_case(&raw_id)
                    || t.disk_label.eq_ignore_ascii_case(&raw_id)
            })
            .cloned();

        let clean_serial = matched_target
            .as_ref()
            .map(|t| normalize_duckypad_serial(&t.serial_number))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| normalize_duckypad_serial(&raw_id));

        if !clean_serial.is_empty() {
            set_selected_duckypad_serial(Some(&clean_serial));
        } else {
            set_selected_duckypad_serial(Some(&raw_id));
        }

        // Clear previous device's mounted SD root before resolving the newly selected device's SD card
        self.as_mut().rust_mut().get_mut().sd_root_path = None;
        self.as_mut().set_sd_mount_path(QString::from(""));

        if let Some(target) = matched_target {
            self.as_mut()
                .update_hardware_model_properties(target.hw_model);
            self.as_mut()
                .set_serial_number(QString::from(&target.serial_number));
            self.as_mut()
                .set_firmware_version(QString::from(&target.firmware_version));
            self.as_mut().set_connected(target.usb_connected);
            self.as_mut()
                .set_model_name(QString::from(&target.model_name));

            if target.sd_mounted && !target.sd_mount_path.is_empty() {
                let _ = self
                    .as_mut()
                    .load_sd_directory(QString::from(&target.sd_mount_path));
            } else {
                let loaded_opt =
                    load_profiles_for_connected_device(target.pid, &target.serial_number)
                        .or_else(|| {
                            crate::kwin::auto_switcher::get_secondary_device_runtime(
                                &target.serial_number,
                            )
                            .and_then(|sec| (!sec.profiles.is_empty()).then_some(sec.profiles))
                        })
                        .or_else(|| {
                            if target.usb_connected
                                && (target.pid == PID_DUCKYPAD_PRO
                                    || target.pid == PID_DUCKYPAD_2020)
                                && !target.disk_label.is_empty()
                            {
                                read_duckypad_pro_profiles_via_msc(
                                    &target.disk_label,
                                    &target.serial_number,
                                    target.active_profile,
                                )
                                .ok()
                            } else {
                                None
                            }
                        });
                let loaded = loaded_opt.unwrap_or_else(|| {
                    crate::hardware::profile::create_default_profiles_for_device(
                        target.pid,
                        &target.serial_number,
                    )
                });
                crate::kwin::auto_switcher::update_secondary_device_profiles(
                    &target.serial_number,
                    loaded.clone(),
                );
                let (dev_enabled, dev_default, dev_rules) =
                    crate::kwin::auto_switcher::load_device_rules_for_serial(
                        &target.serial_number,
                        &loaded,
                        None,
                    );
                let rust = self.as_mut().rust_mut().get_mut();
                if let Ok(mut sw) = rust.switcher_state.lock() {
                    sw.enabled = dev_enabled;
                    sw.default_profile = dev_default;
                    sw.rules = dev_rules;
                    sw.align_rules_with_profiles(&loaded);
                    sw.last_switched_profile = target.active_profile.clamp(1, 64);
                }
                rust.profiles = loaded;
                self.as_mut().set_auto_switch_enabled(dev_enabled);
                self.as_mut()
                    .set_default_fallback_profile(dev_default.unwrap_or(1) as i32);
            }

            let active_p = (target.active_profile as i32).clamp(1, 64);
            self.as_mut().set_active_profile(active_p);
            let _ = self.as_mut().refresh_sd_mount_status();
            let _ = self.as_mut().refresh_available_devices_state();
            self.as_mut().sync_active_profile_state();

            let msg = format!(
                "Now managing {} (SN: {}{}) — {} profile(s) loaded",
                target.model_name,
                target.serial_number,
                if target.sd_mounted {
                    format!(", SD at {}", target.sd_mount_path)
                } else if target.usb_connected {
                    ", USB HID connected".to_string()
                } else {
                    ", offline cache".to_string()
                },
                self.rust().profiles.len()
            );
            record_system_event("INFO", "HARDWARE", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            return true;
        }

        self.as_mut().refresh_device();
        true
    }

    pub fn refresh_device(mut self: Pin<&mut Self>) {
        if is_demo_showcase_env() {
            self.as_mut().sync_active_profile_state();
            self.as_mut().sync_tray_state();
            return;
        }
        if Self::has_permission_denied_hidraw() {
            self.as_mut().set_connected(false);
            self.as_mut().set_udev_rule_missing(true);
            self.as_mut()
                .set_status_message(QString::from("USB udev permissions required"));
            let _ = self.as_mut().refresh_available_devices_state();
            self.as_mut().sync_tray_state();
            if !is_ui_test_suite_env() && !self.rust().kwin_listener_started {
                self.as_mut().start_kwin_listener();
            }
            return;
        }
        self.as_mut().set_udev_rule_missing(false);

        if let Some(ref sd_root) = self.rust().sd_root_path.clone() {
            if !crate::hardware::storage::is_duckypad_sd_root(sd_root) {
                self.as_mut().rust_mut().get_mut().sd_root_path = None;
                self.as_mut().set_sd_mount_path(QString::from(""));
            }
        }

        match query_connected_duckypad_info() {
            Ok(info) => {
                if !info.serial.is_empty()
                    && info.serial != "-"
                    && get_selected_duckypad_serial().is_none()
                {
                    set_selected_duckypad_serial(Some(&info.serial));
                }
                let model = model_name_for_hw_model(info.hw_model);
                self.as_mut()
                    .update_hardware_model_properties(info.hw_model);
                self.as_mut().set_model_name(QString::from(model));
                self.as_mut().set_connected(true);

                // Check if the SD card for THIS specific duckyPad is mounted (or can be auto-mounted)
                let matching_sd = try_auto_mount_duckypad_sd_for_device(
                    if info.serial.is_empty() || info.serial == "-" {
                        None
                    } else {
                        Some(&info.serial)
                    },
                    if info.disk_label.is_empty() {
                        None
                    } else {
                        Some(&info.disk_label)
                    },
                    Some(info.hw_model),
                );
                if let Some(ref sd_root) = matching_sd {
                    if self.rust().sd_root_path.as_ref() != Some(sd_root) {
                        self.as_mut()
                            .load_sd_directory(QString::from(sd_root.to_string_lossy().as_ref()));
                    }
                } else if self.rust().sd_root_path.is_some() {
                    // Previously mounted SD card doesn't belong to this device or was unmounted
                    self.as_mut().rust_mut().get_mut().sd_root_path = None;
                    self.as_mut().set_sd_mount_path(QString::from(""));
                }

                // Load the profile list for this connected duckyPad if not already loaded from a mounted SD card
                let prev_serial = self.serial_number().to_string();
                let need_profile_load = self.rust().sd_root_path.is_none()
                    && (self.rust().profiles.len() <= 1
                        || (info.serial != "-"
                            && prev_serial != "-"
                            && prev_serial != info.serial));

                if need_profile_load {
                    let loaded_opt = load_profiles_for_connected_device(info.pid, &info.serial)
                        .or_else(|| {
                            if !info.disk_label.is_empty() {
                                read_duckypad_pro_profiles_via_msc(
                                    &info.disk_label,
                                    &info.serial,
                                    info.active_profile,
                                )
                                .ok()
                            } else {
                                None
                            }
                        });

                    if let Some(loaded) = loaded_opt {
                        let sd_root_for_rules = self.rust().sd_root_path.clone();
                        let (dev_enabled, dev_default, dev_rules) =
                            crate::kwin::auto_switcher::load_device_rules_for_serial(
                                &info.serial,
                                &loaded,
                                sd_root_for_rules.as_deref(),
                            );
                        let rust = self.as_mut().rust_mut().get_mut();
                        if let Ok(mut sw) = rust.switcher_state.lock() {
                            sw.enabled = dev_enabled;
                            sw.default_profile = dev_default;
                            sw.rules = dev_rules;
                            sw.align_rules_with_profiles(&loaded);
                        }
                        rust.profiles = loaded;
                        self.as_mut().set_auto_switch_enabled(dev_enabled);
                        self.as_mut()
                            .set_default_fallback_profile(dev_default.unwrap_or(1) as i32);
                    }
                } else if self.rust().sd_root_path.is_none() {
                    let rust = self.as_mut().rust_mut().get_mut();
                    if let Ok(mut sw) = rust.switcher_state.lock() {
                        sw.align_rules_with_profiles(&rust.profiles);
                    }
                }

                let current_prof = (info.active_profile as i32).clamp(1, 64);
                {
                    let rust = self.as_mut().rust_mut().get_mut();
                    if let Ok(mut sw) = rust.switcher_state.lock() {
                        sw.last_switched_profile = current_prof as u8;
                    }
                }

                if info.fw_version != "-" {
                    self.as_mut()
                        .set_firmware_version(QString::from(&info.fw_version));
                }
                if info.serial != "-" {
                    self.as_mut().set_serial_number(QString::from(&info.serial));
                }
                self.as_mut().set_active_profile(current_prof);
                let _ = self.as_mut().refresh_available_devices_state();
                self.as_mut().sync_active_profile_state();

                let count = self.rust().profiles.len();
                self.as_mut().set_status_message(QString::from(&format!(
                    "Connected to {model} (FW v{}) — {count} profile(s) loaded",
                    info.fw_version
                )));
                self.as_mut().sync_tray_state();
                if !is_ui_test_suite_env() && self.latest_firmware_version().to_string() == "-" {
                    self.as_mut().check_firmware_update(false);
                }
            }
            Err(msg) => {
                self.as_mut().set_connected(false);
                if self.rust().sd_root_path.is_none() {
                    if let Some(sd_root) = try_auto_mount_duckypad_sd_for_device(None, None, None) {
                        self.as_mut()
                            .load_sd_directory(QString::from(sd_root.to_string_lossy().as_ref()));
                    }
                }
                if self.rust().sd_root_path.is_none() {
                    self.as_mut().set_model_name(QString::from("Disconnected"));
                    self.as_mut().set_firmware_version(QString::from("-"));
                    self.as_mut().set_serial_number(QString::from("-"));
                    self.as_mut()
                        .set_latest_firmware_version(QString::from("-"));
                    self.as_mut().set_firmware_update_available(false);
                    self.as_mut().set_status_message(QString::from(&msg));
                }
                let _ = self.as_mut().refresh_available_devices_state();
                self.as_mut().sync_active_profile_state();
                self.as_mut().sync_tray_state();
            }
        }

        let _ = self.as_mut().refresh_onboarding_state();

        // Start the KWin Wayland Auto-Switcher D-Bus listener if not started yet
        if !is_ui_test_suite_env() {
            if !self.rust().kwin_listener_started {
                self.as_mut().start_kwin_listener();
            } else {
                self.as_mut().apply_auto_switch_for_current_window();
            }
        }
    }
}
