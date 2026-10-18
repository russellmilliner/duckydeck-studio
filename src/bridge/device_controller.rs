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
        fn duckydeck_exit_app(exit_code: i32);
        fn duckydeck_restart_app();
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
        #[qproperty(bool, rest_api_enabled)]
        #[qproperty(QString, rest_api_bind_ip)]
        #[qproperty(i32, rest_api_port)]
        #[qproperty(bool, rest_api_require_auth)]
        #[qproperty(QString, rest_api_token)]
        #[qproperty(QString, rest_api_status)]
        #[qproperty(bool, firmware_fetch_busy)]
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
        #[cxx_name = "isUiTestSuiteMode"]
        fn is_ui_test_suite_mode(self: &DeviceController) -> bool;

        #[qinvokable]
        #[cxx_name = "writeUiTestReport"]
        fn write_ui_test_report(
            self: &DeviceController,
            report_json: QString,
            exit_code: i32,
        );

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
    }

    impl cxx_qt::Threading for DeviceController {}
}

use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use hidapi::{HidApi, HidDevice};
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
    discard_unsynced_change_in_profiles, download_official_firmware_binary, encode_pgv31_trigger,
    execute_app_launch, execute_profile_webhook_request, execute_web_api_request,
    extract_app_launch_config, extract_web_api_config, fetch_and_search_home_assistant_entities,
    fetch_official_firmwares_from_site, find_duckypad_msc_block_device, find_mounted_duckypad_sd,
    flash_slot_led_feedback, get_sd_card_space_info, 
    instantiate_profile_from_template, 
    list_available_duckypad_pro_firmwares, load_all_sd_profiles, load_home_assistant_config,
    load_profiles_for_connected_device, load_web_api_destinations, mark_profiles_sd_synced,
    merge_host_only_profile_settings, mount_duckypad_sd_only, oled_label_to_hardware_ascii,
    poll_and_clear_pgv31_trigger, poll_hardware_active_profile, profile_cache_dir_for_serial,
    query_connected_duckypad_info, read_dpp_config, read_duckypad_pro_profiles_via_msc,
    read_duckypad_pro_profiles_via_msc_with_progress, reboot_duckypad, save_home_assistant_config,
    save_profile_as_user_template, save_profile_order_to_sd, scan_installed_desktop_apps,
    seed_web_api_destinations_from_profiles, set_single_key_led_hid,
    stage_duckypad_pro_firmware_via_msc_with_progress, sync_profile_web_api_triggers,
    sync_profiles_to_cache, sync_profiles_to_device_cache, unmount_mounted_duckypad_sd,
    upsert_web_api_destination, write_duckypad_pro_profiles_via_msc_with_progress,
    write_profiles_to_sd_root, AppLaunchConfig, DuckyKeySlot, DuckyProfile, DuckypadEvdevListener,
    GlobalMacroEvdevRecorder, ProfileWebhookConfig, WebApiConfig, DUCKYPAD_VID,
    HID_CMD_SET_LED_SINGLE, HID_REPORT_ID_OUT, MAX_INPUT_SLOTS, PID_DUCKYPAD_2020,
    PID_DUCKYPAD_PRO,
};
use crate::kwin::auto_switcher::{
    is_kde_autostart_enabled, is_web_browser_class,
    set_kde_autostart_enabled, strip_browser_title_suffix, sync_local_desktop_and_icon,
    AutoSwitchRule, SwitcherState,
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
    pub(crate) rest_api_enabled: bool,
    pub(crate) rest_api_bind_ip: QString,
    pub(crate) rest_api_port: i32,
    pub(crate) rest_api_require_auth: bool,
    pub(crate) rest_api_token: QString,
    pub(crate) rest_api_status: QString,
    pub(crate) firmware_fetch_busy: bool,
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
            if let Ok(mut loaded) = load_all_sd_profiles(sd_dir) {
                if !loaded.is_empty() {
                    if let Some(ref info) = connected_info {
                        if let Some(cached) =
                            load_profiles_for_connected_device(info.pid, &info.serial)
                        {
                            merge_host_only_profile_settings(&mut loaded, &cached);
                        }
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
        let used_colors_json = Self::collect_used_colors_json(&profiles);
        let autostart = is_kde_autostart_enabled();
        if autostart {
            if let Err(e) = set_kde_autostart_enabled(true) {
                record_system_event("WARN", "SYSTEM", format!("Failed refreshing KDE autostart: {e}"));
            }
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
            rest_api_enabled: rest_enabled,
            rest_api_bind_ip: QString::from(&rest_ip),
            rest_api_port: rest_port,
            rest_api_require_auth: rest_auth,
            rest_api_token: QString::from(&rest_token),
            rest_api_status: QString::from(&rest_status),
            firmware_fetch_busy: false,
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

    pub fn serialize_profile_slots(profile: &DuckyProfile) -> String {
        crate::bridge::serialize::serialize_profile_slots(profile)
    }
}

impl qobject::DeviceController {
    pub(crate) fn sync_tray_state(mut self: Pin<&mut Self>) {
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

        if !is_ui_test_suite_env() {
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
        let oled_snapshot = json!({
            "connected": connected,
            "modelName": model,
            "firmwareVersion": fw,
            "serialNumber": sn,
            "activeProfile": prof_num,
            "activeProfileName": prof_name,
            "activeProfileBgColor": bg_color_str,
            "oledRotation": oled_rot,
            "widgetRotation": widget_rot,
            "effectiveWidgetRotation": eff_widget_rot,
            "totalProfiles": total_profiles,
            "sdSpaceSummary": sd_space.summary_text,
            "slots": slots_val,
        });
        if let Ok(test_state_path) = std::env::var("DUCKYDECK_UI_TEST_OLED_STATE_PATH") {
            if !test_state_path.trim().is_empty() {
                if let Ok(serialized) = serde_json::to_string(&oled_snapshot) {
                    if let Err(e) = fs::write(test_state_path.trim(), serialized) {
                        record_system_event(
                            "WARN",
                            "STORAGE",
                            format!("Failed writing test OLED state {}: {e}", test_state_path.trim()),
                        );
                    }
                }
            }
        } else if !is_demo_showcase_env() {
            let state_dir = crate::paths::data_dir();
            if let Err(e) = fs::create_dir_all(&state_dir) {
                record_system_event(
                    "WARN",
                    "STORAGE",
                    format!("Failed creating state dir {}: {e}", state_dir.display()),
                );
            }
            if let Ok(serialized) = serde_json::to_string(&oled_snapshot) {
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

        // If a duckyPad SD card is mounted, load directly from it
        if self.rust().sd_root_path.is_none() {
            if let Some(sd_root) = find_mounted_duckypad_sd() {
                self.as_mut()
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
                if !is_ui_test_suite_env() && self.latest_firmware_version().to_string() == "-" {
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
        self.as_mut().sync_active_profile_state();

        if !is_demo_showcase_env()
            && *self.connected()
            && !crate::kwin::auto_switcher::send_hid_switch_profile(clamped as u8)
        {
            record_system_event(
                "WARN",
                "HID",
                format!("Failed switching hardware to profile #{clamped}"),
            );
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
            self.as_mut().sync_active_profile_state();
            self.as_mut().dispatch_profile_change_webhooks_if_changed(
                prev_prof,
                num.clamp(1, 64) as u8,
                "switch_by_name",
            );
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
                                    format!("Failed removing deleted profile dir {}: {e}", dir_path.display()),
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
            self.as_mut().set_status_message(QString::from(
                "Cannot delete the last remaining profile",
            ));
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
                if let Err(e) = device.read_timeout(&mut rx, 200) {
                    record_system_event("WARN", "HID", format!("HID ACK read timeout in set_key_rgb: {e}"));
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

    pub fn apply_area_text_scale(
        mut self: Pin<&mut Self>,
        _area: QString,
        scale_percent: i32,
    ) {
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
        self.as_mut().set_status_message(QString::from(
            "Application zoom reset to 100% (saved)",
        ));
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

    pub fn send_key_to_active_window(
        &self,
        qt_key: i32,
        qt_modifiers: i32,
        text: QString,
    ) -> bool {
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
                record_system_event("WARN", "HID", format!("HID ACK read timeout in sync_rtc_clock: {e}"));
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
                record_system_event("WARN", "HID", format!("HID ACK read timeout in set_sleep_state: {e}"));
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
        crate::bridge::serialize::get_slot_meta_json_impl(self, slot_number)
    }

    pub fn check_script_syntax(
        &self,
        script: QString,
        allow_abort: bool,
        disable_autorepeat: bool,
    ) -> QString {
        crate::bridge::serialize::check_script_syntax_impl(
            script,
            allow_abort,
            disable_autorepeat,
        )
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
                                let prev_prof = (*qobject.active_profile()).clamp(1, 64) as u8;
                                if prev_prof != prof {
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
                                    qobject.as_mut().dispatch_profile_change_webhooks_if_changed(
                                        prev_prof,
                                        prof,
                                        "kwin_auto_switch",
                                    );
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
            let prev_prof = (*self.active_profile()).clamp(1, 64) as u8;
            if prev_prof != prof {
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
                self.as_mut().dispatch_profile_change_webhooks_if_changed(
                    prev_prof,
                    prof,
                    "kwin_auto_switch",
                );
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
                if let Err(e) = sw.save_to_disk() {
                    record_system_event("WARN", "CONFIG", format!("Failed saving auto-switch config: {e}"));
                }
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
                if let Err(e) = sw.save_to_disk() {
                    record_system_event("WARN", "CONFIG", format!("Failed saving auto-switch timings: {e}"));
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
                if let Err(e) = sw.save_to_disk() {
                    record_system_event("WARN", "CONFIG", format!("Failed saving auto-switch rule: {e}"));
                }
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
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event("WARN", "CONFIG", format!("Failed saving after removing rule: {e}"));
                    }
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
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event("WARN", "CONFIG", format!("Failed saving after toggling rule: {e}"));
                    }
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
                    if let Err(e) = sw.save_to_disk() {
                        record_system_event("WARN", "CONFIG", format!("Failed saving after reordering rule: {e}"));
                    }
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
                    record_system_event("WARN", "CONFIG", format!("Failed saving tray icon mode: {e}"));
                }
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
                if let Err(e) = sw.save_to_disk() {
                    record_system_event("WARN", "CONFIG", format!("Failed saving app theme mode: {e}"));
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
            if let Err(e) = std::process::Command::new("xdg-open").arg(&url).spawn() {
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
            if let Err(e) = std::process::Command::new("xdg-open").arg(&target).spawn() {
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

    pub fn sync_event_log_properties(self: Pin<&mut Self>) {
        crate::bridge::event_log::sync_event_log_properties_impl(self);
    }

    pub fn log_event(
        self: Pin<&mut Self>,
        level: QString,
        category: QString,
        message: QString,
    ) {
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
        let system_candidates = [
            Path::new("/usr/share/doc/duckydeck-studio/user-guide.html"),
            Path::new("/app/share/doc/duckydeck-studio/user-guide.html"),
        ];
        let target_path = if let Some(found) = system_candidates.iter().find(|p| p.is_file()) {
            found.to_path_buf()
        } else {
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
                        format!("Failed writing embedded user guide to {}: {e}", extracted.display()),
                    );
                }
            }
            extracted
        };

        if !is_ui_test_suite_env() {
            if let Err(e) = std::process::Command::new("xdg-open")
                .arg(&target_path)
                .stdin( std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
            {
                record_system_event(
                    "WARN",
                    "SYSTEM",
                    format!("Failed launching xdg-open for {}: {e}", target_path.display()),
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
            self.as_mut().load_sd_directory(QString::from(&mounted_path));
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
            format!(
                "✓ Saved Global Profile Change Webhook ({})",
                cfg.url.trim()
            )
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



