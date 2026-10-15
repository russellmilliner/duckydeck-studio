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
        #[qproperty(QString, status_message)]
        #[qproperty(bool, ai_busy)]
        #[qproperty(QString, generated_script_preview)]
        type DeviceController = super::DeviceControllerRust;

        #[qinvokable]
        fn refresh_device(self: Pin<&mut DeviceController>);

        #[qinvokable]
        fn switch_profile(self: Pin<&mut DeviceController>, profile_number: i32) -> bool;

        #[qinvokable]
        fn switch_profile_by_name(self: Pin<&mut DeviceController>, profile_name: QString) -> bool;

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
        fn generate_ai_macro(
            self: Pin<&mut DeviceController>,
            app_name: QString,
            prompt: QString,
        );
    }
}

use core::pin::Pin;
use cxx_qt_lib::QString;
use hidapi::{HidApi, HidDevice};
use std::fs::{self, OpenOptions};
use std::path::Path;

use crate::hardware::storage::{
    DUCKYPAD_VID, HID_CMD_GOTO_PROFILE_BY_NUM, HID_CMD_QUERY_INFO, HID_CMD_SET_LED_SINGLE,
    PID_DUCKYPAD_2020, PID_DUCKYPAD_PRO,
};

pub struct DeviceControllerRust {
    pub(crate) connected: bool,
    pub(crate) udev_rule_missing: bool,
    pub(crate) model_name: QString,
    pub(crate) firmware_version: QString,
    pub(crate) serial_number: QString,
    pub(crate) active_profile: i32,
    pub(crate) status_message: QString,
    pub(crate) ai_busy: bool,
    pub(crate) generated_script_preview: QString,
}

impl Default for DeviceControllerRust {
    fn default() -> Self {
        Self {
            connected: false,
            udev_rule_missing: false,
            model_name: QString::from("Disconnected"),
            firmware_version: QString::from("-"),
            serial_number: QString::from("-"),
            active_profile: 1,
            status_message: QString::from("Click Scan USB HID to connect to duckyPad Pro / 2020"),
            ai_busy: false,
            generated_script_preview: QString::from(
                "// Select a key or use AI Architect to generate duckyScript 3
DEFAULTDELAY 20
OLED_CLEAR
OLED_CPRINT DuckyDeck Ready!
OLED_UPDATE
",
            ),
        }
    }
}

impl qobject::DeviceController {
    pub fn refresh_device(mut self: Pin<&mut Self>) {
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
                            let current_prof = rx[11] as i32;

                            self.as_mut().set_firmware_version(QString::from(&fw));
                            self.as_mut().set_serial_number(QString::from(&serial));
                            self.as_mut().set_active_profile(current_prof);
                            self.as_mut().set_status_message(QString::from(&format!(
                                "Connected to {model} (FW v{fw})"
                            )));
                        }
                    }
                }
            }
            Err(msg) => {
                self.as_mut().set_connected(false);
                self.as_mut().set_model_name(QString::from("Disconnected"));
                self.as_mut().set_status_message(QString::from(&msg));
            }
        }
    }

    pub fn switch_profile(mut self: Pin<&mut Self>, profile_number: i32) -> bool {
        let Ok((device, _)) = Self::open_duckypad() else {
            return false;
        };

        let mut tx = [0u8; 64];
        tx[0] = 0x00;
        tx[1] = 0x02;
        tx[2] = HID_CMD_GOTO_PROFILE_BY_NUM; // 0x01
        tx[3] = profile_number.clamp(1, 64) as u8;

        if device.write(&tx).is_ok() {
            self.as_mut().set_active_profile(profile_number);
            self.as_mut().set_status_message(QString::from(&format!(
                "Switched to Profile #{profile_number}"
            )));
            true
        } else {
            false
        }
    }

    pub fn switch_profile_by_name(mut self: Pin<&mut Self>, profile_name: QString) -> bool {
        use crate::hardware::storage::{goto_profile_by_name, open_duckypad_hid};
        let name = profile_name.to_string();
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

    pub fn set_key_rgb(
        mut self: Pin<&mut Self>,
        key_index: i32,
        r: i32,
        g: i32,
        b: i32,
    ) -> bool {
        let Ok((device, _)) = Self::open_duckypad() else {
            return false;
        };

        let mut tx = [0u8; 64];
        tx[0] = 0x00;
        tx[1] = 0x03;
        tx[2] = HID_CMD_SET_LED_SINGLE;
        tx[3] = key_index.clamp(0, 19) as u8;
        tx[4] = r.clamp(0, 255) as u8;
        tx[5] = g.clamp(0, 255) as u8;
        tx[6] = b.clamp(0, 255) as u8;

        let ok = device.write(&tx).is_ok();
        if ok {
            self.as_mut().set_status_message(QString::from(&format!(
                "Key {} RGB -> ({r}, {g}, {b})",
                key_index + 1
            )));
        }
        ok
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
        use crate::hardware::storage::{find_mounted_duckypad_sd, load_all_sd_profiles};
        let Some(sd_root) = find_mounted_duckypad_sd() else {
            let msg = "No mounted duckyPad microSD found in /run/media or /media";
            self.as_mut().set_status_message(QString::from(msg));
            return QString::from("");
        };

        match load_all_sd_profiles(&sd_root) {
            Ok(profiles) => {
                let msg = format!(
                    "Loaded {} profiles from {}",
                    profiles.len(),
                    sd_root.display()
                );
                self.as_mut().set_status_message(QString::from(&msg));
                QString::from(sd_root.to_string_lossy().as_ref())
            }
            Err(e) => {
                let msg = format!("SD read error: {e}");
                self.as_mut().set_status_message(QString::from(&msg));
                QString::from("")
            }
        }
    }

    pub fn generate_ai_macro(mut self: Pin<&mut Self>, app_name: QString, prompt: QString) {
        self.as_mut().set_ai_busy(true);
        let app = app_name.to_string();
        let user_goal = prompt.to_string();

        let local_shortcuts = crate::ai::shortcut_scraper::scrape_kde_shortcuts(&app);
        let shortcut_comment = if let Some((action, key)) = local_shortcuts.first() {
            format!("// Grounded from ~/.config/kglobalshortcutsrc: {action} = {key}
")
        } else {
            String::new()
        };

        let preview = format!(
            "// AI Generated Macro for: {app}
             // Goal: {user_goal}
             {shortcut_comment}             DEFAULTDELAY 20
             SWC_SET 0 0 255 128
             OLED_CLEAR
             OLED_CURSOR 8 40
             OLED_PRINT {app}
             OLED_CURSOR 8 56
             OLED_PRINT Running Macro...
             OLED_UPDATE
             CONTROL SHIFT m
             DELAY 150
             SWC_RESET 0
             OLED_RESTORE
"
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
