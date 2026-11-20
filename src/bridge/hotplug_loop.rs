use crate::bridge::device_controller::{is_ui_test_suite_env, qobject, record_system_event};
use crate::hardware::storage::{
    load_sd_synced_snapshot, poll_and_clear_pgv31_trigger, poll_hardware_active_profile,
    try_auto_mount_duckypad_sd_for_device, DuckypadEvdevListener,
};
use crate::kwin::auto_switcher::{
    is_web_browser_class, send_hid_switch_profile_checked, strip_browser_title_suffix,
    HidSwitchOutcome,
};
use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
#[cfg(not(target_os = "windows"))]
use std::fs;
use std::path::PathBuf;

impl qobject::DeviceController {
    pub fn start_kwin_listener(mut self: Pin<&mut Self>) {
        if is_ui_test_suite_env() || self.rust().kwin_listener_started {
            return;
        }
        self.as_mut().rust_mut().get_mut().kwin_listener_started = true;

        // Start the HTTP REST API server if enabled in persisted settings
        self.as_mut().sync_rest_api_server_from_state();

        let state = self.rust().switcher_state.clone();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let qt_thread = self.qt_thread();
        let hotplug_qt_thread = self.qt_thread();
        let web_api_qt_thread = self.qt_thread();

        let switcher_state = state.clone();
        crate::actions::web_api::spawn_shared_async(async move {
            let _ = crate::kwin::auto_switcher::run_kwin_auto_switcher(switcher_state, tx).await;
        });

        crate::actions::web_api::spawn_shared_async(async move {
            while let Some(evt) = rx.recv().await {
                let _ = qt_thread.queue(move |mut qobject| {
                    qobject
                        .as_mut()
                        .set_active_window_class(QString::from(&evt.resource_class));
                    qobject
                        .as_mut()
                        .set_active_window_title(QString::from(&evt.caption));

                    if evt.picked {
                        let picked_title = if is_web_browser_class(&evt.resource_class) {
                            strip_browser_title_suffix(&evt.caption)
                        } else {
                            evt.caption.clone()
                        };
                        qobject.as_mut().set_window_picker_armed(false);
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
                        return;
                    }

                    let is_paused = qobject
                        .rust()
                        .switcher_state
                        .lock()
                        .map(|mut sw| {
                            sw.current_window_class = evt.resource_class.clone();
                            sw.current_window_title = evt.caption.clone();
                            sw.paused_for_editing
                        })
                        .unwrap_or(false);

                    if !is_paused {
                        if let Some(prof) = evt.matched_profile {
                            qobject.as_mut().handle_auto_switched_profile(
                                prof,
                                &evt.resource_class,
                                &evt.caption,
                                evt.hid_rejected_not_on_device,
                            );
                        } else {
                            qobject.as_mut().sync_tray_state();
                        }
                    } else {
                        qobject.as_mut().sync_tray_state();
                    }
                });
            }
        });

        // Dedicated OS thread for blocking evdev F13..F24, modifier layer, and PGV31 input polling
        let modifier_state = state;
        let _ = std::thread::Builder::new()
            .name("duckydeck-input-poll".to_string())
            .spawn(move || {
                let mut evdev_listener = DuckypadEvdevListener::new();
                let mut modifier_listener =
                    crate::actions::app_launch::HostKeyboardModifierListener::new();
                let mut tick_count = 0u32;
                let mut was_modifier_polling_active = false;
                loop {
                    if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                        .load(std::sync::atomic::Ordering::SeqCst)
                    {
                        crate::hardware::storage::invalidate_cached_hid_device();
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        continue;
                    }

                    // 1. Poll evdev F13..F24 sentinel keys + track EV_KEY press/release state
                    for fkey_name in evdev_listener.poll_fkey_triggers() {
                        let qt = web_api_qt_thread.clone();
                        let _ = qt.queue(move |mut qobject| {
                            qobject
                                .as_mut()
                                .handle_hardware_fkey_web_api_trigger(&fkey_name);
                        });
                    }

                    // 2. Poll Main-Keyboard Modifier Layer Switcher every 50ms (skip cloning when idle/disabled)
                    let modifier_work = if let Ok(sw) = modifier_state.lock() {
                        if sw.modifier_switch_enabled && sw.modifier_rules.iter().any(|r| r.enabled)
                        {
                            Some((
                                sw.paused_for_editing,
                                sw.modifier_rules.clone(),
                                sw.current_window_class.clone(),
                                sw.current_window_title.clone(),
                            ))
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    if let Some((mod_paused, mod_rules, cur_win_class, cur_win_title)) =
                        modifier_work
                    {
                        was_modifier_polling_active = true;
                        if let Some(transition) = modifier_listener.poll_modifier_layer(
                            true,
                            mod_paused,
                            &mod_rules,
                            &cur_win_class,
                            &cur_win_title,
                        ) {
                            let qt = web_api_qt_thread.clone();
                            let _ = qt.queue(move |mut qobject| {
                                qobject
                                    .as_mut()
                                    .handle_modifier_layer_transition(transition);
                            });
                        }
                    } else if was_modifier_polling_active {
                        was_modifier_polling_active = false;
                        if let Some(transition) =
                            modifier_listener.poll_modifier_layer(false, false, &[], "", "")
                        {
                            let qt = web_api_qt_thread.clone();
                            let _ = qt.queue(move |mut qobject| {
                                qobject
                                    .as_mut()
                                    .handle_modifier_layer_transition(transition);
                            });
                        }
                    }

                    // 3. Poll silent PGV _GV31 hardware trigger (0x444Bxxxx) every 150ms when active profile uses PGV31
                    tick_count = tick_count.wrapping_add(1);
                    if tick_count.is_multiple_of(3)
                        && crate::hardware::storage::ACTIVE_PROFILE_HAS_PGV31_TRIGGER
                            .load(std::sync::atomic::Ordering::SeqCst)
                    {
                        if let Some((encoded_prof, slot_num, is_release)) =
                            poll_and_clear_pgv31_trigger()
                        {
                            crate::hardware::storage::LAST_EVDEV_KEYDOWN_MS.store(
                                crate::kwin::auto_switcher::current_epoch_ms(),
                                std::sync::atomic::Ordering::SeqCst,
                            );
                            let qt = web_api_qt_thread.clone();
                            let _ = qt.queue(move |mut qobject| {
                                qobject.as_mut().handle_hardware_pgv_web_api_trigger(
                                    encoded_prof,
                                    slot_num,
                                    is_release,
                                );
                            });
                        }
                    }

                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            });

        // Dedicated OS thread for blocking USB HID hotplug, SD mount, and hardware profile sync polling
        let _ = std::thread::Builder::new()
            .name("duckydeck-hotplug-poll".to_string())
            .spawn(move || {
                let mut last_mounts_hash: u64 = 0;
                let mut mounts_tick: u32 = 0;
                let mut last_observed_hw_prof: Option<u8> = None;
                let mut prev_connected_count: Option<i32> = None;
                let mut prev_hid_present: Option<bool> = None;
                let mut pending_post_cooldown_sync = false;
                loop {
                    if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                        .load(std::sync::atomic::Ordering::SeqCst)
                    {
                        crate::hardware::storage::invalidate_cached_hid_device();
                        last_observed_hw_prof = None;
                        std::thread::sleep(std::time::Duration::from_millis(800));
                        continue;
                    }

                    let (new_connected_count_usize, hid_present) =
                        crate::hardware::storage::scan_duckypad_hid_presence();
                    let new_connected_count = new_connected_count_usize as i32;
                    let hw_active_prof = if hid_present {
                        poll_hardware_active_profile()
                    } else {
                        None
                    };
                    let hw_prof_changed = match (last_observed_hw_prof, hw_active_prof) {
                        (Some(prev), Some(curr)) => prev != curr,
                        _ => false,
                    };
                    if !hid_present {
                        last_observed_hw_prof = None;
                    } else if hw_active_prof.is_some() {
                        last_observed_hw_prof = hw_active_prof;
                    }
                    let selected_ser = crate::hardware::storage::get_selected_duckypad_serial();
                    let mut secondary_prof_changed = false;
                    for sec_dev in crate::kwin::auto_switcher::get_secondary_devices_snapshot() {
                        if !sec_dev.usb_connected || sec_dev.serial.is_empty() {
                            continue;
                        }
                        let is_sel = selected_ser
                            .as_deref()
                            .map(|s| {
                                crate::hardware::storage::serial_matches_target(
                                    &sec_dev.serial,
                                    s,
                                )
                            })
                            .unwrap_or(false);
                        if !is_sel {
                            if let Some(sec_prof) =
                                crate::hardware::storage::poll_hardware_active_profile_for_serial(
                                    Some(&sec_dev.serial),
                                )
                            {
                                if crate::kwin::auto_switcher::update_secondary_device_active_profile(
                                    &sec_dev.serial,
                                    sec_prof,
                                ) {
                                    secondary_prof_changed = true;
                                }
                            }
                        }
                    }

                    // Only scan for newly mounted SD cards when system mount table changes
                    mounts_tick = mounts_tick.wrapping_add(1);
                    let mut mounts_changed = false;
                    let mut newly_mounted_sd: Option<PathBuf> = None;
                    if mounts_tick == 1 || mounts_tick.is_multiple_of(3) {
                        let cur_hash = {
                            use std::hash::{Hash, Hasher};
                            let mut hasher = std::collections::hash_map::DefaultHasher::new();
                            #[cfg(target_os = "windows")]
                            {
                                for letter in b'D'..=b'Z' {
                                    let drive = PathBuf::from(format!("{}:\\", letter as char));
                                    if drive.is_dir() {
                                        letter.hash(&mut hasher);
                                    }
                                }
                            }
                            #[cfg(not(target_os = "windows"))]
                            {
                                if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
                                    mounts.hash(&mut hasher);
                                }
                                if let Ok(entries) = fs::read_dir("/sys/block") {
                                    for entry in entries.flatten() {
                                        let name = entry.file_name().to_string_lossy().to_string();
                                        if name.starts_with("mmcblk") || name.starts_with("sd") {
                                            name.hash(&mut hasher);
                                            if let Ok(sz) =
                                                fs::read_to_string(entry.path().join("size"))
                                            {
                                                sz.hash(&mut hasher);
                                            }
                                        }
                                    }
                                }
                            }
                            hasher.finish()
                        };
                        if cur_hash != last_mounts_hash {
                            last_mounts_hash = cur_hash;
                            mounts_changed = true;
                            newly_mounted_sd = try_auto_mount_duckypad_sd_for_device(
                                selected_ser.as_deref(),
                                None,
                                None,
                            );
                        }
                    }

                    let in_msc_cooldown_bg =
                        crate::hardware::storage::is_msc_sync_or_cooldown_active(8000);
                    let state_changed = prev_connected_count != Some(new_connected_count)
                        || prev_hid_present != Some(hid_present);
                    if in_msc_cooldown_bg && (state_changed || !hid_present) {
                        pending_post_cooldown_sync = true;
                    }
                    let should_notify_qt = state_changed
                        || hw_prof_changed
                        || secondary_prof_changed
                        || mounts_changed
                        || newly_mounted_sd.is_some()
                        || (pending_post_cooldown_sync && !in_msc_cooldown_bg);
                    prev_connected_count = Some(new_connected_count);
                    prev_hid_present = Some(hid_present);
                    if !in_msc_cooldown_bg {
                        pending_post_cooldown_sync = false;
                    }

                    if should_notify_qt {
                        let qt = hotplug_qt_thread.clone();
                        let _ = qt.queue(move |mut qobject| {
                            if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                                .load(std::sync::atomic::Ordering::SeqCst)
                            {
                                return;
                            }
                            let in_msc_cooldown = *qobject.sd_sync_busy()
                                || crate::hardware::storage::is_msc_sync_or_cooldown_active(8000);

                            if *qobject.connected_device_count() != new_connected_count {
                                if !in_msc_cooldown
                                    || (new_connected_count > 0
                                        && new_connected_count >= *qobject.connected_device_count())
                                {
                                    let _ = qobject.as_mut().refresh_available_devices_state();
                                }
                            } else if secondary_prof_changed {
                                qobject.as_mut().sync_multi_device_oled_states();
                            }

                            let mut sd_unmounted = false;
                            if let Some(ref sd_root) = qobject.rust().sd_root_path.clone() {
                                if !crate::hardware::storage::is_duckypad_sd_root(sd_root) {
                                    qobject.as_mut().rust_mut().get_mut().sd_root_path = None;
                                    qobject.as_mut().set_sd_mount_path(QString::from(""));
                                    sd_unmounted = true;
                                }
                            } else if let Some(mounted) = newly_mounted_sd {
                                let mounted_str = mounted.to_string_lossy().to_string();
                                if !*qobject.connected() || qobject.rust().profiles.len() <= 1 {
                                    let _ = qobject
                                        .as_mut()
                                        .load_sd_directory(QString::from(&mounted_str));
                                } else {
                                    qobject.as_mut().rust_mut().get_mut().sd_root_path =
                                        Some(mounted);
                                    qobject.as_mut().set_sd_mount_path(QString::from(&mounted_str));
                                }
                            }

                            let was_connected = *qobject.connected();
                            if was_connected && !hid_present {
                                if in_msc_cooldown {
                                    return;
                                }
                                qobject.as_mut().set_connected(false);
                                if qobject.rust().sd_root_path.is_none() {
                                    qobject
                                        .as_mut()
                                        .set_model_name(QString::from("Disconnected"));
                                    qobject
                                        .as_mut()
                                        .set_firmware_version(QString::from("-"));
                                    qobject.as_mut().set_serial_number(QString::from("-"));
                                    qobject
                                        .as_mut()
                                        .set_latest_firmware_version(QString::from("-"));
                                    qobject.as_mut().set_firmware_update_available(false);
                                    qobject.as_mut().set_status_message(QString::from(
                                        "duckyPad disconnected",
                                    ));
                                } else {
                                    qobject.as_mut().set_status_message(QString::from(
                                        "duckyPad USB HID disconnected (SD Card Mode)",
                                    ));
                                }
                                qobject.as_mut().sync_active_profile_state();
                                qobject.as_mut().sync_tray_state();
                                let _ = qobject.as_mut().refresh_available_devices_state();
                                crate::kwin::auto_switcher::send_kde_notification(
                                    "deviceConnection",
                                    "duckyPad Disconnected",
                                    "duckyPad USB HID connection closed.",
                                    0,
                                );
                            } else if !was_connected && hid_present {
                                qobject.as_mut().refresh_device();
                                let _ = qobject.as_mut().refresh_available_devices_state();
                                if *qobject.connected() {
                                    let model = qobject.model_name().to_string();
                                    let fw = qobject.firmware_version().to_string();
                                    crate::kwin::auto_switcher::send_kde_notification(
                                        "deviceConnection",
                                        "duckyPad Connected",
                                        &format!("Connected to {model} (FW v{fw})."),
                                        0,
                                    );
                                }
                            } else if was_connected && hid_present {
                                if let Some(hw_prof) = hw_active_prof {
                                    let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                                    let last_sw_ms =
                                        crate::kwin::auto_switcher::LAST_SW_PROFILE_SWITCH_MS
                                            .load(std::sync::atomic::Ordering::SeqCst);
                                    let last_sw_target =
                                        crate::kwin::auto_switcher::LAST_SW_PROFILE_TARGET
                                            .load(std::sync::atomic::Ordering::SeqCst);
                                    let in_sw_cooldown = now_ms.saturating_sub(last_sw_ms) < 1800
                                        && last_sw_target >= 1
                                        && (hw_prof != last_sw_target || hw_prof == last_sw_target);
                                    let is_paused = qobject
                                        .rust()
                                        .switcher_state
                                        .lock()
                                        .map(|sw| sw.paused_for_editing)
                                        .unwrap_or(false);

                                    let current_prof = *qobject.active_profile() as u8;
                                    let max_prof = qobject.rust().profiles.len() as u8;
                                    if hw_prof_changed
                                        && !in_sw_cooldown
                                        && !is_paused
                                        && hw_prof != current_prof
                                        && hw_prof >= 1
                                        && (max_prof == 0 || hw_prof <= max_prof)
                                    {
                                        if let Ok(mut sw) = qobject.rust().switcher_state.lock() {
                                            sw.last_switched_profile = hw_prof;
                                        }
                                        let prof_name = qobject
                                            .rust()
                                            .profiles
                                            .iter()
                                            .find(|p| p.number == hw_prof)
                                            .map(|p| p.name.clone())
                                            .unwrap_or_else(|| format!("Profile {hw_prof}"));
                                        qobject.as_mut().set_active_profile(hw_prof as i32);
                                        qobject.as_mut().sync_active_profile_selection_only();
                                        let msg = format!(
                                            "Followed hardware profile switch → Profile #{hw_prof} ({prof_name})"
                                        );
                                        record_system_event("INFO", "PROFILE", &msg);
                                        qobject.as_mut().sync_event_log_properties();
                                        qobject.as_mut().set_status_message(QString::from(&msg));
                                        qobject.as_mut().dispatch_profile_change_webhooks_if_changed(
                                            current_prof,
                                            hw_prof,
                                            "hardware_button",
                                        );
                                    }
                                }
                            } else if sd_unmounted && !was_connected {
                                qobject
                                    .as_mut()
                                    .set_model_name(QString::from("Disconnected"));
                                qobject
                                    .as_mut()
                                    .set_firmware_version(QString::from("-"));
                                qobject.as_mut().set_serial_number(QString::from("-"));
                                qobject
                                    .as_mut()
                                    .set_latest_firmware_version(QString::from("-"));
                                qobject.as_mut().set_firmware_update_available(false);
                                qobject.as_mut().set_status_message(QString::from(
                                    "duckyPad SD card unmounted / disconnected",
                                ));
                                qobject.as_mut().sync_active_profile_state();
                                qobject.as_mut().sync_tray_state();
                            }
                        });
                    }

                    std::thread::sleep(std::time::Duration::from_millis(800));
                }
            });
    }

    pub(crate) fn handle_auto_switched_profile(
        mut self: Pin<&mut Self>,
        prof: u8,
        win_class: &str,
        win_title: &str,
        hid_rejected_not_on_device: bool,
    ) {
        let win_label = if !win_class.is_empty() {
            win_class.to_string()
        } else {
            win_title.to_string()
        };

        let existing_profile_name = self
            .rust()
            .profiles
            .iter()
            .find(|p| p.number == prof)
            .map(|p| p.name.clone());

        let Some(prof_name) = existing_profile_name else {
            let err_short = format!("⚠️ Profile #{} not found (window: {})", prof, win_label);
            let err_status = format!(
                "⚠️ Auto-Switch Error: Profile #{} not found for window '{}' — update rule or reload SD",
                prof, win_label
            );
            if self.auto_switch_error_message().to_string() != err_short {
                self.as_mut()
                    .set_auto_switch_error_message(QString::from(&err_short));
                self.as_mut().set_status_message(QString::from(&err_status));
                record_system_event("ERROR", "AUTOSWITCH", err_status);
                self.as_mut().sync_event_log_properties();
            }
            self.as_mut().sync_tray_state();
            return;
        };

        let not_in_synced_snapshot = if self.profile_sync_state().to_string() == "synced" {
            false
        } else {
            let serial = self.serial_number().to_string();
            let sd_root = self.rust().sd_root_path.clone();
            if let Some(snap) = load_sd_synced_snapshot(&serial, sd_root.as_deref()) {
                !snap
                    .iter()
                    .any(|sp| sp.number == prof || sp.name.eq_ignore_ascii_case(&prof_name))
            } else {
                false
            }
        };

        let is_unsynced = hid_rejected_not_on_device || not_in_synced_snapshot;
        let prev_prof = (*self.active_profile()).clamp(1, 64) as u8;

        if is_unsynced {
            let err_short = format!(
                "⚠️ P{} ('{}') not synced to duckyPad — Sync SD to activate",
                prof, prof_name
            );
            let err_status = format!(
                "⚠️ Auto-Switch Error: Profile #{} ('{}') is not yet synced to duckyPad — click Sync SD to push to hardware",
                prof, prof_name
            );
            let err_changed = self.auto_switch_error_message().to_string() != err_short;
            if prev_prof != prof {
                self.as_mut().set_active_profile(prof as i32);
                self.as_mut().sync_active_profile_selection_only();
            }
            if err_changed {
                self.as_mut()
                    .set_auto_switch_error_message(QString::from(&err_short));
                self.as_mut().set_status_message(QString::from(&err_status));
                record_system_event("ERROR", "AUTOSWITCH", err_status);
                self.as_mut().sync_event_log_properties();
            } else {
                self.as_mut().sync_tray_state();
            }
            return;
        }

        if !self.auto_switch_error_message().is_empty() {
            self.as_mut()
                .set_auto_switch_error_message(QString::from(""));
        }

        if prev_prof != prof {
            record_system_event(
                "INFO",
                "AUTOSWITCH",
                format!(
                    "Auto-switched to Profile #{} ('{}') for window '{}' (\"{}\")",
                    prof, prof_name, win_class, win_title
                ),
            );
            self.as_mut().set_active_profile(prof as i32);
            self.as_mut().sync_active_profile_selection_only();
            self.as_mut().sync_event_log_properties();
            self.as_mut().dispatch_profile_change_webhooks_if_changed(
                prev_prof,
                prof,
                "kwin_auto_switch",
            );
        } else {
            self.as_mut().sync_tray_state();
        }
    }

    pub(crate) fn apply_auto_switch_for_current_window(mut self: Pin<&mut Self>) {
        let active_class = self.active_window_class().to_string();
        let active_title = self.active_window_title().to_string();
        if active_class.is_empty() || active_class == "Listening for KWin..." {
            return;
        }

        let mut matched_now: Option<u8> = None;
        let mut hid_rejected = false;
        let mut modifier_override_active = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if sw.enabled && !sw.paused_for_editing {
                    matched_now = sw.match_profile_for_window(&active_class, &active_title);
                    if let Some(target) = matched_now {
                        sw.last_window_matched_profile = Some(target);
                        if sw.active_modifier_override.is_some() {
                            modifier_override_active = true;
                        } else {
                            match send_hid_switch_profile_checked(target) {
                                HidSwitchOutcome::Switched => {
                                    sw.last_switched_profile = target;
                                }
                                HidSwitchOutcome::NotLoadedOnHardware => {
                                    hid_rejected = true;
                                }
                                HidSwitchOutcome::DeviceUnavailable => {}
                            }
                        }
                    }
                }
            }
        }

        if !modifier_override_active {
            if let Some(prof) = matched_now {
                self.as_mut().handle_auto_switched_profile(
                    prof,
                    &active_class,
                    &active_title,
                    hid_rejected,
                );
            }
        }
        crate::kwin::auto_switcher::dispatch_window_switch_to_secondary_devices(
            &active_class,
            &active_title,
            false,
        );
    }
}
