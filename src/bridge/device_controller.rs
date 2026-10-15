#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
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
        #[qproperty(QString, profiles_json)]
        #[qproperty(QString, active_profile_slots_json)]
        #[qproperty(QString, status_message)]
        #[qproperty(bool, ai_busy)]
        #[qproperty(QString, generated_script_preview)]
        #[qproperty(bool, auto_switch_enabled)]
        #[qproperty(i32, default_fallback_profile)]
        #[qproperty(QString, active_window_class)]
        #[qproperty(QString, active_window_title)]
        #[qproperty(bool, window_picker_armed)]
        #[qproperty(QString, last_picked_window_class)]
        #[qproperty(QString, last_picked_window_title)]
        #[qproperty(QString, auto_switch_rules_json)]
        type DeviceController = super::DeviceControllerRust;

        #[qinvokable]
        fn refresh_device(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn switch_profile(self: Pin<&mut DeviceController>, profile_number: i32) -> bool;

        #[qinvokable]
        fn select_profile(self: Pin<&mut DeviceController>, profile_number: i32) -> bool;

        #[qinvokable]
        fn switch_profile_by_name(self: Pin<&mut DeviceController>, profile_name: QString) -> bool;

        #[qinvokable]
        fn create_new_profile(self: Pin<&mut DeviceController>, profile_name: QString) -> bool;

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
        fn remove_auto_switch_rule(self: Pin<&mut DeviceController>, rule_index: i32) -> bool;

        #[qinvokable]
        fn toggle_auto_switch_rule(
            self: Pin<&mut DeviceController>,
            rule_index: i32,
            enabled: bool,
        ) -> bool;

        #[qinvokable]
        fn generate_ai_macro(
            self: Pin<&mut DeviceController>,
            app_name: QString,
            prompt: QString,
        );
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
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::hardware::storage::{
    find_mounted_duckypad_sd, load_all_sd_profiles, read_dpp_config, save_profile_order_to_sd,
    DuckyKeySlot, DuckyProfile, DUCKYPAD_VID, HID_CMD_GOTO_PROFILE_BY_NUM, HID_CMD_QUERY_INFO,
    HID_CMD_SET_LED_SINGLE, MAX_INPUT_SLOTS, PID_DUCKYPAD_2020, PID_DUCKYPAD_PRO,
};
use crate::kwin::auto_switcher::{AutoSwitchRule, SwitcherState};

pub struct DeviceControllerRust {
    pub(crate) connected: bool,
    pub(crate) udev_rule_missing: bool,
    pub(crate) model_name: QString,
    pub(crate) firmware_version: QString,
    pub(crate) serial_number: QString,
    pub(crate) active_profile: i32,
    pub(crate) active_profile_name: QString,
    pub(crate) sd_mount_path: QString,
    pub(crate) profiles_json: QString,
    pub(crate) active_profile_slots_json: QString,
    pub(crate) status_message: QString,
    pub(crate) ai_busy: bool,
    pub(crate) generated_script_preview: QString,
    pub(crate) auto_switch_enabled: bool,
    pub(crate) default_fallback_profile: i32,
    pub(crate) active_window_class: QString,
    pub(crate) active_window_title: QString,
    pub(crate) window_picker_armed: bool,
    pub(crate) last_picked_window_class: QString,
    pub(crate) last_picked_window_title: QString,
    pub(crate) auto_switch_rules_json: QString,
    pub(crate) sd_root_path: Option<PathBuf>,
    pub(crate) profiles: Vec<DuckyProfile>,
    pub(crate) switcher_state: Arc<Mutex<SwitcherState>>,
    pub(crate) kwin_listener_started: bool,
    pub(crate) picker_bind_to_active_profile: bool,
}

impl Default for DeviceControllerRust {
    fn default() -> Self {
        let default_profile = Self::create_default_demo_profile();
        let profiles = vec![default_profile];
        let switcher = SwitcherState::default();
        let auto_enabled = switcher.enabled;
        let fallback_prof = switcher.default_profile.unwrap_or(0) as i32;
        let rules_json = Self::serialize_rules(&switcher.rules, &profiles);
        let profiles_json = Self::serialize_profiles_summary(&profiles, &switcher.rules);
        let slots_json = Self::serialize_profile_slots(&profiles[0]);

        Self {
            connected: false,
            udev_rule_missing: false,
            model_name: QString::from("Disconnected"),
            firmware_version: QString::from("-"),
            serial_number: QString::from("-"),
            active_profile: 1,
            active_profile_name: QString::from("Blender Sculpt"),
            sd_mount_path: QString::from(""),
            profiles_json: QString::from(&profiles_json),
            active_profile_slots_json: QString::from(&slots_json),
            status_message: QString::from(
                "Click Scan USB HID or Load SD Card to configure duckyPad Pro / 2020",
            ),
            ai_busy: false,
            generated_script_preview: QString::from(
                "// Slot 1: Front View\nDEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Front View\nOLED_UPDATE\nKP_1\n",
            ),
            auto_switch_enabled: auto_enabled,
            default_fallback_profile: fallback_prof,
            active_window_class: QString::from("Listening for KWin..."),
            active_window_title: QString::from(""),
            window_picker_armed: false,
            last_picked_window_class: QString::from(""),
            last_picked_window_title: QString::from(""),
            auto_switch_rules_json: QString::from(&rules_json),
            sd_root_path: None,
            profiles,
            switcher_state: Arc::new(Mutex::new(switcher)),
            kwin_listener_started: false,
            picker_bind_to_active_profile: false,
        }
    }
}

impl DeviceControllerRust {
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

    pub fn serialize_profiles_summary(
        profiles: &[DuckyProfile],
        rules: &[AutoSwitchRule],
    ) -> String {
        let items: Vec<_> = profiles
            .iter()
            .map(|p| {
                let bound_class = rules
                    .iter()
                    .find(|r| r.enabled && r.target_profile == p.number)
                    .map(|r| r.resource_class.as_str())
                    .unwrap_or("");
                json!({
                    "number": p.number,
                    "name": p.name,
                    "folder": p.folder_name(),
                    "bgColor": p.bg_color,
                    "slotCount": p.slots.len(),
                    "boundClass": bound_class,
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
                json!({
                    "index": idx,
                    "enabled": r.enabled,
                    "resourceClass": r.resource_class,
                    "titleContains": r.title_contains.as_deref().unwrap_or(""),
                    "targetProfile": r.target_profile,
                    "targetProfileName": prof_name,
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
                    || !slot.label_line2.is_empty();
                let default_rgb = if !has_script && profile.dim_unused_keys {
                    [20, 24, 32]
                } else {
                    profile.bg_color
                };
                let rgb = slot.custom_rgb.unwrap_or(default_rgb);
                let has_rel = !slot.on_release_script.trim().is_empty();
                let has_loop = slot.on_press_script.contains("WHILE")
                    || slot.on_press_script.contains("LOOP");

                items.push(json!({
                    "slot": slot_idx,
                    "l1": slot.label_line1,
                    "l2": slot.label_line2,
                    "r": rgb[0],
                    "g": rgb[1],
                    "b": rgb[2],
                    "hasCustomRgb": slot.custom_rgb.is_some(),
                    "hasRel": has_rel,
                    "hasLoop": has_loop,
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
                    "r": rgb[0],
                    "g": rgb[1],
                    "b": rgb[2],
                    "hasCustomRgb": false,
                    "hasRel": false,
                    "hasLoop": false,
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
    fn sync_active_profile_state(mut self: Pin<&mut Self>) {
        let active_num = *self.active_profile() as u8;
        let (prof_name, profiles_json, slots_json, rules_json) = {
            let rust = self.rust();
            let rules_vec = rust
                .switcher_state
                .lock()
                .map(|s| s.rules.clone())
                .unwrap_or_default();
            let profiles_json =
                DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_vec);
            let rules_json = DeviceControllerRust::serialize_rules(&rules_vec, &rust.profiles);
            if let Some(prof) = rust
                .profiles
                .iter()
                .find(|p| p.number == active_num)
                .or_else(|| rust.profiles.first())
            {
                (
                    prof.name.clone(),
                    profiles_json,
                    DeviceControllerRust::serialize_profile_slots(prof),
                    rules_json,
                )
            } else {
                (
                    "Default".to_string(),
                    profiles_json,
                    "[]".to_string(),
                    rules_json,
                )
            }
        };

        self.as_mut()
            .set_active_profile_name(QString::from(&prof_name));
        self.as_mut()
            .set_profiles_json(QString::from(&profiles_json));
        self.as_mut()
            .set_active_profile_slots_json(QString::from(&slots_json));
        self.as_mut()
            .set_auto_switch_rules_json(QString::from(&rules_json));
    }

    pub fn refresh_device(mut self: Pin<&mut Self>) {
        // Start the KWin Wayland Auto-Switcher D-Bus listener if not started yet
        if !self.rust().kwin_listener_started {
            self.as_mut().start_kwin_listener();
        }

        // Also auto-scan for a mounted duckyPad SD card if none is loaded yet
        if self.rust().sd_root_path.is_none() {
            if let Some(sd_root) = find_mounted_duckypad_sd() {
                let _ = self
                    .as_mut()
                    .load_sd_directory(QString::from(sd_root.to_string_lossy().as_ref()));
            }
        }

        if Self::has_permission_denied_hidraw() {
            self.as_mut().set_connected(false);
            self.as_mut().set_udev_rule_missing(true);
            self.as_mut()
                .set_status_message(QString::from("USB udev permissions required"));
            return;
        }
        self.as_mut().set_udev_rule_missing(false);

        match Self::open_duckypad() {
            Ok((device, pid)) => {
                let model = if pid == PID_DUCKYPAD_PRO {
                    "duckyPad Pro"
                } else {
                    "duckyPad (2020)"
                };
                self.as_mut().set_model_name(QString::from(model));
                self.as_mut().set_connected(true);

                let mut tx = [0u8; 64];
                tx[0] = 0x00;
                tx[1] = 0x01;
                tx[2] = HID_CMD_QUERY_INFO;

                if device.write(&tx).is_ok() {
                    let mut rx = [0u8; 64];
                    if let Ok(len) = device.read_timeout(&mut rx, 500) {
                        if len >= 12 {
                            let fw = format!("{}.{}.{}", rx[3], rx[4], rx[5]);
                            let serial =
                                format!("{:02X}{:02X}{:02X}{:02X}", rx[7], rx[8], rx[9], rx[10]);
                            let current_prof = (rx[11] as i32).max(1);

                            self.as_mut().set_firmware_version(QString::from(&fw));
                            self.as_mut().set_serial_number(QString::from(&serial));
                            self.as_mut().set_active_profile(current_prof);
                            self.as_mut().sync_active_profile_state();
                            self.as_mut().set_status_message(QString::from(&format!(
                                "Connected to {model} (FW v{fw})"
                            )));
                        }
                    }
                }
            }
            Err(msg) => {
                self.as_mut().set_connected(false);
                if self.rust().sd_root_path.is_none() {
                    self.as_mut().set_model_name(QString::from("Disconnected"));
                    self.as_mut().set_status_message(QString::from(&msg));
                }
            }
        }
    }

    pub fn select_profile(mut self: Pin<&mut Self>, profile_number: i32) -> bool {
        let clamped = profile_number.clamp(1, 64);
        self.as_mut().set_active_profile(clamped);
        self.as_mut().sync_active_profile_state();

        // Also switch on hardware if connected over USB HID
        if let Ok((device, _)) = Self::open_duckypad() {
            let mut tx = [0u8; 64];
            tx[0] = 0x00;
            tx[1] = 0x02;
            tx[2] = HID_CMD_GOTO_PROFILE_BY_NUM;
            tx[3] = clamped as u8;
            let _ = device.write(&tx);
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
                .unwrap_or(false);
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
            if let Some(prof) = rust.profiles.get_mut(prof_idx) {
                prof.ensure_slot(slot_idx).custom_rgb = Some(rgb);
                if let Some(root) = sd_root {
                    prof.save_to_sd_root(&root).is_ok()
                } else {
                    false
                }
            } else {
                false
            }
        };

        self.as_mut().sync_active_profile_state();

        let mut hid_ok = false;
        if let Ok((device, _)) = Self::open_duckypad() {
            let mut tx = [0u8; 64];
            tx[0] = 0x00;
            tx[1] = 0x03;
            tx[2] = HID_CMD_SET_LED_SINGLE;
            tx[3] = key_index.clamp(0, 19) as u8;
            tx[4] = rgb[0];
            tx[5] = rgb[1];
            tx[6] = rgb[2];
            hid_ok = device.write(&tx).is_ok();
        }

        let suffix = if saved_to_sd { " (saved to SD)" } else { "" };
        self.as_mut().set_status_message(QString::from(&format!(
            "Key {slot_idx} RGB -> ({}, {}, {}){suffix}",
            rgb[0], rgb[1], rgb[2]
        )));
        hid_ok || saved_to_sd
    }

    pub fn sync_rtc_clock(mut self: Pin<&mut Self>) -> bool {
        use chrono::{Datelike, Local, Timelike};
        let Ok((device, _)) = Self::open_duckypad() else {
            return false;
        };

        let now = Local::now();
        let mut tx = [0u8; 64];
        tx[0] = 0x00;
        tx[1] = 0x04;
        tx[2] = 0x1A; // HID RTC Sync Command
        let year = now.year() as u16;
        tx[3] = (year & 0xFF) as u8;
        tx[4] = (year >> 8) as u8;
        tx[5] = now.month() as u8;
        tx[6] = now.day() as u8;
        tx[7] = now.weekday().num_days_from_monday() as u8;
        tx[8] = now.hour() as u8;
        tx[9] = now.minute() as u8;
        tx[10] = now.second() as u8;

        let ok = device.write(&tx).is_ok();
        if ok {
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
        tx[0] = 0x00;
        tx[1] = 0x05;
        tx[2] = if sleep { 0x15 } else { 0x16 };
        device.write(&tx).is_ok()
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
        let Some(sd_root) = find_mounted_duckypad_sd() else {
            let msg = "No mounted duckyPad microSD found in /run/media or /media — use 'Open SD Folder' to choose a directory";
            self.as_mut().set_status_message(QString::from(msg));
            return QString::from("");
        };

        let path_str = sd_root.to_string_lossy().to_string();
        if self.as_mut().load_sd_directory(QString::from(&path_str)) {
            QString::from(&path_str)
        } else {
            QString::from("")
        }
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

                {
                    let rust = self.as_mut().rust_mut().get_mut();
                    if let Ok(mut sw) = rust.switcher_state.lock() {
                        sw.align_rules_with_profiles(&profiles);
                    }
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
                let meta = json!({
                    "slot": slot_idx,
                    "l1": slot.label_line1,
                    "l2": slot.label_line2,
                    "r": rgb[0],
                    "g": rgb[1],
                    "b": rgb[2],
                    "hasCustomRgb": slot.custom_rgb.is_some(),
                    "allowAbort": slot.allow_abort,
                    "disableAutorepeat": slot.disable_autorepeat,
                    "hasRelease": !slot.on_release_script.trim().is_empty(),
                });
                return QString::from(&meta.to_string());
            } else {
                let meta = json!({
                    "slot": slot_idx,
                    "l1": "",
                    "l2": "",
                    "r": prof.bg_color[0],
                    "g": prof.bg_color[1],
                    "b": prof.bg_color[2],
                    "hasCustomRgb": false,
                    "allowAbort": false,
                    "disableAutorepeat": false,
                    "hasRelease": false,
                });
                return QString::from(&meta.to_string());
            }
        }
        QString::from("{}")
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

        let l1 = label1.to_string();
        let l2 = label2.to_string();
        let press = on_press_script.to_string();
        let release = on_release_script.to_string();

        let save_result: Result<Option<PathBuf>, String> = {
            let rust = self.as_mut().rust_mut().get_mut();
            let sd_root = rust.sd_root_path.clone();
            let prof_idx = rust
                .profiles
                .iter()
                .position(|p| p.number == active_num)
                .unwrap_or(0);
            if let Some(prof) = rust.profiles.get_mut(prof_idx) {
                let entry = prof.ensure_slot(slot_idx);
                entry.label_line1 = l1;
                entry.label_line2 = l2;
                entry.on_press_script = press;
                entry.on_release_script = release;
                entry.allow_abort = allow_abort;
                entry.disable_autorepeat = disable_autorepeat;

                if let Some(root) = sd_root {
                    prof.save_to_sd_root(&root)
                        .map(Some)
                        .map_err(|e| format!("Failed saving to SD card: {e}"))
                } else {
                    Ok(None)
                }
            } else {
                Err("No active profile loaded".to_string())
            }
        };

        self.as_mut().sync_active_profile_state();

        match save_result {
            Ok(Some(profile_dir)) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Saved key{slot_idx}.txt & config.txt to {}",
                    profile_dir.display()
                )));
                true
            }
            Ok(None) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Updated Slot {slot_idx} in memory (load an SD card folder to persist to disk)"
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
            prof.save_to_sd_root(sd_root)
        };

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

                while let Some(evt) = rx.recv().await {
                    let _ = qt_thread.queue(move |mut qobject| {
                        qobject
                            .as_mut()
                            .set_active_window_class(QString::from(&evt.resource_class));
                        qobject
                            .as_mut()
                            .set_active_window_title(QString::from(&evt.caption));

                        if evt.picked {
                            qobject.as_mut().set_window_picker_armed(false);
                            qobject
                                .as_mut()
                                .set_last_picked_window_class(QString::from(&evt.resource_class));
                            qobject
                                .as_mut()
                                .set_last_picked_window_title(QString::from(&evt.caption));

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
                                    "Picked KWin window: '{}' ({})",
                                    evt.resource_class, evt.caption
                                )));
                            }
                            return;
                        }

                        if let Some(prof) = evt.matched_profile {
                            if *qobject.active_profile() != prof as i32 {
                                qobject.as_mut().set_active_profile(prof as i32);
                                qobject.as_mut().sync_active_profile_state();
                            }
                        }
                    });
                }
            });
        });
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
        self.as_mut().sync_active_profile_state();
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
        self.as_mut().set_window_picker_armed(true);
        self.as_mut().set_status_message(QString::from(
            "🎯 Window Picker Armed — Click any open KDE Plasma 6 window with the crosshair cursor",
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
                            .set_last_picked_window_title(QString::from(&evt.caption));

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
                                "Picked KWin window: '{}' ({})",
                                evt.resource_class, evt.caption
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
        mut self: Pin<&mut Self>,
        rule_index: i32,
        enabled: bool,
        resource_class: QString,
        title_contains: QString,
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
        let target = target_profile.clamp(1, 64) as u8;

        let new_rule = AutoSwitchRule {
            enabled,
            resource_class: class_str.clone(),
            title_contains: title_opt,
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

        self.as_mut().sync_active_profile_state();
        self.as_mut().set_status_message(QString::from(&format!(
            "Saved KWin Auto-Switch rule: '{class_str}' → Profile #{target}"
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
            self.as_mut().sync_active_profile_state();
            self.as_mut()
                .set_status_message(QString::from("Removed KWin Auto-Switch rule"));
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
            self.as_mut().sync_active_profile_state();
        }
        updated
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
}
