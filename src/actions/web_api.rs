use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::actions::app_launch::{
    apply_app_launch_config_to_script, extract_app_launch_config, has_app_launch_config,
};
use crate::hardware::profile::{DuckyProfile, ProfileWebhookConfig};
use crate::hardware::storage::{
    open_duckypad_hid, write_single_pgv, HID_CMD_DUMP_PGV, HID_CMD_SET_LED_SINGLE, HID_IO_LOCK,
    HID_REPORT_ID_OUT, MAX_INPUT_SLOTS, MSC_SYNC_IN_PROGRESS, ONBOARD_KEY_COUNT,
};

// ============================================================================
// Web API Key Action Builder, Hardware Trigger Poller (_GV31 + evdev F13-F24)
// ============================================================================

pub use crate::vm::{
    default_fkey_for_slot, default_web_api_fkey, default_web_api_led_feedback,
    default_web_api_method, default_web_api_trigger_mode, WebApiConfig, WEB_API_META_PREFIX,
};
pub const WEB_API_BLOCK_BEGIN: &str = "// --- BEGIN DUCKYDECK WEB API TRIGGER ---";
pub const WEB_API_BLOCK_END: &str = "// --- END DUCKYDECK WEB API TRIGGER ---";
pub const WEB_API_PGV_MAGIC: u32 = 0x444B_0000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebApiDestination {
    pub name: String,
    #[serde(default = "default_web_api_method")]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: String,
}

pub fn web_api_destinations_path() -> PathBuf {
    crate::paths::config_dir().join("web_api_destinations.json")
}

pub fn load_web_api_destinations() -> Vec<WebApiDestination> {
    let path = web_api_destinations_path();
    let Ok(raw) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    let mut list: Vec<WebApiDestination> = serde_json::from_str(&raw).unwrap_or_default();
    for item in &mut list {
        item.name = item.name.trim().to_string();
        item.method = item.method.trim().to_ascii_uppercase();
        if item.method.is_empty() {
            item.method = "POST".to_string();
        }
        item.url = item.url.trim().to_string();
        item.headers = item.headers.trim().to_string();
    }
    list.retain(|d| !d.name.is_empty() && !d.url.is_empty());
    list
}

pub fn save_web_api_destinations(destinations: &[WebApiDestination]) -> Result<(), String> {
    let path = web_api_destinations_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed creating config dir {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(destinations)
        .map_err(|e| format!("Failed serializing Web API destinations: {e}"))?;
    fs::write(&path, json).map_err(|e| format!("Failed writing {}: {e}", path.display()))
}

pub fn upsert_web_api_destination(
    name: &str,
    method: &str,
    url: &str,
    headers: &str,
) -> Result<Vec<WebApiDestination>, String> {
    let clean_name = name.trim();
    let clean_url = url.trim();
    if clean_name.is_empty() {
        return Err("Destination name cannot be empty.".to_string());
    }
    if clean_url.is_empty() {
        return Err("Destination URL cannot be empty.".to_string());
    }
    let clean_method = {
        let m = method.trim().to_ascii_uppercase();
        if m.is_empty() {
            "POST".to_string()
        } else {
            m
        }
    };
    let clean_headers = headers.trim().to_string();

    let mut list = load_web_api_destinations();
    if let Some(existing) = list
        .iter_mut()
        .find(|d| d.name.eq_ignore_ascii_case(clean_name))
    {
        existing.name = clean_name.to_string();
        existing.method = clean_method;
        existing.url = clean_url.to_string();
        existing.headers = clean_headers;
    } else {
        list.push(WebApiDestination {
            name: clean_name.to_string(),
            method: clean_method,
            url: clean_url.to_string(),
            headers: clean_headers,
        });
    }
    save_web_api_destinations(&list)?;
    Ok(list)
}

pub fn delete_web_api_destination_by_name(name: &str) -> Result<Vec<WebApiDestination>, String> {
    let clean_name = name.trim();
    let mut list = load_web_api_destinations();
    list.retain(|d| !d.name.eq_ignore_ascii_case(clean_name));
    save_web_api_destinations(&list)?;
    Ok(list)
}

pub fn seed_web_api_destinations_from_profiles(
    profiles: &[DuckyProfile],
) -> Vec<WebApiDestination> {
    let path = web_api_destinations_path();
    let existing = load_web_api_destinations();
    if path.exists() || !existing.is_empty() {
        return existing;
    }
    let mut seeded: Vec<WebApiDestination> = Vec::new();
    for prof in profiles {
        for (&slot_num, slot) in &prof.slots {
            for script in [&slot.on_press_script, &slot.on_release_script] {
                if !has_web_api_config(script) {
                    continue;
                }
                let cfg = extract_web_api_config(script, slot_num);
                let url = cfg.url.trim();
                if url.is_empty() {
                    continue;
                }
                let already_have = seeded.iter().any(|d| {
                    d.url.eq_ignore_ascii_case(url)
                        && d.method.eq_ignore_ascii_case(&cfg.method)
                        && d.headers.trim() == cfg.headers.trim()
                });
                if already_have {
                    continue;
                }
                let base_name = if !cfg.destination.trim().is_empty() {
                    cfg.destination.trim().to_string()
                } else {
                    let lbl = format!("{} {}", slot.label_line1.trim(), slot.label_line2.trim())
                        .trim()
                        .to_string();
                    if !lbl.is_empty() {
                        format!("{} ({})", prof.name.trim(), lbl)
                    } else {
                        format!("{} — Key {}", prof.name.trim(), slot_num)
                    }
                };
                let mut candidate_name = base_name.clone();
                let mut suffix = 2usize;
                while seeded
                    .iter()
                    .any(|d| d.name.eq_ignore_ascii_case(&candidate_name))
                {
                    candidate_name = format!("{base_name} #{suffix}");
                    suffix += 1;
                }
                seeded.push(WebApiDestination {
                    name: candidate_name,
                    method: cfg.method.clone(),
                    url: url.to_string(),
                    headers: cfg.headers.trim().to_string(),
                });
            }
        }
    }
    if !seeded.is_empty() {
        if let Err(e) = save_web_api_destinations(&seeded) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "WEB_API",
                format!("Failed saving seeded Web API destinations: {e}"),
            );
        }
    }
    seeded
}

