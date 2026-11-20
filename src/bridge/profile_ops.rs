use crate::bridge::device_controller::{
    is_demo_showcase_env, qobject, record_system_event, DeviceControllerRust, SlotConfigSaveParams,
};
use crate::hardware::storage::{
    delete_user_profile_template, instantiate_profile_from_template, mark_profiles_sd_synced,
    oled_label_to_hardware_ascii, save_profile_as_user_template, save_profile_order_to_sd,
    set_single_key_led_hid, sync_profiles_to_cache, sync_profiles_to_device_cache, DuckyKeySlot,
    DuckyProfile, HID_CMD_SET_LED_SINGLE, HID_REPORT_ID_OUT, MAX_INPUT_SLOTS,
};
use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

impl qobject::DeviceController {
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
}
