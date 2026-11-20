use std::fs;
use std::path::{Path, PathBuf};

#[cfg(not(target_os = "windows"))]
use super::kwin_dbus::build_uncached_proxy;
#[cfg(target_os = "windows")]
use super::win32_backend::win32_window;

pub fn autostart_desktop_path() -> PathBuf {
    crate::paths::xdg_config_home()
        .join("autostart")
        .join("io.github.duckydeck.Studio.desktop")
}

pub fn is_kde_autostart_enabled() -> bool {
    #[cfg(target_os = "windows")]
    {
        return win32_window::is_registry_autostart_enabled();
    }

    #[cfg(not(target_os = "windows"))]
    {
        let path = autostart_desktop_path();
        let Ok(content) = fs::read_to_string(&path) else {
            return false;
        };
        for line in content.lines() {
            let t = line.trim();
            if t.eq_ignore_ascii_case("Hidden=true")
                || t.eq_ignore_ascii_case("X-GNOME-Autostart-enabled=false")
            {
                return false;
            }
        }
        true
    }
}

pub const DESKTOP_NOTIFICATIONS_NOTIFYRC: &str =
    include_str!("../../../data/io.github.duckydeck.Studio.notifyrc");

pub const GLOBAL_SHORTCUT_ACTIONS: &[(&str, &str, &str, &str)] = &[
    (
        "ToggleMainWindow",
        "Show / Hide DuckyDeck Studio",
        "--toggle",
        "toggle",
    ),
    ("ShowMainWindow", "Show Main Window", "--show", "show"),
    (
        "MinimizeToTray",
        "Minimize to System Tray",
        "--minimize",
        "hide",
    ),
    (
        "ToggleOledWidget",
        "Show / Hide Live OLED Companion",
        "--oled-widget",
        "toggle-oled-widget",
    ),
    (
        "ToggleAutoSwitcher",
        "Toggle Window Auto-Switcher",
        "--toggle-autoswitch",
        "auto-switch:toggle",
    ),
    (
        "NextProfile",
        "Switch to Next Profile",
        "--next-profile",
        "next-profile",
    ),
    (
        "PrevProfile",
        "Switch to Previous Profile",
        "--prev-profile",
        "prev-profile",
    ),
    (
        "SyncSdCard",
        "Sync Profiles to microSD Card",
        "--sync-sd",
        "sync-sd",
    ),
    (
        "SleepDevice",
        "Put duckyPad to Sleep",
        "--sleep",
        "sleep-device",
    ),
    ("WakeDevice", "Wake duckyPad", "--wake", "wake-device"),
    (
        "OpenAiArchitect",
        "Open AI Macro Architect",
        "--ai-architect",
        "open-ai-architect",
    ),
    (
        "OpenEventLog",
        "Open Device Event Log",
        "--event-log",
        "open-event-log",
    ),
    (
        "ArmWindowPicker",
        "Pick Active Window for Auto-Switcher",
        "--window-picker",
        "arm-window-picker",
    ),
    (
        "SwitchProfile1",
        "Switch to Profile 1",
        "--switch-profile 1",
        "switch-profile:1",
    ),
    (
        "SwitchProfile2",
        "Switch to Profile 2",
        "--switch-profile 2",
        "switch-profile:2",
    ),
    (
        "SwitchProfile3",
        "Switch to Profile 3",
        "--switch-profile 3",
        "switch-profile:3",
    ),
    (
        "SwitchProfile4",
        "Switch to Profile 4",
        "--switch-profile 4",
        "switch-profile:4",
    ),
];

pub fn global_shortcut_action_to_ipc_command(action_id: &str) -> Option<&'static str> {
    let trimmed = action_id.trim();
    if trimmed.is_empty() {
        return None;
    }
    for &(id, friendly, _, ipc) in GLOBAL_SHORTCUT_ACTIONS {
        if trimmed.eq_ignore_ascii_case(id) || trimmed.eq_ignore_ascii_case(friendly) {
            return Some(ipc);
        }
    }
    None
}

