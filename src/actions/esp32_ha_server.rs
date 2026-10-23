use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU16, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use crate::hardware::profile::{
    is_esp32_ha_configured_profiles, list_manageable_duckypad_targets,
    load_cached_profiles_for_serial_for_model, mark_profiles_sd_synced,
    sync_profiles_to_device_cache_with_meta,
};
use crate::hardware::storage::{
    format_disk_label_for_device, model_name_for_hw_model, normalize_duckypad_serial,
    with_cached_duckypad_hid_for_serial, DuckyKeySlot, DuckyProfile, DuckySdDeviceMeta,
    DP_MODEL_DUCKYPAD_PRO, DP_MODEL_OG_DUCKYPAD, HID_CMD_SET_LED_SINGLE, HID_REPORT_ID_OUT,
};

pub const DEFAULT_ESP32_HA_PORT: u16 = 8099;
pub const DEFAULT_ESP32_HA_WEB_PORT: u16 = DEFAULT_ESP32_HA_PORT;
const ESP32_HA_WEB_UI_HTML: &str = include_str!("../../data/esp32_ha_web_ui.html");
const DUCKYPAD_KEY_COUNT: usize = 26;
const MAX_STATE_COLORS: usize = 6;
const STATE_LABEL_REVERT_MS: u64 = 3000;

/// Loads the Web UI HTML directly from `/home/millinr/Builds/duckypad-esp32-ha/src/web_ui.h`
/// when available on disk so the ESP32-S3 firmware Web UI and DuckyDeck Studio Web UI stay
/// automatically in 100% feature sync, falling back to the embedded `data/esp32_ha_web_ui.html`.
pub fn get_esp32_ha_web_ui_html() -> String {
    let esp32_header = PathBuf::from("/home/millinr/Builds/duckypad-esp32-ha/src/web_ui.h");
    if let Ok(raw) = fs::read_to_string(&esp32_header) {
        if let Some((_, after_delim)) = raw.split_once("R\"rawliteral(") {
            if let Some((html, _)) = after_delim.rsplit_once(")rawliteral\"") {
                if !html.trim().is_empty() {
                    return html.to_string();
                }
            }
        }
    }
    ESP32_HA_WEB_UI_HTML.to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Esp32HaStateColorRule {
    pub v: String,
    pub col: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Esp32HaKeyMeta {
    #[serde(default)]
    pub entity_id: String,
    #[serde(default)]
    pub state_colors: Vec<Esp32HaStateColorRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedEsp32HaConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_model_mode")]
    pub model_mode: String,
    #[serde(default = "default_ssid")]
    pub ssid: String,
    #[serde(default = "default_mqtt_uri")]
    pub mqtt_uri: String,
    #[serde(default)]
    pub target_serial: String,
    #[serde(default)]
    pub key_meta: HashMap<String, Esp32HaKeyMeta>,
}

fn default_port() -> u16 {
    DEFAULT_ESP32_HA_PORT
}
fn default_model_mode() -> String {
    "auto".to_string()
}
fn default_ssid() -> String {
    "DuckyDeck-Studio-Host".to_string()
}
fn default_mqtt_uri() -> String {
    "mqtt://mqtt.milliner.me:1883".to_string()
}

impl Default for PersistedEsp32HaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            port: default_port(),
            model_mode: default_model_mode(),
            ssid: default_ssid(),
            mqtt_uri: default_mqtt_uri(),
            target_serial: String::new(),
            key_meta: HashMap::new(),
        }
    }
}

fn config_file_path() -> PathBuf {
    crate::paths::config_dir().join("esp32_ha_bridge.json")
}

pub fn load_esp32_ha_config() -> PersistedEsp32HaConfig {
    let path = config_file_path();
    if let Ok(raw) = fs::read_to_string(&path) {
        if let Ok(cfg) = serde_json::from_str::<PersistedEsp32HaConfig>(&raw) {
            return cfg;
        }
    }
    PersistedEsp32HaConfig::default()
}

pub fn save_esp32_ha_config(cfg: &PersistedEsp32HaConfig) {
    let path = config_file_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(cfg) {
        let _ = fs::write(&path, json);
    }
}

struct ServerRuntime {
    stop_flag: Arc<AtomicBool>,
    port: u16,
}

#[derive(Debug, Clone, Default)]
struct LiveKeyRuntimeState {
    raw_state: String,
    live_label: String,
    revert_at_unix_ms: u64,
    live_color: Option<[u8; 3]>,
}

static SERVER_RUNTIME: OnceLock<Mutex<Option<ServerRuntime>>> = OnceLock::new();
static SERVER_RUNNING: AtomicBool = AtomicBool::new(false);
static SERVER_PORT: AtomicU16 = AtomicU16::new(DEFAULT_ESP32_HA_PORT);
static SYNC_BUSY: AtomicBool = AtomicBool::new(false);
static SYNC_PCT: AtomicI32 = AtomicI32::new(100);
static SYNC_MSG: OnceLock<Mutex<String>> = OnceLock::new();
static LAST_EVENT_STR: OnceLock<Mutex<String>> = OnceLock::new();
static SLEEPING_STATE: AtomicBool = AtomicBool::new(false);
static LIVE_KEY_STATES: OnceLock<Mutex<HashMap<String, LiveKeyRuntimeState>>> = OnceLock::new();
static ENTITY_POLL_WAKEUP: AtomicBool = AtomicBool::new(false);

fn now_unix_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn live_key_states_store() -> &'static Mutex<HashMap<String, LiveKeyRuntimeState>> {
    LIVE_KEY_STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn sync_msg_store() -> &'static Mutex<String> {
    SYNC_MSG.get_or_init(|| Mutex::new("Complete!".to_string()))
}

fn last_event_store() -> &'static Mutex<String> {
    LAST_EVENT_STR.get_or_init(|| Mutex::new("none".to_string()))
}

