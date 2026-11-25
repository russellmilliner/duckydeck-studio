use std::fs;
#[cfg(not(target_os = "windows"))]
use std::io::Write;
#[cfg(not(target_os = "windows"))]
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
#[cfg(not(target_os = "windows"))]
use zbus::{interface, Result as ZbusResult};
#[cfg(target_os = "windows")]
pub type ZbusResult<T> = std::result::Result<T, String>;

#[cfg(not(target_os = "windows"))]
use super::kde_integration::{invalidate_shared_dbus_session, shared_dbus_session};
#[cfg(not(target_os = "windows"))]
use super::linux_backends::{
    latest_bridge_window, linux_desktop_window, record_last_bridge_window,
};
#[cfg(target_os = "windows")]
use super::win32_backend::win32_window;
use super::{
    dispatch_window_switch_to_secondary_devices, resolve_full_browser_caption,
    send_hid_switch_profile_checked, HidSwitchOutcome, SwitcherState, WindowChangeEvent,
};
#[cfg(not(target_os = "windows"))]
use super::{
    global_shortcut_action_to_ipc_command, is_allowed_ipc_command, GLOBAL_SHORTCUT_ACTIONS,
};

struct PendingWindowSwitch {
    raw_class: String,
    raw_name: String,
    raw_caption: String,
    window_entered_at: std::time::Instant,
    last_motion_at: std::time::Instant,
    settled_and_dispatched: bool,
}

impl Default for PendingWindowSwitch {
    fn default() -> Self {
        let now = std::time::Instant::now();
        Self {
            raw_class: String::new(),
            raw_name: String::new(),
            raw_caption: String::new(),
            window_entered_at: now,
            last_motion_at: now,
            settled_and_dispatched: true,
        }
    }
}

pub struct KWinBridgeService {
    state: Arc<Mutex<SwitcherState>>,
    ui_event_tx: mpsc::UnboundedSender<WindowChangeEvent>,
    pending: Arc<Mutex<PendingWindowSwitch>>,
    settle_notify: Arc<tokio::sync::Notify>,
}

impl KWinBridgeService {
    pub(crate) fn handle_active_window(
        &mut self,
        resource_class: String,
        resource_name: String,
        caption: String,
    ) {
        let mut effective_class = resource_class.trim().to_string();
        if effective_class.is_empty() {
            effective_class = resource_name.trim().to_string();
        }
        if effective_class.is_empty() {
            effective_class = "plasmashell".to_string();
        }

        let lower_class = effective_class.to_lowercase();
        if lower_class.contains("duckydeck") {
            return;
        }

        #[cfg(not(target_os = "windows"))]
        record_last_bridge_window(&effective_class, &resource_name, &caption);

        let (picker_armed, settle_ms, dwell_ms) = if let Ok(state) = self.state.lock() {
            (
                state.picker_armed,
                state.mouse_settle_ms.clamp(0, 5000),
                state.window_dwell_ms.clamp(0, 5000),
            )
        } else {
            return;
        };

        if picker_armed {
            return;
        }

        let now = std::time::Instant::now();
        let mut window_changed = false;
        let mut already_settled = false;

        if let Ok(mut pending) = self.pending.lock() {
            window_changed = pending.raw_class != effective_class || pending.raw_caption != caption;
            if window_changed {
                pending.raw_class = effective_class.clone();
                pending.raw_name = resource_name.clone();
                pending.raw_caption = caption.clone();
                pending.window_entered_at = now;
                pending.last_motion_at = now;
                pending.settled_and_dispatched = false;
            } else {
                pending.last_motion_at = now;
                already_settled = pending.settled_and_dispatched;
            }

            if settle_ms <= 0 && dwell_ms <= 0 && !pending.settled_and_dispatched {
                pending.settled_and_dispatched = true;
            }
        }

        if already_settled {
            return;
        }

        if settle_ms <= 0 && dwell_ms <= 0 {
            let resolved_caption = resolve_full_browser_caption(&effective_class, &caption);
            let mut matched_profile: Option<u8> = None;
            let mut hid_rejected_not_on_device = false;
            let mut paused = false;

            if let Ok(mut state) = self.state.lock() {
                paused = state.picker_armed || state.paused_for_editing;
                if !state.picker_armed {
                    state.current_window_class = effective_class.clone();
                    state.current_window_title = resolved_caption.clone();
                }
                if !state.picker_armed && state.enabled && !state.paused_for_editing {
                    let window_prof =
                        state.match_profile_for_window(&effective_class, &resolved_caption);
                    if let Some(wp) = window_prof {
                        state.last_window_matched_profile = Some(wp);
                    } else if state.active_modifier_override.is_none() {
                        state.last_window_matched_profile = None;
                    }

                    if state.active_modifier_override.is_none() {
                        matched_profile = window_prof;
                        if let Some(target) = matched_profile {
                            match send_hid_switch_profile_checked(target) {
                                HidSwitchOutcome::Switched => {
                                    state.last_switched_profile = target;
                                }
                                HidSwitchOutcome::NotLoadedOnHardware => {
                                    hid_rejected_not_on_device = true;
                                }
                                HidSwitchOutcome::DeviceUnavailable => {}
                            }
                        }
                    }
                }
            }

            dispatch_window_switch_to_secondary_devices(
                &effective_class,
                &resolved_caption,
                paused,
            );

            if let Err(e) = self.ui_event_tx.send(WindowChangeEvent {
                resource_class: effective_class,
                resource_name,
                caption: resolved_caption,
                matched_profile,
                picked: false,
                hid_rejected_not_on_device,
            }) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "KWIN",
                    format!("Failed sending window change event to UI: {e}"),
                );
            }
            return;
        }

        if window_changed {
            let resolved_caption = resolve_full_browser_caption(&effective_class, &caption);
            if let Err(e) = self.ui_event_tx.send(WindowChangeEvent {
                resource_class: effective_class,
                resource_name,
                caption: resolved_caption,
                matched_profile: None,
                picked: false,
                hid_rejected_not_on_device: false,
            }) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "KWIN",
                    format!("Failed sending window change preview event to UI: {e}"),
                );
            }
        }

        self.settle_notify.notify_one();
    }
}

#[cfg(not(target_os = "windows"))]
pub static KWIN_NOTIFY_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg(not(target_os = "windows"))]
pub static KWIN_HOOK_REGISTERED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(not(target_os = "windows"))]
#[interface(name = "io.github.duckydeck.KWinBridge")]
impl KWinBridgeService {
    fn notify_active_window(
        &mut self,
        resource_class: String,
        resource_name: String,
        caption: String,
    ) {
        KWIN_NOTIFY_SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.handle_active_window(resource_class, resource_name, caption);
    }

    fn notify_main_window_minimized(&mut self) {
        crate::bridge::device_controller::qobject::duckydeck_hide_main_window();
    }

    fn notify_main_window_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        crate::bridge::device_controller::qobject::duckydeck_record_main_geometry(
            x, y, width, height,
        );
    }

    fn notify_oled_widget_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        crate::bridge::device_controller::qobject::duckydeck_record_oled_geometry(
            x, y, width, height,
        );
    }

    fn notify_oled_widget_mapped(&mut self, is_pinned: bool) {
        apply_kwin_oled_keep_above(is_pinned, true);
    }

    fn dispatch_ipc_command(&mut self, command: String) -> u32 {
        let trimmed = command.trim();
        if is_allowed_ipc_command(trimmed) {
            let qcmd = cxx_qt_lib::QString::from(trimmed);
            crate::bridge::device_controller::qobject::duckydeck_dispatch_ipc_command(&qcmd);
        } else {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "IPC",
                format!(
                    "Ignored disallowed D-Bus IPC command ({} bytes)",
                    command.len()
                ),
            );
        }
        std::process::id()
    }

    fn invoke_global_action(&mut self, action_id: String) -> bool {
        if let Some(ipc_cmd) = global_shortcut_action_to_ipc_command(&action_id) {
            let qcmd = cxx_qt_lib::QString::from(ipc_cmd);
            crate::bridge::device_controller::qobject::duckydeck_dispatch_ipc_command(&qcmd);
            true
        } else {
            false
        }
    }

    fn get_pid(&self) -> u32 {
        std::process::id()
    }
}

