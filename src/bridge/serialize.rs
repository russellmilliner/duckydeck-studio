use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use crate::bridge::device_controller::{qobject, DeviceControllerRust};
use crate::bridge::event_log::record_system_event;
use crate::hardware::storage::{
    has_app_launch_config, has_web_api_config, list_all_profile_templates, load_all_sd_profiles,
    mark_profiles_sd_synced, save_profile_order_to_sd, set_batch_key_leds_hid,
    sync_profiles_to_cache, sync_profiles_to_device_cache, DuckyProfile, ProfileWebhookConfig,
    MAX_INPUT_SLOTS,
};
use crate::kwin::auto_switcher::{resolve_rule_icon, AutoSwitchRule};

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

pub fn serialize_profiles_summary(profiles: &[DuckyProfile], rules: &[AutoSwitchRule]) -> String {
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
            let raw_title = r.title_contains.as_deref().unwrap_or("");
            json!({
                "index": idx,
                "enabled": r.enabled,
                "resourceClass": r.resource_class,
                "titleContains": raw_title,
                "titleIsRegex": r.title_is_regex,
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
            let has_loop =
                slot.on_press_script.contains("WHILE") || slot.on_press_script.contains("LOOP");
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

pub fn get_slot_meta_json_impl(
    controller: &qobject::DeviceController,
    slot_number: i32,
) -> QString {
    let self_ref = controller;
    let slot_idx = slot_number.clamp(1, MAX_INPUT_SLOTS as i32) as usize;
    let active_num = *self_ref.active_profile() as u8;
    let rust = self_ref.rust();

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

pub fn check_script_syntax_impl(
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
    let compiled =
        crate::vm::compiler::compile_duckyscript_to_dsb(&text, allow_abort, disable_autorepeat);
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

pub fn export_profile_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
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
        let rust = controller.as_mut().rust_mut().get_mut();
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
        controller
            .as_mut()
            .set_status_message(QString::from("No active profile available to export"));
        return false;
    };

    let bundle = ProfileExportBundle {
        version: 1,
        profile: profile.clone(),
        auto_switch_rules: matching_rules,
    };

    let Ok(json_str) = serde_json::to_string_pretty(&bundle) else {
        controller
            .as_mut()
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
                controller
                    .as_mut()
                    .set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {} and {}",
                        profile.name,
                        folder_path.display(),
                        json_path.display()
                    )));
                true
            }
            (Ok(folder_path), Err(_)) => {
                controller
                    .as_mut()
                    .set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {}",
                        profile.name,
                        folder_path.display()
                    )));
                true
            }
            (Err(_), Ok(())) => {
                controller
                    .as_mut()
                    .set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {}",
                        profile.name,
                        json_path.display()
                    )));
                true
            }
            (Err(e), Err(_)) => {
                controller
                    .as_mut()
                    .set_status_message(QString::from(&format!("Failed exporting profile: {e}")));
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
            if let Err(e) = fs::create_dir_all(parent) {
                record_system_event(
                    "WARN",
                    "STORAGE",
                    format!(
                        "Failed creating export parent dir {}: {e}",
                        parent.display()
                    ),
                );
            }
        }
        match fs::write(&target_file, &json_str) {
            Ok(()) => {
                controller
                    .as_mut()
                    .set_status_message(QString::from(&format!(
                        "Exported Profile '{}' to {}",
                        profile.name,
                        target_file.display()
                    )));
                true
            }
            Err(e) => {
                controller
                    .as_mut()
                    .set_status_message(QString::from(&format!(
                        "Failed writing {}: {e}",
                        target_file.display()
                    )));
                false
            }
        }
    }
}