pub fn resolve_web_api_config_destination(cfg: &mut WebApiConfig) {
    let dest_name = cfg.destination.trim();
    if dest_name.is_empty() {
        return;
    }
    let destinations = load_web_api_destinations();
    if let Some(found) = destinations
        .iter()
        .find(|d| d.name.eq_ignore_ascii_case(dest_name))
    {
        cfg.destination = found.name.clone();
        cfg.method = found.method.clone();
        cfg.url = found.url.clone();
        cfg.headers = found.headers.clone();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebApiExecutionResult {
    pub ok: bool,
    pub status_code: u16,
    pub elapsed_ms: u64,
    pub summary: String,
    pub response_preview: String,
    pub route_note: Option<String>,
}

pub fn extract_web_api_config(script: &str, slot_num: usize) -> WebApiConfig {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(WEB_API_META_PREFIX) {
            if let Ok(mut cfg) = serde_json::from_str::<WebApiConfig>(json_part.trim()) {
                resolve_web_api_config_destination(&mut cfg);
                cfg.method = cfg.method.trim().to_ascii_uppercase();
                if cfg.method.is_empty() {
                    cfg.method = "POST".to_string();
                }
                if cfg.trigger_mode != "fkey" {
                    cfg.trigger_mode = "pgv".to_string();
                }
                if cfg.fkey.trim().is_empty() {
                    cfg.fkey = default_fkey_for_slot(slot_num);
                }
                return cfg;
            }
        }
    }
    WebApiConfig {
        fkey: default_fkey_for_slot(slot_num),
        ..Default::default()
    }
}

pub fn has_web_api_config(script: &str) -> bool {
    for line in script.lines() {
        let trimmed = line.trim();
        if let Some(json_part) = trimmed.strip_prefix(WEB_API_META_PREFIX) {
            if let Ok(mut cfg) = serde_json::from_str::<WebApiConfig>(json_part.trim()) {
                resolve_web_api_config_destination(&mut cfg);
                if cfg.enabled && !cfg.url.trim().is_empty() {
                    return true;
                }
            }
        }
    }
    false
}

pub fn strip_web_api_from_script(script: &str) -> String {
    let mut out_lines = Vec::new();
    let mut inside_block = false;
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(WEB_API_META_PREFIX) {
            continue;
        }
        if trimmed == WEB_API_BLOCK_BEGIN {
            inside_block = true;
            continue;
        }
        if trimmed == WEB_API_BLOCK_END {
            inside_block = false;
            continue;
        }
        if !inside_block {
            out_lines.push(line);
        }
    }
    let joined = out_lines.join("\n");
    let clean = joined.trim_start_matches('\n');
    if clean.is_empty() {
        String::new()
    } else if clean.ends_with('\n') {
        clean.to_string()
    } else {
        format!("{clean}\n")
    }
}

