use crate::bridge::device_controller::{qobject, record_system_event, DeviceControllerRust};
use crate::hardware::storage::{
    compute_profiles_fingerprint, detect_unmounted_duckypad_sd,
    discard_unsynced_change_in_profiles, find_duckypad_msc_block_device,
    find_mounted_duckypad_sd_for_device, format_disk_label_for_device, inspect_duckypad_sd_root,
    load_all_sd_profiles, mark_profiles_sd_synced, model_name_for_hw_model, mount_duckypad_sd_only,
    profile_cache_dir_for_serial, query_connected_duckypad_info, read_dpp_config,
    read_duckypad_pro_profiles_via_msc, read_duckypad_pro_profiles_via_msc_with_progress,
    reboot_duckypad, sync_profiles_to_cache, sync_profiles_to_device_cache,
    sync_profiles_to_device_cache_with_meta, try_auto_mount_duckypad_sd_for_device,
    unmount_mounted_duckypad_sd, write_duckypad_pro_profiles_via_msc_with_progress,
    write_profiles_to_sd_root_for_model, DuckyProfile, DuckySdDeviceMeta, DP_MODEL_DUCKYPAD_PRO,
    DP_MODEL_OG_DUCKYPAD, HID_REPORT_ID_OUT, PID_DUCKYPAD_2020, PID_DUCKYPAD_PRO,
};
use crate::kwin::auto_switcher::load_rules_bundle_from_dir;
use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use std::collections::HashMap;
use std::path::PathBuf;