/// Pure helper to cycle to the next or previous profile number given a list of existing profile numbers.
pub fn cycle_profile_number(nums: &[u8], current: u8, next: bool) -> u8 {
    let clamped = current.clamp(1, 64);
    let mut sorted: Vec<u8> = nums
        .iter()
        .copied()
        .filter(|&n| (1..=64).contains(&n))
        .collect();
    sorted.sort_unstable();
    sorted.dedup();
    if sorted.is_empty() {
        if next {
            if clamped >= 64 {
                1
            } else {
                clamped + 1
            }
        } else if clamped <= 1 {
            64
        } else {
            clamped - 1
        }
    } else if let Some(pos) = sorted.iter().position(|&n| n == clamped) {
        if next {
            sorted[(pos + 1) % sorted.len()]
        } else if pos == 0 {
            sorted[sorted.len() - 1]
        } else {
            sorted[pos - 1]
        }
    } else {
        sorted[0]
    }
}

/// Validates that a D-Bus / CLI IPC command matches the expected single-line command surface.
pub fn is_allowed_ipc_command(raw_cmd: &str) -> bool {
    let cmd = raw_cmd.trim();
    if cmd.is_empty() || cmd.chars().any(char::is_control) {
        return false;
    }
    if matches!(
        cmd,
        "show"
            | "hide"
            | "toggle"
            | "toggle-window"
            | "show-oled-widget"
            | "hide-oled-widget"
            | "toggle-oled-widget"
            | "show-about"
            | "restart"
            | "reload"
            | "auto-switch:on"
            | "auto-switch:off"
            | "auto-switch:toggle"
            | "next-profile"
            | "prev-profile"
            | "sync-sd"
            | "sleep-device"
            | "wake-device"
            | "open-ai-architect"
            | "open-event-log"
            | "arm-window-picker"
    ) {
        return true;
    }
    if let Some(target) = cmd.strip_prefix("switch-profile:") {
        let t = target.trim();
        return !t.is_empty() && t.len() <= 64;
    }
    if let Some(path) = cmd
        .strip_prefix("import-group:")
        .or_else(|| cmd.strip_prefix("import-profile:"))
    {
        let p = path.trim();
        return !p.is_empty() && p.len() <= 4096;
    }
    false
}

pub(super) fn quote_desktop_exec_binary(raw_path: &str) -> String {
    let mut escaped = String::with_capacity(raw_path.len() + 4);
    escaped.push('"');
    for ch in raw_path.chars() {
        if matches!(ch, '"' | '`' | '$' | '\\') {
            escaped.push('\\');
        }
        escaped.push(ch);
    }
    escaped.push('"');
    escaped
}

pub fn generate_desktop_entry_content(exe_cmd: &str) -> String {
    let actions_list: String = GLOBAL_SHORTCUT_ACTIONS
        .iter()
        .map(|(id, _, _, _)| format!("{id};"))
        .collect();
    let mut out = format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=DuckyDeck Studio\n\
         Comment=KDE Plasma 6 Configurator & AI Macro Architect for duckyPad Pro\n\
         Exec={exe_cmd}\n\
         Icon=io.github.duckydeck.Studio\n\
         Terminal=false\n\
         Categories=Utility;HardwareSettings;Qt;KDE;\n\
         Keywords=duckyPad;macropad;duckyScript;mechanical;keyboard;macro;\n\
         StartupWMClass=io.github.duckydeck.Studio\n\
         X-GNOME-UsesNotifications=true\n\
         X-KDE-Notifications=io.github.duckydeck.Studio\n\
         Actions={actions_list}\n"
    );
    for &(id, friendly, cli_args, _) in GLOBAL_SHORTCUT_ACTIONS {
        out.push_str(&format!(
            "\n[Desktop Action {id}]\n\
             Name={friendly}\n\
             Exec={exe_cmd} {cli_args}\n"
        ));
    }
    out
}