fn set_sync_status(busy: bool, pct: i32, msg: impl Into<String>) {
    SYNC_BUSY.store(busy, Ordering::SeqCst);
    SYNC_PCT.store(pct.clamp(0, 100), Ordering::SeqCst);
    if let Ok(mut guard) = sync_msg_store().lock() {
        *guard = msg.into();
    }
}

pub fn is_esp32_ha_server_running() -> bool {
    SERVER_RUNNING.load(Ordering::SeqCst)
}

pub fn esp32_ha_server_port() -> u16 {
    let p = SERVER_PORT.load(Ordering::SeqCst);
    if p == 0 {
        load_esp32_ha_config().port.max(1024)
    } else {
        p
    }
}

pub fn esp32_ha_server_url() -> String {
    format!("http://127.0.0.1:{}/", esp32_ha_server_port())
}

/// Finds the serial number of any currently USB-connected duckyPad configured for `duckypad-esp32-ha`.
pub fn detect_esp32_ha_device_serial(
    active_profiles: &[DuckyProfile],
    active_serial: &str,
) -> Option<String> {
    let connected = crate::hardware::storage::list_connected_duckypads();
    let is_usb_connected = |sn: &str| {
        connected
            .iter()
            .any(|d| crate::hardware::storage::serial_matches_target(&d.serial, sn))
    };
    if is_esp32_ha_configured_profiles(active_profiles) {
        let clean = normalize_duckypad_serial(active_serial);
        if !clean.is_empty() && is_usb_connected(&clean) {
            return Some(clean);
        }
    }
    for sec in crate::kwin::auto_switcher::get_secondary_devices_snapshot() {
        if sec.usb_connected && is_esp32_ha_configured_profiles(&sec.profiles) {
            let clean = normalize_duckypad_serial(&sec.serial);
            if !clean.is_empty() {
                return Some(clean);
            }
        }
    }
    for t in list_manageable_duckypad_targets() {
        if t.usb_connected && t.is_esp32_ha_configured {
            let clean = normalize_duckypad_serial(&t.serial_number);
            if !clean.is_empty() {
                return Some(clean);
            }
        }
    }
    None
}

pub fn load_esp32_ha_bridge_config() -> PersistedEsp32HaConfig {
    load_esp32_ha_config()
}

pub fn start_esp32_ha_web_server(
    port: u16,
    target_serial: Option<String>,
) -> Result<String, String> {
    start_esp32_ha_server(port, target_serial)
}

pub fn stop_esp32_ha_web_server() {
    let mut cfg = load_esp32_ha_config();
    cfg.enabled = false;
    save_esp32_ha_config(&cfg);
    stop_esp32_ha_server();
}

pub fn start_esp32_ha_server(port: u16, target_serial: Option<String>) -> Result<String, String> {
    stop_esp32_ha_server();

    let bind_port = if port < 1024 {
        DEFAULT_ESP32_HA_PORT
    } else {
        port
    };

    let mut cfg = load_esp32_ha_config();
    cfg.enabled = true;
    cfg.port = bind_port;
    if let Some(ref s) = target_serial {
        let clean = normalize_duckypad_serial(s);
        if !clean.is_empty() {
            cfg.target_serial = clean;
        }
    }
    save_esp32_ha_config(&cfg);

    let addr = format!("0.0.0.0:{bind_port}");
    let listener = TcpListener::bind(&addr)
        .map_err(|e| format!("Failed binding ESP32-HA Web Server on {addr}: {e}"))?;
    let _ = listener.set_nonblocking(true);

    let stop_flag = Arc::new(AtomicBool::new(false));
    let thread_stop = stop_flag.clone();
    let poll_stop = stop_flag.clone();

    SERVER_PORT.store(bind_port, Ordering::SeqCst);
    SERVER_RUNNING.store(true, Ordering::SeqCst);

    let store = SERVER_RUNTIME.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = store.lock() {
        *guard = Some(ServerRuntime {
            stop_flag,
            port: bind_port,
        });
    }

    std::thread::spawn(move || {
        while !thread_stop.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _peer)) => {
                    let stop_inner = thread_stop.clone();
                    std::thread::spawn(move || {
                        if !stop_inner.load(Ordering::Relaxed) {
                            handle_http_connection(stream);
                        }
                    });
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(_) => {
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        }
    });

    std::thread::spawn(move || {
        ha_entity_poll_loop(poll_stop);
    });

    Ok(format!("http://127.0.0.1:{bind_port}/"))
}

pub fn stop_esp32_ha_server() {
    let store = SERVER_RUNTIME.get_or_init(|| Mutex::new(None));
    if let Ok(mut guard) = store.lock() {
        if let Some(rt) = guard.take() {
            rt.stop_flag.store(true, Ordering::SeqCst);
            let _ = TcpStream::connect(("127.0.0.1", rt.port));
        }
    }
    SERVER_RUNNING.load(Ordering::SeqCst);
    SERVER_RUNNING.store(false, Ordering::SeqCst);
}

