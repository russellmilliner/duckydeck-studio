#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("app_helper.h");
        type QString = cxx_qt_lib::QString;

        fn duckydeck_sync_tray(
            controller: Pin<&mut DeviceController>,
            use_light_on_dark_icon: bool,
            icon_mode: &QString,
            status_header: &QString,
            profile_header: &QString,
            sd_space_header: &QString,
            tooltip_text: &QString,
            auto_switch_enabled: bool,
            autostart_enabled: bool,
        );
        fn duckydeck_show_main_window();
        fn duckydeck_hide_main_window();
        fn duckydeck_quit_app();
        fn duckydeck_restart_app();
        fn duckydeck_dispatch_ipc_command(command: &QString);
        fn duckydeck_toggle_oled_widget();
        fn duckydeck_restore_oled_widget_geometry();
        fn duckydeck_save_oled_widget_appearance();
        fn duckydeck_record_oled_geometry(x: i32, y: i32, width: i32, height: i32);
        fn duckydeck_apply_app_theme(theme_mode: &QString);
        fn duckydeck_run_app(start_minimized: bool) -> i32;
        fn duckydeck_capture_composite_frame(
            output_path: &QString,
            banner_title: &QString,
            banner_subtitle: &QString,
        ) -> bool;
    }

    #[auto_cxx_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool, connected)]
        #[qproperty(bool, udev_rule_missing)]
        #[qproperty(QString, model_name)]
        #[qproperty(QString, firmware_version)]
        #[qproperty(QString, serial_number)]
        #[qproperty(i32, active_profile)]
        #[qproperty(QString, active_profile_name)]
        #[qproperty(QString, sd_mount_path)]
        #[qproperty(bool, sd_card_detected)]
        #[qproperty(QString, sd_space_summary)]
        #[qproperty(QString, sd_space_detail)]
        #[qproperty(i32, sd_space_used_percent)]
        #[qproperty(QString, profiles_json)]
        #[qproperty(QString, active_profile_slots_json)]
        #[qproperty(QString, status_message)]
        #[qproperty(bool, ai_busy)]
        #[qproperty(QString, generated_script_preview)]
        #[qproperty(bool, auto_switch_enabled)]
        #[qproperty(i32, default_fallback_profile)]
        #[qproperty(i32, auto_switch_settle_ms)]
        #[qproperty(i32, auto_switch_dwell_ms)]
        #[qproperty(QString, active_window_class)]
        #[qproperty(QString, active_window_title)]
        #[qproperty(bool, window_picker_armed)]
        #[qproperty(QString, last_picked_window_class)]
        #[qproperty(QString, last_picked_window_title)]
        #[qproperty(QString, auto_switch_rules_json)]
        #[qproperty(bool, autostart_enabled)]
        #[qproperty(QString, tray_icon_mode)]
        #[qproperty(QString, app_theme_mode)]
        #[qproperty(bool, close_to_tray)]
        #[qproperty(bool, system_dark_theme)]
        #[qproperty(QString, latest_firmware_version)]
        #[qproperty(bool, firmware_update_available)]
        #[qproperty(bool, firmware_check_busy)]
        #[qproperty(QString, firmware_update_url)]
        #[qproperty(QString, firmware_check_summary)]
        #[qproperty(QString, profile_sync_state)]
        #[qproperty(QString, profile_sync_detail)]
        #[qproperty(bool, sd_sync_busy)]
        #[qproperty(i32, sd_sync_progress)]
        #[qproperty(QString, sd_sync_step)]
        #[qproperty(QString, sd_sync_log)]
        #[qproperty(QString, sd_sync_result_state)]
        #[qproperty(QString, sd_sync_result_summary)]
        #[qproperty(bool, web_api_test_busy)]
        #[qproperty(bool, web_api_last_ok)]
        #[qproperty(QString, web_api_last_result)]
        #[qproperty(QString, web_api_destinations_json)]
        #[qproperty(bool, app_launch_last_ok)]
        #[qproperty(QString, app_launch_last_result)]
        #[qproperty(QString, installed_apps_json)]
        #[qproperty(i32, ui_text_scale)]
        #[qproperty(i32, sidebar_text_scale)]
        #[qproperty(i32, canvas_text_scale)]
        #[qproperty(i32, inspector_text_scale)]
        #[qproperty(i32, editor_text_scale)]
        #[qproperty(i32, status_bar_text_scale)]
        #[qproperty(i32, event_log_text_scale)]
        #[qproperty(QString, active_profile_bg_color)]
        #[qproperty(QString, unsynced_changes_json)]
        #[qproperty(bool, has_copied_slot)]
        #[qproperty(QString, copied_slot_summary)]
        #[qproperty(QString, app_version)]
        #[qproperty(QString, app_author)]
        #[qproperty(QString, app_github_url)]
        #[qproperty(QString, log_file_path)]
        #[qproperty(QString, event_log_json)]
        #[qproperty(i32, event_log_count)]
        #[qproperty(QString, saved_colors_json)]
        #[qproperty(QString, used_colors_json)]
        type DeviceController = super::DeviceControllerRust;

        #[qinvokable]
        fn refresh_device(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn sync_profiles_to_device_sd(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        fn reload_profiles_from_device_sd(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        fn switch_profile(self: Pin<&mut DeviceController>, profile_number: i32) -> bool;

        #[qinvokable]
        fn select_profile(self: Pin<&mut DeviceController>, profile_number: i32) -> bool;

        #[qinvokable]
        fn switch_profile_by_name(self: Pin<&mut DeviceController>, profile_name: QString) -> bool;

        #[qinvokable]
        fn create_new_profile(self: Pin<&mut DeviceController>, profile_name: QString) -> bool;

        #[qinvokable]
        fn rename_profile(
            self: Pin<&mut DeviceController>,
            profile_number: i32,
            new_name: QString,
        ) -> bool;

        #[qinvokable]
        fn delete_profile(self: Pin<&mut DeviceController>, profile_number: i32) -> bool;

        #[qinvokable]
        fn reorder_profile(
            self: Pin<&mut DeviceController>,
            from_index: i32,
            to_index: i32,
        ) -> bool;

        #[qinvokable]
        fn set_key_rgb(
            self: Pin<&mut DeviceController>,
            key_index: i32,
            r: i32,
            g: i32,
            b: i32,
        ) -> bool;

        #[qinvokable]
        fn clear_key_custom_rgb(self: Pin<&mut DeviceController>, key_index: i32) -> bool;

        #[qinvokable]
        fn set_profile_bg_color(
            self: Pin<&mut DeviceController>,
            profile_number: i32,
            r: i32,
            g: i32,
            b: i32,
        ) -> bool;

        #[qinvokable]
        fn apply_ui_text_scale(self: Pin<&mut DeviceController>, scale_percent: i32);

        #[qinvokable]
        fn apply_area_text_scale(
            self: Pin<&mut DeviceController>,
            area: QString,
            scale_percent: i32,
        );

        #[qinvokable]
        fn reset_all_text_scales(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn refresh_unsynced_changes(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        fn sync_rtc_clock(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        fn set_sleep_state(self: Pin<&mut DeviceController>, sleep: bool) -> bool;

        #[qinvokable]
        fn install_udev_rules(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn read_pgv(self: Pin<&mut DeviceController>, pgv_index: i32) -> i32;

        #[qinvokable]
        fn write_pgv(self: Pin<&mut DeviceController>, pgv_index: i32, value: i32) -> bool;

        #[qinvokable]
        fn dump_all_pgvs_json(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        fn scan_mounted_sd_card(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        fn load_sd_directory(self: Pin<&mut DeviceController>, path: QString) -> bool;

        #[qinvokable]
        fn get_slot_script(self: &DeviceController, slot_number: i32, is_release: bool) -> QString;

        #[qinvokable]
        fn get_slot_meta_json(self: &DeviceController, slot_number: i32) -> QString;

        #[qinvokable]
        fn check_script_syntax(
            self: &DeviceController,
            script: QString,
            allow_abort: bool,
            disable_autorepeat: bool,
        ) -> QString;

        #[qinvokable]
        fn save_slot_config(
            self: Pin<&mut DeviceController>,
            slot_number: i32,
            label1: QString,
            label2: QString,
            on_press_script: QString,
            on_release_script: QString,
            allow_abort: bool,
            disable_autorepeat: bool,
        ) -> bool;

        #[qinvokable]
        fn save_active_profile_to_sd(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        fn start_kwin_listener(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn set_auto_switch_config(
            self: Pin<&mut DeviceController>,
            enabled: bool,
            default_profile: i32,
        );

        #[qinvokable]
        fn set_auto_switch_timings(
            self: Pin<&mut DeviceController>,
            settle_ms: i32,
            dwell_ms: i32,
        );

        #[qinvokable]
        fn set_script_editor_open(self: Pin<&mut DeviceController>, open: bool);

        #[qinvokable]
        fn arm_window_picker(self: Pin<&mut DeviceController>, bind_to_active_profile: bool);

        #[qinvokable]
        fn add_or_update_auto_switch_rule(
            self: Pin<&mut DeviceController>,
            rule_index: i32,
            enabled: bool,
            resource_class: QString,
            title_contains: QString,
            target_profile: i32,
        ) -> bool;

        #[qinvokable]
        fn save_auto_switch_rule_with_favicon(
            self: Pin<&mut DeviceController>,
            rule_index: i32,
            enabled: bool,
            resource_class: QString,
            title_contains: QString,
            favicon_url: QString,
            target_profile: i32,
        ) -> bool;

        #[qinvokable]
        fn remove_auto_switch_rule(self: Pin<&mut DeviceController>, rule_index: i32) -> bool;

        #[qinvokable]
        fn toggle_auto_switch_rule(
            self: Pin<&mut DeviceController>,
            rule_index: i32,
            enabled: bool,
        ) -> bool;

        #[qinvokable]
        fn reorder_auto_switch_rule(
            self: Pin<&mut DeviceController>,
            from_index: i32,
            to_index: i32,
        ) -> bool;

        #[qinvokable]
        fn is_browser_window_class(self: &DeviceController, resource_class: QString) -> bool;

        #[qinvokable]
        fn clean_browser_window_title(self: &DeviceController, title: QString) -> QString;

        #[qinvokable]
        fn apply_autostart_enabled(self: Pin<&mut DeviceController>, enabled: bool) -> bool;

        #[qinvokable]
        fn toggle_autostart_from_tray(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn toggle_auto_switch_from_tray(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn apply_tray_icon_mode(self: Pin<&mut DeviceController>, mode: QString);

        #[qinvokable]
        fn apply_app_theme_mode(self: Pin<&mut DeviceController>, mode: QString);

        #[qinvokable]
        fn export_profile(
            self: Pin<&mut DeviceController>,
            profile_number: i32,
            destination_path: QString,
        ) -> bool;

        #[qinvokable]
        fn export_active_profile(
            self: Pin<&mut DeviceController>,
            destination_path: QString,
        ) -> bool;

        #[qinvokable]
        fn import_profile_from_path(
            self: Pin<&mut DeviceController>,
            source_path: QString,
        ) -> bool;

        #[qinvokable]
        fn save_profile_group(
            self: Pin<&mut DeviceController>,
            destination_path: QString,
            group_name: QString,
        ) -> bool;

        #[qinvokable]
        fn load_profile_group(
            self: Pin<&mut DeviceController>,
            source_path: QString,
        ) -> bool;

        #[qinvokable]
        fn create_new_profile_group(
            self: Pin<&mut DeviceController>,
            group_name: QString,
            initial_profile_name: QString,
        ) -> bool;

        #[qinvokable]
        fn reboot_device(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        fn copy_slot(self: Pin<&mut DeviceController>, slot_number: i32) -> bool;

        #[qinvokable]
        fn paste_slot(self: Pin<&mut DeviceController>, target_slot_number: i32) -> bool;

        #[qinvokable]
        fn clear_slot(self: Pin<&mut DeviceController>, slot_number: i32) -> bool;

        #[qinvokable]
        fn move_or_swap_slot(
            self: Pin<&mut DeviceController>,
            from_slot: i32,
            to_slot: i32,
            shift_between: bool,
        ) -> bool;

        #[qinvokable]
        fn set_close_to_tray_enabled(self: Pin<&mut DeviceController>, enabled: bool);

        #[qinvokable]
        fn notify_system_dark_theme(self: Pin<&mut DeviceController>, is_dark: bool);

        #[qinvokable]
        fn minimize_to_tray(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn show_from_tray(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn quit_application(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn restart_application(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn toggle_oled_desktop_widget(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn set_oled_widget_keep_above(self: Pin<&mut DeviceController>, keep_above: bool);

        #[qinvokable]
        fn restore_oled_widget_placement(self: Pin<&mut DeviceController>, keep_above: bool);

        #[qinvokable]
        fn generate_ai_macro(
            self: Pin<&mut DeviceController>,
            app_name: QString,
            prompt: QString,
        );

        #[qinvokable]
        fn check_firmware_update(self: Pin<&mut DeviceController>, manual: bool);

        #[qinvokable]
        fn open_firmware_update_page(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn download_firmware_binary(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn extract_script_web_api_json(
            self: &DeviceController,
            script: QString,
            slot_number: i32,
        ) -> QString;

        #[qinvokable]
        fn apply_web_api_to_script(
            self: &DeviceController,
            existing_script: QString,
            config_json: QString,
            slot_number: i32,
            is_release: bool,
        ) -> QString;

        #[qinvokable]
        fn test_web_api_call(
            self: Pin<&mut DeviceController>,
            config_json: QString,
            slot_number: i32,
        );

        #[qinvokable]
        #[cxx_name = "reloadWebApiDestinations"]
        fn reload_web_api_destinations(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        #[cxx_name = "saveWebApiDestination"]
        fn save_web_api_destination(
            self: Pin<&mut DeviceController>,
            name: QString,
            method: QString,
            url: QString,
            headers: QString,
        ) -> QString;

        #[qinvokable]
        #[cxx_name = "deleteWebApiDestination"]
        fn delete_web_api_destination(
            self: Pin<&mut DeviceController>,
            name: QString,
        ) -> QString;

        #[qinvokable]
        fn refresh_installed_apps(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        fn extract_script_app_launch_json(
            self: &DeviceController,
            script: QString,
            slot_number: i32,
        ) -> QString;

        #[qinvokable]
        fn apply_app_launch_to_script(
            self: &DeviceController,
            existing_script: QString,
            config_json: QString,
            slot_number: i32,
            is_release: bool,
        ) -> QString;

        #[qinvokable]
        fn test_app_launch(
            self: Pin<&mut DeviceController>,
            config_json: QString,
            slot_number: i32,
        );

        #[qinvokable]
        fn log_event(
            self: Pin<&mut DeviceController>,
            level: QString,
            category: QString,
            message: QString,
        );

        #[qinvokable]
        fn record_status_log(self: Pin<&mut DeviceController>, message: QString);

        #[qinvokable]
        fn refresh_event_log(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        fn clear_event_log(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn export_event_log(self: Pin<&mut DeviceController>, destination_path: QString) -> bool;

        #[qinvokable]
        fn format_event_log_text(
            self: &DeviceController,
            category_filter: QString,
            level_filter: QString,
            search_query: QString,
        ) -> QString;

        #[qinvokable]
        fn refresh_color_palettes(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn save_palette_color(self: Pin<&mut DeviceController>, hex_color: QString);

        #[qinvokable]
        fn remove_palette_color(self: Pin<&mut DeviceController>, hex_color: QString);

        #[qinvokable]
        fn is_demo_showcase_mode(self: &DeviceController) -> bool;

        #[qinvokable]
        fn is_start_minimized_mode(self: &DeviceController) -> bool;

        #[qinvokable]
        fn startup_profile_arg(self: &DeviceController) -> QString;

        #[qinvokable]
        fn is_startup_sync_sd_requested(self: &DeviceController) -> bool;

        #[qinvokable]
        fn apply_ipc_command(self: Pin<&mut DeviceController>, command: QString);

        #[qinvokable]
        fn capture_showcase_frame(
            self: &DeviceController,
            output_path: QString,
            banner_title: QString,
            banner_subtitle: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "discardUnsyncedChange"]
        fn discard_unsynced_change(
            self: Pin<&mut DeviceController>,
            profile_number: i32,
            profile_name: QString,
            change_key: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "startGlobalMacroCapture"]
        fn start_global_macro_capture(self: Pin<&mut DeviceController>) -> i32;

        #[qinvokable]
        #[cxx_name = "stopGlobalMacroCapture"]
        fn stop_global_macro_capture(self: Pin<&mut DeviceController>);

        #[qinvokable]
        #[cxx_name = "pollGlobalMacroEventsJson"]
        fn poll_global_macro_events_json(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        #[cxx_name = "pushHardwareLedFrameJson"]
        fn push_hardware_led_frame_json(self: &DeviceController, frame_json: QString) -> bool;

        #[qinvokable]
        #[cxx_name = "getProfilePropertiesJson"]
        fn get_profile_properties_json(self: &DeviceController, profile_number: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "saveProfilePropertiesJson"]
        fn save_profile_properties_json(
            self: Pin<&mut DeviceController>,
            profile_number: i32,
            props_json: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "oledLabelToHardwareAscii"]
        fn oled_label_to_hardware_ascii_qstr(self: &DeviceController, label: QString) -> QString;

        #[qinvokable]
        #[cxx_name = "listAvailableFirmwaresJson"]
        fn list_available_firmwares_json(self: &DeviceController) -> QString;

        #[qinvokable]
        #[cxx_name = "installFirmwareToDevice"]
        fn install_firmware_to_device(
            self: Pin<&mut DeviceController>,
            firmware_path: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "refreshSdMountStatus"]
        fn refresh_sd_mount_status(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        #[cxx_name = "mountSdCard"]
        fn mount_sd_card(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        #[cxx_name = "unmountSdCard"]
        fn unmount_sd_card(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        #[cxx_name = "saveSlotWidgetLabels"]
        fn save_slot_widget_labels(
            self: Pin<&mut DeviceController>,
            slot_number: i32,
            widget_label1: QString,
            widget_label2: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "setActiveProfileOledRotation"]
        fn set_active_profile_oled_rotation(
            self: Pin<&mut DeviceController>,
            degrees: i32,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "setActiveProfileWidgetRotation"]
        fn set_active_profile_widget_rotation(
            self: Pin<&mut DeviceController>,
            degrees: i32,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "listProfileTemplatesJson"]
        fn list_profile_templates_json(self: &DeviceController) -> QString;

        #[qinvokable]
        #[cxx_name = "saveProfileAsTemplate"]
        fn save_profile_as_template(
            self: Pin<&mut DeviceController>,
            profile_number: i32,
            template_name: QString,
            description: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "deleteProfileTemplate"]
        fn delete_profile_template(
            self: Pin<&mut DeviceController>,
            template_id: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "createProfileFromTemplate"]
        fn create_profile_from_template(
            self: Pin<&mut DeviceController>,
            profile_name: QString,
            template_id: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "applyTemplateToProfile"]
        fn apply_template_to_profile(
            self: Pin<&mut DeviceController>,
            profile_number: i32,
            template_id: QString,
        ) -> bool;
    }

    impl cxx_qt::Threading for DeviceController {}
}

use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use hidapi::{HidApi, HidDevice};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use crate::hardware::storage::{
    apply_app_launch_config_to_script, apply_web_api_config_to_script,
    check_latest_duckypad_firmware, check_profile_sync_status, compute_profiles_fingerprint,
    compute_unsynced_changes_json, delete_user_profile_template,
    delete_web_api_destination_by_name, detect_unmounted_duckypad_sd,
    discard_unsynced_change_in_profiles, encode_pgv31_trigger, execute_app_launch,
    execute_web_api_request, extract_app_launch_config, extract_web_api_config,
    find_duckypad_msc_block_device, find_mounted_duckypad_sd, flash_slot_led_feedback,
    get_sd_card_space_info, has_app_launch_config, has_web_api_config,
    instantiate_profile_from_template, list_all_profile_templates,
    list_available_duckypad_pro_firmwares, load_all_sd_profiles,
    load_profiles_for_connected_device, load_web_api_destinations, mark_profiles_sd_synced,
    mount_duckypad_sd_only, oled_label_to_hardware_ascii, poll_and_clear_pgv31_trigger,
    poll_hardware_active_profile, profile_cache_dir_for_serial, query_connected_duckypad_info,
    read_dpp_config, read_duckypad_pro_profiles_via_msc,
    read_duckypad_pro_profiles_via_msc_with_progress, reboot_duckypad, save_profile_as_user_template,
    save_profile_order_to_sd, scan_installed_desktop_apps,
    seed_web_api_destinations_from_profiles, set_batch_key_leds_hid, set_single_key_led_hid,
    stage_duckypad_pro_firmware_via_msc_with_progress, sync_profile_web_api_triggers,
    sync_profiles_to_cache, sync_profiles_to_device_cache, unmount_mounted_duckypad_sd,
    upsert_web_api_destination, write_duckypad_pro_profiles_via_msc_with_progress,
    write_profiles_to_sd_root, AppLaunchConfig, DuckyKeySlot, DuckyProfile, DuckypadEvdevListener,
    GlobalMacroEvdevRecorder, WebApiConfig, DUCKYPAD_VID, HID_CMD_SET_LED_SINGLE,
    HID_REPORT_ID_OUT, MAX_INPUT_SLOTS, PID_DUCKYPAD_2020, PID_DUCKYPAD_PRO,
};
use crate::kwin::auto_switcher::{
    is_kde_autostart_enabled, is_web_browser_class, resolve_rule_icon, set_kde_autostart_enabled,
    strip_browser_title_suffix, sync_local_desktop_and_icon, AutoSwitchRule, SwitcherState,
};

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "Russell Milliner";
pub const APP_GITHUB_URL: &str = "https://github.com/russellmilliner/duckydeck-studio";

pub fn is_demo_showcase_env() -> bool {
    std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLogEntry {
    pub timestamp: String,
    pub level: String,
    pub category: String,
    pub message: String,
}

static SYSTEM_EVENT_LOG: OnceLock<Mutex<Vec<SystemLogEntry>>> = OnceLock::new();

pub fn get_system_log_file_path() -> PathBuf {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(|| PathBuf::from("/tmp"));
    let dir = base.join("duckydeck-studio");
    let _ = fs::create_dir_all(&dir);
    dir.join("duckydeck.log")
}

fn system_event_log_store() -> &'static Mutex<Vec<SystemLogEntry>> {
    SYSTEM_EVENT_LOG.get_or_init(|| Mutex::new(Vec::with_capacity(256)))
}

pub fn record_system_event(level: &str, category: &str, message: impl AsRef<str>) {
    let msg = message.as_ref().trim();
    if msg.is_empty() {
        return;
    }
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let lvl = level.trim().to_ascii_uppercase();
    let cat = category.trim().to_ascii_uppercase();

    if let Ok(mut guard) = system_event_log_store().lock() {
        if let Some(last) = guard.last() {
            if last.message == msg && last.category == cat && last.level == lvl {
                return;
            }
        }
        guard.push(SystemLogEntry {
            timestamp: ts.clone(),
            level: lvl.clone(),
            category: cat.clone(),
            message: msg.to_string(),
        });
        if guard.len() > 1000 {
            let drain_count = guard.len() - 1000;
            guard.drain(0..drain_count);
        }
    }

    if is_demo_showcase_env() {
        return;
    }

    let log_path = get_system_log_file_path();
    if let Ok(meta) = fs::metadata(&log_path) {
        if meta.len() > 2 * 1024 * 1024 {
            let backup = log_path.with_extension("log.1");
            let _ = fs::rename(&log_path, backup);
        }
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = writeln!(f, "[{ts}] [{lvl:<5}] [{cat:<10}] {msg}");
    }
}

pub fn classify_status_message(msg: &str) -> (&'static str, &'static str) {
    let lower = msg.to_ascii_lowercase();
    let level = if lower.contains("error")
        || lower.contains("failed")
        || lower.contains("invalid")
        || lower.contains("denied")
        || msg.contains('⚠')
    {
        "ERROR"
    } else if lower.contains("unsynced")
        || lower.contains("mismatch")
        || lower.contains("disconnected")
        || lower.contains("unmounted")
    {
        "WARN"
    } else {
        "INFO"
    };

    let category = if lower.contains("web api") || lower.contains("http") || msg.contains('🌐') {
        "WEB_API"
    } else if lower.contains("sd") || lower.contains("sync") || lower.contains("msc") {
        "SD_SYNC"
    } else if lower.contains("compile")
        || lower.contains(".dsb")
        || lower.contains(".txt")
        || lower.contains("slot")
        || lower.contains("key ")
    {
        "SCRIPT"
    } else if lower.contains("auto-switch")
        || lower.contains("rule")
        || lower.contains("window")
        || lower.contains("kwin")
    {
        "AUTOSWITCH"
    } else if lower.contains("profile") || lower.contains("group") {
        "PROFILE"
    } else if lower.contains("duckypad")
        || lower.contains("usb")
        || lower.contains("hid")
        || lower.contains("rtc")
        || lower.contains("firmware")
        || lower.contains("pgv")
        || lower.contains("sleep")
        || lower.contains("reboot")
    {
        "DEVICE"
    } else {
        "SYSTEM"
    };

    (level, category)
}

pub fn serialize_system_event_log_json() -> (String, i32) {
    if let Ok(guard) = system_event_log_store().lock() {
        let count = guard.len() as i32;
        let json_str = serde_json::to_string(&*guard).unwrap_or_else(|_| "[]".to_string());
        (json_str, count)
    } else {
        ("[]".to_string(), 0)
    }
}

fn default_bundle_version() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileExportBundle {
    #[serde(default = "default_bundle_version")]
    pub version: u32,
    pub profile: DuckyProfile,
    #[serde(default)]
    pub auto_switch_rules: Vec<AutoSwitchRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileGroupBundle {
    #[serde(default = "default_bundle_version")]
    pub version: u32,
    #[serde(default)]
    pub group_name: String,
    #[serde(default)]
    pub active_profile: u8,
    #[serde(default)]
    pub default_fallback_profile: Option<u8>,
    pub profiles: Vec<DuckyProfile>,
    #[serde(default)]
    pub auto_switch_rules: Vec<AutoSwitchRule>,
}

pub fn run_application(start_minimized: bool) -> i32 {
    record_system_event(
        "INFO",
        "SYSTEM",
        format!(
            "DuckyDeck Studio v{} started (start_minimized={})",
            APP_VERSION, start_minimized
        ),
    );
    sync_local_desktop_and_icon(true);
    qobject::duckydeck_run_app(start_minimized)
}

pub struct DeviceControllerRust {
    pub(crate) connected: bool,
    pub(crate) udev_rule_missing: bool,
    pub(crate) model_name: QString,
    pub(crate) firmware_version: QString,
    pub(crate) serial_number: QString,
    pub(crate) active_profile: i32,
    pub(crate) active_profile_name: QString,
    pub(crate) sd_mount_path: QString,
    pub(crate) sd_card_detected: bool,
    pub(crate) sd_space_summary: QString,
    pub(crate) sd_space_detail: QString,
    pub(crate) sd_space_used_percent: i32,
    pub(crate) profiles_json: QString,
    pub(crate) active_profile_slots_json: QString,
    pub(crate) status_message: QString,
    pub(crate) ai_busy: bool,
    pub(crate) generated_script_preview: QString,
    pub(crate) auto_switch_enabled: bool,
    pub(crate) default_fallback_profile: i32,
    pub(crate) auto_switch_settle_ms: i32,
    pub(crate) auto_switch_dwell_ms: i32,
    pub(crate) active_window_class: QString,
    pub(crate) active_window_title: QString,
    pub(crate) window_picker_armed: bool,
    pub(crate) last_picked_window_class: QString,
    pub(crate) last_picked_window_title: QString,
    pub(crate) auto_switch_rules_json: QString,
    pub(crate) autostart_enabled: bool,
    pub(crate) tray_icon_mode: QString,
    pub(crate) app_theme_mode: QString,
    pub(crate) close_to_tray: bool,
    pub(crate) system_dark_theme: bool,
    pub(crate) latest_firmware_version: QString,
    pub(crate) firmware_update_available: bool,
    pub(crate) firmware_check_busy: bool,
    pub(crate) firmware_update_url: QString,
    pub(crate) firmware_check_summary: QString,
    pub(crate) profile_sync_state: QString,
    pub(crate) profile_sync_detail: QString,
    pub(crate) sd_sync_busy: bool,
    pub(crate) sd_sync_progress: i32,
    pub(crate) sd_sync_step: QString,
    pub(crate) sd_sync_log: QString,
    pub(crate) sd_sync_result_state: QString,
    pub(crate) sd_sync_result_summary: QString,
    pub(crate) web_api_test_busy: bool,
    pub(crate) web_api_last_ok: bool,
    pub(crate) web_api_last_result: QString,
    pub(crate) web_api_destinations_json: QString,
    pub(crate) app_launch_last_ok: bool,
    pub(crate) app_launch_last_result: QString,
    pub(crate) installed_apps_json: QString,
    pub(crate) ui_text_scale: i32,
    pub(crate) sidebar_text_scale: i32,
    pub(crate) canvas_text_scale: i32,
    pub(crate) inspector_text_scale: i32,
    pub(crate) editor_text_scale: i32,
    pub(crate) status_bar_text_scale: i32,
    pub(crate) event_log_text_scale: i32,
    pub(crate) active_profile_bg_color: QString,
    pub(crate) unsynced_changes_json: QString,
    pub(crate) has_copied_slot: bool,
    pub(crate) copied_slot_summary: QString,
    pub(crate) app_version: QString,
    pub(crate) app_author: QString,
    pub(crate) app_github_url: QString,
    pub(crate) log_file_path: QString,
    pub(crate) event_log_json: QString,
    pub(crate) event_log_count: i32,
    pub(crate) saved_colors_json: QString,
    pub(crate) used_colors_json: QString,
    pub(crate) copied_key_slot: Option<DuckyKeySlot>,
    pub(crate) firmware_download_url: String,
    pub(crate) sd_root_path: Option<PathBuf>,
    pub(crate) profiles: Vec<DuckyProfile>,
    pub(crate) switcher_state: Arc<Mutex<SwitcherState>>,
    pub(crate) kwin_listener_started: bool,
    pub(crate) picker_bind_to_active_profile: bool,
}

impl Default for DeviceControllerRust {
    fn default() -> Self {
        if is_demo_showcase_env() {
            return Self::create_showcase_instance();
        }

        let mounted_sd = find_mounted_duckypad_sd();
        let connected_info = query_connected_duckypad_info().ok();

        let mut sd_root_path: Option<PathBuf> = None;
        let mut profiles: Vec<DuckyProfile> = Vec::new();

        if let Some(ref sd_dir) = mounted_sd {
            if let Ok(loaded) = load_all_sd_profiles(sd_dir) {
                if !loaded.is_empty() {
                    if let Some(ref info) = connected_info {
                        sync_profiles_to_device_cache(&info.serial, &loaded);
                        mark_profiles_sd_synced(&info.serial, &loaded);
                    } else {
                        sync_profiles_to_cache(&loaded);
                        mark_profiles_sd_synced("", &loaded);
                    }
                    sd_root_path = Some(sd_dir.clone());
                    profiles = loaded;
                }
            }
        }

        if profiles.is_empty() {
            if let Some(ref info) = connected_info {
                if let Some(loaded) = load_profiles_for_connected_device(info.pid, &info.serial) {
                    profiles = loaded;
                } else if info.pid == PID_DUCKYPAD_PRO && !info.disk_label.is_empty() {
                    if let Ok(loaded) = read_duckypad_pro_profiles_via_msc(
                        &info.disk_label,
                        &info.serial,
                        info.active_profile,
                    ) {
                        profiles = loaded;
                    }
                }
            }
        }

        if profiles.is_empty() {
            profiles = vec![Self::create_default_demo_profile()];
        }

        let initial_destinations = seed_web_api_destinations_from_profiles(&profiles);
        let destinations_json = serde_json::to_string(&initial_destinations)
            .unwrap_or_else(|_| "[]".to_string());

        if sync_profile_web_api_triggers(&mut profiles) {
            if let Some(ref info) = connected_info {
                sync_profiles_to_device_cache(&info.serial, &profiles);
            } else {
                sync_profiles_to_cache(&profiles);
            }
        }

        let mut switcher = SwitcherState::default();
        if connected_info.is_some() || sd_root_path.is_some() {
            switcher.align_rules_with_profiles(&profiles);
        }

        let (connected, model_str, fw_str, sn_str, active_prof, status_str) =
            if let Some(ref info) = connected_info {
                let model = if info.pid == PID_DUCKYPAD_PRO {
                    "duckyPad Pro"
                } else {
                    "duckyPad (2020)"
                };
                let prof = (info.active_profile as i32).clamp(1, 64);
                switcher.last_switched_profile = prof as u8;
                let count = profiles.len();
                (
                    true,
                    model.to_string(),
                    info.fw_version.clone(),
                    info.serial.clone(),
                    prof,
                    format!(
                        "Connected to {model} (FW v{}) — {count} profile(s) loaded",
                        info.fw_version
                    ),
                )
            } else if let Some(ref sd_dir) = sd_root_path {
                let dpp_cfg = read_dpp_config(sd_dir).unwrap_or_default();
                let fw = dpp_cfg.fw_ver.unwrap_or_else(|| "-".to_string());
                let sn = dpp_cfg.serial_number.unwrap_or_else(|| "-".to_string());
                if sn != "-" {
                    mark_profiles_sd_synced(&sn, &profiles);
                }
                let model = if fw != "-" {
                    "duckyPad Pro (SD Mode)"
                } else {
                    "duckyPad (SD Mode)"
                };
                let prof = dpp_cfg.last_profile.unwrap_or(1).max(1) as i32;
                (
                    false,
                    model.to_string(),
                    fw,
                    sn,
                    prof,
                    format!(
                        "Loaded {} profile(s) from {}",
                        profiles.len(),
                        sd_dir.display()
                    ),
                )
            } else {
                (
                    false,
                    "Disconnected".to_string(),
                    "-".to_string(),
                    "-".to_string(),
                    1,
                    "Click Scan USB HID or Load SD Card to configure duckyPad Pro / 2020"
                        .to_string(),
                )
            };

        let auto_enabled = switcher.enabled;
        let fallback_prof = switcher.default_profile.unwrap_or(0) as i32;
        let settle_ms = switcher.mouse_settle_ms.clamp(0, 5000);
        let dwell_ms = switcher.window_dwell_ms.clamp(0, 5000);
        let tray_mode = switcher.tray_icon_mode.clone();
        let app_theme = switcher.app_theme_mode.clone();
        let close_tray = switcher.close_to_tray;
        let text_scale = switcher.ui_text_scale.clamp(75, 160);
        let sidebar_scale = switcher.sidebar_text_scale.clamp(75, 160);
        let canvas_scale = switcher.canvas_text_scale.clamp(75, 160);
        let inspector_scale = switcher.inspector_text_scale.clamp(75, 160);
        let editor_scale = switcher.editor_text_scale.clamp(75, 160);
        let status_bar_scale = switcher.status_bar_text_scale.clamp(75, 160);
        let event_log_scale = switcher.event_log_text_scale.clamp(75, 160);
        let saved_colors_json =
            serde_json::to_string(&switcher.saved_colors).unwrap_or_else(|_| "[]".to_string());
        let used_colors_json = Self::collect_used_colors_json(&profiles);
        let autostart = is_kde_autostart_enabled();
        if autostart {
            let _ = set_kde_autostart_enabled(true);
        }
        let rules_json = Self::serialize_rules(&switcher.rules, &profiles);
        let profiles_json = Self::serialize_profiles_summary(&profiles, &switcher.rules);
        let active_prof_obj = profiles
            .iter()
            .find(|p| p.number == active_prof as u8)
            .or_else(|| profiles.first());
        let active_prof_name = active_prof_obj
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Default".to_string());
        let active_bg_hex = active_prof_obj
            .map(|p| format!("#{:02x}{:02x}{:02x}", p.bg_color[0], p.bg_color[1], p.bg_color[2]))
            .unwrap_or_else(|| "#00aaff".to_string());
        let slots_json = active_prof_obj
            .map(Self::serialize_profile_slots)
            .unwrap_or_else(|| "[]".to_string());
        let sd_mount_str = sd_root_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let sd_detected = !sd_mount_str.is_empty() || detect_unmounted_duckypad_sd("").is_some();
        let (sync_state_str, sync_detail_str) = check_profile_sync_status(
            &sn_str,
            &profiles,
            sd_root_path.as_deref(),
            connected,
            active_prof as u8,
        );
        let unsynced_json = compute_unsynced_changes_json(
            &sn_str,
            &profiles,
            sd_root_path.as_deref(),
            &sync_state_str,
        );
        let sd_space = get_sd_card_space_info(&sn_str, sd_root_path.as_deref());

        let (init_lvl, init_cat) = classify_status_message(&status_str);
        record_system_event(init_lvl, init_cat, &status_str);
        let (init_log_json, init_log_count) = serialize_system_event_log_json();
        let log_path_str = get_system_log_file_path().to_string_lossy().to_string();

        Self {
            connected,
            udev_rule_missing: false,
            model_name: QString::from(&model_str),
            firmware_version: QString::from(&fw_str),
            serial_number: QString::from(&sn_str),
            active_profile: active_prof,
            active_profile_name: QString::from(&active_prof_name),
            sd_mount_path: QString::from(&sd_mount_str),
            sd_card_detected: sd_detected,
            sd_space_summary: QString::from(&sd_space.summary_text),
            sd_space_detail: QString::from(&sd_space.detail_tooltip),
            sd_space_used_percent: sd_space.used_percent,
            profiles_json: QString::from(&profiles_json),
            active_profile_slots_json: QString::from(&slots_json),
            status_message: QString::from(&status_str),
            ai_busy: false,
            generated_script_preview: QString::from(
                "// Slot 1: Front View\nDEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Front View\nOLED_UPDATE\nKP_1\n",
            ),
            auto_switch_enabled: auto_enabled,
            default_fallback_profile: fallback_prof,
            auto_switch_settle_ms: settle_ms,
            auto_switch_dwell_ms: dwell_ms,
            active_window_class: QString::from("Listening for KWin..."),
            active_window_title: QString::from(""),
            window_picker_armed: false,
            last_picked_window_class: QString::from(""),
            last_picked_window_title: QString::from(""),
            auto_switch_rules_json: QString::from(&rules_json),
            autostart_enabled: autostart,
            tray_icon_mode: QString::from(&tray_mode),
            app_theme_mode: QString::from(&app_theme),
            close_to_tray: close_tray,
            system_dark_theme: true,
            latest_firmware_version: QString::from("-"),
            firmware_update_available: false,
            firmware_check_busy: false,
            firmware_update_url: QString::from(
                "https://dekuNukem.github.io/duckyPad-Pro/doc/fw_update.html",
            ),
            firmware_check_summary: QString::from(""),
            profile_sync_state: QString::from(&sync_state_str),
            profile_sync_detail: QString::from(&sync_detail_str),
            sd_sync_busy: false,
            sd_sync_progress: 0,
            sd_sync_step: QString::from("Ready"),
            sd_sync_log: QString::from(""),
            sd_sync_result_state: QString::from("idle"),
            sd_sync_result_summary: QString::from(""),
            web_api_test_busy: false,
            web_api_last_ok: true,
            web_api_last_result: QString::from(""),
            web_api_destinations_json: QString::from(&destinations_json),
            app_launch_last_ok: true,
            app_launch_last_result: QString::from(""),
            installed_apps_json: QString::from("[]"),
            ui_text_scale: text_scale,
            sidebar_text_scale: sidebar_scale,
            canvas_text_scale: canvas_scale,
            inspector_text_scale: inspector_scale,
            editor_text_scale: editor_scale,
            status_bar_text_scale: status_bar_scale,
            event_log_text_scale: event_log_scale,
            active_profile_bg_color: QString::from(&active_bg_hex),
            unsynced_changes_json: QString::from(&unsynced_json),
            has_copied_slot: false,
            copied_slot_summary: QString::from(""),
            app_version: QString::from(APP_VERSION),
            app_author: QString::from(APP_AUTHOR),
            app_github_url: QString::from(APP_GITHUB_URL),
            log_file_path: QString::from(&log_path_str),
            event_log_json: QString::from(&init_log_json),
            event_log_count: init_log_count,
            saved_colors_json: QString::from(&saved_colors_json),
            used_colors_json: QString::from(&used_colors_json),
            copied_key_slot: None,
            firmware_download_url: String::new(),
            sd_root_path,
            profiles,
            switcher_state: Arc::new(Mutex::new(switcher)),
            kwin_listener_started: false,
            picker_bind_to_active_profile: false,
        }
    }
}

impl DeviceControllerRust {
    pub fn collect_used_colors_json(profiles: &[DuckyProfile]) -> String {
        let mut seen: Vec<String> = Vec::new();
        for prof in profiles {
            let bg_hex = format!(
                "#{:02x}{:02x}{:02x}",
                prof.bg_color[0], prof.bg_color[1], prof.bg_color[2]
            );
            if !seen.iter().any(|c| c.eq_ignore_ascii_case(&bg_hex)) {
                seen.push(bg_hex);
            }
            for slot in prof.slots.values() {
                if let Some(rgb) = slot.custom_rgb {
                    let hex = format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
                    if !seen.iter().any(|c| c.eq_ignore_ascii_case(&hex)) {
                        seen.push(hex);
                    }
                }
            }
        }
        serde_json::to_string(&seen).unwrap_or_else(|_| "[]".to_string())
    }
    pub fn create_default_demo_profile() -> DuckyProfile {
        let mut profile = DuckyProfile {
            number: 1,
            name: "Blender Sculpt".to_string(),
            bg_color: [0, 170, 255],
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            ..Default::default()
        };

        let defaults: &[(usize, &str, &str, [u8; 3], &str, &str)] = &[
            (1, "Front", "View", [0, 170, 255], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Front View\nOLED_UPDATE\nKP_1\n", ""),
            (2, "Side", "View", [0, 170, 255], "DEFAULTDELAY 20\nKP_3\n", ""),
            (3, "Top", "View", [0, 170, 255], "DEFAULTDELAY 20\nKP_7\n", ""),
            (4, "Wire", "Frame", [0, 220, 200], "DEFAULTDELAY 20\nKEYDOWN z\n", "KEYUP z\n"),
            (5, "Grab", "Brush", [40, 220, 100], "DEFAULTDELAY 20\nSTRING g\n", ""),
            (6, "Clay", "Strips", [40, 220, 100], "DEFAULTDELAY 20\nSTRING c\n", ""),
            (7, "Crease", "Tool", [40, 220, 100], "DEFAULTDELAY 20\nSHIFT c\n", ""),
            (8, "Smooth", "Hold", [40, 220, 100], "KEYDOWN SHIFT\n", "KEYUP SHIFT\n"),
            (9, "Draw", "Sharp", [40, 220, 100], "DEFAULTDELAY 20\nSTRING x\n", ""),
            (10, "Inflat", "Brush", [40, 220, 100], "DEFAULTDELAY 20\nSTRING i\n", ""),
            (11, "Pinch", "Edge", [40, 220, 100], "DEFAULTDELAY 20\nSTRING p\n", ""),
            (12, "Mask", "Lasso", [255, 180, 0], "DEFAULTDELAY 20\nCONTROL SHIFT LCLICK\n", ""),
            (13, "DynTop", "Toggle", [255, 140, 0], "DEFAULTDELAY 20\nCONTROL d\n", ""),
            (14, "Voxel", "Remesh", [255, 140, 0], "DEFAULTDELAY 20\nCONTROL r\n", ""),
            (15, "Invert", "Mask", [255, 180, 0], "DEFAULTDELAY 20\nCONTROL i\n", ""),
            (16, "Clear", "Mask", [255, 180, 0], "DEFAULTDELAY 20\nALT m\n", ""),
            (17, "Undo", "Step", [255, 60, 60], "DEFAULTDELAY 20\nCONTROL z\n", ""),
            (18, "Redo", "Step", [255, 60, 60], "DEFAULTDELAY 20\nCONTROL SHIFT z\n", ""),
            (19, "Auto", "Click", [180, 80, 255], "WHILE _KEYPRESS\n    LCLICK\n    DELAY 50\nEND_WHILE\n", ""),
            (20, "Save", "Incr+", [180, 80, 255], "DEFAULTDELAY 20\nCONTROL ALT s\n", ""),
            (21, "Size -", "", [56, 189, 248], "STRING [\n", ""),
            (22, "Size +", "", [56, 189, 248], "STRING ]\n", ""),
            (23, "Invert", "", [56, 189, 248], "CONTROL i\n", ""),
            (24, "Zoom -", "", [56, 189, 248], "KP_MINUS\n", ""),
            (25, "Zoom +", "", [56, 189, 248], "KP_PLUS\n", ""),
            (26, "Frame", "", [56, 189, 248], "KP_PERIOD\n", ""),
        ];

        for &(slot_idx, l1, l2, rgb, press, release) in defaults {
            profile.slots.insert(
                slot_idx,
                DuckyKeySlot {
                    slot: slot_idx,
                    label_line1: l1.to_string(),
                    label_line2: l2.to_string(),
                    widget_label_line1: String::new(),
                    widget_label_line2: String::new(),
                    custom_rgb: Some(rgb),
                    allow_abort: false,
                    disable_autorepeat: false,
                    on_press_script: press.to_string(),
                    on_release_script: release.to_string(),
                },
            );
        }

        profile
    }

    pub fn create_showcase_profiles_and_rules() -> (Vec<DuckyProfile>, Vec<AutoSwitchRule>) {
        fn build_profile(
            number: u8,
            name: &str,
            bg_color: [u8; 3],
            slots: &[(usize, &str, &str, [u8; 3], &str, &str)],
        ) -> DuckyProfile {
            let mut p = DuckyProfile {
                number,
                name: name.to_string(),
                folder_name_override: Some(format!("profile_{}", name.replace(' ', "_"))),
                bg_color,
                keydown_color: Some([255, 255, 255]),
                dim_unused_keys: true,
                ..Default::default()
            };
            for &(slot_idx, l1, l2, rgb, press, release) in slots {
                p.slots.insert(
                    slot_idx,
                    DuckyKeySlot {
                        slot: slot_idx,
                        label_line1: l1.to_string(),
                        label_line2: l2.to_string(),
                        widget_label_line1: String::new(),
                        widget_label_line2: String::new(),
                        custom_rgb: Some(rgb),
                        allow_abort: press.contains("WHILE"),
                        disable_autorepeat: false,
                        on_press_script: press.to_string(),
                        on_release_script: release.to_string(),
                    },
                );
            }
            p
        }

        // 1. Dolphin File Manager
        let dolphin = build_profile(
            1,
            "Dolphin",
            [0, 170, 255],
            &[
                (1, "New", "Tab", [0, 190, 255], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT New Dolphin Tab\nOLED_UPDATE\nCONTROL t\n", ""),
                (2, "Split", "View", [0, 190, 255], "DEFAULTDELAY 20\nF3\n", ""),
                (3, "Term", "Panel", [16, 185, 129], "DEFAULTDELAY 20\nF4\n", ""),
                (4, "Hidden", "Files", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL h\n", ""),
                (5, "Copy", "Path", [250, 204, 21], "DEFAULTDELAY 20\nCONTROL ALT c\n", ""),
                (6, "Filter", "Bar", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL i\n", ""),
                (7, "Search", "Files", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL f\n", ""),
                (8, "New", "Folder", [34, 197, 94], "DEFAULTDELAY 20\nF10\n", ""),
                (9, "Rename", "Item", [249, 115, 22], "DEFAULTDELAY 20\nF2\n", ""),
                (10, "Props", "Dialog", [168, 85, 247], "DEFAULTDELAY 20\nALT RETURN\n", ""),
                (11, "Back", "Dir", [99, 102, 241], "DEFAULTDELAY 20\nALT LEFT\n", ""),
                (12, "Up", "Folder", [99, 102, 241], "DEFAULTDELAY 20\nALT UP\n", ""),
                (13, "Fwd", "Dir", [99, 102, 241], "DEFAULTDELAY 20\nALT RIGHT\n", ""),
                (14, "Home", "Folder", [99, 102, 241], "DEFAULTDELAY 20\nALT HOME\n", ""),
                (15, "Move", "Trash", [239, 68, 68], "DEFAULTDELAY 20\nDELETE\n", ""),
                (16, "Open", "Konsole", [16, 185, 129], "DEFAULTDELAY 20\nSHIFT F4\n", ""),
                (17, "Select", "All", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL a\n", ""),
                (18, "Invert", "Select", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL SHIFT a\n", ""),
                (19, "Preview", "Pane", [168, 85, 247], "DEFAULTDELAY 20\nF12\n", ""),
                (20, "Close", "Tab", [239, 68, 68], "DEFAULTDELAY 20\nCONTROL w\n", ""),
                (21, "Icon -", "", [56, 189, 248], "CONTROL MINUS\n", ""),
                (22, "Icon +", "", [56, 189, 248], "CONTROL EQUAL\n", ""),
                (23, "Reset", "", [56, 189, 248], "CONTROL 0\n", ""),
                (24, "Tab <", "", [56, 189, 248], "CONTROL PAGEUP\n", ""),
                (25, "Tab >", "", [56, 189, 248], "CONTROL PAGEDOWN\n", ""),
                (26, "Reload", "", [56, 189, 248], "F5\n", ""),
            ],
        );

        // 2. Google Chrome
        let chrome = build_profile(
            2,
            "Google Chrome",
            [66, 133, 244],
            &[
                (1, "New", "Tab", [66, 133, 244], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Chrome: New Tab\nOLED_UPDATE\nCONTROL t\n", ""),
                (2, "Reopen", "Closed", [52, 168, 83], "DEFAULTDELAY 20\nCONTROL SHIFT t\n", ""),
                (3, "Close", "Tab", [234, 67, 53], "DEFAULTDELAY 20\nCONTROL w\n", ""),
                (4, "Incog", "Window", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL SHIFT n\n", ""),
                (5, "Dev", "Tools", [251, 188, 5], "DEFAULTDELAY 20\nCONTROL SHIFT i\n", ""),
                (6, "Address", "Bar", [66, 133, 244], "DEFAULTDELAY 20\nCONTROL l\n", ""),
                (7, "Find", "Page", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL f\n", ""),
                (8, "Book", "Mark", [251, 188, 5], "DEFAULTDELAY 20\nCONTROL d\n", ""),
                (9, "History", "List", [99, 102, 241], "DEFAULTDELAY 20\nCONTROL h\n", ""),
                (10, "Down", "Loads", [52, 168, 83], "DEFAULTDELAY 20\nCONTROL j\n", ""),
                (11, "Hard", "Reload", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT r\n", ""),
                (12, "Mute", "Tab", [234, 67, 53], "DEFAULTDELAY 20\nALT m\n", ""),
                (13, "Back", "Page", [66, 133, 244], "DEFAULTDELAY 20\nALT LEFT\n", ""),
                (14, "Next", "Page", [66, 133, 244], "DEFAULTDELAY 20\nALT RIGHT\n", ""),
                (15, "Tab", "Search", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT a\n", ""),
                (16, "Source", "View", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL u\n", ""),
                (17, "Full", "Screen", [52, 168, 83], "DEFAULTDELAY 20\nF11\n", ""),
                (18, "Clear", "Data", [234, 67, 53], "DEFAULTDELAY 20\nCONTROL SHIFT DELETE\n", ""),
                (19, "Task", "Mgr", [251, 188, 5], "DEFAULTDELAY 20\nSHIFT ESCAPE\n", ""),
                (20, "Print", "PDF", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL p\n", ""),
                (21, "Tab <", "", [66, 133, 244], "CONTROL SHIFT TAB\n", ""),
                (22, "Tab >", "", [66, 133, 244], "CONTROL TAB\n", ""),
                (23, "Reload", "", [66, 133, 244], "F5\n", ""),
                (24, "Zoom -", "", [52, 168, 83], "CONTROL MINUS\n", ""),
                (25, "Zoom +", "", [52, 168, 83], "CONTROL EQUAL\n", ""),
                (26, "100%", "", [52, 168, 83], "CONTROL 0\n", ""),
            ],
        );

        // 3. VS Code
        let vscode = build_profile(
            3,
            "VS Code",
            [0, 122, 204],
            &[
                (1, "Cmd", "Palette", [0, 152, 255], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Command Palette\nOLED_UPDATE\nCONTROL SHIFT p\n", ""),
                (2, "Quick", "Open", [0, 152, 255], "DEFAULTDELAY 20\nCONTROL p\n", ""),
                (3, "Toggle", "Term", [16, 185, 129], "DEFAULTDELAY 20\nCONTROL GRAVE\n", ""),
                (4, "Build", "Task", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT b\n", ""),
                (5, "Format", "Code", [52, 211, 153], "DEFAULTDELAY 20\nCONTROL SHIFT i\n", ""),
                (6, "Go To", "Def", [56, 189, 248], "DEFAULTDELAY 20\nF12\n", ""),
                (7, "Rename", "Symbol", [250, 204, 21], "DEFAULTDELAY 20\nF2\n", ""),
                (8, "Search", "All", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT f\n", ""),
                (9, "Source", "Git", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT g\n", ""),
                (10, "Debug", "Start", [34, 197, 94], "DEFAULTDELAY 20\nF5\n", ""),
                (11, "Step", "Over", [56, 189, 248], "DEFAULTDELAY 20\nF10\n", ""),
                (12, "Step", "Into", [56, 189, 248], "DEFAULTDELAY 20\nF11\n", ""),
                (13, "Break", "Point", [239, 68, 68], "DEFAULTDELAY 20\nF9\n", ""),
                (14, "Split", "Editor", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL BACKSLASH\n", ""),
                (15, "Side", "Bar", [99, 102, 241], "DEFAULTDELAY 20\nCONTROL b\n", ""),
                (16, "Zen", "Mode", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL k\nDELAY 40\nSTRING z\n", ""),
                (17, "Comment", "Line", [34, 197, 94], "DEFAULTDELAY 20\nCONTROL SLASH\n", ""),
                (18, "Multi", "Cursor", [250, 204, 21], "DEFAULTDELAY 20\nCONTROL d\n", ""),
                (19, "Problem", "Panel", [239, 68, 68], "DEFAULTDELAY 20\nCONTROL SHIFT m\n", ""),
                (20, "Save", "All", [34, 197, 94], "DEFAULTDELAY 20\nCONTROL ALT s\n", ""),
                (21, "Font -", "", [0, 122, 204], "CONTROL MINUS\n", ""),
                (22, "Font +", "", [0, 122, 204], "CONTROL EQUAL\n", ""),
                (23, "Reset", "", [0, 122, 204], "CONTROL NUMPAD0\n", ""),
                (24, "Edit <", "", [0, 122, 204], "CONTROL PAGEUP\n", ""),
                (25, "Edit >", "", [0, 122, 204], "CONTROL PAGEDOWN\n", ""),
                (26, "Fold", "", [0, 122, 204], "CONTROL SHIFT [\n", ""),
            ],
        );

        // 4. OBS Studio (with Web API triggers)
        let obs_slot17 = apply_web_api_config_to_script(
            "",
            &WebApiConfig {
                enabled: true,
                destination: "Home Assistant — Light Toggle".to_string(),
                method: "POST".to_string(),
                url: "http://homeassistant.local:8123/api/services/light/toggle".to_string(),
                headers: "Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\nContent-Type: application/json".to_string(),
                body: "{\"entity_id\": \"light.studio_key_light\"}".to_string(),
                oled_text: "Key Light API".to_string(),
                led_feedback: true,
                trigger_mode: "pgv".to_string(),
                fkey: "F17".to_string(),
            },
            4,
            17,
            false,
        );
        let obs_slot18 = apply_web_api_config_to_script(
            "",
            &WebApiConfig {
                enabled: true,
                destination: "Home Assistant — Switch Turn On".to_string(),
                method: "POST".to_string(),
                url: "http://homeassistant.local:8123/api/services/switch/turn_on".to_string(),
                headers: "Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\nContent-Type: application/json".to_string(),
                body: "{\"entity_id\": \"switch.on_air_lamp\"}".to_string(),
                oled_text: "On-Air Lamp ON".to_string(),
                led_feedback: true,
                trigger_mode: "pgv".to_string(),
                fkey: "F18".to_string(),
            },
            4,
            18,
            false,
        );
        let obs = build_profile(
            4,
            "OBS Studio",
            [139, 92, 246],
            &[
                (1, "Scene", "Main", [56, 189, 248], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Scene: Main Cam\nOLED_UPDATE\nF13\n", ""),
                (2, "Scene", "Screen", [56, 189, 248], "DEFAULTDELAY 20\nF14\n", ""),
                (3, "Scene", "Pip", [56, 189, 248], "DEFAULTDELAY 20\nF15\n", ""),
                (4, "Scene", "BRB", [250, 204, 21], "DEFAULTDELAY 20\nF16\n", ""),
                (5, "Record", "Toggle", [239, 68, 68], "DEFAULTDELAY 20\nCONTROL SHIFT r\n", ""),
                (6, "Pause", "Rec", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT p\n", ""),
                (7, "Stream", "Live", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL SHIFT s\n", ""),
                (8, "Replay", "Buffer", [34, 197, 94], "DEFAULTDELAY 20\nCONTROL SHIFT b\n", ""),
                (9, "Mic", "Mute", [239, 68, 68], "DEFAULTDELAY 20\nCONTROL SHIFT m\n", ""),
                (10, "Desktop", "Mute", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT d\n", ""),
                (11, "Studio", "Mode", [139, 92, 246], "DEFAULTDELAY 20\nCONTROL SHIFT t\n", ""),
                (12, "Cut", "Trans", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT c\n", ""),
                (13, "Fade", "Trans", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT f\n", ""),
                (14, "Snap", "Shot", [34, 197, 94], "DEFAULTDELAY 20\nF17\n", ""),
                (15, "Virtual", "Cam", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL SHIFT v\n", ""),
                (16, "Stats", "Dock", [99, 102, 241], "DEFAULTDELAY 20\nCONTROL SHIFT i\n", ""),
                (17, "Key", "Light", [250, 204, 21], &obs_slot17, ""),
                (18, "On Air", "Lamp", [239, 68, 68], &obs_slot18, ""),
                (19, "Stream", "Marker", [34, 197, 94], "DEFAULTDELAY 20\nF18\n", ""),
                (20, "Panic", "Mute", [239, 68, 68], "DEFAULTDELAY 20\nF19\n", ""),
                (21, "Mic -", "", [139, 92, 246], "F20\n", ""),
                (22, "Mic +", "", [139, 92, 246], "F21\n", ""),
                (23, "Mute", "", [239, 68, 68], "CONTROL SHIFT m\n", ""),
                (24, "Desk -", "", [139, 92, 246], "F22\n", ""),
                (25, "Desk +", "", [139, 92, 246], "F23\n", ""),
                (26, "Mute", "", [249, 115, 22], "CONTROL SHIFT d\n", ""),
            ],
        );

        // 5. Konsole Terminal
        let konsole = build_profile(
            5,
            "Konsole",
            [16, 185, 129],
            &[
                (1, "New", "Tab", [16, 185, 129], "DEFAULTDELAY 20\nCONTROL SHIFT t\n", ""),
                (2, "Split", "L / R", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL (\n", ""),
                (3, "Split", "T / B", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL )\n", ""),
                (4, "Clear", "Screen", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL l\n", ""),
                (5, "Git", "Status", [249, 115, 22], "DEFAULTDELAY 20\nSTRINGLN git status -sb\n", ""),
                (6, "Git", "Log", [249, 115, 22], "DEFAULTDELAY 20\nSTRINGLN git log --oneline -n 15\n", ""),
                (7, "Cargo", "Build", [250, 204, 21], "DEFAULTDELAY 20\nSTRINGLN cargo build --release\n", ""),
                (8, "Cargo", "Test", [34, 197, 94], "DEFAULTDELAY 20\nSTRINGLN cargo test\n", ""),
                (9, "Docker", "PS", [56, 189, 248], "DEFAULTDELAY 20\nSTRINGLN docker ps\n", ""),
                (10, "System", "Btop", [168, 85, 247], "DEFAULTDELAY 20\nSTRINGLN btop\n", ""),
                (11, "Journal", "Follow", [250, 204, 21], "DEFAULTDELAY 20\nSTRINGLN journalctl -f -n 40\n", ""),
                (12, "Hist", "Search", [16, 185, 129], "DEFAULTDELAY 20\nCONTROL r\n", ""),
                (13, "Copy", "Text", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT c\n", ""),
                (14, "Paste", "Clip", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT v\n", ""),
                (15, "Find", "Output", [99, 102, 241], "DEFAULTDELAY 20\nCONTROL SHIFT f\n", ""),
                (16, "Cancel", "SIGINT", [239, 68, 68], "DEFAULTDELAY 20\nCONTROL c\n", ""),
                (21, "Tab <", "", [16, 185, 129], "SHIFT LEFT\n", ""),
                (22, "Tab >", "", [16, 185, 129], "SHIFT RIGHT\n", ""),
                (23, "New", "", [16, 185, 129], "CONTROL SHIFT t\n", ""),
                (24, "Font -", "", [16, 185, 129], "CONTROL MINUS\n", ""),
                (25, "Font +", "", [16, 185, 129], "CONTROL PLUS\n", ""),
                (26, "100%", "", [16, 185, 129], "CONTROL ALT 0\n", ""),
            ],
        );

        // 6. Blender 3D
        let mut blender = Self::create_default_demo_profile();
        blender.number = 6;
        blender.name = "Blender 3D".to_string();
        blender.folder_name_override = Some("profile_Blender_3D".to_string());
        blender.bg_color = [249, 115, 22];

        // 7. Spotify & Media
        let spotify = build_profile(
            7,
            "Spotify",
            [30, 215, 96],
            &[
                (1, "Play", "Pause", [30, 215, 96], "DEFAULTDELAY 20\nMK_PLAYPAUSE\n", ""),
                (2, "Prev", "Track", [56, 189, 248], "DEFAULTDELAY 20\nMK_PREV\n", ""),
                (3, "Next", "Track", [56, 189, 248], "DEFAULTDELAY 20\nMK_NEXT\n", ""),
                (4, "Stop", "Audio", [239, 68, 68], "DEFAULTDELAY 20\nMK_STOP\n", ""),
                (5, "Mute", "Audio", [249, 115, 22], "DEFAULTDELAY 20\nMK_MUTE\n", ""),
                (6, "Like", "Song", [236, 72, 153], "DEFAULTDELAY 20\nALT SHIFT b\n", ""),
                (7, "Shuffle", "Queue", [30, 215, 96], "DEFAULTDELAY 20\nCONTROL s\n", ""),
                (8, "Repeat", "Track", [30, 215, 96], "DEFAULTDELAY 20\nCONTROL r\n", ""),
                (9, "Search", "Music", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL l\n", ""),
                (10, "Queue", "Panel", [168, 85, 247], "DEFAULTDELAY 20\nALT SHIFT q\n", ""),
                (11, "Lyrics", "View", [250, 204, 21], "DEFAULTDELAY 20\nALT SHIFT l\n", ""),
                (12, "Now", "Playing", [30, 215, 96], "DEFAULTDELAY 20\nALT SHIFT j\n", ""),
                (21, "Vol -", "", [30, 215, 96], "MK_VOLDOWN\n", ""),
                (22, "Vol +", "", [30, 215, 96], "MK_VOLUP\n", ""),
                (23, "Mute", "", [239, 68, 68], "MK_MUTE\n", ""),
                (24, "Back", "", [56, 189, 248], "SHIFT LEFT\n", ""),
                (25, "Fwd", "", [56, 189, 248], "SHIFT RIGHT\n", ""),
                (26, "Play", "", [30, 215, 96], "MK_PLAYPAUSE\n", ""),
            ],
        );

        // 8. App Launcher (with @DUCKYDECK_APP_LAUNCH triggers)
        let make_app_script = |slot_num: usize, name: &str, desktop_id: &str, cmd: &str, args: &str, oled: &str| {
            apply_app_launch_config_to_script(
                "",
                &AppLaunchConfig {
                    enabled: true,
                    app_name: name.to_string(),
                    desktop_id: desktop_id.to_string(),
                    command: cmd.to_string(),
                    args: args.to_string(),
                    working_dir: "/home/demo/Projects".to_string(),
                    run_in_terminal: false,
                    oled_text: oled.to_string(),
                    led_feedback: true,
                    trigger_mode: "pgv".to_string(),
                    fkey: format!("F{}", 12 + slot_num.min(12)),
                },
                8,
                slot_num,
                false,
            )
        };
        let app1 = make_app_script(1, "Dolphin", "org.kde.dolphin.desktop", "dolphin", "~/Projects", "Run: Dolphin");
        let app2 = make_app_script(2, "Google Chrome", "google-chrome.desktop", "google-chrome-stable", "https://www.duckypad.com", "Run: Chrome");
        let app3 = make_app_script(3, "Visual Studio Code", "code.desktop", "code", "~/Projects/duckydeck", "Run: VS Code");
        let app4 = make_app_script(4, "OBS Studio", "com.obsproject.Studio.desktop", "obs", "", "Run: OBS Studio");
        let app5 = make_app_script(5, "Konsole", "org.kde.konsole.desktop", "konsole", "", "Run: Konsole");
        let app6 = make_app_script(6, "Blender", "blender.desktop", "blender", "", "Run: Blender");
        let app7 = make_app_script(7, "Spotify", "spotify.desktop", "spotify", "", "Run: Spotify");
        let app8 = make_app_script(8, "GIMP", "gimp.desktop", "gimp", "", "Run: GIMP");

        let app_launcher = build_profile(
            8,
            "App Launcher",
            [236, 72, 153],
            &[
                (1, "Dolphin", "Files", [0, 170, 255], &app1, ""),
                (2, "Google", "Chrome", [66, 133, 244], &app2, ""),
                (3, "VS", "Code", [0, 122, 204], &app3, ""),
                (4, "OBS", "Studio", [139, 92, 246], &app4, ""),
                (5, "Konsole", "Term", [16, 185, 129], &app5, ""),
                (6, "Blender", "3D", [249, 115, 22], &app6, ""),
                (7, "Spotify", "Music", [30, 215, 96], &app7, ""),
                (8, "GIMP", "Image", [250, 204, 21], &app8, ""),
            ],
        );

        let profiles = vec![
            dolphin,
            chrome,
            vscode,
            obs,
            konsole,
            blender,
            spotify,
            app_launcher,
        ];

        let rules = vec![
            AutoSwitchRule {
                enabled: true,
                resource_class: "org.kde.dolphin".to_string(),
                title_contains: None,
                favicon_url: None,
                target_profile: 1,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "google-chrome".to_string(),
                title_contains: None,
                favicon_url: None,
                target_profile: 2,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "code".to_string(),
                title_contains: None,
                favicon_url: None,
                target_profile: 3,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "com.obsproject.Studio".to_string(),
                title_contains: None,
                favicon_url: None,
                target_profile: 4,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "org.kde.konsole".to_string(),
                title_contains: None,
                favicon_url: None,
                target_profile: 5,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "blender".to_string(),
                title_contains: None,
                favicon_url: None,
                target_profile: 6,
            },
            AutoSwitchRule {
                enabled: true,
                resource_class: "spotify".to_string(),
                title_contains: None,
                favicon_url: None,
                target_profile: 7,
            },
        ];

        (profiles, rules)
    }

    pub fn create_showcase_instance() -> Self {
        let (profiles, rules) = Self::create_showcase_profiles_and_rules();
        let saved_colors = crate::kwin::auto_switcher::default_saved_colors();
        let switcher = SwitcherState {
            enabled: true,
            paused_for_editing: false,
            default_profile: Some(8),
            rules: rules.clone(),
            mouse_settle_ms: 250,
            window_dwell_ms: 150,
            tray_icon_mode: "auto".to_string(),
            app_theme_mode: "dark".to_string(),
            close_to_tray: true,
            ui_text_scale: 100,
            sidebar_text_scale: 100,
            canvas_text_scale: 100,
            inspector_text_scale: 100,
            editor_text_scale: 100,
            status_bar_text_scale: 100,
            event_log_text_scale: 100,
            saved_colors: saved_colors.clone(),
            last_switched_profile: 1,
            picker_armed: false,
            loaded_from_disk: true,
        };

        if let Ok(mut guard) = system_event_log_store().lock() {
            guard.clear();
        }
        record_system_event(
            "INFO",
            "SYSTEM",
            format!("DuckyDeck Studio v{} started (KDE Plasma 6 Wayland)", APP_VERSION),
        );
        record_system_event(
            "INFO",
            "DEVICE",
            "Connected to duckyPad Pro (FW v2.3.1, SN DP24-8F4A1C9E) — 8 profile(s) loaded",
        );
        record_system_event(
            "INFO",
            "AUTOSWITCH",
            "KWin Wayland D-Bus Auto-Switcher active (7 application rules loaded, fallback=#8 App Launcher)",
        );
        record_system_event(
            "INFO",
            "AUTOSWITCH",
            "Window focus matched rule 'org.kde.dolphin' → switched hardware to Profile #1 (Dolphin)",
        );
        record_system_event(
            "INFO",
            "WEB_API",
            "Web API PGV[31] listener armed for Profile #4 (OBS Studio): Slots 17 & 18 ready",
        );
        record_system_event(
            "INFO",
            "APP_LAUNCH",
            "Application Launcher PGV[31] listener armed for Profile #8 (App Launcher): 8 desktop apps ready",
        );

        let rules_json = Self::serialize_rules(&rules, &profiles);
        let profiles_json = Self::serialize_profiles_summary(&profiles, &rules);
        let active_prof_obj = profiles.first().unwrap();
        let slots_json = Self::serialize_profile_slots(active_prof_obj);
        let saved_colors_json =
            serde_json::to_string(&saved_colors).unwrap_or_else(|_| "[]".to_string());
        let used_colors_json = Self::collect_used_colors_json(&profiles);
        let (init_log_json, init_log_count) = serialize_system_event_log_json();

        let demo_apps = json!([
            {"name": "Dolphin", "exec": "dolphin", "icon": "system-file-manager", "desktopId": "org.kde.dolphin.desktop", "comment": "KDE File Manager"},
            {"name": "Google Chrome", "exec": "google-chrome-stable", "icon": "google-chrome", "desktopId": "google-chrome.desktop", "comment": "Access the Internet"},
            {"name": "Visual Studio Code", "exec": "code", "icon": "visual-studio-code", "desktopId": "code.desktop", "comment": "Code Editing. Redefined."},
            {"name": "OBS Studio", "exec": "obs", "icon": "com.obsproject.Studio", "desktopId": "com.obsproject.Studio.desktop", "comment": "Live Streaming and Screen Recorder"},
            {"name": "Konsole", "exec": "konsole", "icon": "utilities-terminal", "desktopId": "org.kde.konsole.desktop", "comment": "KDE Terminal Emulator"},
            {"name": "Blender", "exec": "blender", "icon": "blender", "desktopId": "blender.desktop", "comment": "3D modeling, animation, rendering and post-production"}
        ]);

        Self {
            connected: true,
            udev_rule_missing: false,
            model_name: QString::from("duckyPad Pro"),
            firmware_version: QString::from("1.0.0 (DuckyDeck Studio)"),
            serial_number: QString::from("DP24-8F4A1C9E"),
            active_profile: 1,
            active_profile_name: QString::from("Dolphin"),
            sd_mount_path: QString::from("/run/media/ducky/DUCKYPAD"),
            sd_card_detected: true,
            sd_space_summary: QString::from("SD: 1.42 MB used • 3.74 GB free"),
            sd_space_detail: QString::from(
                "duckyPad microSD Card Capacity\n• Used Space: 1.42 MB (0.04%)\n• Free Space: 3.74 GB (99.96%)\n• Total Capacity: 3.74 GB\n• Source: Mounted at /run/media/ducky/DUCKYPAD",
            ),
            sd_space_used_percent: 1,
            profiles_json: QString::from(&profiles_json),
            active_profile_slots_json: QString::from(&slots_json),
            status_message: QString::from(
                "Connected to duckyPad Pro (FW v1.0.0 DuckyDeck Studio) — 8 profile(s) synced & verified",
            ),
            ai_busy: false,
            generated_script_preview: QString::from(
                "// Slot 1: New Tab\nDEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT New Dolphin Tab\nOLED_UPDATE\nCONTROL t\n",
            ),
            auto_switch_enabled: true,
            default_fallback_profile: 8,
            auto_switch_settle_ms: 250,
            auto_switch_dwell_ms: 150,
            active_window_class: QString::from("org.kde.dolphin"),
            active_window_title: QString::from("Projects — Dolphin"),
            window_picker_armed: false,
            last_picked_window_class: QString::from("google-chrome"),
            last_picked_window_title: QString::from("duckyPad Pro Documentation"),
            auto_switch_rules_json: QString::from(&rules_json),
            autostart_enabled: true,
            tray_icon_mode: QString::from("auto"),
            app_theme_mode: QString::from("dark"),
            close_to_tray: true,
            system_dark_theme: true,
            latest_firmware_version: QString::from("1.0.0"),
            firmware_update_available: false,
            firmware_check_busy: false,
            firmware_update_url: QString::from(
                "https://dekuNukem.github.io/duckyPad-Pro/doc/fw_update.html",
            ),
            firmware_check_summary: QString::from("DuckyDeck Studio Firmware v1.0.0 is up to date"),
            profile_sync_state: QString::from("synced"),
            profile_sync_detail: QString::from(
                "All 8 profiles verified in sync with duckyPad Pro microSD [hash #8f4a1c9e]",
            ),
            sd_sync_busy: false,
            sd_sync_progress: 100,
            sd_sync_step: QString::from("Ready"),
            sd_sync_log: QString::from(""),
            sd_sync_result_state: QString::from("idle"),
            sd_sync_result_summary: QString::from(""),
            web_api_test_busy: false,
            web_api_last_ok: true,
            web_api_last_result: QString::from("HTTP 200 OK (18 ms) — light.studio_key_light toggled"),
            web_api_destinations_json: QString::from(
                "[{\"name\":\"Home Assistant — Light Toggle\",\"method\":\"POST\",\"url\":\"http://homeassistant.local:8123/api/services/light/toggle\",\"headers\":\"Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\\nContent-Type: application/json\"},{\"name\":\"Home Assistant — Switch Turn On\",\"method\":\"POST\",\"url\":\"http://homeassistant.local:8123/api/services/switch/turn_on\",\"headers\":\"Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\\nContent-Type: application/json\"}]",
            ),
            app_launch_last_ok: true,
            app_launch_last_result: QString::from("Launched Dolphin (PID 48210)"),
            installed_apps_json: QString::from(&demo_apps.to_string()),
            ui_text_scale: 100,
            sidebar_text_scale: 100,
            canvas_text_scale: 100,
            inspector_text_scale: 100,
            editor_text_scale: 100,
            status_bar_text_scale: 100,
            event_log_text_scale: 100,
            active_profile_bg_color: QString::from("#00aaff"),
            unsynced_changes_json: QString::from("[]"),
            has_copied_slot: false,
            copied_slot_summary: QString::from(""),
            app_version: QString::from(APP_VERSION),
            app_author: QString::from(APP_AUTHOR),
            app_github_url: QString::from(APP_GITHUB_URL),
            log_file_path: QString::from("/home/demo/.local/share/duckydeck-studio/duckydeck.log"),
            event_log_json: QString::from(&init_log_json),
            event_log_count: init_log_count,
            saved_colors_json: QString::from(&saved_colors_json),
            used_colors_json: QString::from(&used_colors_json),
            copied_key_slot: None,
            firmware_download_url: String::new(),
            sd_root_path: None,
            profiles,
            switcher_state: Arc::new(Mutex::new(switcher)),
            kwin_listener_started: true,
            picker_bind_to_active_profile: false,
        }
    }

    pub fn serialize_profiles_summary(
        profiles: &[DuckyProfile],
        rules: &[AutoSwitchRule],
    ) -> String {
        let items: Vec<_> = profiles
            .iter()
            .map(|p| {
                let bound_rule = rules
                    .iter()
                    .find(|r| r.enabled && r.target_profile == p.number);
                let bound_class = bound_rule.map(|r| r.resource_class.as_str()).unwrap_or("");
                let bound_icon = match bound_rule {
                    Some(r) if !r.resource_class.trim().is_empty() => resolve_rule_icon(
                        &r.resource_class,
                        r.title_contains.as_deref(),
                        r.favicon_url.as_deref(),
                        Some(&p.name),
                    ),
                    _ => String::new(),
                };
                json!({
                    "number": p.number,
                    "name": p.name,
                    "folder": p.folder_name(),
                    "bgColor": p.bg_color,
                    "slotCount": p.slots.len(),
                    "boundClass": bound_class,
                    "boundIcon": bound_icon,
                    "oledRotation": p.oled_rotation_degrees(),
                    "widgetRotation": p.widget_rotation_degrees(),
                    "effectiveWidgetRotation": p.effective_widget_rotation_degrees(),
                })
            })
            .collect();
        serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn serialize_rules(rules: &[AutoSwitchRule], profiles: &[DuckyProfile]) -> String {
        let items: Vec<_> = rules
            .iter()
            .enumerate()
            .map(|(idx, r)| {
                let prof_name = profiles
                    .iter()
                    .find(|p| p.number == r.target_profile)
                    .map(|p| p.name.as_str())
                    .unwrap_or("Unassigned");
                let icon_name = resolve_rule_icon(
                    &r.resource_class,
                    r.title_contains.as_deref(),
                    r.favicon_url.as_deref(),
                    Some(prof_name),
                );
                json!({
                    "index": idx,
                    "enabled": r.enabled,
                    "resourceClass": r.resource_class,
                    "titleContains": r.title_contains.as_deref().unwrap_or(""),
                    "faviconUrl": r.favicon_url.as_deref().unwrap_or(""),
                    "targetProfile": r.target_profile,
                    "targetProfileName": prof_name,
                    "icon": icon_name,
                })
            })
            .collect();
        serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn serialize_profile_slots(profile: &DuckyProfile) -> String {
        let mut items = Vec::with_capacity(MAX_INPUT_SLOTS);
        for slot_idx in 1..=MAX_INPUT_SLOTS {
            if let Some(slot) = profile.slots.get(&slot_idx) {
                let has_script = !slot.on_press_script.trim().is_empty()
                    || !slot.on_release_script.trim().is_empty()
                    || !slot.label_line1.is_empty()
                    || !slot.label_line2.is_empty()
                    || !slot.widget_label_line1.trim().is_empty()
                    || !slot.widget_label_line2.trim().is_empty();
                let default_rgb = if !has_script && profile.dim_unused_keys {
                    [20, 24, 32]
                } else {
                    profile.bg_color
                };
                let rgb = slot.custom_rgb.unwrap_or(default_rgb);
                let has_rel = !slot.on_release_script.trim().is_empty();
                let has_loop = slot.on_press_script.contains("WHILE")
                    || slot.on_press_script.contains("LOOP");
                let has_web_api = has_web_api_config(&slot.on_press_script)
                    || has_web_api_config(&slot.on_release_script);
                let has_app_launch = has_app_launch_config(&slot.on_press_script)
                    || has_app_launch_config(&slot.on_release_script);

                items.push(json!({
                    "slot": slot_idx,
                    "l1": slot.label_line1,
                    "l2": slot.label_line2,
                    "wl1": slot.widget_label_line1,
                    "wl2": slot.widget_label_line2,
                    "r": rgb[0],
                    "g": rgb[1],
                    "b": rgb[2],
                    "hasCustomRgb": slot.custom_rgb.is_some(),
                    "hasRel": has_rel,
                    "hasLoop": has_loop,
                    "hasWebApi": has_web_api,
                    "hasAppLaunch": has_app_launch,
                    "allowAbort": slot.allow_abort,
                    "disableAutorepeat": slot.disable_autorepeat,
                    "hasScript": has_script,
                }));
            } else {
                let rgb = if profile.dim_unused_keys {
                    [20, 24, 32]
                } else {
                    profile.bg_color
                };
                items.push(json!({
                    "slot": slot_idx,
                    "l1": "",
                    "l2": "",
                    "wl1": "",
                    "wl2": "",
                    "r": rgb[0],
                    "g": rgb[1],
                    "b": rgb[2],
                    "hasCustomRgb": false,
                    "hasRel": false,
                    "hasLoop": false,
                    "hasWebApi": false,
                    "hasAppLaunch": false,
                    "allowAbort": false,
                    "disableAutorepeat": false,
                    "hasScript": false,
                }));
            }
        }
        serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string())
    }
}

impl qobject::DeviceController {
    fn sync_tray_state(mut self: Pin<&mut Self>) {
        let mode_str = self.tray_icon_mode().to_string();
        let use_light_on_dark_icon = match mode_str.as_str() {
            "dark" => true,
            "light" => false,
            _ => *self.system_dark_theme(),
        };

        let model = self.model_name().to_string();
        let connected = *self.connected();
        let sd_path = self.sd_mount_path().to_string();
        let fw = self.firmware_version().to_string();
        let sn = self.serial_number().to_string();
        let prof_num = *self.active_profile();
        let prof_name = self.active_profile_name().to_string();
        let auto_enabled = *self.auto_switch_enabled();
        let autostart = *self.autostart_enabled();
        let win_class = self.active_window_class().to_string();
        let total_profiles = self.rust().profiles.len();

        let conn_badge = if connected {
            "USB HID Connected"
        } else if !sd_path.is_empty() {
            "SD Card Mode"
        } else {
            "Disconnected"
        };

        let status_header = QString::from(&format!("🦆 {model} ({conn_badge})"));
        let profile_header = QString::from(&format!(
            "Selected Profile: #{prof_num}/{total_profiles} — {prof_name}"
        ));

        let sd_mount_opt = if sd_path.is_empty() {
            None
        } else {
            Some(Path::new(&sd_path))
        };
        let sd_space = get_sd_card_space_info(&sn, sd_mount_opt);
        if self.sd_space_summary().to_string() != sd_space.summary_text {
            self.as_mut()
                .set_sd_space_summary(QString::from(&sd_space.summary_text));
        }
        if self.sd_space_detail().to_string() != sd_space.detail_tooltip {
            self.as_mut()
                .set_sd_space_detail(QString::from(&sd_space.detail_tooltip));
        }
        if *self.sd_space_used_percent() != sd_space.used_percent {
            self.as_mut()
                .set_sd_space_used_percent(sd_space.used_percent);
        }
        let sd_space_header = QString::from(&format!("💾 {}", sd_space.tray_text));

        let mut tooltip = format!(
            "DuckyDeck Studio — {model}\n\
             Selected Profile: #{prof_num} ({prof_name}) [{total_profiles} loaded]\n\
             Connection: {conn_badge}\n\
             {}",
            sd_space.tray_text
        );
        if fw != "-" || sn != "-" {
            tooltip.push_str(&format!("\nFirmware: v{fw} • Serial: {sn}"));
        }
        if !sd_path.is_empty() {
            tooltip.push_str(&format!("\nSD Mount: {sd_path}"));
        }
        let auto_str = if auto_enabled {
            format!("Enabled (Window: {win_class})")
        } else {
            "Disabled".to_string()
        };
        tooltip.push_str(&format!("\nAuto-Switcher: {auto_str}"));

        let tooltip_qstr = QString::from(&tooltip);
        let mode_qstr = QString::from(&mode_str);

        qobject::duckydeck_sync_tray(
            self.as_mut(),
            use_light_on_dark_icon,
            &mode_qstr,
            &status_header,
            &profile_header,
            &sd_space_header,
            &tooltip_qstr,
            auto_enabled,
            autostart,
        );

        let slots_raw = self.active_profile_slots_json().to_string();
        let bg_color_str = self.active_profile_bg_color().to_string();
        let slots_val: serde_json::Value =
            serde_json::from_str(&slots_raw).unwrap_or_else(|_| json!([]));
        let oled_snapshot = json!({
            "connected": connected,
            "modelName": model,
            "firmwareVersion": fw,
            "serialNumber": sn,
            "activeProfile": prof_num,
            "activeProfileName": prof_name,
            "activeProfileBgColor": bg_color_str,
            "totalProfiles": total_profiles,
            "sdSpaceSummary": sd_space.summary_text,
            "slots": slots_val,
        });
        if let Some(home) = std::env::var_os("HOME") {
            let state_dir = PathBuf::from(home).join(".local/share/duckydeck-studio");
            let _ = fs::create_dir_all(&state_dir);
            if let Ok(serialized) = serde_json::to_string(&oled_snapshot) {
                let _ = fs::write(state_dir.join("oled_state.json"), serialized);
            }
        }
    }

    fn sync_active_profile_state(mut self: Pin<&mut Self>) {
        let active_num = *self.active_profile() as u8;
        let connected = *self.connected();
        let serial_str = self.serial_number().to_string();
        let (
            prof_name,
            prof_bg_hex,
            profiles_json,
            slots_json,
            rules_json,
            sync_state,
            sync_detail,
            unsynced_json,
        ) = {
            let rust = self.rust();
            let rules_vec = rust
                .switcher_state
                .lock()
                .map(|s| s.rules.clone())
                .unwrap_or_default();
            let profiles_json =
                DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_vec);
            let rules_json = DeviceControllerRust::serialize_rules(&rules_vec, &rust.profiles);
            let (sync_state, sync_detail, unsynced_json) = if is_demo_showcase_env() {
                (
                    "synced".to_string(),
                    "All 8 profiles verified in sync with duckyPad Pro microSD [hash #8f4a1c9e]"
                        .to_string(),
                    "[]".to_string(),
                )
            } else {
                let (s_state, s_detail) = check_profile_sync_status(
                    &serial_str,
                    &rust.profiles,
                    rust.sd_root_path.as_deref(),
                    connected,
                    active_num,
                );
                let u_json = compute_unsynced_changes_json(
                    &serial_str,
                    &rust.profiles,
                    rust.sd_root_path.as_deref(),
                    &s_state,
                );
                (s_state, s_detail, u_json)
            };
            if let Some(prof) = rust
                .profiles
                .iter()
                .find(|p| p.number == active_num)
                .or_else(|| rust.profiles.first())
            {
                let bg_hex = format!(
                    "#{:02x}{:02x}{:02x}",
                    prof.bg_color[0], prof.bg_color[1], prof.bg_color[2]
                );
                (
                    prof.name.clone(),
                    bg_hex,
                    profiles_json,
                    DeviceControllerRust::serialize_profile_slots(prof),
                    rules_json,
                    sync_state,
                    sync_detail,
                    unsynced_json,
                )
            } else {
                (
                    "Default".to_string(),
                    "#00aaff".to_string(),
                    profiles_json,
                    "[]".to_string(),
                    rules_json,
                    sync_state,
                    sync_detail,
                    unsynced_json,
                )
            }
        };

        if self.active_profile_name().to_string() != prof_name {
            self.as_mut()
                .set_active_profile_name(QString::from(&prof_name));
        }
        if self.active_profile_bg_color().to_string() != prof_bg_hex {
            self.as_mut()
                .set_active_profile_bg_color(QString::from(&prof_bg_hex));
        }
        if self.profiles_json().to_string() != profiles_json {
            self.as_mut()
                .set_profiles_json(QString::from(&profiles_json));
        }
        if self.active_profile_slots_json().to_string() != slots_json {
            self.as_mut()
                .set_active_profile_slots_json(QString::from(&slots_json));
        }
        if self.auto_switch_rules_json().to_string() != rules_json {
            self.as_mut()
                .set_auto_switch_rules_json(QString::from(&rules_json));
        }
        if self.profile_sync_state().to_string() != sync_state {
            self.as_mut()
                .set_profile_sync_state(QString::from(&sync_state));
        }
        if self.profile_sync_detail().to_string() != sync_detail {
            self.as_mut()
                .set_profile_sync_detail(QString::from(&sync_detail));
        }
        if self.unsynced_changes_json().to_string() != unsynced_json {
            self.as_mut()
                .set_unsynced_changes_json(QString::from(&unsynced_json));
        }
        self.as_mut().refresh_color_palettes();
        self.as_mut().sync_tray_state();
    }

    pub fn sync_profiles_to_device_sd(mut self: Pin<&mut Self>) -> bool {
        if *self.sd_sync_busy() {
            return false;
        }

        let active_num = (*self.active_profile()).clamp(1, 64) as u8;
        let serial_str = self.serial_number().to_string();
        let profiles_snapshot = self.rust().profiles.clone();
        let mounted_sd = self
            .rust()
            .sd_root_path
            .clone()
            .or_else(find_mounted_duckypad_sd);

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
                "• Target mounted microSD: {}\n• Writing {} profile(s) and compiling DSVM bytecode...",
                sd_root.display(),
                profiles_snapshot.len()
            )));
            self.as_mut().set_status_message(QString::from(
                "Syncing profiles to mounted duckyPad microSD...",
            ));

            let qt_thread = self.qt_thread();
            std::thread::spawn(move || {
                let res = write_profiles_to_sd_root(&sd_root, &profiles_snapshot);
                if res.is_ok() {
                    sync_profiles_to_device_cache(&serial_str, &profiles_snapshot);
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
                        }
                    }
                });
            });
            return true;
        }

        if let Ok(info) = query_connected_duckypad_info() {
            if info.pid == PID_DUCKYPAD_PRO {
                let disk_label = info.disk_label.clone();
                let dev_serial = info.serial.clone();
                let label_display = if disk_label.is_empty() {
                    "duckyPad Pro".to_string()
                } else {
                    disk_label.clone()
                };

                self.as_mut().set_sd_sync_busy(true);
                self.as_mut().set_sd_sync_progress(5);
                self.as_mut()
                    .set_sd_sync_result_state(QString::from("running"));
                self.as_mut().set_sd_sync_result_summary(QString::from(""));
                self.as_mut().set_sd_sync_step(QString::from(
                    "Preparing to sync profiles to duckyPad Pro microSD...",
                ));
                self.as_mut().set_sd_sync_log(QString::from(&format!(
                    "• Connected device: duckyPad Pro ({label_display}, SN {dev_serial})\n• Preparing {} profile(s) for USB Mass Storage sync...",
                    profiles_snapshot.len()
                )));
                self.as_mut().set_status_message(QString::from(
                    "Syncing profiles to duckyPad Pro microSD via USB MSC...",
                ));

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
                        qobject.as_mut().set_sd_sync_busy(false);
                        match res {
                            Ok(summary) => {
                                let short_fp = &summary.validated_fingerprint
                                    [..8.min(summary.validated_fingerprint.len())];
                                let summary_msg = format!(
                                    "Validated & Saved! {} profile(s) verified on duckyPad Pro ({label_display}) — {} file(s) updated, {} unchanged file(s) verified, read-back hash #{short_fp} confirmed.",
                                    summary.profiles_count,
                                    summary.files_written,
                                    summary.files_verified_unchanged
                                );
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
                                qobject.as_mut().set_connected(true);
                                qobject.as_mut().sync_active_profile_state();
                                qobject
                                    .as_mut()
                                    .set_status_message(QString::from(&summary_msg));
                            }
                            Err(err) => {
                                let prev = qobject.sd_sync_log().to_string();
                                let next = format!("{prev}\n✗ Error: {err}");
                                let err_msg =
                                    format!("Failed syncing profiles to duckyPad Pro SD: {err}");
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
                                qobject
                                    .as_mut()
                                    .set_status_message(QString::from(&err_msg));
                            }
                        }
                    });
                });
                return true;
            }
        }

        let msg =
            "Connect duckyPad Pro via USB or mount the microSD card to sync changes to hardware";
        self.as_mut()
            .set_sd_sync_result_state(QString::from("error"));
        self.as_mut()
            .set_sd_sync_step(QString::from("No duckyPad storage available"));
        self.as_mut()
            .set_sd_sync_result_summary(QString::from(msg));
        self.as_mut().set_status_message(QString::from(msg));
        false
    }

    pub fn reload_profiles_from_device_sd(mut self: Pin<&mut Self>) -> bool {
        if *self.sd_sync_busy() {
            return false;
        }

        if let Some(sd_root) = self
            .rust()
            .sd_root_path
            .clone()
            .or_else(find_mounted_duckypad_sd)
        {
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
                self.as_mut().set_sd_sync_step(QString::from(
                    "Validated: Profiles reloaded from microSD!",
                ));
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
            if info.pid == PID_DUCKYPAD_PRO {
                Some((info.disk_label, info.serial, info.active_profile))
            } else {
                None
            }
        } else if find_duckypad_msc_block_device("", true).is_some() {
            let cur_serial = self.serial_number().to_string();
            let cur_prof = (*self.active_profile() as u8).clamp(1, 64);
            Some((String::new(), cur_serial, cur_prof))
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
            self.as_mut().set_status_message(QString::from(
                "Mounting and scanning duckyPad microSD...",
            ));

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
                    qobject.as_mut().set_sd_sync_busy(false);
                    match res {
                        Ok(profiles) => {
                            let count = profiles.len();
                            let fp = compute_profiles_fingerprint(&profiles);
                            let short_fp = &fp[..8.min(fp.len())];
                            {
                                let rust = qobject.as_mut().rust_mut().get_mut();
                                if let Ok(mut sw) = rust.switcher_state.lock() {
                                    sw.align_rules_with_profiles(&profiles);
                                }
                                rust.profiles = profiles;
                            }
                            qobject
                                .as_mut()
                                .set_active_profile((active_prof as i32).clamp(1, 64));
                            qobject.as_mut().set_connected(true);
                            qobject.as_mut().sync_active_profile_state();

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
                            qobject
                                .as_mut()
                                .set_status_message(QString::from(&err_msg));
                        }
                    }
                });
            });
            return true;
        }

        let msg = "No mounted duckyPad microSD or connected duckyPad Pro found to reload from";
        self.as_mut()
            .set_sd_sync_result_state(QString::from("error"));
        self.as_mut()
            .set_sd_sync_step(QString::from("No duckyPad storage available"));
        self.as_mut()
            .set_sd_sync_result_summary(QString::from(msg));
        self.as_mut().set_status_message(QString::from(msg));
        false
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
            self.as_mut().sync_tray_state();
            if !self.rust().kwin_listener_started {
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

        // If a duckyPad SD card is mounted, load directly from it
        if self.rust().sd_root_path.is_none() {
            if let Some(sd_root) = find_mounted_duckypad_sd() {
                let _ = self
                    .as_mut()
                    .load_sd_directory(QString::from(sd_root.to_string_lossy().as_ref()));
            }
        }

        match query_connected_duckypad_info() {
            Ok(info) => {
                let model = if info.pid == PID_DUCKYPAD_PRO {
                    "duckyPad Pro"
                } else {
                    "duckyPad (2020)"
                };
                self.as_mut().set_model_name(QString::from(model));
                self.as_mut().set_connected(true);

                // Load the profile list for this connected duckyPad if not already loaded from a mounted SD card
                let prev_serial = self.serial_number().to_string();
                let need_profile_load = self.rust().sd_root_path.is_none()
                    && (self.rust().profiles.len() <= 1
                        || (info.serial != "-" && prev_serial != "-" && prev_serial != info.serial));

                if need_profile_load {
                    let loaded_opt = load_profiles_for_connected_device(info.pid, &info.serial)
                        .or_else(|| {
                            if info.pid == PID_DUCKYPAD_PRO && !info.disk_label.is_empty() {
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
                        let rust = self.as_mut().rust_mut().get_mut();
                        if let Ok(mut sw) = rust.switcher_state.lock() {
                            sw.align_rules_with_profiles(&loaded);
                        }
                        rust.profiles = loaded;
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
                self.as_mut().sync_active_profile_state();

                let count = self.rust().profiles.len();
                self.as_mut().set_status_message(QString::from(&format!(
                    "Connected to {model} (FW v{}) — {count} profile(s) loaded",
                    info.fw_version
                )));
                self.as_mut().sync_tray_state();
                if self.latest_firmware_version().to_string() == "-" {
                    self.as_mut().check_firmware_update(false);
                }
            }
            Err(msg) => {
                self.as_mut().set_connected(false);
                if self.rust().sd_root_path.is_none() {
                    self.as_mut().set_model_name(QString::from("Disconnected"));
                    self.as_mut().set_firmware_version(QString::from("-"));
                    self.as_mut().set_serial_number(QString::from("-"));
                    self.as_mut().set_latest_firmware_version(QString::from("-"));
                    self.as_mut().set_firmware_update_available(false);
                    self.as_mut().set_status_message(QString::from(&msg));
                }
                self.as_mut().sync_active_profile_state();
                self.as_mut().sync_tray_state();
            }
        }

        // Start the KWin Wayland Auto-Switcher D-Bus listener if not started yet
        if !self.rust().kwin_listener_started {
            self.as_mut().start_kwin_listener();
        } else {
            self.as_mut().apply_auto_switch_for_current_window();
        }
    }

    pub fn select_profile(mut self: Pin<&mut Self>, profile_number: i32) -> bool {
        let clamped = profile_number.clamp(1, 64);
        self.as_mut().set_active_profile(clamped);
        if is_demo_showcase_env() {
            let bound_cls = self
                .rust()
                .switcher_state
                .lock()
                .ok()
                .and_then(|sw| {
                    sw.rules
                        .iter()
                        .find(|r| r.enabled && r.target_profile == clamped as u8)
                        .map(|r| r.resource_class.clone())
                })
                .unwrap_or_else(|| "org.kde.plasmashell".to_string());
            self.as_mut()
                .set_active_window_class(QString::from(&bound_cls));
        }
        self.as_mut().sync_active_profile_state();

        if !is_demo_showcase_env() {
            // Also switch on hardware if connected over USB HID
            let _ = crate::kwin::auto_switcher::send_hid_switch_profile(clamped as u8);
        }

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.last_switched_profile = clamped as u8;
            }
        }

        let prof_name = self.active_profile_name().to_string();
        self.as_mut().set_status_message(QString::from(&format!(
            "Loaded Profile #{clamped}: {prof_name}"
        )));
        true
    }

    pub fn switch_profile(mut self: Pin<&mut Self>, profile_number: i32) -> bool {
        self.as_mut().select_profile(profile_number)
    }

    pub fn switch_profile_by_name(mut self: Pin<&mut Self>, profile_name: QString) -> bool {
        use crate::hardware::storage::{goto_profile_by_name, open_duckypad_hid};
        let name = profile_name.to_string();

        if let Some(num) = self
            .rust()
            .profiles
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(&name))
            .map(|p| p.number as i32)
        {
            self.as_mut().set_active_profile(num);
            self.as_mut().sync_active_profile_state();
        }

        match open_duckypad_hid().and_then(|dev| goto_profile_by_name(&dev, &name)) {
            Ok(()) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Switched to profile '{name}'"
                )));
                true
            }
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&e));
                false
            }
        }
    }

    pub fn create_new_profile(mut self: Pin<&mut Self>, profile_name: QString) -> bool {
        let trimmed = profile_name.to_string().trim().to_string();
        if trimmed.is_empty() {
            return false;
        }
        let clean_name: String = trimmed
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .take(16)
            .collect();
        if clean_name.is_empty() {
            return false;
        }

        let (next_num, saved_to_sd) = {
            let rust = self.as_mut().rust_mut().get_mut();
            let max_num = rust.profiles.iter().map(|p| p.number).max().unwrap_or(0);
            if max_num >= 64 {
                return false;
            }
            let next_num = max_num + 1;
            let is_dpp = rust
                .sd_root_path
                .as_ref()
                .map(|r| r.join("profile_info.txt").is_file())
                .unwrap_or(true);
            let folder_override = if is_dpp {
                Some(format!("profile_{clean_name}"))
            } else {
                None
            };

            let new_prof = DuckyProfile {
                number: next_num,
                name: clean_name.clone(),
                folder_name_override: folder_override,
                bg_color: [0, 170, 255],
                keydown_color: Some([255, 255, 255]),
                dim_unused_keys: true,
                ..Default::default()
            };

            let mut saved = false;
            if let Some(ref sd_root) = rust.sd_root_path {
                if new_prof.save_to_sd_root(sd_root).is_ok() {
                    saved = true;
                    let info_path = sd_root.join("profile_info.txt");
                    if info_path.is_file() {
                        if let Ok(mut existing) = fs::read_to_string(&info_path) {
                            if !existing.ends_with('\n') && !existing.is_empty() {
                                existing.push('\n');
                            }
                            existing.push_str(&format!("{next_num} {clean_name}\n"));
                            let _ = fs::write(&info_path, existing);
                        }
                    }
                }
            }

            rust.profiles.push(new_prof);
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if saved {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            (next_num as i32, saved)
        };

        self.as_mut().set_active_profile(next_num);
        self.as_mut().sync_active_profile_state();
        let suffix = if saved_to_sd { " (saved to SD)" } else { "" };
        self.as_mut().set_status_message(QString::from(&format!(
            "Created Profile #{next_num}: {clean_name}{suffix}"
        )));
        true
    }

    pub fn rename_profile(
        mut self: Pin<&mut Self>,
        profile_number: i32,
        new_name: QString,
    ) -> bool {
        if profile_number < 1 {
            return false;
        }
        let target_num = profile_number as u8;
        let raw = new_name.to_string();
        let clean_name: String = raw
            .trim()
            .chars()
            .filter(|c| *c != '/' && *c != '\\' && *c != ':' && !c.is_control())
            .take(16)
            .collect();
        if clean_name.is_empty() {
            self.as_mut()
                .set_status_message(QString::from("Profile name cannot be empty"));
            return false;
        }

        let duplicate_exists = self
            .rust()
            .profiles
            .iter()
            .any(|p| p.number != target_num && p.name.eq_ignore_ascii_case(&clean_name));
        if duplicate_exists {
            self.as_mut().set_status_message(QString::from(&format!(
                "A profile named '{clean_name}' already exists"
            )));
            return false;
        }

        let rename_res = {
            let rust = self.as_mut().rust_mut().get_mut();
            let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == target_num) else {
                return false;
            };
            let old_name = prof.name.clone();
            if old_name == clean_name {
                return true;
            }
            let old_folder = prof.folder_name();
            let old_legacy_folder = format!("profile{target_num}_{old_name}");

            prof.name = clean_name.clone();
            if prof.folder_name_override.is_some() {
                prof.folder_name_override = Some(format!("profile_{clean_name}"));
            }
            let new_folder = prof.folder_name();

            let mut saved_to_sd = false;
            if let Some(ref sd_root) = rust.sd_root_path {
                let new_dir = sd_root.join(&new_folder);
                for old_f in [&old_folder, &old_legacy_folder] {
                    let old_dir = sd_root.join(old_f);
                    if old_dir.is_dir() && old_dir != new_dir {
                        let _ = fs::rename(&old_dir, &new_dir);
                    }
                }
                if save_profile_order_to_sd(sd_root, &rust.profiles).is_ok() {
                    saved_to_sd = true;
                }
            }

            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if saved_to_sd {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            Some((old_name, saved_to_sd))
        };

        let Some((old_name, saved_to_sd)) = rename_res else {
            return false;
        };

        self.as_mut().sync_active_profile_state();
        let suffix = if saved_to_sd { " (saved to SD)" } else { "" };
        self.as_mut().set_status_message(QString::from(&format!(
            "Renamed Profile #{target_num} from '{old_name}' to '{clean_name}'{suffix}"
        )));
        true
    }

    pub fn delete_profile(mut self: Pin<&mut Self>, profile_number: i32) -> bool {
        if profile_number < 1 {
            return false;
        }
        let target_num = profile_number as u8;
        let active_old_num = *self.active_profile() as u8;
        let mut new_active_num = active_old_num as i32;
        let mut new_fallback_num: Option<i32> = None;

        let delete_result = {
            let rust = self.as_mut().rust_mut().get_mut();
            if rust.profiles.len() <= 1 {
                None
            } else if let Some(remove_idx) =
                rust.profiles.iter().position(|p| p.number == target_num)
            {
                let deleted_prof = rust.profiles.remove(remove_idx);
                let deleted_name = deleted_prof.name.clone();
                let deleted_folder = deleted_prof.folder_name();
                let deleted_legacy_folder =
                    format!("profile{}_{}", deleted_prof.number, deleted_prof.name);

                let mut old_to_new: HashMap<u8, u8> =
                    HashMap::with_capacity(rust.profiles.len());
                for (idx, prof) in rust.profiles.iter_mut().enumerate() {
                    let old_num = prof.number;
                    let new_num = (idx + 1) as u8;
                    old_to_new.insert(old_num, new_num);
                    if old_num != new_num {
                        if let Some(ref sd_root) = rust.sd_root_path {
                            if prof.folder_name_override.is_none() {
                                let old_dir =
                                    sd_root.join(format!("profile{old_num}_{}", prof.name));
                                let new_dir =
                                    sd_root.join(format!("profile{new_num}_{}", prof.name));
                                if old_dir.is_dir() && !new_dir.exists() {
                                    let _ = fs::rename(&old_dir, &new_dir);
                                }
                            }
                        }
                    }
                    prof.number = new_num;
                }

                if active_old_num == target_num {
                    let clamped_idx = remove_idx.min(rust.profiles.len().saturating_sub(1));
                    new_active_num = (clamped_idx + 1) as i32;
                } else if let Some(&mapped_active) = old_to_new.get(&active_old_num) {
                    new_active_num = mapped_active as i32;
                }

                if let Ok(mut sw) = rust.switcher_state.lock() {
                    let before_len = sw.rules.len();
                    sw.rules.retain(|r| r.target_profile != target_num);
                    let mut rules_changed = sw.rules.len() != before_len;
                    for rule in &mut sw.rules {
                        if let Some(&mapped_target) = old_to_new.get(&rule.target_profile) {
                            if rule.target_profile != mapped_target {
                                rule.target_profile = mapped_target;
                                rules_changed = true;
                            }
                        }
                    }
                    if let Some(fb) = sw.default_profile {
                        if fb == target_num {
                            sw.default_profile = Some(1);
                            new_fallback_num = Some(1);
                            rules_changed = true;
                        } else if let Some(&mapped_fb) = old_to_new.get(&fb) {
                            if fb != mapped_fb {
                                sw.default_profile = Some(mapped_fb);
                                new_fallback_num = Some(mapped_fb as i32);
                                rules_changed = true;
                            }
                        }
                    }
                    if rules_changed || sw.loaded_from_disk {
                        let _ = sw.save_to_disk();
                    }
                }

                let mut saved_to_sd = false;
                if let Some(ref sd_root) = rust.sd_root_path {
                    for folder in [&deleted_folder, &deleted_legacy_folder] {
                        let dir_path = sd_root.join(folder);
                        if dir_path.is_dir() {
                            let _ = fs::remove_dir_all(&dir_path);
                        }
                    }
                    if save_profile_order_to_sd(sd_root, &rust.profiles).is_ok() {
                        saved_to_sd = true;
                    }
                }

                let sn = rust.serial_number.to_string();
                sync_profiles_to_device_cache(&sn, &rust.profiles);
                if saved_to_sd {
                    mark_profiles_sd_synced(&sn, &rust.profiles);
                }
                Some((deleted_name, saved_to_sd))
            } else {
                return false;
            }
        };

        let Some((deleted_name, saved_to_sd)) = delete_result else {
            self.as_mut().set_status_message(QString::from(
                "Cannot delete the last remaining profile",
            ));
            return false;
        };

        if let Some(fb) = new_fallback_num {
            self.as_mut().set_default_fallback_profile(fb);
        }
        self.as_mut().set_active_profile(new_active_num);
        if *self.connected() {
            let _ = crate::kwin::auto_switcher::send_hid_switch_profile(new_active_num as u8);
        }
        self.as_mut().sync_active_profile_state();

        let suffix = if saved_to_sd {
            " (removed from SD)"
        } else {
            ""
        };
        self.as_mut().set_status_message(QString::from(&format!(
            "Deleted Profile #{target_num}: {deleted_name}{suffix}"
        )));
        true
    }

    pub fn reorder_profile(
        mut self: Pin<&mut Self>,
        from_index: i32,
        to_index: i32,
    ) -> bool {
        if from_index < 0 || to_index < 0 || from_index == to_index {
            return false;
        }
        let from_idx = from_index as usize;
        let to_idx = to_index as usize;

        let active_old_num = *self.active_profile() as u8;
        let mut new_active_num = active_old_num as i32;
        let mut new_fallback_num: Option<i32> = None;

        let (moved_name, moved_new_num, saved_to_sd) = {
            let rust = self.as_mut().rust_mut().get_mut();
            if from_idx >= rust.profiles.len() || to_idx >= rust.profiles.len() {
                return false;
            }

            let moved_prof = rust.profiles.remove(from_idx);
            let moved_name = moved_prof.name.clone();
            rust.profiles.insert(to_idx, moved_prof);

            let mut old_to_new: HashMap<u8, u8> = HashMap::with_capacity(rust.profiles.len());
            for (idx, prof) in rust.profiles.iter_mut().enumerate() {
                let new_num = (idx + 1) as u8;
                old_to_new.insert(prof.number, new_num);
                prof.number = new_num;
            }

            let moved_new_num = (to_idx + 1) as u8;
            if let Some(&mapped_active) = old_to_new.get(&active_old_num) {
                new_active_num = mapped_active as i32;
            }

            if let Ok(mut sw) = rust.switcher_state.lock() {
                let mut rules_changed = false;
                for rule in &mut sw.rules {
                    if let Some(&mapped_target) = old_to_new.get(&rule.target_profile) {
                        if rule.target_profile != mapped_target {
                            rule.target_profile = mapped_target;
                            rules_changed = true;
                        }
                    }
                }
                if let Some(fb) = sw.default_profile {
                    if let Some(&mapped_fb) = old_to_new.get(&fb) {
                        if fb != mapped_fb {
                            sw.default_profile = Some(mapped_fb);
                            new_fallback_num = Some(mapped_fb as i32);
                            rules_changed = true;
                        }
                    }
                }
                if rules_changed || sw.loaded_from_disk {
                    let _ = sw.save_to_disk();
                }
            }

            let saved = if let Some(ref sd_root) = rust.sd_root_path {
                save_profile_order_to_sd(sd_root, &rust.profiles).is_ok()
            } else {
                false
            };

            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if saved {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            (moved_name, moved_new_num, saved)
        };

        if let Some(fb) = new_fallback_num {
            self.as_mut().set_default_fallback_profile(fb);
        }
        self.as_mut().set_active_profile(new_active_num);
        self.as_mut().sync_active_profile_state();

        let suffix = if saved_to_sd {
            " (saved to profile_info.txt)"
        } else {
            ""
        };
        self.as_mut().set_status_message(QString::from(&format!(
            "Moved '{moved_name}' to Profile #{moved_new_num}{suffix}"
        )));
        true
    }

    pub fn set_key_rgb(
        mut self: Pin<&mut Self>,
        key_index: i32,
        r: i32,
        g: i32,
        b: i32,
    ) -> bool {
        let slot_idx = (key_index.clamp(0, 19) + 1) as usize;
        let rgb = [
            r.clamp(0, 255) as u8,
            g.clamp(0, 255) as u8,
            b.clamp(0, 255) as u8,
        ];

        let active_num = *self.active_profile() as u8;
        let saved_to_sd = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            let res = if let Some(prof) = rust.profiles.get_mut(prof_idx) {
                prof.ensure_slot(slot_idx).custom_rgb = Some(rgb);
                if let Some(root) = sd_root {
                    prof.save_to_sd_root(&root).is_ok()
                } else {
                    false
                }
            } else {
                false
            };
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if res {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            res
        };

        self.as_mut().sync_active_profile_state();

        let mut hid_ok = false;
        if let Ok((device, _)) = Self::open_duckypad() {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x03;
            tx[2] = HID_CMD_SET_LED_SINGLE;
            tx[3] = key_index.clamp(0, 19) as u8;
            tx[4] = rgb[0];
            tx[5] = rgb[1];
            tx[6] = rgb[2];
            hid_ok = device.write(&tx).is_ok();
            if hid_ok {
                let mut rx = [0u8; 64];
                let _ = device.read_timeout(&mut rx, 200);
            }
        }

        let suffix = if saved_to_sd { " (saved to SD)" } else { "" };
        self.as_mut().set_status_message(QString::from(&format!(
            "Key {slot_idx} RGB -> ({}, {}, {}){suffix}",
            rgb[0], rgb[1], rgb[2]
        )));
        hid_ok || saved_to_sd
    }

    pub fn clear_key_custom_rgb(mut self: Pin<&mut Self>, key_index: i32) -> bool {
        let slot_idx = (key_index.clamp(0, 25) + 1) as usize;
        let active_num = *self.active_profile() as u8;
        let (saved_to_sd, bg_rgb) = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            let mut bg = [0u8, 170, 255];
            let res = if let Some(prof) = rust.profiles.get_mut(prof_idx) {
                bg = prof.bg_color;
                if let Some(slot) = prof.slots.get_mut(&slot_idx) {
                    slot.custom_rgb = None;
                }
                if prof
                    .slots
                    .get(&slot_idx)
                    .map(|s| s.is_effectively_empty())
                    .unwrap_or(false)
                {
                    prof.slots.remove(&slot_idx);
                }
                if let Some(root) = sd_root {
                    prof.save_to_sd_root(&root).is_ok()
                } else {
                    false
                }
            } else {
                false
            };
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if res {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            (res, bg)
        };

        self.as_mut().sync_active_profile_state();

        if (1..=20).contains(&slot_idx) {
            set_single_key_led_hid(slot_idx, bg_rgb);
        }

        let suffix = if saved_to_sd { " (saved to SD)" } else { "" };
        self.as_mut().set_status_message(QString::from(&format!(
            "Key {slot_idx} reset to Profile Default RGB ({}, {}, {}){suffix}",
            bg_rgb[0], bg_rgb[1], bg_rgb[2]
        )));
        true
    }

    pub fn set_profile_bg_color(
        mut self: Pin<&mut Self>,
        profile_number: i32,
        r: i32,
        g: i32,
        b: i32,
    ) -> bool {
        let target_num = if profile_number > 0 {
            profile_number as u8
        } else {
            (*self.active_profile()).max(1) as u8
        };
        let rgb = [
            r.clamp(0, 255) as u8,
            g.clamp(0, 255) as u8,
            b.clamp(0, 255) as u8,
        ];

        let mut updated_slots_for_hid: Vec<usize> = Vec::new();
        let (found, prof_name, saved_to_sd) = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            if let Some(prof) = rust
                .profiles
                .iter_mut()
                .find(|p| p.number == target_num)
            {
                let old_bg = prof.bg_color;
                prof.bg_color = rgb;
                for slot_idx in 1..=MAX_INPUT_SLOTS {
                    if let Some(slot) = prof.slots.get_mut(&slot_idx) {
                        if slot.custom_rgb == Some(old_bg) {
                            slot.custom_rgb = None;
                        }
                        let has_script = !slot.on_press_script.trim().is_empty()
                            || !slot.on_release_script.trim().is_empty()
                            || !slot.label_line1.is_empty()
                            || !slot.label_line2.is_empty();
                        if slot.custom_rgb.is_none()
                            && (has_script || !prof.dim_unused_keys)
                            && (1..=20).contains(&slot_idx)
                        {
                            updated_slots_for_hid.push(slot_idx);
                        }
                    } else if !prof.dim_unused_keys && (1..=20).contains(&slot_idx) {
                        updated_slots_for_hid.push(slot_idx);
                    }
                }
                let p_name = prof.name.clone();
                let saved = if let Some(root) = sd_root {
                    prof.save_to_sd_root(&root).is_ok()
                } else {
                    false
                };
                let sn = rust.serial_number.to_string();
                sync_profiles_to_device_cache(&sn, &rust.profiles);
                if saved {
                    mark_profiles_sd_synced(&sn, &rust.profiles);
                }
                (true, p_name, saved)
            } else {
                (false, String::new(), false)
            }
        };

        if !found {
            return false;
        }

        self.as_mut().sync_active_profile_state();

        if target_num == (*self.active_profile()).max(1) as u8 {
            if let Ok((device, _)) = Self::open_duckypad() {
                for slot_idx in updated_slots_for_hid {
                    let mut tx = [0u8; 64];
                    tx[0] = HID_REPORT_ID_OUT;
                    tx[1] = 0x03;
                    tx[2] = HID_CMD_SET_LED_SINGLE;
                    tx[3] = (slot_idx - 1) as u8;
                    tx[4] = rgb[0];
                    tx[5] = rgb[1];
                    tx[6] = rgb[2];
                    if device.write(&tx).is_ok() {
                        let mut rx = [0u8; 64];
                        let _ = device.read_timeout(&mut rx, 60);
                    }
                }
            }
        }

        let suffix = if saved_to_sd {
            " (saved to SD)"
        } else {
            " (unsynced — click Sync to SD to persist on duckyPad)"
        };
        self.as_mut().set_status_message(QString::from(&format!(
            "Profile #{target_num} ('{prof_name}') default LED color -> RGB({}, {}, {}){suffix}",
            rgb[0], rgb[1], rgb[2]
        )));
        true
    }

    pub fn apply_ui_text_scale(mut self: Pin<&mut Self>, scale_percent: i32) {
        self.as_mut()
            .apply_area_text_scale(QString::from("all"), scale_percent);
    }

    pub fn apply_area_text_scale(
        mut self: Pin<&mut Self>,
        area: QString,
        scale_percent: i32,
    ) {
        let clamped = scale_percent.clamp(75, 160);
        let key = area.to_string().trim().to_ascii_lowercase();
        let label = match key.as_str() {
            "sidebar" => "Profiles & Auto-Switcher Sidebar",
            "canvas" => "Hardware Pad & Key Labels",
            "inspector" => "Key Inspector & Controls",
            "editor" => "duckyScript Code Editor",
            "statusbar" => "Bottom Status Bar",
            "eventlog" => "System Event Log Window",
            _ => "All Application Areas",
        };

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                match key.as_str() {
                    "sidebar" => sw.sidebar_text_scale = clamped,
                    "canvas" => sw.canvas_text_scale = clamped,
                    "inspector" => sw.inspector_text_scale = clamped,
                    "editor" => sw.editor_text_scale = clamped,
                    "statusbar" => sw.status_bar_text_scale = clamped,
                    "eventlog" => sw.event_log_text_scale = clamped,
                    _ => {
                        sw.ui_text_scale = clamped;
                        sw.sidebar_text_scale = clamped;
                        sw.canvas_text_scale = clamped;
                        sw.inspector_text_scale = clamped;
                        sw.editor_text_scale = clamped;
                        sw.status_bar_text_scale = clamped;
                        sw.event_log_text_scale = clamped;
                    }
                }
                let _ = sw.save_to_disk();
            }
        }

        match key.as_str() {
            "sidebar" => self.as_mut().set_sidebar_text_scale(clamped),
            "canvas" => self.as_mut().set_canvas_text_scale(clamped),
            "inspector" => self.as_mut().set_inspector_text_scale(clamped),
            "editor" => self.as_mut().set_editor_text_scale(clamped),
            "statusbar" => self.as_mut().set_status_bar_text_scale(clamped),
            "eventlog" => self.as_mut().set_event_log_text_scale(clamped),
            _ => {
                self.as_mut().set_ui_text_scale(clamped);
                self.as_mut().set_sidebar_text_scale(clamped);
                self.as_mut().set_canvas_text_scale(clamped);
                self.as_mut().set_inspector_text_scale(clamped);
                self.as_mut().set_editor_text_scale(clamped);
                self.as_mut().set_status_bar_text_scale(clamped);
                self.as_mut().set_event_log_text_scale(clamped);
            }
        }

        self.as_mut().set_status_message(QString::from(&format!(
            "{label} text size set to {clamped}%"
        )));
    }

    pub fn reset_all_text_scales(mut self: Pin<&mut Self>) {
        self.as_mut()
            .apply_area_text_scale(QString::from("all"), 100);
        self.as_mut().set_status_message(QString::from(
            "All application area text sizes reset to 100%",
        ));
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
            let _ = device.read_timeout(&mut rx, 200);
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
            let _ = device.read_timeout(&mut rx, 200);
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
        if let Some(sd_root) = find_mounted_duckypad_sd() {
            let path_str = sd_root.to_string_lossy().to_string();
            if self.as_mut().load_sd_directory(QString::from(&path_str)) {
                return QString::from(&path_str);
            }
            return QString::from("");
        }

        if let Ok(info) = query_connected_duckypad_info() {
            if info.pid == PID_DUCKYPAD_PRO && !info.disk_label.is_empty() {
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
                        "Synced {count} profile(s) from connected duckyPad Pro ({})",
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
        let cleaned = raw_path
            .strip_prefix("file://")
            .unwrap_or(&raw_path)
            .trim();
        if cleaned.is_empty() {
            return false;
        }

        let sd_root = PathBuf::from(cleaned);
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
                            mark_profiles_sd_synced(&rust.serial_number.to_string(), &rust.profiles);
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

                    let msg = format!(
                        "No duckyPad profile folders found in {}",
                        sd_root.display()
                    );
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
                    if let Ok(mut sw) = rust.switcher_state.lock() {
                        sw.align_rules_with_profiles(&profiles);
                    }
                    let sn_for_cache = dpp_cfg
                        .serial_number
                        .as_deref()
                        .filter(|s| !s.is_empty())
                        .unwrap_or(&current_serial);
                    sync_profiles_to_device_cache(sn_for_cache, &profiles);
                    mark_profiles_sd_synced(sn_for_cache, &profiles);
                    rust.sd_root_path = Some(sd_root.clone());
                    rust.profiles = profiles;
                }

                if !*self.connected() {
                    if let Some(fw) = dpp_cfg.fw_ver.as_deref().filter(|s| !s.is_empty()) {
                        self.as_mut().set_firmware_version(QString::from(fw));
                        self.as_mut()
                            .set_model_name(QString::from("duckyPad Pro (SD Mode)"));
                    } else {
                        self.as_mut()
                            .set_model_name(QString::from("duckyPad (SD Mode)"));
                    }
                    if let Some(sn) = dpp_cfg.serial_number.as_deref().filter(|s| !s.is_empty()) {
                        self.as_mut().set_serial_number(QString::from(sn));
                    }
                }

                self.as_mut()
                    .set_sd_mount_path(QString::from(sd_root.to_string_lossy().as_ref()));
                self.as_mut().set_active_profile(chosen_num);
                self.as_mut().sync_active_profile_state();

                let msg = format!(
                    "Loaded {count} profile(s) from {}",
                    sd_root.display()
                );
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

    pub fn get_slot_script(&self, slot_number: i32, is_release: bool) -> QString {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;
        let rust = self.rust();

        if let Some(prof) = rust
            .profiles
            .iter()
            .find(|p| p.number == active_num)
            .or_else(|| rust.profiles.first())
        {
            if let Some(slot) = prof.slots.get(&slot_idx) {
                let script = if is_release {
                    &slot.on_release_script
                } else {
                    &slot.on_press_script
                };
                return QString::from(script);
            }
        }
        QString::from("")
    }

    pub fn get_slot_meta_json(&self, slot_number: i32) -> QString {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;
        let rust = self.rust();

        if let Some(prof) = rust
            .profiles
            .iter()
            .find(|p| p.number == active_num)
            .or_else(|| rust.profiles.first())
        {
            if let Some(slot) = prof.slots.get(&slot_idx) {
                let rgb = slot.custom_rgb.unwrap_or(prof.bg_color);
                let has_web_api = has_web_api_config(&slot.on_press_script)
                    || has_web_api_config(&slot.on_release_script);
                let meta = json!({
                    "slot": slot_idx,
                    "l1": slot.label_line1,
                    "l2": slot.label_line2,
                    "wl1": slot.widget_label_line1,
                    "wl2": slot.widget_label_line2,
                    "r": rgb[0],
                    "g": rgb[1],
                    "b": rgb[2],
                    "hasCustomRgb": slot.custom_rgb.is_some(),
                    "allowAbort": slot.allow_abort,
                    "disableAutorepeat": slot.disable_autorepeat,
                    "hasRelease": !slot.on_release_script.trim().is_empty(),
                    "hasWebApi": has_web_api,
                });
                return QString::from(&meta.to_string());
            } else {
                let meta = json!({
                    "slot": slot_idx,
                    "l1": "",
                    "l2": "",
                    "wl1": "",
                    "wl2": "",
                    "r": prof.bg_color[0],
                    "g": prof.bg_color[1],
                    "b": prof.bg_color[2],
                    "hasCustomRgb": false,
                    "allowAbort": false,
                    "disableAutorepeat": false,
                    "hasRelease": false,
                    "hasWebApi": false,
                });
                return QString::from(&meta.to_string());
            }
        }
        QString::from("{}")
    }

    pub fn check_script_syntax(
        &self,
        script: QString,
        allow_abort: bool,
        disable_autorepeat: bool,
    ) -> QString {
        let text = script.to_string();
        if text.trim().is_empty() {
            let empty_res = json!({
                "ok": true,
                "empty": true,
                "bytes": 0,
                "errorLine": 0,
                "errorMsg": "",
            });
            return QString::from(&empty_res.to_string());
        }
        let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
            &text,
            allow_abort,
            disable_autorepeat,
        );
        let res = json!({
            "ok": compiled.is_success,
            "empty": false,
            "bytes": compiled.bin_array.len(),
            "errorLine": compiled.error_line_number_starting_from_1,
            "errorMsg": compiled.error_comment,
            "errorLineStr": compiled.error_line_str,
        });
        QString::from(&res.to_string())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save_slot_config(
        mut self: Pin<&mut Self>,
        slot_number: i32,
        label1: QString,
        label2: QString,
        on_press_script: QString,
        on_release_script: QString,
        allow_abort: bool,
        disable_autorepeat: bool,
    ) -> bool {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;

        let take_5_visible = |raw: &str| -> String {
            let mut out = String::new();
            let mut count = 0usize;
            for ch in raw.chars() {
                if matches!(ch, '\u{FE0E}' | '\u{FE0F}' | '\u{200D}') {
                    if count > 0 && count <= 5 {
                        out.push(ch);
                    }
                    continue;
                }
                if count >= 5 {
                    break;
                }
                out.push(ch);
                count += 1;
            }
            out
        };
        let l1: String = take_5_visible(&label1.to_string());
        let l2: String = take_5_visible(&label2.to_string());
        let press = on_press_script.to_string();
        let release = on_release_script.to_string();

        // Pre-validate with DSVM v2 compiler so errors are caught before modifying state
        let mut press_dsb_bytes = 0usize;
        if !press.trim().is_empty() {
            let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                &press,
                allow_abort,
                disable_autorepeat,
            );
            if !compiled.is_success {
                let err_msg = format!(
                    "Compile Error in key{slot_idx}.txt (Line {}): {}",
                    compiled.error_line_number_starting_from_1, compiled.error_comment
                );
                self.as_mut().set_status_message(QString::from(&err_msg));
                return false;
            }
            press_dsb_bytes = compiled.bin_array.len();
        }

        let mut release_dsb_bytes = 0usize;
        if !release.trim().is_empty() {
            let compiled = crate::vm::compiler::compile_duckyscript_to_dsb(
                &release,
                allow_abort,
                disable_autorepeat,
            );
            if !compiled.is_success {
                let err_msg = format!(
                    "Compile Error in key{slot_idx}-release.txt (Line {}): {}",
                    compiled.error_line_number_starting_from_1, compiled.error_comment
                );
                self.as_mut().set_status_message(QString::from(&err_msg));
                return false;
            }
            release_dsb_bytes = compiled.bin_array.len();
        }

        let save_result: Result<Option<PathBuf>, String> = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            let res = if let Some(prof) = rust.profiles.get_mut(prof_idx) {
                let entry = prof.ensure_slot(slot_idx);
                entry.label_line1 = l1;
                entry.label_line2 = l2;
                entry.on_press_script = press;
                entry.on_release_script = release;
                entry.allow_abort = allow_abort;
                entry.disable_autorepeat = disable_autorepeat;
                if entry.is_effectively_empty() {
                    prof.slots.remove(&slot_idx);
                }

                if let Some(root) = sd_root {
                    prof.save_to_sd_root(&root)
                        .map(Some)
                        .map_err(|e| format!("Failed saving to SD card: {e}"))
                } else {
                    Ok(None)
                }
            } else {
                Err("No active profile loaded".to_string())
            };
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if matches!(res, Ok(Some(_))) {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            res
        };

        self.as_mut().sync_active_profile_state();

        let dsb_summary = match (press_dsb_bytes > 0, release_dsb_bytes > 0) {
            (true, true) => format!(
                "key{slot_idx}.txt/.dsb ({press_dsb_bytes} B) + key{slot_idx}-release.txt/.dsb ({release_dsb_bytes} B)"
            ),
            (true, false) => format!("key{slot_idx}.txt + key{slot_idx}.dsb ({press_dsb_bytes} B)"),
            (false, true) => format!(
                "key{slot_idx}-release.txt + key{slot_idx}-release.dsb ({release_dsb_bytes} B)"
            ),
            (false, false) => format!("Slot {slot_idx} metadata"),
        };

        match save_result {
            Ok(Some(profile_dir)) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Saved {dsb_summary} & config.txt to {}",
                    profile_dir.display()
                )));
                true
            }
            Ok(None) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Compiled {dsb_summary} in profile cache (use 'Sync to SD' to write to duckyPad)"
                )));
                true
            }
            Err(err) => {
                self.as_mut().set_status_message(QString::from(&err));
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
                self.as_mut().set_status_message(QString::from(&format!(
                    "Error saving profile: {e}"
                )));
                false
            }
        }
    }

    pub fn start_kwin_listener(mut self: Pin<&mut Self>) {
        if self.rust().kwin_listener_started {
            return;
        }
        self.as_mut().rust_mut().get_mut().kwin_listener_started = true;

        let state = self.rust().switcher_state.clone();
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let qt_thread = self.qt_thread();
        let hotplug_qt_thread = self.qt_thread();
        let web_api_qt_thread = self.qt_thread();

        std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Runtime::new() else {
                return;
            };
            rt.block_on(async move {
                let switcher_state = state.clone();
                tokio::spawn(async move {
                    let _ =
                        crate::kwin::auto_switcher::run_kwin_auto_switcher(switcher_state, tx)
                            .await;
                });

                // Background Hybrid Web API Trigger Listener (PGV _GV31 + evdev F13..F24)
                tokio::spawn(async move {
                    let mut interval =
                        tokio::time::interval(std::time::Duration::from_millis(45));
                    let mut evdev_listener = DuckypadEvdevListener::new();
                    let mut idle_ticks = 0u32;
                    loop {
                        interval.tick().await;
                        if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                            .load(std::sync::atomic::Ordering::SeqCst)
                        {
                            continue;
                        }

                        // 1. Poll evdev F13..F24 sentinel keys (fallback for duckyPad 2020 / fkey mode)
                        for fkey_name in evdev_listener.poll_fkey_triggers() {
                            let _ = web_api_qt_thread.queue(move |mut qobject| {
                                qobject.as_mut().handle_hardware_fkey_web_api_trigger(&fkey_name);
                            });
                        }

                        // 2. Poll silent PGV _GV31 hardware trigger (0x444Bxxxx)
                        if !crate::hardware::storage::is_duckypad_hid_present() {
                            idle_ticks = idle_ticks.saturating_add(1);
                            if idle_ticks % 10 != 0 {
                                continue;
                            }
                        } else {
                            idle_ticks = 0;
                        }

                        if let Some((encoded_prof, slot_num, is_release)) =
                            poll_and_clear_pgv31_trigger()
                        {
                            let _ = web_api_qt_thread.queue(move |mut qobject| {
                                qobject.as_mut().handle_hardware_pgv_web_api_trigger(
                                    encoded_prof,
                                    slot_num,
                                    is_release,
                                );
                            });
                        }
                    }
                });

                tokio::spawn(async move {
                    let mut interval =
                        tokio::time::interval(std::time::Duration::from_millis(400));
                    loop {
                        interval.tick().await;
                        if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                            .load(std::sync::atomic::Ordering::SeqCst)
                        {
                            continue;
                        }
                        let hid_present = crate::hardware::storage::is_duckypad_hid_present();
                        let hw_active_prof = if hid_present {
                            poll_hardware_active_profile()
                        } else {
                            None
                        };
                        let _ = hotplug_qt_thread.queue(move |mut qobject| {
                            if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                                .load(std::sync::atomic::Ordering::SeqCst)
                            {
                                return;
                            }

                            let mut sd_unmounted = false;
                            if let Some(ref sd_root) = qobject.rust().sd_root_path.clone() {
                                if !crate::hardware::storage::is_duckypad_sd_root(sd_root) {
                                    qobject.as_mut().rust_mut().get_mut().sd_root_path = None;
                                    qobject.as_mut().set_sd_mount_path(QString::from(""));
                                    sd_unmounted = true;
                                }
                            } else if let Some(mounted) = find_mounted_duckypad_sd() {
                                let mounted_str = mounted.to_string_lossy().to_string();
                                qobject.as_mut().rust_mut().get_mut().sd_root_path = Some(mounted);
                                qobject.as_mut().set_sd_mount_path(QString::from(&mounted_str));
                            }

                            let was_connected = *qobject.connected();
                            if was_connected && !hid_present {
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
                            } else if !was_connected && hid_present {
                                qobject.as_mut().refresh_device();
                            } else if was_connected && hid_present {
                                if let Some(hw_prof) = hw_active_prof {
                                    let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                                    let last_sw_ms = crate::kwin::auto_switcher::LAST_SW_PROFILE_SWITCH_MS
                                        .load(std::sync::atomic::Ordering::SeqCst);
                                    let last_sw_target = crate::kwin::auto_switcher::LAST_SW_PROFILE_TARGET
                                        .load(std::sync::atomic::Ordering::SeqCst);
                                    let in_sw_cooldown = now_ms.saturating_sub(last_sw_ms) < 1400
                                        && last_sw_target >= 1
                                        && hw_prof != last_sw_target;

                                    let current_prof = *qobject.active_profile() as u8;
                                    let max_prof = qobject.rust().profiles.len() as u8;
                                    if !in_sw_cooldown
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
                                        qobject.as_mut().sync_active_profile_state();
                                        let msg = format!(
                                            "Followed hardware profile switch → Profile #{hw_prof} ({prof_name})"
                                        );
                                        record_system_event("INFO", "PROFILE", &msg);
                                        qobject.as_mut().sync_event_log_properties();
                                        qobject.as_mut().set_status_message(QString::from(&msg));
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
                });

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
                            .map(|sw| sw.paused_for_editing)
                            .unwrap_or(false);

                        if !is_paused {
                            if let Some(prof) = evt.matched_profile {
                                if *qobject.active_profile() != prof as i32 {
                                    record_system_event(
                                        "INFO",
                                        "AUTOSWITCH",
                                        format!(
                                            "Auto-switched to Profile #{} for window '{}' (\"{}\")",
                                            prof, evt.resource_class, evt.caption
                                        ),
                                    );
                                    qobject.as_mut().set_active_profile(prof as i32);
                                    qobject.as_mut().sync_active_profile_state();
                                    qobject.as_mut().sync_event_log_properties();
                                } else {
                                    qobject.as_mut().sync_tray_state();
                                }
                            } else {
                                qobject.as_mut().sync_tray_state();
                            }
                        } else {
                            qobject.as_mut().sync_tray_state();
                        }
                    });
                }
            });
        });
    }

    fn apply_auto_switch_for_current_window(mut self: Pin<&mut Self>) {
        let active_class = self.active_window_class().to_string();
        let active_title = self.active_window_title().to_string();
        if active_class.is_empty() || active_class == "Listening for KWin..." {
            return;
        }

        let mut matched_now: Option<u8> = None;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if sw.enabled && !sw.paused_for_editing {
                    matched_now = sw.match_profile_for_window(&active_class, &active_title);
                    if let Some(target) = matched_now {
                        if crate::kwin::auto_switcher::send_hid_switch_profile(target) {
                            sw.last_switched_profile = target;
                        }
                    }
                }
            }
        }

        if let Some(prof) = matched_now {
            if *self.active_profile() != prof as i32 {
                record_system_event(
                    "INFO",
                    "AUTOSWITCH",
                    format!(
                        "Auto-switched to Profile #{} for window '{}' (\"{}\")",
                        prof, active_class, active_title
                    ),
                );
                self.as_mut().set_active_profile(prof as i32);
                self.as_mut().sync_active_profile_state();
                self.as_mut().sync_event_log_properties();
            }
        }
    }

    pub fn set_script_editor_open(mut self: Pin<&mut Self>, open: bool) {
        let rust = self.as_mut().rust_mut().get_mut();
        if let Ok(mut sw) = rust.switcher_state.lock() {
            sw.paused_for_editing = open;
        }
    }

    pub fn set_auto_switch_config(
        mut self: Pin<&mut Self>,
        enabled: bool,
        default_profile: i32,
    ) {
        let fallback = if (1..=64).contains(&default_profile) {
            Some(default_profile as u8)
        } else {
            None
        };

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.enabled = enabled;
                sw.default_profile = fallback;
                let _ = sw.save_to_disk();
            }
        }

        self.as_mut().set_auto_switch_enabled(enabled);
        self.as_mut()
            .set_default_fallback_profile(fallback.unwrap_or(0) as i32);
        self.as_mut().apply_auto_switch_for_current_window();
        self.as_mut().sync_active_profile_state();
    }

    pub fn set_auto_switch_timings(
        mut self: Pin<&mut Self>,
        settle_ms: i32,
        dwell_ms: i32,
    ) {
        let clamped_settle = settle_ms.clamp(0, 5000);
        let clamped_dwell = dwell_ms.clamp(0, 5000);

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.mouse_settle_ms = clamped_settle;
                sw.window_dwell_ms = clamped_dwell;
                let _ = sw.save_to_disk();
            }
        }

        self.as_mut().set_auto_switch_settle_ms(clamped_settle);
        self.as_mut().set_auto_switch_dwell_ms(clamped_dwell);
        self.as_mut().set_status_message(QString::from(&format!(
            "Auto-Switcher timings set: mouse settle = {} ms, window dwell = {} ms",
            clamped_settle, clamped_dwell
        )));
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
        self.as_mut().set_last_picked_window_class(QString::from(""));
        self.as_mut().set_last_picked_window_title(QString::from(""));
        self.as_mut().set_window_picker_armed(true);
        self.as_mut().set_status_message(QString::from(
            "🎯 Window Picker Armed — Click any open window with the crosshair cursor",
        ));

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Runtime::new() else {
                return;
            };
            let result = rt.block_on(crate::kwin::auto_switcher::pick_window_via_kwin_dbus());
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
                        qobject
                            .as_mut()
                            .set_status_message(QString::from(&err));
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
        mut self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
        resource_class: QString,
        title_contains: QString,
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
            favicon_url: fav_opt,
            target_profile: target,
        };

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
                let _ = sw.save_to_disk();
            }
        }

        self.as_mut().apply_auto_switch_for_current_window();
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "Saved Auto-Switcher rule: '{class_str}' → Profile #{target}"
        )));
        true
    }

    pub fn remove_auto_switch_rule(mut self: Pin<&mut Self>, rule_index: i32) -> bool {
        let mut removed = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let idx = rule_index as usize;
                if rule_index >= 0 && idx < sw.rules.len() {
                    sw.rules.remove(idx);
                    sw.loaded_from_disk = true;
                    let _ = sw.save_to_disk();
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
        let mut updated = false;
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let idx = rule_index as usize;
                if rule_index >= 0 && idx < sw.rules.len() {
                    sw.rules[idx].enabled = enabled;
                    sw.loaded_from_disk = true;
                    let _ = sw.save_to_disk();
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

    pub fn reorder_auto_switch_rule(
        mut self: Pin<&mut Self>,
        from_index: i32,
        to_index: i32,
    ) -> bool {
        if from_index < 0 || to_index < 0 || from_index == to_index {
            return false;
        }
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
                    let _ = sw.save_to_disk();
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

    pub fn apply_autostart_enabled(mut self: Pin<&mut Self>, enabled: bool) -> bool {
        match set_kde_autostart_enabled(enabled) {
            Ok(path) => {
                self.as_mut().set_autostart_enabled(enabled);
                self.as_mut().sync_tray_state();
                let msg = if enabled {
                    format!("Added DuckyDeck Studio to KDE System Autostart ({})", path.display())
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
        let _ = self.as_mut().apply_autostart_enabled(next);
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
                let _ = sw.save_to_disk();
            }
        }
        self.as_mut()
            .set_tray_icon_mode(QString::from(&normalized));
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
                let _ = sw.save_to_disk();
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

    pub fn export_active_profile(mut self: Pin<&mut Self>, destination_path: QString) -> bool {
        let active_num = *self.active_profile();
        self.as_mut().export_profile(active_num, destination_path)
    }

    pub fn export_profile(
        mut self: Pin<&mut Self>,
        profile_number: i32,
        destination_path: QString,
    ) -> bool {
        let raw_str = destination_path.to_string();
        let trimmed = raw_str.trim().trim_start_matches("file://");
        if trimmed.is_empty() {
            return false;
        }
        let dest = PathBuf::from(trimmed);
        let active_num = profile_number.clamp(1, 64) as u8;

        let (maybe_profile, matching_rules) = {
            let rust = self.as_mut().rust_mut().get_mut();
            let prof = rust
                .profiles
                .iter()
                .find(|p| p.number == active_num)
                .or_else(|| rust.profiles.first())
                .cloned();
            let rules = if let Ok(sw) = rust.switcher_state.lock() {
                sw.rules
                    .iter()
                    .filter(|r| r.target_profile == active_num)
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            (prof, rules)
        };

        let Some(profile) = maybe_profile else {
            self.as_mut()
                .set_status_message(QString::from("No active profile available to export"));
            return false;
        };

        let bundle = ProfileExportBundle {
            version: 1,
            profile: profile.clone(),
            auto_switch_rules: matching_rules,
        };

        let Ok(json_str) = serde_json::to_string_pretty(&bundle) else {
            self.as_mut()
                .set_status_message(QString::from("Failed serializing profile bundle"));
            return false;
        };

        if dest.is_dir() {
            let safe_name: String = profile
                .name
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                .take(24)
                .collect();
            let file_stem = if safe_name.is_empty() {
                format!("profile_{active_num}")
            } else {
                safe_name
            };
            let json_path = dest.join(format!("{file_stem}.duckyprofile.json"));
            let mut export_prof = profile.clone();
            export_prof.folder_name_override = None;
            let folder_res = export_prof.save_to_sd_root(&dest);
            let json_res = fs::write(&json_path, &json_str);
            match (folder_res, json_res) {
                (Ok(folder_path), Ok(())) => {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {} and {}",
                        profile.name,
                        folder_path.display(),
                        json_path.display()
                    )));
                    true
                }
                (Ok(folder_path), Err(_)) => {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {}",
                        profile.name,
                        folder_path.display()
                    )));
                    true
                }
                (Err(_), Ok(())) => {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {}",
                        profile.name,
                        json_path.display()
                    )));
                    true
                }
                (Err(e), Err(_)) => {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Failed exporting profile: {e}"
                    )));
                    false
                }
            }
        } else {
            let target_file = if dest
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
            {
                dest
            } else {
                PathBuf::from(format!("{}.duckyprofile.json", dest.display()))
            };
            if let Some(parent) = target_file.parent() {
                let _ = fs::create_dir_all(parent);
            }
            match fs::write(&target_file, &json_str) {
                Ok(()) => {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {}",
                        profile.name,
                        target_file.display()
                    )));
                    true
                }
                Err(e) => {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Failed writing {}: {e}",
                        target_file.display()
                    )));
                    false
                }
            }
        }
    }

    pub fn import_profile_from_path(mut self: Pin<&mut Self>, source_path: QString) -> bool {
        let raw_str = source_path.to_string();
        let trimmed = raw_str.trim().trim_start_matches("file://");
        if trimmed.is_empty() {
            return false;
        }
        let src = PathBuf::from(trimmed);
        if !src.exists() {
            self.as_mut().set_status_message(QString::from(&format!(
                "Import source not found: {}",
                src.display()
            )));
            return false;
        }

        let load_from_json_file = |p: &Path| -> Result<(DuckyProfile, Vec<AutoSwitchRule>), String> {
            let raw = fs::read_to_string(p)
                .map_err(|e| format!("Failed reading {}: {e}", p.display()))?;
            if let Ok(bundle) = serde_json::from_str::<ProfileExportBundle>(&raw) {
                return Ok((bundle.profile, bundle.auto_switch_rules));
            }
            let prof = serde_json::from_str::<DuckyProfile>(&raw)
                .map_err(|e| format!("Invalid profile JSON in {}: {e}", p.display()))?;
            Ok((prof, Vec::new()))
        };

        let load_from_profile_dir =
            |dir: &Path| -> Result<(DuckyProfile, Vec<AutoSwitchRule>), String> {
                if let Ok(prof) = DuckyProfile::load_from_dir(dir) {
                    return Ok((prof, Vec::new()));
                }
                let raw_name = dir
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Imported");
                let clean_name = raw_name
                    .strip_prefix("profile_")
                    .or_else(|| {
                        raw_name
                            .strip_prefix("profile")
                            .and_then(|r| r.split_once('_').map(|(_, n)| n))
                    })
                    .unwrap_or(raw_name)
                    .to_string();
                let prof = DuckyProfile::load_from_dir_with_meta(dir, 1, clean_name)
                    .map_err(|e| format!("Failed loading profile folder {}: {e}", dir.display()))?;
                Ok((prof, Vec::new()))
            };

        let loaded_result = if src.is_dir() {
            if src.join("config.txt").is_file() {
                load_from_profile_dir(&src)
            } else {
                let mut found_json = None;
                if let Ok(entries) = fs::read_dir(&src) {
                    for entry in entries.flatten() {
                        let ep = entry.path();
                        if ep
                            .extension()
                            .and_then(|e| e.to_str())
                            .map(|e| e.eq_ignore_ascii_case("json"))
                            .unwrap_or(false)
                        {
                            found_json = Some(ep);
                            break;
                        }
                    }
                }
                if let Some(jp) = found_json {
                    load_from_json_file(&jp)
                } else {
                    Err(format!(
                        "No config.txt or .duckyprofile.json found in {}",
                        src.display()
                    ))
                }
            }
        } else if src
            .file_name()
            .and_then(|s| s.to_str())
            .map(|n| n.eq_ignore_ascii_case("config.txt"))
            .unwrap_or(false)
        {
            if let Some(parent) = src.parent() {
                load_from_profile_dir(parent)
            } else {
                Err("Invalid config.txt parent folder".to_string())
            }
        } else {
            load_from_json_file(&src)
        };

        let (mut imported_profile, imported_rules) = match loaded_result {
            Ok(pair) => pair,
            Err(err) => {
                self.as_mut().set_status_message(QString::from(&err));
                return false;
            }
        };

        let (new_num, final_name, sd_msg, profiles_json, rules_json) = {
            let rust = self.as_mut().rust_mut().get_mut();
            if rust.profiles.len() >= 64 {
                self.as_mut()
                    .set_status_message(QString::from("Maximum of 64 profiles reached"));
                return false;
            }

            let base_name: String = imported_profile
                .name
                .trim()
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
                .take(16)
                .collect();
            let base_name = if base_name.trim().is_empty() {
                "Imported".to_string()
            } else {
                base_name.trim().to_string()
            };

            let mut candidate_name = base_name.clone();
            let mut suffix = 2u32;
            while rust
                .profiles
                .iter()
                .any(|p| p.name.eq_ignore_ascii_case(&candidate_name))
            {
                let tag = format!("_{suffix}");
                let max_base = 16usize.saturating_sub(tag.len());
                let prefix: String = base_name.chars().take(max_base).collect();
                candidate_name = format!("{prefix}{tag}");
                suffix += 1;
            }

            let next_num = (rust.profiles.len() + 1).clamp(1, 64) as u8;
            imported_profile.number = next_num;
            imported_profile.name = candidate_name.clone();
            imported_profile.folder_name_override = None;

            let mut sd_status = String::new();
            if let Some(ref sd_root) = rust.sd_root_path {
                if let Ok(saved_dir) = imported_profile.save_to_sd_root(sd_root) {
                    if let Some(fname) = saved_dir.file_name().and_then(|s| s.to_str()) {
                        imported_profile.folder_name_override = Some(fname.to_string());
                    }
                    sd_status = format!(" (saved to {})", saved_dir.display());
                }
            }

            rust.profiles.push(imported_profile);
            for (idx, p) in rust.profiles.iter_mut().enumerate() {
                p.number = (idx + 1) as u8;
            }
            if let Some(ref sd_root) = rust.sd_root_path {
                let _ = save_profile_order_to_sd(sd_root, &rust.profiles);
            }
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if !sd_status.is_empty() {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }

            let rules_snapshot = if let Ok(mut sw) = rust.switcher_state.lock() {
                for mut r in imported_rules {
                    r.target_profile = next_num;
                    let already_exists = sw.rules.iter().any(|existing| {
                        existing
                            .resource_class
                            .eq_ignore_ascii_case(&r.resource_class)
                            && existing.target_profile == next_num
                    });
                    if !already_exists {
                        sw.rules.push(r);
                    }
                }
                let _ = sw.save_to_disk();
                sw.rules.clone()
            } else {
                Vec::new()
            };

            let p_json = DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_snapshot);
            let r_json = DeviceControllerRust::serialize_rules(&rules_snapshot, &rust.profiles);
            (next_num, candidate_name, sd_status, p_json, r_json)
        };

        self.as_mut()
            .set_profiles_json(QString::from(&profiles_json));
        self.as_mut()
            .set_auto_switch_rules_json(QString::from(&rules_json));
        let _ = self.as_mut().select_profile(new_num as i32);
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "Imported Profile #{new_num} '{final_name}'{sd_msg}"
        )));
        true
    }

    pub fn save_profile_group(
        mut self: Pin<&mut Self>,
        destination_path: QString,
        group_name: QString,
    ) -> bool {
        let raw_str = destination_path.to_string();
        let trimmed = raw_str.trim().trim_start_matches("file://");
        if trimmed.is_empty() {
            return false;
        }
        let dest = PathBuf::from(trimmed);
        let raw_group = group_name.to_string().trim().to_string();

        let (profiles_clone, rules_clone, active_prof, default_fb) = {
            let rust = self.as_mut().rust_mut().get_mut();
            let rules_and_fb = if let Ok(sw) = rust.switcher_state.lock() {
                (sw.rules.clone(), sw.default_profile)
            } else {
                (Vec::new(), Some(1))
            };
            (
                rust.profiles.clone(),
                rules_and_fb.0,
                (rust.active_profile.clamp(1, 64)) as u8,
                rules_and_fb.1,
            )
        };

        if profiles_clone.is_empty() {
            self.as_mut()
                .set_status_message(QString::from("No profiles available to save as a group"));
            return false;
        }

        let effective_group_name = if raw_group.is_empty() {
            dest.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("DuckyDeck Group")
                .trim_end_matches(".duckygroup")
                .to_string()
        } else {
            raw_group
        };

        let bundle = ProfileGroupBundle {
            version: 1,
            group_name: effective_group_name.clone(),
            active_profile: active_prof,
            default_fallback_profile: default_fb,
            profiles: profiles_clone.clone(),
            auto_switch_rules: rules_clone,
        };

        let Ok(json_str) = serde_json::to_string_pretty(&bundle) else {
            self.as_mut()
                .set_status_message(QString::from("Failed serializing profile group bundle"));
            return false;
        };

        let target_file = if dest.is_dir() {
            let safe_stem: String = effective_group_name
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                .take(32)
                .collect();
            let stem = if safe_stem.is_empty() {
                "profile_group".to_string()
            } else {
                safe_stem
            };
            dest.join(format!("{stem}.duckygroup.json"))
        } else if dest
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            dest
        } else {
            PathBuf::from(format!("{}.duckygroup.json", dest.display()))
        };

        if let Some(parent) = target_file.parent() {
            let _ = fs::create_dir_all(parent);
        }

        match fs::write(&target_file, &json_str) {
            Ok(()) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "✓ Saved Profile Group '{}' ({} profiles) to {}",
                    effective_group_name,
                    profiles_clone.len(),
                    target_file.display()
                )));
                true
            }
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Failed saving profile group to {}: {e}",
                    target_file.display()
                )));
                false
            }
        }
    }

    pub fn load_profile_group(mut self: Pin<&mut Self>, source_path: QString) -> bool {
        let raw_str = source_path.to_string();
        let trimmed = raw_str.trim().trim_start_matches("file://");
        if trimmed.is_empty() {
            return false;
        }
        let src = PathBuf::from(trimmed);
        if !src.exists() {
            self.as_mut().set_status_message(QString::from(&format!(
                "Profile group file not found: {}",
                src.display()
            )));
            return false;
        }

        let loaded_bundle: Result<ProfileGroupBundle, String> = if src.is_dir() {
            let profs = load_all_sd_profiles(&src).unwrap_or_default();
            if profs.is_empty() {
                Err(format!("No profiles found in folder {}", src.display()))
            } else {
                let folder_name = src
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Loaded Folder")
                    .to_string();
                Ok(ProfileGroupBundle {
                    version: 1,
                    group_name: folder_name,
                    active_profile: 1,
                    default_fallback_profile: Some(1),
                    profiles: profs,
                    auto_switch_rules: Vec::new(),
                })
            }
        } else {
            match fs::read_to_string(&src) {
                Ok(raw) => {
                    if let Ok(bundle) = serde_json::from_str::<ProfileGroupBundle>(&raw) {
                        Ok(bundle)
                    } else if let Ok(profs) = serde_json::from_str::<Vec<DuckyProfile>>(&raw) {
                        Ok(ProfileGroupBundle {
                            version: 1,
                            group_name: src
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("Loaded Group")
                                .to_string(),
                            active_profile: 1,
                            default_fallback_profile: Some(1),
                            profiles: profs,
                            auto_switch_rules: Vec::new(),
                        })
                    } else {
                        Err(format!(
                            "Invalid .duckygroup.json format in {}",
                            src.display()
                        ))
                    }
                }
                Err(e) => Err(format!("Failed reading {}: {e}", src.display())),
            }
        };

        let mut bundle = match loaded_bundle {
            Ok(b) if !b.profiles.is_empty() => b,
            Ok(_) => {
                self.as_mut()
                    .set_status_message(QString::from("Profile group contains 0 profiles"));
                return false;
            }
            Err(err) => {
                self.as_mut().set_status_message(QString::from(&err));
                return false;
            }
        };

        bundle.profiles.truncate(64);
        for (idx, p) in bundle.profiles.iter_mut().enumerate() {
            p.number = (idx + 1) as u8;
        }
        let target_active = bundle
            .active_profile
            .clamp(1, bundle.profiles.len() as u8) as i32;
        let prof_count = bundle.profiles.len();
        let group_title = if bundle.group_name.trim().is_empty() {
            "Loaded Group".to_string()
        } else {
            bundle.group_name.trim().to_string()
        };

        let (profiles_json, rules_json) = {
            let rust = self.as_mut().rust_mut().get_mut();
            rust.profiles = bundle.profiles;
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);

            let rules_snapshot = if let Ok(mut sw) = rust.switcher_state.lock() {
                if !bundle.auto_switch_rules.is_empty() {
                    sw.rules = bundle.auto_switch_rules;
                }
                if let Some(fb) = bundle.default_fallback_profile {
                    sw.default_profile = Some(fb.clamp(1, prof_count as u8));
                }
                sw.loaded_from_disk = true;
                let _ = sw.save_to_disk();
                sw.rules.clone()
            } else {
                Vec::new()
            };

            let p_json =
                DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_snapshot);
            let r_json = DeviceControllerRust::serialize_rules(&rules_snapshot, &rust.profiles);
            (p_json, r_json)
        };

        self.as_mut()
            .set_profiles_json(QString::from(&profiles_json));
        self.as_mut()
            .set_auto_switch_rules_json(QString::from(&rules_json));
        let _ = self.as_mut().select_profile(target_active);
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "✓ Loaded Profile Group '{group_title}' ({prof_count} profiles) — Review & Sync to duckyPad when ready"
        )));
        true
    }

    pub fn create_new_profile_group(
        mut self: Pin<&mut Self>,
        group_name: QString,
        initial_profile_name: QString,
    ) -> bool {
        let raw_group = group_name.to_string().trim().to_string();
        let group_label = if raw_group.is_empty() {
            "New Profile Group".to_string()
        } else {
            raw_group
        };

        let raw_prof = initial_profile_name.to_string().trim().to_string();
        let clean_prof_name: String = raw_prof
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .take(16)
            .collect();
        let first_prof_name = if clean_prof_name.trim().is_empty() {
            "Profile 1".to_string()
        } else {
            clean_prof_name.trim().to_string()
        };

        let new_prof = DuckyProfile {
            number: 1,
            name: first_prof_name.clone(),
            folder_name_override: Some(format!("profile_{first_prof_name}")),
            bg_color: [0, 170, 255],
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            ..Default::default()
        };

        let (profiles_json, rules_json) = {
            let rust = self.as_mut().rust_mut().get_mut();
            rust.profiles = vec![new_prof];
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);

            let rules_snapshot = if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.rules.clear();
                sw.default_profile = Some(1);
                sw.loaded_from_disk = true;
                let _ = sw.save_to_disk();
                sw.rules.clone()
            } else {
                Vec::new()
            };

            let p_json =
                DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_snapshot);
            let r_json = DeviceControllerRust::serialize_rules(&rules_snapshot, &rust.profiles);
            (p_json, r_json)
        };

        self.as_mut()
            .set_profiles_json(QString::from(&profiles_json));
        self.as_mut()
            .set_auto_switch_rules_json(QString::from(&rules_json));
        let _ = self.as_mut().select_profile(1);
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "✓ Created new Profile Group '{group_label}' starting with Profile #1 ('{first_prof_name}')"
        )));
        true
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
                self.as_mut().set_status_message(QString::from(&format!(
                    "Failed to reboot duckyPad: {e}"
                )));
                false
            }
        }
    }

    pub fn copy_slot(mut self: Pin<&mut Self>, slot_number: i32) -> bool {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;

        let (copied, summary_label) = {
            let rust = self.as_mut().rust_mut().get_mut();
            let prof_opt = rust
                .profiles
                .iter()
                .find(|p| p.number == active_num)
                .or_else(|| rust.profiles.first());
            let Some(prof) = prof_opt else {
                return false;
            };
            let slot_data = prof
                .slots
                .get(&slot_idx)
                .cloned()
                .unwrap_or_else(|| DuckyKeySlot {
                    slot: slot_idx,
                    ..Default::default()
                });
            let combined_label = format!(
                "{} {}",
                slot_data.label_line1.trim(),
                slot_data.label_line2.trim()
            )
            .trim()
            .to_string();
            let summary = if combined_label.is_empty() {
                format!("Key #{slot_idx}")
            } else {
                format!("Key #{slot_idx} ('{combined_label}')")
            };
            rust.copied_key_slot = Some(slot_data.clone());
            (slot_data, summary)
        };

        let _ = copied;
        self.as_mut().set_has_copied_slot(true);
        self.as_mut()
            .set_copied_slot_summary(QString::from(&summary_label));
        self.as_mut().set_status_message(QString::from(&format!(
            "📋 Copied {summary_label} to clipboard"
        )));
        true
    }

    pub fn paste_slot(mut self: Pin<&mut Self>, target_slot_number: i32) -> bool {
        let target_idx = target_slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;

        let paste_res: Result<(String, bool), String> = {
            let rust = self.as_mut().rust_mut().get_mut();
            let Some(mut src_slot) = rust.copied_key_slot.clone() else {
                return false;
            };
            src_slot.slot = target_idx;
            if target_idx > crate::hardware::storage::ONBOARD_KEY_COUNT {
                src_slot.custom_rgb = None;
            }
            let combined_label = format!(
                "{} {}",
                src_slot.label_line1.trim(),
                src_slot.label_line2.trim()
            )
            .trim()
            .to_string();
            let label_desc = if combined_label.is_empty() {
                format!("Key #{target_idx}")
            } else {
                format!("'{combined_label}' into Key #{target_idx}")
            };

            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            let Some(prof) = rust.profiles.get_mut(prof_idx) else {
                return false;
            };
            prof.slots.insert(target_idx, src_slot);

            let saved_sd = if let Some(root) = sd_root {
                prof.save_to_sd_root(&root).is_ok()
            } else {
                false
            };
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if saved_sd {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            Ok((label_desc, saved_sd))
        };

        self.as_mut().sync_active_profile_state();

        match paste_res {
            Ok((desc, saved_sd)) => {
                let suffix = if saved_sd { " (saved to SD)" } else { "" };
                self.as_mut().set_status_message(QString::from(&format!(
                    "📥 Pasted {desc}{suffix}"
                )));
                true
            }
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&e));
                false
            }
        }
    }

    pub fn clear_slot(mut self: Pin<&mut Self>, slot_number: i32) -> bool {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;

        let saved_sd = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            let Some(prof) = rust.profiles.get_mut(prof_idx) else {
                return false;
            };
            prof.slots.remove(&slot_idx);
            let sd_ok = if let Some(root) = sd_root {
                prof.save_to_sd_root(&root).is_ok()
            } else {
                false
            };
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if sd_ok {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            sd_ok
        };

        self.as_mut().sync_active_profile_state();
        let suffix = if saved_sd { " (saved to SD)" } else { "" };
        self.as_mut().set_status_message(QString::from(&format!(
            "Cleared Key #{slot_idx}{suffix}"
        )));
        true
    }

    pub fn move_or_swap_slot(
        mut self: Pin<&mut Self>,
        from_slot: i32,
        to_slot: i32,
        shift_between: bool,
    ) -> bool {
        let from_idx = from_slot.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let to_idx = to_slot.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        if from_idx == to_idx {
            return false;
        }
        let active_num = *self.active_profile() as u8;

        let saved_sd = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            let Some(prof) = rust.profiles.get_mut(prof_idx) else {
                return false;
            };

            if shift_between {
                let moved = prof.slots.remove(&from_idx);
                if from_idx < to_idx {
                    for k in (from_idx + 1)..=to_idx {
                        if let Some(mut s) = prof.slots.remove(&k) {
                            s.slot = k - 1;
                            prof.slots.insert(k - 1, s);
                        }
                    }
                } else {
                    for k in (to_idx..from_idx).rev() {
                        if let Some(mut s) = prof.slots.remove(&k) {
                            s.slot = k + 1;
                            prof.slots.insert(k + 1, s);
                        }
                    }
                }
                if let Some(mut m) = moved {
                    m.slot = to_idx;
                    if to_idx > crate::hardware::storage::ONBOARD_KEY_COUNT {
                        m.custom_rgb = None;
                    }
                    prof.slots.insert(to_idx, m);
                }
            } else {
                let a = prof.slots.remove(&from_idx);
                let b = prof.slots.remove(&to_idx);
                if let Some(mut slot_a) = a {
                    slot_a.slot = to_idx;
                    if to_idx > crate::hardware::storage::ONBOARD_KEY_COUNT {
                        slot_a.custom_rgb = None;
                    }
                    prof.slots.insert(to_idx, slot_a);
                }
                if let Some(mut slot_b) = b {
                    slot_b.slot = from_idx;
                    if from_idx > crate::hardware::storage::ONBOARD_KEY_COUNT {
                        slot_b.custom_rgb = None;
                    }
                    prof.slots.insert(from_idx, slot_b);
                }
            }

            let sd_ok = if let Some(root) = sd_root {
                prof.save_to_sd_root(&root).is_ok()
            } else {
                false
            };
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if sd_ok {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            sd_ok
        };

        self.as_mut().sync_active_profile_state();
        let suffix = if saved_sd { " (saved to SD)" } else { "" };
        let action_word = if shift_between {
            format!("Moved Key #{from_idx} -> Key #{to_idx} (shifted intermediate buttons)")
        } else {
            format!("Swapped Key #{from_idx} ⇄ Key #{to_idx}")
        };
        self.as_mut().set_status_message(QString::from(&format!(
            "✓ {action_word}{suffix}"
        )));
        true
    }

    pub fn set_close_to_tray_enabled(mut self: Pin<&mut Self>, enabled: bool) {
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.close_to_tray = enabled;
                let _ = sw.save_to_disk();
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

    pub fn generate_ai_macro(mut self: Pin<&mut Self>, app_name: QString, prompt: QString) {
        self.as_mut().set_ai_busy(true);
        let app = app_name.to_string();
        let user_goal = prompt.to_string();

        let local_shortcuts = crate::ai::shortcut_scraper::scrape_kde_shortcuts(&app);
        let shortcut_comment = if let Some((action, key)) = local_shortcuts.first() {
            format!("// Grounded from ~/.config/kglobalshortcutsrc: {action} = {key}\n")
        } else {
            String::new()
        };

        let preview = format!(
            "// AI Generated Macro for: {app}\n// Goal: {user_goal}\n{shortcut_comment}DEFAULTDELAY 20\nSWC_SET 0 0 255 128\nOLED_CLEAR\nOLED_CURSOR 8 40\nOLED_PRINT {app}\nOLED_CURSOR 8 56\nOLED_PRINT Running Macro...\nOLED_UPDATE\nCONTROL SHIFT m\nDELAY 150\nSWC_RESET 0\nOLED_RESTORE\n"
        );

        self.as_mut()
            .set_generated_script_preview(QString::from(&preview));
        self.as_mut().set_ai_busy(false);
        self.as_mut().set_status_message(QString::from(&format!(
            "AI generated duckyScript 3 for {app}"
        )));
    }

    pub fn check_firmware_update(mut self: Pin<&mut Self>, manual: bool) {
        if *self.firmware_check_busy() {
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
        std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Runtime::new() else {
                return;
            };
            let result = rt.block_on(check_latest_duckypad_firmware(is_pro, &curr_fw));
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
            let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
        }
    }

    pub fn download_firmware_binary(self: Pin<&mut Self>) {
        let dl = self.rust().firmware_download_url.clone();
        let fallback = self.firmware_update_url().to_string();
        let target = if !dl.is_empty() { dl } else { fallback };
        if !target.is_empty() {
            let _ = std::process::Command::new("xdg-open").arg(&target).spawn();
        }
    }

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
        let cfg: WebApiConfig =
            serde_json::from_str(&config_json.to_string()).unwrap_or_default();
        let updated = apply_web_api_config_to_script(
            &existing_script.to_string(),
            &cfg,
            active_prof,
            slot_idx,
            is_release,
        );
        QString::from(&updated)
    }

    pub fn test_web_api_call(
        mut self: Pin<&mut Self>,
        config_json: QString,
        slot_number: i32,
    ) {
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
        self.as_mut().dispatch_web_api_execution(
            cfg,
            slot_idx,
            restore_rgb,
            &source_label,
        );
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

    pub fn delete_web_api_destination(
        mut self: Pin<&mut Self>,
        name: QString,
    ) -> QString {
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
        let updated = apply_app_launch_config_to_script(
            &existing_script.to_string(),
            &cfg,
            active_prof,
            slot_idx,
            is_release,
        );
        QString::from(&updated)
    }

    pub fn test_app_launch(
        mut self: Pin<&mut Self>,
        config_json: QString,
        slot_number: i32,
    ) {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let cfg: AppLaunchConfig = match serde_json::from_str(&config_json.to_string()) {
            Ok(c) => c,
            Err(e) => {
                let err = format!("Invalid App Launch configuration on Key {slot_idx}: {e}");
                record_system_event("ERROR", "APP_LAUNCH", &err);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_app_launch_last_ok(false);
                self.as_mut().set_app_launch_last_result(QString::from(&err));
                self.as_mut().set_status_message(QString::from(&err));
                return;
            }
        };

        let active_prof = (*self.active_profile()).clamp(1, 64) as u8;
        let restore_rgb = self.lookup_slot_rgb(active_prof, slot_idx);
        let source_label = format!("Test Launch (Profile #{active_prof}, Key {slot_idx})");
        self.as_mut().dispatch_app_launch_execution(
            cfg,
            slot_idx,
            restore_rgb,
            &source_label,
        );
    }

    fn lookup_slot_rgb(&self, profile_num: u8, slot_idx: usize) -> [u8; 3] {
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

    fn handle_hardware_pgv_web_api_trigger(
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
                        if found_web.is_none() && web_cfg.enabled && !web_cfg.url.trim().is_empty() {
                            found_web = Some((
                                prof.number,
                                prof.name.clone(),
                                label.clone(),
                                web_cfg,
                                rgb,
                            ));
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

    fn handle_hardware_fkey_web_api_trigger(mut self: Pin<&mut Self>, fkey_name: &str) {
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

    fn dispatch_app_launch_execution(
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

        if cfg.led_feedback && (1..=20).contains(&slot_num) {
            std::thread::spawn(move || {
                let Ok(rt) = tokio::runtime::Runtime::new() else {
                    return;
                };
                rt.block_on(async move {
                    flash_slot_led_feedback(slot_num, ok, restore_rgb).await;
                });
            });
        }
    }

    fn dispatch_web_api_execution(
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
            format!(
                "Dispatching {method} {url} [source: {source_label}, body: {body_len} B]"
            ),
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

        std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Runtime::new() else {
                return;
            };
            rt.block_on(async move {
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
                });

                if led_feedback && (1..=20).contains(&slot_num) {
                    flash_slot_led_feedback(slot_num, ok, restore_rgb).await;
                }
            });
        });
    }

    fn open_duckypad() -> Result<(HidDevice, u16), String> {
        let api = HidApi::new().map_err(|e| format!("HID init error: {e}"))?;
        for pid in [PID_DUCKYPAD_PRO, PID_DUCKYPAD_2020] {
            if let Ok(dev) = api.open(DUCKYPAD_VID, pid) {
                return Ok((dev, pid));
            }
        }
        Err("No duckyPad Pro (0xd11d) or duckyPad 2020 (0xd11c) found".to_string())
    }

    fn has_permission_denied_hidraw() -> bool {
        let Ok(entries) = fs::read_dir("/sys/class/hidraw") else {
            return false;
        };
        for entry in entries.flatten() {
            if let Ok(uevent) = fs::read_to_string(entry.path().join("device/uevent")) {
                let is_duckypad = uevent.contains("00000483:0000D11D")
                    || uevent.contains("00000483:0000D11C");
                if is_duckypad {
                    let dev_node = Path::new("/dev").join(entry.file_name());
                    if let Err(e) = OpenOptions::new().read(true).write(true).open(&dev_node) {
                        if e.kind() == std::io::ErrorKind::PermissionDenied {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn sync_event_log_properties(mut self: Pin<&mut Self>) {
        let (json_str, count) = serialize_system_event_log_json();
        self.as_mut().set_event_log_count(count);
        self.as_mut().set_event_log_json(QString::from(&json_str));
    }

    pub fn log_event(
        mut self: Pin<&mut Self>,
        level: QString,
        category: QString,
        message: QString,
    ) {
        let lvl = level.to_string();
        let cat = category.to_string();
        let msg = message.to_string();
        record_system_event(&lvl, &cat, &msg);
        self.as_mut().sync_event_log_properties();
    }

    pub fn record_status_log(mut self: Pin<&mut Self>, message: QString) {
        let msg = message.to_string();
        let trimmed = msg.trim();
        if trimmed.is_empty() {
            return;
        }
        // Web API and App Launch log structured entries directly in dispatch_*_execution
        if trimmed.starts_with("🌐 ")
            || trimmed.starts_with("🚀 ")
            || ((trimmed.starts_with("✓ ") || trimmed.starts_with("⚠ "))
                && (trimmed.contains("Test Web API")
                    || trimmed.contains("Test Launch")
                    || trimmed.contains(": HTTP ")
                    || trimmed.contains(": Launched ")
                    || trimmed.contains(": Failed to start ")))
        {
            self.as_mut().sync_event_log_properties();
            return;
        }
        let (lvl, cat) = classify_status_message(trimmed);
        record_system_event(lvl, cat, trimmed);
        self.as_mut().sync_event_log_properties();
    }

    pub fn refresh_event_log(mut self: Pin<&mut Self>) -> QString {
        let (json_str, count) = serialize_system_event_log_json();
        self.as_mut().set_event_log_count(count);
        self.as_mut().set_event_log_json(QString::from(&json_str));
        QString::from(&json_str)
    }

    pub fn clear_event_log(mut self: Pin<&mut Self>) {
        if let Ok(mut guard) = system_event_log_store().lock() {
            guard.clear();
        }
        let log_path = get_system_log_file_path();
        let _ = fs::write(&log_path, "");
        record_system_event("INFO", "SYSTEM", "Event log cleared by user");
        self.as_mut().sync_event_log_properties();
    }

    pub fn export_event_log(mut self: Pin<&mut Self>, destination_path: QString) -> bool {
        let raw_str = destination_path.to_string();
        let trimmed = raw_str.trim().trim_start_matches("file://");
        if trimmed.is_empty() {
            return false;
        }
        let mut dest = PathBuf::from(trimmed);
        if dest.extension().is_none() {
            dest.set_extension("log");
        }
        let text = self
            .format_event_log_text(
                QString::from("ALL"),
                QString::from("ALL"),
                QString::from(""),
            )
            .to_string();
        match fs::write(&dest, text) {
            Ok(_) => {
                let msg = format!("Exported system event log to {}", dest.display());
                record_system_event("INFO", "SYSTEM", &msg);
                self.as_mut().set_status_message(QString::from(&msg));
                self.as_mut().sync_event_log_properties();
                true
            }
            Err(e) => {
                let err_msg = format!("Failed to export event log: {e}");
                record_system_event("ERROR", "SYSTEM", &err_msg);
                self.as_mut().set_status_message(QString::from(&err_msg));
                self.as_mut().sync_event_log_properties();
                false
            }
        }
    }

    pub fn format_event_log_text(
        &self,
        category_filter: QString,
        level_filter: QString,
        search_query: QString,
    ) -> QString {
        let cat_f = category_filter.to_string().trim().to_ascii_uppercase();
        let lvl_f = level_filter.to_string().trim().to_ascii_uppercase();
        let q = search_query.to_string().trim().to_ascii_lowercase();

        let mut lines = Vec::new();
        lines.push(format!(
            "# DuckyDeck Studio v{} — System Event Log ({})",
            APP_VERSION,
            get_system_log_file_path().display()
        ));
        if let Ok(guard) = system_event_log_store().lock() {
            for entry in guard.iter() {
                if !cat_f.is_empty() && cat_f != "ALL" && entry.category != cat_f {
                    continue;
                }
                if !lvl_f.is_empty() && lvl_f != "ALL" && entry.level != lvl_f {
                    continue;
                }
                if !q.is_empty() {
                    let hay = format!(
                        "{} {} {} {}",
                        entry.timestamp, entry.level, entry.category, entry.message
                    )
                    .to_ascii_lowercase();
                    if !hay.contains(&q) {
                        continue;
                    }
                }
                lines.push(format!(
                    "[{}] [{:<5}] [{:<10}] {}",
                    entry.timestamp, entry.level, entry.category, entry.message
                ));
            }
        }
        QString::from(&lines.join("\n"))
    }

    pub fn refresh_color_palettes(mut self: Pin<&mut Self>) {
        let (saved_json, used_json) = {
            let rust = self.rust();
            let saved = rust
                .switcher_state
                .lock()
                .map(|sw| sw.saved_colors.clone())
                .unwrap_or_default();
            let s_json = serde_json::to_string(&saved).unwrap_or_else(|_| "[]".to_string());
            let u_json = DeviceControllerRust::collect_used_colors_json(&rust.profiles);
            (s_json, u_json)
        };
        if self.saved_colors_json().to_string() != saved_json {
            self.as_mut()
                .set_saved_colors_json(QString::from(&saved_json));
        }
        if self.used_colors_json().to_string() != used_json {
            self.as_mut()
                .set_used_colors_json(QString::from(&used_json));
        }
    }

    pub fn save_palette_color(mut self: Pin<&mut Self>, hex_color: QString) {
        let raw = hex_color.to_string();
        let trimmed = raw.trim().to_ascii_lowercase();
        let normalized = if trimmed.starts_with('#') {
            trimmed
        } else {
            format!("#{trimmed}")
        };
        if normalized.len() != 7
            || !normalized[1..].chars().all(|c| c.is_ascii_hexdigit())
        {
            return;
        }
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if !sw
                    .saved_colors
                    .iter()
                    .any(|c| c.eq_ignore_ascii_case(&normalized))
                {
                    sw.saved_colors.push(normalized);
                    let _ = sw.save_to_disk();
                }
            }
        }
        self.as_mut().refresh_color_palettes();
    }

    pub fn remove_palette_color(mut self: Pin<&mut Self>, hex_color: QString) {
        let raw = hex_color.to_string();
        let target = raw.trim().to_ascii_lowercase();
        if target.is_empty() {
            return;
        }
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let before = sw.saved_colors.len();
                sw.saved_colors
                    .retain(|c| !c.eq_ignore_ascii_case(&target));
                if sw.saved_colors.len() != before {
                    let _ = sw.save_to_disk();
                }
            }
        }
        self.as_mut().refresh_color_palettes();
    }

    pub fn is_demo_showcase_mode(&self) -> bool {
        is_demo_showcase_env()
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
            }
            "auto-switch:off" => {
                let fb = *self.default_fallback_profile();
                self.as_mut().set_auto_switch_config(false, fb);
                self.as_mut()
                    .set_status_message(QString::from("Auto-Switcher disabled via CLI"));
            }
            "auto-switch:toggle" => {
                self.as_mut().toggle_auto_switch_from_tray();
            }
            "reload" => {
                self.as_mut().refresh_device();
            }
            _ => {
                if let Some(path) = cmd.strip_prefix("import-group:") {
                    let _ = self.as_mut().load_profile_group(QString::from(path));
                } else if let Some(path) = cmd.strip_prefix("import-profile:") {
                    let _ = self.as_mut().import_profile_from_path(QString::from(path));
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
        qobject::duckydeck_capture_composite_frame(&output_path, &banner_title, &banner_subtitle)
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

        let res = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sn = rust.serial_number.to_string();
            let sd_root = rust.sd_root_path.clone();
            let out = discard_unsynced_change_in_profiles(
                &sn,
                &mut rust.profiles,
                sd_root.as_deref(),
                p_num,
                &p_name,
                &key,
            );
            if out.is_ok() {
                if let Ok(mut sw) = rust.switcher_state.lock() {
                    sw.align_rules_with_profiles(&rust.profiles);
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

    pub fn push_hardware_led_frame_json(&self, frame_json: QString) -> bool {
        if !*self.connected() {
            return false;
        }
        let raw = frame_json.to_string();
        let Ok(arr) = serde_json::from_str::<Vec<serde_json::Value>>(&raw) else {
            return false;
        };
        let mut updates: Vec<(usize, [u8; 3])> = Vec::with_capacity(arr.len());
        for item in arr {
            let slot = item.get("slot").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
            if !(1..=20).contains(&slot) {
                continue;
            }
            let r = item.get("r").and_then(|v| v.as_i64()).unwrap_or(0).clamp(0, 255) as u8;
            let g = item.get("g").and_then(|v| v.as_i64()).unwrap_or(0).clamp(0, 255) as u8;
            let b = item.get("b").and_then(|v| v.as_i64()).unwrap_or(0).clamp(0, 255) as u8;
            updates.push((slot, [r, g, b]));
        }
        if updates.is_empty() {
            return false;
        }
        std::thread::spawn(move || {
            let _ = set_batch_key_leds_hid(&updates);
        });
        true
    }

    pub fn get_profile_properties_json(&self, profile_number: i32) -> QString {
        let target_num = if profile_number > 0 {
            profile_number as u8
        } else {
            (*self.active_profile()).max(1) as u8
        };
        let rust = self.rust();
        let Some(prof) = rust.profiles.iter().find(|p| p.number == target_num) else {
            return QString::from("{}");
        };
        let bg_hex = format!(
            "#{:02x}{:02x}{:02x}",
            prof.bg_color[0], prof.bg_color[1], prof.bg_color[2]
        );
        let (has_kd, kd_hex) = match prof.keydown_color {
            Some(kd) => (true, format!("#{:02x}{:02x}{:02x}", kd[0], kd[1], kd[2])),
            None => (false, "#ffffff".to_string()),
        };
        let oled_rotation = prof.oled_rotation_degrees();
        let widget_rotation = prof.widget_rotation_degrees();
        let effective_widget_rotation = prof.effective_widget_rotation_degrees();
        let is_landscape = oled_rotation == 90 || oled_rotation == 270;
        let is_upper_hs = prof.is_upper_hs();
        let extra_lines = prof.canonical_extra_config_lines();
        let configured_keys = prof
            .slots
            .values()
            .filter(|s| !s.is_effectively_empty())
            .count();
        let obj = serde_json::json!({
            "number": prof.number,
            "name": prof.name,
            "folderName": prof.folder_name(),
            "bgColor": bg_hex,
            "hasKeydownColor": has_kd,
            "keydownColor": kd_hex,
            "dimUnusedKeys": prof.dim_unused_keys,
            "isLandscape": is_landscape,
            "oledRotation": oled_rotation,
            "widgetRotation": widget_rotation,
            "effectiveWidgetRotation": effective_widget_rotation,
            "isUpperHs": is_upper_hs,
            "extraConfigLines": extra_lines.join("\n"),
            "configuredSlotCount": configured_keys,
        });
        QString::from(&obj.to_string())
    }

    pub fn save_profile_properties_json(
        mut self: Pin<&mut Self>,
        profile_number: i32,
        props_json: QString,
    ) -> bool {
        let target_num = if profile_number > 0 {
            profile_number as u8
        } else {
            (*self.active_profile()).max(1) as u8
        };
        let raw = props_json.to_string();
        let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) else {
            return false;
        };

        let parse_hex = |hex_str: &str, default_rgb: [u8; 3]| -> [u8; 3] {
            let clean = hex_str.trim().trim_start_matches('#');
            if clean.len() == 6 {
                if let (Ok(r), Ok(g), Ok(b)) = (
                    u8::from_str_radix(&clean[0..2], 16),
                    u8::from_str_radix(&clean[2..4], 16),
                    u8::from_str_radix(&clean[4..6], 16),
                ) {
                    return [r, g, b];
                }
            }
            default_rgb
        };

        let new_name_raw = val
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let sanitized_name: String = new_name_raw
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == ' ')
            .take(16)
            .collect::<String>()
            .trim()
            .to_string();

        let bg_hex = val.get("bgColor").and_then(|v| v.as_str()).unwrap_or("#103060");
        let has_kd = val
            .get("hasKeydownColor")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let kd_hex = val
            .get("keydownColor")
            .and_then(|v| v.as_str())
            .unwrap_or("#ffffff");
        let dim_unused = val
            .get("dimUnusedKeys")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let is_landscape = val
            .get("isLandscape")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let oled_rotation = val
            .get("oledRotation")
            .and_then(|v| v.as_i64())
            .map(|d| d as i32)
            .unwrap_or(if is_landscape { 90 } else { 0 });
        let widget_rotation_opt = val
            .get("widgetRotation")
            .and_then(|v| v.as_i64())
            .map(|d| d as i32);
        let is_upper_hs = val
            .get("isUpperHs")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let extra_lines_vec: Vec<String> = if let Some(arr) = val
            .get("extraConfigLines")
            .and_then(|v| v.as_array())
        {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else if let Some(raw_str) = val.get("extraConfigLines").and_then(|v| v.as_str()) {
            raw_str
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            Vec::new()
        };

        let updated = {
            let rust = self.as_mut().rust_mut().get_mut();
            let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == target_num) else {
                return false;
            };
            let old_name = prof.name.clone();
            if !sanitized_name.is_empty() && sanitized_name != old_name {
                prof.name = sanitized_name.clone();
                prof.folder_name_override = Some(format!(
                    "profile{}_{}",
                    prof.number, sanitized_name
                ));
            }
            let new_bg = parse_hex(bg_hex, prof.bg_color);
            let old_bg = prof.bg_color;
            prof.bg_color = new_bg;
            for slot in prof.slots.values_mut() {
                if slot.custom_rgb == Some(old_bg) {
                    slot.custom_rgb = None;
                }
            }
            prof.keydown_color = if has_kd {
                Some(parse_hex(kd_hex, [255, 255, 255]))
            } else {
                None
            };
            prof.dim_unused_keys = dim_unused;

            let existing_widget_rot = prof.widget_rotation_degrees();
            let final_widget_rot = widget_rotation_opt.unwrap_or(existing_widget_rot);

            let mut preserved: Vec<String> = Vec::new();
            match oled_rotation {
                90 => {
                    preserved.push("IS_LANDSCAPE 1".to_string());
                }
                180 => {
                    preserved.push("IS_LANDSCAPE 2".to_string());
                    preserved.push("OLED_ROTATION 180".to_string());
                }
                270 => {
                    preserved.push("IS_LANDSCAPE 3".to_string());
                    preserved.push("OLED_ROTATION 270".to_string());
                }
                _ => {}
            }
            match final_widget_rot {
                0 | 90 | 180 | 270 => {
                    preserved.push(format!("DW_ROTATION {final_widget_rot}"));
                }
                _ => {}
            }
            if is_upper_hs {
                preserved.push("IS_UPPER_HS 1".to_string());
            }
            for line in &extra_lines_vec {
                let t = line.trim();
                if !t.is_empty()
                    && !t.starts_with("IS_LANDSCAPE ")
                    && !t.starts_with("OLED_ROTATION ")
                    && !t.starts_with("IS_ROTATED_180 ")
                    && !t.starts_with("DW_ROTATION ")
                    && !t.starts_with("IS_UPPER_HS ")
                    && !t.starts_with("BG_COLOR ")
                    && !t.starts_with("KEYDOWN_COLOR ")
                    && !t.starts_with("DIM_UNUSED_KEYS ")
                {
                    preserved.push(t.to_string());
                }
            }
            prof.preserved_config_lines = preserved;
            let final_name = prof.name.clone();

            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.align_rules_with_profiles(&rust.profiles);
                let _ = sw.save_to_disk();
            }
            let sn = rust.serial_number.to_string();
            if sn != "-" && !sn.is_empty() {
                sync_profiles_to_device_cache(&sn, &rust.profiles);
            } else {
                sync_profiles_to_cache(&rust.profiles);
            }
            Some(final_name)
        };

        if let Some(final_name) = updated {
            self.as_mut().sync_active_profile_state();
            self.as_mut().refresh_color_palettes();
            let msg = format!(
                "✓ Updated Profile #{} ({}) properties",
                target_num, final_name
            );
            record_system_event("INFO", "PROFILE", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            true
        } else {
            false
        }
    }

    pub fn oled_label_to_hardware_ascii_qstr(&self, label: QString) -> QString {
        QString::from(&oled_label_to_hardware_ascii(&label.to_string()))
    }

    pub fn list_available_firmwares_json(&self) -> QString {
        let curr = self.firmware_version().to_string();
        let list = list_available_duckypad_pro_firmwares(&curr);
        let json_str = serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string());
        QString::from(&json_str)
    }

    pub fn install_firmware_to_device(
        mut self: Pin<&mut Self>,
        firmware_path: QString,
    ) -> bool {
        if *self.sd_sync_busy() {
            return false;
        }
        let raw_path = firmware_path.to_string();
        let clean_path = raw_path.trim().trim_start_matches("file://").to_string();
        if clean_path.is_empty() {
            let msg = "No firmware binary selected";
            self.as_mut().set_status_message(QString::from(msg));
            return false;
        }

        let dev_serial = self.serial_number().to_string();
        let disk_label = if dev_serial.len() >= 4 && dev_serial != "-" {
            format!("DP24_{}", &dev_serial[dev_serial.len() - 4..])
        } else {
            String::new()
        };
        let label_display = if disk_label.is_empty() {
            "DP24".to_string()
        } else {
            disk_label.clone()
        };

        self.as_mut().set_sd_sync_busy(true);
        self.as_mut().set_sd_sync_progress(5);
        self.as_mut()
            .set_sd_sync_result_state(QString::from("running"));
        self.as_mut().set_sd_sync_result_summary(QString::from(""));
        self.as_mut().set_sd_sync_step(QString::from(
            "Preparing duckyPad Pro firmware installation...",
        ));
        self.as_mut().set_sd_sync_log(QString::from(&format!(
            "• Target device: duckyPad Pro ({label_display})\n• Firmware image: {clean_path}"
        )));
        self.as_mut().set_status_message(QString::from(
            "Staging firmware update to duckyPad Pro microSD...",
        ));

        let qt_thread = self.qt_thread();
        let qt_thread_done = self.qt_thread();
        let fw_path_buf = PathBuf::from(&clean_path);

        std::thread::spawn(move || {
            let res = stage_duckypad_pro_firmware_via_msc_with_progress(
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
            );

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
                        qobject
                            .as_mut()
                            .set_status_message(QString::from(&err_msg));
                    }
                }
            });
        });

        true
    }

    pub fn refresh_sd_mount_status(mut self: Pin<&mut Self>) -> bool {
        if let Some(ref sd_root) = self.rust().sd_root_path.clone() {
            if !crate::hardware::storage::is_duckypad_sd_root(sd_root) {
                self.as_mut().rust_mut().get_mut().sd_root_path = None;
                self.as_mut().set_sd_mount_path(QString::from(""));
            }
        }
        if self.rust().sd_root_path.is_none() {
            if let Some(mounted) = find_mounted_duckypad_sd() {
                let mounted_str = mounted.to_string_lossy().to_string();
                self.as_mut().rust_mut().get_mut().sd_root_path = Some(mounted);
                self.as_mut().set_sd_mount_path(QString::from(&mounted_str));
            }
        }
        let is_mounted = !self.sd_mount_path().is_empty();
        let dev_serial = self.serial_number().to_string();
        let disk_label = if dev_serial.len() >= 4 && dev_serial != "-" {
            format!("DP24_{}", &dev_serial[dev_serial.len() - 4..])
        } else {
            String::new()
        };
        let detected = is_mounted || detect_unmounted_duckypad_sd(&disk_label).is_some();
        if *self.sd_card_detected() != detected {
            self.as_mut().set_sd_card_detected(detected);
        }
        is_mounted
    }

    pub fn mount_sd_card(mut self: Pin<&mut Self>) -> bool {
        if self.as_mut().refresh_sd_mount_status() {
            let mounted_path = self.sd_mount_path().to_string();
            let _ = self.as_mut().load_sd_directory(QString::from(&mounted_path));
            return true;
        }
        let dev_serial = self.serial_number().to_string();
        let disk_label = if dev_serial.len() >= 4 && dev_serial != "-" {
            format!("DP24_{}", &dev_serial[dev_serial.len() - 4..])
        } else {
            String::new()
        };
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
                let _ = self.as_mut().refresh_sd_mount_status();
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

    pub fn save_slot_widget_labels(
        mut self: Pin<&mut Self>,
        slot_number: i32,
        widget_label1: QString,
        widget_label2: QString,
    ) -> bool {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;
        let wl1: String = widget_label1
            .to_string()
            .trim()
            .chars()
            .take(24)
            .collect();
        let wl2: String = widget_label2
            .to_string()
            .trim()
            .chars()
            .take(24)
            .collect();

        let updated = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            if let Some(prof) = rust.profiles.get_mut(prof_idx) {
                let entry = prof.ensure_slot(slot_idx);
                entry.widget_label_line1 = wl1;
                entry.widget_label_line2 = wl2;
                if entry.is_effectively_empty() {
                    prof.slots.remove(&slot_idx);
                }
                if let Some(root) = sd_root {
                    let _ = prof.save_to_sd_root(&root);
                }
                let sn = rust.serial_number.to_string();
                if sn != "-" && !sn.is_empty() {
                    sync_profiles_to_device_cache(&sn, &rust.profiles);
                } else {
                    sync_profiles_to_cache(&rust.profiles);
                }
                true
            } else {
                false
            }
        };

        if updated {
            self.as_mut().sync_active_profile_state();
            true
        } else {
            false
        }
    }

    pub fn set_active_profile_oled_rotation(
        mut self: Pin<&mut Self>,
        degrees: i32,
    ) -> bool {
        let active_num = (*self.active_profile()).max(1) as u8;
        let normalized = match degrees {
            90 => 90,
            180 => 180,
            270 => 270,
            _ => 0,
        };
        let updated_name = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            if let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == active_num) {
                prof.set_oled_rotation_degrees(normalized);
                if let Some(root) = sd_root {
                    let _ = prof.save_to_sd_root(&root);
                }
                let name = prof.name.clone();
                let sn = rust.serial_number.to_string();
                if sn != "-" && !sn.is_empty() {
                    sync_profiles_to_device_cache(&sn, &rust.profiles);
                } else {
                    sync_profiles_to_cache(&rust.profiles);
                }
                Some(name)
            } else {
                None
            }
        };

        if let Some(name) = updated_name {
            self.as_mut().sync_active_profile_state();
            let msg = format!(
                "✓ Set OLED Orientation to {}° for Profile #{} ({})",
                normalized, active_num, name
            );
            record_system_event("INFO", "PROFILE", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            true
        } else {
            false
        }
    }

    pub fn set_active_profile_widget_rotation(
        mut self: Pin<&mut Self>,
        degrees: i32,
    ) -> bool {
        let active_num = (*self.active_profile()).max(1) as u8;
        let normalized = match degrees {
            0 => 0,
            90 => 90,
            180 => 180,
            270 => 270,
            _ => -1,
        };
        let updated_name = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            if let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == active_num) {
                prof.set_widget_rotation_degrees(normalized);
                if let Some(root) = sd_root {
                    let _ = prof.save_to_sd_root(&root);
                }
                let name = prof.name.clone();
                let sn = rust.serial_number.to_string();
                if sn != "-" && !sn.is_empty() {
                    sync_profiles_to_device_cache(&sn, &rust.profiles);
                } else {
                    sync_profiles_to_cache(&rust.profiles);
                }
                Some(name)
            } else {
                None
            }
        };

        if let Some(name) = updated_name {
            self.as_mut().sync_active_profile_state();
            let label = if normalized < 0 {
                "Follow Hardware OLED".to_string()
            } else {
                format!("{normalized}°")
            };
            let msg = format!(
                "✓ Set Desktop Widget Orientation to {} for Profile #{} ({})",
                label, active_num, name
            );
            record_system_event("INFO", "PROFILE", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            true
        } else {
            false
        }
    }

    pub fn list_profile_templates_json(&self) -> QString {
        let list = list_all_profile_templates();
        let items: Vec<_> = list
            .into_iter()
            .map(|t| {
                let slot_count = t
                    .profile
                    .slots
                    .values()
                    .filter(|s| !s.is_effectively_empty())
                    .count();
                let bg_hex = format!(
                    "#{:02x}{:02x}{:02x}",
                    t.profile.bg_color[0], t.profile.bg_color[1], t.profile.bg_color[2]
                );
                serde_json::json!({
                    "id": t.id,
                    "name": t.name,
                    "description": t.description,
                    "category": t.category,
                    "builtin": t.is_builtin,
                    "slotCount": slot_count,
                    "bgColor": bg_hex,
                    "oledRotation": t.profile.oled_rotation_degrees(),
                    "widgetRotation": t.profile.widget_rotation_degrees(),
                })
            })
            .collect();
        let json_str = serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string());
        QString::from(&json_str)
    }

    pub fn save_profile_as_template(
        mut self: Pin<&mut Self>,
        profile_number: i32,
        template_name: QString,
        description: QString,
    ) -> bool {
        let target_num = if profile_number > 0 {
            profile_number as u8
        } else {
            (*self.active_profile()).max(1) as u8
        };
        let prof_opt = self
            .rust()
            .profiles
            .iter()
            .find(|p| p.number == target_num)
            .cloned();
        let Some(prof) = prof_opt else {
            self.as_mut()
                .set_status_message(QString::from("Profile not found to save as template"));
            return false;
        };
        match save_profile_as_user_template(
            &prof,
            &template_name.to_string(),
            &description.to_string(),
        ) {
            Ok(saved) => {
                let msg = format!(
                    "✓ Saved Profile #{} ({}) as template \"{}\"",
                    target_num, prof.name, saved.name
                );
                record_system_event("INFO", "PROFILE", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                true
            }
            Err(err) => {
                let err_msg = format!("Failed saving template: {err}");
                self.as_mut().set_status_message(QString::from(&err_msg));
                false
            }
        }
    }

    pub fn delete_profile_template(
        mut self: Pin<&mut Self>,
        template_id: QString,
    ) -> bool {
        let id = template_id.to_string();
        match delete_user_profile_template(&id) {
            Ok(()) => {
                let msg = "✓ Deleted custom profile template";
                record_system_event("INFO", "PROFILE", msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(msg));
                true
            }
            Err(err) => {
                let err_msg = format!("Failed deleting template: {err}");
                self.as_mut().set_status_message(QString::from(&err_msg));
                false
            }
        }
    }

    pub fn create_profile_from_template(
        mut self: Pin<&mut Self>,
        profile_name: QString,
        template_id: QString,
    ) -> bool {
        let tpl_id = template_id.to_string();
        if tpl_id.trim().is_empty() || tpl_id == "builtin_blank" {
            return self.create_new_profile(profile_name);
        }
        let trimmed = profile_name.to_string().trim().to_string();
        if trimmed.is_empty() {
            return false;
        }
        let clean_name: String = trimmed
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
            .take(16)
            .collect();
        if clean_name.is_empty() {
            return false;
        }

        let res = {
            let rust = self.as_mut().rust_mut().get_mut();
            let max_num = rust.profiles.iter().map(|p| p.number).max().unwrap_or(0);
            if max_num >= 64 {
                return false;
            }
            let next_num = max_num + 1;
            let mut new_prof = instantiate_profile_from_template(&tpl_id, next_num, &clean_name);
            let is_dpp = rust
                .sd_root_path
                .as_ref()
                .map(|r| r.join("profile_info.txt").is_file())
                .unwrap_or(true);
            if is_dpp {
                new_prof.folder_name_override = Some(format!("profile_{clean_name}"));
            } else {
                new_prof.folder_name_override = None;
            }

            let mut saved = false;
            if let Some(ref sd_root) = rust.sd_root_path {
                if new_prof.save_to_sd_root(sd_root).is_ok() {
                    saved = true;
                    let _ = save_profile_order_to_sd(sd_root, &rust.profiles);
                }
            }
            rust.profiles.push(new_prof);
            if let Some(ref sd_root) = rust.sd_root_path {
                let _ = save_profile_order_to_sd(sd_root, &rust.profiles);
            }
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if saved {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            Some((next_num as i32, saved))
        };

        if let Some((next_num, saved_to_sd)) = res {
            self.as_mut().set_active_profile(next_num);
            self.as_mut().sync_active_profile_state();
            let suffix = if saved_to_sd { " (saved to SD)" } else { "" };
            let msg = format!(
                "✓ Created Profile #{next_num}: {clean_name} from template{suffix}"
            );
            record_system_event("INFO", "PROFILE", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            true
        } else {
            false
        }
    }

    pub fn apply_template_to_profile(
        mut self: Pin<&mut Self>,
        profile_number: i32,
        template_id: QString,
    ) -> bool {
        let target_num = if profile_number > 0 {
            profile_number as u8
        } else {
            (*self.active_profile()).max(1) as u8
        };
        let tpl_id = template_id.to_string();
        let updated_name = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == target_num) else {
                return false;
            };
            let cur_name = prof.name.clone();
            let cur_folder = prof.folder_name_override.clone();
            let inst = instantiate_profile_from_template(&tpl_id, target_num, &cur_name);
            *prof = inst;
            prof.folder_name_override = cur_folder;
            if let Some(root) = sd_root {
                let _ = prof.save_to_sd_root(&root);
            }
            let sn = rust.serial_number.to_string();
            if sn != "-" && !sn.is_empty() {
                sync_profiles_to_device_cache(&sn, &rust.profiles);
            } else {
                sync_profiles_to_cache(&rust.profiles);
            }
            Some(cur_name)
        };

        if let Some(name) = updated_name {
            self.as_mut().sync_active_profile_state();
            let msg = format!(
                "✓ Applied template to Profile #{} ({})",
                target_num, name
            );
            record_system_event("INFO", "PROFILE", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            true
        } else {
            false
        }
    }
}

static GLOBAL_MACRO_RECORDER: OnceLock<Mutex<Option<GlobalMacroEvdevRecorder>>> = OnceLock::new();

fn global_macro_recorder_store() -> &'static Mutex<Option<GlobalMacroEvdevRecorder>> {
    GLOBAL_MACRO_RECORDER.get_or_init(|| Mutex::new(None))
}