pub fn parse_notifyrc_event_popup_enabled(notifyrc_content: &str, event_id: &str) -> bool {
    let target_section = format!("[Event/{}]", event_id.trim());
    let mut in_target = false;
    for line in notifyrc_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_target = trimmed.eq_ignore_ascii_case(&target_section);
            continue;
        }
        if in_target {
            if let Some((k, v)) = trimmed.split_once('=') {
                if k.trim().eq_ignore_ascii_case("Action") {
                    return v
                        .split('|')
                        .any(|part| part.trim().eq_ignore_ascii_case("Popup"));
                }
            }
        }
    }
    true
}

pub fn is_kde_notification_event_popup_enabled(event_id: &str) -> bool {
    let mut candidates =
        vec![crate::paths::xdg_config_home().join("io.github.duckydeck.Studio.notifyrc")];
    if let Some(home) = crate::paths::home_dir() {
        let host_cfg = home.join(".config/io.github.duckydeck.Studio.notifyrc");
        if !candidates.contains(&host_cfg) {
            candidates.push(host_cfg);
        }
    }
    for path in candidates {
        if let Ok(content) = fs::read_to_string(&path) {
            let section_header = format!("[Event/{}]", event_id.trim()).to_ascii_lowercase();
            if content.to_ascii_lowercase().contains(&section_header) {
                return parse_notifyrc_event_popup_enabled(&content, event_id);
            }
        }
    }
    true
}

#[cfg(not(target_os = "windows"))]
static LAST_KDE_NOTIFICATION_ID: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

#[cfg(not(target_os = "windows"))]
static SHARED_DBUS_SESSION: std::sync::LazyLock<tokio::sync::Mutex<Option<zbus::Connection>>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(None));

#[cfg(not(target_os = "windows"))]
pub(crate) async fn shared_dbus_session() -> Option<zbus::Connection> {
    let mut guard = SHARED_DBUS_SESSION.lock().await;
    if let Some(ref conn) = *guard {
        return Some(conn.clone());
    }
    match zbus::Connection::session().await {
        Ok(conn) => {
            *guard = Some(conn.clone());
            Some(conn)
        }
        Err(_) => None,
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) async fn invalidate_shared_dbus_session() {
    let mut guard = SHARED_DBUS_SESSION.lock().await;
    *guard = None;
}

#[cfg(not(target_os = "windows"))]
async fn try_send_kde_notification_dbus(
    ev: &str,
    summary: &str,
    message: &str,
    urg: u8,
    replaces_id: u32,
) -> Option<u32> {
    for attempt in 0..2 {
        let conn = shared_dbus_session().await?;
        let proxy = build_uncached_proxy(
            &conn,
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
        )
        .await;
        let Ok(proxy) = proxy else {
            if attempt == 0 {
                invalidate_shared_dbus_session().await;
                continue;
            }
            return None;
        };

        let mut hints: std::collections::HashMap<&str, zbus::zvariant::Value<'_>> =
            std::collections::HashMap::new();
        hints.insert(
            "desktop-entry",
            zbus::zvariant::Value::from("io.github.duckydeck.Studio"),
        );
        hints.insert(
            "x-kde-appname",
            zbus::zvariant::Value::from("io.github.duckydeck.Studio"),
        );
        hints.insert(
            "x-kde-display-appname",
            zbus::zvariant::Value::from("DuckyDeck Studio"),
        );
        hints.insert("x-kde-eventId", zbus::zvariant::Value::from(ev));
        hints.insert("urgency", zbus::zvariant::Value::U8(urg));

        let actions: Vec<&str> = Vec::new();
        match proxy
            .call::<_, _, u32>(
                "Notify",
                &(
                    "DuckyDeck Studio",
                    replaces_id,
                    "io.github.duckydeck.Studio",
                    summary,
                    message,
                    actions,
                    hints,
                    -1i32,
                ),
            )
            .await
        {
            Ok(returned_id) => return Some(returned_id),
            Err(_) if attempt == 0 => {
                invalidate_shared_dbus_session().await;
                continue;
            }
            Err(_) => return None,
        }
    }
    None
}

pub fn send_kde_notification(event_id: &str, title: &str, body: &str, urgency: u8) {
    if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1") {
        return;
    }
    if !is_kde_notification_event_popup_enabled(event_id) {
        return;
    }

    #[cfg(target_os = "windows")]
    {
        let _ = urgency;
        let qtitle = cxx_qt_lib::QString::from(title);
        let qbody = cxx_qt_lib::QString::from(body);
        crate::bridge::device_controller::qobject::duckydeck_show_tray_notification(
            &qtitle, &qbody,
        );
    }

    #[cfg(not(target_os = "windows"))]
    {
        let ev = event_id.trim().to_string();
        let summary = title.to_string();
        let message = body.to_string();
        let urg = urgency.min(2);
        let replaces_id = LAST_KDE_NOTIFICATION_ID.load(std::sync::atomic::Ordering::Relaxed);
        let fallback_title = summary.clone();
        let fallback_body = message.clone();
        let spawned = crate::actions::web_api::spawn_shared_async(async move {
            let sent_via_dbus = if let Some(returned_id) =
                try_send_kde_notification_dbus(&ev, &summary, &message, urg, replaces_id).await
            {
                LAST_KDE_NOTIFICATION_ID.store(returned_id, std::sync::atomic::Ordering::Relaxed);
                true
            } else {
                false
            };

            if !sent_via_dbus {
                let qtitle = cxx_qt_lib::QString::from(&summary);
                let qbody = cxx_qt_lib::QString::from(&message);
                crate::bridge::device_controller::qobject::duckydeck_show_tray_notification(
                    &qtitle, &qbody,
                );
            }
        });
        if !spawned {
            let qtitle = cxx_qt_lib::QString::from(&fallback_title);
            let qbody = cxx_qt_lib::QString::from(&fallback_body);
            crate::bridge::device_controller::qobject::duckydeck_show_tray_notification(
                &qtitle, &qbody,
            );
        }
    }
}