#[cfg(not(target_os = "windows"))]
async fn register_and_listen_kde_global_shortcuts(conn: zbus::Connection) {
    // 1. Broadcast KNotification reparseConfiguration signal so System Settings -> Notifications
    //    reloads `~/.local/share/knotifications6/io.github.duckydeck.Studio.notifyrc` immediately.
    let _ = conn
        .emit_signal(
            None::<&str>,
            "/Config",
            "org.kde.knotification",
            "reparseConfiguration",
            &("io.github.duckydeck.Studio",),
        )
        .await;

    // 2. Register all global shortcut actions with `org.kde.kglobalaccel` (`/kglobalaccel`).
    if let Ok(kglobal_proxy) = build_uncached_proxy(
        &conn,
        "org.kde.kglobalaccel",
        "/kglobalaccel",
        "org.kde.KGlobalAccel",
    )
    .await
    {
        let empty_keys: Vec<i32> = Vec::new();
        for &(action_id, friendly_name, _, _) in GLOBAL_SHORTCUT_ACTIONS {
            let action_id_list: Vec<&str> = vec![
                "io.github.duckydeck.Studio.desktop",
                action_id,
                "DuckyDeck Studio",
                friendly_name,
            ];
            let _: std::result::Result<(), _> =
                kglobal_proxy.call("doRegister", &(&action_id_list,)).await;
            // SetPresent = 2 (loads existing user keybinding without overwriting it)
            let _: std::result::Result<Vec<i32>, _> = kglobal_proxy
                .call("setShortcut", &(&action_id_list, &empty_keys, 2u32))
                .await;
            let existing: Vec<i32> = kglobal_proxy
                .call("shortcut", &(&action_id_list,))
                .await
                .unwrap_or_default();
            if existing.is_empty() {
                let _: std::result::Result<(), _> = kglobal_proxy
                    .call("setForeignShortcut", &(&action_id_list, &empty_keys))
                    .await;
            }
        }
    }

    // 3. Subscribe to `globalShortcutPressed(QString componentUnique, QString actionUnique, qlonglong timestamp)`
    //    on `/component/io_github_duckydeck_Studio_desktop` (`org.kde.kglobalaccel.Component`).
    let Ok(comp_proxy) = build_uncached_proxy(
        &conn,
        "org.kde.kglobalaccel",
        "/component/io_github_duckydeck_Studio_desktop",
        "org.kde.kglobalaccel.Component",
    )
    .await
    else {
        return;
    };

    let Ok(mut stream) = comp_proxy.receive_signal("globalShortcutPressed").await else {
        return;
    };

    while let Some(msg) = std::future::poll_fn(|cx| {
        zbus::export::futures_core::Stream::poll_next(std::pin::Pin::new(&mut stream), cx)
    })
    .await
    {
        if let Ok((_component_unique, action_unique, _timestamp)) =
            msg.body().deserialize::<(String, String, i64)>()
        {
            if let Some(ipc_cmd) = global_shortcut_action_to_ipc_command(&action_unique) {
                crate::bridge::device_controller::record_system_event(
                    "INFO",
                    "SHORTCUT",
                    format!("KDE global shortcut triggered: {action_unique} -> {ipc_cmd}"),
                );
                let qcmd = cxx_qt_lib::QString::from(ipc_cmd);
                crate::bridge::device_controller::qobject::duckydeck_dispatch_ipc_command(&qcmd);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub(crate) async fn build_uncached_proxy<'a>(
    conn: &'a zbus::Connection,
    dest: &'a str,
    path: &'a str,
    iface: &'a str,
) -> zbus::Result<zbus::Proxy<'a>> {
    zbus::proxy::Builder::new(conn)
        .destination(dest)?
        .path(path)?
        .interface(iface)?
        .cache_properties(zbus::proxy::CacheProperties::No)
        .build()
        .await
}

static KWIN_HOOK_RELOAD_REQUESTED: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

#[cfg(not(target_os = "windows"))]
static KWIN_HOOK_RELOAD_NOTIFY: std::sync::LazyLock<tokio::sync::Notify> =
    std::sync::LazyLock::new(tokio::sync::Notify::new);

pub fn request_kwin6_hook_reload() {
    KWIN_HOOK_RELOAD_REQUESTED.store(true, std::sync::atomic::Ordering::SeqCst);
    #[cfg(not(target_os = "windows"))]
    {
        KWIN_HOOK_RELOAD_NOTIFY.notify_one();
    }
}

fn read_unfocused_hover_enabled(state: &Arc<Mutex<SwitcherState>>) -> bool {
    state
        .lock()
        .map(|s| s.auto_switch_unfocused_hover)
        .unwrap_or(false)
}

#[cfg(not(target_os = "windows"))]
pub async fn pick_window_via_kwin_dbus() -> std::result::Result<WindowChangeEvent, String> {
    let conn = zbus::Connection::session()
        .await
        .map_err(|e| format!("D-Bus session error: {e}"))?;

    // 1. Try native KWin interactive crosshair picker first (KDE Plasma 6 Wayland & X11)
    if let Ok(kwin_proxy) =
        build_uncached_proxy(&conn, "org.kde.KWin", "/KWin", "org.kde.KWin").await
    {
        if let Ok(info) = kwin_proxy
            .call::<_, _, std::collections::HashMap<String, zbus::zvariant::OwnedValue>>(
                "queryWindowInfo",
                &(),
            )
            .await
        {
            let extract_str = |key: &str| -> String {
                info.get(key)
                    .and_then(|v| <&str>::try_from(v).ok().map(|s| s.to_string()))
                    .unwrap_or_default()
            };

            let resource_class = extract_str("resourceClass");
            let resource_name = extract_str("resourceName");
            let raw_caption = extract_str("caption");

            if resource_class.is_empty() && resource_name.is_empty() {
                return Err("Window picker canceled".to_string());
            }

            let effective_class = if !resource_class.is_empty() {
                resource_class
            } else {
                resource_name.clone()
            };
            let caption = resolve_full_browser_caption(&effective_class, &raw_caption);

            return Ok(WindowChangeEvent {
                resource_class: effective_class,
                resource_name,
                caption,
                matched_profile: None,
                picked: true,
                hid_rejected_not_on_device: false,
            });
        }
    }

    // 2. Multi-Desktop Linux Window Picker fallback for:
    //    - Omarchy / Hyprland (`HYPRLAND_INSTANCE_SIGNATURE`)
    //    - COSMIC (`cosmic-comp` Wayland `zwlr_foreign_toplevel_manager_v1` / `zcosmic_toplevel_info_v1`)
    //    - GNOME (`mutter` Wayland & Xorg via D-Bus Bridge / Introspect)
    //    - Cinnamon / Linux Mint / MATE / Xfce (X11 `_NET_ACTIVE_WINDOW` & `XQueryPointer` click)
    linux_desktop_window::ensure_gnome_shell_extension_installed(&conn).await;
    linux_desktop_window::ensure_wayland_toplevel_watcher_started();

    let baseline_bridge_seq = latest_bridge_window().map(|w| w.3).unwrap_or(0);
    let baseline_wayland_seq = linux_desktop_window::latest_wayland_active_window()
        .map(|w| w.3)
        .unwrap_or(0);
    let mut dummy_hypr_cursor = None;
    let baseline_hypr =
        linux_desktop_window::query_hyprland_window_and_hover(&mut dummy_hypr_cursor)
            .and_then(|(act, _)| act);
    let mut dummy_x11_cursor = None;
    let (baseline_x11, _, _) =
        linux_desktop_window::query_x11_window_and_hover(&mut dummy_x11_cursor, false);
    let baseline_gnome = linux_desktop_window::query_gnome_introspect_active_window(&conn).await;

    // Wait up to 15 seconds (300 * 50ms) for the user to click/focus a target window.
    for tick in 0..300u32 {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let mut picked: Option<(String, String, String)> = None;

        // A. Check D-Bus Bridge notification (e.g. GNOME Shell extension or external notifier)
        if let Some((cls, rname, cap, seq)) = latest_bridge_window() {
            if seq != baseline_bridge_seq {
                picked = Some((cls, rname, cap));
            }
        }

        // B. Check Omarchy / Hyprland active window change or cursor click
        if picked.is_none() && linux_desktop_window::is_hyprland_session() {
            let mut cur = None;
            if let Some((Some(act), _)) =
                linux_desktop_window::query_hyprland_window_and_hover(&mut cur)
            {
                if Some(&act) != baseline_hypr.as_ref() {
                    picked = Some(act);
                }
            }
            // Also check X11 mouse click state on XWayland + Hyprland cursor hit-test
            let (_, _, b1_down) =
                linux_desktop_window::query_x11_window_and_hover(&mut dummy_x11_cursor, true);
            if picked.is_none() && b1_down && tick >= 3 {
                picked = linux_desktop_window::query_hyprland_window_at_current_cursor();
            }
        }

        // C. Check COSMIC / Wayland foreign-toplevel activation
        if picked.is_none() {
            if let Some((cls, rname, cap, seq)) =
                linux_desktop_window::latest_wayland_active_window()
            {
                if seq != baseline_wayland_seq {
                    picked = Some((cls, rname, cap));
                }
            }
        }

        // D. Check Cinnamon / Linux Mint / X11 cursor click (`Button1Mask`) or `_NET_ACTIVE_WINDOW` change
        if picked.is_none() {
            let (act_x11, hover_x11, b1_down) =
                linux_desktop_window::query_x11_window_and_hover(&mut dummy_x11_cursor, true);
            if b1_down && tick >= 3 {
                picked = hover_x11.or(act_x11);
            } else if act_x11.is_some() && act_x11 != baseline_x11 {
                picked = act_x11;
            }
        }

        // E. Check GNOME Shell Introspect focus change (every 150ms to keep D-Bus chatter minimal)
        if picked.is_none() && tick.is_multiple_of(3) {
            if let Some(act_gnome) =
                linux_desktop_window::query_gnome_introspect_active_window(&conn).await
            {
                if Some(&act_gnome) != baseline_gnome.as_ref() {
                    picked = Some(act_gnome);
                }
            }
        }

        if let Some((resource_class, resource_name, raw_caption)) = picked {
            let caption = resolve_full_browser_caption(&resource_class, &raw_caption);
            return Ok(WindowChangeEvent {
                resource_class,
                resource_name,
                caption,
                matched_profile: None,
                picked: true,
                hid_rejected_not_on_device: false,
            });
        }
    }

    Err("Window picker timed out (click the target window to focus it)".to_string())
}

pub async fn run_kwin_auto_switcher(
    state: Arc<Mutex<SwitcherState>>,
    ui_event_tx: mpsc::UnboundedSender<WindowChangeEvent>,
) -> ZbusResult<()> {
    if std::env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1") {
        return Ok(());
    }

    let pending = Arc::new(Mutex::new(PendingWindowSwitch::default()));
    let settle_notify = Arc::new(tokio::sync::Notify::new());

    {
        let state_bg = Arc::clone(&state);
        let pending_bg = Arc::clone(&pending);
        let notify_bg = Arc::clone(&settle_notify);
        let ui_tx_bg = ui_event_tx.clone();

        tokio::spawn(async move {
            loop {
                let maybe_deadline = {
                    let (settle_ms, dwell_ms) = if let Ok(st) = state_bg.lock() {
                        (
                            st.mouse_settle_ms.clamp(0, 5000) as u64,
                            st.window_dwell_ms.clamp(0, 5000) as u64,
                        )
                    } else {
                        (250, 150)
                    };

                    if let Ok(p) = pending_bg.lock() {
                        if p.settled_and_dispatched || p.raw_class.is_empty() {
                            None
                        } else {
                            let settle_dl =
                                p.last_motion_at + std::time::Duration::from_millis(settle_ms);
                            let dwell_dl =
                                p.window_entered_at + std::time::Duration::from_millis(dwell_ms);
                            Some(settle_dl.max(dwell_dl))
                        }
                    } else {
                        None
                    }
                };

                let Some(deadline) = maybe_deadline else {
                    notify_bg.notified().await;
                    continue;
                };

                let now = std::time::Instant::now();
                if now < deadline {
                    tokio::select! {
                        _ = tokio::time::sleep_until(tokio::time::Instant::from_std(deadline)) => {}
                        _ = notify_bg.notified() => {
                            continue;
                        }
                    }
                    continue;
                }

                // Deadline reached without newer cursor motion or window change!
                let dispatch_target = if let Ok(mut p) = pending_bg.lock() {
                    if p.settled_and_dispatched || p.raw_class.is_empty() {
                        None
                    } else {
                        p.settled_and_dispatched = true;
                        Some((
                            p.raw_class.clone(),
                            p.raw_name.clone(),
                            p.raw_caption.clone(),
                        ))
                    }
                } else {
                    None
                };

                if let Some((effective_class, resource_name, raw_caption)) = dispatch_target {
                    let resolved_caption =
                        resolve_full_browser_caption(&effective_class, &raw_caption);
                    let mut matched_profile: Option<u8> = None;
                    let mut hid_rejected_not_on_device = false;
                    let mut paused = false;

                    if let Ok(mut st) = state_bg.lock() {
                        paused = st.picker_armed || st.paused_for_editing;
                        if !st.picker_armed {
                            st.current_window_class = effective_class.clone();
                            st.current_window_title = resolved_caption.clone();
                        }
                        if !st.picker_armed && st.enabled && !st.paused_for_editing {
                            let window_prof =
                                st.match_profile_for_window(&effective_class, &resolved_caption);
                            if let Some(wp) = window_prof {
                                st.last_window_matched_profile = Some(wp);
                            } else if st.active_modifier_override.is_none() {
                                st.last_window_matched_profile = None;
                            }

                            if st.active_modifier_override.is_none() {
                                matched_profile = window_prof;
                                if let Some(target) = matched_profile {
                                    match send_hid_switch_profile_checked(target) {
                                        HidSwitchOutcome::Switched => {
                                            st.last_switched_profile = target;
                                        }
                                        HidSwitchOutcome::NotLoadedOnHardware => {
                                            hid_rejected_not_on_device = true;
                                        }
                                        HidSwitchOutcome::DeviceUnavailable => {}
                                    }
                                }
                            }
                        }
                    }

                    dispatch_window_switch_to_secondary_devices(
                        &effective_class,
                        &resolved_caption,
                        paused,
                    );

                    if let Err(e) = ui_tx_bg.send(WindowChangeEvent {
                        resource_class: effective_class,
                        resource_name,
                        caption: resolved_caption,
                        matched_profile,
                        picked: false,
                        hid_rejected_not_on_device,
                    }) {
                        crate::bridge::device_controller::record_system_event(
                            "WARN",
                            "KWIN",
                            format!("Failed sending settled window change event to UI: {e}"),
                        );
                    }
                }
            }
        });
    }

    #[cfg(target_os = "windows")]
    {
        let state_for_hover = Arc::clone(&state);
        let mut service = KWinBridgeService {
            state,
            ui_event_tx,
            pending,
            settle_notify,
        };
        let mut last_fg: Option<(String, String, String)> = None;
        let mut last_pt: Option<(i32, i32)> = None;
        let ipc_file = crate::paths::runtime_dir().join("duckydeck-ipc.cmd");
        let mut tick_counter: u32 = 0;
        loop {
            tick_counter = tick_counter.wrapping_add(1);
            if tick_counter.is_multiple_of(4) && ipc_file.is_file() {
                if let Ok(raw_cmd) = fs::read_to_string(&ipc_file) {
                    let _ = fs::remove_file(&ipc_file);
                    let trimmed = raw_cmd.trim();
                    if !trimmed.is_empty() {
                        let qcmd = cxx_qt_lib::QString::from(trimmed);
                        crate::bridge::device_controller::qobject::duckydeck_dispatch_ipc_command(
                            &qcmd,
                        );
                    }
                }
            }
            let unfocused_hover = read_unfocused_hover_enabled(&state_for_hover);
            // Poll active foreground window (and optional cursor hover window) on Windows every 50ms
            // SAFETY: GetForegroundWindow, GetCursorPos, and WindowFromPoint are thread-safe Win32 queries.
            let (fg_info, cursor_moved_info) = unsafe {
                let fg = win32_window::inspect_hwnd(win32_window::GetForegroundWindow());
                let mut hover = None;
                if unfocused_hover {
                    let mut pt = win32_window::POINT::default();
                    if win32_window::GetCursorPos(&mut pt) != 0 {
                        let cur_xy = (pt.x, pt.y);
                        if last_pt != Some(cur_xy) {
                            last_pt = Some(cur_xy);
                            hover = win32_window::inspect_hwnd(win32_window::WindowFromPoint(pt));
                        }
                    }
                } else {
                    last_pt = None;
                }
                (fg, hover)
            };
            if let Some((cls, rname, cap)) = cursor_moved_info {
                service.handle_active_window(cls, rname, cap);
            } else if fg_info != last_fg {
                last_fg = fg_info.clone();
                if let Some((cls, rname, cap)) = fg_info {
                    service.handle_active_window(cls, rname, cap);
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let pending_check = Arc::clone(&pending);
        let state_for_hook = Arc::clone(&state);
        let mut fallback_service = KWinBridgeService {
            state: Arc::clone(&state),
            ui_event_tx: ui_event_tx.clone(),
            pending: Arc::clone(&pending),
            settle_notify: Arc::clone(&settle_notify),
        };
        let service = KWinBridgeService {
            state,
            ui_event_tx,
            pending,
            settle_notify,
        };
        let conn = zbus::conn::Builder::session()?
            .name("io.github.duckydeck.Studio")?
            .serve_at("/KWinBridge", service)?
            .build()
            .await?;

        {
            let conn_for_shortcuts = conn.clone();
            tokio::spawn(async move {
                register_and_listen_kde_global_shortcuts(conn_for_shortcuts).await;
            });
        }

        let mut active_kwin_hook: Option<(String, i32)> = None;
        let mut kwin_scripting_opt: Option<zbus::Proxy<'_>> = None;
        let mut kwin_introspect_opt: Option<zbus::Proxy<'_>> = None;

        if !linux_desktop_window::is_hyprland_session() {
            let initial_unfocused_hover = read_unfocused_hover_enabled(&state_for_hook);
            if let Ok(script_path) = write_kwin6_hook_script(initial_unfocused_hover) {
                if let (Ok(kwin_scripting), Ok(kwin_introspect)) = (
                    build_uncached_proxy(
                        &conn,
                        "org.kde.KWin",
                        "/Scripting",
                        "org.kde.kwin.Scripting",
                    )
                    .await,
                    build_uncached_proxy(
                        &conn,
                        "org.kde.KWin",
                        "/Scripting",
                        "org.freedesktop.DBus.Introspectable",
                    )
                    .await,
                ) {
                    KWIN_HOOK_REGISTERED.store(true, std::sync::atomic::Ordering::SeqCst);
                    active_kwin_hook = register_kwin6_hook_script(
                        &conn,
                        &kwin_scripting,
                        &kwin_introspect,
                        &script_path,
                        &pending_check,
                        true,
                        false,
                    )
                    .await;
                    if active_kwin_hook.is_none() {
                        KWIN_HOOK_REGISTERED.store(false, std::sync::atomic::Ordering::SeqCst);
                    }
                    kwin_scripting_opt = Some(kwin_scripting);
                    kwin_introspect_opt = Some(kwin_introspect);
                }
            }
        }

        if let (
            Some((mut active_plugin, mut active_script_id)),
            Some(kwin_scripting),
            Some(kwin_introspect),
        ) = (active_kwin_hook, kwin_scripting_opt, kwin_introspect_opt)
        {
            loop {
                if !KWIN_HOOK_RELOAD_REQUESTED.load(std::sync::atomic::Ordering::SeqCst) {
                    tokio::select! {
                        _ = tokio::time::sleep(std::time::Duration::from_millis(30_000)) => {}
                        _ = KWIN_HOOK_RELOAD_NOTIFY.notified() => {}
                    }
                }
                let reload_requested =
                    KWIN_HOOK_RELOAD_REQUESTED.swap(false, std::sync::atomic::Ordering::SeqCst);

                let (is_loaded, node_alive) = if reload_requested {
                    (true, true)
                } else {
                    let loaded = kwin_scripting
                        .call::<_, _, bool>("isScriptLoaded", &(active_plugin.as_str(),))
                        .await
                        .unwrap_or(false);
                    let current_ids = introspect_kwin_script_ids_with_proxy(&kwin_introspect).await;
                    (loaded, current_ids.contains(&active_script_id))
                };

                if reload_requested || !is_loaded || !node_alive {
                    {
                        let _guard = KWIN_SCRIPT_LOCK.lock().await;
                        stop_and_unload_kwin_script(
                            &conn,
                            &kwin_scripting,
                            &active_plugin,
                            Some(active_script_id),
                        )
                        .await;
                    }
                    let unfocused_hover = read_unfocused_hover_enabled(&state_for_hook);
                    if let Ok(script_path) = write_kwin6_hook_script(unfocused_hover) {
                        if let Some((new_plugin, new_id)) = register_kwin6_hook_script(
                            &conn,
                            &kwin_scripting,
                            &kwin_introspect,
                            &script_path,
                            &pending_check,
                            false,
                            reload_requested,
                        )
                        .await
                        {
                            active_plugin = new_plugin;
                            active_script_id = new_id;
                        }
                    }
                }
            }
        }

        // Multi-Desktop Linux Auto-Switcher Loop for:
        // - Omarchy / Hyprland (`HYPRLAND_INSTANCE_SIGNATURE`)
        // - COSMIC (`cosmic-comp` Wayland `zwlr_foreign_toplevel_manager_v1` / `zcosmic_toplevel_info_v1`)
        // - GNOME (`mutter` Wayland & Xorg via auto-installed Shell extension + Introspect)
        // - Cinnamon / Linux Mint / MATE / Xfce (X11 `_NET_ACTIVE_WINDOW` & `XQueryPointer`)
        linux_desktop_window::ensure_gnome_shell_extension_installed(&conn).await;
        linux_desktop_window::ensure_wayland_toplevel_watcher_started();

        crate::bridge::device_controller::record_system_event(
            "INFO",
            "KWIN",
            "Multi-desktop Linux Auto-Switcher active (Omarchy/Hyprland, COSMIC, GNOME, Cinnamon/X11)",
        );

        let mut last_fg: Option<(String, String, String)> = None;
        let mut last_hypr_cursor: Option<(i64, i64)> = None;
        let mut last_x11_cursor: Option<(i32, i32)> = None;
        let mut last_wayland_seq: u64 = 0;
        let mut tick_counter: u32 = 0;

        loop {
            tick_counter = tick_counter.wrapping_add(1);
            let unfocused_hover = read_unfocused_hover_enabled(&state_for_hook);
            if !unfocused_hover {
                last_hypr_cursor = None;
                last_x11_cursor = None;
            }
            let mut fg_info: Option<(String, String, String)> = None;
            let mut hover_info: Option<(String, String, String)> = None;

            // 1. Omarchy / Hyprland IPC socket (`j/activewindow` + optional `j/cursorpos` + `j/clients`)
            if let Some((hypr_act, hypr_hover)) =
                linux_desktop_window::query_hyprland_window_and_hover(&mut last_hypr_cursor)
            {
                fg_info = hypr_act;
                if unfocused_hover {
                    hover_info = hypr_hover;
                }
            }

            // 2. COSMIC / wlroots / Sway / Niri Wayland foreign-toplevel watcher
            if fg_info.is_none() {
                if let Some((cls, rname, cap, seq)) =
                    linux_desktop_window::latest_wayland_active_window()
                {
                    if seq != last_wayland_seq {
                        last_wayland_seq = seq;
                    }
                    fg_info = Some((cls, rname, cap));
                }
            }

            // 3. Cinnamon / Linux Mint / MATE / Xfce / Xorg (`_NET_ACTIVE_WINDOW` + optional `XQueryPointer`)
            if fg_info.is_none() || (unfocused_hover && hover_info.is_none()) {
                let (x11_act, x11_hover, _) =
                    linux_desktop_window::query_x11_window_and_hover(&mut last_x11_cursor, false);
                if fg_info.is_none() {
                    fg_info = x11_act;
                }
                if unfocused_hover && hover_info.is_none() {
                    hover_info = x11_hover;
                }
            }

            // 4. GNOME Shell Introspect fallback (every 150ms when not already reported)
            if fg_info.is_none() && tick_counter.is_multiple_of(3) {
                fg_info = linux_desktop_window::query_gnome_introspect_active_window(&conn).await;
            }

            if let Some((cls, rname, cap)) = hover_info {
                fallback_service.handle_active_window(cls, rname, cap);
            } else if fg_info != last_fg {
                last_fg = fg_info.clone();
                if let Some((cls, rname, cap)) = fg_info {
                    fallback_service.handle_active_window(cls, rname, cap);
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn write_kwin6_hook_script(unfocused_hover: bool) -> std::io::Result<String> {
    let initial_rect_js = match load_saved_oled_geometry_tuple() {
        Some((gx, gy, gw, gh)) => {
            format!("{{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }}")
        }
        None => "null".to_string(),
    };
    let initial_main_rect_js = match load_saved_main_window_geometry_tuple() {
        Some((gx, gy, gw, gh)) => {
            format!("{{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }}")
        }
        None => "null".to_string(),
    };
    let js_code = generate_kwin6_hook_js(unfocused_hover, &initial_rect_js, &initial_main_rect_js);
    let file_path = crate::paths::kwin_runtime_dir()
        .join("duckydeck-kwin6-hook.js")
        .to_string_lossy()
        .to_string();

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(js_code.as_bytes())?;
    Ok(file_path)
}

#[cfg(not(target_os = "windows"))]
pub(super) fn generate_kwin6_hook_js(
    unfocused_hover: bool,
    initial_rect_js: &str,
    initial_main_rect_js: &str,
) -> String {
    let hover_functions_js = if unfocused_hover {
        r#"
var lastMotionSentMs = 0;

function isWindowOnCurrentDesktop(w) {
    if (!w) return false;
    if (w.onAllDesktops || w.desktopWindow || w.dock) return true;
    if (!workspace.currentDesktop || !w.desktops || w.desktops.length === 0) return true;
    for (var d = 0; d < w.desktops.length; ++d) {
        if (w.desktops[d] === workspace.currentDesktop) {
            return true;
        }
    }
    return false;
}

function topWindowAtPoint(pt) {
    if (!pt) return null;
    var order = workspace.stackingOrder || (workspace.windowList ? workspace.windowList() : null);
    if (!order || !order.length) return null;
    for (var i = order.length - 1; i >= 0; --i) {
        var w = order[i];
        if (!w || w.minimized || w.hidden || w.deleted) continue;
        if (w.tooltip || w.notification || w.criticalNotification || w.onScreenDisplay || w.dndIcon || w.splash) continue;
        if (!isWindowOnCurrentDesktop(w)) continue;
        var fg = w.frameGeometry;
        if (!fg || fg.width <= 0 || fg.height <= 0) continue;
        if (pt.x >= fg.x && pt.x < (fg.x + fg.width) && pt.y >= fg.y && pt.y < (fg.y + fg.height)) {
            return w;
        }
    }
    return null;
}

function onCursorPosChanged() {
    var now = Date.now();
    if ((now - lastMotionSentMs) < 120) return;
    lastMotionSentMs = now;
    var pt = workspace.cursorPos;
    if (!pt) return;
    var topWin = topWindowAtPoint(pt);
    if (!topWin || isDesktopOrPanelWindow(topWin)) {
        if (lastCursorKey !== "plasmashell::Desktop") {
            disconnectTrackedWindow();
            sendDesktop();
        }
        return;
    }
    if (isDuckyDeckWindow(topWin)) {
        lastCursorKey = "__duckydeck__";
        return;
    }
    var cls = effectiveWindowClass(topWin);
    var cap = topWin.caption ? topWin.caption.toString() : "";
    var key = cls + "::" + cap;
    if (lastCursorKey !== key) {
        onWindowActivated(topWin);
    }
}
"#
    } else {
        ""
    };

    let initial_hover_check_js = if unfocused_hover {
        r#"    if (workspace.cursorPos) {
        var underCursor = topWindowAtPoint(workspace.cursorPos);
        if (!underCursor || isDesktopOrPanelWindow(underCursor)) {
            sendDesktop();
            return;
        }
        if (!isDuckyDeckWindow(underCursor)) {
            onWindowActivated(underCursor);
            return;
        }
    }
"#
    } else {
        ""
    };

    let cursor_connect_js = if unfocused_hover {
        r#"if (workspace.cursorPosChanged) {
    workspace.cursorPosChanged.connect(onCursorPosChanged);
}
"#
    } else {
        ""
    };

    format!(
        r#"
var trackedWindow = null;
var hookedWindows = [];
var lastCursorKey = "";
var initialSavedOledRect = {initial_rect_js};
var initialSavedMainRect = {initial_main_rect_js};

function isWindowHooked(win) {{
    for (var i = 0; i < hookedWindows.length; ++i) {{
        if (hookedWindows[i] === win) {{
            return true;
        }}
    }}
    return false;
}}

function isOledWidgetWindow(win) {{
    if (!win) return false;
    var cap = win.caption ? win.caption.toString() : "";
    return cap.indexOf("duckyPad OLED —") === 0;
}}

function isDuckyDeckMainWindow(win) {{
    if (!win || isOledWidgetWindow(win)) return false;
    if (win.dialog || win.modal || win.transient || win.utility || win.splash || win.tooltip || win.notification) return false;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var name = (win.resourceName ? win.resourceName.toString() : "").toLowerCase();
    if (cls.indexOf("duckydeck") === -1 && name.indexOf("duckydeck") === -1) return false;
    var cap = win.caption ? win.caption.toString() : "";
    return cap.indexOf("DuckyDeck Studio v") === 0 || cap === "DuckyDeck Studio";
}}

function checkOledKeepAbove(win) {{
    if (!win) return;
    if (isOledWidgetWindow(win)) {{
        var cap = win.caption ? win.caption.toString() : "";
        var isPinned = (cap.indexOf("[Unpinned]") === -1);
        var isAllDesktops = (cap.indexOf("[SingleDesktop]") === -1);
        win.keepAbove = isPinned;
        win.onAllDesktops = isAllDesktops;
        win.skipTaskbar = isPinned;
        win.skipPager = isPinned;
        if (isPinned && win.minimized) {{
            win.minimized = false;
        }}
        if (!isWindowHooked(win)) {{
            hookedWindows.push(win);
            try {{
                if (win.closed) {{
                    win.closed.connect(function() {{
                        var idx = hookedWindows.indexOf(win);
                        if (idx !== -1) {{
                            hookedWindows.splice(idx, 1);
                        }}
                    }});
                }}
                var trackedOledX = initialSavedOledRect ? initialSavedOledRect.x : Math.round(win.frameGeometry.x);
                var trackedOledTopY = initialSavedOledRect ? initialSavedOledRect.y : Math.round(win.frameGeometry.y);
                var trackedOledW = initialSavedOledRect ? initialSavedOledRect.width : Math.round(win.frameGeometry.width);
                var trackedOledH = initialSavedOledRect ? initialSavedOledRect.height : Math.round(win.frameGeometry.height);
                var isAdjustingOledGeometry = false;
                if (initialSavedOledRect) {{
                    isAdjustingOledGeometry = true;
                    try {{
                        win.frameGeometry = {{
                            x: initialSavedOledRect.x,
                            y: initialSavedOledRect.y,
                            width: initialSavedOledRect.width,
                            height: initialSavedOledRect.height
                        }};
                    }} catch (e) {{}}
                    isAdjustingOledGeometry = false;
                    trackedOledX = initialSavedOledRect.x;
                    trackedOledTopY = initialSavedOledRect.y;
                    trackedOledW = initialSavedOledRect.width;
                    trackedOledH = initialSavedOledRect.height;
                }}
                if (isAllDesktops) {{
                    win.onAllDesktops = false;
                    win.onAllDesktops = true;
                }} else {{
                    win.onAllDesktops = false;
                }}
                win.keepAbove = isPinned;
                callDBus(
                    "io.github.duckydeck.Studio",
                    "/KWinBridge",
                    "io.github.duckydeck.KWinBridge",
                    "NotifyOledWidgetMapped",
                    isPinned
                );
                if (win.minimizedChanged) {{
                    win.minimizedChanged.connect(function() {{
                        var c = win.caption ? win.caption.toString() : "";
                        var pinnedNow = (c.indexOf("[Unpinned]") === -1);
                        if (pinnedNow && win.minimized) {{
                            win.minimized = false;
                            win.keepAbove = true;
                        }}
                    }});
                }}
                if (win.captionChanged) {{
                    win.captionChanged.connect(function() {{
                        checkOledKeepAbove(win);
                    }});
                }}
                if (win.moveResizedChanged) {{
                    win.moveResizedChanged.connect(function() {{
                        if (win.frameGeometry && (win.move || win.resize)) {{
                            trackedOledX = Math.round(win.frameGeometry.x);
                            trackedOledTopY = Math.round(win.frameGeometry.y);
                            trackedOledW = Math.round(win.frameGeometry.width);
                            trackedOledH = Math.round(win.frameGeometry.height);
                        }}
                    }});
                }}
                if (win.frameGeometryChanged) {{
                    win.frameGeometryChanged.connect(function() {{
                        if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 180 && win.frameGeometry.height >= 220) {{
                            if (isAdjustingOledGeometry) return;

                            var curX = Math.round(win.frameGeometry.x);
                            var curY = Math.round(win.frameGeometry.y);
                            var curW = Math.round(win.frameGeometry.width);
                            var curH = Math.round(win.frameGeometry.height);

                            var onlyHeightChanged = (curX === trackedOledX && curW === trackedOledW && curH !== trackedOledH);
                            if (win.move || win.resize || !onlyHeightChanged) {{
                                trackedOledTopY = curY;
                            }} else if (trackedOledTopY !== null && Math.abs(curY - trackedOledTopY) >= 1) {{
                                isAdjustingOledGeometry = true;
                                try {{
                                    win.frameGeometry = {{
                                        x: curX,
                                        y: trackedOledTopY,
                                        width: curW,
                                        height: curH
                                    }};
                                    curY = trackedOledTopY;
                                }} catch (e) {{}}
                                isAdjustingOledGeometry = false;
                            }} else if (trackedOledTopY === null) {{
                                trackedOledTopY = curY;
                            }}

                            trackedOledX = curX;
                            trackedOledW = curW;
                            trackedOledH = curH;
                            initialSavedOledRect = {{
                                x: curX,
                                y: curY,
                                width: curW,
                                height: curH
                            }};

                            callDBus(
                                "io.github.duckydeck.Studio",
                                "/KWinBridge",
                                "io.github.duckydeck.KWinBridge",
                                "NotifyOledWidgetGeometry",
                                curX,
                                curY,
                                curW,
                                curH
                            );
                        }}
                    }});
                }}
            }} catch (e) {{}}
        }}
        return;
    }}
"#
    ) + r#"
    if (isDuckyDeckMainWindow(win)) {
        if (!isWindowHooked(win)) {
            hookedWindows.push(win);
            try {
                if (win.closed) {
                    win.closed.connect(function() {
                        var idx = hookedWindows.indexOf(win);
                        if (idx !== -1) {
                            hookedWindows.splice(idx, 1);
                        }
                    });
                }
                var mainMappedAtMs = Date.now();
                var isAdjustingMainGeometry = false;
                if (initialSavedMainRect && win.frameGeometry) {
                    isAdjustingMainGeometry = true;
                    try {
                        var curW0 = Math.round(win.frameGeometry.width);
                        var curH0 = Math.round(win.frameGeometry.height);
                        var targetW0 = curW0 >= 600 ? curW0 : initialSavedMainRect.width;
                        var targetH0 = (initialSavedMainRect.height && initialSavedMainRect.height >= 680)
                            ? initialSavedMainRect.height
                            : (curH0 >= 400 ? curH0 : 800);
                        win.frameGeometry = {
                            x: initialSavedMainRect.x,
                            y: initialSavedMainRect.y,
                            width: targetW0,
                            height: targetH0
                        };
                    } catch (e) {}
                    isAdjustingMainGeometry = false;
                } else if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 600 && win.frameGeometry.height >= 400) {
                    var initX = Math.round(win.frameGeometry.x);
                    var initY = Math.round(win.frameGeometry.y);
                    var initW = Math.round(win.frameGeometry.width);
                    var initH = Math.round(win.frameGeometry.height);
                    initialSavedMainRect = {
                        x: initX,
                        y: initY,
                        width: initW,
                        height: initH
                    };
                    callDBus(
                        "io.github.duckydeck.Studio",
                        "/KWinBridge",
                        "io.github.duckydeck.KWinBridge",
                        "NotifyMainWindowGeometry",
                        initX,
                        initY,
                        initW,
                        initH
                    );
                }
                if (win.minimizedChanged) {
                    win.minimizedChanged.connect(function() {
                        if (win.minimized) {
                            win.skipTaskbar = true;
                            win.skipPager = true;
                            win.skipSwitcher = true;
                            callDBus(
                                "io.github.duckydeck.Studio",
                                "/KWinBridge",
                                "io.github.duckydeck.KWinBridge",
                                "NotifyMainWindowMinimized"
                            );
                        } else {
                            win.skipTaskbar = false;
                            win.skipPager = false;
                            win.skipSwitcher = false;
                            mainMappedAtMs = Date.now();
                            if (initialSavedMainRect && win.frameGeometry) {
                                isAdjustingMainGeometry = true;
                                try {
                                    win.frameGeometry = {
                                        x: initialSavedMainRect.x,
                                        y: initialSavedMainRect.y,
                                        width: Math.round(win.frameGeometry.width),
                                        height: Math.round(win.frameGeometry.height)
                                    };
                                } catch (e) {}
                                isAdjustingMainGeometry = false;
                            }
                        }
                    });
                }
                if (win.frameGeometryChanged) {
                    win.frameGeometryChanged.connect(function() {
                        if (!win.minimized && win.frameGeometry && win.frameGeometry.width >= 600 && win.frameGeometry.height >= 400) {
                            if (isAdjustingMainGeometry) return;

                            var curX = Math.round(win.frameGeometry.x);
                            var curY = Math.round(win.frameGeometry.y);
                            var curW = Math.round(win.frameGeometry.width);
                            var curH = Math.round(win.frameGeometry.height);

                            if (!win.move && !win.resize && initialSavedMainRect && (Date.now() - mainMappedAtMs) < 800) {
                                if (Math.abs(curX - initialSavedMainRect.x) >= 1 || Math.abs(curY - initialSavedMainRect.y) >= 1) {
                                    isAdjustingMainGeometry = true;
                                    try {
                                        win.frameGeometry = {
                                            x: initialSavedMainRect.x,
                                            y: initialSavedMainRect.y,
                                            width: curW,
                                            height: curH
                                        };
                                        curX = initialSavedMainRect.x;
                                        curY = initialSavedMainRect.y;
                                    } catch (e) {}
                                    isAdjustingMainGeometry = false;
                                }
                            }

                            initialSavedMainRect = {
                                x: curX,
                                y: curY,
                                width: curW,
                                height: curH
                            };

                            callDBus(
                                "io.github.duckydeck.Studio",
                                "/KWinBridge",
                                "io.github.duckydeck.KWinBridge",
                                "NotifyMainWindowGeometry",
                                curX,
                                curY,
                                curW,
                                curH
                            );
                        }
                    });
                }
            } catch (e) {}
        }
        if (win.minimized) {
            win.skipTaskbar = true;
            win.skipPager = true;
            win.skipSwitcher = true;
            callDBus(
                "io.github.duckydeck.Studio",
                "/KWinBridge",
                "io.github.duckydeck.KWinBridge",
                "NotifyMainWindowMinimized"
            );
        } else {
            win.skipTaskbar = false;
            win.skipPager = false;
            win.skipSwitcher = false;
        }
    }
}

function isDuckyDeckWindow(win) {
    if (!win) return false;
    checkOledKeepAbove(win);
    if (isOledWidgetWindow(win) || isDuckyDeckMainWindow(win)) return true;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var name = (win.resourceName ? win.resourceName.toString() : "").toLowerCase();
    return cls.indexOf("duckydeck") !== -1 || name.indexOf("duckydeck") !== -1;
}

function isDesktopOrPanelWindow(win) {
    if (!win) return true;
    if (win.desktopWindow || win.dock) return true;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    var name = (win.resourceName ? win.resourceName.toString() : "").toLowerCase();
    return cls === "plasmashell" || cls === "org.kde.plasmashell" || name === "plasmashell";
}

function effectiveWindowClass(win) {
    if (!win || isDesktopOrPanelWindow(win)) return "plasmashell";
    var cls = win.resourceClass ? win.resourceClass.toString().trim() : "";
    if (cls !== "") return cls;
    var name = win.resourceName ? win.resourceName.toString().trim() : "";
    if (name !== "") return name;
    var df = win.desktopFileName ? win.desktopFileName.toString().trim() : "";
    if (df !== "") return df;
    return "unmatched-window";
}

function disconnectTrackedWindow() {
    if (trackedWindow) {
        try {
            if (trackedWindow.captionChanged) {
                trackedWindow.captionChanged.disconnect(onTrackedCaptionChanged);
            }
        } catch (e) {}
        trackedWindow = null;
    }
}

function sendDesktop() {
    lastCursorKey = "plasmashell::Desktop";
    callDBus(
        "io.github.duckydeck.Studio",
        "/KWinBridge",
        "io.github.duckydeck.KWinBridge",
        "NotifyActiveWindow",
        "plasmashell",
        "plasmashell",
        "Desktop"
    );
}

function sendWindow(win) {
    if (!win || isDesktopOrPanelWindow(win)) {
        sendDesktop();
        return;
    }
    var cls = effectiveWindowClass(win);
    var rname = win.resourceName ? win.resourceName.toString() : cls;
    var cap = win.caption ? win.caption.toString() : "";
    lastCursorKey = cls + "::" + cap;
    callDBus(
        "io.github.duckydeck.Studio",
        "/KWinBridge",
        "io.github.duckydeck.KWinBridge",
        "NotifyActiveWindow",
        cls,
        rname,
        cap
    );
}

function onTrackedCaptionChanged() {
    if (workspace.activeWindow && isDuckyDeckWindow(workspace.activeWindow)) {
        return;
    }
    if (trackedWindow && !isDuckyDeckWindow(trackedWindow)) {
        sendWindow(trackedWindow);
    }
}

function onWindowActivated(win) {
    if (!win) {
        disconnectTrackedWindow();
        sendDesktop();
        return;
    }
    if (isDuckyDeckWindow(win)) {
        disconnectTrackedWindow();
        lastCursorKey = "__duckydeck__";
        return;
    }
    if (isDesktopOrPanelWindow(win)) {
        disconnectTrackedWindow();
        sendDesktop();
        return;
    }
    if (trackedWindow !== win) {
        if (trackedWindow) {
            disconnectTrackedWindow();
        }
        trackedWindow = win;
        try {
            if (trackedWindow.captionChanged) {
                trackedWindow.captionChanged.connect(onTrackedCaptionChanged);
            }
        } catch (e) {}
    }
    sendWindow(win);
}
"# + hover_functions_js
        + r#"
function sendInitialTopWindow() {
    var order = workspace.stackingOrder || (workspace.windowList ? workspace.windowList() : null);
    if (order && order.length) {
        for (var j = 0; j < order.length; ++j) {
            checkOledKeepAbove(order[j]);
        }
    }
"# + initial_hover_check_js
        + r#"    if (workspace.activeWindow && !isDuckyDeckWindow(workspace.activeWindow)) {
        onWindowActivated(workspace.activeWindow);
        return;
    }
    if (order && order.length) {
        for (var i = order.length - 1; i >= 0; --i) {
            var w = order[i];
            if (!isDuckyDeckWindow(w) && (!w.minimized)) {
                onWindowActivated(w);
                return;
            }
        }
    }
    sendDesktop();
}

function onWindowAdded(win) {
    if (win) {
        var idx = hookedWindows.indexOf(win);
        if (idx !== -1) {
            hookedWindows.splice(idx, 1);
        }
    }
    checkOledKeepAbove(win);
}

if (workspace.windowAdded) {
    workspace.windowAdded.connect(onWindowAdded);
}
"# + cursor_connect_js
        + r#"if (workspace.windowActivated) {
    workspace.windowActivated.connect(onWindowActivated);
    sendInitialTopWindow();
} else if (workspace.clientActivated) {
    workspace.clientActivated.connect(onWindowActivated);
    onWindowActivated(workspace.activeClient);
}
"#
}

#[cfg(not(target_os = "windows"))]
fn load_saved_main_window_geometry_tuple() -> Option<(i32, i32, i32, i32)> {
    let path = crate::paths::config_dir().join("main_window_geometry.json");
    let raw = fs::read_to_string(path).ok()?;
    let val: serde_json::Value = serde_json::from_str(&raw).ok()?;
    if !val
        .get("has_position")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return None;
    }
    let x = val.get("x").and_then(|v| v.as_i64())? as i32;
    let y = val.get("y").and_then(|v| v.as_i64())? as i32;
    let w = (val.get("width").and_then(|v| v.as_i64()).unwrap_or(1060) as i32).clamp(680, 3840);
    let h = (val.get("height").and_then(|v| v.as_i64()).unwrap_or(800) as i32).clamp(680, 2160);
    Some((x, y, w, h))
}

fn load_saved_oled_geometry_tuple() -> Option<(i32, i32, i32, i32)> {
    let path = crate::paths::config_dir().join("oled_widget_geometry.json");
    let raw = fs::read_to_string(path).ok()?;
    let val: serde_json::Value = serde_json::from_str(&raw).ok()?;
    if !val
        .get("has_position")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return None;
    }
    let x = val.get("x").and_then(|v| v.as_i64())? as i32;
    let y = val.get("y").and_then(|v| v.as_i64())? as i32;
    let w = (val.get("width").and_then(|v| v.as_i64()).unwrap_or(284) as i32).clamp(184, 1600);
    let h = (val.get("height").and_then(|v| v.as_i64()).unwrap_or(356) as i32).clamp(240, 1600);
    Some((x, y, w, h))
}

fn load_saved_oled_on_all_desktops() -> bool {
    let path = crate::paths::config_dir().join("oled_widget_geometry.json");
    let Ok(raw) = fs::read_to_string(path) else {
        return true;
    };
    let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return true;
    };
    val.get("on_all_desktops")
        .and_then(|v| v.as_bool())
        .unwrap_or(true)
}

#[cfg(not(target_os = "windows"))]
static KWIN_SCRIPT_LOCK: std::sync::LazyLock<tokio::sync::Mutex<()>> =
    std::sync::LazyLock::new(|| tokio::sync::Mutex::new(()));

#[cfg(not(target_os = "windows"))]
static ACTIVE_KWIN_HOOK_SLOT: std::sync::LazyLock<std::sync::Mutex<Option<(String, i32)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

#[cfg(not(target_os = "windows"))]
fn kwin_hook_sid_path() -> PathBuf {
    crate::paths::kwin_runtime_dir().join("duckydeck-kwin6-hook.sid")
}

#[cfg(not(target_os = "windows"))]
fn load_persisted_kwin_hook_sid() -> Option<(String, i32)> {
    let raw = fs::read_to_string(kwin_hook_sid_path()).ok()?;
    let line = raw.lines().next()?.trim();
    let (plugin, id_str) = line.split_once('\t')?;
    let plugin = plugin.trim();
    if plugin.is_empty() {
        return None;
    }
    let id = id_str.trim().parse::<i32>().ok()?;
    if id < 0 {
        return None;
    }
    Some((plugin.to_string(), id))
}

#[cfg(not(target_os = "windows"))]
fn save_persisted_kwin_hook_sid(plugin_name: &str, script_id: i32) {
    if let Ok(mut guard) = ACTIVE_KWIN_HOOK_SLOT.lock() {
        *guard = Some((plugin_name.to_string(), script_id));
    }
    let sid_path = kwin_hook_sid_path();
    let payload = format!("{plugin_name}\t{script_id}\n");
    let _ = fs::write(sid_path, payload.as_bytes());
}

#[cfg(not(target_os = "windows"))]
async fn stop_and_unload_kwin_script(
    conn: &zbus::Connection,
    kwin_scripting: &zbus::Proxy<'_>,
    plugin_name: &str,
    script_id: Option<i32>,
) {
    if let Some(id) = script_id.filter(|&i| i >= 0) {
        let script_obj_path = format!("/Scripting/Script{id}");
        if let Ok(script_proxy) = build_uncached_proxy(
            conn,
            "org.kde.KWin",
            script_obj_path.as_str(),
            "org.kde.kwin.Script",
        )
        .await
        {
            let _: std::result::Result<(), _> = script_proxy.call("stop", &()).await;
        }
    }
    let _: std::result::Result<bool, _> =
        kwin_scripting.call("unloadScript", &(plugin_name,)).await;
}

#[cfg(not(target_os = "windows"))]
async fn introspect_kwin_script_ids_with_proxy(
    kwin_introspect: &zbus::Proxy<'_>,
) -> std::collections::BTreeSet<i32> {
    let mut ids = std::collections::BTreeSet::new();
    let Ok(xml) = kwin_introspect
        .call::<_, _, String>("Introspect", &())
        .await
    else {
        return ids;
    };
    for line in xml.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("<node name=\"Script") {
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if let Ok(id) = digits.parse::<i32>() {
                ids.insert(id);
            }
        }
    }
    ids
}