pub fn encode_pgv31_trigger(profile_num: u8, slot_num: usize, is_release: bool) -> u32 {
    let rel_bit = if is_release { 1u32 << 15 } else { 0u32 };
    let prof_bits = ((profile_num as u32) & 0x7F) << 8;
    let slot_bits = (slot_num as u32) & 0xFF;
    WEB_API_PGV_MAGIC | rel_bit | prof_bits | slot_bits
}

pub fn decode_pgv31_trigger(raw_val: i32) -> Option<(u8, usize, bool)> {
    let uval = raw_val as u32;
    if (uval & 0xFFFF_0000) != WEB_API_PGV_MAGIC {
        return None;
    }
    let is_release = ((uval >> 15) & 1) != 0;
    let profile_num = ((uval >> 8) & 0x7F) as u8;
    let slot_num = (uval & 0xFF) as usize;
    if (1..=MAX_INPUT_SLOTS).contains(&slot_num) {
        Some((profile_num, slot_num, is_release))
    } else {
        None
    }
}

pub fn apply_web_api_config_to_script(
    existing_script: &str,
    cfg: &WebApiConfig,
    profile_num: u8,
    slot_num: usize,
    is_release: bool,
) -> String {
    let remainder = strip_web_api_from_script(existing_script);
    if !cfg.enabled {
        return remainder;
    }

    let mut normalized = cfg.clone();
    normalized.destination = normalized.destination.trim().to_string();
    resolve_web_api_config_destination(&mut normalized);
    normalized.method = normalized.method.trim().to_ascii_uppercase();
    if normalized.method.is_empty() {
        normalized.method = "POST".to_string();
    }
    if normalized.trigger_mode != "fkey" {
        normalized.trigger_mode = "pgv".to_string();
    }
    let valid_fkey = {
        let up = normalized.fkey.trim().to_ascii_uppercase();
        if matches!(
            up.as_str(),
            "F13"
                | "F14"
                | "F15"
                | "F16"
                | "F17"
                | "F18"
                | "F19"
                | "F20"
                | "F21"
                | "F22"
                | "F23"
                | "F24"
        ) {
            up
        } else {
            default_fkey_for_slot(slot_num)
        }
    };
    normalized.fkey = valid_fkey.clone();

    let meta_json = serde_json::to_string(&normalized).unwrap_or_else(|_| "{}".to_string());
    let trigger_code = encode_pgv31_trigger(profile_num, slot_num, is_release);

    let mut out = String::new();
    out.push_str(&format!("{WEB_API_META_PREFIX}{meta_json}\n"));
    out.push_str(&format!("{WEB_API_BLOCK_BEGIN}\n"));

    let oled_clean = normalized.oled_text.trim();
    if !oled_clean.is_empty() {
        out.push_str("OLED_CLEAR\n");
        out.push_str(&format!("OLED_CPRINT {oled_clean}\n"));
        out.push_str("OLED_UPDATE\n");
    }

    if normalized.trigger_mode == "fkey" {
        out.push_str(&format!("CONTROL ALT SHIFT {valid_fkey}\n"));
    } else {
        out.push_str(&format!("$_GV31 = 0x{trigger_code:08X}\n"));
    }

    out.push_str(&format!("{WEB_API_BLOCK_END}\n"));
    if !remainder.trim().is_empty() {
        out.push_str(&remainder);
    }
    out
}

