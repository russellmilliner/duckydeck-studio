use crate::bridge::device_controller::{is_ui_test_suite_env, qobject, record_system_event};
use crate::hardware::storage::{
    check_latest_duckypad_firmware, download_official_firmware_binary,
    fetch_and_search_home_assistant_entities, fetch_official_firmwares_from_site,
    install_duckypad_2020_firmware_dfu_with_progress, list_available_duckypad_2020_firmwares,
    list_available_duckypad_pro_firmwares, load_home_assistant_config, save_home_assistant_config,
    stage_duckypad_pro_firmware_via_msc_with_progress,
};
use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use std::path::PathBuf;

impl qobject::DeviceController {
    pub fn set_check_updates_on_startup_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.check_updates_on_startup = enabled;
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving check_updates_on_startup setting: {e}"),
                    );
                }
            }
        }
        self.as_mut().set_check_updates_on_startup(enabled);
    }

    pub fn check_app_updates(mut self: Pin<&mut Self>, manual: bool) {
        if is_ui_test_suite_env() || *self.app_update_check_busy() {
            return;
        }
        let curr_ver = self.app_version().to_string();
        self.as_mut().set_app_update_check_busy(true);
        if manual {
            self.as_mut().set_app_update_status(QString::from(
                "Checking GitHub Releases for DuckyDeck Studio updates...",
            ));
            self.as_mut()
                .set_status_message(QString::from("Checking for DuckyDeck Studio updates..."));
        }

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let result = crate::updater::check_for_app_updates_blocking(&curr_ver);
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_app_update_check_busy(false);
                match result {
                    Ok(res) => {
                        qobject
                            .as_mut()
                            .set_latest_app_version(QString::from(&res.latest_version));
                        qobject
                            .as_mut()
                            .set_app_update_available(res.update_available);
                        qobject
                            .as_mut()
                            .set_app_update_url(QString::from(&res.release_url));
                        qobject
                            .as_mut()
                            .set_app_update_download_url(QString::from(&res.download_url));

                        if res.update_available {
                            let summary = format!(
                                "DuckyDeck Studio v{} is available! (Installed: v{})",
                                res.latest_version, res.current_version
                            );
                            qobject
                                .as_mut()
                                .set_app_update_status(QString::from(&summary));
                            qobject.as_mut().set_status_message(QString::from(&summary));

                            let already_notified = qobject.rust().notified_app_update_version
                                == res.latest_version;
                            if !already_notified {
                                qobject
                                    .as_mut()
                                    .rust_mut()
                                    .get_mut()
                                    .notified_app_update_version = res.latest_version.clone();
                                let body_str = format!(
                                    "Version {} is available on GitHub (installed: v{}). Open DuckyDeck Studio to install.",
                                    res.latest_version, res.current_version
                                );
                                crate::kwin::auto_switcher::send_kde_notification(
                                    "updateAvailable",
                                    "DuckyDeck Studio Update Available",
                                    &body_str,
                                    0,
                                );
                            }
                        } else {
                            let summary = format!(
                                "DuckyDeck Studio v{} is up to date! (Latest release: v{})",
                                res.current_version, res.latest_version
                            );
                            qobject
                                .as_mut()
                                .set_app_update_status(QString::from(&summary));
                            if manual {
                                qobject.as_mut().set_status_message(QString::from(&summary));
                            }
                        }
                    }
                    Err(err) => {
                        let msg = format!("App update check failed: {err}");
                        qobject.as_mut().set_app_update_status(QString::from(&msg));
                        if manual {
                            qobject.as_mut().set_status_message(QString::from(&msg));
                        }
                    }
                }
            });
        });
    }

    pub fn install_app_update(mut self: Pin<&mut Self>) {
        if is_ui_test_suite_env() || *self.app_update_install_busy() {
            return;
        }
        let dl_url = self.app_update_download_url().to_string();
        let latest_ver = self.latest_app_version().to_string();
        if dl_url.trim().is_empty() {
            self.open_app_update_page();
            return;
        }

        self.as_mut().set_app_update_install_busy(true);
        let msg = format!("Downloading and installing DuckyDeck Studio v{latest_ver}...");
        self.as_mut().set_app_update_status(QString::from(&msg));
        self.as_mut().set_status_message(QString::from(&msg));

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let result =
                crate::updater::download_and_install_app_update_blocking(&dl_url, &latest_ver);
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_app_update_install_busy(false);
                match result {
                    Ok(done_msg) => {
                        let full = format!("✓ {done_msg}");
                        qobject.as_mut().set_app_update_status(QString::from(&full));
                        qobject.as_mut().set_status_message(QString::from(&full));
                    }
                    Err(err) => {
                        let full = format!("⚠ Update install failed: {err}");
                        qobject.as_mut().set_app_update_status(QString::from(&full));
                        qobject.as_mut().set_status_message(QString::from(&full));
                    }
                }
            });
        });
    }

    pub fn open_app_update_page(&self) {
        let url = self.app_update_url().to_string();
        let target = if url.trim().is_empty() {
            crate::updater::GITHUB_RELEASES_HTML_URL
        } else {
            url.trim()
        };
        if let Err(e) = crate::paths::open_url_or_path_in_os(target) {
            record_system_event(
                "WARN",
                "SYSTEM",
                format!("Failed opening app update page {target}: {e}"),
            );
        }
    }

    pub fn check_firmware_update(mut self: Pin<&mut Self>, manual: bool) {
        if is_ui_test_suite_env() || *self.firmware_check_busy() {
            return;
        }
        let model_str = self.model_name().to_string();
        let is_pro = !model_str.contains("2020");
        let curr_fw = self.firmware_version().to_string();

        self.as_mut().set_firmware_check_busy(true);
        if manual {
            self.as_mut().set_firmware_check_summary(QString::from(
                "Checking GitHub for official duckyPad firmware updates...",
            ));
            self.as_mut()
                .set_status_message(QString::from("Checking for firmware updates..."));
        }

        let qt_thread = self.qt_thread();
        crate::actions::web_api::spawn_shared_async(async move {
            let result = check_latest_duckypad_firmware(is_pro, &curr_fw).await;
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_firmware_check_busy(false);
                match result {
                    Ok(res) => {
                        qobject
                            .as_mut()
                            .set_latest_firmware_version(QString::from(&res.latest_version));
                        qobject
                            .as_mut()
                            .set_firmware_update_available(res.update_available);
                        qobject
                            .as_mut()
                            .set_firmware_update_url(QString::from(&res.guide_url));
                        qobject.as_mut().rust_mut().get_mut().firmware_download_url =
                            res.download_url.clone();

                        let dev_label = if res.is_pro {
                            "duckyPad Pro"
                        } else {
                            "duckyPad (2020)"
                        };
                        let summary = if res.current_version == "-" || res.current_version.is_empty()
                        {
                            format!(
                                "Latest official {dev_label} firmware is v{}. Connect your duckyPad over USB HID to compare against your installed firmware.",
                                res.latest_version
                            )
                        } else if res.update_available {
                            format!(
                                "A newer {dev_label} firmware update is available! Installed: v{}  →  Latest: v{}",
                                res.current_version, res.latest_version
                            )
                        } else {
                            format!(
                                "Your {dev_label} firmware (v{}) is up to date! (Latest official release: v{})",
                                res.current_version, res.latest_version
                            )
                        };

                        qobject
                            .as_mut()
                            .set_firmware_check_summary(QString::from(&summary));
                        if manual || res.update_available {
                            qobject.as_mut().set_status_message(QString::from(&summary));
                        }
                    }
                    Err(err) => {
                        let msg = format!("Firmware update check failed: {err}");
                        qobject
                            .as_mut()
                            .set_firmware_check_summary(QString::from(&msg));
                        if manual {
                            qobject.as_mut().set_status_message(QString::from(&msg));
                        }
                    }
                }
            });
        });
    }

    pub fn open_firmware_update_page(self: Pin<&mut Self>) {
        let url = self.firmware_update_url().to_string();
        if !url.is_empty() {
            if let Err(e) = crate::paths::open_url_or_path_in_os(&url) {
                record_system_event(
                    "WARN",
                    "SYSTEM",
                    format!("Failed opening firmware update page {url}: {e}"),
                );
            }
        }
    }

    pub fn download_firmware_binary(self: Pin<&mut Self>) {
        let dl = self.rust().firmware_download_url.clone();
        let fallback = self.firmware_update_url().to_string();
        let target = if !dl.is_empty() { dl } else { fallback };
        if !target.is_empty() {
            if let Err(e) = crate::paths::open_url_or_path_in_os(&target) {
                record_system_event(
                    "WARN",
                    "SYSTEM",
                    format!("Failed opening firmware download URL {target}: {e}"),
                );
            }
        }
    }

    pub fn list_available_firmwares_json(&self) -> QString {
        let curr = self.firmware_version().to_string();
        let list = if *self.is_duckypad_2020() {
            list_available_duckypad_2020_firmwares(&curr)
        } else {
            list_available_duckypad_pro_firmwares(&curr)
        };
        let json_str = serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
        QString::from(&json_str)
    }

    pub fn install_firmware_to_device(mut self: Pin<&mut Self>, firmware_path: QString) -> bool {
        if *self.sd_sync_busy() {
            return false;
        }
        let raw_path = firmware_path.to_string();
        let clean_path = crate::paths::clean_file_uri_to_path_str(&raw_path);
        if clean_path.is_empty() {
            let msg = "No firmware binary selected";
            self.as_mut().set_status_message(QString::from(msg));
            return false;
        }

        let is_dp20 = *self.is_duckypad_2020() || clean_path.to_ascii_lowercase().ends_with(".dfu");
        let dev_serial = self.serial_number().to_string();
        let disk_label = if dev_serial.len() >= 4 && dev_serial != "-" {
            format!(
                "{}_{}",
                if is_dp20 { "DP20" } else { "DP24" },
                &dev_serial[dev_serial.len() - 4..]
            )
        } else {
            String::new()
        };
        let label_display = if disk_label.is_empty() {
            if is_dp20 { "DP20" } else { "DP24" }.to_string()
        } else {
            disk_label.clone()
        };
        let dev_model_name = if is_dp20 {
            "duckyPad (2020)"
        } else {
            "duckyPad Pro"
        };

        self.as_mut().set_sd_sync_busy(true);
        self.as_mut().set_sd_sync_progress(5);
        self.as_mut()
            .set_sd_sync_result_state(QString::from("running"));
        self.as_mut().set_sd_sync_result_summary(QString::from(""));
        self.as_mut().set_sd_sync_step(QString::from(&format!(
            "Preparing {dev_model_name} firmware installation..."
        )));
        self.as_mut().set_sd_sync_log(QString::from(&format!(
            "• Target device: {dev_model_name} ({label_display})\n• Firmware image: {clean_path}"
        )));
        self.as_mut().set_status_message(QString::from(&format!(
            "Installing firmware update to {dev_model_name}..."
        )));

        let qt_thread = self.qt_thread();
        let qt_thread_done = self.qt_thread();
        let fw_path_buf = PathBuf::from(&clean_path);

        std::thread::spawn(move || {
            let res = if is_dp20 {
                install_duckypad_2020_firmware_dfu_with_progress(
                    &fw_path_buf,
                    |pct, step, log_line| {
                        let step_s = step.to_string();
                        let log_s = log_line.to_string();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject.as_mut().set_sd_sync_progress(pct);
                            qobject.as_mut().set_sd_sync_step(QString::from(&step_s));
                            if !log_s.is_empty() {
                                let prev = qobject.sd_sync_log().to_string();
                                let next = if prev.is_empty() {
                                    log_s.clone()
                                } else {
                                    format!("{prev}\n{log_s}")
                                };
                                qobject.as_mut().set_sd_sync_log(QString::from(&next));
                            }
                            qobject.as_mut().set_status_message(QString::from(&step_s));
                        });
                    },
                )
            } else {
                stage_duckypad_pro_firmware_via_msc_with_progress(
                    &disk_label,
                    &fw_path_buf,
                    |pct, step, log_line| {
                        let step_s = step.to_string();
                        let log_s = log_line.to_string();
                        let _ = qt_thread.queue(move |mut qobject| {
                            qobject.as_mut().set_sd_sync_progress(pct);
                            qobject.as_mut().set_sd_sync_step(QString::from(&step_s));
                            if !log_s.is_empty() {
                                let prev = qobject.sd_sync_log().to_string();
                                let next = if prev.is_empty() {
                                    log_s.clone()
                                } else {
                                    format!("{prev}\n{log_s}")
                                };
                                qobject.as_mut().set_sd_sync_log(QString::from(&next));
                            }
                            qobject.as_mut().set_status_message(QString::from(&step_s));
                        });
                    },
                )
            };

            let _ = qt_thread_done.queue(move |mut qobject| {
                qobject.as_mut().set_sd_sync_busy(false);
                match res {
                    Ok(summary_msg) => {
                        record_system_event("INFO", "FIRMWARE", &summary_msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_sd_sync_progress(100);
                        qobject
                            .as_mut()
                            .set_sd_sync_result_state(QString::from("success"));
                        qobject
                            .as_mut()
                            .set_sd_sync_step(QString::from(&summary_msg));
                        qobject
                            .as_mut()
                            .set_sd_sync_result_summary(QString::from(&summary_msg));
                        qobject
                            .as_mut()
                            .set_status_message(QString::from(&summary_msg));
                        crate::kwin::auto_switcher::send_kde_notification(
                            "sdSyncComplete",
                            "Firmware Staged to microSD",
                            &summary_msg,
                            0,
                        );
                    }
                    Err(err) => {
                        let prev = qobject.sd_sync_log().to_string();
                        let next = format!("{prev}\n✗ Error: {err}");
                        let err_msg = format!("Firmware installation failed: {err}");
                        record_system_event("ERROR", "FIRMWARE", &err_msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject
                            .as_mut()
                            .set_sd_sync_result_state(QString::from("error"));
                        qobject
                            .as_mut()
                            .set_sd_sync_step(QString::from("Firmware installation failed"));
                        qobject.as_mut().set_sd_sync_log(QString::from(&next));
                        qobject
                            .as_mut()
                            .set_sd_sync_result_summary(QString::from(&err_msg));
                        qobject.as_mut().set_status_message(QString::from(&err_msg));
                        crate::kwin::auto_switcher::send_kde_notification(
                            "sdSyncError",
                            "Firmware Staging Failed",
                            &err_msg,
                            2,
                        );
                    }
                }
            });
        });

        true
    }

    pub fn get_home_assistant_config_json(&self) -> QString {
        let cfg = load_home_assistant_config();
        let obj = serde_json::json!({
            "base_url": cfg.base_url,
            "baseUrl": cfg.base_url,
            "access_token": cfg.access_token,
            "accessToken": cfg.access_token,
        });
        QString::from(&obj.to_string())
    }

    pub fn save_home_assistant_config_qinvokable(
        mut self: Pin<&mut Self>,
        base_url: QString,
        access_token: QString,
    ) -> bool {
        let b_str = base_url.to_string().trim().to_string();
        let t_str = access_token.to_string().trim().to_string();
        match save_home_assistant_config(&b_str, &t_str) {
            Ok(saved) => {
                let msg = format!("✓ Saved Home Assistant connection ({})", saved.base_url);
                record_system_event("INFO", "HOME_ASSISTANT", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                true
            }
            Err(err) => {
                let msg = format!("Failed saving Home Assistant config: {err}");
                self.as_mut().set_status_message(QString::from(&msg));
                false
            }
        }
    }

    pub fn search_home_assistant_entities_json(
        mut self: Pin<&mut Self>,
        base_url: QString,
        access_token: QString,
        query: QString,
        domain_filter: QString,
    ) -> QString {
        let url_str = base_url.to_string().trim().to_string();
        let tok_str = access_token.to_string().trim().to_string();
        let q_str = query.to_string();
        let dom_str = domain_filter.to_string();

        if !url_str.is_empty() && !tok_str.is_empty() {
            if let Err(e) = save_home_assistant_config(&url_str, &tok_str) {
                record_system_event(
                    "WARN",
                    "HOME_ASSISTANT",
                    format!("Failed saving Home Assistant config: {e}"),
                );
            }
        }

        let res = crate::actions::web_api::block_on_shared_async(
            fetch_and_search_home_assistant_entities(&url_str, &tok_str, &q_str, &dom_str),
        )
        .and_then(|r| r);

        match res {
            Ok(entities) => {
                let msg = format!(
                    "✓ Home Assistant: found {} matching entit{}",
                    entities.len(),
                    if entities.len() == 1 { "y" } else { "ies" }
                );
                record_system_event("INFO", "HOME_ASSISTANT", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                let resp = serde_json::json!({
                    "ok": true,
                    "count": entities.len(),
                    "entities": entities,
                });
                QString::from(&resp.to_string())
            }
            Err(err) => {
                let msg = format!("Home Assistant error: {err}");
                record_system_event("WARN", "HOME_ASSISTANT", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                let resp = serde_json::json!({
                    "ok": false,
                    "error": err,
                    "entities": [],
                });
                QString::from(&resp.to_string())
            }
        }
    }

    pub fn start_search_home_assistant_entities(
        mut self: Pin<&mut Self>,
        base_url: QString,
        access_token: QString,
        query: QString,
        domain_filter: QString,
    ) {
        if *self.ha_search_busy() {
            return;
        }
        static HA_SEARCH_REQ_ID: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(1);
        let req_id = HA_SEARCH_REQ_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let url_str = base_url.to_string().trim().to_string();
        let tok_str = access_token.to_string().trim().to_string();
        let q_str = query.to_string();
        let dom_str = domain_filter.to_string();

        if !url_str.is_empty() && !tok_str.is_empty() {
            if let Err(e) = save_home_assistant_config(&url_str, &tok_str) {
                record_system_event(
                    "WARN",
                    "HOME_ASSISTANT",
                    format!("Failed saving Home Assistant config: {e}"),
                );
            }
        }

        self.as_mut().set_ha_search_busy(true);
        self.as_mut()
            .set_status_message(QString::from("🔍 Querying Home Assistant entities..."));

        let qt_thread = self.qt_thread();
        let spawned = crate::actions::web_api::spawn_shared_async(async move {
            let res =
                fetch_and_search_home_assistant_entities(&url_str, &tok_str, &q_str, &dom_str)
                    .await;
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_ha_search_busy(false);
                match res {
                    Ok(entities) => {
                        let msg = format!(
                            "✓ Home Assistant: found {} matching entit{}",
                            entities.len(),
                            if entities.len() == 1 { "y" } else { "ies" }
                        );
                        record_system_event("INFO", "HOME_ASSISTANT", &msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_status_message(QString::from(&msg));
                        let resp = serde_json::json!({
                            "ok": true,
                            "requestId": req_id,
                            "count": entities.len(),
                            "entities": entities,
                        });
                        qobject
                            .as_mut()
                            .set_ha_search_result_json(QString::from(&resp.to_string()));
                    }
                    Err(err) => {
                        let msg = format!("Home Assistant error: {err}");
                        record_system_event("WARN", "HOME_ASSISTANT", &msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_status_message(QString::from(&msg));
                        let resp = serde_json::json!({
                            "ok": false,
                            "requestId": req_id,
                            "error": err,
                            "entities": [],
                        });
                        qobject
                            .as_mut()
                            .set_ha_search_result_json(QString::from(&resp.to_string()));
                    }
                }
            });
        });

        if !spawned {
            self.as_mut().set_ha_search_busy(false);
            let err = "Failed scheduling Home Assistant search task".to_string();
            let msg = format!("Home Assistant error: {err}");
            record_system_event("WARN", "HOME_ASSISTANT", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            let resp = serde_json::json!({
                "ok": false,
                "requestId": req_id,
                "error": err,
                "entities": [],
            });
            self.as_mut()
                .set_ha_search_result_json(QString::from(&resp.to_string()));
        }
    }

    pub fn fetch_official_firmwares_from_site(mut self: Pin<&mut Self>) {
        if *self.firmware_fetch_busy() || is_ui_test_suite_env() {
            return;
        }
        let curr_ver = self.firmware_version().to_string();
        self.as_mut().set_firmware_fetch_busy(true);
        self.as_mut().set_status_message(QString::from(
            "Fetching official duckyPad Pro firmware list from dekuNukem/duckyPad-Pro...",
        ));

        let qt_thread = self.qt_thread();
        crate::actions::web_api::spawn_shared_async(async move {
            let res = fetch_official_firmwares_from_site(&curr_ver).await;
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_firmware_fetch_busy(false);
                match res {
                    Ok(remote_list) => {
                        let msg = format!(
                            "✓ Fetched {} firmware version(s) including official releases from dekuNukem/duckyPad-Pro",
                            remote_list.len()
                        );
                        record_system_event("INFO", "FIRMWARE", &msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_status_message(QString::from(&msg));
                    }
                    Err(err) => {
                        let msg = format!("Failed fetching official firmwares: {err}");
                        record_system_event("WARN", "FIRMWARE", &msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_status_message(QString::from(&msg));
                    }
                }
            });
        });
    }

    pub fn download_official_firmware_only(
        mut self: Pin<&mut Self>,
        filename: QString,
        download_url: QString,
    ) {
        if *self.firmware_fetch_busy() {
            return;
        }
        let fname = filename.to_string().trim().to_string();
        let durl = download_url.to_string().trim().to_string();
        if fname.is_empty() || durl.is_empty() {
            return;
        }
        self.as_mut().set_firmware_fetch_busy(true);
        self.as_mut().set_status_message(QString::from(&format!(
            "Downloading official firmware {fname}..."
        )));

        let qt_thread = self.qt_thread();
        crate::actions::web_api::spawn_shared_async(async move {
            let res = download_official_firmware_binary(&fname, &durl).await;
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_firmware_fetch_busy(false);
                match res {
                    Ok(path) => {
                        let msg = format!(
                            "✓ Downloaded & CRC32-verified official firmware: {}",
                            path.display()
                        );
                        record_system_event("INFO", "FIRMWARE", &msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_status_message(QString::from(&msg));
                    }
                    Err(err) => {
                        let msg = format!("Failed downloading {fname}: {err}");
                        record_system_event("ERROR", "FIRMWARE", &msg);
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_status_message(QString::from(&msg));
                    }
                }
            });
        });
    }

    pub fn download_and_install_official_firmware(
        mut self: Pin<&mut Self>,
        filename: QString,
        download_url: QString,
    ) -> bool {
        if *self.sd_sync_busy() || *self.firmware_fetch_busy() {
            return false;
        }
        let fname = filename.to_string().trim().to_string();
        let durl = download_url.to_string().trim().to_string();
        if fname.is_empty() || durl.is_empty() {
            return false;
        }
        self.as_mut().set_firmware_fetch_busy(true);
        self.as_mut().set_sd_sync_busy(true);
        self.as_mut().set_sd_sync_progress(5);
        self.as_mut().set_sd_sync_step(QString::from(&format!(
            "Downloading official firmware {fname} from GitHub..."
        )));
        self.as_mut()
            .set_sd_sync_result_state(QString::from("running"));
        self.as_mut().set_sd_sync_result_summary(QString::from(""));
        self.as_mut().set_status_message(QString::from(&format!(
            "Downloading official firmware {fname} from GitHub..."
        )));

        let qt_thread = self.qt_thread();
        let fname_bg = fname.clone();
        let spawned = crate::actions::web_api::spawn_shared_async(async move {
            let res = download_official_firmware_binary(&fname_bg, &durl).await;
            let _ = qt_thread.queue(move |mut qobject| {
                qobject.as_mut().set_firmware_fetch_busy(false);
                qobject.as_mut().set_sd_sync_busy(false);
                match res {
                    Ok(local_path) => {
                        let path_str = local_path.to_string_lossy().to_string();
                        let _ = qobject
                            .as_mut()
                            .install_firmware_to_device(QString::from(&path_str));
                    }
                    Err(err) => {
                        let msg = format!("Failed downloading official firmware {fname_bg}: {err}");
                        record_system_event("ERROR", "FIRMWARE", &msg);
                        qobject.as_mut().set_sd_sync_progress(0);
                        qobject
                            .as_mut()
                            .set_sd_sync_step(QString::from("Firmware download failed"));
                        qobject
                            .as_mut()
                            .set_sd_sync_result_state(QString::from("error"));
                        qobject
                            .as_mut()
                            .set_sd_sync_result_summary(QString::from(&msg));
                        qobject.as_mut().sync_event_log_properties();
                        qobject.as_mut().set_status_message(QString::from(&msg));
                    }
                }
            });
        });

        if !spawned {
            self.as_mut().set_firmware_fetch_busy(false);
            self.as_mut().set_sd_sync_busy(false);
            let msg = format!("Failed scheduling background download for {fname}");
            record_system_event("ERROR", "FIRMWARE", &msg);
            self.as_mut()
                .set_sd_sync_result_state(QString::from("error"));
            self.as_mut()
                .set_sd_sync_result_summary(QString::from(&msg));
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            return false;
        }
        true
    }
}