fn handle_http_connection(mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(10)));

    let mut buf = Vec::with_capacity(4096);
    let mut temp = [0u8; 2048];
    let header_end = loop {
        match stream.read(&mut temp) {
            Ok(0) => return,
            Ok(n) => {
                buf.extend_from_slice(&temp[..n]);
                if let Some(pos) = find_header_end(&buf) {
                    break pos;
                }
                if buf.len() > 65536 {
                    let _ =
                        write_http_response(&mut stream, 400, "text/plain", b"Header too large");
                    return;
                }
            }
            Err(_) => return,
        }
    };

    let header_str = String::from_utf8_lossy(&buf[..header_end]).to_string();
    let mut lines = header_str.lines();
    let Some(req_line) = lines.next() else {
        return;
    };
    let mut parts = req_line.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_ascii_uppercase();
    let raw_target = parts.next().unwrap_or("/");
    let (path, query) = match raw_target.split_once('?') {
        Some((p, q)) => (p, q),
        None => (raw_target, ""),
    };

    let mut content_len = 0usize;
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                content_len = v.trim().parse::<usize>().unwrap_or(0);
            }
        }
    }

    let body_start = header_end + 4;
    while buf.len() < body_start + content_len && buf.len() < 131_072 {
        match stream.read(&mut temp) {
            Ok(0) => break,
            Ok(n) => buf.extend_from_slice(&temp[..n]),
            Err(_) => break,
        }
    }
    let body_bytes = if buf.len() >= body_start {
        let end = (body_start + content_len).min(buf.len());
        &buf[body_start..end]
    } else {
        &[][..]
    };
    let body_str = String::from_utf8_lossy(body_bytes);

    match (method.as_str(), path) {
        ("GET", "/") | ("GET", "/index.html") => {
            let html = get_esp32_ha_web_ui_html();
            let _ = write_http_response(
                &mut stream,
                200,
                "text/html; charset=utf-8",
                html.as_bytes(),
            );
        }
        ("GET", "/api/status") => {
            let resp = build_status_json();
            let _ = write_http_response(&mut stream, 200, "application/json", resp.as_bytes());
        }
        ("GET", "/api/profiles") => {
            let resp = build_profiles_json();
            let _ = write_http_response(&mut stream, 200, "application/json", resp.as_bytes());
        }
        ("POST", "/api/profiles") => {
            let resp = handle_profiles_post(&body_str);
            let _ = write_http_response(&mut stream, 200, "application/json", resp.as_bytes());
        }
        ("GET", "/api/ha_entity") => {
            let resp = handle_ha_entity_get(query);
            let _ = write_http_response(&mut stream, 200, "application/json", resp.as_bytes());
        }
        ("POST", "/api/control") => {
            let resp = handle_control_post(&body_str);
            let _ = write_http_response(&mut stream, 200, "application/json", resp.as_bytes());
        }
        ("GET", "/api/network") => {
            let cfg = load_esp32_ha_config();
            let resp = json!({
                "ssid": cfg.ssid,
                "mqtt_uri": cfg.mqtt_uri,
            })
            .to_string();
            let _ = write_http_response(&mut stream, 200, "application/json", resp.as_bytes());
        }
        ("POST", "/api/network") => {
            let mut cfg = load_esp32_ha_config();
            for line in body_str.lines() {
                if let Some(ssid) = line.strip_prefix("SSID|") {
                    if !ssid.trim().is_empty() {
                        cfg.ssid = ssid.trim().to_string();
                    }
                } else if let Some(mqtt) = line.strip_prefix("MQTT|") {
                    if !mqtt.trim().is_empty() {
                        cfg.mqtt_uri = mqtt.trim().to_string();
                    }
                }
            }
            save_esp32_ha_config(&cfg);
            let _ = write_http_response(&mut stream, 200, "application/json", br#"{"ok":true}"#);
        }
        _ => {
            let _ = write_http_response(
                &mut stream,
                404,
                "application/json",
                br#"{"ok":false,"error":"Not found"}"#,
            );
        }
    }
    let _ = stream.shutdown(Shutdown::Both);
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn write_http_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Server Error",
    };
    let hdr = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(hdr.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

struct ResolvedEsp32Target {
    serial: String,
    hw_rev: u8,
    model_name: String,
    fw_version: String,
    connected: bool,
    active_profile: u8,
    profiles: Vec<DuckyProfile>,
}

fn resolve_target_device() -> ResolvedEsp32Target {
    let cfg = load_esp32_ha_config();
    let targets = list_manageable_duckypad_targets();

    // Prefer:
    // 1. Explicitly configured target_serial in esp32_ha_bridge.json
    // 2. Any target marked is_esp32_ha_configured (especially duckyPad 2020)
    // 3. Any connected duckyPad (2020)
    // 4. First available target
    let chosen = targets
        .iter()
        .find(|t| {
            !cfg.target_serial.is_empty()
                && crate::hardware::storage::serial_matches_target(
                    &t.serial_number,
                    &cfg.target_serial,
                )
        })
        .or_else(|| {
            targets
                .iter()
                .find(|t| t.is_esp32_ha_configured && t.usb_connected)
        })
        .or_else(|| targets.iter().find(|t| t.is_esp32_ha_configured))
        .or_else(|| {
            targets
                .iter()
                .find(|t| t.hw_model == DP_MODEL_OG_DUCKYPAD && t.usb_connected)
        })
        .or_else(|| targets.first());

    if let Some(t) = chosen {
        let sn = normalize_duckypad_serial(&t.serial_number);
        let profiles = load_cached_profiles_for_serial_for_model(&sn, Some(t.hw_model))
            .or_else(|| {
                crate::kwin::auto_switcher::get_secondary_device_runtime(&sn)
                    .map(|sec| sec.profiles)
            })
            .or_else(|| crate::hardware::profile::load_profiles_for_connected_device(t.pid, &sn))
            .unwrap_or_default();
        let active_prof = if t.usb_connected {
            crate::hardware::storage::poll_hardware_active_profile_for_serial(Some(&sn))
                .unwrap_or(t.active_profile.max(1))
        } else {
            t.active_profile.max(1)
        };
        return ResolvedEsp32Target {
            serial: sn,
            hw_rev: t.hw_model,
            model_name: t.model_name.clone(),
            fw_version: t.firmware_version.clone(),
            connected: t.usb_connected,
            active_profile: active_prof,
            profiles,
        };
    }

    ResolvedEsp32Target {
        serial: "00000000".to_string(),
        hw_rev: DP_MODEL_OG_DUCKYPAD,
        model_name: "duckyPad (2020)".to_string(),
        fw_version: "1.3.2".to_string(),
        connected: false,
        active_profile: 1,
        profiles: Vec::new(),
    }
}

fn build_status_json() -> String {
    let cfg = load_esp32_ha_config();
    let target = resolve_target_device();
    let serial_u32 = u32::from_str_radix(&target.serial, 16).unwrap_or(0);
    let sync_msg = sync_msg_store()
        .lock()
        .map(|g| g.clone())
        .unwrap_or_else(|_| "Complete!".to_string());
    let last_evt = last_event_store()
        .lock()
        .map(|g| g.clone())
        .unwrap_or_else(|_| "none".to_string());

    json!({
        "connected": target.connected,
        "model": target.model_name,
        "hw_rev": target.hw_rev,
        "model_mode": cfg.model_mode,
        "fw": target.fw_version,
        "serial": serial_u32,
        "serial_hex": target.serial,
        "profile": target.active_profile,
        "profile_count": target.profiles.len(),
        "sleeping": SLEEPING_STATE.load(Ordering::Relaxed),
        "last_event": last_evt,
        "sync_busy": SYNC_BUSY.load(Ordering::Relaxed),
        "sync_pct": SYNC_PCT.load(Ordering::Relaxed),
        "sync_msg": sync_msg,
    })
    .to_string()
}

/// Converts UTF-8 smart-home icons & degree symbols into 7-bit printable ASCII (`0x21..0x7E`)
/// compatible with SSD1306 Font_6x10 / Font_6x8 on duckyPad 2020 & duckyPad Pro (matches `sanitize_oled_ascii` in `main.c`).
pub fn sanitize_oled_ascii(input: &str, max_len: usize) -> String {
    let mut out = String::with_capacity(max_len);
    for ch in input.chars() {
        if out.len() >= max_len {
            break;
        }
        match ch {
            '!'..='~' => out.push(ch),
            ' ' | '\t' => out.push('_'),
            '°' => {}
            '▶' => out.push('>'),
            '⏸' => {
                out.push('|');
                if out.len() < max_len {
                    out.push('|');
                }
            }
            '❄' => out.push('*'),
            '⚡' => out.push('!'),
            '💡' => out.push('o'),
            '🔒' => out.push('L'),
            '🔓' => out.push('U'),
            '🌡' => out.push('T'),
            '🔥' => out.push('^'),
            '🏠' => out.push('H'),
            _ => {}
        }
    }
    out
}

/// Resolves the static/defined label name for `(p_idx, k_idx, line1, line2)` (matches `resolve_key_defined_label` in `main.c`).
pub fn resolve_key_defined_label(p_idx: usize, k_idx: usize, line1: &str, line2: &str) -> String {
    let mut tmp: String;
    if let Some((pre, _)) = line1.split_once("{s}") {
        tmp = pre.trim_end_matches([':', '-', '_']).to_string();
        if tmp.is_empty() {
            if !line2.is_empty() && line2 != "-" {
                tmp = line2.chars().take(7).collect();
            } else {
                tmp = format!("P{}_K{}", p_idx + 1, k_idx + 1);
            }
        }
    } else if !line1.is_empty() {
        tmp = line1.chars().take(7).collect();
        if tmp.len() > 1 && tmp.ends_with([':', '-']) {
            tmp.pop();
        }
    } else if !line2.is_empty() && line2 != "-" {
        tmp = line2.chars().take(7).collect();
    } else {
        tmp = format!("P{}_K{}", p_idx + 1, k_idx + 1);
    }
    sanitize_oled_ascii(&tmp, 7)
}

/// Formats a Home Assistant entity state into a 7-char duckyPad OLED label (matches `format_entity_state_label` in `main.c`).
pub fn format_entity_state_label(tpl: &str, raw_state: &str) -> String {
    let st = if raw_state.eq_ignore_ascii_case("on") {
        "ON".to_string()
    } else if raw_state.eq_ignore_ascii_case("off") {
        "OFF".to_string()
    } else if raw_state.eq_ignore_ascii_case("unavailable")
        || raw_state.eq_ignore_ascii_case("unknown")
    {
        "N/A".to_string()
    } else {
        raw_state.chars().take(12).collect()
    };

    let tmp = if let Some((pre, post)) = tpl.split_once("{s}") {
        let pre_trunc: String = pre.chars().take(7).collect();
        format!("{pre_trunc}{st}{post}")
    } else if !tpl.is_empty() && tpl.ends_with([':', '-', '_']) {
        format!("{tpl}{st}")
    } else {
        st
    };

    sanitize_oled_ascii(&tmp, 7)
}

/// Matches an entity state string against a rule value (exact case-insensitive, numeric comparison, or wildcard `*`)
/// (matches `match_state_color_rule` in `main.c`).
pub fn match_state_color_rule(rule_val: &str, raw_state: &str) -> bool {
    let rv = rule_val.trim();
    if rv.is_empty() {
        return false;
    }
    if rv == "*" || rv.eq_ignore_ascii_case("default") || rv.eq_ignore_ascii_case(raw_state) {
        return true;
    }
    if rv.starts_with(['>', '<', '=', '!']) {
        let (op, num_str) = if let Some(rest) = rv.strip_prefix(">=") {
            (">=", rest)
        } else if let Some(rest) = rv.strip_prefix("<=") {
            ("<=", rest)
        } else if let Some(rest) = rv.strip_prefix("!=") {
            ("!=", rest)
        } else if let Some(rest) = rv.strip_prefix("==") {
            ("==", rest)
        } else {
            (&rv[..1], &rv[1..])
        };
        if let (Ok(sv), Ok(tv)) = (
            raw_state.trim().parse::<f64>(),
            num_str.trim().parse::<f64>(),
        ) {
            return match op {
                ">=" => sv >= tv,
                "<=" => sv <= tv,
                "!=" => (sv - tv).abs() > f64::EPSILON,
                "==" | "=" => (sv - tv).abs() <= f64::EPSILON,
                ">" => sv > tv,
                "<" => sv < tv,
                _ => false,
            };
        }
    }
    false
}

pub fn evaluate_key_state_color(
    rules: &[Esp32HaStateColorRule],
    raw_state: &str,
) -> Option<[u8; 3]> {
    for rule in rules.iter().take(MAX_STATE_COLORS) {
        if match_state_color_rule(&rule.v, raw_state) {
            return Some(rule.col);
        }
    }
    None
}

fn build_profiles_json() -> String {
    let cfg = load_esp32_ha_config();
    let target = resolve_target_device();
    let use_pinfo = target.hw_rev == DP_MODEL_DUCKYPAD_PRO;
    let now_ms = now_unix_ms();
    let live_states = live_key_states_store()
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default();

    let mut out_profiles = Vec::with_capacity(target.profiles.len());
    for (p_idx, prof) in target.profiles.iter().enumerate() {
        let kd = prof.keydown_color.unwrap_or([0, 255, 128]);
        let mut keys = Vec::with_capacity(DUCKYPAD_KEY_COUNT);
        for k_idx in 0..DUCKYPAD_KEY_COUNT {
            let slot_num = k_idx + 1;
            let slot_opt = prof.slots.get(&slot_num);
            let l1 = slot_opt.map(|s| s.label_line1.clone()).unwrap_or_default();
            let l2 = slot_opt.map(|s| s.label_line2.clone()).unwrap_or_default();
            let custom = slot_opt.and_then(|s| s.custom_rgb).is_some();
            let col = slot_opt.and_then(|s| s.custom_rgb).unwrap_or(prof.bg_color);
            let meta_key = format!("{}:{slot_num}", p_idx + 1);
            let meta = cfg.key_meta.get(&meta_key).cloned().unwrap_or_default();
            let live_rt = live_states.get(&meta_key);
            let live_eff = if !meta.entity_id.is_empty() {
                if let Some(rt) = live_rt {
                    if rt.revert_at_unix_ms > now_ms && !rt.live_label.is_empty() {
                        sanitize_oled_ascii(&rt.live_label, 7)
                    } else {
                        resolve_key_defined_label(p_idx, k_idx, &l1, &l2)
                    }
                } else {
                    resolve_key_defined_label(p_idx, k_idx, &l1, &l2)
                }
            } else {
                sanitize_oled_ascii(&l1, 7)
            };

            let mut key_obj = json!({
                "l1": l1,
                "l2": l2,
                "custom": custom,
                "col": col,
                "entity": meta.entity_id,
                "live": live_eff,
                "sc": meta.state_colors,
            });
            if let Some(live_col) = live_rt.and_then(|rt| rt.live_color) {
                if let Some(map) = key_obj.as_object_mut() {
                    map.insert("live_col".to_string(), json!(live_col));
                }
            }
            keys.push(key_obj);
        }
        out_profiles.push(json!({
            "name": prof.name,
            "bg": prof.bg_color,
            "kd": kd,
            "keys": keys,
        }));
    }

    json!({
        "model_mode": cfg.model_mode,
        "hw_rev": target.hw_rev,
        "use_pinfo": use_pinfo,
        "profiles": out_profiles,
    })
    .to_string()
}

fn default_bridge_script_for_slot(slot_num: usize, is_pro: bool) -> String {
    if !(1..=DUCKYPAD_KEY_COUNT).contains(&slot_num) {
        return String::new();
    }
    let ch = (b'a' + (slot_num as u8 - 1)) as char;
    let is_rotary_turn = matches!(slot_num, 21 | 22 | 24 | 25);
    if is_pro || is_rotary_turn {
        format!("STRING {ch}\n")
    } else {
        format!("EMUK {ch}\n")
    }
}

fn handle_profiles_post(body: &str) -> String {
    let mut cfg = load_esp32_ha_config();
    let target = resolve_target_device();
    let is_pro = target.hw_rev == DP_MODEL_DUCKYPAD_PRO;
    let mut profiles = target.profiles.clone();

    for raw_line in body.lines() {
        let line = raw_line.trim_end_matches('\r');
        if let Some(cnt_str) = line.strip_prefix("COUNT|") {
            if let Ok(cnt) = cnt_str.trim().parse::<usize>() {
                let clamped = cnt.clamp(1, 32);
                while profiles.len() < clamped {
                    let n = (profiles.len() + 1) as u8;
                    profiles.push(DuckyProfile {
                        number: n,
                        name: format!("HA_Page{n}"),
                        bg_color: [16, 48, 36],
                        keydown_color: Some([0, 255, 128]),
                        dim_unused_keys: false,
                        ..Default::default()
                    });
                }
                profiles.truncate(clamped);
            }
        } else if let Some(rest) = line.strip_prefix("P|") {
            let parts: Vec<&str> = rest.split('|').collect();
            if parts.len() >= 8 {
                if let Ok(p_idx) = parts[0].parse::<usize>() {
                    while profiles.len() <= p_idx && p_idx < 32 {
                        let n = (profiles.len() + 1) as u8;
                        profiles.push(DuckyProfile {
                            number: n,
                            name: format!("HA_Page{n}"),
                            ..Default::default()
                        });
                    }
                    if let Some(prof) = profiles.get_mut(p_idx) {
                        prof.number = (p_idx + 1) as u8;
                        prof.name = parts[1].chars().take(12).collect();
                        prof.bg_color = [
                            parts[2].parse().unwrap_or(16),
                            parts[3].parse().unwrap_or(48),
                            parts[4].parse().unwrap_or(36),
                        ];
                        prof.keydown_color = Some([
                            parts[5].parse().unwrap_or(0),
                            parts[6].parse().unwrap_or(255),
                            parts[7].parse().unwrap_or(128),
                        ]);
                        prof.dim_unused_keys = false;
                    }
                }
            }
        } else if let Some(rest) = line.strip_prefix("K|") {
            let parts: Vec<&str> = rest.split('|').collect();
            if parts.len() >= 8 {
                if let (Ok(p_idx), Ok(k_idx)) =
                    (parts[0].parse::<usize>(), parts[1].parse::<usize>())
                {
                    let slot_num = k_idx + 1;
                    if let Some(prof) = profiles.get_mut(p_idx) {
                        let l1 = if parts[2] == "-" { "" } else { parts[2] };
                        let l2 = if parts[3] == "-" { "" } else { parts[3] };
                        let cust = parts[4].parse::<i32>().unwrap_or(0) != 0;
                        let cr: u8 = parts[5].parse().unwrap_or(0);
                        let cg: u8 = parts[6].parse().unwrap_or(0);
                        let cb: u8 = parts[7].parse().unwrap_or(0);
                        let ent = if parts.len() >= 9 && parts[8] != "-" {
                            parts[8].trim()
                        } else {
                            ""
                        };

                        let max_slot = if is_pro { DUCKYPAD_KEY_COUNT } else { 15 };
                        if slot_num <= max_slot || !l1.is_empty() || !l2.is_empty() {
                            let existing_script = prof
                                .slots
                                .get(&slot_num)
                                .map(|s| s.on_press_script.clone())
                                .filter(|s| !s.trim().is_empty())
                                .unwrap_or_else(|| {
                                    default_bridge_script_for_slot(slot_num, is_pro)
                                });
                            let slot = prof.slots.entry(slot_num).or_insert_with(|| DuckyKeySlot {
                                slot: slot_num,
                                ..Default::default()
                            });
                            slot.slot = slot_num;
                            slot.label_line1 = l1.chars().take(7).collect();
                            slot.label_line2 = l2.chars().take(7).collect();
                            slot.custom_rgb = if cust { Some([cr, cg, cb]) } else { None };
                            slot.on_press_script = existing_script;
                        }

                        let meta_key = format!("{}:{slot_num}", p_idx + 1);
                        let entry = cfg.key_meta.entry(meta_key).or_default();
                        entry.entity_id = ent.to_string();
                        entry.state_colors.clear();
                    }
                }
            }
        } else if let Some(rest) = line.strip_prefix("S|") {
            let parts: Vec<&str> = rest.split('|').collect();
            if parts.len() >= 7 {
                if let (Ok(p_idx), Ok(k_idx)) =
                    (parts[0].parse::<usize>(), parts[1].parse::<usize>())
                {
                    let slot_num = k_idx + 1;
                    let meta_key = format!("{}:{slot_num}", p_idx + 1);
                    let entry = cfg.key_meta.entry(meta_key).or_default();
                    if entry.state_colors.len() < MAX_STATE_COLORS {
                        entry.state_colors.push(Esp32HaStateColorRule {
                            v: parts[3].chars().take(15).collect(),
                            col: [
                                parts[4].parse().unwrap_or(0),
                                parts[5].parse().unwrap_or(255),
                                parts[6].parse().unwrap_or(128),
                            ],
                        });
                    }
                }
            }
        }
    }

    save_esp32_ha_config(&cfg);
    ENTITY_POLL_WAKEUP.store(true, Ordering::Relaxed);

    let disk_label = format_disk_label_for_device(target.hw_rev, &target.serial);
    let meta = DuckySdDeviceMeta {
        serial_number: target.serial.clone(),
        hw_model: target.hw_rev,
        model_name: model_name_for_hw_model(target.hw_rev).to_string(),
        firmware_version: target.fw_version.clone(),
        disk_label: disk_label.clone(),
    };
    sync_profiles_to_device_cache_with_meta(&target.serial, &profiles, Some(&meta));
    crate::kwin::auto_switcher::update_secondary_device_profiles(&target.serial, profiles.clone());

    if target.connected && !SYNC_BUSY.load(Ordering::SeqCst) {
        let serial_for_sync = target.serial.clone();
        let hw_rev = target.hw_rev;
        let active_prof = target.active_profile;
        set_sync_status(true, 5, "Syncing duckyPad SD card...");
        std::thread::spawn(move || {
            let res = if hw_rev == DP_MODEL_OG_DUCKYPAD {
                crate::hardware::msc::write_duckypad_2020_profiles_via_hid_with_progress(
                    &disk_label,
                    &serial_for_sync,
                    &profiles,
                    active_prof,
                    |pct, step, _| {
                        set_sync_status(true, pct, step);
                    },
                )
            } else {
                crate::hardware::msc::write_duckypad_pro_profiles_via_msc_with_progress(
                    &disk_label,
                    &serial_for_sync,
                    &profiles,
                    active_prof,
                    |pct, step, _| {
                        set_sync_status(true, pct, step);
                    },
                )
            };
            match res {
                Ok(_) => {
                    mark_profiles_sd_synced(&serial_for_sync, &profiles);
                    set_sync_status(false, 100, "Complete!");
                }
                Err(e) => {
                    set_sync_status(false, 100, format!("Error: {e}"));
                }
            }
        });
    } else {
        set_sync_status(false, 100, "Saved to DuckyDeck Studio cache!");
    }

    r#"{"ok":true}"#.to_string()
}

fn fetch_ha_state_json(entity_id: &str) -> Option<serde_json::Value> {
    let ha_cfg = crate::actions::web_api::load_home_assistant_config();
    let base_url = if ha_cfg.base_url.trim().is_empty() {
        "http://192.168.1.205:8123".to_string()
    } else {
        ha_cfg.base_url.trim().trim_end_matches('/').to_string()
    };
    let token = ha_cfg.access_token.trim().to_string();
    if token.is_empty() {
        return None;
    }
    let url = format!("{base_url}/api/states/{entity_id}");
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .ok()
        .and_then(|rt| {
            rt.block_on(async move {
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_millis(2000))
                    .build()
                    .ok()?;
                let resp = client
                    .get(&url)
                    .bearer_auth(&token)
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .ok()?;
                if !resp.status().is_success() {
                    return None;
                }
                resp.json::<serde_json::Value>().await.ok()
            })
        })
}