pub fn sync_profile_web_api_triggers(profiles: &mut [DuckyProfile]) -> bool {
    let mut changed = false;
    for prof in profiles.iter_mut() {
        let pnum = prof.number;
        for (&slot_num, slot) in prof.slots.iter_mut() {
            if has_web_api_config(&slot.on_press_script) {
                let cfg = extract_web_api_config(&slot.on_press_script, slot_num);
                let updated = apply_web_api_config_to_script(
                    &slot.on_press_script,
                    &cfg,
                    pnum,
                    slot_num,
                    false,
                );
                if updated != slot.on_press_script {
                    slot.on_press_script = updated;
                    changed = true;
                }
            }
            if has_web_api_config(&slot.on_release_script) {
                let cfg = extract_web_api_config(&slot.on_release_script, slot_num);
                let updated = apply_web_api_config_to_script(
                    &slot.on_release_script,
                    &cfg,
                    pnum,
                    slot_num,
                    true,
                );
                if updated != slot.on_release_script {
                    slot.on_release_script = updated;
                    changed = true;
                }
            }
            if has_app_launch_config(&slot.on_press_script) {
                let cfg = extract_app_launch_config(&slot.on_press_script, slot_num);
                let updated = apply_app_launch_config_to_script(
                    &slot.on_press_script,
                    &cfg,
                    pnum,
                    slot_num,
                    false,
                );
                if updated != slot.on_press_script {
                    slot.on_press_script = updated;
                    changed = true;
                }
            }
            if has_app_launch_config(&slot.on_release_script) {
                let cfg = extract_app_launch_config(&slot.on_release_script, slot_num);
                let updated = apply_app_launch_config_to_script(
                    &slot.on_release_script,
                    &cfg,
                    pnum,
                    slot_num,
                    true,
                );
                if updated != slot.on_release_script {
                    slot.on_release_script = updated;
                    changed = true;
                }
            }
        }
    }
    changed
}

pub fn poll_and_clear_pgv31_trigger() -> Option<(u8, usize, bool)> {
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return None;
    }
    let _guard = HID_IO_LOCK.try_lock().ok()?;
    let device = open_duckypad_hid().ok()?;

    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x3F;
    tx[2] = HID_CMD_DUMP_PGV;
    tx[3] = 30; // Dump _GV30 and _GV31 in a single packet

    if device.write(&tx).is_err() {
        return None;
    }

    let mut rx = [0u8; 64];
    let bytes_read = device.read_timeout(&mut rx, 120).ok()?;
    if bytes_read < 11 || rx[2] != 0 {
        return None;
    }

    // rx[0]=ReportID(4), rx[1]=Seq, rx[2]=Status(0), rx[3..7]=_GV30, rx[7..11]=_GV31
    let raw_gv31 = i32::from_le_bytes([rx[7], rx[8], rx[9], rx[10]]);
    if let Some(decoded) = decode_pgv31_trigger(raw_gv31) {
        if let Err(e) = write_single_pgv(&device, 31, 0) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "HID",
                format!("Failed clearing _GV31 trigger register: {e}"),
            );
        }
        return Some(decoded);
    }
    None
}

pub fn set_single_key_led_hid(slot_num: usize, rgb: [u8; 3]) -> bool {
    if !(1..=ONBOARD_KEY_COUNT).contains(&slot_num) {
        return false;
    }
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    let Ok(_guard) = HID_IO_LOCK.lock() else {
        return false;
    };
    let Ok(device) = open_duckypad_hid() else {
        return false;
    };
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x33;
    tx[2] = HID_CMD_SET_LED_SINGLE;
    tx[3] = (slot_num - 1) as u8;
    tx[4] = rgb[0];
    tx[5] = rgb[1];
    tx[6] = rgb[2];
    let ok = device.write(&tx).is_ok();
    if ok {
        let mut rx = [0u8; 64];
        if let Err(e) = device.read_timeout(&mut rx, 120) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "HID",
                format!("HID ACK read timeout in set_single_key_led_hid(slot {slot_num}): {e}"),
            );
        }
    }
    ok
}

pub fn set_batch_key_leds_hid(updates: &[(usize, [u8; 3])]) -> bool {
    if updates.is_empty() {
        return true;
    }
    if MSC_SYNC_IN_PROGRESS.load(std::sync::atomic::Ordering::SeqCst) {
        return false;
    }
    let Ok(_guard) = HID_IO_LOCK.try_lock() else {
        return false;
    };
    let Ok(device) = open_duckypad_hid() else {
        return false;
    };
    let mut any_ok = false;
    for (idx, &(slot_num, rgb)) in updates.iter().enumerate() {
        if !(1..=ONBOARD_KEY_COUNT).contains(&slot_num) {
            continue;
        }
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x40u8.wrapping_add(idx as u8);
        tx[2] = HID_CMD_SET_LED_SINGLE;
        tx[3] = (slot_num - 1) as u8;
        tx[4] = rgb[0];
        tx[5] = rgb[1];
        tx[6] = rgb[2];
        if device.write(&tx).is_ok() {
            any_ok = true;
            let mut rx = [0u8; 64];
            if let Err(e) = device.read_timeout(&mut rx, 20) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "HID",
                    format!("HID ACK read timeout in set_batch_key_leds_hid(slot {slot_num}): {e}"),
                );
            }
        }
    }
    any_ok
}

