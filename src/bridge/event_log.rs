use cxx_qt_lib::QString;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Mutex, OnceLock};

use crate::bridge::device_controller::{is_demo_showcase_env, qobject, APP_VERSION};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLogEntry {
    pub timestamp: String,
    pub level: String,
    pub category: String,
    pub message: String,
}

static SYSTEM_EVENT_LOG: OnceLock<Mutex<Vec<SystemLogEntry>>> = OnceLock::new();

pub fn get_system_log_file_path() -> PathBuf {
    let dir = crate::paths::data_dir();
    let _ = fs::create_dir_all(&dir); // ignore: best-effort log directory creation inside logger
    dir.join("duckydeck.log")
}

pub(crate) fn system_event_log_store() -> &'static Mutex<Vec<SystemLogEntry>> {
    SYSTEM_EVENT_LOG.get_or_init(|| Mutex::new(Vec::with_capacity(256)))
}

pub fn record_system_event(level: &str, category: &str, message: impl AsRef<str>) {
    let msg = message.as_ref().trim();
    if msg.is_empty() {
        return;
    }
    let ts = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string();
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
            let _ = fs::rename(&log_path, backup); // ignore: best-effort log rotation inside logger
        }
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = writeln!(f, "[{ts}] [{lvl:<5}] [{cat:<10}] {msg}"); // ignore: best-effort log file write inside logger
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

pub fn sync_event_log_properties_impl(mut controller: Pin<&mut qobject::DeviceController>) {
    let (json_str, count) = serialize_system_event_log_json();
    controller.as_mut().set_event_log_count(count);
    controller
        .as_mut()
        .set_event_log_json(QString::from(&json_str));
}

pub fn log_event_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
    level: QString,
    category: QString,
    message: QString,
) {
    let lvl = level.to_string();
    let cat = category.to_string();
    let msg = message.to_string();
    record_system_event(&lvl, &cat, &msg);
    sync_event_log_properties_impl(controller.as_mut());
}

pub fn record_status_log_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
    message: QString,
) {
    let msg = message.to_string();
    let trimmed = msg.trim();
    if trimmed.is_empty() {
        return;
    }
    if trimmed.starts_with("🌐 ")
        || trimmed.starts_with("🚀 ")
        || ((trimmed.starts_with("✓ ") || trimmed.starts_with("⚠ "))
            && (trimmed.contains("Test Web API")
                || trimmed.contains("Test Launch")
                || trimmed.contains(": HTTP ")
                || trimmed.contains(": Launched ")
                || trimmed.contains(": Failed to start ")))
    {
        sync_event_log_properties_impl(controller.as_mut());
        return;
    }
    let (lvl, cat) = classify_status_message(trimmed);
    record_system_event(lvl, cat, trimmed);
    sync_event_log_properties_impl(controller.as_mut());
}

pub fn refresh_event_log_impl(mut controller: Pin<&mut qobject::DeviceController>) -> QString {
    let (json_str, count) = serialize_system_event_log_json();
    controller.as_mut().set_event_log_count(count);
    controller
        .as_mut()
        .set_event_log_json(QString::from(&json_str));
    QString::from(&json_str)
}

pub fn clear_event_log_impl(mut controller: Pin<&mut qobject::DeviceController>) {
    if let Ok(mut guard) = system_event_log_store().lock() {
        guard.clear();
    }
    let log_path = get_system_log_file_path();
    if let Err(e) = fs::write(&log_path, "") {
        record_system_event(
            "WARN",
            "SYSTEM",
            format!("Failed truncating log file {}: {e}", log_path.display()),
        );
    }
    record_system_event("INFO", "SYSTEM", "Event log cleared by user");
    sync_event_log_properties_impl(controller.as_mut());
}

pub fn export_event_log_impl(
    mut controller: Pin<&mut qobject::DeviceController>,
    destination_path: QString,
) -> bool {
    let raw_str = destination_path.to_string();
    let trimmed = crate::paths::clean_file_uri_to_path_str(&raw_str);
    if trimmed.is_empty() {
        return false;
    }
    let mut dest = PathBuf::from(&trimmed);
    if dest.extension().is_none() {
        dest.set_extension("log");
    }
    let text = format_event_log_text_impl(
        QString::from("ALL"),
        QString::from("ALL"),
        QString::from(""),
    )
    .to_string();
    match fs::write(&dest, text) {
        Ok(_) => {
            let msg = format!("Exported system event log to {}", dest.display());
            record_system_event("INFO", "SYSTEM", &msg);
            controller.as_mut().set_status_message(QString::from(&msg));
            sync_event_log_properties_impl(controller.as_mut());
            true
        }
        Err(e) => {
            let err_msg = format!("Failed to export event log: {e}");
            record_system_event("ERROR", "SYSTEM", &err_msg);
            controller
                .as_mut()
                .set_status_message(QString::from(&err_msg));
            sync_event_log_properties_impl(controller.as_mut());
            false
        }
    }
}

pub fn format_event_log_text_impl(
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