#[cfg(not(target_os = "windows"))]
async fn register_kwin6_hook_script(
    conn: &zbus::Connection,
    kwin_scripting: &zbus::Proxy<'_>,
    kwin_introspect: &zbus::Proxy<'_>,
    script_path: &str,
    pending_check: &Arc<Mutex<PendingWindowSwitch>>,
    initial_startup: bool,
    reload_requested: bool,
) -> Option<(String, i32)> {
    let base_plugin_name = "duckydeck_kwin6_autoswitcher";
    let pid_fallback_name = format!("duckydeck_kwin6_autoswitcher_{}", std::process::id());

    // Cleanly stop & unload any active/persisted DuckyDeck KWin hook script first without
    // firing 24 blind `_slot_{1..23}` unloadScript D-Bus calls unless a collision actually occurs.
    {
        let _guard = KWIN_SCRIPT_LOCK.lock().await;
        let mut prior_slots: Vec<(String, i32)> = Vec::new();
        if let Ok(guard) = ACTIVE_KWIN_HOOK_SLOT.lock() {
            if let Some(entry) = guard.clone() {
                prior_slots.push(entry);
            }
        }
        if let Some(persisted) = load_persisted_kwin_hook_sid() {
            if !prior_slots
                .iter()
                .any(|(p, id)| p == &persisted.0 && *id == persisted.1)
            {
                prior_slots.push(persisted);
            }
        }
        let mut unloaded_names: Vec<String> = Vec::new();
        for (prev_plugin, prev_id) in prior_slots {
            stop_and_unload_kwin_script(conn, kwin_scripting, &prev_plugin, Some(prev_id)).await;
            unloaded_names.push(prev_plugin);
        }
        for known_name in [
            "duckydeck_oled_keep_above_oneshot",
            base_plugin_name,
            pid_fallback_name.as_str(),
        ] {
            if unloaded_names.iter().any(|n| n == known_name) {
                continue;
            }
            if kwin_scripting
                .call::<_, _, bool>("isScriptLoaded", &(known_name,))
                .await
                .unwrap_or(false)
            {
                let _: std::result::Result<bool, _> =
                    kwin_scripting.call("unloadScript", &(known_name,)).await;
            }
        }
    }

    if let Ok(mut p) = pending_check.lock() {
        p.raw_class.clear();
        p.settled_and_dispatched = false;
    }

    for attempt in 0..24 {
        let candidate_name = if attempt == 0 {
            base_plugin_name.to_string()
        } else {
            format!("{base_plugin_name}_slot_{attempt}")
        };

        let seq_before = KWIN_NOTIFY_SEQ.load(std::sync::atomic::Ordering::SeqCst);
        let mut run_succeeded = false;
        let mut assigned_id: i32 = -1;
        {
            let _guard = KWIN_SCRIPT_LOCK.lock().await;
            if attempt > 0
                && kwin_scripting
                    .call::<_, _, bool>("isScriptLoaded", &(candidate_name.as_str(),))
                    .await
                    .unwrap_or(false)
            {
                let _: std::result::Result<bool, _> = kwin_scripting
                    .call("unloadScript", &(candidate_name.as_str(),))
                    .await;
            }
            let pre_ids = introspect_kwin_script_ids_with_proxy(kwin_introspect).await;
            let script_id: i32 = kwin_scripting
                .call("loadScript", &(script_path, candidate_name.as_str()))
                .await
                .unwrap_or(-1);

            if script_id >= 0 {
                assigned_id = script_id;
                if pre_ids.contains(&script_id) {
                    // `/Scripting/Script{script_id}` was already owned by another script before `loadScript`!
                    // Keep `candidate_name` in KWin's `scripts` list as an inert gap-filler to advance
                    // `scripts.size()`, and do NOT call `.run()` on the foreign D-Bus object.
                    continue;
                }
                let script_obj_path = format!("/Scripting/Script{script_id}");
                if let Ok(kwin_script) = build_uncached_proxy(
                    conn,
                    "org.kde.KWin",
                    script_obj_path.as_str(),
                    "org.kde.kwin.Script",
                )
                .await
                {
                    match kwin_script.call::<_, _, ()>("run", &()).await {
                        Ok(()) => {
                            run_succeeded = true;
                        }
                        Err(e) => {
                            let _: std::result::Result<(), _> = kwin_script.call("stop", &()).await;
                            crate::bridge::device_controller::record_system_event(
                                "WARN",
                                "KWIN",
                                format!(
                                    "KWin Script{script_id} slot collision on attempt {attempt} ({e}); advancing to next slot"
                                ),
                            );
                        }
                    }
                }
            }
        }

        if !run_succeeded || assigned_id < 0 {
            continue;
        }

        // Wait up to 400ms for sendInitialTopWindow() to deliver the initial window over D-Bus.
        let mut received_initial = false;
        for _ in 0..16 {
            tokio::time::sleep(std::time::Duration::from_millis(25)).await;
            let seq_now = KWIN_NOTIFY_SEQ.load(std::sync::atomic::Ordering::SeqCst);
            let has_class = pending_check
                .lock()
                .map(|p| !p.raw_class.is_empty())
                .unwrap_or(false);
            if seq_now > seq_before || has_class {
                received_initial = true;
                break;
            }
        }
        if received_initial {
            save_persisted_kwin_hook_sid(&candidate_name, assigned_id);
            let msg = if initial_startup {
                format!(
                    "KWin 6 Auto-Switcher hook active (plugin '{candidate_name}', Script{assigned_id})"
                )
            } else if reload_requested {
                format!(
                    "KWin 6 Auto-Switcher hook reloaded after settings update (plugin '{candidate_name}', Script{assigned_id})"
                )
            } else {
                format!(
                    "KWin 6 Auto-Switcher hook self-healed & re-registered (plugin '{candidate_name}', Script{assigned_id})"
                )
            };
            crate::bridge::device_controller::record_system_event("INFO", "KWIN", msg);
            return Some((candidate_name, assigned_id));
        }
    }
    None
}