pub async fn flash_slot_led_feedback(slot_num: usize, success: bool, restore_rgb: [u8; 3]) {
    if !(1..=ONBOARD_KEY_COUNT).contains(&slot_num) {
        return;
    }
    if success {
        set_single_key_led_hid(slot_num, [0, 255, 80]);
        tokio::time::sleep(std::time::Duration::from_millis(650)).await;
        set_single_key_led_hid(slot_num, restore_rgb);
    } else {
        set_single_key_led_hid(slot_num, [255, 30, 30]);
        tokio::time::sleep(std::time::Duration::from_millis(260)).await;
        set_single_key_led_hid(slot_num, [40, 0, 0]);
        tokio::time::sleep(std::time::Duration::from_millis(120)).await;
        set_single_key_led_hid(slot_num, [255, 30, 30]);
        tokio::time::sleep(std::time::Duration::from_millis(320)).await;
        set_single_key_led_hid(slot_num, restore_rgb);
    }
}

fn resolve_home_assistant_service_url(url: &str, body: &str) -> (String, Option<String>) {
    let trimmed_body = body.trim();
    if trimmed_body.is_empty() {
        return (url.to_string(), None);
    }
    let Ok(json_val) = serde_json::from_str::<serde_json::Value>(trimmed_body) else {
        return (url.to_string(), None);
    };
    let Some(entity_val) = json_val.get("entity_id") else {
        return (url.to_string(), None);
    };

    let mut entity_ids: Vec<&str> = Vec::new();
    match entity_val {
        serde_json::Value::String(s) => entity_ids.push(s.as_str()),
        serde_json::Value::Array(arr) => {
            for item in arr {
                if let Some(s) = item.as_str() {
                    entity_ids.push(s);
                }
            }
        }
        _ => {}
    }
    if entity_ids.is_empty() {
        return (url.to_string(), None);
    }

    for &(src_segment, expected_prefix) in &[
        ("/api/services/switch/toggle", "switch."),
        ("/api/services/switch/turn_on", "switch."),
        ("/api/services/switch/turn_off", "switch."),
        ("/api/services/light/toggle", "light."),
        ("/api/services/light/turn_on", "light."),
        ("/api/services/light/turn_off", "light."),
    ] {
        if url.contains(src_segment)
            && entity_ids
                .iter()
                .any(|eid| eid.contains('.') && !eid.starts_with(expected_prefix))
        {
            let action = src_segment.rsplit('/').next().unwrap_or("toggle");
            let dst_segment = format!("/api/services/homeassistant/{action}");
            let new_url = url.replacen(src_segment, &dst_segment, 1);
            let note = format!(
                "Home Assistant service auto-routed '{src_segment}' -> '{dst_segment}' to match entity_id domain ({})",
                entity_ids.join(", ")
            );
            return (new_url, Some(note));
        }
    }

    (url.to_string(), None)
}

