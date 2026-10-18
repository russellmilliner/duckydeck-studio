use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use std::path::PathBuf;
use std::pin::Pin;

use crate::api_server::{
    current_rest_api_status, start_or_restart_rest_api_server, RestApiServerConfig,
};
use crate::bridge::device_controller::{
    is_ui_test_suite_env, qobject, DeviceControllerRust, APP_VERSION,
};
use crate::bridge::event_log::record_system_event;
use crate::hardware::storage::{
    find_mounted_duckypad_sd, mark_profiles_sd_synced, mount_duckypad_sd_only,
    query_connected_duckypad_info, reboot_duckypad, set_single_key_led_hid,
    sync_profiles_to_device_cache, unmount_mounted_duckypad_sd,
    write_duckypad_pro_profiles_via_msc_with_progress, write_profiles_to_sd_root,
    MAX_INPUT_SLOTS, PID_DUCKYPAD_2020,
};
use crate::kwin::auto_switcher::generate_rest_api_token;

pub fn sync_rest_api_server_from_state_impl(mut controller: Pin<&mut qobject::DeviceController>) {
        if is_ui_test_suite_env() {
            controller.as_mut()
                .set_rest_api_status(QString::from("Disabled (UI test mode)"));
            return;
        }
        let cfg = RestApiServerConfig {
            enabled: *controller.rest_api_enabled(),
            bind_ip: controller.rest_api_bind_ip().to_string(),
            port: (*controller.rest_api_port()).clamp(1024, 65535) as u16,
            require_auth: *controller.rest_api_require_auth(),
            token: controller.rest_api_token().to_string(),
        };
        let qt_thread = controller.qt_thread();
        match start_or_restart_rest_api_server(cfg, qt_thread) {
            Ok(status) => {
                controller.as_mut().set_rest_api_status(QString::from(&status));
            }
            Err(err) => {
                controller.as_mut()
                    .set_rest_api_status(QString::from(&format!("Error: {err}")));
            }
        }
    }

pub fn apply_rest_api_config_impl(
        mut controller: Pin<&mut qobject::DeviceController>,
        enabled: bool,
        bind_ip: QString,
        port: i32,
        require_auth: bool,
        token: QString,
    ) -> bool {
        let clean_ip = {
            let trimmed = bind_ip.to_string().trim().to_string();
            if trimmed.is_empty() {
                "127.0.0.1".to_string()
            } else {
                trimmed
            }
        };
        let clean_port = port.clamp(1024, 65535) as u16;
        let clean_token = {
            let t = token.to_string().trim().to_string();
            if t.is_empty() {
                generate_rest_api_token()
            } else {
                t
            }
        };

        {
            let rust = controller.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.rest_api_enabled = enabled;
                sw.rest_api_bind_ip = clean_ip.clone();
                sw.rest_api_port = clean_port;
                sw.rest_api_require_auth = require_auth;
                sw.rest_api_token = clean_token.clone();
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving REST API config to disk: {e}"),
                    );
                }
            }
        }

        controller.as_mut().set_rest_api_enabled(enabled);
        controller.as_mut()
            .set_rest_api_bind_ip(QString::from(&clean_ip));
        controller.as_mut().set_rest_api_port(clean_port as i32);
        controller.as_mut().set_rest_api_require_auth(require_auth);
        controller.as_mut()
            .set_rest_api_token(QString::from(&clean_token));

        if is_ui_test_suite_env() {
            controller.as_mut()
                .set_rest_api_status(QString::from("Disabled (UI test mode)"));
            return true;
        }

        let cfg = RestApiServerConfig {
            enabled,
            bind_ip: clean_ip.clone(),
            port: clean_port,
            require_auth,
            token: clean_token,
        };
        let qt_thread = controller.qt_thread();
        match start_or_restart_rest_api_server(cfg, qt_thread) {
            Ok(status) => {
                controller.as_mut().set_rest_api_status(QString::from(&status));
                let msg = format!("✓ HTTP REST API Server: {status}");
                record_system_event("INFO", "REST_API", &msg);
                controller.as_mut().sync_event_log_properties();
                controller.as_mut().set_status_message(QString::from(&msg));
                true
            }
            Err(err) => {
                let err_status = format!("Error: {err}");
                controller.as_mut()
                    .set_rest_api_status(QString::from(&err_status));
                let msg = format!("✗ HTTP REST API Server failed: {err}");
                record_system_event("ERROR", "REST_API", &msg);
                controller.as_mut().sync_event_log_properties();
                controller.as_mut().set_status_message(QString::from(&msg));
                false
            }
        }
    }

pub fn regenerate_rest_api_token_impl(mut controller: Pin<&mut qobject::DeviceController>) -> QString {
        let new_tok = generate_rest_api_token();
        {
            let rust = controller.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.rest_api_token = new_tok.clone();
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving regenerated REST API token: {e}"),
                    );
                }
            }
        }
        controller.as_mut().set_rest_api_token(QString::from(&new_tok));
        if *controller.rest_api_enabled() {
            controller.as_mut().sync_rest_api_server_from_state();
        }
        let msg = "✓ Generated new HTTP REST API authentication token";
        record_system_event("INFO", "REST_API", msg);
        controller.as_mut().sync_event_log_properties();
        controller.as_mut().set_status_message(QString::from(msg));
        QString::from(&new_tok)
    }