fn ha_entity_poll_loop(stop_flag: Arc<AtomicBool>) {
    while !stop_flag.load(Ordering::Relaxed) {
        let cfg = load_esp32_ha_config();
        if !cfg.key_meta.is_empty() {
            let target = resolve_target_device();
            let serial_opt = if target.serial.is_empty() || target.serial == "00000000" {
                None
            } else {
                Some(target.serial.as_str())
            };
            let active_p_idx = (target.active_profile.max(1) as usize).saturating_sub(1);
            for (p_idx, prof) in target.profiles.iter().enumerate() {
                for k_idx in 0..DUCKYPAD_KEY_COUNT {
                    if stop_flag.load(Ordering::Relaxed) {
                        return;
                    }
                    let slot_num = k_idx + 1;
                    let meta_key = format!("{}:{slot_num}", p_idx + 1);
                    let Some(meta) = cfg.key_meta.get(&meta_key) else {
                        continue;
                    };
                    let ent_id = meta.entity_id.trim();
                    if ent_id.is_empty() {
                        continue;
                    }
                    let Some(val) = fetch_ha_state_json(ent_id) else {
                        continue;
                    };
                    let raw_state = val
                        .get("state")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let slot_opt = prof.slots.get(&slot_num);
                    let l1 = slot_opt.map(|s| s.label_line1.as_str()).unwrap_or("");
                    let l2 = slot_opt.map(|s| s.label_line2.as_str()).unwrap_or("");
                    let formatted = format_entity_state_label(l1, &raw_state);
                    let def_lbl = resolve_key_defined_label(p_idx, k_idx, l1, l2);
                    let matched_col = evaluate_key_state_color(&meta.state_colors, &raw_state);

                    let mut color_changed = false;
                    if let Ok(mut guard) = live_key_states_store().lock() {
                        let entry = guard.entry(meta_key.clone()).or_default();
                        if entry.raw_state != raw_state {
                            entry.raw_state = raw_state.clone();
                            entry.live_label = formatted.clone();
                            if formatted != def_lbl {
                                entry.revert_at_unix_ms = now_unix_ms() + STATE_LABEL_REVERT_MS;
                            } else {
                                entry.revert_at_unix_ms = 0;
                            }
                        }
                        if entry.live_color != matched_col {
                            entry.live_color = matched_col;
                            color_changed = true;
                        }
                    }

                    if color_changed && p_idx == active_p_idx && k_idx < 20 && target.connected {
                        let eff_rgb = matched_col
                            .or_else(|| slot_opt.and_then(|s| s.custom_rgb))
                            .unwrap_or(prof.bg_color);
                        send_raw_hid_cmd_to_serial(
                            serial_opt,
                            HID_CMD_SET_LED_SINGLE,
                            &[k_idx as u8, eff_rgb[0], eff_rgb[1], eff_rgb[2]],
                        );
                    }
                }
            }
        }

        ENTITY_POLL_WAKEUP.store(false, Ordering::Relaxed);
        for _ in 0..50 {
            if stop_flag.load(Ordering::Relaxed)
                || ENTITY_POLL_WAKEUP.swap(false, Ordering::Relaxed)
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

fn handle_ha_entity_get(query: &str) -> String {
    let mut entity_id = String::new();
    for pair in query.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == "id" {
                entity_id = url_decode(v);
            }
        }
    }
    let entity_id = entity_id.trim().to_string();
    if entity_id.is_empty() {
        return json!({
            "ok": false,
            "error": "Missing entity_id"
        })
        .to_string();
    }

    let ha_cfg = crate::actions::web_api::load_home_assistant_config();
    let token = ha_cfg.access_token.trim().to_string();
    let domain = entity_id.split('.').next().unwrap_or("").to_string();

    let default_states: Vec<String> = match domain.as_str() {
        "lock" => vec![
            "locked",
            "unlocked",
            "jammed",
            "locking",
            "unlocking",
            "unavailable",
        ],
        "cover" => vec!["open", "closed", "opening", "closing", "unavailable"],
        "media_player" => vec!["playing", "paused", "idle", "off", "standby", "unavailable"],
        "climate" => vec![
            "off",
            "heat",
            "cool",
            "heat_cool",
            "auto",
            "dry",
            "fan_only",
            "unavailable",
        ],
        "alarm_control_panel" => vec![
            "disarmed",
            "armed_home",
            "armed_away",
            "armed_night",
            "Pending",
            "triggered",
        ],
        "person" | "device_tracker" => vec!["home", "not_home", "unavailable"],
        "vacuum" => vec!["cleaning", "docked", "returning", "idle", "paused", "error"],
        "sensor" | "input_number" | "number" => vec![">=80", ">=50", "<50", "unavailable", "*"],
        _ => vec!["on", "off", "unavailable"],
    }
    .into_iter()
    .map(String::from)
    .collect();

    if token.is_empty() {
        return json!({
            "ok": false,
            "entity_id": entity_id,
            "domain": domain,
            "current_state": "",
            "states": default_states,
            "error": "Configure Home Assistant URL & Long-Lived Access Token in DuckyDeck Studio Settings"
        })
        .to_string();
    }

    if let Some(val) = fetch_ha_state_json(&entity_id) {
        let cur_state = val
            .get("state")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let mut states = Vec::new();
        if let Some(attrs) = val.get("attributes").and_then(|v| v.as_object()) {
            for key in ["options", "hvac_modes", "preset_modes", "fan_modes"] {
                if let Some(arr) = attrs.get(key).and_then(|v| v.as_array()) {
                    for item in arr {
                        if let Some(s) = item.as_str() {
                            states.push(s.to_string());
                        }
                    }
                    break;
                }
            }
        }
        for s in default_states {
            if !states.contains(&s) {
                states.push(s);
            }
        }
        return json!({
            "ok": true,
            "entity_id": entity_id,
            "domain": domain,
            "current_state": cur_state,
            "states": states,
        })
        .to_string();
    }

    json!({
        "ok": false,
        "entity_id": entity_id,
        "domain": domain,
        "current_state": "",
        "states": default_states,
    })
    .to_string()
}

fn handle_control_post(body: &str) -> String {
    let cmd = body.trim();
    let target = resolve_target_device();
    let serial_opt = if target.serial.is_empty() || target.serial == "00000000" {
        None
    } else {
        Some(target.serial.as_str())
    };

    if let Some(rest) = cmd.strip_prefix("goto_profile|") {
        if let Ok(p) = rest.trim().parse::<u8>() {
            let clamped = p.clamp(1, 64);
            let _ =
                crate::kwin::auto_switcher::send_hid_switch_profile_for_serial(serial_opt, clamped);
            crate::kwin::auto_switcher::update_secondary_device_active_profile(
                &target.serial,
                clamped,
            );
        }
    } else if cmd.starts_with("next_profile|") {
        let max_p = target.profiles.len().max(1) as u8;
        let next_p = if target.active_profile >= max_p {
            1
        } else {
            target.active_profile + 1
        };
        let _ = crate::kwin::auto_switcher::send_hid_switch_profile_for_serial(serial_opt, next_p);
        crate::kwin::auto_switcher::update_secondary_device_active_profile(&target.serial, next_p);
    } else if cmd.starts_with("prev_profile|") {
        let max_p = target.profiles.len().max(1) as u8;
        let prev_p = if target.active_profile <= 1 {
            max_p
        } else {
            target.active_profile - 1
        };
        let _ = crate::kwin::auto_switcher::send_hid_switch_profile_for_serial(serial_opt, prev_p);
        crate::kwin::auto_switcher::update_secondary_device_active_profile(&target.serial, prev_p);
    } else if cmd.starts_with("sleep|") {
        SLEEPING_STATE.store(true, Ordering::Relaxed);
        send_raw_hid_cmd_to_serial(serial_opt, 0x15, &[]);
    } else if cmd.starts_with("wake|") {
        SLEEPING_STATE.store(false, Ordering::Relaxed);
        send_raw_hid_cmd_to_serial(serial_opt, 0x16, &[]);
    } else if let Some(m) = cmd.strip_prefix("set_model_mode|") {
        let mut cfg = load_esp32_ha_config();
        cfg.model_mode = if m.starts_with("pro") || m.starts_with('2') {
            if m.starts_with("2020") {
                "2020".to_string()
            } else {
                "pro".to_string()
            }
        } else {
            "auto".to_string()
        };
        save_esp32_ha_config(&cfg);
    } else if let Some(rest) = cmd.strip_prefix("set_rgb|") {
        let parts: Vec<&str> = rest.split('|').collect();
        if parts.len() >= 4 {
            let k: u8 = parts[0].parse().unwrap_or(1);
            let r: u8 = parts[1].parse().unwrap_or(0);
            let g: u8 = parts[2].parse().unwrap_or(0);
            let b: u8 = parts[3].parse().unwrap_or(0);
            if (1..=20).contains(&k) {
                send_raw_hid_cmd_to_serial(serial_opt, HID_CMD_SET_LED_SINGLE, &[k - 1, r, g, b]);
                let p_num = target.active_profile.max(1);
                let meta_key = format!("{p_num}:{k}");
                if let Ok(mut guard) = live_key_states_store().lock() {
                    guard.entry(meta_key).or_default().live_color = Some([r, g, b]);
                }
            }
        }
    } else if let Some(rest) = cmd.strip_prefix("test_key|") {
        let parts: Vec<&str> = rest.split('|').collect();
        if parts.len() >= 2 {
            let prof: u8 = parts[0].parse().unwrap_or(1);
            let k: u8 = parts[1].parse().unwrap_or(1);
            if let Ok(mut guard) = last_event_store().lock() {
                *guard = format!("p{prof}_k{k}_press");
            }
            let meta_key = format!("{prof}:{k}");
            if let Ok(mut guard) = live_key_states_store().lock() {
                if let Some(st) = guard.get_mut(&meta_key) {
                    st.raw_state.clear();
                }
            }
            ENTITY_POLL_WAKEUP.store(true, Ordering::Relaxed);
        }
    }

    r#"{"ok":true}"#.to_string()
}

fn send_raw_hid_cmd_to_serial(target_serial: Option<&str>, cmd: u8, payload: &[u8]) {
    let _guard = crate::hardware::storage::HID_IO_LOCK.lock().ok();
    let _ = with_cached_duckypad_hid_for_serial(target_serial, |device| {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x55;
        tx[2] = cmd;
        let len = payload.len().min(60);
        tx[3..3 + len].copy_from_slice(&payload[..len]);
        let _ = device.write(&tx);
        let mut rx = [0u8; 64];
        let _ = crate::hardware::storage::read_hid_report4(device, &mut rx, 200);
        Ok(())
    });
}

fn url_decode(input: &str) -> String {
    let mut out = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(hex) = std::str::from_utf8(&bytes[i + 1..i + 3]) {
                if let Ok(val) = u8::from_str_radix(hex, 16) {
                    out.push(val);
                    i += 3;
                    continue;
                }
            }
        } else if bytes[i] == b'+' {
            out.push(b' ');
            i += 1;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_esp32_ha_web_ui_html_in_sync_with_firmware_repo() {
        let fw_header = PathBuf::from("/home/millinr/Builds/duckypad-esp32-ha/src/web_ui.h");
        if fw_header.exists() {
            let raw = fs::read_to_string(&fw_header).expect("read web_ui.h");
            let (_, after) = raw.split_once("R\"rawliteral(").expect("rawliteral start");
            let (fw_html, _) = after.rsplit_once(")rawliteral\"").expect("rawliteral end");
            assert_eq!(
                ESP32_HA_WEB_UI_HTML.trim(),
                fw_html.trim(),
                "data/esp32_ha_web_ui.html must stay in 100% sync with duckypad-esp32-ha/src/web_ui.h"
            );
        }
    }

    #[test]
    fn test_state_color_and_oled_label_parity_with_esp32_firmware() {
        assert_eq!(format_entity_state_label("Ofc:{s}", "on"), "Ofc:ON");
        assert_eq!(format_entity_state_label("{s}F", "72.4°"), "72.4F");
        assert_eq!(resolve_key_defined_label(0, 0, "Ofc:{s}", "Light"), "Ofc");
        assert!(match_state_color_rule(">=75", "78.2"));
        assert!(!match_state_color_rule(">=75", "71.0"));
        assert!(match_state_color_rule("on", "ON"));
        assert!(match_state_color_rule("*", "anything"));
    }
}