pub fn import_profile_from_path_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
    source_path: QString,
) -> bool {
    let raw_str = source_path.to_string();
    let trimmed = raw_str.trim().trim_start_matches("file://");
    if trimmed.is_empty() {
        return false;
    }
    let src = PathBuf::from(trimmed);
    if !src.exists() {
        controller
            .as_mut()
            .set_status_message(QString::from(&format!(
                "Import source not found: {}",
                src.display()
            )));
        return false;
    }

    let load_from_json_file = |p: &Path| -> Result<(DuckyProfile, Vec<AutoSwitchRule>), String> {
        let raw =
            fs::read_to_string(p).map_err(|e| format!("Failed reading {}: {e}", p.display()))?;
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
            controller.as_mut().set_status_message(QString::from(&err));
            return false;
        }
    };

    let (new_num, final_name, sd_msg, profiles_json, rules_json) = {
        let rust = controller.as_mut().rust_mut().get_mut();
        if rust.profiles.len() >= 64 {
            controller
                .as_mut()
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

        let mut old_to_new: std::collections::HashMap<u8, u8> =
            std::collections::HashMap::with_capacity(rust.profiles.len());
        for (idx, p) in rust.profiles.iter_mut().enumerate() {
            let new_num = (idx + 1) as u8;
            old_to_new.insert(p.number, new_num);
            p.number = new_num;
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
        if let Some(ref sd_root) = rust.sd_root_path {
            if let Err(e) = save_profile_order_to_sd(sd_root, &rust.profiles) {
                record_system_event(
                    "WARN",
                    "STORAGE",
                    format!("Failed saving profile_info.txt after import: {e}"),
                );
            }
        }
        let sn = rust.serial_number.to_string();
        sync_profiles_to_device_cache(&sn, &rust.profiles);
        if !sd_status.is_empty() {
            mark_profiles_sd_synced(&sn, &rust.profiles);
        }

        let rules_snapshot = if let Ok(mut sw) = rust.switcher_state.lock() {
            let old_fb = sw.default_profile.unwrap_or(1);
            let mapped_fb = old_to_new
                .get(&old_fb)
                .copied()
                .unwrap_or(old_fb)
                .clamp(1, next_num);
            sw.default_profile = Some(mapped_fb);
            for rule in &mut sw.rules {
                if let Some(&mapped_target) = old_to_new.get(&rule.target_profile) {
                    rule.target_profile = mapped_target;
                }
            }
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
            if let Err(e) = sw.save_to_disk() {
                record_system_event(
                    "WARN",
                    "CONFIG",
                    format!("Failed saving imported rules: {e}"),
                );
            }
            sw.rules.clone()
        } else {
            Vec::new()
        };

        let p_json =
            DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_snapshot);
        let r_json = DeviceControllerRust::serialize_rules(&rules_snapshot, &rust.profiles);
        (next_num, candidate_name, sd_status, p_json, r_json)
    };

    controller
        .as_mut()
        .set_profiles_json(QString::from(&profiles_json));
    controller
        .as_mut()
        .set_auto_switch_rules_json(QString::from(&rules_json));
    controller.as_mut().select_profile(new_num as i32);
    controller.as_mut().sync_active_profile_state();
    controller
        .as_mut()
        .set_status_message(QString::from(&format!(
            "Imported Profile #{new_num} '{final_name}'{sd_msg}"
        )));
    true
}

pub fn save_profile_group_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
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
        let rust = controller.as_mut().rust_mut().get_mut();
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
        controller
            .as_mut()
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
        controller
            .as_mut()
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
        if let Err(e) = fs::create_dir_all(parent) {
            record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed creating profile group parent dir {}: {e}",
                    parent.display()
                ),
            );
        }
    }

    match fs::write(&target_file, &json_str) {
        Ok(()) => {
            controller
                .as_mut()
                .set_status_message(QString::from(&format!(
                    "✓ Saved Profile Group '{}' ({} profiles) to {}",
                    effective_group_name,
                    profiles_clone.len(),
                    target_file.display()
                )));
            true
        }
        Err(e) => {
            controller
                .as_mut()
                .set_status_message(QString::from(&format!(
                    "Failed saving profile group to {}: {e}",
                    target_file.display()
                )));
            false
        }
    }
}