pub async fn execute_web_api_request(input_cfg: &WebApiConfig) -> WebApiExecutionResult {
    let mut resolved_cfg = input_cfg.clone();
    resolve_web_api_config_destination(&mut resolved_cfg);
    let cfg = &resolved_cfg;
    let raw_url = cfg.url.trim();
    if raw_url.is_empty() {
        return WebApiExecutionResult {
            ok: false,
            status_code: 0,
            elapsed_ms: 0,
            summary: "Error: URL is empty".to_string(),
            response_preview: String::new(),
            route_note: None,
        };
    }

    let (effective_url, route_note) = resolve_home_assistant_service_url(raw_url, &cfg.body);

    let start = std::time::Instant::now();
    let client = match reqwest::Client::builder()
        .user_agent(concat!("DuckyDeck-Studio/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return WebApiExecutionResult {
                ok: false,
                status_code: 0,
                elapsed_ms: 0,
                summary: format!("HTTP client init failed: {e}"),
                response_preview: String::new(),
                route_note,
            };
        }
    };

    let method_str = cfg.method.trim().to_ascii_uppercase();
    let method = match method_str.as_str() {
        "GET" => reqwest::Method::GET,
        "PUT" => reqwest::Method::PUT,
        "PATCH" => reqwest::Method::PATCH,
        "DELETE" => reqwest::Method::DELETE,
        "HEAD" => reqwest::Method::HEAD,
        _ => reqwest::Method::POST,
    };

    let mut req = client.request(method.clone(), &effective_url);
    let mut has_content_type = false;

    for raw_line in cfg.headers.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            let val = v.trim();
            if key.eq_ignore_ascii_case("content-type") {
                has_content_type = true;
            }
            if !key.is_empty() {
                req = req.header(key, val);
            }
        } else if line.to_ascii_lowercase().starts_with("bearer ") {
            req = req.header("Authorization", line);
        }
    }

    let body_trimmed = cfg.body.trim();
    if !body_trimmed.is_empty() && method != reqwest::Method::GET && method != reqwest::Method::HEAD
    {
        if !has_content_type && (body_trimmed.starts_with('{') || body_trimmed.starts_with('[')) {
            req = req.header("Content-Type", "application/json");
        }
        req = req.body(cfg.body.clone());
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status();
            let code = status.as_u16();
            let ok = status.is_success();
            let body_text = resp.text().await.unwrap_or_default();
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let single_line: String = body_text
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            let preview: String = if single_line.chars().count() > 180 {
                let trunc: String = single_line.chars().take(180).collect();
                format!("{trunc}…")
            } else {
                single_line
            };
            let reason = status.canonical_reason().unwrap_or("");
            let summary = if preview.is_empty() {
                format!("HTTP {code} {reason} ({elapsed_ms} ms)")
                    .trim()
                    .to_string()
            } else {
                format!("HTTP {code} {reason} ({elapsed_ms} ms) — {preview}")
            };
            WebApiExecutionResult {
                ok,
                status_code: code,
                elapsed_ms,
                summary,
                response_preview: preview,
                route_note,
            }
        }
        Err(e) => {
            let elapsed_ms = start.elapsed().as_millis() as u64;
            let err_msg = if e.is_timeout() {
                format!("Request timed out after {elapsed_ms} ms")
            } else if e.is_connect() {
                format!("Connection error: {e}")
            } else {
                format!("Request error: {e}")
            };
            WebApiExecutionResult {
                ok: false,
                status_code: 0,
                elapsed_ms,
                summary: err_msg.clone(),
                response_preview: err_msg,
                route_note,
            }
        }
    }
}

