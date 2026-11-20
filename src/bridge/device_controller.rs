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
        fn duckydeck_record_main_geometry(x: i32, y: i32, width: i32, height: i32);
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
        #[qproperty(bool, auto_switch_unfocused_hover)]
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
        #[qproperty(bool, ha_search_busy)]
        #[qproperty(QString, ha_search_result_json)]
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
        #[cxx_name = "updateAutoSwitchUnfocusedHover"]
        fn update_auto_switch_unfocused_hover(self: Pin<&mut DeviceController>, enabled: bool);

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
            resource_class: QString,
            title_contains: QString,
            title_is_regex: bool,
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
        #[cxx_name = "startSearchHomeAssistantEntities"]
        fn start_search_home_assistant_entities(
            self: Pin<&mut DeviceController>,
            base_url: QString,
            access_token: QString,
            query: QString,
            domain_filter: QString,
        );

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

use crate::hardware::storage::{
    check_profile_sync_status, compute_unsynced_changes_json, detect_unmounted_duckypad_sd,
    discover_dekunukem_profile_backups, get_sd_card_space_info, get_selected_duckypad_serial,
    inspect_duckypad_sd_root, list_manageable_duckypad_targets, load_all_sd_profiles,
    load_profiles_for_connected_device, mark_profiles_sd_synced, merge_host_only_profile_settings,
    model_name_for_hw_model, onboard_key_count_for_model, profile_cache_dir_for_serial,
    query_connected_duckypad_info, read_dpp_config, read_duckypad_pro_profiles_via_msc,
    run_storage_cmd, seed_web_api_destinations_from_profiles, set_selected_duckypad_serial,
    sync_profile_web_api_triggers, sync_profiles_to_cache, sync_profiles_to_device_cache,
    sync_profiles_to_device_cache_with_meta, try_auto_mount_duckypad_sd_for_device, DuckyKeySlot,
    DuckyProfile, DuckySdDeviceMeta, DP_MODEL_DUCKYPAD_PRO, DP_MODEL_OG_DUCKYPAD, PID_DUCKYPAD_PRO,
};
use crate::kwin::auto_switcher::{
    discover_dekunukem_autoswitcher_configs, is_kde_autostart_enabled, load_rules_bundle_from_dir,
    parse_dekunukem_autoswitcher_file, save_rules_bundle_to_dir, set_kde_autostart_enabled,
    sync_local_desktop_and_icon, AutoSwitchRule, ModifierLayerRule, SwitcherState,
};
use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use hidapi::HidDevice;
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

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
    pub(crate) auto_switch_unfocused_hover: bool,
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
    pub(crate) ha_search_busy: bool,
    pub(crate) ha_search_result_json: QString,
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
        let auto_unfocused_hover = switcher.auto_switch_unfocused_hover;
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
            auto_switch_unfocused_hover: auto_unfocused_hover,
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
            ha_search_busy: false,
            ha_search_result_json: QString::from(""),
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

    pub(crate) fn open_duckypad() -> Result<(HidDevice, u16), String> {
        let dev = crate::hardware::storage::open_duckypad_hid()?;
        let pid = crate::hardware::storage::find_duckypad_hidraw_node()
            .map(|(_, p, _)| p)
            .unwrap_or(PID_DUCKYPAD_PRO);
        Ok((dev, pid))
    }

    pub(crate) fn has_permission_denied_hidraw() -> bool {
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