pub fn load_profile_group_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
    source_path: QString,
) -> bool {
    let raw_str = source_path.to_string();
    let trimmed = raw_str.trim().trim_start_matches("file://");
    if trimmed.is_empty() {
        return false;
    }
    let src = PathBuf::from(trimmed);
    if !src.exists() {
        controller
            .as_mut()
            .set_status_message(QString::from(&format!(
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
            controller
                .as_mut()
                .set_status_message(QString::from("Profile group contains 0 profiles"));
            return false;
        }
        Err(err) => {
            controller.as_mut().set_status_message(QString::from(&err));
            return false;
        }
    };

    bundle.profiles.truncate(64);
    for (idx, p) in bundle.profiles.iter_mut().enumerate() {
        p.number = (idx + 1) as u8;
    }
    let target_active = bundle.active_profile.clamp(1, bundle.profiles.len() as u8) as i32;
    let prof_count = bundle.profiles.len();
    let group_title = if bundle.group_name.trim().is_empty() {
        "Loaded Group".to_string()
    } else {
        bundle.group_name.trim().to_string()
    };

    let (profiles_json, rules_json, new_fb) = {
        let rust = controller.as_mut().rust_mut().get_mut();
        rust.profiles = bundle.profiles;
        let sn = rust.serial_number.to_string();
        sync_profiles_to_device_cache(&sn, &rust.profiles);

        let fb_clamped = bundle
            .default_fallback_profile
            .unwrap_or(1)
            .clamp(1, prof_count as u8);
        let rules_snapshot = if let Ok(mut sw) = rust.switcher_state.lock() {
            if !bundle.auto_switch_rules.is_empty() {
                sw.rules = bundle.auto_switch_rules;
            }
            sw.default_profile = Some(fb_clamped);
            sw.loaded_from_disk = true;
            if let Err(e) = sw.save_to_disk() {
                record_system_event(
                    "WARN",
                    "CONFIG",
                    format!("Failed saving switcher state after group load: {e}"),
                );
            }
            sw.rules.clone()
        } else {
            Vec::new()
        };

        let p_json =
            DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_snapshot);
        let r_json = DeviceControllerRust::serialize_rules(&rules_snapshot, &rust.profiles);
        (p_json, r_json, fb_clamped as i32)
    };

    controller.as_mut().set_default_fallback_profile(new_fb);
    controller
        .as_mut()
        .set_profiles_json(QString::from(&profiles_json));
    controller
        .as_mut()
        .set_auto_switch_rules_json(QString::from(&rules_json));
    controller.as_mut().select_profile(target_active);
    controller.as_mut().sync_active_profile_state();
    controller.as_mut().set_status_message(QString::from(&format!(
            "✓ Loaded Profile Group '{group_title}' ({prof_count} profiles) — Review & Sync to duckyPad when ready"
        )));
    true
}

pub fn create_new_profile_group_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
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
    let custom_prof_name = clean_prof_name.trim().to_string();

    let default_prof = DuckyProfile {
        number: 1,
        name: "Default".to_string(),
        folder_name_override: Some("profile_Default".to_string()),
        bg_color: [0, 170, 255],
        keydown_color: Some([255, 255, 255]),
        dim_unused_keys: true,
        ..Default::default()
    };

    let mut initial_profiles = vec![default_prof];
    if !custom_prof_name.is_empty() && !custom_prof_name.eq_ignore_ascii_case("Default") {
        initial_profiles.push(DuckyProfile {
            number: 2,
            name: custom_prof_name.clone(),
            folder_name_override: Some(format!("profile_{custom_prof_name}")),
            bg_color: [0, 170, 255],
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            ..Default::default()
        });
    }

    let (profiles_json, rules_json) = {
        let rust = controller.as_mut().rust_mut().get_mut();
        rust.profiles = initial_profiles;
        let sn = rust.serial_number.to_string();
        sync_profiles_to_device_cache(&sn, &rust.profiles);

        let rules_snapshot = if let Ok(mut sw) = rust.switcher_state.lock() {
            sw.rules.clear();
            sw.default_profile = Some(1);
            sw.loaded_from_disk = true;
            if let Err(e) = sw.save_to_disk() {
                record_system_event(
                    "WARN",
                    "CONFIG",
                    format!("Failed saving switcher state after creating group: {e}"),
                );
            }
            sw.rules.clone()
        } else {
            Vec::new()
        };

        let p_json =
            DeviceControllerRust::serialize_profiles_summary(&rust.profiles, &rules_snapshot);
        let r_json = DeviceControllerRust::serialize_rules(&rules_snapshot, &rust.profiles);
        (p_json, r_json)
    };

    controller.as_mut().set_default_fallback_profile(1);
    controller
        .as_mut()
        .set_profiles_json(QString::from(&profiles_json));
    controller
        .as_mut()
        .set_auto_switch_rules_json(QString::from(&rules_json));
    controller.as_mut().select_profile(1);
    controller.as_mut().sync_active_profile_state();
    controller
        .as_mut()
        .set_status_message(QString::from(&format!(
            "✓ Created new Profile Group '{group_label}' with 'Default' as Fallback Profile (#1)"
        )));
    true
}