pub fn sync_local_desktop_and_icon(use_light_on_dark_icon: bool) {
    let data_home = crate::paths::xdg_data_home();

    let icon_dir = data_home
        .join("icons")
        .join("hicolor")
        .join("scalable")
        .join("apps");
    if fs::create_dir_all(&icon_dir).is_ok() {
        let svg_content = if use_light_on_dark_icon {
            include_str!("../../qml/icons/tray-icon-dark.svg")
        } else {
            include_str!("../../qml/icons/tray-icon-light.svg")
        };
        let icon_file = icon_dir.join("io.github.duckydeck.Studio.svg");
        if let Err(e) = fs::write(&icon_file, svg_content) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "CONFIG",
                format!("Failed writing local icon {}: {e}", icon_file.display()),
            );
        }
    }

    // Install KNotifications6 `.notifyrc` so DuckyDeck Studio appears under
    // KDE System Settings -> Notifications -> Application Settings.
    let mut notify_dirs = vec![data_home.join("knotifications6")];
    if let Some(home) = crate::paths::home_dir() {
        let host_notify_dir = home.join(".local/share/knotifications6");
        if !notify_dirs.contains(&host_notify_dir) {
            notify_dirs.push(host_notify_dir);
        }
    }
    for ndir in notify_dirs {
        if fs::create_dir_all(&ndir).is_ok() {
            let notify_file = ndir.join("io.github.duckydeck.Studio.notifyrc");
            let needs_write = match fs::read_to_string(&notify_file) {
                Ok(existing) => existing != DESKTOP_NOTIFICATIONS_NOTIFYRC,
                Err(_) => true,
            };
            if needs_write {
                let _ = fs::write(&notify_file, DESKTOP_NOTIFICATIONS_NOTIFYRC);
            }
        }
    }

    let is_flatpak = Path::new("/.flatpak-info").exists();
    let exe_cmd = if is_flatpak {
        "flatpak run io.github.duckydeck.Studio".to_string()
    } else if let Ok(appimage) = std::env::var("APPIMAGE") {
        quote_desktop_exec_binary(&appimage)
    } else if let Some(local_bin) = crate::paths::home_dir()
        .map(|h| h.join(".local/bin/duckydeck-studio"))
        .filter(|p| p.exists())
    {
        quote_desktop_exec_binary(&local_bin.to_string_lossy())
    } else {
        let raw_exe = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "duckydeck-studio".to_string());
        quote_desktop_exec_binary(&raw_exe)
    };

    let desktop_content = generate_desktop_entry_content(&exe_cmd);

    // Install into `~/.local/share/kglobalaccel/io.github.duckydeck.Studio.desktop`
    // so KDE System Settings -> Keyboard -> Shortcuts (`kglobalacceld` / `kcm_keys`)
    // discovers all DuckyDeck Studio global shortcut actions.
    let mut kglobal_dirs = vec![data_home.join("kglobalaccel")];
    if let Some(home) = crate::paths::home_dir() {
        let host_kglobal_dir = home.join(".local/share/kglobalaccel");
        if !kglobal_dirs.contains(&host_kglobal_dir) {
            kglobal_dirs.push(host_kglobal_dir);
        }
    }
    for kdir in kglobal_dirs {
        if fs::create_dir_all(&kdir).is_ok() {
            let kfile = kdir.join("io.github.duckydeck.Studio.desktop");
            let needs_write = match fs::read_to_string(&kfile) {
                Ok(existing) => existing != desktop_content,
                Err(_) => true,
            };
            if needs_write {
                let _ = fs::write(&kfile, &desktop_content);
            }
        }
    }

    // When running inside Flatpak, Flatpak itself exports io.github.duckydeck.Studio.desktop on the host
    if is_flatpak {
        return;
    }

    let apps_dir = data_home.join("applications");
    if fs::create_dir_all(&apps_dir).is_ok() {
        let desktop_file = apps_dir.join("io.github.duckydeck.Studio.desktop");
        if let Err(e) = fs::write(&desktop_file, &desktop_content) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "CONFIG",
                format!(
                    "Failed writing local desktop entry {}: {e}",
                    desktop_file.display()
                ),
            );
        }
    }
}

