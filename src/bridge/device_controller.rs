#![allow(clippy::too_many_arguments)]

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        include!("app_helper.h");
        type QString = cxx_qt_lib::QString;

        fn duckydeck_sync_tray(
            controller: Pin<&mut DeviceController>,
            icon_mode: &QString,
            status_header: &QString,
            profile_header: &QString,
            sd_space_header: &QString,
            tooltip_text: &QString,
            flags: u8,
        );
        fn duckydeck_show_main_window();
        fn duckydeck_hide_main_window();
        fn duckydeck_quit_app();
        fn duckydeck_exit_app(exit_code: i32);
        fn duckydeck_restart_app();
        fn duckydeck_show_tray_notification(title: &QString, message: &QString);
        fn duckydeck_extract_qrc_user_guide(dest_path: &QString) -> bool;
        fn duckydeck_dispatch_ipc_command(command: &QString);
        fn duckydeck_toggle_oled_widget();
        fn duckydeck_restore_oled_widget_geometry();
        fn duckydeck_save_oled_widget_appearance();
        fn duckydeck_record_oled_geometry(x: i32, y: i32, width: i32, height: i32);
        fn duckydeck_apply_app_theme(theme_mode: &QString);
        fn duckydeck_apply_ui_font_scale(scale_percent: i32);
        fn duckydeck_send_key_to_active_window(
            qt_key: i32,
            qt_modifiers: i32,
            text: &QString,
        ) -> bool;
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
        #[qproperty(bool, autoswitch_enable_section_expanded)]
        #[qproperty(bool, autoswitch_rule_editor_expanded)]
        #[qproperty(bool, modifier_switch_enabled)]
        #[qproperty(bool, modifier_section_expanded)]
        #[qproperty(QString, modifier_rules_json)]
        #[qproperty(bool, modifier_layer_active)]
        #[qproperty(QString, modifier_layer_status)]
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
        #[qproperty(QString, saved_color_names_json)]
        #[qproperty(QString, used_colors_json)]
        #[qproperty(bool, rest_api_enabled)]
        #[qproperty(QString, rest_api_bind_ip)]
        #[qproperty(i32, rest_api_port)]
        #[qproperty(bool, rest_api_require_auth)]
        #[qproperty(QString, rest_api_token)]
        #[qproperty(QString, rest_api_status)]
        #[qproperty(bool, firmware_fetch_busy)]
        #[qproperty(bool, show_first_run_dialog)]
        #[qproperty(QString, onboarding_state_json)]
        #[qproperty(bool, google_search_shortcuts_permitted)]
        #[qproperty(bool, search_shortcuts_busy)]
        #[qproperty(QString, discovered_shortcuts_json)]
        #[qproperty(QString, auto_switch_error_message)]
        #[qproperty(i32, multi_click_interval_ms)]
        #[qproperty(QString, available_devices_json)]
        #[qproperty(QString, multi_device_oled_states_json)]
        #[qproperty(i32, connected_device_count)]
        #[qproperty(i32, active_hw_model)]
        #[qproperty(bool, is_duckypad_2020, cxx_name = "isDuckyPad2020")]
        #[qproperty(i32, onboard_key_count)]
        #[qproperty(QString, custom_device_name, cxx_name = "customDeviceName")]
        #[qproperty(bool, esp32_ha_detected)]
        #[qproperty(QString, esp32_ha_device_serial)]
        #[qproperty(bool, esp32_ha_server_running)]
        #[qproperty(i32, esp32_ha_server_port)]
        #[qproperty(QString, esp32_ha_server_url)]
        #[qproperty(bool, app_update_available)]
        #[qproperty(QString, latest_app_version)]
        #[qproperty(QString, app_update_url)]
        #[qproperty(QString, app_update_download_url)]
        #[qproperty(QString, app_update_status)]
        #[qproperty(bool, app_update_check_busy)]
        #[qproperty(bool, app_update_install_busy)]
        #[qproperty(bool, check_updates_on_startup)]
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
        fn set_auto_switch_timings(self: Pin<&mut DeviceController>, settle_ms: i32, dwell_ms: i32);

        #[qinvokable]
        #[cxx_name = "setAutoSwitchSectionsExpanded"]
        fn set_auto_switch_sections_expanded(
            self: Pin<&mut DeviceController>,
            enable_expanded: bool,
            rule_editor_expanded: bool,
        );

        #[qinvokable]
        #[cxx_name = "reloadAutoSwitchSectionsFromDisk"]
        fn reload_auto_switch_sections_from_disk(self: Pin<&mut DeviceController>);

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
        #[cxx_name = "saveAutoSwitchRuleFull"]
        fn save_auto_switch_rule_full(
            self: Pin<&mut DeviceController>,
            rule_index: i32,
            enabled: bool,
            resource_class: QString,
            title_contains: QString,
            title_is_regex: bool,
            favicon_url: QString,
            target_profile: i32,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "validateTitleRegexPattern"]
        fn validate_title_regex_pattern(self: &DeviceController, pattern: QString) -> QString;

        #[qinvokable]
        fn remove_auto_switch_rule(self: Pin<&mut DeviceController>, rule_index: i32) -> bool;

        #[qinvokable]
        fn toggle_auto_switch_rule(
            self: Pin<&mut DeviceController>,
            rule_index: i32,
            enabled: bool,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "updateModifierSwitchEnabled"]
        fn update_modifier_switch_enabled(self: Pin<&mut DeviceController>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "updateModifierSectionExpanded"]
        fn update_modifier_section_expanded(self: Pin<&mut DeviceController>, expanded: bool);

        #[qinvokable]
        #[cxx_name = "saveModifierRule"]
        fn save_modifier_rule(
            self: Pin<&mut DeviceController>,
            rule_index: i32,
            enabled: bool,
            trigger_key: QString,
            mode: QString,
            hold_delay_ms: i32,
            oneshot_timeout_sec: i32,
            target_profile: i32,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "removeModifierRule"]
        fn remove_modifier_rule(self: Pin<&mut DeviceController>, rule_index: i32) -> bool;

        #[qinvokable]
        #[cxx_name = "toggleModifierRule"]
        fn toggle_modifier_rule(
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
        fn import_profile_from_path(self: Pin<&mut DeviceController>, source_path: QString)
            -> bool;

        #[qinvokable]
        fn save_profile_group(
            self: Pin<&mut DeviceController>,
            destination_path: QString,
            group_name: QString,
        ) -> bool;

        #[qinvokable]
        fn load_profile_group(self: Pin<&mut DeviceController>, source_path: QString) -> bool;

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
        fn scrape_app_shortcuts_json(self: &DeviceController, app_name: QString) -> QString;

        #[qinvokable]
        fn generate_ai_macro(self: Pin<&mut DeviceController>, app_name: QString, prompt: QString);

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
        fn delete_web_api_destination(self: Pin<&mut DeviceController>, name: QString) -> QString;

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
        #[cxx_name = "saveSavedColorName"]
        fn save_saved_color_name(
            self: Pin<&mut DeviceController>,
            hex_color: QString,
            name: QString,
        ) -> bool;

        #[qinvokable]
        fn is_demo_showcase_mode(self: &DeviceController) -> bool;

        #[qinvokable]
        #[cxx_name = "isUiTestSuiteMode"]
        fn is_ui_test_suite_mode(self: &DeviceController) -> bool;

        #[qinvokable]
        #[cxx_name = "writeUiTestReport"]
        fn write_ui_test_report(self: &DeviceController, report_json: QString, exit_code: i32);

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
        #[cxx_name = "demoCaptureDir"]
        fn demo_capture_dir(self: &DeviceController) -> QString;

        #[qinvokable]
        #[cxx_name = "openUserGuide"]
        fn open_user_guide(self: &DeviceController);

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
        fn set_active_profile_oled_rotation(self: Pin<&mut DeviceController>, degrees: i32)
            -> bool;

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
        fn delete_profile_template(self: Pin<&mut DeviceController>, template_id: QString) -> bool;

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

        #[qinvokable]
        #[cxx_name = "applyRestApiConfig"]
        fn apply_rest_api_config(
            self: Pin<&mut DeviceController>,
            enabled: bool,
            bind_ip: QString,
            port: i32,
            require_auth: bool,
            token: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "regenerateRestApiToken"]
        fn regenerate_rest_api_token(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        #[cxx_name = "resetRestApiDefaults"]
        fn reset_rest_api_defaults(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        #[cxx_name = "getGlobalProfileWebhookJson"]
        fn get_global_profile_webhook_json(self: &DeviceController) -> QString;

        #[qinvokable]
        #[cxx_name = "saveGlobalProfileWebhookJson"]
        fn save_global_profile_webhook_json(
            self: Pin<&mut DeviceController>,
            config_json: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "testProfileWebhookJson"]
        fn test_profile_webhook_json(
            self: Pin<&mut DeviceController>,
            config_json: QString,
            profile_number: i32,
        );

        #[qinvokable]
        #[cxx_name = "getHomeAssistantConfigJson"]
        fn get_home_assistant_config_json(self: &DeviceController) -> QString;

        #[qinvokable]
        #[cxx_name = "saveHomeAssistantConfig"]
        fn save_home_assistant_config_qinvokable(
            self: Pin<&mut DeviceController>,
            base_url: QString,
            access_token: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "searchHomeAssistantEntitiesJson"]
        fn search_home_assistant_entities_json(
            self: Pin<&mut DeviceController>,
            base_url: QString,
            access_token: QString,
            query: QString,
            domain_filter: QString,
        ) -> QString;

        #[qinvokable]
        #[cxx_name = "fetchOfficialFirmwaresFromSite"]
        fn fetch_official_firmwares_from_site(self: Pin<&mut DeviceController>);

        #[qinvokable]
        #[cxx_name = "downloadOfficialFirmwareOnly"]
        fn download_official_firmware_only(
            self: Pin<&mut DeviceController>,
            filename: QString,
            download_url: QString,
        );

        #[qinvokable]
        #[cxx_name = "downloadAndInstallOfficialFirmware"]
        fn download_and_install_official_firmware(
            self: Pin<&mut DeviceController>,
            filename: QString,
            download_url: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "zoomInUiTextScale"]
        fn zoom_in_ui_text_scale(self: Pin<&mut DeviceController>) -> i32;

        #[qinvokable]
        #[cxx_name = "zoomOutUiTextScale"]
        fn zoom_out_ui_text_scale(self: Pin<&mut DeviceController>) -> i32;

        #[qinvokable]
        #[cxx_name = "reloadZoomLevelFromDisk"]
        fn reload_zoom_level_from_disk(self: Pin<&mut DeviceController>) -> i32;

        #[qinvokable]
        #[cxx_name = "sendKeyToActiveWindow"]
        fn send_key_to_active_window(
            self: &DeviceController,
            qt_key: i32,
            qt_modifiers: i32,
            text: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "refreshOnboardingState"]
        fn refresh_onboarding_state(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        #[cxx_name = "stopConflictingSoftware"]
        fn stop_conflicting_software(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        #[cxx_name = "applyFirstRunImport"]
        fn apply_first_run_import(
            self: Pin<&mut DeviceController>,
            profile_source_id: QString,
            rule_source_id: QString,
            shutdown_conflicts: bool,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "dismissFirstRunDialog"]
        fn dismiss_first_run_dialog(self: Pin<&mut DeviceController>);

        #[qinvokable]
        #[cxx_name = "searchAppShortcuts"]
        fn search_app_shortcuts(
            self: Pin<&mut DeviceController>,
            app_name: QString,
            allow_google: bool,
        );

        #[qinvokable]
        #[cxx_name = "setGoogleSearchPermission"]
        fn set_google_search_permission(self: Pin<&mut DeviceController>, permitted: bool);

        #[qinvokable]
        #[cxx_name = "createWizardProfile"]
        fn create_wizard_profile(self: Pin<&mut DeviceController>, wizard_json: QString) -> bool;

        #[qinvokable]
        #[cxx_name = "assignShortcutToKeySlot"]
        fn assign_shortcut_to_key_slot(
            self: Pin<&mut DeviceController>,
            profile_num: i32,
            slot: i32,
            action: QString,
            shortcut: QString,
            duckyscript: QString,
            category: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "isGamingHoldScript"]
        fn is_gaming_hold_script(
            self: &DeviceController,
            press_script: QString,
            release_script: QString,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "convertScriptsForGamingHold"]
        fn convert_scripts_for_gaming_hold(
            self: &DeviceController,
            press_script: QString,
            release_script: QString,
            enable_gaming_hold: bool,
            action_hint: QString,
        ) -> QString;

        #[qinvokable]
        #[cxx_name = "applyMultiClickIntervalMs"]
        fn apply_multi_click_interval_ms(
            self: Pin<&mut DeviceController>,
            interval_ms: i32,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "splitMultiClickScriptJson"]
        fn split_multi_click_script_json(
            self: &DeviceController,
            on_press_script: QString,
        ) -> QString;

        #[qinvokable]
        #[cxx_name = "composeMultiClickScript"]
        fn compose_multi_click_script_qstr(
            self: &DeviceController,
            single_script: QString,
            double_script: QString,
            triple_script: QString,
        ) -> QString;

        #[qinvokable]
        #[cxx_name = "scanAvailableDevices"]
        fn scan_available_devices(self: Pin<&mut DeviceController>) -> QString;

        #[qinvokable]
        #[cxx_name = "selectManagedDevice"]
        fn select_managed_device(self: Pin<&mut DeviceController>, device_id: QString) -> bool;

        #[qinvokable]
        #[cxx_name = "setSimulatedDeviceModel"]
        fn set_simulated_device_model(self: Pin<&mut DeviceController>, hw_model: i32) -> bool;

        #[qinvokable]
        #[cxx_name = "switchProfileForDevice"]
        fn switch_profile_for_device(
            self: Pin<&mut DeviceController>,
            device_id: QString,
            profile_number: i32,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "setDeviceProfileWidgetRotation"]
        fn set_device_profile_widget_rotation(
            self: Pin<&mut DeviceController>,
            device_id: QString,
            degrees: i32,
        ) -> bool;

        #[qinvokable]
        #[cxx_name = "checkAppUpdates"]
        fn check_app_updates(self: Pin<&mut DeviceController>, manual: bool);

        #[qinvokable]
        #[cxx_name = "installAppUpdate"]
        fn install_app_update(self: Pin<&mut DeviceController>);

        #[qinvokable]
        #[cxx_name = "openAppUpdatePage"]
        fn open_app_update_page(self: &DeviceController);

        #[qinvokable]
        #[cxx_name = "setCheckUpdatesOnStartupEnabled"]
        fn set_check_updates_on_startup_enabled(self: Pin<&mut DeviceController>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "startEsp32HaWebServer"]
        fn start_esp32_ha_web_server(self: Pin<&mut DeviceController>, port: i32) -> bool;

        #[qinvokable]
        #[cxx_name = "stopEsp32HaWebServer"]
        fn stop_esp32_ha_web_server(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        #[cxx_name = "toggleEsp32HaWebServer"]
        fn toggle_esp32_ha_web_server(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        #[cxx_name = "openEsp32HaWebUi"]
        fn open_esp32_ha_web_ui(self: Pin<&mut DeviceController>) -> bool;

        #[qinvokable]
        #[cxx_name = "assignCustomDeviceName"]
        fn assign_custom_device_name(
            self: Pin<&mut DeviceController>,
            serial: &QString,
            name: &QString,
        ) -> bool;
    }

    impl cxx_qt::Threading for DeviceController {}
}

use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use hidapi::HidDevice;
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
    discard_unsynced_change_in_profiles, discover_dekunukem_profile_backups,
    download_official_firmware_binary, encode_pgv31_trigger, execute_app_launch,
    execute_profile_webhook_request, execute_web_api_request, extract_app_launch_config,
    extract_web_api_config, fetch_and_search_home_assistant_entities,
    fetch_official_firmwares_from_site, find_duckypad_msc_block_device, find_mounted_duckypad_sd,
    find_mounted_duckypad_sd_for_device, flash_slot_led_feedback, format_disk_label_for_device,
    get_sd_card_space_info, get_selected_duckypad_serial, inspect_duckypad_sd_root,
    install_duckypad_2020_firmware_dfu_with_progress, instantiate_profile_from_template,
    list_available_duckypad_2020_firmwares, list_available_duckypad_pro_firmwares,
    list_manageable_duckypad_targets, load_all_sd_profiles, load_home_assistant_config,
    load_profiles_for_connected_device, load_sd_synced_snapshot, load_web_api_destinations,
    mark_profiles_sd_synced, merge_host_only_profile_settings, model_name_for_hw_model,
    mount_duckypad_sd_only, normalize_duckypad_serial, oled_label_to_hardware_ascii,
    onboard_key_count_for_model, poll_and_clear_pgv31_trigger, poll_hardware_active_profile,
    profile_cache_dir_for_serial, query_connected_duckypad_info, read_dpp_config,
    read_duckypad_pro_profiles_via_msc, read_duckypad_pro_profiles_via_msc_with_progress,
    reboot_duckypad, run_storage_cmd, save_home_assistant_config, save_profile_as_user_template,
    save_profile_order_to_sd, scan_installed_desktop_apps, seed_web_api_destinations_from_profiles,
    set_selected_duckypad_serial, set_single_key_led_hid,
    stage_duckypad_pro_firmware_via_msc_with_progress, sync_profile_web_api_triggers,
    sync_profiles_to_cache, sync_profiles_to_device_cache, sync_profiles_to_device_cache_with_meta,
    try_auto_mount_duckypad_sd_for_device, unmount_mounted_duckypad_sd, upsert_web_api_destination,
    write_duckypad_pro_profiles_via_msc_with_progress, write_profiles_to_sd_root_for_model,
    AppLaunchConfig, DuckyKeySlot, DuckyProfile, DuckySdDeviceMeta, DuckypadEvdevListener,
    GlobalMacroEvdevRecorder, ProfileWebhookConfig, WebApiConfig, DP_MODEL_DUCKYPAD_PRO,
    DP_MODEL_OG_DUCKYPAD, HID_CMD_SET_LED_SINGLE, HID_REPORT_ID_OUT, MAX_INPUT_SLOTS,
    PID_DUCKYPAD_2020, PID_DUCKYPAD_PRO,
};
use crate::kwin::auto_switcher::{
    discover_dekunukem_autoswitcher_configs, is_kde_autostart_enabled, is_web_browser_class,
    load_rules_bundle_from_dir, modifier_mode_label, modifier_trigger_key_label,
    normalize_modifier_mode, normalize_modifier_trigger_key, parse_dekunukem_autoswitcher_file,
    save_rules_bundle_to_dir, send_hid_switch_profile_checked, set_kde_autostart_enabled,
    strip_browser_title_suffix, sync_local_desktop_and_icon, AutoSwitchRule, HidSwitchOutcome,
    ModifierLayerRule, ModifierLayerTransition, SwitcherState,
};

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_AUTHOR: &str = "Russell Milliner";
pub const APP_GITHUB_URL: &str = "https://github.com/russellmilliner/duckydeck-studio";

pub fn is_ui_test_suite_env() -> bool {
    std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1")
}

pub fn is_demo_showcase_env() -> bool {
    std::env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1") || is_ui_test_suite_env()
}

pub use crate::bridge::event_log::{
    classify_status_message, get_system_log_file_path, record_system_event,
    serialize_system_event_log_json, SystemLogEntry,
};
pub use crate::bridge::serialize::{ProfileExportBundle, ProfileGroupBundle};

pub fn run_application(start_minimized: bool) -> i32 {
    record_system_event(
        "INFO",
        "SYSTEM",
        format!(
            "DuckyDeck Studio v{} started (start_minimized={})",
            APP_VERSION, start_minimized
        ),
    );
    if !is_ui_test_suite_env() {
        sync_local_desktop_and_icon(true);
    }
    qobject::duckydeck_run_app(start_minimized)
}

pub struct SlotConfigSaveParams {
    pub slot_number: i32,
    pub label1: String,
    pub label2: String,
    pub on_press_script: String,
    pub on_release_script: String,
    pub allow_abort: bool,
    pub disable_autorepeat: bool,
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
    pub(crate) autoswitch_enable_section_expanded: bool,
    pub(crate) autoswitch_rule_editor_expanded: bool,
    pub(crate) modifier_switch_enabled: bool,
    pub(crate) modifier_section_expanded: bool,
    pub(crate) modifier_rules_json: QString,
    pub(crate) modifier_layer_active: bool,
    pub(crate) modifier_layer_status: QString,
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
    pub(crate) saved_color_names_json: QString,
    pub(crate) used_colors_json: QString,
    pub(crate) rest_api_enabled: bool,
    pub(crate) rest_api_bind_ip: QString,
    pub(crate) rest_api_port: i32,
    pub(crate) rest_api_require_auth: bool,
    pub(crate) rest_api_token: QString,
    pub(crate) rest_api_status: QString,
    pub(crate) firmware_fetch_busy: bool,
    pub(crate) show_first_run_dialog: bool,
    pub(crate) onboarding_state_json: QString,
    pub(crate) google_search_shortcuts_permitted: bool,
    pub(crate) search_shortcuts_busy: bool,
    pub(crate) discovered_shortcuts_json: QString,
    pub(crate) auto_switch_error_message: QString,
    pub(crate) multi_click_interval_ms: i32,
    pub(crate) available_devices_json: QString,
    pub(crate) multi_device_oled_states_json: QString,
    pub(crate) connected_device_count: i32,
    pub(crate) active_hw_model: i32,
    pub(crate) is_duckypad_2020: bool,
    pub(crate) onboard_key_count: i32,
    pub(crate) custom_device_name: QString,
    pub(crate) esp32_ha_detected: bool,
    pub(crate) esp32_ha_device_serial: QString,
    pub(crate) esp32_ha_server_running: bool,
    pub(crate) esp32_ha_server_port: i32,
    pub(crate) esp32_ha_server_url: QString,
    pub(crate) app_update_available: bool,
    pub(crate) latest_app_version: QString,
    pub(crate) app_update_url: QString,
    pub(crate) app_update_download_url: QString,
    pub(crate) app_update_status: QString,
    pub(crate) app_update_check_busy: bool,
    pub(crate) app_update_install_busy: bool,
    pub(crate) check_updates_on_startup: bool,
    pub(crate) notified_app_update_version: String,
    pub(crate) copied_key_slot: Option<DuckyKeySlot>,
    pub(crate) firmware_download_url: String,
    pub(crate) sd_root_path: Option<PathBuf>,
    pub(crate) profiles: Vec<DuckyProfile>,
    pub(crate) switcher_state: Arc<Mutex<SwitcherState>>,
    pub(crate) kwin_listener_started: bool,
    pub(crate) picker_bind_to_active_profile: bool,
    pub(crate) conflict_warning_dismissed: bool,
    pub(crate) pending_rule_import_source: Option<String>,
}

impl Default for DeviceControllerRust {
    fn default() -> Self {
        if is_demo_showcase_env() {
            return Self::create_showcase_instance();
        }

        let connected_info = query_connected_duckypad_info().ok();
        if let Some(ref info) = connected_info {
            if !info.serial.is_empty() && info.serial != "-" {
                set_selected_duckypad_serial(Some(&info.serial));
            }
        }
        let mounted_sd = if is_ui_test_suite_env() {
            None
        } else if let Some(ref info) = connected_info {
            try_auto_mount_duckypad_sd_for_device(
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
            )
        } else {
            try_auto_mount_duckypad_sd_for_device(None, None, None)
        };

        let mut sd_root_path: Option<PathBuf> = None;
        let mut profiles: Vec<DuckyProfile> = Vec::new();

        if let Some(ref sd_dir) = mounted_sd {
            if let Ok(mut loaded) = load_all_sd_profiles(sd_dir) {
                if !loaded.is_empty() {
                    if let Some(ref info) = connected_info {
                        if let Some(cached) =
                            load_profiles_for_connected_device(info.pid, &info.serial)
                        {
                            merge_host_only_profile_settings(&mut loaded, &cached);
                        }
                        let meta = DuckySdDeviceMeta {
                            serial_number: info.serial.clone(),
                            hw_model: info.hw_model,
                            model_name: model_name_for_hw_model(info.hw_model).to_string(),
                            firmware_version: info.fw_version.clone(),
                            disk_label: info.disk_label.clone(),
                        };
                        sync_profiles_to_device_cache_with_meta(&info.serial, &loaded, Some(&meta));
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
                } else if !info.disk_label.is_empty() {
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
        let destinations_json =
            serde_json::to_string(&initial_destinations).unwrap_or_else(|_| "[]".to_string());

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

        let (connected, model_str, fw_str, sn_str, active_prof, status_str, init_hw_model) =
            if let Some(ref info) = connected_info {
                let model = model_name_for_hw_model(info.hw_model);
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
                    info.hw_model,
                )
            } else if let Some(ref sd_dir) = sd_root_path {
                let dpp_cfg = read_dpp_config(sd_dir).unwrap_or_default();
                let sd_info = inspect_duckypad_sd_root(sd_dir);
                let hw_m = sd_info
                    .as_ref()
                    .map(|s| s.hw_model)
                    .or(dpp_cfg.hw_model)
                    .unwrap_or(DP_MODEL_DUCKYPAD_PRO);
                let fw = dpp_cfg.fw_ver.unwrap_or_else(|| "-".to_string());
                let sn = dpp_cfg
                    .serial_number
                    .or_else(|| sd_info.as_ref().and_then(|s| s.serial_number.clone()))
                    .unwrap_or_else(|| "-".to_string());
                if sn != "-" {
                    set_selected_duckypad_serial(Some(&sn));
                    mark_profiles_sd_synced(&sn, &profiles);
                }
                let model = if hw_m == DP_MODEL_OG_DUCKYPAD {
                    "duckyPad (2020) (SD Mode)"
                } else {
                    "duckyPad Pro (SD Mode)"
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
                    hw_m,
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
                    DP_MODEL_DUCKYPAD_PRO,
                )
            };

        if !sn_str.is_empty() && sn_str != "-" {
            let (dev_en, dev_fb, dev_rules) =
                crate::kwin::auto_switcher::load_device_rules_for_serial(
                    &sn_str,
                    &profiles,
                    sd_root_path.as_deref(),
                );
            switcher.enabled = dev_en;
            if let Some(fb) = dev_fb {
                switcher.default_profile = Some(fb);
            }
            if !dev_rules.is_empty() || crate::hardware::profile::has_other_cached_devices(&sn_str)
            {
                switcher.rules = dev_rules;
                switcher.loaded_from_disk = true;
            }
        }
        crate::kwin::auto_switcher::sync_secondary_connected_devices(
            &crate::hardware::storage::list_connected_duckypads(),
            get_selected_duckypad_serial().as_deref(),
        );

        let targets = list_manageable_duckypad_targets();
        let connected_count = targets.iter().filter(|t| t.usb_connected).count() as i32;
        let available_devices_json =
            serde_json::to_string(&targets).unwrap_or_else(|_| "[]".to_string());
        let is_dp20 = init_hw_model == DP_MODEL_OG_DUCKYPAD;
        let onboard_keys = onboard_key_count_for_model(init_hw_model) as i32;
        let init_custom_device_name =
            crate::hardware::profile::get_custom_duckypad_name(&sn_str).unwrap_or_default();
        let esp32_target_serial =
            if connected && crate::hardware::profile::is_esp32_ha_configured_profiles(&profiles) {
                Some(sn_str.clone())
            } else {
                targets
                    .iter()
                    .find(|t| t.is_esp32_ha_configured && t.usb_connected)
                    .map(|t| t.serial_number.clone())
            };
        let esp32_ha_detected = esp32_target_serial.is_some();
        let esp32_ha_device_serial = esp32_target_serial.unwrap_or_default();
        let esp32_cfg_exists = crate::paths::config_dir()
            .join("esp32_ha_bridge.json")
            .exists();
        let esp32_cfg = crate::actions::esp32_ha_server::load_esp32_ha_bridge_config();
        let esp32_ha_server_port = (esp32_cfg.port.max(1024)) as i32;
        if esp32_ha_detected && (esp32_cfg.enabled || !esp32_cfg_exists) {
            let _ = crate::actions::esp32_ha_server::start_esp32_ha_web_server(
                esp32_cfg.port.max(1024),
                if esp32_ha_device_serial.is_empty() {
                    None
                } else {
                    Some(esp32_ha_device_serial.clone())
                },
            );
        } else if !esp32_ha_detected {
            crate::actions::esp32_ha_server::stop_esp32_ha_server();
        }
        let esp32_ha_server_running = crate::actions::esp32_ha_server::is_esp32_ha_server_running();
        let esp32_ha_server_url = crate::actions::esp32_ha_server::esp32_ha_server_url();

        let auto_enabled = switcher.enabled;
        let max_init_prof = profiles.len().max(1) as u8;
        let clamped_fb = switcher
            .default_profile
            .unwrap_or(1)
            .clamp(1, max_init_prof);
        switcher.default_profile = Some(clamped_fb);
        let fallback_prof = clamped_fb as i32;
        let settle_ms = switcher.mouse_settle_ms.clamp(0, 5000);
        let dwell_ms = switcher.window_dwell_ms.clamp(0, 5000);
        let enable_sec_expanded = switcher.autoswitch_enable_section_expanded;
        let rule_editor_expanded = switcher.autoswitch_rule_editor_expanded;
        let mod_switch_enabled = switcher.modifier_switch_enabled;
        let mod_section_expanded = switcher.modifier_section_expanded;
        let tray_mode = switcher.tray_icon_mode.clone();
        let app_theme = switcher.app_theme_mode.clone();
        let close_tray = switcher.close_to_tray;
        let check_updates_startup = switcher.check_updates_on_startup;
        let google_shortcuts_permitted = switcher.google_search_shortcuts_permitted;
        let multi_click_ms = switcher.multi_click_interval_ms.clamp(80, 1000);
        let text_scale = switcher.ui_text_scale.clamp(75, 160);
        qobject::duckydeck_apply_ui_font_scale(text_scale);
        let sidebar_scale = text_scale;
        let canvas_scale = text_scale;
        let inspector_scale = text_scale;
        let editor_scale = text_scale;
        let status_bar_scale = text_scale;
        let event_log_scale = text_scale;
        let rest_enabled = switcher.rest_api_enabled;
        let rest_ip = switcher.rest_api_bind_ip.clone();
        let rest_port = switcher.rest_api_port as i32;
        let rest_auth = switcher.rest_api_require_auth;
        let rest_token = switcher.rest_api_token.clone();
        let rest_status = if rest_enabled {
            format!("Starting on http://{rest_ip}:{rest_port}...")
        } else {
            "Disabled".to_string()
        };
        let saved_colors_json =
            serde_json::to_string(&switcher.saved_colors).unwrap_or_else(|_| "[]".to_string());
        let saved_color_names_json =
            serde_json::to_string(&switcher.saved_color_names).unwrap_or_else(|_| "{}".to_string());
        let used_colors_json = Self::collect_used_colors_json(&profiles);
        let autostart = is_kde_autostart_enabled();
        if autostart {
            if let Err(e) = set_kde_autostart_enabled(true) {
                record_system_event(
                    "WARN",
                    "SYSTEM",
                    format!("Failed refreshing KDE autostart: {e}"),
                );
            }
        }
        let rules_json = Self::serialize_rules(&switcher.rules, &profiles);
        let modifier_rules_json =
            Self::serialize_modifier_rules(&switcher.modifier_rules, &profiles);
        let profiles_json = Self::serialize_profiles_summary(&profiles, &switcher.rules);
        let active_prof_obj = profiles
            .iter()
            .find(|p| p.number == active_prof as u8)
            .or_else(|| profiles.first());
        let active_prof_name = active_prof_obj
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Default".to_string());
        let active_bg_hex = active_prof_obj
            .map(|p| {
                format!(
                    "#{:02x}{:02x}{:02x}",
                    p.bg_color[0], p.bg_color[1], p.bg_color[2]
                )
            })
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

        let (onboarding_json, show_first_run) = Self::build_onboarding_state_json(
            connected,
            sd_detected,
            &model_str,
            &sn_str,
            &profiles,
            &switcher,
            false,
        );

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
            autoswitch_enable_section_expanded: enable_sec_expanded,
            autoswitch_rule_editor_expanded: rule_editor_expanded,
            modifier_switch_enabled: mod_switch_enabled,
            modifier_section_expanded: mod_section_expanded,
            modifier_rules_json: QString::from(&modifier_rules_json),
            modifier_layer_active: false,
            modifier_layer_status: QString::from(""),
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
            saved_color_names_json: QString::from(&saved_color_names_json),
            used_colors_json: QString::from(&used_colors_json),
            rest_api_enabled: rest_enabled,
            rest_api_bind_ip: QString::from(&rest_ip),
            rest_api_port: rest_port,
            rest_api_require_auth: rest_auth,
            rest_api_token: QString::from(&rest_token),
            rest_api_status: QString::from(&rest_status),
            firmware_fetch_busy: false,
            show_first_run_dialog: show_first_run,
            onboarding_state_json: QString::from(&onboarding_json),
            google_search_shortcuts_permitted: google_shortcuts_permitted,
            search_shortcuts_busy: false,
            discovered_shortcuts_json: QString::from("[]"),
            auto_switch_error_message: QString::from(""),
            multi_click_interval_ms: multi_click_ms,
            available_devices_json: QString::from(&available_devices_json),
            multi_device_oled_states_json: QString::from("{}"),
            connected_device_count: connected_count,
            active_hw_model: init_hw_model as i32,
            is_duckypad_2020: is_dp20,
            onboard_key_count: onboard_keys,
            custom_device_name: QString::from(&init_custom_device_name),
            esp32_ha_detected,
            esp32_ha_device_serial: QString::from(&esp32_ha_device_serial),
            esp32_ha_server_running,
            esp32_ha_server_port,
            esp32_ha_server_url: QString::from(&esp32_ha_server_url),
            app_update_available: false,
            latest_app_version: QString::from(APP_VERSION),
            app_update_url: QString::from(crate::updater::GITHUB_RELEASES_HTML_URL),
            app_update_download_url: QString::from(""),
            app_update_status: QString::from(""),
            app_update_check_busy: false,
            app_update_install_busy: false,
            check_updates_on_startup: check_updates_startup,
            notified_app_update_version: String::new(),
            copied_key_slot: None,
            firmware_download_url: String::new(),
            sd_root_path,
            profiles,
            switcher_state: Arc::new(Mutex::new(switcher)),
            kwin_listener_started: false,
            picker_bind_to_active_profile: false,
            conflict_warning_dismissed: false,
            pending_rule_import_source: None,
        }
    }
}

impl DeviceControllerRust {
    pub fn collect_used_colors_json(profiles: &[DuckyProfile]) -> String {
        crate::bridge::serialize::collect_used_colors_json(profiles)
    }

    pub fn create_default_demo_profile() -> DuckyProfile {
        crate::bridge::showcase::create_default_demo_profile()
    }

    pub fn create_showcase_profiles_and_rules() -> (Vec<DuckyProfile>, Vec<AutoSwitchRule>) {
        crate::bridge::showcase::create_showcase_profiles_and_rules()
    }

    pub fn create_showcase_instance() -> Self {
        crate::bridge::showcase::create_showcase_instance()
    }

    pub fn serialize_profiles_summary(
        profiles: &[DuckyProfile],
        rules: &[AutoSwitchRule],
    ) -> String {
        crate::bridge::serialize::serialize_profiles_summary(profiles, rules)
    }

    pub fn serialize_rules(rules: &[AutoSwitchRule], profiles: &[DuckyProfile]) -> String {
        crate::bridge::serialize::serialize_rules(rules, profiles)
    }

    pub fn serialize_modifier_rules(
        rules: &[ModifierLayerRule],
        profiles: &[DuckyProfile],
    ) -> String {
        crate::bridge::serialize::serialize_modifier_rules(rules, profiles)
    }

    pub fn serialize_profile_slots(profile: &DuckyProfile) -> String {
        crate::bridge::serialize::serialize_profile_slots(profile)
    }

    pub fn first_run_marker_paths() -> Vec<PathBuf> {
        let primary = crate::paths::config_dir().join("first_run_onboarding_state.json");
        let host = crate::paths::host_config_dir().join("first_run_onboarding_state.json");
        if primary == host {
            vec![primary]
        } else {
            vec![primary, host]
        }
    }

    pub fn is_first_run_onboarding_completed() -> bool {
        Self::first_run_marker_paths().iter().any(|p| p.is_file())
    }

    pub fn mark_first_run_onboarding_completed() {
        let payload = json!({
            "completed": true,
            "version": APP_VERSION,
        })
        .to_string();
        for path in Self::first_run_marker_paths() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&path, &payload);
        }
    }

    fn classify_legacy_cmdline(cmdline: &str) -> Option<(&'static str, &'static str)> {
        let lower = cmdline.to_ascii_lowercase();
        if lower.contains("duckydeck") || lower.contains("flatpak-spawn") {
            return None;
        }
        if lower.contains("duckypad_autoprofile")
            || lower.contains("duckypad-profile-autoswitcher")
            || lower.contains("duckypad_autoswitcher")
        {
            return Some(("autoswitcher", "dekuNukem duckyPad Auto-Switcher"));
        }
        if lower.contains("duckypad_config") || lower.contains("duckypad-configurator") {
            return Some(("configurator", "dekuNukem duckyPad Configurator"));
        }
        None
    }

    pub fn detect_conflicting_legacy_processes() -> Vec<serde_json::Value> {
        let self_pid = std::process::id();
        let mut seen_pids = std::collections::HashSet::new();
        let mut conflicts = Vec::new();

        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let fname = entry.file_name();
                let Ok(pid) = fname.to_string_lossy().parse::<u32>() else {
                    continue;
                };
                if pid == self_pid {
                    continue;
                }
                let Ok(raw) = fs::read(entry.path().join("cmdline")) else {
                    continue;
                };
                if raw.is_empty() {
                    continue;
                }
                let cmdline: String = raw
                    .split(|&b| b == 0)
                    .filter(|s| !s.is_empty())
                    .map(|s| String::from_utf8_lossy(s).into_owned())
                    .collect::<Vec<_>>()
                    .join(" ");
                if let Some((kind, label)) = Self::classify_legacy_cmdline(&cmdline) {
                    if seen_pids.insert(pid) {
                        conflicts.push(json!({
                            "pid": pid,
                            "kind": kind,
                            "name": label,
                            "cmdline": cmdline,
                        }));
                    }
                }
            }
        }

        if crate::paths::is_flatpak() {
            if let Ok(out) = run_storage_cmd("ps", &["-eo", "pid=,args="]) {
                if out.status.success() {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    for line in stdout.lines() {
                        let trimmed = line.trim();
                        let Some((pid_str, args_str)) = trimmed.split_once(char::is_whitespace)
                        else {
                            continue;
                        };
                        let Ok(pid) = pid_str.trim().parse::<u32>() else {
                            continue;
                        };
                        let cmdline = args_str.trim();
                        if let Some((kind, label)) = Self::classify_legacy_cmdline(cmdline) {
                            if seen_pids.insert(pid) {
                                conflicts.push(json!({
                                    "pid": pid,
                                    "kind": kind,
                                    "name": label,
                                    "cmdline": cmdline,
                                }));
                            }
                        }
                    }
                }
            }
        }

        conflicts
    }

    pub fn stop_conflicting_legacy_processes() -> (usize, Vec<String>) {
        let initial = Self::detect_conflicting_legacy_processes();
        if initial.is_empty() {
            return (0, Vec::new());
        }
        let mut names = Vec::new();
        for item in &initial {
            if let Some(pid) = item.get("pid").and_then(|v| v.as_u64()) {
                let pid_str = pid.to_string();
                let _ = run_storage_cmd("kill", &["-TERM", &pid_str]);
                if let Some(name) = item.get("name").and_then(|v| v.as_str()) {
                    names.push(format!("{name} (PID {pid})"));
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
        let remaining = Self::detect_conflicting_legacy_processes();
        for item in &remaining {
            if let Some(pid) = item.get("pid").and_then(|v| v.as_u64()) {
                let pid_str = pid.to_string();
                let _ = run_storage_cmd("kill", &["-KILL", &pid_str]);
            }
        }
        (initial.len(), names)
    }

    pub fn build_onboarding_state_json(
        connected: bool,
        sd_detected: bool,
        model_name: &str,
        serial_number: &str,
        profiles: &[DuckyProfile],
        switcher: &SwitcherState,
        conflict_warning_dismissed: bool,
    ) -> (String, bool) {
        let conflicts = Self::detect_conflicting_legacy_processes();
        let first_run_pending = !Self::is_first_run_onboarding_completed();
        let deck_detected = connected || sd_detected;

        let backups = discover_dekunukem_profile_backups();
        let autoswitcher_configs = discover_dekunukem_autoswitcher_configs();

        let mut profile_sources = Vec::new();
        let deck_detail = if serial_number != "-" && !serial_number.is_empty() {
            format!("Load configurations directly from connected {model_name} (SN {serial_number})")
        } else {
            "Mount and load existing profile folders directly from duckyPad microSD".to_string()
        };
        profile_sources.push(json!({
            "id": "deck_sd",
            "label": format!(
                "Connected duckyPad Hardware / microSD ({} profile(s) currently loaded)",
                profiles.len()
            ),
            "detail": deck_detail,
            "profileCount": profiles.len(),
            "recommended": true
        }));

        for (b_path, b_name, b_count) in &backups {
            let path_str = b_path.to_string_lossy().to_string();
            profile_sources.push(json!({
                "id": format!("dekunukem_backup:{path_str}"),
                "label": format!("dekuNukem Configurator Backup — {b_name} ({b_count} profiles)"),
                "detail": path_str,
                "profileCount": b_count,
                "recommended": false
            }));
        }

        profile_sources.push(json!({
            "id": "keep_current",
            "label": format!("Keep Currently Loaded Profiles ({} profiles)", profiles.len()),
            "detail": "Continue without reloading profiles",
            "profileCount": profiles.len(),
            "recommended": false
        }));

        let mut rule_sources = Vec::new();
        let existing_rule_count = switcher.rules.len();
        let recommend_existing_rules = existing_rule_count > 14 || autoswitcher_configs.is_empty();

        rule_sources.push(json!({
            "id": "existing_duckydeck_rules",
            "label": format!("Existing DuckyDeck Studio / SD Rules ({existing_rule_count} rules)"),
            "detail": "Preserves your DuckyDeck Studio KWin auto-switch rules (host, Flatpak & SD synced)",
            "ruleCount": existing_rule_count,
            "recommended": recommend_existing_rules
        }));

        for (idx, (cfg_path, rule_count)) in autoswitcher_configs.iter().enumerate() {
            let path_str = cfg_path.to_string_lossy().to_string();
            let cfg_name = cfg_path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("duckypad_autoprofile_config.json");
            rule_sources.push(json!({
                "id": format!("dekunukem_rules:{path_str}"),
                "label": format!("dekuNukem Auto-Switcher — {cfg_name} ({rule_count} rules)"),
                "detail": path_str,
                "ruleCount": rule_count,
                "recommended": !recommend_existing_rules && idx == 0
            }));
        }

        if !recommend_existing_rules {
            rule_sources.push(json!({
                "id": "keep_current",
                "label": format!("Keep Current Auto-Switch Rules ({existing_rule_count} rules)"),
                "detail": "Do not import external auto-switcher rules",
                "ruleCount": existing_rule_count,
                "recommended": false
            }));
        }

        let should_show = !is_demo_showcase_env()
            && ((deck_detected && first_run_pending)
                || (!conflicts.is_empty() && !conflict_warning_dismissed));

        let payload = json!({
            "firstRunPending": first_run_pending,
            "deckDetected": deck_detected,
            "modelName": model_name,
            "serialNumber": serial_number,
            "conflicts": conflicts,
            "profileSources": profile_sources,
            "ruleSources": rule_sources,
        });

        (payload.to_string(), should_show)
    }

    pub fn apply_rule_source_to_switcher(
        switcher: &mut SwitcherState,
        profiles: &[DuckyProfile],
        rule_source_id: &str,
        sd_root_path: Option<&Path>,
        serial_number: &str,
    ) -> Result<usize, String> {
        if let Some(path_str) = rule_source_id.strip_prefix("dekunukem_rules:") {
            let (switch_enabled, fallback_prof, imported_rules) =
                parse_dekunukem_autoswitcher_file(Path::new(path_str), profiles)?;
            switcher.rules = imported_rules;
            if let Some(fb) = fallback_prof {
                switcher.default_profile = Some(fb);
            }
            switcher.enabled = switch_enabled;
            switcher.loaded_from_disk = true;
            let _ = switcher.save_to_disk();
            if let Some(sd_root) = sd_root_path {
                let _ = save_rules_bundle_to_dir(
                    sd_root,
                    switcher.enabled,
                    switcher.default_profile,
                    &switcher.rules,
                    profiles,
                );
            }
            let cache_dir = profile_cache_dir_for_serial(serial_number);
            let _ = save_rules_bundle_to_dir(
                &cache_dir,
                switcher.enabled,
                switcher.default_profile,
                &switcher.rules,
                profiles,
            );
            return Ok(switcher.rules.len());
        }

        if rule_source_id == "existing_duckydeck_rules" {
            if let Some(sd_root) = sd_root_path {
                if let Some((sd_en, sd_fb, sd_rules)) =
                    load_rules_bundle_from_dir(sd_root, profiles)
                {
                    if sd_rules.len() > switcher.rules.len() {
                        switcher.enabled = sd_en;
                        switcher.rules = sd_rules;
                        if let Some(fb) = sd_fb {
                            switcher.default_profile = Some(fb);
                        }
                    }
                }
            }
            switcher.align_rules_with_profiles(profiles);
            let _ = switcher.save_to_disk();
            if let Some(sd_root) = sd_root_path {
                let _ = save_rules_bundle_to_dir(
                    sd_root,
                    switcher.enabled,
                    switcher.default_profile,
                    &switcher.rules,
                    profiles,
                );
            }
            let cache_dir = profile_cache_dir_for_serial(serial_number);
            let _ = save_rules_bundle_to_dir(
                &cache_dir,
                switcher.enabled,
                switcher.default_profile,
                &switcher.rules,
                profiles,
            );
            return Ok(switcher.rules.len());
        }

        switcher.align_rules_with_profiles(profiles);
        Ok(switcher.rules.len())
    }
}

impl qobject::DeviceController {
    pub(crate) fn sync_tray_state(self: Pin<&mut Self>) {
        self.sync_tray_state_inner(false);
    }

    pub(crate) fn sync_tray_state_inner(mut self: Pin<&mut Self>, refresh_sd_space: bool) {
        static LAST_OLED_STATE_JSON: std::sync::OnceLock<std::sync::Mutex<String>> =
            std::sync::OnceLock::new();

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

        let cached_summary = self.sd_space_summary().to_string();
        let (summary_text, tray_text) = if refresh_sd_space || cached_summary.is_empty() {
            let sd_mount_opt = if sd_path.is_empty() {
                None
            } else {
                Some(Path::new(&sd_path))
            };
            let sd_space = get_sd_card_space_info(&sn, sd_mount_opt);
            if cached_summary != sd_space.summary_text {
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
            (sd_space.summary_text, sd_space.tray_text)
        } else {
            let tray = format!("SD Space: {cached_summary}");
            (cached_summary, tray)
        };
        let sd_space_header = QString::from(&format!("💾 {tray_text}"));

        let mut tooltip = format!(
            "DuckyDeck Studio — {model}\n\
             Selected Profile: #{prof_num} ({prof_name}) [{total_profiles} loaded]\n\
             Connection: {conn_badge}\n\
             {tray_text}"
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

        if !is_ui_test_suite_env() {
            let flags = (u8::from(use_light_on_dark_icon))
                | (u8::from(auto_enabled) << 1)
                | (u8::from(autostart) << 2);
            qobject::duckydeck_sync_tray(
                self.as_mut(),
                &mode_qstr,
                &status_header,
                &profile_header,
                &sd_space_header,
                &tooltip_qstr,
                flags,
            );
        }

        let slots_raw = self.active_profile_slots_json().to_string();
        let bg_color_str = self.active_profile_bg_color().to_string();
        let slots_val: serde_json::Value =
            serde_json::from_str(&slots_raw).unwrap_or_else(|_| json!([]));
        let (oled_rot, widget_rot, eff_widget_rot) = self
            .rust()
            .profiles
            .iter()
            .find(|p| p.number as i32 == prof_num)
            .map(|p| {
                (
                    p.oled_rotation_degrees(),
                    p.widget_rotation_degrees(),
                    p.effective_widget_rotation_degrees(),
                )
            })
            .unwrap_or((0, -1, 0));
        let is_dp20 = *self.is_duckypad_2020();
        let onboard_keys = *self.onboard_key_count();
        let hw_model = *self.active_hw_model();
        let oled_snapshot = json!({
            "connected": connected,
            "modelName": model,
            "firmwareVersion": fw,
            "serialNumber": sn,
            "hwModel": hw_model,
            "isDuckyPad2020": is_dp20,
            "onboardKeyCount": onboard_keys,
            "activeProfile": prof_num,
            "activeProfileName": prof_name,
            "activeProfileBgColor": bg_color_str,
            "oledRotation": oled_rot,
            "widgetRotation": widget_rot,
            "effectiveWidgetRotation": eff_widget_rot,
            "totalProfiles": total_profiles,
            "sdSpaceSummary": summary_text,
            "slots": slots_val,
        });
        if let Ok(test_state_path) = std::env::var("DUCKYDECK_UI_TEST_OLED_STATE_PATH") {
            if !test_state_path.trim().is_empty() {
                if let Ok(serialized) = serde_json::to_string(&oled_snapshot) {
                    if let Err(e) = fs::write(test_state_path.trim(), serialized) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!(
                                "Failed writing test OLED state {}: {e}",
                                test_state_path.trim()
                            ),
                        );
                    }
                }
            }
        } else if !is_demo_showcase_env() {
            if let Ok(serialized) = serde_json::to_string(&oled_snapshot) {
                let cache =
                    LAST_OLED_STATE_JSON.get_or_init(|| std::sync::Mutex::new(String::new()));
                let changed = if let Ok(mut guard) = cache.lock() {
                    if *guard == serialized {
                        false
                    } else {
                        *guard = serialized.clone();
                        true
                    }
                } else {
                    true
                };
                if changed {
                    let state_dir = crate::paths::data_dir();
                    if let Err(e) = fs::create_dir_all(&state_dir) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed creating state dir {}: {e}", state_dir.display()),
                        );
                    }
                    let state_file = state_dir.join("oled_state.json");
                    if let Err(e) = fs::write(&state_file, serialized) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed writing {}: {e}", state_file.display()),
                        );
                    }
                }
            }
        }
    }

    pub(crate) fn sync_active_profile_selection_only(mut self: Pin<&mut Self>) {
        let active_num = *self.active_profile() as u8;
        let (prof_name, prof_bg_hex, slots_json, profile_exists) = {
            let rust = self.rust();
            if let Some(prof) = rust
                .profiles
                .iter()
                .find(|p| p.number == active_num)
                .or_else(|| rust.profiles.first())
            {
                let has_pgv31 = prof.slots.values().any(|s| {
                    s.on_press_script.contains("_GV31")
                        || s.on_release_script.contains("_GV31")
                        || crate::actions::web_api::has_web_api_config(&s.on_press_script)
                        || crate::actions::web_api::has_web_api_config(&s.on_release_script)
                        || crate::actions::app_launch::has_app_launch_config(&s.on_press_script)
                        || crate::actions::app_launch::has_app_launch_config(&s.on_release_script)
                });
                crate::hardware::storage::ACTIVE_PROFILE_HAS_PGV31_TRIGGER
                    .store(has_pgv31, std::sync::atomic::Ordering::SeqCst);
                let bg_hex = format!(
                    "#{:02x}{:02x}{:02x}",
                    prof.bg_color[0], prof.bg_color[1], prof.bg_color[2]
                );
                (
                    prof.name.clone(),
                    bg_hex,
                    DeviceControllerRust::serialize_profile_slots(prof),
                    prof.number == active_num,
                )
            } else {
                crate::hardware::storage::ACTIVE_PROFILE_HAS_PGV31_TRIGGER
                    .store(false, std::sync::atomic::Ordering::SeqCst);
                (
                    "Default".to_string(),
                    "#00aaff".to_string(),
                    "[]".to_string(),
                    false,
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
        if self.active_profile_slots_json().to_string() != slots_json {
            self.as_mut()
                .set_active_profile_slots_json(QString::from(&slots_json));
        }
        if self.profile_sync_state().to_string() == "synced"
            && profile_exists
            && !self.auto_switch_error_message().is_empty()
        {
            self.as_mut()
                .set_auto_switch_error_message(QString::from(""));
        }
        self.as_mut().sync_multi_device_oled_states();
        self.as_mut().sync_tray_state_inner(false);
    }

    pub(crate) fn sync_active_profile_state(mut self: Pin<&mut Self>) {
        let active_num = *self.active_profile() as u8;
        let connected = *self.connected();
        let serial_str = self.serial_number().to_string();
        let (
            prof_name,
            prof_bg_hex,
            profiles_json,
            slots_json,
            rules_json,
            modifier_rules_json,
            mod_switch_enabled,
            mod_section_expanded,
            sync_state,
            sync_detail,
            unsynced_json,
            effective_fallback,
        ) = {
            let rust = self.rust();
            let max_prof = rust.profiles.len().max(1) as u8;
            let (rules_vec, mod_rules_vec, mod_en, mod_exp, fb_num) =
                if let Ok(mut s) = rust.switcher_state.lock() {
                    let fb = s.default_profile.unwrap_or(1).clamp(1, max_prof);
                    if s.default_profile != Some(fb) {
                        s.default_profile = Some(fb);
                    }
                    (
                        s.rules.clone(),
                        s.modifier_rules.clone(),
                        s.modifier_switch_enabled,
                        s.modifier_section_expanded,
                        fb as i32,
                    )
                } else {
                    (Vec::new(), Vec::new(), true, true, 1)
                };
            let profiles_json =
                DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_vec);
            let rules_json = DeviceControllerRust::serialize_rules(&rules_vec, &rust.profiles);
            let modifier_rules_json =
                DeviceControllerRust::serialize_modifier_rules(&mod_rules_vec, &rust.profiles);
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
                let has_pgv31 = prof.slots.values().any(|s| {
                    s.on_press_script.contains("_GV31")
                        || s.on_release_script.contains("_GV31")
                        || crate::actions::web_api::has_web_api_config(&s.on_press_script)
                        || crate::actions::web_api::has_web_api_config(&s.on_release_script)
                        || crate::actions::app_launch::has_app_launch_config(&s.on_press_script)
                        || crate::actions::app_launch::has_app_launch_config(&s.on_release_script)
                });
                crate::hardware::storage::ACTIVE_PROFILE_HAS_PGV31_TRIGGER
                    .store(has_pgv31, std::sync::atomic::Ordering::SeqCst);
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
                    modifier_rules_json,
                    mod_en,
                    mod_exp,
                    sync_state,
                    sync_detail,
                    unsynced_json,
                    fb_num,
                )
            } else {
                crate::hardware::storage::ACTIVE_PROFILE_HAS_PGV31_TRIGGER
                    .store(false, std::sync::atomic::Ordering::SeqCst);
                (
                    "Default".to_string(),
                    "#00aaff".to_string(),
                    profiles_json,
                    "[]".to_string(),
                    rules_json,
                    modifier_rules_json,
                    mod_en,
                    mod_exp,
                    sync_state,
                    sync_detail,
                    unsynced_json,
                    fb_num,
                )
            }
        };

        if *self.modifier_switch_enabled() != mod_switch_enabled {
            self.as_mut()
                .set_modifier_switch_enabled(mod_switch_enabled);
        }
        if *self.modifier_section_expanded() != mod_section_expanded {
            self.as_mut()
                .set_modifier_section_expanded(mod_section_expanded);
        }
        if self.modifier_rules_json().to_string() != modifier_rules_json {
            self.as_mut()
                .set_modifier_rules_json(QString::from(&modifier_rules_json));
        }

        if *self.default_fallback_profile() != effective_fallback {
            self.as_mut()
                .set_default_fallback_profile(effective_fallback);
        }
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
        if sync_state == "synced"
            && self.rust().profiles.iter().any(|p| p.number == active_num)
            && !self.auto_switch_error_message().is_empty()
        {
            self.as_mut()
                .set_auto_switch_error_message(QString::from(""));
        }
        self.as_mut().sync_multi_device_oled_states();
        self.as_mut().sync_esp32_ha_state();
        self.as_mut().refresh_color_palettes();
        self.as_mut().sync_tray_state_inner(true);
    }

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
                        qobject.as_mut().set_sd_sync_busy(false);
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
                    qobject.as_mut().set_sd_sync_busy(false);
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

        let msg = "No mounted duckyPad microSD or connected duckyPad found to reload from";
        self.as_mut()
            .set_sd_sync_result_state(QString::from("error"));
        self.as_mut()
            .set_sd_sync_step(QString::from("No duckyPad storage available"));
        self.as_mut().set_sd_sync_result_summary(QString::from(msg));
        self.as_mut().set_status_message(QString::from(msg));
        false
    }

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

    pub fn select_profile(mut self: Pin<&mut Self>, profile_number: i32) -> bool {
        let prev_prof = (*self.active_profile()).clamp(1, 64) as u8;
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
        self.as_mut().sync_active_profile_selection_only();

        crate::kwin::auto_switcher::LAST_SW_PROFILE_TARGET
            .store(clamped as u8, std::sync::atomic::Ordering::SeqCst);
        crate::kwin::auto_switcher::LAST_SW_PROFILE_SWITCH_MS.store(
            crate::kwin::auto_switcher::current_epoch_ms(),
            std::sync::atomic::Ordering::SeqCst,
        );

        let mut switched_on_hw = false;
        if !is_demo_showcase_env() && *self.connected() {
            match crate::kwin::auto_switcher::send_hid_switch_profile_checked(clamped as u8) {
                crate::kwin::auto_switcher::HidSwitchOutcome::Switched => {
                    switched_on_hw = true;
                }
                crate::kwin::auto_switcher::HidSwitchOutcome::NotLoadedOnHardware => {
                    // Profile exists in DuckyDeck Studio cache but has not been synced to the
                    // physical microSD card yet; keep it selected in the editor without error spam.
                }
                crate::kwin::auto_switcher::HidSwitchOutcome::DeviceUnavailable => {
                    record_system_event(
                        "WARN",
                        "HID",
                        format!("Failed switching hardware to profile #{clamped}"),
                    );
                }
            }
        }

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if switched_on_hw {
                    sw.last_switched_profile = clamped as u8;
                }
            }
        }

        let prof_name = self.active_profile_name().to_string();
        self.as_mut().set_status_message(QString::from(&format!(
            "Loaded Profile #{clamped}: {prof_name}"
        )));
        self.as_mut().dispatch_profile_change_webhooks_if_changed(
            prev_prof,
            clamped as u8,
            "manual_select",
        );
        true
    }

    pub fn switch_profile(mut self: Pin<&mut Self>, profile_number: i32) -> bool {
        self.as_mut().select_profile(profile_number)
    }

    pub fn switch_profile_by_name(mut self: Pin<&mut Self>, profile_name: QString) -> bool {
        use crate::hardware::storage::{goto_profile_by_name, open_duckypad_hid};
        let name = profile_name.to_string();
        let prev_prof = (*self.active_profile()).clamp(1, 64) as u8;

        if let Some(num) = self
            .rust()
            .profiles
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(&name))
            .map(|p| p.number as i32)
        {
            self.as_mut().set_active_profile(num);
            self.as_mut().sync_active_profile_selection_only();
            self.as_mut().dispatch_profile_change_webhooks_if_changed(
                prev_prof,
                num.clamp(1, 64) as u8,
                "switch_by_name",
            );
        }

        match open_duckypad_hid().and_then(|dev| goto_profile_by_name(&dev, &name)) {
            Ok(()) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Switched to profile '{name}'")));
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
            if rust.profiles.len() >= 64 {
                return false;
            }
            let mut old_to_new: HashMap<u8, u8> = HashMap::with_capacity(rust.profiles.len());
            for (idx, prof) in rust.profiles.iter_mut().enumerate() {
                let new_num = (idx + 1) as u8;
                old_to_new.insert(prof.number, new_num);
                prof.number = new_num;
            }
            let next_num = (rust.profiles.len() + 1) as u8;
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
                            if let Err(e) = fs::write(&info_path, existing) {
                                record_system_event(
                                    "WARN",
                                    "STORAGE",
                                    format!("Failed updating {}: {e}", info_path.display()),
                                );
                            }
                        }
                    }
                }
            }

            rust.profiles.push(new_prof);
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let mut rules_changed = false;
                let old_fb = sw.default_profile.unwrap_or(1);
                let mapped_fb = old_to_new
                    .get(&old_fb)
                    .copied()
                    .unwrap_or(old_fb)
                    .clamp(1, next_num);
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
                if rules_changed {
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving switcher state after profile add: {e}"),
                        );
                    }
                }
            }
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if saved {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            (next_num as i32, saved)
        };

        crate::kwin::auto_switcher::LAST_SW_PROFILE_TARGET
            .store(next_num as u8, std::sync::atomic::Ordering::SeqCst);
        crate::kwin::auto_switcher::LAST_SW_PROFILE_SWITCH_MS.store(
            crate::kwin::auto_switcher::current_epoch_ms(),
            std::sync::atomic::Ordering::SeqCst,
        );
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
                        if let Err(e) = fs::rename(&old_dir, &new_dir) {
                            record_system_event(
                                "WARN",
                                "STORAGE",
                                format!(
                                    "Failed renaming {} to {}: {e}",
                                    old_dir.display(),
                                    new_dir.display()
                                ),
                            );
                        }
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
        let current_fb = self
            .rust()
            .switcher_state
            .lock()
            .ok()
            .and_then(|sw| sw.default_profile)
            .unwrap_or_else(|| (*self.default_fallback_profile()).max(1) as u8);
        if target_num == current_fb {
            self.as_mut().set_status_message(QString::from(&format!(
                "Cannot delete Profile #{target_num} because it is the Fallback Profile"
            )));
            return false;
        }

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

                let mut old_to_new: HashMap<u8, u8> = HashMap::with_capacity(rust.profiles.len());
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
                                    if let Err(e) = fs::rename(&old_dir, &new_dir) {
                                        record_system_event(
                                            "WARN",
                                            "STORAGE",
                                            format!(
                                                "Failed renumbering profile dir {} to {}: {e}",
                                                old_dir.display(),
                                                new_dir.display()
                                            ),
                                        );
                                    }
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
                    let mut rules_changed = false;
                    let old_fb = sw.default_profile.unwrap_or(current_fb);
                    let mapped_fb = old_to_new.get(&old_fb).copied().unwrap_or(1);
                    if sw.default_profile != Some(mapped_fb) {
                        sw.default_profile = Some(mapped_fb);
                        rules_changed = true;
                    }
                    new_fallback_num = Some(mapped_fb as i32);

                    for rule in &mut sw.rules {
                        if rule.target_profile == target_num {
                            if rule.target_profile != mapped_fb {
                                rule.target_profile = mapped_fb;
                                rules_changed = true;
                            }
                        } else if let Some(&mapped_target) = old_to_new.get(&rule.target_profile) {
                            if rule.target_profile != mapped_target {
                                rule.target_profile = mapped_target;
                                rules_changed = true;
                            }
                        }
                    }
                    for mrule in &mut sw.modifier_rules {
                        if mrule.target_profile == target_num {
                            if mrule.target_profile != mapped_fb {
                                mrule.target_profile = mapped_fb;
                                rules_changed = true;
                            }
                        } else if let Some(&mapped_target) = old_to_new.get(&mrule.target_profile) {
                            if mrule.target_profile != mapped_target {
                                mrule.target_profile = mapped_target;
                                rules_changed = true;
                            }
                        }
                    }
                    if rules_changed || sw.loaded_from_disk {
                        if let Err(e) = sw.save_to_disk() {
                            record_system_event(
                                "WARN",
                                "CONFIG",
                                format!("Failed saving switcher state after delete: {e}"),
                            );
                        }
                    }
                }

                let mut saved_to_sd = false;
                if let Some(ref sd_root) = rust.sd_root_path {
                    for folder in [&deleted_folder, &deleted_legacy_folder] {
                        let dir_path = sd_root.join(folder);
                        if dir_path.is_dir() {
                            if let Err(e) = fs::remove_dir_all(&dir_path) {
                                record_system_event(
                                    "WARN",
                                    "STORAGE",
                                    format!(
                                        "Failed removing deleted profile dir {}: {e}",
                                        dir_path.display()
                                    ),
                                );
                            }
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
            self.as_mut()
                .set_status_message(QString::from("Cannot delete the last remaining profile"));
            return false;
        };

        if let Some(fb) = new_fallback_num {
            self.as_mut().set_default_fallback_profile(fb);
        }
        self.as_mut().set_active_profile(new_active_num);
        if *self.connected()
            && !crate::kwin::auto_switcher::send_hid_switch_profile(new_active_num as u8)
        {
            record_system_event(
                "WARN",
                "HID",
                format!("Failed switching hardware to profile #{new_active_num} after delete"),
            );
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

    pub fn reorder_profile(mut self: Pin<&mut Self>, from_index: i32, to_index: i32) -> bool {
        if from_index < 0 || to_index < 0 || from_index == to_index {
            return false;
        }
        let from_idx = from_index as usize;
        let to_idx = to_index as usize;

        let active_old_num = *self.active_profile() as u8;
        let current_fb = (*self.default_fallback_profile()).max(1) as u8;
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
                let old_fb = sw.default_profile.unwrap_or(current_fb);
                let mapped_fb = old_to_new.get(&old_fb).copied().unwrap_or(1);
                if sw.default_profile != Some(mapped_fb) {
                    sw.default_profile = Some(mapped_fb);
                    rules_changed = true;
                }
                new_fallback_num = Some(mapped_fb as i32);

                for rule in &mut sw.rules {
                    let mapped_target = old_to_new
                        .get(&rule.target_profile)
                        .copied()
                        .unwrap_or(mapped_fb);
                    if rule.target_profile != mapped_target {
                        rule.target_profile = mapped_target;
                        rules_changed = true;
                    }
                }
                for mrule in &mut sw.modifier_rules {
                    let mapped_target = old_to_new
                        .get(&mrule.target_profile)
                        .copied()
                        .unwrap_or(mapped_fb);
                    if mrule.target_profile != mapped_target {
                        mrule.target_profile = mapped_target;
                        rules_changed = true;
                    }
                }
                if rules_changed || sw.loaded_from_disk {
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving switcher state after reorder: {e}"),
                        );
                    }
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

    pub fn set_key_rgb(mut self: Pin<&mut Self>, key_index: i32, r: i32, g: i32, b: i32) -> bool {
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
                if let Err(e) = device.read_timeout(&mut rx, 200) {
                    record_system_event(
                        "WARN",
                        "HID",
                        format!("HID ACK read timeout in set_key_rgb: {e}"),
                    );
                }
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
            if let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == target_num) {
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
                        if let Err(e) = device.read_timeout(&mut rx, 60) {
                            record_system_event(
                                "WARN",
                                "HID",
                                format!("HID ACK read timeout in set_profile_bg_color(slot {slot_idx}): {e}"),
                            );
                        }
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
        crate::bridge::serialize::get_slot_meta_json_impl(self, slot_number)
    }

    pub fn check_script_syntax(
        &self,
        script: QString,
        allow_abort: bool,
        disable_autorepeat: bool,
    ) -> QString {
        crate::bridge::serialize::check_script_syntax_impl(script, allow_abort, disable_autorepeat)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save_slot_config(
        self: Pin<&mut Self>,
        slot_number: i32,
        label1: QString,
        label2: QString,
        on_press_script: QString,
        on_release_script: QString,
        allow_abort: bool,
        disable_autorepeat: bool,
    ) -> bool {
        self.save_slot_config_with_params(SlotConfigSaveParams {
            slot_number,
            label1: label1.to_string(),
            label2: label2.to_string(),
            on_press_script: on_press_script.to_string(),
            on_release_script: on_release_script.to_string(),
            allow_abort,
            disable_autorepeat,
        })
    }

    pub fn save_slot_config_with_params(
        mut self: Pin<&mut Self>,
        params: SlotConfigSaveParams,
    ) -> bool {
        let SlotConfigSaveParams {
            slot_number,
            label1,
            label2,
            on_press_script: press,
            on_release_script: release,
            allow_abort,
            disable_autorepeat,
        } = params;
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
        let l1: String = take_5_visible(&label1);
        let l2: String = take_5_visible(&label2);

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
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error saving profile: {e}")));
                false
            }
        }
    }

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

        std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
            else {
                return;
            };
            rt.block_on(async move {
                let switcher_state = state.clone();
                let modifier_state = state.clone();
                tokio::spawn(async move {
                    let _ =
                        crate::kwin::auto_switcher::run_kwin_auto_switcher(switcher_state, tx)
                            .await;
                });

                // Background Hybrid Web API Trigger Listener (PGV _GV31 + evdev F13..F24) + Main-Keyboard Modifier Layer Listener
                tokio::spawn(async move {
                    let mut interval =
                        tokio::time::interval(std::time::Duration::from_millis(50));
                    let mut evdev_listener = DuckypadEvdevListener::new();
                    let mut modifier_listener =
                        crate::actions::app_launch::HostKeyboardModifierListener::new();
                    let mut tick_count = 0u32;
                    loop {
                        interval.tick().await;
                        if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                            .load(std::sync::atomic::Ordering::SeqCst)
                        {
                            crate::hardware::storage::invalidate_cached_hid_device();
                            continue;
                        }

                        // 1. Poll evdev F13..F24 sentinel keys + track EV_KEY press/release state
                        for fkey_name in evdev_listener.poll_fkey_triggers() {
                            let _ = web_api_qt_thread.queue(move |mut qobject| {
                                qobject.as_mut().handle_hardware_fkey_web_api_trigger(&fkey_name);
                            });
                        }

                        // 2. Poll Main-Keyboard Modifier Layer Switcher every 50ms
                        let (mod_enabled, mod_paused, mod_rules) =
                            if let Ok(sw) = modifier_state.lock() {
                                (
                                    sw.modifier_switch_enabled,
                                    sw.paused_for_editing,
                                    sw.modifier_rules.clone(),
                                )
                            } else {
                                (false, false, Vec::new())
                            };
                        if let Some(transition) = modifier_listener.poll_modifier_layer(
                            mod_enabled,
                            mod_paused,
                            &mod_rules,
                        ) {
                            let _ = web_api_qt_thread.queue(move |mut qobject| {
                                qobject.as_mut().handle_modifier_layer_transition(transition);
                            });
                        }

                        // 3. Poll silent PGV _GV31 hardware trigger (0x444Bxxxx) every 150ms when active profile uses PGV31
                        tick_count = tick_count.wrapping_add(1);
                        if !tick_count.is_multiple_of(3)
                            || !crate::hardware::storage::ACTIVE_PROFILE_HAS_PGV31_TRIGGER
                                .load(std::sync::atomic::Ordering::SeqCst)
                        {
                            continue;
                        }

                        if let Some((encoded_prof, slot_num, is_release)) =
                            poll_and_clear_pgv31_trigger()
                        {
                            crate::hardware::storage::LAST_EVDEV_KEYDOWN_MS.store(
                                crate::kwin::auto_switcher::current_epoch_ms(),
                                std::sync::atomic::Ordering::SeqCst,
                            );
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
                        tokio::time::interval(std::time::Duration::from_millis(800));
                    let mut last_mounts_hash: u64 = 0;
                    let mut mounts_tick: u32 = 0;
                    let mut last_observed_hw_prof: Option<u8> = None;
                    loop {
                        interval.tick().await;
                        if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                            .load(std::sync::atomic::Ordering::SeqCst)
                        {
                            crate::hardware::storage::invalidate_cached_hid_device();
                            last_observed_hw_prof = None;
                            continue;
                        }
                        let new_connected_count =
                            crate::hardware::storage::count_connected_duckypad_hid_nodes() as i32;
                        let hid_present = crate::hardware::storage::is_duckypad_hid_present();
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
                        for sec_dev in crate::kwin::auto_switcher::get_secondary_devices_snapshot()
                        {
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
                                            let name =
                                                entry.file_name().to_string_lossy().to_string();
                                            if name.starts_with("mmcblk") || name.starts_with("sd")
                                            {
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
                                newly_mounted_sd = try_auto_mount_duckypad_sd_for_device(
                                    selected_ser.as_deref(),
                                    None,
                                    None,
                                );
                            }
                        }

                        let _ = hotplug_qt_thread.queue(move |mut qobject| {
                            if crate::hardware::storage::MSC_SYNC_IN_PROGRESS
                                .load(std::sync::atomic::Ordering::SeqCst)
                            {
                                return;
                            }

                            if *qobject.connected_device_count() != new_connected_count {
                                let _ = qobject.as_mut().refresh_available_devices_state();
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
                            } else if !was_connected && hid_present {
                                qobject.as_mut().refresh_device();
                                let _ = qobject.as_mut().refresh_available_devices_state();
                            } else if was_connected && hid_present {
                                if let Some(hw_prof) = hw_active_prof {
                                    let now_ms = crate::kwin::auto_switcher::current_epoch_ms();
                                    let last_sw_ms = crate::kwin::auto_switcher::LAST_SW_PROFILE_SWITCH_MS
                                        .load(std::sync::atomic::Ordering::SeqCst);
                                    let last_sw_target = crate::kwin::auto_switcher::LAST_SW_PROFILE_TARGET
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
        });
    }

    fn handle_auto_switched_profile(
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

    fn apply_auto_switch_for_current_window(mut self: Pin<&mut Self>) {
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

    fn handle_modifier_layer_transition(
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
                        sw.active_modifier_override = Some((
                            clamped_target,
                            trigger_key.clone(),
                            mode.clone(),
                        ));
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

                let badge_text = format!(
                    "⌨️ {key_lbl} [{mode_lbl}] → #{clamped_target} {prof_name}"
                );
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
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.paused_for_editing = open;
            }
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
        let (enable_exp, rule_exp) = SwitcherState::load_config_from_disk()
            .map(|c| {
                (
                    c.autoswitch_enable_section_expanded,
                    c.autoswitch_rule_editor_expanded,
                )
            })
            .unwrap_or((true, true));
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                sw.autoswitch_enable_section_expanded = enable_exp;
                sw.autoswitch_rule_editor_expanded = rule_exp;
            }
        }
        self.as_mut()
            .set_autoswitch_enable_section_expanded(enable_exp);
        self.as_mut().set_autoswitch_rule_editor_expanded(rule_exp);
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

    pub fn save_modifier_rule(
        mut self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
        trigger_key: QString,
        mode: QString,
        hold_delay_ms: i32,
        oneshot_timeout_sec: i32,
        target_profile: i32,
    ) -> bool {
        let norm_key = normalize_modifier_trigger_key(&trigger_key.to_string());
        let norm_mode = normalize_modifier_mode(&mode.to_string());
        let clamped_delay = hold_delay_ms.clamp(0, 2000);
        let clamped_timeout = oneshot_timeout_sec.clamp(1, 60);
        let max_prof = self.rust().profiles.len().max(1) as i32;
        let target = target_profile.clamp(1, max_prof.max(1)) as u8;

        let new_rule = ModifierLayerRule {
            enabled,
            trigger_key: norm_key.clone(),
            mode: norm_mode.clone(),
            hold_delay_ms: clamped_delay,
            oneshot_timeout_sec: clamped_timeout,
            target_profile: target,
        };

        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if rule_index >= 0 && (rule_index as usize) < sw.modifier_rules.len() {
                    sw.modifier_rules[rule_index as usize] = new_rule;
                } else if let Some(existing) = sw
                    .modifier_rules
                    .iter_mut()
                    .find(|r| r.trigger_key.eq_ignore_ascii_case(&norm_key))
                {
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
        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "Saved Modifier Layer rule: {key_lbl} ({mode_lbl}) → Profile #{target}"
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

    pub fn toggle_modifier_rule(
        mut self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
    ) -> bool {
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

    pub fn copy_slot(mut self: Pin<&mut Self>, slot_number: i32) -> bool {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;

        let summary_label = {
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
            rust.copied_key_slot = Some(slot_data);
            summary
        };

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
                self.as_mut()
                    .set_status_message(QString::from(&format!("📥 Pasted {desc}{suffix}")));
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
        self.as_mut()
            .set_status_message(QString::from(&format!("Cleared Key #{slot_idx}{suffix}")));
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
        self.as_mut()
            .set_status_message(QString::from(&format!("✓ {action_word}{suffix}")));
        true
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
                                let title = QString::from("DuckyDeck Studio Update Available");
                                let body = QString::from(&format!(
                                    "Version {} is available on GitHub (installed: v{}). Open DuckyDeck Studio to install.",
                                    res.latest_version, res.current_version
                                ));
                                qobject::duckydeck_show_tray_notification(&title, &body);
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
        let dev = crate::hardware::storage::open_duckypad_hid()?;
        let pid = crate::hardware::storage::find_duckypad_hidraw_node()
            .map(|(_, p, _)| p)
            .unwrap_or(PID_DUCKYPAD_PRO);
        Ok((dev, pid))
    }

    fn has_permission_denied_hidraw() -> bool {
        let Ok(entries) = fs::read_dir("/sys/class/hidraw") else {
            return false;
        };
        for entry in entries.flatten() {
            if let Ok(uevent) = fs::read_to_string(entry.path().join("device/uevent")) {
                let is_duckypad =
                    uevent.contains("00000483:0000D11D") || uevent.contains("00000483:0000D11C");
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

    pub fn sync_event_log_properties(self: Pin<&mut Self>) {
        crate::bridge::event_log::sync_event_log_properties_impl(self);
    }

    pub fn log_event(self: Pin<&mut Self>, level: QString, category: QString, message: QString) {
        crate::bridge::event_log::log_event_impl(self, level, category, message);
    }

    pub fn record_status_log(self: Pin<&mut Self>, message: QString) {
        crate::bridge::event_log::record_status_log_impl(self, message);
    }

    pub fn refresh_event_log(self: Pin<&mut Self>) -> QString {
        crate::bridge::event_log::refresh_event_log_impl(self)
    }

    pub fn clear_event_log(self: Pin<&mut Self>) {
        crate::bridge::event_log::clear_event_log_impl(self);
    }

    pub fn export_event_log(self: Pin<&mut Self>, destination_path: QString) -> bool {
        crate::bridge::event_log::export_event_log_impl(self, destination_path)
    }

    pub fn format_event_log_text(
        &self,
        category_filter: QString,
        level_filter: QString,
        search_query: QString,
    ) -> QString {
        crate::bridge::event_log::format_event_log_text_impl(
            category_filter,
            level_filter,
            search_query,
        )
    }

    pub fn refresh_color_palettes(mut self: Pin<&mut Self>) {
        let (saved_json, saved_names_json, used_json) = {
            let rust = self.rust();
            let (saved, names) = rust
                .switcher_state
                .lock()
                .map(|sw| (sw.saved_colors.clone(), sw.saved_color_names.clone()))
                .unwrap_or_default();
            let s_json = serde_json::to_string(&saved).unwrap_or_else(|_| "[]".to_string());
            let n_json = serde_json::to_string(&names).unwrap_or_else(|_| "{}".to_string());
            let u_json = DeviceControllerRust::collect_used_colors_json(&rust.profiles);
            (s_json, n_json, u_json)
        };
        if self.saved_colors_json().to_string() != saved_json {
            self.as_mut()
                .set_saved_colors_json(QString::from(&saved_json));
        }
        if self.saved_color_names_json().to_string() != saved_names_json {
            self.as_mut()
                .set_saved_color_names_json(QString::from(&saved_names_json));
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
        if normalized.len() != 7 || !normalized[1..].chars().all(|c| c.is_ascii_hexdigit()) {
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
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving color palette to disk: {e}"),
                        );
                    }
                }
            }
        }
        self.as_mut().refresh_color_palettes();
    }

    pub fn save_saved_color_name(
        mut self: Pin<&mut Self>,
        hex_color: QString,
        name: QString,
    ) -> bool {
        let raw_hex = hex_color.to_string();
        let trimmed = raw_hex.trim().to_ascii_lowercase();
        let target_hex = if trimmed.starts_with('#') {
            trimmed
        } else {
            format!("#{trimmed}")
        };
        let new_name = name.to_string().trim().to_string();
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                if new_name.is_empty() {
                    sw.saved_color_names.remove(&target_hex);
                } else {
                    sw.saved_color_names.insert(target_hex, new_name);
                }
                if let Err(e) = sw.save_to_disk() {
                    record_system_event(
                        "WARN",
                        "CONFIG",
                        format!("Failed saving color names to disk: {e}"),
                    );
                }
            }
        }
        self.as_mut().refresh_color_palettes();
        true
    }

    pub fn remove_palette_color(mut self: Pin<&mut Self>, hex_color: QString) {
        let raw = hex_color.to_string();
        let target = raw.trim().to_ascii_lowercase();
        let target_norm = if target.starts_with('#') {
            target.clone()
        } else {
            format!("#{target}")
        };
        if target.is_empty() {
            return;
        }
        {
            let rust = self.as_mut().rust_mut().get_mut();
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let before = sw.saved_colors.len();
                sw.saved_colors.retain(|c| !c.eq_ignore_ascii_case(&target));
                sw.saved_color_names.remove(&target_norm);
                if sw.saved_colors.len() != before {
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving color palette after removal: {e}"),
                        );
                    }
                }
            }
        }
        self.as_mut().refresh_color_palettes();
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
                    self.as_mut().load_profile_group(QString::from(path));
                } else if let Some(path) = cmd.strip_prefix("import-profile:") {
                    self.as_mut().import_profile_from_path(QString::from(path));
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
            let cache = crate::paths::cache_dir();
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
        crate::bridge::serialize::push_hardware_led_frame_json_impl(self, frame_json)
    }

    pub fn get_profile_properties_json(&self, profile_number: i32) -> QString {
        crate::bridge::serialize::get_profile_properties_json_impl(self, profile_number)
    }

    pub fn save_profile_properties_json(
        self: Pin<&mut Self>,
        profile_number: i32,
        props_json: QString,
    ) -> bool {
        crate::bridge::serialize::save_profile_properties_json_impl(
            self,
            profile_number,
            props_json,
        )
    }

    pub fn oled_label_to_hardware_ascii_qstr(&self, label: QString) -> QString {
        QString::from(&oled_label_to_hardware_ascii(&label.to_string()))
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
                    }
                }
            });
        });

        true
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

    pub fn save_slot_widget_labels(
        mut self: Pin<&mut Self>,
        slot_number: i32,
        widget_label1: QString,
        widget_label2: QString,
    ) -> bool {
        let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
        let active_num = *self.active_profile() as u8;
        let wl1: String = widget_label1.to_string().trim().chars().take(24).collect();
        let wl2: String = widget_label2.to_string().trim().chars().take(24).collect();

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
                    if let Err(e) = prof.save_to_sd_root(&root) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed saving slot widget labels to SD: {e}"),
                        );
                    }
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

    pub fn set_active_profile_oled_rotation(mut self: Pin<&mut Self>, degrees: i32) -> bool {
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
                    if let Err(e) = prof.save_to_sd_root(&root) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed saving OLED rotation to SD: {e}"),
                        );
                    }
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

    pub fn set_active_profile_widget_rotation(mut self: Pin<&mut Self>, degrees: i32) -> bool {
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
                    if let Err(e) = prof.save_to_sd_root(&root) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed saving widget rotation to SD: {e}"),
                        );
                    }
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
        crate::bridge::serialize::list_profile_templates_json_impl()
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

    pub fn delete_profile_template(mut self: Pin<&mut Self>, template_id: QString) -> bool {
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
        let raw_p = profile_name.to_string();
        let raw_t = template_id.to_string();
        let (trimmed, tpl_id) = if (raw_p.starts_with("builtin_") || raw_p.starts_with("user_"))
            && !raw_t.starts_with("builtin_")
            && !raw_t.starts_with("user_")
        {
            (raw_t.trim().to_string(), raw_p)
        } else {
            (raw_p.trim().to_string(), raw_t)
        };
        if tpl_id.trim().is_empty() || tpl_id == "builtin_blank" {
            return self.create_new_profile(QString::from(&trimmed));
        }
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
            if rust.profiles.len() >= 64 {
                return false;
            }
            let mut old_to_new: HashMap<u8, u8> = HashMap::with_capacity(rust.profiles.len());
            for (idx, prof) in rust.profiles.iter_mut().enumerate() {
                let new_num = (idx + 1) as u8;
                old_to_new.insert(prof.number, new_num);
                prof.number = new_num;
            }
            let next_num = (rust.profiles.len() + 1) as u8;
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
                    if let Err(e) = save_profile_order_to_sd(sd_root, &rust.profiles) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed saving profile order to SD: {e}"),
                        );
                    }
                }
            }
            rust.profiles.push(new_prof);
            if let Ok(mut sw) = rust.switcher_state.lock() {
                let mut rules_changed = false;
                let old_fb = sw.default_profile.unwrap_or(1);
                let mapped_fb = old_to_new
                    .get(&old_fb)
                    .copied()
                    .unwrap_or(old_fb)
                    .clamp(1, next_num);
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
                if rules_changed {
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event(
                            "WARN",
                            "CONFIG",
                            format!("Failed saving switcher state after template profile add: {e}"),
                        );
                    }
                }
            }
            if let Some(ref sd_root) = rust.sd_root_path {
                if let Err(e) = save_profile_order_to_sd(sd_root, &rust.profiles) {
                    record_system_event(
                        "WARN",
                        "STORAGE",
                        format!("Failed saving profile order to SD: {e}"),
                    );
                }
            }
            let sn = rust.serial_number.to_string();
            sync_profiles_to_device_cache(&sn, &rust.profiles);
            if saved {
                mark_profiles_sd_synced(&sn, &rust.profiles);
            }
            Some((next_num as i32, saved))
        };

        if let Some((next_num, saved_to_sd)) = res {
            crate::kwin::auto_switcher::LAST_SW_PROFILE_TARGET
                .store(next_num as u8, std::sync::atomic::Ordering::SeqCst);
            crate::kwin::auto_switcher::LAST_SW_PROFILE_SWITCH_MS.store(
                crate::kwin::auto_switcher::current_epoch_ms(),
                std::sync::atomic::Ordering::SeqCst,
            );
            self.as_mut().set_active_profile(next_num);
            self.as_mut().sync_active_profile_state();
            let suffix = if saved_to_sd { " (saved to SD)" } else { "" };
            let msg = format!("✓ Created Profile #{next_num}: {clean_name} from template{suffix}");
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
                if let Err(e) = prof.save_to_sd_root(&root) {
                    record_system_event(
                        "WARN",
                        "STORAGE",
                        format!("Failed saving template-applied profile to SD: {e}"),
                    );
                }
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
            let msg = format!("✓ Applied template to Profile #{} ({})", target_num, name);
            record_system_event("INFO", "PROFILE", &msg);
            self.as_mut().sync_event_log_properties();
            self.as_mut().set_status_message(QString::from(&msg));
            true
        } else {
            false
        }
    }

    pub(crate) fn sync_rest_api_server_from_state(self: Pin<&mut Self>) {
        crate::bridge::rest_routes::sync_rest_api_server_from_state_impl(self);
    }

    pub fn apply_rest_api_config(
        self: Pin<&mut Self>,
        enabled: bool,
        bind_ip: QString,
        port: i32,
        require_auth: bool,
        token: QString,
    ) -> bool {
        crate::bridge::rest_routes::apply_rest_api_config_impl(
            self,
            enabled,
            bind_ip,
            port,
            require_auth,
            token,
        )
    }

    pub fn regenerate_rest_api_token(self: Pin<&mut Self>) -> QString {
        crate::bridge::rest_routes::regenerate_rest_api_token_impl(self)
    }

    pub fn reset_rest_api_defaults(self: Pin<&mut Self>) -> QString {
        crate::bridge::rest_routes::reset_rest_api_defaults_impl(self)
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
        std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Runtime::new() else {
                return;
            };
            let res = rt.block_on(execute_profile_webhook_request(
                &cfg,
                target_num,
                &prof_name,
                target_num,
                &prof_name,
                "test_button",
                &serial,
            ));
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

        std::thread::spawn(move || {
            let Ok(rt) = tokio::runtime::Runtime::new() else {
                return;
            };
            let mut logs: Vec<(bool, String)> = Vec::new();
            if let Some(ref g_cfg) = global_webhook {
                let res = rt.block_on(execute_profile_webhook_request(
                    g_cfg,
                    new_profile,
                    &new_name,
                    previous_profile,
                    &prev_name,
                    &reason_str,
                    &serial,
                ));
                logs.push((
                    res.ok,
                    format!(
                        "Global profile webhook → #{} ({}) [HTTP {}]: {}",
                        new_profile, new_name, res.status_code, res.summary
                    ),
                ));
            }
            if let Some(ref p_cfg) = per_prof_webhook {
                let res = rt.block_on(execute_profile_webhook_request(
                    p_cfg,
                    new_profile,
                    &new_name,
                    previous_profile,
                    &prev_name,
                    &reason_str,
                    &serial,
                ));
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

        let res = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {e}"))?;
            rt.block_on(fetch_and_search_home_assistant_entities(
                &url_str, &tok_str, &q_str, &dom_str,
            ))
        })
        .join()
        .unwrap_or_else(|_| Err("Home Assistant search worker thread panicked".to_string()));

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
        std::thread::spawn(move || {
            let res = tokio::runtime::Runtime::new()
                .map_err(|e| format!("Runtime error: {e}"))
                .and_then(|rt| rt.block_on(fetch_official_firmwares_from_site(&curr_ver)));
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
        std::thread::spawn(move || {
            let res = tokio::runtime::Runtime::new()
                .map_err(|e| format!("Runtime error: {e}"))
                .and_then(|rt| rt.block_on(download_official_firmware_binary(&fname, &durl)));
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
        self.as_mut().set_status_message(QString::from(&format!(
            "Downloading official firmware {fname} from GitHub..."
        )));
        let fname_clone = fname.clone();
        let res = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {e}"))?;
            rt.block_on(download_official_firmware_binary(&fname_clone, &durl))
        })
        .join()
        .unwrap_or_else(|_| Err("Firmware download thread panicked".to_string()));

        match res {
            Ok(local_path) => {
                self.as_mut().set_firmware_fetch_busy(false);
                let path_str = local_path.to_string_lossy().to_string();
                self.as_mut()
                    .install_firmware_to_device(QString::from(&path_str))
            }
            Err(err) => {
                self.as_mut().set_firmware_fetch_busy(false);
                let msg = format!("Failed downloading official firmware {fname}: {err}");
                record_system_event("ERROR", "FIRMWARE", &msg);
                self.as_mut().sync_event_log_properties();
                self.as_mut().set_status_message(QString::from(&msg));
                false
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
                            title_contains: input.auto_switch_title.filter(|t| !t.trim().is_empty()),
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

    pub fn handle_rest_api_request(
        self: Pin<&mut Self>,
        method: &str,
        path: &str,
        query: &str,
        body: &str,
    ) -> (u16, String) {
        crate::bridge::rest_routes::handle_rest_api_request_impl(self, method, path, query, body)
    }
}

static GLOBAL_MACRO_RECORDER: OnceLock<Mutex<Option<GlobalMacroEvdevRecorder>>> = OnceLock::new();

fn global_macro_recorder_store() -> &'static Mutex<Option<GlobalMacroEvdevRecorder>> {
    GLOBAL_MACRO_RECORDER.get_or_init(|| Mutex::new(None))
}
