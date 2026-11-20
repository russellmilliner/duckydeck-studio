use crate::bridge::device_controller::{
    is_demo_showcase_env, is_ui_test_suite_env, qobject, record_system_event, DeviceControllerRust,
};
use crate::hardware::storage::{
    find_mounted_duckypad_sd, instantiate_profile_from_template, load_all_sd_profiles,
    save_profile_order_to_sd, sync_profiles_to_device_cache, DuckyProfile, MAX_INPUT_SLOTS,
};
use crate::kwin::auto_switcher::{
    set_kde_autostart_enabled, sync_local_desktop_and_icon, AutoSwitchRule, SwitcherState,
};
use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

impl qobject::DeviceController {
    pub fn apply_ui_text_scale(mut self: Pin<&mut Self>, scale_percent: i32) {
        self.as_mut()
            .apply_area_text_scale(QString::from("all"), scale_percent);
    }

    pub fn apply_area_text_scale(mut self: Pin<&mut Self>, _area: QString, scale_percent: i32) {
        let clamped = scale_percent.clamp(75, 160);

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.ui_text_scale = clamped;
                sw.sidebar_text_scale = clamped;
                sw.canvas_text_scale = clamped;
                sw.inspector_text_scale = clamped;
                sw.editor_text_scale = clamped;
                sw.status_bar_text_scale = clamped;
                sw.event_log_text_scale = clamped;
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving UI text scale to disk: {e}"),
                    );
                }
            }
        }

        qobject::duckydeck_apply_ui_font_scale(clamped);
        self.as_mut().set_ui_text_scale(clamped);
        self.as_mut().set_sidebar_text_scale(clamped);
        self.as_mut().set_canvas_text_scale(clamped);
        self.as_mut().set_inspector_text_scale(clamped);
        self.as_mut().set_editor_text_scale(clamped);
        self.as_mut().set_status_bar_text_scale(clamped);
        self.as_mut().set_event_log_text_scale(clamped);

        self.as_mut().set_status_message(QString::from(&format!(
            "Application zoom set to {clamped}% (saved)"
        )));
    }

    pub fn reset_all_text_scales(mut self: Pin<&mut Self>) {
        self.as_mut()
            .apply_area_text_scale(QString::from("all"), 100);
        self.as_mut()
            .set_status_message(QString::from("Application zoom reset to 100% (saved)"));
    }

    pub fn zoom_in_ui_text_scale(mut self: Pin<&mut Self>) -> i32 {
        let current = *self.ui_text_scale();
        let next = (current + 5).clamp(75, 160);
        self.as_mut().apply_ui_text_scale(next);
        next
    }

    pub fn zoom_out_ui_text_scale(mut self: Pin<&mut Self>) -> i32 {
        let current = *self.ui_text_scale();
        let next = (current - 5).clamp(75, 160);
        self.as_mut().apply_ui_text_scale(next);
        next
    }

    pub fn reload_zoom_level_from_disk(mut self: Pin<&mut Self>) -> i32 {
        let restored = SwitcherState::load_config_from_disk()
            .map(|c| c.ui_text_scale.clamp(75, 160))
            .unwrap_or(100);
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.ui_text_scale = restored;
                sw.sidebar_text_scale = restored;
                sw.canvas_text_scale = restored;
                sw.inspector_text_scale = restored;
                sw.editor_text_scale = restored;
                sw.status_bar_text_scale = restored;
                sw.event_log_text_scale = restored;
            }
        }
        qobject::duckydeck_apply_ui_font_scale(restored);
        self.as_mut().set_ui_text_scale(restored);
        self.as_mut().set_sidebar_text_scale(restored);
        self.as_mut().set_canvas_text_scale(restored);
        self.as_mut().set_inspector_text_scale(restored);
        self.as_mut().set_editor_text_scale(restored);
        self.as_mut().set_status_bar_text_scale(restored);
        self.as_mut().set_event_log_text_scale(restored);
        restored
    }

    pub fn send_key_to_active_window(&self, qt_key: i32, qt_modifiers: i32, text: QString) -> bool {
        qobject::duckydeck_send_key_to_active_window(qt_key, qt_modifiers, &text)
    }

    pub fn apply_autostart_enabled(mut self: Pin<&mut Self>, enabled: bool) -> bool {
        match set_kde_autostart_enabled(enabled) {
            Ok(path) => {
                self.as_mut().set_autostart_enabled(enabled);
                self.as_mut().sync_tray_state();
                let msg = if enabled {
                    format!(
                        "Added DuckyDeck Studio to KDE System Autostart ({})",
                        path.display()
                    )
                } else {
                    "Removed DuckyDeck Studio from KDE System Autostart".to_string()
                };
                self.as_mut().set_status_message(QString::from(&msg));
                true
            }
            Err(err) => {
                self.as_mut().set_status_message(QString::from(&err));
                false
            }
        }
    }

    pub fn toggle_autostart_from_tray(mut self: Pin<&mut Self>) {
        let next = !*self.autostart_enabled();
        self.as_mut().apply_autostart_enabled(next);
    }

    pub fn toggle_auto_switch_from_tray(mut self: Pin<&mut Self>) {
        let next = !*self.auto_switch_enabled();
        let fb = *self.default_fallback_profile();
        self.as_mut().set_auto_switch_config(next, fb);
        let msg = if next {
            "Auto-Switcher enabled from taskbar tray"
        } else {
            "Auto-Switcher disabled from taskbar tray"
        };
        self.as_mut().set_status_message(QString::from(msg));
        crate::kwin::auto_switcher::send_kde_notification(
            "autoSwitcherState",
            if next {
                "Window Auto-Switcher Enabled"
            } else {
                "Window Auto-Switcher Paused"
            },
            if next {
                "Automatic duckyPad profile switching on active window focus is now active."
            } else {
                "Automatic duckyPad profile switching is now paused."
            },
            0,
        );
    }

    pub fn apply_tray_icon_mode(mut self: Pin<&mut Self>, mode: QString) {
        let normalized = match mode.to_string().trim().to_ascii_lowercase().as_str() {
            "dark" => "dark".to_string(),
            "light" => "light".to_string(),
            _ => "auto".to_string(),
        };
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.tray_icon_mode = normalized.clone();
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving tray icon mode: {e}"),
                    );
                }
            }
        }
        self.as_mut().set_tray_icon_mode(QString::from(&normalized));
        let use_light_on_dark_icon = match normalized.as_str() {
            "dark" => true,
            "light" => false,
            _ => *self.system_dark_theme(),
        };
        sync_local_desktop_and_icon(use_light_on_dark_icon);
        self.as_mut().sync_tray_state();
        let label = match normalized.as_str() {
            "dark" => "Dark Mode Icon (Light-on-Dark)",
            "light" => "Light Mode Icon (Dark-on-Light)",
            _ => "Auto (Match KDE Theme)",
        };
        self.as_mut().set_status_message(QString::from(&format!(
            "Taskbar & Tray icon theme set to: {label}"
        )));
    }

    pub fn apply_app_theme_mode(mut self: Pin<&mut Self>, mode: QString) {
        let normalized = match mode.to_string().trim().to_ascii_lowercase().as_str() {
            "dark" => "dark".to_string(),
            "light" => "light".to_string(),
            _ => "system".to_string(),
        };
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.app_theme_mode = normalized.clone();
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving app theme mode: {e}"),
                    );
                }
            }
        }
        let qmode = QString::from(&normalized);
        self.as_mut().set_app_theme_mode(qmode.clone());
        qobject::duckydeck_apply_app_theme(&qmode);
        let label = match normalized.as_str() {
            "dark" => "Dark Theme (Breeze Dark)",
            "light" => "Light Theme (Breeze Light)",
            _ => "System Theme",
        };
        self.as_mut()
            .set_status_message(QString::from(&format!("Application theme set to: {label}")));
    }

    pub fn set_close_to_tray_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.close_to_tray = enabled;
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving close_to_tray setting: {e}"),
                    );
                }
            }
        }
        self.as_mut().set_close_to_tray(enabled);
    }

    pub fn notify_system_dark_theme(mut self: Pin<&mut Self>, is_dark: bool) {
        let saved_theme = self.app_theme_mode().to_string();
        if saved_theme == "dark" || saved_theme == "light" {
            let qmode = QString::from(&saved_theme);
            qobject::duckydeck_apply_app_theme(&qmode);
        }
        if *self.system_dark_theme() != is_dark {
            self.as_mut().set_system_dark_theme(is_dark);
        }
        if self.tray_icon_mode().to_string() == "auto" {
            sync_local_desktop_and_icon(is_dark);
        }
        self.as_mut().sync_tray_state();
    }

    pub fn minimize_to_tray(mut self: Pin<&mut Self>) {
        self.as_mut().sync_tray_state();
        qobject::duckydeck_hide_main_window();
    }

    pub fn show_from_tray(mut self: Pin<&mut Self>) {
        self.as_mut().sync_tray_state();
        qobject::duckydeck_show_main_window();
    }

    pub fn quit_application(self: Pin<&mut Self>) {
        qobject::duckydeck_quit_app();
    }

    pub fn restart_application(self: Pin<&mut Self>) {
        qobject::duckydeck_restart_app();
    }

    pub fn toggle_oled_desktop_widget(self: Pin<&mut Self>) {
        qobject::duckydeck_toggle_oled_widget();
    }

    pub fn set_oled_widget_keep_above(self: Pin<&mut Self>, keep_above: bool) {
        qobject::duckydeck_save_oled_widget_appearance();
        crate::kwin::auto_switcher::apply_kwin_oled_keep_above(keep_above, false);
    }

    pub fn restore_oled_widget_placement(self: Pin<&mut Self>, keep_above: bool) {
        qobject::duckydeck_restore_oled_widget_geometry();
        crate::kwin::auto_switcher::apply_kwin_oled_keep_above(keep_above, true);
    }

    pub fn scrape_app_shortcuts_json(&self, app_name: QString) -> QString {
        let app = app_name.to_string();
        let matches = crate::ai::shortcut_scraper::scrape_kde_shortcuts(&app);
        let items: Vec<serde_json::Value> = matches
            .into_iter()
            .filter_map(|(action, shortcut)| {
                let ducky = crate::ai::shortcut_scraper::kde_shortcut_to_duckyscript(&shortcut)?;
                Some(serde_json::json!({
                    "action": action,
                    "shortcut": shortcut,
                    "duckyscript": ducky,
                }))
            })
            .collect();
        QString::from(&serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string()))
    }

    pub fn generate_ai_macro(mut self: Pin<&mut Self>, app_name: QString, prompt: QString) {
        self.as_mut().set_ai_busy(true);
        let app = app_name.to_string();
        let raw_prompt = prompt.to_string();

        // Support optional "[SHORTCUT:action|shortcut]" prefix from the UI picker while keeping
        // plain prompts working seamlessly.
        let (picked_action_key, user_goal) =
            if let Some(rest) = raw_prompt.strip_prefix("[SHORTCUT:") {
                if let Some((meta, goal)) = rest.split_once("]\n") {
                    let picked = meta
                        .split_once('|')
                        .map(|(a, k)| (a.trim().to_string(), k.trim().to_string()));
                    (picked, goal.trim().to_string())
                } else {
                    (None, raw_prompt)
                }
            } else {
                (None, raw_prompt)
            };

        let local_shortcuts = crate::ai::shortcut_scraper::scrape_kde_shortcuts(&app);
        let goal_lower = user_goal.to_lowercase();

        let selected = picked_action_key.or_else(|| {
            local_shortcuts
                .iter()
                .find(|(action, key)| {
                    (!action.is_empty() && goal_lower.contains(&action.to_lowercase()))
                        || (!key.is_empty() && goal_lower.contains(&key.to_lowercase()))
                })
                .cloned()
                .or_else(|| local_shortcuts.first().cloned())
        });

        let mut shortcut_comment = String::new();
        if !local_shortcuts.is_empty() {
            shortcut_comment.push_str(
                "// Scraped from ~/.config/kglobalshortcutsrc (all matching shortcuts):\n",
            );
            for (action, key) in &local_shortcuts {
                if let Some(ds) = crate::ai::shortcut_scraper::kde_shortcut_to_duckyscript(key) {
                    shortcut_comment.push_str(&format!("//   {action}: {key} -> {ds}\n"));
                }
            }
        }

        let (selected_label, ducky_combo) = if let Some((action, key)) = selected {
            let combo = crate::ai::shortcut_scraper::kde_shortcut_to_duckyscript(&key)
                .unwrap_or_else(|| "CONTROL SHIFT m".to_string());
            shortcut_comment.push_str(&format!(
                "// Selected shortcut: {action} ({key}) -> {combo}\n"
            ));
            (action, combo)
        } else {
            (
                "Running Macro...".to_string(),
                "CONTROL SHIFT m".to_string(),
            )
        };

        let oled_line: String = selected_label.chars().take(18).collect();
        let preview = format!(
            "// Macro Template Generator (experimental) for: {app}\n// Goal: {user_goal}\n{shortcut_comment}DEFAULTDELAY 20\nSWC_SET 0 0 255 128\nOLED_CLEAR\nOLED_CURSOR 8 40\nOLED_PRINT {app}\nOLED_CURSOR 8 56\nOLED_PRINT {oled_line}\nOLED_UPDATE\n{ducky_combo}\nDELAY 150\nSWC_RESET 0\nOLED_RESTORE\n"
        );

        self.as_mut()
            .set_generated_script_preview(QString::from(&preview));
        self.as_mut().set_ai_busy(false);
        self.as_mut().set_status_message(QString::from(&format!(
            "Generated macro template for {app} ({ducky_combo})"
        )));
    }

    pub fn is_demo_showcase_mode(&self) -> bool {
        std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1")
    }

    pub fn is_ui_test_suite_mode(&self) -> bool {
        is_ui_test_suite_env()
    }

    pub fn write_ui_test_report(&self, report_json: QString, exit_code: i32) {
        let raw = report_json.to_string();
        if let Ok(path) = std::env::var("DUCKYDECK_UI_TEST_REPORT") {
            let trimmed = path.trim();
            if !trimmed.is_empty() {
                if let Err(e) = fs::write(trimmed, &raw) {
                    record_system_event(
                        "WARN",
                        "SYSTEM",
                        format!("Failed writing UI test report to {trimmed}: {e}"),
                    );
                }
            }
        }
        println!("{raw}");
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
        qobject::duckydeck_exit_app(exit_code);
        extern "C" {
            fn _exit(status: i32) -> !;
        }
        unsafe {
            _exit(exit_code);
        }
    }

    pub fn is_start_minimized_mode(&self) -> bool {
        std::env::var("DUCKYDECK_START_MINIMIZED").as_deref() == Ok("1")
    }

    pub fn startup_profile_arg(&self) -> QString {
        QString::from(
            std::env::var("DUCKYDECK_STARTUP_PROFILE")
                .unwrap_or_default()
                .trim(),
        )
    }

    pub fn is_startup_sync_sd_requested(&self) -> bool {
        std::env::var("DUCKYDECK_STARTUP_SYNC_SD").as_deref() == Ok("1")
    }

    pub fn apply_ipc_command(mut self: Pin<&mut Self>, command: QString) {
        let raw = command.to_string();
        let cmd = raw.trim();
        match cmd {
            "auto-switch:on" => {
                let fb = *self.default_fallback_profile();
                self.as_mut().set_auto_switch_config(true, fb);
                self.as_mut()
                    .set_status_message(QString::from("Auto-Switcher enabled via CLI"));
                crate::kwin::auto_switcher::send_kde_notification(
                    "autoSwitcherState",
                    "Window Auto-Switcher Enabled",
                    "Automatic duckyPad profile switching on active window focus is now active.",
                    0,
                );
            }
            "auto-switch:off" => {
                let fb = *self.default_fallback_profile();
                self.as_mut().set_auto_switch_config(false, fb);
                self.as_mut()
                    .set_status_message(QString::from("Auto-Switcher disabled via CLI"));
                crate::kwin::auto_switcher::send_kde_notification(
                    "autoSwitcherState",
                    "Window Auto-Switcher Paused",
                    "Automatic duckyPad profile switching is now paused.",
                    0,
                );
            }
            "auto-switch:toggle" => {
                self.as_mut().toggle_auto_switch_from_tray();
            }
            "next-profile" | "prev-profile" => {
                let is_next = cmd == "next-profile";
                let current_num = (*self.active_profile()).clamp(1, 64) as u8;
                let nums: Vec<u8> = self.rust().profiles.iter().map(|p| p.number).collect();
                let target_num =
                    crate::kwin::auto_switcher::cycle_profile_number(&nums, current_num, is_next);
                self.as_mut().select_profile(target_num as i32);
                let prof_name = self.active_profile_name().to_string();
                crate::kwin::auto_switcher::send_kde_notification(
                    "profileSwitched",
                    "duckyPad Profile Switched",
                    &format!("Active Profile: #{target_num} ({prof_name})"),
                    0,
                );
            }
            "sleep-device" => {
                if self.as_mut().set_sleep_state(true) {
                    self.as_mut().set_status_message(QString::from(
                        "Sent sleep command to duckyPad OLED & RGB LEDs",
                    ));
                    record_system_event(
                        "INFO",
                        "HID",
                        "duckyPad put to sleep via global shortcut / CLI",
                    );
                    self.as_mut().sync_event_log_properties();
                } else {
                    self.as_mut().set_status_message(QString::from(
                        "Failed sending sleep command to duckyPad",
                    ));
                }
            }
            "wake-device" => {
                if self.as_mut().set_sleep_state(false) {
                    self.as_mut().set_status_message(QString::from(
                        "Sent wake command to duckyPad OLED & RGB LEDs",
                    ));
                    record_system_event(
                        "INFO",
                        "HID",
                        "duckyPad woken up via global shortcut / CLI",
                    );
                    self.as_mut().sync_event_log_properties();
                } else {
                    self.as_mut().set_status_message(QString::from(
                        "Failed sending wake command to duckyPad",
                    ));
                }
            }
            "arm-window-picker" => {
                self.as_mut().arm_window_picker(false);
            }
            "reload" => {
                self.as_mut().refresh_device();
            }
            _ => {
                if let Some(path) = cmd.strip_prefix("import-group:") {
                    self.as_mut().load_profile_group(QString::from(path));
                } else if let Some(path) = cmd.strip_prefix("import-profile:") {
                    self.as_mut().import_profile_from_path(QString::from(path));
                } else if let Some(target) = cmd.strip_prefix("switch-profile:") {
                    let trimmed = target.trim();
                    let switched = if let Ok(num) = trimmed.parse::<i32>() {
                        if (1..=64).contains(&num) {
                            self.as_mut().select_profile(num)
                        } else {
                            false
                        }
                    } else {
                        self.as_mut().switch_profile_by_name(QString::from(trimmed))
                    };
                    if switched {
                        let active_num = *self.active_profile();
                        let prof_name = self.active_profile_name().to_string();
                        crate::kwin::auto_switcher::send_kde_notification(
                            "profileSwitched",
                            "duckyPad Profile Switched",
                            &format!("Active Profile: #{active_num} ({prof_name})"),
                            0,
                        );
                    }
                }
            }
        }
    }

    pub fn capture_showcase_frame(
        &self,
        output_path: QString,
        banner_title: QString,
        banner_subtitle: QString,
    ) -> bool {
        crate::bridge::showcase::capture_showcase_frame_impl(
            output_path,
            banner_title,
            banner_subtitle,
        )
    }

    pub fn demo_capture_dir(&self) -> QString {
        crate::bridge::showcase::demo_capture_dir_impl()
    }

    pub fn open_user_guide(&self) {
        let in_flatpak = Path::new("/.flatpak-info").exists();
        let system_candidates = [Path::new("/usr/share/doc/duckydeck-studio/user-guide.html")];
        let target_path = if !in_flatpak {
            system_candidates
                .iter()
                .find(|p| p.is_file())
                .map(|found| found.to_path_buf())
        } else {
            None
        }
        .unwrap_or_else(|| {
            let cache = crate::paths::host_data_dir();
            if let Err(e) = fs::create_dir_all(&cache) {
                record_system_event(
                    "WARN",
                    "SYSTEM",
                    format!("Failed creating cache dir {}: {e}", cache.display()),
                );
            }
            let extracted = cache.join("user-guide.html");
            let dest_qstr = QString::from(extracted.to_string_lossy().as_ref());
            if !qobject::duckydeck_extract_qrc_user_guide(&dest_qstr) {
                const EMBEDDED_USER_GUIDE: &str = include_str!("../../docs/user-guide.html");
                if let Err(e) = fs::write(&extracted, EMBEDDED_USER_GUIDE) {
                    record_system_event(
                        "WARN",
                        "SYSTEM",
                        format!(
                            "Failed writing embedded user guide to {}: {e}",
                            extracted.display()
                        ),
                    );
                }
            }
            extracted
        });

        if !is_ui_test_suite_env() {
            let target_str = target_path.to_string_lossy();
            if let Err(e) = crate::paths::open_url_or_path_in_os(&target_str) {
                record_system_event(
                    "WARN",
                    "SYSTEM",
                    format!("Failed opening user guide {}: {e}", target_path.display()),
                );
            }
        }
    }

    pub fn refresh_onboarding_state(mut self: Pin<&mut Self>) -> QString {
        if is_demo_showcase_env() {
            return self.onboarding_state_json().clone();
        }
        let connected = *self.connected();
        let sd_detected = *self.sd_card_detected();
        let model = self.model_name().to_string();
        let serial = self.serial_number().to_string();
        let (json_str, should_show) = {
            let rust = self.rust();
            let sw = rust
                .switcher_state
                .lock()
                .map(|g| g.clone())
                .unwrap_or_default();
            DeviceControllerRust::build_onboarding_state_json(
                connected,
                sd_detected,
                &model,
                &serial,
                &rust.profiles,
                &sw,
                rust.conflict_warning_dismissed,
            )
        };
        if self.onboarding_state_json().to_string() != json_str {
            self.as_mut()
                .set_onboarding_state_json(QString::from(&json_str));
        }
        if should_show && !*self.show_first_run_dialog() {
            self.as_mut().set_show_first_run_dialog(true);
        }
        QString::from(&json_str)
    }

    pub fn stop_conflicting_software(mut self: Pin<&mut Self>) -> bool {
        let (stopped_count, stopped_names) =
            DeviceControllerRust::stop_conflicting_legacy_processes();
        let _ = self.as_mut().refresh_onboarding_state();
        if stopped_count > 0 {
            let msg = format!(
                "✓ Shut down {} conflicting legacy process(es): {}",
                stopped_count,
                stopped_names.join(", ")
            );
            record_system_event("INFO", "SYSTEM", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            true
        } else {
            let msg = "No conflicting dekuNukem configurator or auto-switcher processes running";
            self.as_mut().set_status_message(QString::from(msg));
            false
        }
    }

    pub fn dismiss_first_run_dialog(mut self: Pin<&mut Self>) {
        DeviceControllerRust::mark_first_run_onboarding_completed();
        self.as_mut()
            .rust_mut()
            .get_mut()
            .conflict_warning_dismissed = true;
        self.as_mut().set_show_first_run_dialog(false);
        let _ = self.as_mut().refresh_onboarding_state();
    }

    pub fn apply_first_run_import(
        mut self: Pin<&mut Self>,
        profile_source_id: QString,
        rule_source_id: QString,
        shutdown_conflicts: bool,
    ) -> bool {
        let prof_src = profile_source_id.to_string().trim().to_string();
        let rule_src = rule_source_id.to_string().trim().to_string();

        let mut actions_summary = Vec::new();
        if shutdown_conflicts {
            let (stopped_count, stopped_names) =
                DeviceControllerRust::stop_conflicting_legacy_processes();
            if stopped_count > 0 {
                actions_summary.push(format!(
                    "stopped {} legacy process(es) ({})",
                    stopped_count,
                    stopped_names.join(", ")
                ));
            }
        }

        DeviceControllerRust::mark_first_run_onboarding_completed();
        self.as_mut()
            .rust_mut()
            .get_mut()
            .conflict_warning_dismissed = true;
        self.as_mut().set_show_first_run_dialog(false);

        let mut triggered_async_sd_reload = false;
        if let Some(backup_dir) = prof_src.strip_prefix("dekunukem_backup:") {
            let dir_path = Path::new(backup_dir.trim());
            if let Ok(loaded) = load_all_sd_profiles(dir_path) {
                if !loaded.is_empty() {
                    let count = loaded.len();
                    let sn = self.serial_number().to_string();
                    sync_profiles_to_device_cache(&sn, &loaded);
                    {
                        let rust = self.as_mut().rust_mut().get_mut();
                        rust.profiles = loaded;
                    }
                    self.as_mut().set_active_profile(1);
                    actions_summary.push(format!(
                        "imported {count} profile(s) from dekuNukem backup '{}'",
                        dir_path
                            .file_name()
                            .map(|f| f.to_string_lossy().to_string())
                            .unwrap_or_else(|| backup_dir.to_string())
                    ));
                }
            }
        } else if prof_src == "deck_sd" {
            if let Some(sd_dir) = self
                .rust()
                .sd_root_path
                .clone()
                .or_else(find_mounted_duckypad_sd)
            {
                let sd_str = sd_dir.to_string_lossy().to_string();
                self.as_mut()
                    .rust_mut()
                    .get_mut()
                    .pending_rule_import_source = Some(rule_src.clone());
                if self.as_mut().load_sd_directory(QString::from(&sd_str)) {
                    actions_summary.push(format!(
                        "loaded {} profile(s) from {}",
                        self.rust().profiles.len(),
                        sd_dir.display()
                    ));
                }
            } else if self.rust().profiles.len() <= 1 && *self.connected() {
                self.as_mut()
                    .rust_mut()
                    .get_mut()
                    .pending_rule_import_source = Some(rule_src.clone());
                triggered_async_sd_reload = self.as_mut().reload_profiles_from_device_sd();
                if triggered_async_sd_reload {
                    actions_summary.push("mounting duckyPad microSD to load profiles".to_string());
                }
            } else {
                actions_summary.push(format!(
                    "verified {} deck profile(s)",
                    self.rust().profiles.len()
                ));
            }
        }

        let rule_apply_res = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_opt = rust.sd_root_path.clone();
            let sn_str = rust.serial_number.to_string();
            let profs = rust.profiles.clone();
            let res = if let Ok(mut sw) = rust.switcher_state.lock() {
                DeviceControllerRust::apply_rule_source_to_switcher(
                    &mut sw,
                    &profs,
                    &rule_src,
                    sd_opt.as_deref(),
                    &sn_str,
                )
            } else {
                Err("Failed locking switcher state".to_string())
            };
            res
        };

        if let Ok(rule_count) = rule_apply_res {
            let enabled = self
                .rust()
                .switcher_state
                .lock()
                .map(|s| s.enabled)
                .unwrap_or(true);
            self.as_mut().set_auto_switch_enabled(enabled);
            actions_summary.push(format!("loaded {rule_count} auto-switch rule(s)"));
        }

        self.as_mut().sync_active_profile_state();
        let _ = self.as_mut().refresh_onboarding_state();

        if !triggered_async_sd_reload {
            let summary_msg = if actions_summary.is_empty() {
                "First-run configuration complete".to_string()
            } else {
                format!("✓ Setup complete: {}", actions_summary.join(" • "))
            };
            record_system_event("INFO", "CONFIG", &summary_msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut()
                .set_status_message(QString::from(&summary_msg));
        }
        true
    }

    pub fn search_app_shortcuts(mut self: Pin<&mut Self>, app_name: QString, allow_google: bool) {
        let app = app_name.to_string();
        self.as_mut().set_search_shortcuts_busy(true);

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let results = crate::ai::web_shortcuts::search_app_shortcuts(&app, allow_google);
            let json = serde_json::to_string(&results).unwrap_or_else(|_| "[]".to_string());
            let _ = qt_thread.queue(move |mut ctrl| {
                ctrl.as_mut()
                    .set_discovered_shortcuts_json(QString::from(&json));
                ctrl.as_mut().set_search_shortcuts_busy(false);
            });
        });
    }

    pub fn set_google_search_permission(mut self: Pin<&mut Self>, permitted: bool) {
        self.as_mut()
            .set_google_search_shortcuts_permitted(permitted);
        let rust = self.as_mut().rust_mut().get_mut();
        if let Ok(mut sw) = rust.switcher_state.lock() {
            sw.google_search_shortcuts_permitted = permitted;
            let _ = sw.save_to_disk();
        }
    }

    pub fn is_gaming_hold_script(&self, press_script: QString, release_script: QString) -> bool {
        crate::ai::web_shortcuts::is_gaming_hold_script(
            &press_script.to_string(),
            &release_script.to_string(),
        )
    }

    pub fn convert_scripts_for_gaming_hold(
        &self,
        press_script: QString,
        release_script: QString,
        enable_gaming_hold: bool,
        action_hint: QString,
    ) -> QString {
        let press_str = press_script.to_string();
        let rel_str = release_script.to_string();
        let hint_str = action_hint.to_string();
        if let Some((new_press, new_rel)) =
            crate::ai::web_shortcuts::convert_scripts_for_gaming_hold(
                &press_str,
                &rel_str,
                enable_gaming_hold,
                &hint_str,
            )
        {
            let out = json!({
                "converted": true,
                "press": new_press,
                "release": new_rel,
            });
            QString::from(&out.to_string())
        } else {
            let out = json!({
                "converted": false,
                "press": press_str,
                "release": rel_str,
            });
            QString::from(&out.to_string())
        }
    }

    pub fn apply_multi_click_interval_ms(mut self: Pin<&mut Self>, interval_ms: i32) -> bool {
        let clamped = interval_ms.clamp(80, 1000);
        self.as_mut().set_multi_click_interval_ms(clamped);
        let mut updated_slots = 0usize;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.multi_click_interval_ms = clamped;
                let _ = sw.save_to_disk();
            }
            for prof in &mut rust.profiles {
                let mut prof_changed = false;
                for slot in prof.slots.values_mut() {
                    if let Some(new_script) =
                        crate::ai::web_shortcuts::update_multi_click_interval_in_script(
                            &slot.on_press_script,
                            clamped as u32,
                        )
                    {
                        slot.on_press_script = new_script;
                        prof_changed = true;
                        updated_slots += 1;
                    }
                }
                if prof_changed {
                    if let Some(ref sd_root) = rust.sd_root_path {
                        let _ = prof.save_to_sd_root(sd_root);
                    }
                }
            }
            if updated_slots > 0 {
                let sn = rust.serial_number.to_string();
                sync_profiles_to_device_cache(&sn, &rust.profiles);
            }
        }
        if updated_slots > 0 {
            self.as_mut().sync_active_profile_state();
            self.as_mut().set_status_message(QString::from(&format!(
                "Double/Triple click window set to {} ms (updated {} multi-click key(s))",
                clamped, updated_slots
            )));
        } else {
            self.as_mut().set_status_message(QString::from(&format!(
                "Double/Triple click window set to {} ms",
                clamped
            )));
        }
        true
    }

    pub fn split_multi_click_script_json(&self, on_press_script: QString) -> QString {
        let raw = on_press_script.to_string();
        let (s1, s2, s3) = crate::ai::web_shortcuts::split_multi_click_script(&raw);
        let out = json!({
            "single": s1,
            "double": s2,
            "triple": s3,
            "hasDoubleClick": !s2.trim().is_empty(),
            "hasTripleClick": !s3.trim().is_empty(),
            "hasMultiClick": !s2.trim().is_empty() || !s3.trim().is_empty(),
        });
        QString::from(&out.to_string())
    }

    pub fn compose_multi_click_script_qstr(
        &self,
        single_script: QString,
        double_script: QString,
        triple_script: QString,
    ) -> QString {
        let interval = (*self.multi_click_interval_ms()).clamp(80, 1000) as u32;
        let composed = crate::ai::web_shortcuts::compose_multi_click_script(
            &single_script.to_string(),
            &double_script.to_string(),
            &triple_script.to_string(),
            interval,
        );
        QString::from(&composed)
    }

    pub fn assign_shortcut_to_key_slot(
        mut self: Pin<&mut Self>,
        profile_num: i32,
        slot: i32,
        action: QString,
        shortcut: QString,
        duckyscript: QString,
        category: QString,
    ) -> bool {
        let act_str = action.to_string();
        let sc_str = shortcut.to_string();
        let ds_str = duckyscript.to_string();
        let cat_str = category.to_string();
        let target_prof = if profile_num > 0 {
            profile_num as u8
        } else {
            *self.active_profile() as u8
        };
        let slot_idx = slot.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let interval_ms = (*self.multi_click_interval_ms()).clamp(80, 1000) as u32;

        let target_click_phase = if cat_str.contains(":triple_click") {
            3u8
        } else if cat_str.contains(":double_click") {
            2u8
        } else {
            1u8
        };

        let (l1, l2) = crate::ai::web_shortcuts::make_smart_labels(&act_str, &sc_str);

        let want_gaming_hold = target_click_phase == 1
            && (cat_str.contains(":gaming_hold")
                || cat_str.eq_ignore_ascii_case("Movement")
                || cat_str.eq_ignore_ascii_case("Combat")
                || act_str.to_ascii_lowercase().contains("(hold)")
                || (ds_str.contains("KEYDOWN ") && !ds_str.contains("KEYUP ")));

        let (press_script, release_script) = if want_gaming_hold {
            if !sc_str.trim().is_empty() {
                crate::ai::web_shortcuts::shortcut_to_gaming_hold_scripts(&sc_str, &act_str)
            } else if let Some(pair) = crate::ai::web_shortcuts::convert_scripts_for_gaming_hold(
                &ds_str, "", true, &act_str,
            ) {
                pair
            } else {
                (ds_str, String::new())
            }
        } else if ds_str.trim().is_empty() {
            (
                crate::ai::web_shortcuts::shortcut_to_duckyscript(&sc_str, &act_str),
                String::new(),
            )
        } else {
            (ds_str, String::new())
        };

        let updated = {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == target_prof) {
                let key_slot = prof.ensure_slot(slot_idx);
                let (mut s1, mut s2, mut s3) =
                    crate::ai::web_shortcuts::split_multi_click_script(&key_slot.on_press_script);
                match target_click_phase {
                    2 => {
                        s2 = press_script;
                        if key_slot.label_line1.trim().is_empty() {
                            key_slot.label_line1 = l1.clone();
                        }
                        if key_slot.label_line2.trim().is_empty() {
                            key_slot.label_line2 =
                                format!("2x{}", l1.chars().take(3).collect::<String>());
                        }
                    }
                    3 => {
                        s3 = press_script;
                        if key_slot.label_line1.trim().is_empty() {
                            key_slot.label_line1 = l1.clone();
                        }
                        if key_slot.label_line2.trim().is_empty() {
                            key_slot.label_line2 =
                                format!("3x{}", l1.chars().take(3).collect::<String>());
                        }
                    }
                    _ => {
                        s1 = press_script;
                        key_slot.label_line1 = l1.clone();
                        key_slot.label_line2 = l2.clone();
                        key_slot.on_release_script = release_script;
                    }
                }
                key_slot.on_press_script = crate::ai::web_shortcuts::compose_multi_click_script(
                    &s1,
                    &s2,
                    &s3,
                    interval_ms,
                );
                if !s2.trim().is_empty() || !s3.trim().is_empty() {
                    key_slot.disable_autorepeat = true;
                }

                if let Some(ref sd_root) = rust.sd_root_path {
                    let _ = prof.save_to_sd_root(sd_root);
                }
                let sn = rust.serial_number.to_string();
                sync_profiles_to_device_cache(&sn, &rust.profiles);
                true
            } else {
                false
            }
        };

        if updated {
            let active_num = *self.active_profile() as u8;
            if target_prof == active_num {
                let rust = self.as_mut().rust_mut().get_mut();
                if let Some(prof) = rust.profiles.iter().find(|p| p.number == active_num) {
                    let slots_json = DeviceControllerRust::serialize_profile_slots(prof);
                    self.as_mut()
                        .set_active_profile_slots_json(QString::from(&slots_json));
                }
            }
            let click_badge = match target_click_phase {
                2 => " [2× Double Click]",
                3 => " [3× Triple Click]",
                _ if want_gaming_hold => " [🎮 Gaming Hold]",
                _ => "",
            };
            self.as_mut().set_status_message(QString::from(&format!(
                "Assigned shortcut '{}' ({}){} to Key {}",
                act_str, sc_str, click_badge, slot_idx
            )));
        }
        updated
    }

    pub fn create_wizard_profile(mut self: Pin<&mut Self>, wizard_json: QString) -> bool {
        #[derive(serde::Deserialize)]
        struct WizardSlotInput {
            slot: usize,
            #[serde(default)]
            label1: String,
            #[serde(default)]
            label2: String,
            #[serde(default)]
            script: String,
            #[serde(default)]
            double_script: String,
            #[serde(default)]
            triple_script: String,
            #[serde(default)]
            release_script: String,
            #[serde(default)]
            gaming_hold: bool,
            #[serde(default)]
            action: String,
            #[serde(default)]
            shortcut: String,
            #[serde(default)]
            double_action: String,
            #[serde(default)]
            double_shortcut: String,
            #[serde(default)]
            triple_action: String,
            #[serde(default)]
            triple_shortcut: String,
        }

        #[derive(serde::Deserialize)]
        struct WizardProfileInput {
            name: String,
            #[serde(default)]
            template_id: Option<String>,
            #[serde(default)]
            bg_color: Option<[u8; 3]>,
            #[serde(default)]
            keydown_color: Option<[u8; 3]>,
            #[serde(default)]
            dim_unused_keys: Option<bool>,
            #[serde(default)]
            auto_switch_class: Option<String>,
            #[serde(default)]
            auto_switch_title: Option<String>,
            #[serde(default)]
            auto_switch_enabled: Option<bool>,
            #[serde(default)]
            slots: Vec<WizardSlotInput>,
        }

        let json_str = wizard_json.to_string();
        let input: WizardProfileInput = match serde_json::from_str(&json_str) {
            Ok(v) => v,
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Invalid wizard profile JSON: {e}"
                )));
                return false;
            }
        };

        let trimmed_name = input.name.trim().to_string();
        if trimmed_name.is_empty() {
            self.as_mut()
                .set_status_message(QString::from("Profile name cannot be blank"));
            return false;
        }

        let clean_name: String = trimmed_name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .take(16)
            .collect();
        if clean_name.is_empty() {
            return false;
        }

        let (target_num, is_dpp, old_to_new) = {
            let rust = self.as_mut().rust_mut().get_mut();
            if rust.profiles.len() >= 64 {
                return false;
            }
            let mut old_to_new: HashMap<u8, u8> = HashMap::with_capacity(rust.profiles.len());
            for (idx, p) in rust.profiles.iter_mut().enumerate() {
                let new_num = (idx + 1) as u8;
                old_to_new.insert(p.number, new_num);
                p.number = new_num;
            }
            let is_dpp = rust
                .sd_root_path
                .as_ref()
                .map(|r| r.join("profile_info.txt").is_file())
                .unwrap_or(true);
            let next_num = (rust.profiles.len() + 1).clamp(1, 64) as u8;
            (next_num, is_dpp, old_to_new)
        };

        let mut prof = if let Some(ref tpl) = input.template_id {
            if !tpl.trim().is_empty() && tpl != "builtin_blank" {
                instantiate_profile_from_template(tpl, target_num, &clean_name)
            } else {
                DuckyProfile {
                    number: target_num,
                    name: clean_name.clone(),
                    ..Default::default()
                }
            }
        } else {
            DuckyProfile {
                number: target_num,
                name: clean_name.clone(),
                ..Default::default()
            }
        };

        prof.number = target_num;
        prof.name = clean_name.clone();
        if is_dpp {
            prof.folder_name_override = Some(format!("profile_{clean_name}"));
        }

        if let Some(bg) = input.bg_color {
            prof.bg_color = bg;
        }
        if let Some(kd) = input.keydown_color {
            prof.keydown_color = Some(kd);
        }
        if let Some(dim) = input.dim_unused_keys {
            prof.dim_unused_keys = dim;
        }

        // Apply slots
        let interval_ms = (*self.multi_click_interval_ms()).clamp(80, 1000) as u32;
        for slot_item in input.slots {
            let s_idx = slot_item.slot.clamp(1, MAX_INPUT_SLOTS);

            let mut l1 = slot_item.label1;
            let mut l2 = slot_item.label2;
            let mut script = slot_item.script;
            let mut double_script = slot_item.double_script;
            let mut triple_script = slot_item.triple_script;
            let mut release_script = slot_item.release_script;

            if (l1.is_empty() || l2.is_empty())
                && (!slot_item.action.is_empty() || !slot_item.shortcut.is_empty())
            {
                let (gen_l1, gen_l2) = crate::ai::web_shortcuts::make_smart_labels(
                    &slot_item.action,
                    &slot_item.shortcut,
                );
                if l1.is_empty() {
                    l1 = gen_l1;
                }
                if l2.is_empty() {
                    l2 = gen_l2;
                }
            }

            if slot_item.gaming_hold
                && double_script.trim().is_empty()
                && triple_script.trim().is_empty()
            {
                if !slot_item.shortcut.trim().is_empty() {
                    let (p_ds, r_ds) = crate::ai::web_shortcuts::shortcut_to_gaming_hold_scripts(
                        &slot_item.shortcut,
                        &slot_item.action,
                    );
                    script = p_ds;
                    release_script = r_ds;
                } else if let Some((p_ds, r_ds)) =
                    crate::ai::web_shortcuts::convert_scripts_for_gaming_hold(
                        &script,
                        &release_script,
                        true,
                        &slot_item.action,
                    )
                {
                    script = p_ds;
                    release_script = r_ds;
                }
            } else if script.trim().is_empty() && !slot_item.shortcut.is_empty() {
                script = crate::ai::web_shortcuts::shortcut_to_duckyscript(
                    &slot_item.shortcut,
                    &slot_item.action,
                );
            }

            if double_script.trim().is_empty() && !slot_item.double_shortcut.trim().is_empty() {
                double_script = crate::ai::web_shortcuts::shortcut_to_duckyscript(
                    &slot_item.double_shortcut,
                    &slot_item.double_action,
                );
            }
            if triple_script.trim().is_empty() && !slot_item.triple_shortcut.trim().is_empty() {
                triple_script = crate::ai::web_shortcuts::shortcut_to_duckyscript(
                    &slot_item.triple_shortcut,
                    &slot_item.triple_action,
                );
            }

            let composed_press = crate::ai::web_shortcuts::compose_multi_click_script(
                &script,
                &double_script,
                &triple_script,
                interval_ms,
            );

            let target_slot = prof.ensure_slot(s_idx);
            target_slot.label_line1 = l1.chars().take(5).collect();
            target_slot.label_line2 = l2.chars().take(5).collect();
            target_slot.on_press_script = composed_press;
            target_slot.on_release_script = release_script;
            if !double_script.trim().is_empty() || !triple_script.trim().is_empty() {
                target_slot.disable_autorepeat = true;
            }
        }

        // Save to SD root if mounted
        let mut saved = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Some(ref sd_root) = rust.sd_root_path {
                if prof.save_to_sd_root(sd_root).is_ok() {
                    saved = true;
                    let _ = save_profile_order_to_sd(sd_root, &rust.profiles);
                }
            }
            let sn = rust.serial_number.to_string();
            rust.profiles.push(prof);
            sync_profiles_to_device_cache(&sn, &rust.profiles);
        }

        // Auto-switch rule creation & profile renumbering sync
        let auto_switch_class = input
            .auto_switch_class
            .unwrap_or_default()
            .trim()
            .to_string();
        let auto_enabled = input.auto_switch_enabled.unwrap_or(true);
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let mut rules_changed = false;
                let old_fb = sw.default_profile.unwrap_or(1);
                let mapped_fb = old_to_new
                    .get(&old_fb)
                    .copied()
                    .unwrap_or(old_fb)
                    .clamp(1, target_num);
                if sw.default_profile != Some(mapped_fb) {
                    sw.default_profile = Some(mapped_fb);
                    rules_changed = true;
                }
                for rule in &mut sw.rules {
                    if let Some(&mapped_target) = old_to_new.get(&rule.target_profile) {
                        if rule.target_profile != mapped_target {
                            rule.target_profile = mapped_target;
                            rules_changed = true;
                        }
                    }
                }
                for mrule in &mut sw.modifier_rules {
                    if let Some(&mapped_target) = old_to_new.get(&mrule.target_profile) {
                        if mrule.target_profile != mapped_target {
                            mrule.target_profile = mapped_target;
                            rules_changed = true;
                        }
                    }
                }
                if auto_enabled && !auto_switch_class.is_empty() {
                    if let Some(existing) = sw
                        .rules
                        .iter_mut()
                        .find(|r| r.resource_class.eq_ignore_ascii_case(&auto_switch_class))
                    {
                        existing.target_profile = target_num;
                        existing.enabled = true;
                    } else {
                        sw.rules.push(AutoSwitchRule {
                            enabled: true,
                            resource_class: auto_switch_class.clone(),
                            title_contains: input
                                .auto_switch_title
                                .filter(|t| !t.trim().is_empty()),
                            title_is_regex: false,
                            favicon_url: None,
                            target_profile: target_num,
                        });
                    }
                    rules_changed = true;
                }
                if rules_changed {
                    let _ = sw.save_to_disk();
                }
            }
        }

        // Select the newly created profile for editing
        crate::kwin::auto_switcher::LAST_SW_PROFILE_TARGET
            .store(target_num, std::sync::atomic::Ordering::SeqCst);
        crate::kwin::auto_switcher::LAST_SW_PROFILE_SWITCH_MS.store(
            crate::kwin::auto_switcher::current_epoch_ms(),
            std::sync::atomic::Ordering::SeqCst,
        );
        self.as_mut().set_active_profile(target_num as i32);
        self.as_mut().sync_active_profile_state();

        record_system_event(
            "INFO",
            "PROFILE",
            format!("Created Profile #{target_num} '{clean_name}' via Profile Wizard"),
        );
        self.as_mut().set_status_message(QString::from(&format!(
            "✓ Created profile '{}' (#{target_num}){}",
            clean_name,
            if saved {
                " (saved to SD)"
            } else {
                " (sync needed)"
            }
        )));

        true
    }
}