// ============================================================================
// Home Assistant Web API Connection & Entity Discovery + Profile Change Webhook
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HomeAssistantConfig {
    #[serde(default, alias = "baseUrl")]
    pub base_url: String,
    #[serde(default, alias = "accessToken")]
    pub access_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeAssistantEntity {
    #[serde(alias = "entityId")]
    pub entity_id: String,
    pub domain: String,
    #[serde(alias = "friendlyName")]
    pub friendly_name: String,
    pub state: String,
    pub icon: String,
    #[serde(alias = "areaOrDevice")]
    pub area_or_device: String,
    #[serde(alias = "suggestedService")]
    pub suggested_service: String,
}

pub fn home_assistant_config_path() -> PathBuf {
    crate::paths::config_dir().join("home_assistant.json")
}

pub fn load_home_assistant_config() -> HomeAssistantConfig {
    let path = home_assistant_config_path();
    if let Ok(raw) = fs::read_to_string(&path) {
        if let Ok(mut cfg) = serde_json::from_str::<HomeAssistantConfig>(&raw) {
            cfg.base_url = cfg.base_url.trim().trim_end_matches('/').to_string();
            cfg.access_token = cfg.access_token.trim().to_string();
            if !cfg.base_url.is_empty() || !cfg.access_token.is_empty() {
                return cfg;
            }
        }
    }

    // Fallback: try to infer base_url and Bearer token from any saved Home Assistant WebApiDestination
    for dest in load_web_api_destinations() {
        let u = dest.url.trim();
        if let Some(idx) = u.find("/api/") {
            let base_url = u[..idx].trim_end_matches('/').to_string();
            let mut token = String::new();
            for line in dest.headers.lines() {
                let t = line.trim();
                if let Some((k, v)) = t.split_once(':') {
                    if k.trim().eq_ignore_ascii_case("authorization") {
                        let val = v.trim();
                        if let Some(rest) = val
                            .strip_prefix("Bearer ")
                            .or_else(|| val.strip_prefix("bearer "))
                        {
                            token = rest.trim().to_string();
                        }
                    }
                }
            }
            if !base_url.is_empty() {
                return HomeAssistantConfig {
                    base_url,
                    access_token: token,
                };
            }
        }
    }

    HomeAssistantConfig {
        base_url: "http://homeassistant.local:8123".to_string(),
        access_token: String::new(),
    }
}

pub fn save_home_assistant_config(
    base_url: &str,
    access_token: &str,
) -> Result<HomeAssistantConfig, String> {
    let mut clean_url = base_url.trim().trim_end_matches('/').to_string();
    if let Some(idx) = clean_url.find("/api/") {
        clean_url = clean_url[..idx].trim_end_matches('/').to_string();
    } else if let Some(stripped) = clean_url.strip_suffix("/api") {
        clean_url = stripped.trim_end_matches('/').to_string();
    }
    let mut clean_token = access_token.trim().to_string();
    if let Some(rest) = clean_token
        .strip_prefix("Bearer ")
        .or_else(|| clean_token.strip_prefix("bearer "))
    {
        clean_token = rest.trim().to_string();
    }
    let cfg = HomeAssistantConfig {
        base_url: clean_url,
        access_token: clean_token,
    };
    let path = home_assistant_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed creating config dir: {e}"))?;
    }
    let json = serde_json::to_string_pretty(&cfg)
        .map_err(|e| format!("Failed serializing Home Assistant config: {e}"))?;
    fs::write(&path, json).map_err(|e| format!("Failed writing {}: {e}", path.display()))?;
    Ok(cfg)
}

pub fn suggested_ha_service_for_domain(domain: &str) -> &'static str {
    match domain {
        "light" | "switch" | "input_boolean" | "fan" | "cover" | "climate" | "humidifier" => {
            "toggle"
        }
        "media_player" => "media_play_pause",
        "scene" | "script" => "turn_on",
        "automation" => "trigger",
        "button" | "input_button" => "press",
        "lock" => "unlock",
        "vacuum" => "start",
        _ => "toggle",
    }
}