impl qobject::DeviceController {
    pub fn sync_profiles_to_device_sd(mut self: Pin<&mut Self>) -> bool {
        if *self.sd_sync_busy() {
            return false;
        }

        let active_num = (*self.active_profile()).clamp(1, 64) as u8;
        let serial_str = self.serial_number().to_string();
        let hw_model = (*self.active_hw_model()).clamp(20, 24) as u8;
        let fw_str = self.firmware_version().to_string();
        let disk_label_for_dev = format_disk_label_for_device(hw_model, &serial_str);
        let profiles_snapshot = self.rust().profiles.clone();
        let mounted_sd = self.rust().sd_root_path.clone().or_else(|| {
            try_auto_mount_duckypad_sd_for_device(
                if serial_str.is_empty() || serial_str == "-" {
                    None
                } else {
                    Some(&serial_str)
                },
                if disk_label_for_dev.is_empty() {
                    None
                } else {
                    Some(&disk_label_for_dev)
                },
                Some(hw_model),
            )
        });

        if let Some(sd_root) = mounted_sd {
            self.as_mut().set_sd_sync_busy(true);
            self.as_mut().set_sd_sync_progress(10);
            self.as_mut()
                .set_sd_sync_result_state(QString::from("running"));
            self.as_mut().set_sd_sync_result_summary(QString::from(""));
            self.as_mut().set_sd_sync_step(QString::from(
                "Step 1/2: Writing profile changes & .dsb bytecode to mounted microSD...",
            ));
            self.as_mut().set_sd_sync_log(QString::from(&format!(
                "• Target mounted microSD: {} ({})\n• Writing {} profile(s) and compiling DSVM bytecode...",
                sd_root.display(),
                model_name_for_hw_model(hw_model),
                profiles_snapshot.len()
            )));
            self.as_mut().set_status_message(QString::from(
                "Syncing profiles to mounted duckyPad microSD...",
            ));

            let qt_thread = self.qt_thread();
            std::thread::spawn(move || {
                let meta = DuckySdDeviceMeta {
                    serial_number: serial_str.clone(),
                    hw_model,
                    model_name: model_name_for_hw_model(hw_model).to_string(),
                    firmware_version: fw_str.clone(),
                    disk_label: disk_label_for_dev.clone(),
                };
                let res = write_profiles_to_sd_root_for_model(
                    &sd_root,
                    &profiles_snapshot,
                    hw_model,
                    Some(&meta),
                );
                if res.is_ok() {
                    sync_profiles_to_device_cache_with_meta(
                        &serial_str,
                        &profiles_snapshot,
                        Some(&meta),
                    );
                    mark_profiles_sd_synced(&serial_str, &profiles_snapshot);
                }
                let sd_display = sd_root.display().to_string();
                let _ = qt_thread.queue(move |mut qobject| {
                    qobject.as_mut().set_sd_sync_busy(false);
                    match res {
                        Ok(summary) => {
                            let short_fp = &summary.validated_fingerprint
                                [..8.min(summary.validated_fingerprint.len())];
                            let prev_log = qobject.sd_sync_log().to_string();
                            let new_log = format!(
                                "{prev_log}\n✓ Wrote {} updated file(s), verified {} unchanged file(s)\n✓ Read-back validation passed: {} profile(s) verified [hash #{short_fp}]",
                                summary.files_written,
                                summary.files_verified_unchanged,
                                summary.profiles_count
                            );
                            let summary_msg = format!(
                                "Validated & Saved! {} profile(s) verified on {} ({} file(s) updated, read-back hash #{short_fp} confirmed).",
                                summary.profiles_count, sd_display, summary.files_written
                            );
                            qobject.as_mut().set_sd_sync_progress(100);
                            qobject
                                .as_mut()
                                .set_sd_sync_result_state(QString::from("success"));
                            qobject.as_mut().set_sd_sync_step(QString::from(
                                "Validated: All changes successfully saved to microSD!",
                            ));
                            qobject.as_mut().set_sd_sync_log(QString::from(&new_log));
                            qobject
                                .as_mut()
                                .set_sd_sync_result_summary(QString::from(&summary_msg));
                            qobject.as_mut().sync_active_profile_state();
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&summary_msg));
                            crate::kwin::auto_switcher::send_kde_notification(
                                "sdSyncComplete",
                                "microSD Sync Complete",
                                &format!(
                                    "{} profile(s) verified on {} ({} file(s) updated).",
                                    summary.profiles_count, sd_display, summary.files_written
                                ),
                                0,
                            );
                        }
                        Err(err) => {
                            let prev_log = qobject.sd_sync_log().to_string();
                            let new_log = format!("{prev_log}\n✗ Error: {err}");
                            let err_msg = format!("Failed writing profiles to SD card: {err}");
                            qobject
                                .as_mut()
                                .set_sd_sync_result_state(QString::from("error"));
                            qobject
                                .as_mut()
                                .set_sd_sync_step(QString::from("Sync failed"));
                            qobject.as_mut().set_sd_sync_log(QString::from(&new_log));
                            qobject
                                .as_mut()
                                .set_sd_sync_result_summary(QString::from(&err_msg));
                            qobject.as_mut().sync_active_profile_state();
                            qobject.as_mut().set_status_message(QString::from(&err_msg));
                            crate::kwin::auto_switcher::send_kde_notification(
                                "sdSyncError",
                                "microSD Sync Failed",
                                &err_msg,
                                2,
                            );
                        }
                    }
                });
            });
            return true;
        }

        if let Ok(info) = query_connected_duckypad_info() {
            if info.pid == PID_DUCKYPAD_PRO || info.pid == PID_DUCKYPAD_2020 {
                let disk_label = info.disk_label.clone();
                let dev_serial = info.serial.clone();
                let dev_model_name = model_name_for_hw_model(info.hw_model).to_string();
                let label_display = if disk_label.is_empty() {
                    dev_model_name.clone()
                } else {
                    disk_label.clone()
                };

                self.as_mut().set_sd_sync_busy(true);
                self.as_mut().set_sd_sync_progress(5);
                self.as_mut()
                    .set_sd_sync_result_state(QString::from("running"));
                self.as_mut().set_sd_sync_result_summary(QString::from(""));
                self.as_mut().set_sd_sync_step(QString::from(&format!(
                    "Preparing to sync profiles to {dev_model_name} microSD..."
                )));
                self.as_mut().set_sd_sync_log(QString::from(&format!(
                    "• Connected device: {dev_model_name} ({label_display}, SN {dev_serial})\n• Preparing {} profile(s) for hardware sync...",
                    profiles_snapshot.len()
                )));
                self.as_mut().set_status_message(QString::from(&format!(
                    "Syncing profiles to {dev_model_name}..."
                )));

                let qt_thread = self.qt_thread();
                let qt_thread_done = self.qt_thread();
                std::thread::spawn(move || {
                    let res = write_duckypad_pro_profiles_via_msc_with_progress(
                        &disk_label,
                        &dev_serial,
                        &profiles_snapshot,
                        active_num,
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
                    );

                    let _ = qt_thread_done.queue(move |mut qobject| {
                        match res {
                            Ok(summary) => {
                                let short_fp = &summary.validated_fingerprint
                                    [..8.min(summary.validated_fingerprint.len())];
                                let summary_msg = format!(
                                    "Validated & Saved! {} profile(s) verified on {dev_model_name} ({label_display}) — {} file(s) updated, {} unchanged file(s) verified, read-back hash #{short_fp} confirmed.",
                                    summary.profiles_count,
                                    summary.files_written,
                                    summary.files_verified_unchanged
                                );
                                qobject.as_mut().set_connected(true);
                                qobject.as_mut().sync_active_profile_state();
                                let _ = qobject.as_mut().refresh_available_devices_state();
                                qobject.as_mut().set_sd_sync_progress(100);
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_state(QString::from("success"));
                                qobject.as_mut().set_sd_sync_step(QString::from(
                                    "Validated: All changes successfully saved to duckyPad!",
                                ));
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_summary(QString::from(&summary_msg));
                                qobject
                                    .as_mut()
                                    .set_status_message(QString::from(&summary_msg));
                                crate::kwin::auto_switcher::send_kde_notification(
                                    "sdSyncComplete",
                                    "duckyPad microSD Sync Complete",
                                    &format!(
                                        "{} profile(s) synced to {dev_model_name} ({} file(s) updated).",
                                        summary.profiles_count, summary.files_written
                                    ),
                                    0,
                                );
                            }
                            Err(err) => {
                                let prev = qobject.sd_sync_log().to_string();
                                let next = format!("{prev}\n✗ Error: {err}");
                                let err_msg =
                                    format!("Failed syncing profiles to {dev_model_name}: {err}");
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_state(QString::from("error"));
                                qobject
                                    .as_mut()
                                    .set_sd_sync_step(QString::from("Sync failed"));
                                qobject.as_mut().set_sd_sync_log(QString::from(&next));
                                qobject
                                    .as_mut()
                                    .set_sd_sync_result_summary(QString::from(&err_msg));
                                qobject.as_mut().sync_active_profile_state();
                                let _ = qobject.as_mut().refresh_available_devices_state();
                                qobject
                                    .as_mut()
                                    .set_status_message(QString::from(&err_msg));
                                crate::kwin::auto_switcher::send_kde_notification(
                                    "sdSyncError",
                                    "duckyPad microSD Sync Failed",
                                    &err_msg,
                                    2,
                                );
                            }
                        }
                        qobject.as_mut().set_sd_sync_busy(false);
                    });
                });
                return true;
            }
        }

        let msg =
            "Connect duckyPad Pro / 2020 via USB or mount the microSD card to sync changes to hardware";
        self.as_mut()
            .set_sd_sync_result_state(QString::from("error"));
        self.as_mut()
            .set_sd_sync_step(QString::from("No duckyPad storage available"));
        self.as_mut().set_sd_sync_result_summary(QString::from(msg));
        self.as_mut().set_status_message(QString::from(msg));
        false
    }

    pub fn reload_profiles_from_device_sd(mut self: Pin<&mut Self>) -> bool {
        if *self.sd_sync_busy() {
            return false;
        }

        let cur_serial = self.serial_number().to_string();
        let cur_hw_model = (*self.active_hw_model()).clamp(20, 24) as u8;
        let cur_disk_label = format_disk_label_for_device(cur_hw_model, &cur_serial);

        if let Some(sd_root) = self.rust().sd_root_path.clone().or_else(|| {
            try_auto_mount_duckypad_sd_for_device(
                if cur_serial.is_empty() || cur_serial == "-" {
                    None
                } else {
                    Some(&cur_serial)
                },
                if cur_disk_label.is_empty() {
                    None
                } else {
                    Some(&cur_disk_label)
                },
                Some(cur_hw_model),
            )
        }) {
            self.as_mut().set_sd_sync_busy(true);
            self.as_mut().set_sd_sync_progress(40);
            self.as_mut()
                .set_sd_sync_result_state(QString::from("running"));
            self.as_mut().set_sd_sync_step(QString::from(
                "Reading profiles from mounted duckyPad microSD...",
            ));
            self.as_mut().set_sd_sync_log(QString::from(&format!(
                "• Reading profiles from {}",
                sd_root.display()
            )));
            let ok = self
                .as_mut()
                .load_sd_directory(QString::from(sd_root.to_string_lossy().as_ref()));
            self.as_mut().set_sd_sync_busy(false);
            if ok {
                let count = self.rust().profiles.len();
                let fp = compute_profiles_fingerprint(&self.rust().profiles);
                let short_fp = &fp[..8.min(fp.len())];
                let summary = format!(
                    "Validated & Reloaded! {count} profile(s) loaded from {} (hash #{short_fp}).",
                    sd_root.display()
                );
                let prev = self.sd_sync_log().to_string();
                self.as_mut().set_sd_sync_log(QString::from(&format!(
                    "{prev}\n✓ Loaded & verified {count} profile(s) [hash #{short_fp}]"
                )));
                self.as_mut().set_sd_sync_progress(100);
                self.as_mut()
                    .set_sd_sync_result_state(QString::from("success"));
                self.as_mut()
                    .set_sd_sync_step(QString::from("Validated: Profiles reloaded from microSD!"));
                self.as_mut()
                    .set_sd_sync_result_summary(QString::from(&summary));
            } else {
                self.as_mut()
                    .set_sd_sync_result_state(QString::from("error"));
                self.as_mut()
                    .set_sd_sync_step(QString::from("Reload failed"));
                let status = self.status_message().to_string();
                self.as_mut()
                    .set_sd_sync_result_summary(QString::from(&status));
            }
            return ok;
        }

        let msc_target = if let Ok(info) = query_connected_duckypad_info() {
            if info.pid == PID_DUCKYPAD_PRO || info.pid == PID_DUCKYPAD_2020 {
                Some((info.disk_label, info.serial, info.active_profile))
            } else {
                None
            }
        } else if find_duckypad_msc_block_device(&cur_disk_label, true).is_some() {
            let cur_prof = (*self.active_profile() as u8).clamp(1, 64);
            Some((cur_disk_label, cur_serial, cur_prof))
        } else {
            None
        };

        if let Some((disk_label, dev_serial, active_prof)) = msc_target {
            let label_display = if disk_label.is_empty() {
                "duckyPad microSD".to_string()
            } else {
                disk_label.clone()
            };

            self.as_mut().set_sd_sync_busy(true);
            self.as_mut().set_sd_sync_progress(5);
            self.as_mut()
                .set_sd_sync_result_state(QString::from("running"));
            self.as_mut().set_sd_sync_result_summary(QString::from(""));
            self.as_mut().set_sd_sync_step(QString::from(
                "Preparing to mount and scan duckyPad microSD...",
            ));
            self.as_mut().set_sd_sync_log(QString::from(&format!(
                "• Target storage: {label_display} (SN {dev_serial})\n• Mounting microSD and scanning profiles..."
            )));
            self.as_mut()
                .set_status_message(QString::from("Mounting and scanning duckyPad microSD..."));

            let qt_thread = self.qt_thread();
            let qt_thread_done = self.qt_thread();
            std::thread::spawn(move || {
                let res = read_duckypad_pro_profiles_via_msc_with_progress(
                    &disk_label,
                    &dev_serial,
                    active_prof,
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
                );

                let _ = qt_thread_done.queue(move |mut qobject| {
                    match res {
                        Ok(profiles) => {
                            let count = profiles.len();
                            let fp = compute_profiles_fingerprint(&profiles);
                            let short_fp = &fp[..8.min(fp.len())];
                            {
                                let rust = qobject.as_mut().rust_mut().get_mut();
                                let pending_rule_src = rust.pending_rule_import_source.take();
                                let sn_str = rust.serial_number.to_string();
                                let sd_opt = rust.sd_root_path.clone();
                                if let Ok(mut sw) = rust.switcher_state.lock() {
                                    if let Some(ref src_id) = pending_rule_src {
                                        let _ = DeviceControllerRust::apply_rule_source_to_switcher(
                                            &mut sw,
                                            &profiles,
                                            src_id,
                                            sd_opt.as_deref(),
                                            &sn_str,
                                        );
                                    } else {
                                        sw.align_rules_with_profiles(&profiles);
                                    }
                                }
                                rust.profiles = profiles;
                            }
                            qobject
                                .as_mut()
                                .set_active_profile((active_prof as i32).clamp(1, 64));
                            qobject.as_mut().set_connected(true);
                            qobject.as_mut().sync_active_profile_state();
                            let _ = qobject.as_mut().refresh_available_devices_state();

                            let summary_msg = format!(
                                "Validated & Reloaded! {count} profile(s) loaded from {label_display} — read-back hash #{short_fp} confirmed."
                            );
                            qobject.as_mut().set_sd_sync_progress(100);
                            qobject
                                .as_mut()
                                .set_sd_sync_result_state(QString::from("success"));
                            qobject.as_mut().set_sd_sync_step(QString::from(
                                "Validated: Profiles reloaded from duckyPad microSD!",
                            ));
                            qobject
                                .as_mut()
                                .set_sd_sync_result_summary(QString::from(&summary_msg));
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&summary_msg));
                        }
                        Err(err) => {
                            let prev = qobject.sd_sync_log().to_string();
                            let next = format!("{prev}\n✗ Error: {err}");
                            let err_msg = format!(
                                "Failed mounting/scanning duckyPad SD: {err}"
                            );
                            qobject
                                .as_mut()
                                .set_sd_sync_result_state(QString::from("error"));
                            qobject
                                .as_mut()
                                .set_sd_sync_step(QString::from("Mount & Scan failed"));
                            qobject.as_mut().set_sd_sync_log(QString::from(&next));
                            qobject
                                .as_mut()
                                .set_sd_sync_result_summary(QString::from(&err_msg));
                            qobject.as_mut().sync_active_profile_state();
                            let _ = qobject.as_mut().refresh_available_devices_state();
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&err_msg));
                        }
                    }
                    qobject.as_mut().set_sd_sync_busy(false);
                });
            });
            return true;
        }

        let msg = "No mounted duckyPad microSD or connected duckyPad found to reload from";
        self.as_mut()
            .set_sd_sync_result_state(QString::from("error"));
        self.as_mut()
            .set_sd_sync_step(QString::from("No duckyPad storage available"));
        self.as_mut().set_sd_sync_result_summary(QString::from(msg));
        self.as_mut().set_status_message(QString::from(msg));
        false
    }

    pub fn refresh_unsynced_changes(mut self: Pin<&mut Self>) -> QString {
        self.as_mut().sync_active_profile_state();
        self.unsynced_changes_json().clone()
    }

    pub fn sync_rtc_clock(mut self: Pin<&mut Self>) -> bool {
        use chrono::Local;
        let Ok((device, _)) = Self::open_duckypad() else {
            return false;
        };

        let now = Local::now();
        let unix_ts = now.timestamp().max(0) as u32;
        let utc_offset_minutes = (now.offset().local_minus_utc() / 60) as i16;
        let ts_be = unix_ts.to_be_bytes();
        let off_be = utc_offset_minutes.to_be_bytes();

        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x04;
        tx[2] = 0x1A; // HID RTC Sync Command
        tx[3..7].copy_from_slice(&ts_be);
        tx[7..9].copy_from_slice(&off_be);

        let ok = device.write(&tx).is_ok();
        if ok {
            let mut rx = [0u8; 64];
            if let Err(e) = device.read_timeout(&mut rx, 200) {
                record_system_event(
                    "WARN",
                    "HID",
                    format!("HID ACK read timeout in sync_rtc_clock: {e}"),
                );
            }
            self.as_mut()
                .set_status_message(QString::from("RTC synchronized with Linux host clock"));
        }
        ok
    }

    pub fn set_sleep_state(self: Pin<&mut Self>, sleep: bool) -> bool {
        let Ok((device, _)) = Self::open_duckypad() else {
            return false;
        };

        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x05;
        tx[2] = if sleep { 0x15 } else { 0x16 };
        let ok = device.write(&tx).is_ok();
        if ok {
            let mut rx = [0u8; 64];
            if let Err(e) = device.read_timeout(&mut rx, 200) {
                record_system_event(
                    "WARN",
                    "HID",
                    format!("HID ACK read timeout in set_sleep_state: {e}"),
                );
            }
        }
        ok
    }

    pub fn install_udev_rules(mut self: Pin<&mut Self>) {
        if crate::hardware::permissions::install_udev_via_pkexec().is_ok() {
            self.as_mut().refresh_device();
        } else {
            self.as_mut().set_status_message(QString::from(
                "Polkit canceled or failed. Run data/udev/install-udev-rules.sh as root.",
            ));
        }
    }

    pub fn read_pgv(mut self: Pin<&mut Self>, pgv_index: i32) -> i32 {
        use crate::hardware::storage::{open_duckypad_hid, read_all_pgvs};
        let idx = pgv_index.clamp(0, 31) as usize;
        match open_duckypad_hid().and_then(|dev| read_all_pgvs(&dev)) {
            Ok(pgvs) => {
                let val = pgvs[idx];
                self.as_mut().set_status_message(QString::from(&format!(
                    "Read _GV{idx} = {val} (0x{val:08X})"
                )));
                val
            }
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&e));
                0
            }
        }
    }

    pub fn write_pgv(mut self: Pin<&mut Self>, pgv_index: i32, value: i32) -> bool {
        use crate::hardware::storage::{open_duckypad_hid, write_single_pgv};
        let idx = pgv_index.clamp(0, 31) as u8;
        match open_duckypad_hid().and_then(|dev| write_single_pgv(&dev, idx, value)) {
            Ok(()) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Wrote _GV{idx} = {value} to non-volatile memory"
                )));
                true
            }
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&e));
                false
            }
        }
    }

    pub fn dump_all_pgvs_json(mut self: Pin<&mut Self>) -> QString {
        use crate::hardware::storage::{open_duckypad_hid, read_all_pgvs};
        match open_duckypad_hid().and_then(|dev| read_all_pgvs(&dev)) {
            Ok(pgvs) => {
                let json = serde_json::to_string(&pgvs.to_vec()).unwrap_or_else(|_| "[]".into());
                self.as_mut().set_status_message(QString::from(
                    "Read all 32 PGVs (_GV0.._GV31) via 3-packet HID burst (0x18)",
                ));
                QString::from(&json)
            }
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&e));
                QString::from("[]")
            }
        }
    }

    pub fn scan_mounted_sd_card(mut self: Pin<&mut Self>) -> QString {
        if let Some(sd_root) = try_auto_mount_duckypad_sd_for_device(None, None, None) {
            let path_str = sd_root.to_string_lossy().to_string();
            if self.as_mut().load_sd_directory(QString::from(&path_str)) {
                return QString::from(&path_str);
            }
            return QString::from("");
        }

        if let Ok(info) = query_connected_duckypad_info() {
            if (info.pid == PID_DUCKYPAD_PRO || info.pid == PID_DUCKYPAD_2020)
                && !info.disk_label.is_empty()
            {
                if let Ok(profiles) = read_duckypad_pro_profiles_via_msc(
                    &info.disk_label,
                    &info.serial,
                    info.active_profile,
                ) {
                    let count = profiles.len();
                    let cache_path = profile_cache_dir_for_serial(&info.serial);
                    {
                        let rust = self.as_mut().rust_mut().get_mut();
                        if let Ok(mut sw) = rust.switcher_state.lock() {
                            sw.align_rules_with_profiles(&profiles);
                        }
                        rust.profiles = profiles;
                    }
                    self.as_mut()
                        .set_active_profile((info.active_profile as i32).clamp(1, 64));
                    self.as_mut().sync_active_profile_state();
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Synced {count} profile(s) from connected {} ({})",
                        model_name_for_hw_model(info.hw_model),
                        info.disk_label
                    )));
                    return QString::from(cache_path.to_string_lossy().as_ref());
                }
            }
        }

        let msg = "No mounted duckyPad microSD found in /run/media or /media — use 'Open SD Folder' to choose a directory";
        self.as_mut().set_status_message(QString::from(msg));
        QString::from("")
    }

    pub fn load_sd_directory(mut self: Pin<&mut Self>, path: QString) -> bool {
        let raw_path = path.to_string();
        let cleaned = crate::paths::clean_file_uri_to_path_str(&raw_path);
        if cleaned.is_empty() {
            return false;
        }

        let sd_root = PathBuf::from(&cleaned);
        if !sd_root.is_dir() {
            let msg = format!("Directory does not exist: {}", sd_root.display());
            self.as_mut().set_status_message(QString::from(&msg));
            return false;
        }

        let dpp_cfg = read_dpp_config(&sd_root).unwrap_or_default();

        match load_all_sd_profiles(&sd_root) {
            Ok(profiles) => {
                if profiles.is_empty() {
                    // Check if the user selected a single profile_<Name> or profile<N>_<Name> directory directly
                    if let Ok(single_prof) = DuckyProfile::load_from_dir(&sd_root) {
                        let parent = sd_root
                            .parent()
                            .map(|p| p.to_path_buf())
                            .unwrap_or_else(|| sd_root.clone());
                        let prof_num = single_prof.number as i32;
                        {
                            let rust = self.as_mut().rust_mut().get_mut();
                            rust.sd_root_path = Some(parent.clone());
                            rust.profiles = vec![single_prof];
                            mark_profiles_sd_synced(
                                &rust.serial_number.to_string(),
                                &rust.profiles,
                            );
                        }
                        self.as_mut()
                            .set_sd_mount_path(QString::from(parent.to_string_lossy().as_ref()));
                        self.as_mut().set_active_profile(prof_num);
                        self.as_mut().sync_active_profile_state();
                        self.as_mut().set_status_message(QString::from(&format!(
                            "Loaded profile #{prof_num} from {}",
                            sd_root.display()
                        )));
                        return true;
                    }

                    let msg = format!("No duckyPad profile folders found in {}", sd_root.display());
                    self.as_mut().set_status_message(QString::from(&msg));
                    return false;
                }

                let count = profiles.len();
                let had_sd_before = self.rust().sd_root_path.is_some();
                let current_active = if !had_sd_before {
                    dpp_cfg
                        .last_profile
                        .filter(|&n| n > 0)
                        .unwrap_or(*self.active_profile() as u8)
                } else {
                    *self.active_profile() as u8
                };
                let chosen_num = profiles
                    .iter()
                    .find(|p| p.number == current_active)
                    .or_else(|| profiles.first())
                    .map(|p| p.number as i32)
                    .unwrap_or(1);

                let current_serial = self.serial_number().to_string();
                {
                    let rust = self.as_mut().rust_mut().get_mut();
                    let sn_for_cache = dpp_cfg
                        .serial_number
                        .as_deref()
                        .filter(|s| !s.is_empty())
                        .unwrap_or(&current_serial);
                    let pending_rule_src = rust.pending_rule_import_source.take();
                    if let Ok(mut sw) = rust.switcher_state.lock() {
                        if let Some(ref src_id) = pending_rule_src {
                            let _ = DeviceControllerRust::apply_rule_source_to_switcher(
                                &mut sw,
                                &profiles,
                                src_id,
                                Some(&sd_root),
                                sn_for_cache,
                            );
                        } else {
                            if let Some((sd_en, sd_fb, sd_rules)) =
                                load_rules_bundle_from_dir(&sd_root, &profiles)
                            {
                                if sd_rules.len() > sw.rules.len() {
                                    sw.enabled = sd_en;
                                    sw.rules = sd_rules;
                                    if let Some(fb) = sd_fb {
                                        sw.default_profile = Some(fb);
                                    }
                                    sw.loaded_from_disk = true;
                                    let _ = sw.save_to_disk();
                                }
                            }
                            sw.align_rules_with_profiles(&profiles);
                        }
                    }
                    sync_profiles_to_device_cache(sn_for_cache, &profiles);
                    mark_profiles_sd_synced(sn_for_cache, &profiles);
                    rust.sd_root_path = Some(sd_root.clone());
                    rust.profiles = profiles;
                }

                if !*self.connected() {
                    let sd_info = inspect_duckypad_sd_root(&sd_root);
                    let fallback_hw_model = (*self.active_hw_model()).clamp(20, 24) as u8;
                    let sd_hw_model = sd_info
                        .as_ref()
                        .map(|s| s.hw_model)
                        .filter(|&m| m == DP_MODEL_OG_DUCKYPAD || m == DP_MODEL_DUCKYPAD_PRO)
                        .or(dpp_cfg.hw_model)
                        .unwrap_or(fallback_hw_model);
                    self.as_mut().update_hardware_model_properties(sd_hw_model);
                    let model_label = if sd_hw_model == DP_MODEL_OG_DUCKYPAD {
                        "duckyPad (2020) (SD Mode)"
                    } else {
                        "duckyPad Pro (SD Mode)"
                    };
                    self.as_mut().set_model_name(QString::from(model_label));
                    if let Some(fw) = dpp_cfg.fw_ver.as_deref().filter(|s| !s.is_empty()) {
                        self.as_mut().set_firmware_version(QString::from(fw));
                    }
                    if let Some(sn) = dpp_cfg
                        .serial_number
                        .as_deref()
                        .or_else(|| sd_info.as_ref().and_then(|s| s.serial_number.as_deref()))
                        .filter(|s| !s.is_empty())
                    {
                        self.as_mut().set_serial_number(QString::from(sn));
                    }
                }

                self.as_mut()
                    .set_sd_mount_path(QString::from(sd_root.to_string_lossy().as_ref()));
                self.as_mut().set_active_profile(chosen_num);
                let _ = self.as_mut().refresh_available_devices_state();
                self.as_mut().sync_active_profile_state();

                let msg = format!("Loaded {count} profile(s) from {}", sd_root.display());
                self.as_mut().set_status_message(QString::from(&msg));
                true
            }
            Err(e) => {
                let msg = format!("SD read error: {e}");
                self.as_mut().set_status_message(QString::from(&msg));
                false
            }
        }
    }

    pub fn save_active_profile_to_sd(mut self: Pin<&mut Self>) -> bool {
        let active_num = *self.active_profile() as u8;
        let result = {
            let rust = self.rust();
            let Some(ref sd_root) = rust.sd_root_path else {
                return false;
            };
            let Some(prof) = rust
                .profiles
                .iter()
                .find(|p| p.number == active_num)
                .or_else(|| rust.profiles.first())
            else {
                return false;
            };
            let res = prof.save_to_sd_root(sd_root);
            if res.is_ok() {
                mark_profiles_sd_synced(&rust.serial_number.to_string(), &rust.profiles);
            }
            res
        };

        self.as_mut().sync_active_profile_state();

        match result {
            Ok(dir) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Saved profile #{active_num} to {}",
                    dir.display()
                )));
                true
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error saving profile: {e}")));
                false
            }
        }
    }

    pub fn export_active_profile(mut self: Pin<&mut Self>, destination_path: QString) -> bool {
        let active_num = *self.active_profile();
        self.as_mut().export_profile(active_num, destination_path)
    }

    pub fn export_profile(
        self: Pin<&mut Self>,
        profile_number: i32,
        destination_path: QString,
    ) -> bool {
        crate::bridge::serialize::export_profile_impl(self, profile_number, destination_path)
    }

    pub fn import_profile_from_path(self: Pin<&mut Self>, source_path: QString) -> bool {
        crate::bridge::serialize::import_profile_from_path_impl(self, source_path)
    }

    pub fn save_profile_group(
        self: Pin<&mut Self>,
        destination_path: QString,
        group_name: QString,
    ) -> bool {
        crate::bridge::serialize::save_profile_group_impl(self, destination_path, group_name)
    }

    pub fn load_profile_group(self: Pin<&mut Self>, source_path: QString) -> bool {
        crate::bridge::serialize::load_profile_group_impl(self, source_path)
    }

    pub fn create_new_profile_group(
        self: Pin<&mut Self>,
        group_name: QString,
        initial_profile_name: QString,
    ) -> bool {
        crate::bridge::serialize::create_new_profile_group_impl(
            self,
            group_name,
            initial_profile_name,
        )
    }

    pub fn reboot_device(mut self: Pin<&mut Self>) -> bool {
        match reboot_duckypad() {
            Ok(()) => {
                self.as_mut().set_status_message(QString::from(
                    "⟳ Sent software reboot command to duckyPad...",
                ));
                let qt_thread = self.qt_thread();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(1600));
                    let _ = qt_thread.queue(|mut ctrl| {
                        ctrl.as_mut().refresh_device();
                    });
                });
                true
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Failed to reboot duckyPad: {e}")));
                false
            }
        }
    }

    pub fn discard_unsynced_change(
        mut self: Pin<&mut Self>,
        profile_number: i32,
        profile_name: QString,
        change_key: QString,
    ) -> bool {
        let p_num = profile_number.clamp(1, 64) as u8;
        let p_name = profile_name.to_string();
        let key = change_key.to_string();

        let current_fb = self
            .rust()
            .switcher_state
            .lock()
            .ok()
            .and_then(|sw| sw.default_profile)
            .unwrap_or_else(|| (*self.default_fallback_profile()).max(1) as u8);
        if key == "profile_added" && p_num == current_fb {
            let status =
                format!("⚠ Cannot discard Profile #{p_num} because it is the Fallback Profile");
            self.as_mut().set_status_message(QString::from(&status));
            return false;
        }

        let mut new_fallback_num: Option<i32> = None;
        let res = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sn = rust.serial_number.to_string();
            let sd_root = rust.sd_root_path.clone();
            let old_profiles: Vec<(u8, String)> = rust
                .profiles
                .iter()
                .map(|p| (p.number, p.name.clone()))
                .collect();
            let out = discard_unsynced_change_in_profiles(
                &sn,
                &mut rust.profiles,
                sd_root.as_deref(),
                p_num,
                &p_name,
                &key,
            );
            if out.is_ok() {
                let mut old_to_new: HashMap<u8, u8> = HashMap::with_capacity(old_profiles.len());
                for (old_num, old_name) in &old_profiles {
                    if let Some(found) = rust
                        .profiles
                        .iter()
                        .find(|p| p.name.eq_ignore_ascii_case(old_name))
                    {
                        old_to_new.insert(*old_num, found.number);
                    }
                }
                if let Ok(mut sw) = rust.switcher_state.lock() {
                    let mut rules_changed = false;
                    let max_p = rust.profiles.len().max(1) as u8;
                    let old_fb = sw.default_profile.unwrap_or(current_fb);
                    let mapped_fb = old_to_new
                        .get(&old_fb)
                        .copied()
                        .unwrap_or(old_fb)
                        .clamp(1, max_p);
                    if sw.default_profile != Some(mapped_fb) {
                        sw.default_profile = Some(mapped_fb);
                        rules_changed = true;
                    }
                    new_fallback_num = Some(mapped_fb as i32);

                    if matches!(key.as_str(), "profile_added" | "profile_deleted" | "order") {
                        for rule in &mut sw.rules {
                            if key == "profile_added" && rule.target_profile == p_num {
                                if rule.target_profile != mapped_fb {
                                    rule.target_profile = mapped_fb;
                                    rules_changed = true;
                                }
                            } else if let Some(&mapped_target) =
                                old_to_new.get(&rule.target_profile)
                            {
                                if rule.target_profile != mapped_target {
                                    rule.target_profile = mapped_target;
                                    rules_changed = true;
                                }
                            }
                        }
                    }
                    sw.align_rules_with_profiles(&rust.profiles);
                    if rules_changed || sw.loaded_from_disk {
                        if let Err(e) = sw.save_to_disk() {
                            record_system_event(
                                "WARN",
                                "CONFIG",
                                format!("Failed saving switcher state after discard: {e}"),
                            );
                        }
                    }
                }
                if sn != "-" && !sn.is_empty() {
                    sync_profiles_to_device_cache(&sn, &rust.profiles);
                } else {
                    sync_profiles_to_cache(&rust.profiles);
                }
            }
            out
        };

        match res {
            Ok(msg) => {
                if let Some(fb) = new_fallback_num {
                    self.as_mut().set_default_fallback_profile(fb);
                }
                let max_prof = self.rust().profiles.len().max(1) as i32;
                let cur_active = *self.active_profile();
                if cur_active > max_prof {
                    self.as_mut().set_active_profile(max_prof);
                }
                self.as_mut().sync_active_profile_state();
                let status = format!("↩ {msg}");
                record_system_event("INFO", "SD_SYNC", &status);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&status));
                true
            }
            Err(err) => {
                let status = format!("⚠ Could not discard change: {err}");
                record_system_event("WARN", "SD_SYNC", &status);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&status));
                false
            }
        }
    }

    pub fn refresh_sd_mount_status(mut self: Pin<&mut Self>) -> bool {
        let dev_serial = self.serial_number().to_string();
        let hw_model = (*self.active_hw_model()).clamp(20, 24) as u8;
        let disk_label = format_disk_label_for_device(hw_model, &dev_serial);

        if let Some(ref sd_root) = self.rust().sd_root_path.clone() {
            if !crate::hardware::storage::is_duckypad_sd_root(sd_root) {
                self.as_mut().rust_mut().get_mut().sd_root_path = None;
                self.as_mut().set_sd_mount_path(QString::from(""));
            }
        }
        if self.rust().sd_root_path.is_none() {
            if let Some(mounted) = find_mounted_duckypad_sd_for_device(
                if dev_serial.is_empty() || dev_serial == "-" {
                    None
                } else {
                    Some(&dev_serial)
                },
                if disk_label.is_empty() {
                    None
                } else {
                    Some(&disk_label)
                },
                Some(hw_model),
            ) {
                let mounted_str = mounted.to_string_lossy().to_string();
                self.as_mut().rust_mut().get_mut().sd_root_path = Some(mounted);
                self.as_mut().set_sd_mount_path(QString::from(&mounted_str));
            }
        }
        let is_mounted = !self.sd_mount_path().is_empty();
        let detected = is_mounted || detect_unmounted_duckypad_sd(&disk_label).is_some();
        if *self.sd_card_detected() != detected {
            self.as_mut().set_sd_card_detected(detected);
        }
        is_mounted
    }

    pub fn mount_sd_card(mut self: Pin<&mut Self>) -> bool {
        if self.as_mut().refresh_sd_mount_status() {
            let mounted_path = self.sd_mount_path().to_string();
            self.as_mut()
                .load_sd_directory(QString::from(&mounted_path));
            return true;
        }
        let dev_serial = self.serial_number().to_string();
        let hw_model = (*self.active_hw_model()).clamp(20, 24) as u8;
        let disk_label = format_disk_label_for_device(hw_model, &dev_serial);
        match mount_duckypad_sd_only(&disk_label) {
            Ok(mount_root) => {
                let mount_str = mount_root.to_string_lossy().to_string();
                self.as_mut().set_sd_card_detected(true);
                let ok = self.as_mut().load_sd_directory(QString::from(&mount_str));
                let msg = format!("✓ Mounted duckyPad microSD card at {mount_str}");
                record_system_event("INFO", "STORAGE", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                ok
            }
            Err(err) => {
                let err_msg = format!("Failed to mount microSD card: {err}");
                record_system_event("ERROR", "STORAGE", &err_msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&err_msg));
                false
            }
        }
    }

    pub fn unmount_sd_card(mut self: Pin<&mut Self>) -> bool {
        let current_mount = self.rust().sd_root_path.clone();
        match unmount_mounted_duckypad_sd(current_mount.as_deref()) {
            Ok(msg) => {
                self.as_mut().rust_mut().get_mut().sd_root_path = None;
                self.as_mut().set_sd_mount_path(QString::from(""));
                self.as_mut().refresh_sd_mount_status();
                self.as_mut().sync_active_profile_state();
                let full_msg = format!("✓ {msg}");
                record_system_event("INFO", "STORAGE", &full_msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&full_msg));
                true
            }
            Err(err) => {
                let err_msg = format!("Failed to unmount microSD card: {err}");
                record_system_event("ERROR", "STORAGE", &err_msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&err_msg));
                false
            }
        }
    }
}