pub fn apply_kwin_oled_keep_above(keep_above: bool, restore_geometry: bool) {
    let on_all_desktops = load_saved_oled_on_all_desktops();
    #[cfg(target_os = "windows")]
    {
        let geom_opt = if restore_geometry {
            load_saved_oled_geometry_tuple()
        } else {
            None
        };
        win32_window::apply_oled_topmost_and_geometry(keep_above, on_all_desktops, geom_opt);
        return;
    }
    #[cfg(not(target_os = "windows"))]
    {
        if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        let geom_opt = if restore_geometry {
            load_saved_oled_geometry_tuple()
        } else {
            None
        };
        crate::actions::web_api::spawn_shared_async(async move {
            if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
                return;
            }
            let mut conn = match shared_dbus_session().await {
                Some(c) => c,
                None => return,
            };
            let kwin_scripting = match build_uncached_proxy(
                &conn,
                "org.kde.KWin",
                "/Scripting",
                "org.kde.kwin.Scripting",
            )
            .await
            {
                Ok(p) => p,
                Err(_) => {
                    invalidate_shared_dbus_session().await;
                    let Some(c2) = shared_dbus_session().await else {
                        return;
                    };
                    conn = c2;
                    let Ok(p2) = build_uncached_proxy(
                        &conn,
                        "org.kde.KWin",
                        "/Scripting",
                        "org.kde.kwin.Scripting",
                    )
                    .await
                    else {
                        return;
                    };
                    p2
                }
            };

            // If our persistent KWin 6 hook script is already loaded, it automatically enforces
            // keepAbove, onAllDesktops, skipTaskbar, skipPager, and saved OLED geometry via
            // workspace.windowAdded and win.captionChanged without creating oneshot Script{id} churn.
            if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst)
                || kwin_scripting
                    .call::<_, _, bool>("isScriptLoaded", &("duckydeck_kwin6_autoswitcher",))
                    .await
                    .unwrap_or(false)
            {
                return;
            }
            let active_slot_name = ACTIVE_KWIN_HOOK_SLOT
                .lock()
                .ok()
                .and_then(|guard| guard.as_ref().map(|(name, _)| name.clone()));
            if let Some(active_name) = active_slot_name {
                if kwin_scripting
                    .call::<_, _, bool>("isScriptLoaded", &(active_name.as_str(),))
                    .await
                    .unwrap_or(false)
                {
                    return;
                }
            }

            let script_path = crate::paths::kwin_runtime_dir()
                .join("duckydeck-kwin6-oled-pin.js")
                .to_string_lossy()
                .to_string();
            let flag_str = if keep_above { "true" } else { "false" };
            let all_desk_str = if on_all_desktops { "true" } else { "false" };
            let geom_js = if let Some((gx, gy, gw, gh)) = geom_opt {
                format!("w.frameGeometry = {{ x: {gx}, y: {gy}, width: {gw}, height: {gh} }};")
            } else {
                String::new()
            };
            let js = format!(
                r#"
var list = workspace.stackingOrder || workspace.windowList();
if (list) {{
    for (var i = 0; i < list.length; ++i) {{
        var w = list[i];
        var cap = w && w.caption ? w.caption.toString() : "";
        if (cap.indexOf("duckyPad OLED —") === 0) {{
            var isPinned = (cap.indexOf("[Unpinned]") === -1) && {flag_str};
            var isAllDesktops = (cap.indexOf("[SingleDesktop]") === -1) && {all_desk_str};
            if (isPinned && w.minimized) {{
                w.minimized = false;
            }}
            {geom_js}
            if (isAllDesktops) {{
                w.onAllDesktops = false;
                w.onAllDesktops = true;
            }} else {{
                w.onAllDesktops = false;
            }}
            w.keepAbove = isPinned;
            w.skipTaskbar = isPinned;
            w.skipPager = isPinned;
        }}
    }}
}}
"#
            );
            let plugin_name = "duckydeck_oled_keep_above_oneshot";
            let delays_ms: &[u64] = if geom_opt.is_some() {
                &[0, 60, 180, 420]
            } else {
                &[0]
            };
            for &delay in delays_ms {
                if delay > 0 {
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
                if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
                    return;
                }
                let _guard = KWIN_SCRIPT_LOCK.lock().await;
                if KWIN_HOOK_REGISTERED.load(std::sync::atomic::Ordering::SeqCst) {
                    return;
                }
                if std::fs::write(&script_path, js.as_bytes()).is_err() {
                    return;
                }
                let _: std::result::Result<bool, _> =
                    kwin_scripting.call("unloadScript", &(plugin_name,)).await;
                if let Ok(script_id) = kwin_scripting
                    .call::<_, _, i32>("loadScript", &(&script_path, plugin_name))
                    .await
                {
                    if script_id >= 0 {
                        let obj_path = format!("/Scripting/Script{script_id}");
                        let script_proxy_opt = build_uncached_proxy(
                            &conn,
                            "org.kde.KWin",
                            obj_path.as_str(),
                            "org.kde.kwin.Script",
                        )
                        .await
                        .ok();
                        if let Some(ref script_proxy) = script_proxy_opt {
                            if let Err(e) = script_proxy.call::<_, _, ()>("run", &()).await {
                                crate::bridge::device_controller::record_system_event(
                                    "WARN",
                                    "KWIN",
                                    format!("Failed running KWin OLED keep-above script: {e}"),
                                );
                            }
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
                        if let Some(ref script_proxy) = script_proxy_opt {
                            let _: std::result::Result<(), _> =
                                script_proxy.call("stop", &()).await;
                        }
                        let _: std::result::Result<bool, _> =
                            kwin_scripting.call("unloadScript", &(plugin_name,)).await;
                    }
                }
            }
        });
    }
}