pub async fn fetch_and_search_home_assistant_entities(
    base_url: &str,
    access_token: &str,
    query: &str,
    domain_filter: &str,
) -> Result<Vec<HomeAssistantEntity>, String> {
    let saved = save_home_assistant_config(base_url, access_token)?;
    if saved.base_url.is_empty() {
        return Err(
            "Home Assistant Base URL is required (e.g. http://homeassistant.local:8123)"
                .to_string(),
        );
    }
    if saved.access_token.is_empty() {
        return Err("Home Assistant Long-Lived Access Token is required".to_string());
    }

    let states_url = format!("{}/api/states", saved.base_url);
    let client = reqwest::Client::builder()
        .user_agent(concat!("DuckyDeck-Studio/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(&states_url)
        .header("Authorization", format!("Bearer {}", saved.access_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed connecting to Home Assistant at {states_url}: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Home Assistant returned HTTP {} from {}",
            resp.status(),
            states_url
        ));
    }

    let raw_states: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid JSON from Home Assistant /api/states: {e}"))?;

    let arr = raw_states
        .as_array()
        .ok_or_else(|| "Unexpected response format from /api/states".to_string())?;

    let q_lower = query.trim().to_ascii_lowercase();
    let dom_lower = domain_filter.trim().to_ascii_lowercase();

    let actionable_domains = [
        "light",
        "switch",
        "scene",
        "script",
        "automation",
        "button",
        "input_button",
        "input_boolean",
        "media_player",
        "fan",
        "cover",
        "climate",
        "lock",
        "vacuum",
        "siren",
        "humidifier",
        "water_heater",
        "remote",
        "select",
        "input_select",
        "number",
        "input_number",
    ];

    let mut results = Vec::new();
    for item in arr {
        let Some(entity_id) = item.get("entity_id").and_then(|v| v.as_str()) else {
            continue;
        };
        let domain = entity_id
            .split('.')
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        if !dom_lower.is_empty() && dom_lower != "all" {
            if domain != dom_lower {
                continue;
            }
        } else if q_lower.is_empty() && !actionable_domains.contains(&domain.as_str()) {
            // When browsing with no search query and "all" filter, prioritize controllable domains first
            continue;
        }

        let state_str = item
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let attrs = item.get("attributes");
        let friendly_name = attrs
            .and_then(|a| a.get("friendly_name"))
            .and_then(|v| v.as_str())
            .unwrap_or(entity_id)
            .to_string();
        let icon = attrs
            .and_then(|a| a.get("icon"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let device_class = attrs
            .and_then(|a| a.get("device_class"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if !q_lower.is_empty() {
            let matches_id = entity_id.to_ascii_lowercase().contains(&q_lower);
            let matches_name = friendly_name.to_ascii_lowercase().contains(&q_lower);
            let matches_class = device_class.to_ascii_lowercase().contains(&q_lower);
            let matches_state = state_str.to_ascii_lowercase().contains(&q_lower);
            if !matches_id && !matches_name && !matches_class && !matches_state {
                continue;
            }
        }

        let suggested_service = suggested_ha_service_for_domain(&domain).to_string();
        results.push(HomeAssistantEntity {
            entity_id: entity_id.to_string(),
            domain,
            friendly_name,
            state: state_str,
            icon,
            area_or_device: device_class,
            suggested_service,
        });
    }

    results.sort_by(|a, b| {
        a.domain
            .cmp(&b.domain)
            .then_with(|| {
                a.friendly_name
                    .to_ascii_lowercase()
                    .cmp(&b.friendly_name.to_ascii_lowercase())
            })
            .then_with(|| a.entity_id.cmp(&b.entity_id))
    });

    if results.len() > 300 {
        results.truncate(300);
    }

    Ok(results)
}

pub fn expand_profile_webhook_template(
    template: &str,
    profile_number: u8,
    profile_name: &str,
    previous_profile_number: u8,
    previous_profile_name: &str,
    source: &str,
    serial: &str,
) -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let trimmed = template.trim();
    if trimmed.is_empty() {
        return serde_json::json!({
            "event": "profile_changed",
            "profile_number": profile_number,
            "profile_name": profile_name,
            "previous_profile_number": previous_profile_number,
            "previous_profile_name": previous_profile_name,
            "source": source,
            "device_serial": serial,
            "timestamp": ts
        })
        .to_string();
    }
    let escape_json = |s: &str| -> String { s.replace('\\', "\\\\").replace('"', "\\\"") };
    template
        .replace("{{profile_number}}", &profile_number.to_string())
        .replace("{{profile_name}}", &escape_json(profile_name))
        .replace(
            "{{previous_profile_number}}",
            &previous_profile_number.to_string(),
        )
        .replace(
            "{{previous_profile_name}}",
            &escape_json(previous_profile_name),
        )
        .replace("{{source}}", &escape_json(source))
        .replace("{{device_serial}}", &escape_json(serial))
        .replace("{{timestamp}}", &ts.to_string())
}

pub async fn execute_profile_webhook_request(
    webhook: &ProfileWebhookConfig,
    profile_number: u8,
    profile_name: &str,
    previous_profile_number: u8,
    previous_profile_name: &str,
    source: &str,
    serial: &str,
) -> WebApiExecutionResult {
    let expanded_url = expand_profile_webhook_template(
        &webhook.url,
        profile_number,
        profile_name,
        previous_profile_number,
        previous_profile_name,
        source,
        serial,
    );
    let expanded_body = expand_profile_webhook_template(
        &webhook.body,
        profile_number,
        profile_name,
        previous_profile_number,
        previous_profile_name,
        source,
        serial,
    );
    let web_cfg = WebApiConfig {
        enabled: true,
        destination: webhook.destination.clone(),
        method: if webhook.method.trim().is_empty() {
            "POST".to_string()
        } else {
            webhook.method.clone()
        },
        url: expanded_url,
        headers: webhook.headers.clone(),
        body: expanded_body,
        oled_text: String::new(),
        led_feedback: false,
        trigger_mode: "pgv".to_string(),
        fkey: "F13".to_string(),
    };
    execute_web_api_request(&web_cfg).await
}