pub fn push_hardware_led_frame_json_impl(
    self_ref: &qobject::DeviceController,
    frame_json: QString,
) -> bool {
    if !*self_ref.connected() {
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
        let r = item
            .get("r")
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
            .clamp(0, 255) as u8;
        let g = item
            .get("g")
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
            .clamp(0, 255) as u8;
        let b = item
            .get("b")
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
            .clamp(0, 255) as u8;
        updates.push((slot, [r, g, b]));
    }
    if updates.is_empty() {
        return false;
    }
    std::thread::spawn(move || {
        if !set_batch_key_leds_hid(&updates) {
            record_system_event(
                "WARN",
                "HID",
                "Failed pushing batch key LED frame over USB HID",
            );
        }
    });
    true
}

pub fn get_profile_properties_json_impl(
    self_ref: &qobject::DeviceController,
    profile_number: i32,
) -> QString {
    let target_num = if profile_number > 0 {
        profile_number as u8
    } else {
        (*self_ref.active_profile()).max(1) as u8
    };
    let rust = self_ref.rust();
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
    let webhook_cfg = prof.profile_webhook();
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
        "webhook": webhook_cfg,
    });
    QString::from(&obj.to_string())
}

pub fn save_profile_properties_json_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
    profile_number: i32,
    props_json: QString,
) -> bool {
    let target_num = if profile_number > 0 {
        profile_number as u8
    } else {
        (*controller.active_profile()).max(1) as u8
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

    let bg_hex = val
        .get("bgColor")
        .and_then(|v| v.as_str())
        .unwrap_or("#103060");
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
    let webhook_opt: Option<ProfileWebhookConfig> = val
        .get("webhook")
        .and_then(|w| serde_json::from_value(w.clone()).ok());
    let extra_lines_vec: Vec<String> =
        if let Some(arr) = val.get("extraConfigLines").and_then(|v| v.as_array()) {
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
        let rust = controller.as_mut().rust_mut().get_mut();
        let Some(prof) = rust.profiles.iter_mut().find(|p| p.number == target_num) else {
            return false;
        };
        let existing_webhook = prof.profile_webhook();
        let old_name = prof.name.clone();
        if !sanitized_name.is_empty() && sanitized_name != old_name {
            prof.name = sanitized_name.clone();
            prof.folder_name_override = Some(format!("profile{}_{}", prof.number, sanitized_name));
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
                && !t.starts_with("PROFILE_WEBHOOK ")
                && !t.starts_with("IS_UPPER_HS ")
                && !t.starts_with("BG_COLOR ")
                && !t.starts_with("KEYDOWN_COLOR ")
                && !t.starts_with("DIM_UNUSED_KEYS ")
            {
                preserved.push(t.to_string());
            }
        }
        prof.preserved_config_lines = preserved;
        let final_webhook = webhook_opt.unwrap_or(existing_webhook);
        prof.set_profile_webhook(&final_webhook);
        let final_name = prof.name.clone();

        let prof_snapshot = prof.clone();
        if let Some(ref sd_root) = rust.sd_root_path {
            if let Err(e) = prof_snapshot.save_to_sd_root(sd_root) {
                record_system_event(
                    "WARN",
                    "STORAGE",
                    format!(
                        "Failed saving profile #{} properties to SD: {e}",
                        prof_snapshot.number
                    ),
                );
            }
        }
        if let Ok(mut sw) = rust.switcher_state.lock() {
            sw.align_rules_with_profiles(&rust.profiles);
            if let Err(e) = sw.save_to_disk() {
                record_system_event(
                    "WARN",
                    "CONFIG",
                    format!("Failed saving switcher state after updating profile properties: {e}"),
                );
            }
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
        controller.as_mut().sync_active_profile_state();
        controller.as_mut().refresh_color_palettes();
        let msg = format!(
            "✓ Updated Profile #{} ({}) properties",
            target_num, final_name
        );
        record_system_event("INFO", "PROFILE", &msg);
        controller.as_mut().sync_event_log_properties();
        controller.as_mut().set_status_message(QString::from(&msg));
        true
    } else {
        false
    }
}

pub fn list_profile_templates_json_impl() -> QString {
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
                "builtIn": t.is_builtin,
                "slotCount": slot_count,
                "configuredSlotCount": slot_count,
                "defaultProfileName": t.profile.name,
                "bgColor": bg_hex,
                "oledRotation": t.profile.oled_rotation_degrees(),
                "widgetRotation": t.profile.widget_rotation_degrees(),
            })
        })
        .collect();
    let json_str = serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string());
    QString::from(&json_str)
}