pub fn reset_rest_api_defaults_impl(mut controller: Pin<&mut qobject::DeviceController>) -> QString {
        let cur_tok = controller.rest_api_token().to_string();
        let tok = if cur_tok.trim().is_empty() {
            generate_rest_api_token()
        } else {
            cur_tok
        };
        controller.as_mut().apply_rest_api_config(
            false,
            QString::from("127.0.0.1"),
            28491,
            true,
            QString::from(&tok),
        );
        let obj = serde_json::json!({
            "enabled": false,
            "bindIp": "127.0.0.1",
            "port": 28491,
            "requireAuth": true,
            "token": tok,
        });
        QString::from(&obj.to_string())
    }

pub fn handle_rest_api_request_impl(
        mut controller: Pin<&mut qobject::DeviceController>,
        method: &str,
        path: &str,
        _query: &str,
        body: &str,
    ) -> (u16, String) {
        let m = method.trim().to_ascii_uppercase();
        let clean_path = path.trim().trim_end_matches('/');
        let clean_path = if clean_path.is_empty() { "/" } else { clean_path };

        record_system_event("INFO", "REST_API", format!("{m} {clean_path}"));
        controller.as_mut().sync_event_log_properties();

        let parse_json = || -> Result<serde_json::Value, (u16, String)> {
            if body.trim().is_empty() {
                Ok(serde_json::json!({}))
            } else {
                serde_json::from_str::<serde_json::Value>(body).map_err(|e| {
                    (
                        400,
                        serde_json::json!({
                            "ok": false,
                            "error": format!("Invalid JSON body: {e}")
                        })
                        .to_string(),
                    )
                })
            }
        };

        // 1. GET /api/v1/status
        if m == "GET" && (clean_path == "/api/v1" || clean_path == "/api/v1/status") {
            let profiles_val: serde_json::Value =
                serde_json::from_str(&controller.profiles_json().to_string())
                    .unwrap_or_else(|_| serde_json::json!([]));
            let resp = serde_json::json!({
                "ok": true,
                "version": APP_VERSION,
                "connected": *controller.connected(),
                "modelName": controller.model_name().to_string(),
                "firmwareVersion": controller.firmware_version().to_string(),
                "serialNumber": controller.serial_number().to_string(),
                "activeProfile": *controller.active_profile(),
                "activeProfileName": controller.active_profile_name().to_string(),
                "activeProfileBgColor": controller.active_profile_bg_color().to_string(),
                "sdMountPath": controller.sd_mount_path().to_string(),
                "sdCardDetected": *controller.sd_card_detected(),
                "sdSpaceSummary": controller.sd_space_summary().to_string(),
                "sdSpaceUsedPercent": *controller.sd_space_used_percent(),
                "profileSyncState": controller.profile_sync_state().to_string(),
                "profileSyncDetail": controller.profile_sync_detail().to_string(),
                "autoSwitchEnabled": *controller.auto_switch_enabled(),
                "activeWindowClass": controller.active_window_class().to_string(),
                "activeWindowTitle": controller.active_window_title().to_string(),
                "statusMessage": controller.status_message().to_string(),
                "restApiStatus": current_rest_api_status(),
                "profileCount": profiles_val.as_array().map(|a| a.len()).unwrap_or(0),
            });
            return (200, resp.to_string());
        }

        // 2. GET /api/v1/profiles
        if m == "GET" && clean_path == "/api/v1/profiles" {
            let profiles_val: serde_json::Value =
                serde_json::from_str(&controller.profiles_json().to_string())
                    .unwrap_or_else(|_| serde_json::json!([]));
            return (
                200,
                serde_json::json!({
                    "ok": true,
                    "activeProfile": *controller.active_profile(),
                    "activeProfileName": controller.active_profile_name().to_string(),
                    "profiles": profiles_val,
                })
                .to_string(),
            );
        }

        // 3. POST /api/v1/profiles (create profile)
        if m == "POST" && clean_path == "/api/v1/profiles" {
            let val = match parse_json() {
                Ok(v) => v,
                Err(r) => return r,
            };
            let name = val
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("NewProfile");
            let tpl = val
                .get("templateId")
                .or_else(|| val.get("template_id"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let ok = if tpl.is_empty() {
                controller.as_mut().create_new_profile(QString::from(name))
            } else {
                controller.as_mut()
                    .create_profile_from_template(QString::from(name), QString::from(tpl))
            };
            return (
                if ok { 201 } else { 400 },
                serde_json::json!({
                    "ok": ok,
                    "activeProfile": *controller.active_profile(),
                    "activeProfileName": controller.active_profile_name().to_string(),
                    "statusMessage": controller.status_message().to_string(),
                })
                .to_string(),
            );
        }

        // 4. POST /api/v1/profiles/select
        if m == "POST" && clean_path == "/api/v1/profiles/select" {
            let val = match parse_json() {
                Ok(v) => v,
                Err(r) => return r,
            };
            let ok = if let Some(num) = val
                .get("profile")
                .or_else(|| val.get("number"))
                .and_then(|v| v.as_i64())
            {
                controller.as_mut().select_profile(num as i32)
            } else if let Some(name) = val.get("name").and_then(|v| v.as_str()) {
                controller.as_mut().switch_profile_by_name(QString::from(name))
            } else {
                return (
                    400,
                    serde_json::json!({
                        "ok": false,
                        "error": "Provide 'profile' (number 1..64) or 'name'"
                    })
                    .to_string(),
                );
            };
            return (
                if ok { 200 } else { 400 },
                serde_json::json!({
                    "ok": ok,
                    "activeProfile": *controller.active_profile(),
                    "activeProfileName": controller.active_profile_name().to_string(),
                })
                .to_string(),
            );
        }

        // 5. POST /api/v1/profiles/reorder
        if m == "POST" && clean_path == "/api/v1/profiles/reorder" {
            let val = match parse_json() {
                Ok(v) => v,
                Err(r) => return r,
            };
            let from_idx = val
                .get("fromIndex")
                .and_then(|v| v.as_i64())
                .or_else(|| {
                    val.get("fromProfile")
                        .or_else(|| val.get("from"))
                        .and_then(|v| v.as_i64())
                        .map(|n| n - 1)
                })
                .unwrap_or(0) as i32;
            let to_idx = val
                .get("toIndex")
                .and_then(|v| v.as_i64())
                .or_else(|| {
                    val.get("toProfile")
                        .or_else(|| val.get("to"))
                        .and_then(|v| v.as_i64())
                        .map(|n| n - 1)
                })
                .unwrap_or(0) as i32;
            let ok = controller.as_mut().reorder_profile(from_idx, to_idx);
            return (
                if ok { 200 } else { 400 },
                serde_json::json!({
                    "ok": ok,
                    "activeProfile": *controller.active_profile(),
                })
                .to_string(),
            );
        }

        // 6. Routes under /api/v1/profiles/<num>...
        if let Some(rest) = clean_path.strip_prefix("/api/v1/profiles/") {
            let parts: Vec<&str> = rest.split('/').collect();
            if let Some(first) = parts.first() {
                if let Ok(prof_num) = first.parse::<i32>() {
                    if parts.len() == 1 {
                        if m == "GET" {
                            let props_str = controller.get_profile_properties_json(prof_num).to_string();
                            let props_val: serde_json::Value = serde_json::from_str(&props_str)
                                .unwrap_or_else(|_| serde_json::json!({}));
                            if props_val.as_object().map(|o| o.is_empty()).unwrap_or(true) {
                                return (
                                    404,
                                    serde_json::json!({
                                        "ok": false,
                                        "error": format!("Profile #{prof_num} not found")
                                    })
                                    .to_string(),
                                );
                            }
                            let slots_val: serde_json::Value = controller
                                .rust()
                                .profiles
                                .iter()
                                .find(|p| p.number == prof_num as u8)
                                .and_then(|p| {
                                    serde_json::from_str(
                                        &DeviceControllerRust::serialize_profile_slots(p),
                                    )
                                    .ok()
                                })
                                .unwrap_or_else(|| serde_json::json!([]));
                            return (
                                200,
                                serde_json::json!({
                                    "ok": true,
                                    "profile": props_val,
                                    "slots": slots_val,
                                })
                                .to_string(),
                            );
                        } else if m == "PUT" || m == "PATCH" {
                            let val = match parse_json() {
                                Ok(v) => v,
                                Err(r) => return r,
                            };
                            let current_str =
                                controller.get_profile_properties_json(prof_num).to_string();
                            let mut merged: serde_json::Value = serde_json::from_str(&current_str)
                                .unwrap_or_else(|_| serde_json::json!({}));
                            if let (Some(dst), Some(src)) =
                                (merged.as_object_mut(), val.as_object())
                            {
                                for (k, v) in src {
                                    dst.insert(k.clone(), v.clone());
                                }
                            }
                            let ok = controller.as_mut().save_profile_properties_json(
                                prof_num,
                                QString::from(&merged.to_string()),
                            );
                            return (
                                if ok { 200 } else { 400 },
                                serde_json::json!({
                                    "ok": ok,
                                    "statusMessage": controller.status_message().to_string(),
                                })
                                .to_string(),
                            );
                        } else if m == "DELETE" {
                            let ok = controller.as_mut().delete_profile(prof_num);
                            return (
                                if ok { 200 } else { 400 },
                                serde_json::json!({
                                    "ok": ok,
                                    "statusMessage": controller.status_message().to_string(),
                                })
                                .to_string(),
                            );
                        }
                    } else if parts.len() == 2 && parts[1] == "slots" && m == "GET" {
                        let slots_val: serde_json::Value = controller
                            .rust()
                            .profiles
                            .iter()
                            .find(|p| p.number == prof_num as u8)
                            .and_then(|p| {
                                serde_json::from_str(
                                    &DeviceControllerRust::serialize_profile_slots(p),
                                )
                                .ok()
                            })
                            .unwrap_or_else(|| serde_json::json!([]));
                        return (
                            200,
                            serde_json::json!({
                                "ok": true,
                                "profile": prof_num,
                                "slots": slots_val,
                            })
                            .to_string(),
                        );
                    } else if parts.len() >= 3 && parts[1] == "slots" {
                        if let Ok(slot_num) = parts[2].parse::<i32>() {
                            controller.as_mut().select_profile(prof_num);
                            if parts.len() == 3 && m == "GET" {
                                let meta_str = controller.get_slot_meta_json(slot_num).to_string();
                                let mut detail_val: serde_json::Value =
                                    serde_json::from_str(&meta_str)
                                        .unwrap_or_else(|_| serde_json::json!({}));
                                let on_press =
                                    controller.get_slot_script(slot_num, false).to_string();
                                let on_release =
                                    controller.get_slot_script(slot_num, true).to_string();
                                if let Some(obj) = detail_val.as_object_mut() {
                                    obj.insert(
                                        "onPressScript".to_string(),
                                        serde_json::Value::String(on_press),
                                    );
                                    obj.insert(
                                        "onReleaseScript".to_string(),
                                        serde_json::Value::String(on_release),
                                    );
                                }
                                return (
                                    200,
                                    serde_json::json!({
                                        "ok": true,
                                        "profile": prof_num,
                                        "slot": slot_num,
                                        "detail": detail_val,
                                    })
                                    .to_string(),
                                );
                            } else if parts.len() == 3 && (m == "PUT" || m == "POST" || m == "PATCH")
                            {
                                let val = match parse_json() {
                                    Ok(v) => v,
                                    Err(r) => return r,
                                };
                                let cur_str = controller.get_slot_meta_json(slot_num).to_string();
                                let cur: serde_json::Value = serde_json::from_str(&cur_str)
                                    .unwrap_or_else(|_| serde_json::json!({}));
                                let cur_press =
                                    controller.get_slot_script(slot_num, false).to_string();
                                let cur_release =
                                    controller.get_slot_script(slot_num, true).to_string();

                                let l1 = val
                                    .get("label1")
                                    .or_else(|| val.get("l1"))
                                    .and_then(|v| v.as_str())
                                    .or_else(|| cur.get("l1").and_then(|v| v.as_str()))
                                    .unwrap_or("")
                                    .to_string();
                                let l2 = val
                                    .get("label2")
                                    .or_else(|| val.get("l2"))
                                    .and_then(|v| v.as_str())
                                    .or_else(|| cur.get("l2").and_then(|v| v.as_str()))
                                    .unwrap_or("")
                                    .to_string();
                                let wl1 = val
                                    .get("widgetLabel1")
                                    .or_else(|| val.get("wl1"))
                                    .and_then(|v| v.as_str())
                                    .or_else(|| cur.get("wl1").and_then(|v| v.as_str()))
                                    .unwrap_or("")
                                    .to_string();
                                let wl2 = val
                                    .get("widgetLabel2")
                                    .or_else(|| val.get("wl2"))
                                    .and_then(|v| v.as_str())
                                    .or_else(|| cur.get("wl2").and_then(|v| v.as_str()))
                                    .unwrap_or("")
                                    .to_string();
                                let allow_abort = val
                                    .get("allowAbort")
                                    .and_then(|v| v.as_bool())
                                    .or_else(|| cur.get("allowAbort").and_then(|v| v.as_bool()))
                                    .unwrap_or(false);
                                let disable_autorepeat = val
                                    .get("disableAutorepeat")
                                    .and_then(|v| v.as_bool())
                                    .or_else(|| {
                                        cur.get("disableAutorepeat").and_then(|v| v.as_bool())
                                    })
                                    .unwrap_or(false);
                                let mut on_press = val
                                    .get("onPressScript")
                                    .or_else(|| val.get("onPress"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or(&cur_press)
                                    .to_string();
                                let on_release = val
                                    .get("onReleaseScript")
                                    .or_else(|| val.get("onRelease"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or(&cur_release)
                                    .to_string();

                                if let Some(web_cfg) = val.get("webApi") {
                                    on_press = controller
                                        .apply_web_api_to_script(
                                            QString::from(&on_press),
                                            QString::from(&web_cfg.to_string()),
                                            slot_num,
                                            false,
                                        )
                                        .to_string();
                                }
                                if let Some(app_cfg) = val.get("appLaunch") {
                                    on_press = controller
                                        .apply_app_launch_to_script(
                                            QString::from(&on_press),
                                            QString::from(&app_cfg.to_string()),
                                            slot_num,
                                            false,
                                        )
                                        .to_string();
                                }

                                let ok1 = controller.as_mut().save_slot_config(
                                    slot_num,
                                    QString::from(&l1),
                                    QString::from(&l2),
                                    QString::from(&on_press),
                                    QString::from(&on_release),
                                    allow_abort,
                                    disable_autorepeat,
                                );
                                controller.as_mut().save_slot_widget_labels(
                                    slot_num,
                                    QString::from(&wl1),
                                    QString::from(&wl2),
                                );

                                if let Some(has_rgb) =
                                    val.get("hasCustomRgb").and_then(|v| v.as_bool())
                                {
                                    if has_rgb {
                                        let r = val
                                            .get("r")
                                            .and_then(|v| v.as_i64())
                                            .unwrap_or(0)
                                            as i32;
                                        let g = val
                                            .get("g")
                                            .and_then(|v| v.as_i64())
                                            .unwrap_or(170)
                                            as i32;
                                        let b = val
                                            .get("b")
                                            .and_then(|v| v.as_i64())
                                            .unwrap_or(255)
                                            as i32;
                                        controller.as_mut().set_key_rgb(slot_num, r, g, b);
                                    } else {
                                        controller.as_mut().clear_key_custom_rgb(slot_num);
                                    }
                                } else if val.get("r").is_some()
                                    || val.get("g").is_some()
                                    || val.get("b").is_some()
                                {
                                    let r =
                                        val.get("r").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                                    let g = val
                                        .get("g")
                                        .and_then(|v| v.as_i64())
                                        .unwrap_or(170)
                                        as i32;
                                    let b = val
                                        .get("b")
                                        .and_then(|v| v.as_i64())
                                        .unwrap_or(255)
                                        as i32;
                                    controller.as_mut().set_key_rgb(slot_num, r, g, b);
                                }

                                return (
                                    if ok1 { 200 } else { 400 },
                                    serde_json::json!({
                                        "ok": ok1,
                                        "profile": prof_num,
                                        "slot": slot_num,
                                        "statusMessage": controller.status_message().to_string(),
                                    })
                                    .to_string(),
                                );
                            } else if parts.len() == 3 && m == "DELETE" {
                                let ok = controller.as_mut().clear_slot(slot_num);
                                return (
                                    if ok { 200 } else { 400 },
                                    serde_json::json!({
                                        "ok": ok,
                                        "profile": prof_num,
                                        "slot": slot_num,
                                    })
                                    .to_string(),
                                );
                            } else if parts.len() == 4
                                && parts[3] == "trigger"
                                && m == "POST"
                            {
                                let val = match parse_json() {
                                    Ok(v) => v,
                                    Err(r) => return r,
                                };
                                let is_rel = val
                                    .get("release")
                                    .and_then(|v| v.as_bool())
                                    .unwrap_or(false);
                                controller.as_mut().handle_hardware_pgv_web_api_trigger(
                                    prof_num.clamp(1, 64) as u8,
                                    slot_num.clamp(1, MAX_INPUT_SLOTS as i32) as usize,
                                    is_rel,
                                );
                                return (
                                    200,
                                    serde_json::json!({
                                        "ok": true,
                                        "profile": prof_num,
                                        "slot": slot_num,
                                        "release": is_rel,
                                        "statusMessage": controller.status_message().to_string(),
                                    })
                                    .to_string(),
                                );
                            }
                        }
                    }
                }
            }
        }

        // 7. POST /api/v1/dsvm/compile
        if m == "POST" && clean_path == "/api/v1/dsvm/compile" {
            let val = match parse_json() {
                Ok(v) => v,
                Err(r) => return r,
            };
            let script = val
                .get("script")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let allow_abort = val
                .get("allowAbort")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let disable_autorepeat = val
                .get("disableAutorepeat")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let res_str = controller
                .check_script_syntax(
                    QString::from(script),
                    allow_abort,
                    disable_autorepeat,
                )
                .to_string();
            return (200, res_str);
        }

        // 8. GET /api/v1/templates
        if m == "GET" && clean_path == "/api/v1/templates" {
            let tpl_str = controller.list_profile_templates_json().to_string();
            let tpl_val: serde_json::Value =
                serde_json::from_str(&tpl_str).unwrap_or_else(|_| serde_json::json!([]));
            return (
                200,
                serde_json::json!({
                    "ok": true,
                    "templates": tpl_val,
                })
                .to_string(),
            );
        }

        // 9. GET / PUT /api/v1/pgv
        if clean_path == "/api/v1/pgv" {
            if m == "GET" {
                let pgv_str = controller.as_mut().dump_all_pgvs_json().to_string();
                let pgv_val: serde_json::Value =
                    serde_json::from_str(&pgv_str).unwrap_or_else(|_| serde_json::json!([]));
                return (
                    200,
                    serde_json::json!({
                        "ok": true,
                        "variables": pgv_val,
                    })
                    .to_string(),
                );
            } else if m == "PUT" || m == "POST" {
                let val = match parse_json() {
                    Ok(v) => v,
                    Err(r) => return r,
                };
                let index = val.get("index").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let value = val.get("value").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                let ok = controller.as_mut().write_pgv(index, value);
                return (
                    if ok { 200 } else { 400 },
                    serde_json::json!({
                        "ok": ok,
                        "index": index,
                        "value": value,
                    })
                    .to_string(),
                );
            }
        }

        // 10. POST /api/v1/leds
        if m == "POST" && clean_path == "/api/v1/leds" {
            let val = match parse_json() {
                Ok(v) => v,
                Err(r) => return r,
            };
            if let Some(arr) = val.get("leds").or_else(|| val.as_array().map(|_| &val)) {
                let ok = controller.push_hardware_led_frame_json(QString::from(&arr.to_string()));
                return (
                    if ok { 200 } else { 400 },
                    serde_json::json!({ "ok": ok }).to_string(),
                );
            }
            let slot = val.get("slot").and_then(|v| v.as_i64()).unwrap_or(1).clamp(1, 20) as usize;
            let r = val.get("r").and_then(|v| v.as_i64()).unwrap_or(0).clamp(0, 255) as u8;
            let g = val.get("g").and_then(|v| v.as_i64()).unwrap_or(170).clamp(0, 255) as u8;
            let b = val.get("b").and_then(|v| v.as_i64()).unwrap_or(255).clamp(0, 255) as u8;
            let ok = set_single_key_led_hid(slot, [r, g, b]);
            return (
                if ok { 200 } else { 400 },
                serde_json::json!({
                    "ok": ok,
                    "slot": slot,
                    "r": r,
                    "g": g,
                    "b": b,
                })
                .to_string(),
            );
        }

        // 11. Storage & Sync endpoints
        if m == "POST" && clean_path == "/api/v1/sd/sync" {
            let active_num = (*controller.active_profile()).clamp(1, 64) as u8;
            let serial_str = controller.serial_number().to_string();
            let profiles_snapshot = controller.rust().profiles.clone();
            let requested_sd_root = serde_json::from_str::<serde_json::Value>(body)
                .ok()
                .and_then(|v| {
                    v.get("sdRootPath")
                        .and_then(|s| s.as_str())
                        .map(|s| s.trim().to_string())
                })
                .filter(|s| !s.is_empty())
                .map(PathBuf::from);
            let mounted_sd = requested_sd_root.or_else(|| {
                controller.rust()
                    .sd_root_path
                    .clone()
                    .filter(|p| p.exists())
                    .or_else(find_mounted_duckypad_sd)
            });

            controller.as_mut().set_sd_sync_busy(true);
            controller.as_mut().set_sd_sync_progress(5);
            controller.as_mut()
                .set_sd_sync_result_state(QString::from("running"));
            controller.as_mut().set_sd_sync_result_summary(QString::from(""));
            controller.as_mut()
                .set_sd_sync_step(QString::from("Starting SD card sync..."));

            let job = crate::api_server::create_api_job("sd/sync");
            let job_id = job.id.clone();
            let qt_thread = controller.qt_thread();

            std::thread::spawn(move || {
                if let Some(sd_root) = mounted_sd {
                    match write_profiles_to_sd_root(&sd_root, &profiles_snapshot) {
                        Ok(summary) => {
                            sync_profiles_to_device_cache(&serial_str, &profiles_snapshot);
                            mark_profiles_sd_synced(&serial_str, &profiles_snapshot);
                            let sd_display = sd_root.display().to_string();
                            let short_fp = &summary.validated_fingerprint
                                [..8.min(summary.validated_fingerprint.len())];
                            let msg = format!(
                                "Validated & Saved! {} profile(s) verified on {sd_display} ({} file(s) updated, read-back hash #{short_fp} confirmed).",
                                summary.profiles_count, summary.files_written
                            );
                            let sd_clone = sd_display.clone();
                            let msg_clone = msg.clone();
                            let _ = qt_thread.queue(move |mut qobject| {
                                qobject.as_mut().set_sd_sync_busy(false);
                                qobject.as_mut().set_sd_sync_progress(100);
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_state(QString::from("success"));
                                qobject
                                    .as_mut()
                                    .set_sd_sync_step(QString::from(&msg_clone));
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_summary(QString::from(&msg_clone));
                                qobject.as_mut().load_sd_directory(QString::from(&sd_clone));
                                qobject
                                    .as_mut()
                                    .set_status_message(QString::from(&msg_clone));
                            });
                            crate::api_server::update_api_job(
                                &job_id,
                                "completed",
                                Some(true),
                                &msg,
                                Some(serde_json::json!({
                                    "syncedProfiles": summary.profiles_count,
                                    "filesWritten": summary.files_written,
                                    "sdRootPath": sd_display,
                                })),
                            );
                        }
                        Err(err) => {
                            let msg = format!("SD Sync Error: {err}");
                            let msg_clone = msg.clone();
                            let _ = qt_thread.queue(move |mut qobject| {
                                qobject.as_mut().set_sd_sync_busy(false);
                                qobject.as_mut().set_sd_sync_progress(0);
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_state(QString::from("error"));
                                qobject
                                    .as_mut()
                                    .set_sd_sync_step(QString::from(&msg_clone));
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_summary(QString::from(&msg_clone));
                                qobject
                                    .as_mut()
                                    .set_status_message(QString::from(&msg_clone));
                            });
                            crate::api_server::update_api_job(
                                &job_id,
                                "failed",
                                Some(false),
                                &msg,
                                None,
                            );
                        }
                    }
                    return;
                }

                let Ok(info) = query_connected_duckypad_info() else {
                    let msg = "Cannot sync: duckyPad not connected and no mounted SD card found."
                        .to_string();
                    let msg_clone = msg.clone();
                    let _ = qt_thread.queue(move |mut qobject| {
                        qobject.as_mut().set_sd_sync_busy(false);
                        qobject.as_mut().set_sd_sync_progress(0);
                        qobject
                            .as_mut()
                            .set_sd_sync_result_state(QString::from("error"));
                        qobject.as_mut().set_sd_sync_step(QString::from(""));
                        qobject
                            .as_mut()
                            .set_status_message(QString::from(&msg_clone));
                    });
                    crate::api_server::update_api_job(&job_id, "failed", Some(false), &msg, None);
                    return;
                };

                if info.pid == PID_DUCKYPAD_2020 {
                    let msg = "duckyPad (2020): Insert microSD into computer to sync profiles."
                        .to_string();
                    let msg_clone = msg.clone();
                    let _ = qt_thread.queue(move |mut qobject| {
                        qobject.as_mut().set_sd_sync_busy(false);
                        qobject.as_mut().set_sd_sync_progress(0);
                        qobject
                            .as_mut()
                            .set_sd_sync_result_state(QString::from("error"));
                        qobject.as_mut().set_sd_sync_step(QString::from(""));
                        qobject
                            .as_mut()
                            .set_status_message(QString::from(&msg_clone));
                    });
                    crate::api_server::update_api_job(&job_id, "failed", Some(false), &msg, None);
                    return;
                }

                let disk_label = info.disk_label.clone();
                let dev_serial = if !info.serial.is_empty() && info.serial != "-" {
                    info.serial.clone()
                } else {
                    serial_str
                };
                let label_display = if disk_label.is_empty() {
                    "duckyPad Pro".to_string()
                } else {
                    disk_label.clone()
                };
                let qt_progress = qt_thread.clone();

                let res = write_duckypad_pro_profiles_via_msc_with_progress(
                    &disk_label,
                    &dev_serial,
                    &profiles_snapshot,
                    active_num,
                    move |pct, step, _log_line| {
                        let step_str = step.to_string();
                        let _ = qt_progress.queue(move |mut qobject| {
                            qobject.as_mut().set_sd_sync_progress(pct);
                            qobject.as_mut().set_sd_sync_step(QString::from(&step_str));
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&step_str));
                        });
                    },
                );

                match res {
                    Ok(summary) => {
                        let short_fp = &summary.validated_fingerprint
                            [..8.min(summary.validated_fingerprint.len())];
                        let msg = format!(
                            "Validated & Saved! {} profile(s) verified on duckyPad Pro ({label_display}) — {} file(s) updated, read-back hash #{short_fp} confirmed.",
                            summary.profiles_count, summary.files_written
                        );
                        let msg_clone = msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject.as_mut().set_sd_sync_busy(false);
                            qobject.as_mut().set_sd_sync_progress(100);
                            qobject
                                .as_mut()
                                .set_sd_sync_result_state(QString::from("success"));
                            qobject
                                .as_mut()
                                .set_sd_sync_step(QString::from(&msg_clone));
                            qobject
                                .as_mut()
                                .set_sd_sync_result_summary(QString::from(&msg_clone));
                            qobject.as_mut().refresh_device();
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&msg_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "completed",
                            Some(true),
                            &msg,
                            Some(serde_json::json!({
                                "syncedProfiles": summary.profiles_count,
                                "filesWritten": summary.files_written,
                                "summary": msg,
                            })),
                        );
                    }
                    Err(err) => {
                        let err_msg = format!("SD Sync Error: {err}");
                        let err_clone = err_msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject.as_mut().set_sd_sync_busy(false);
                            qobject.as_mut().set_sd_sync_progress(0);
                            qobject
                                .as_mut()
                                .set_sd_sync_result_state(QString::from("error"));
                            qobject
                                .as_mut()
                                .set_sd_sync_step(QString::from(&err_clone));
                            qobject
                                .as_mut()
                                .set_sd_sync_result_summary(QString::from(&err_clone));
                            qobject.as_mut().refresh_device();
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&err_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "failed",
                            Some(false),
                            &err_msg,
                            None,
                        );
                    }
                }
            });

            return crate::api_server::format_job_accepted_response(&job);
        }
        if m == "POST" && clean_path == "/api/v1/sd/reload" {
            let ok = controller.as_mut().reload_profiles_from_device_sd();
            return (
                if ok { 200 } else { 400 },
                serde_json::json!({
                    "ok": ok,
                    "statusMessage": controller.status_message().to_string(),
                })
                .to_string(),
            );
        }
        if m == "POST" && clean_path == "/api/v1/sd/mount" {
            let dev_serial = controller.serial_number().to_string();
            let job = crate::api_server::create_api_job("sd/mount");
            let job_id = job.id.clone();
            let qt_thread = controller.qt_thread();

            std::thread::spawn(move || {
                let serial = query_connected_duckypad_info()
                    .ok()
                    .map(|info| info.serial)
                    .filter(|s| !s.is_empty() && s != "-")
                    .unwrap_or(dev_serial);
                let disk_label = if serial.len() >= 4 && serial != "-" {
                    format!("DP24_{}", serial[serial.len() - 4..].to_ascii_uppercase())
                } else {
                    String::new()
                };

                let mount_res = find_mounted_duckypad_sd()
                    .map(Ok)
                    .unwrap_or_else(|| mount_duckypad_sd_only(&disk_label));

                match mount_res {
                    Ok(mount_path) => {
                        let mount_str = mount_path.to_string_lossy().to_string();
                        let msg = format!("✓ Mounted duckyPad microSD card at {mount_str}");
                        let mp_clone = mount_str.clone();
                        let msg_clone = msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject.as_mut().set_sd_card_detected(true);
                            qobject.as_mut().load_sd_directory(QString::from(&mp_clone));
                            qobject.as_mut().refresh_device();
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&msg_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "completed",
                            Some(true),
                            &msg,
                            Some(serde_json::json!({ "sdMountPath": mount_str })),
                        );
                    }
                    Err(err) => {
                        let msg = format!("Mount SD Error: {err}");
                        let msg_clone = msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&msg_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "failed",
                            Some(false),
                            &msg,
                            None,
                        );
                    }
                }
            });

            return crate::api_server::format_job_accepted_response(&job);
        }
        if m == "POST" && clean_path == "/api/v1/sd/unmount" {
            let current_mount = controller.rust().sd_root_path.clone();
            let job = crate::api_server::create_api_job("sd/unmount");
            let job_id = job.id.clone();
            let qt_thread = controller.qt_thread();

            std::thread::spawn(move || {
                match unmount_mounted_duckypad_sd(current_mount.as_deref()) {
                    Ok(msg) => {
                        let full_msg = format!("✓ {msg}");
                        let msg_clone = full_msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject.as_mut().rust_mut().get_mut().sd_root_path = None;
                            qobject.as_mut().set_sd_mount_path(QString::from(""));
                            qobject.as_mut().refresh_sd_mount_status();
                            qobject.as_mut().sync_active_profile_state();
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&msg_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "completed",
                            Some(true),
                            &full_msg,
                            None,
                        );
                    }
                    Err(err) => {
                        let msg = format!("Unmount SD Error: {err}");
                        let msg_clone = msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&msg_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "failed",
                            Some(false),
                            &msg,
                            None,
                        );
                    }
                }
            });

            return crate::api_server::format_job_accepted_response(&job);
        }

        // 12. Device power/system control endpoints
        if m == "POST" && clean_path == "/api/v1/device/sleep" {
            let ok = controller.as_mut().set_sleep_state(true);
            return (
                if ok { 200 } else { 400 },
                serde_json::json!({ "ok": ok }).to_string(),
            );
        }
        if m == "POST" && clean_path == "/api/v1/device/wake" {
            let ok = controller.as_mut().set_sleep_state(false);
            return (
                if ok { 200 } else { 400 },
                serde_json::json!({ "ok": ok }).to_string(),
            );
        }
        if m == "POST" && clean_path == "/api/v1/device/reboot" {
            let job = crate::api_server::create_api_job("device/reboot");
            let job_id = job.id.clone();
            let qt_thread = controller.qt_thread();

            std::thread::spawn(move || {
                match reboot_duckypad() {
                    Ok(()) => {
                        let msg = "Rebooting duckyPad into normal operating mode...".to_string();
                        let msg_clone = msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&msg_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "completed",
                            Some(true),
                            &msg,
                            None,
                        );
                        std::thread::sleep(std::time::Duration::from_millis(1600));
                        let _ = qt_thread.queue(|mut qobject| {
                            qobject.as_mut().refresh_device();
                        });
                    }
                    Err(err) => {
                        let msg = format!("Reboot Error: {err}");
                        let msg_clone = msg.clone();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&msg_clone));
                        });
                        crate::api_server::update_api_job(
                            &job_id,
                            "failed",
                            Some(false),
                            &msg,
                            None,
                        );
                    }
                }
            });

            return crate::api_server::format_job_accepted_response(&job);
        }
        if m == "POST" && clean_path == "/api/v1/device/sync-rtc" {
            let ok = controller.as_mut().sync_rtc_clock();
            return (
                if ok { 200 } else { 400 },
                serde_json::json!({ "ok": ok }).to_string(),
            );
        }

        // 13. GET /api/v1/jobs/{id}
        if m == "GET" {
            if let Some(job_id) = clean_path.strip_prefix("/api/v1/jobs/") {
                return crate::api_server::handle_get_job_status(job_id.trim());
            }
        }

        // 14. GET /api/v1/logs
        if m == "GET" && clean_path == "/api/v1/logs" {
            let log_str = controller.event_log_json().to_string();
            let log_val: serde_json::Value =
                serde_json::from_str(&log_str).unwrap_or_else(|_| serde_json::json!([]));
            return (
                200,
                serde_json::json!({
                    "ok": true,
                    "count": *controller.event_log_count(),
                    "events": log_val,
                })
                .to_string(),
            );
        }

        (
            404,
            serde_json::json!({
                "ok": false,
                "error": format!("Unknown endpoint: {m} {clean_path}"),
            })
            .to_string(),
        )
    }