pub fn set_kde_autostart_enabled(enabled: bool) -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        return win32_window::set_registry_autostart_enabled(enabled);
    }

    #[cfg(not(target_os = "windows"))]
    {
        let path = autostart_desktop_path();
        if enabled {
            sync_local_desktop_and_icon(true);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed creating autostart directory: {e}"))?;
            }
            let exec_line = if Path::new("/.flatpak-info").exists() {
                "flatpak run io.github.duckydeck.Studio --autostart --minimized --foreground"
                    .to_string()
            } else if let Ok(appimage) = std::env::var("APPIMAGE") {
                format!("\"{appimage}\" --autostart --minimized --foreground")
            } else {
                let exe_path = if let Some(local_bin) = crate::paths::home_dir()
                    .map(|h| h.join(".local/bin/duckydeck-studio"))
                    .filter(|p| p.exists())
                {
                    local_bin.to_string_lossy().to_string()
                } else {
                    std::env::current_exe()
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| "duckydeck-studio".to_string())
                };
                format!("\"{exe_path}\" --autostart --minimized --foreground")
            };
            let desktop_entry = format!(
                "[Desktop Entry]\n\
             Type=Application\n\
             Name=DuckyDeck Studio\n\
             Comment=KDE Plasma 6 Configurator & Auto-Switcher for duckyPad Pro\n\
             Exec={exec_line}\n\
             Icon=io.github.duckydeck.Studio\n\
             Terminal=false\n\
             Categories=Utility;HardwareSettings;Qt;KDE;\n\
             StartupWMClass=io.github.duckydeck.Studio\n\
             X-GNOME-Autostart-enabled=true\n\
             X-KDE-StartupNotify=false\n\
             X-KDE-AutostartScript=false\n\
             X-KDE-autostart-after=panel\n"
            );
            fs::write(&path, desktop_entry)
                .map_err(|e| format!("Failed writing {}: {e}", path.display()))?;
            Ok(path)
        } else {
            if path.exists() {
                fs::remove_file(&path)
                    .map_err(|e| format!("Failed removing {}: {e}", path.display()))?;
            }
            Ok(path)
        }
    }
}
