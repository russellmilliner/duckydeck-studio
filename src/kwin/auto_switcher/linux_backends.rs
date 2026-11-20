use std::sync::Mutex;

#[cfg(not(target_os = "windows"))]
use super::kwin_dbus::build_uncached_proxy;

static LAST_BRIDGE_WINDOW: std::sync::OnceLock<Mutex<(String, String, String, u64)>> =
    std::sync::OnceLock::new();

pub(super) fn record_last_bridge_window(resource_class: &str, resource_name: &str, caption: &str) {
    let lock = LAST_BRIDGE_WINDOW
        .get_or_init(|| Mutex::new((String::new(), String::new(), String::new(), 0)));
    if let Ok(mut guard) = lock.lock() {
        let next_seq = guard.3.wrapping_add(1);
        *guard = (
            resource_class.to_string(),
            resource_name.to_string(),
            caption.to_string(),
            next_seq,
        );
    }
}

#[cfg(not(target_os = "windows"))]
pub(super) fn latest_bridge_window() -> Option<(String, String, String, u64)> {
    let lock = LAST_BRIDGE_WINDOW.get()?;
    let guard = lock.lock().ok()?;
    if guard.3 == 0 || guard.0.is_empty() {
        None
    } else {
        Some(guard.clone())
    }
}

#[cfg(not(target_os = "windows"))]
pub(super) mod linux_desktop_window {
    use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_void};
    use std::fs;
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;
    use std::path::{Path, PathBuf};
    use std::sync::{Mutex, OnceLock};
    use std::time::Duration;

    pub type WindowTuple = (String, String, String);

    // =========================================================================
    // 1. Omarchy / Hyprland Backend ($XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE)
    // =========================================================================

    pub fn is_hyprland_session() -> bool {
        find_hyprland_socket_path().is_some()
    }

    pub fn find_hyprland_socket_path() -> Option<PathBuf> {
        let sig = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").ok()?;
        let sig = sig.trim();
        if sig.is_empty() {
            return None;
        }
        if let Ok(rt) = std::env::var("XDG_RUNTIME_DIR") {
            let p = PathBuf::from(rt)
                .join("hypr")
                .join(sig)
                .join(".socket.sock");
            if p.exists() {
                return Some(p);
            }
        }
        let fallback = PathBuf::from("/tmp/hypr").join(sig).join(".socket.sock");
        if fallback.exists() {
            return Some(fallback);
        }
        None
    }

    fn hyprland_ipc_request(sock_path: &Path, command: &str) -> Option<String> {
        let mut stream = UnixStream::connect(sock_path).ok()?;
        let _ = stream.set_write_timeout(Some(Duration::from_millis(50)));
        let _ = stream.set_read_timeout(Some(Duration::from_millis(50)));
        stream.write_all(command.as_bytes()).ok()?;
        let mut out = String::new();
        stream.read_to_string(&mut out).ok()?;
        Some(out)
    }

    pub fn parse_hyprland_active_window_json(raw_json: &str) -> Option<WindowTuple> {
        let val: serde_json::Value = serde_json::from_str(raw_json.trim()).ok()?;
        let obj = val.as_object()?;
        let class = obj
            .get("class")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let initial_class = obj
            .get("initialClass")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        let title = obj
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();

        let effective_class = if !class.is_empty() {
            class.clone()
        } else {
            initial_class.clone()
        };
        if effective_class.is_empty() {
            return None;
        }
        let lower = effective_class.to_ascii_lowercase();
        if lower.contains("duckydeck") || title.starts_with("duckyPad OLED —") {
            return None;
        }
        let res_name = if !initial_class.is_empty() {
            initial_class
        } else {
            effective_class.clone()
        };
        Some((effective_class, res_name, title))
    }

    pub fn parse_hyprland_cursorpos_json(raw_json: &str) -> Option<(i64, i64)> {
        let val: serde_json::Value = serde_json::from_str(raw_json.trim()).ok()?;
        let obj = val.as_object()?;
        let x = obj.get("x")?.as_i64()?;
        let y = obj.get("y")?.as_i64()?;
        Some((x, y))
    }

    pub fn find_hyprland_client_at_cursor_json(
        clients_json: &str,
        active_workspace_id: Option<i64>,
        cx: i64,
        cy: i64,
    ) -> Option<WindowTuple> {
        let val: serde_json::Value = serde_json::from_str(clients_json.trim()).ok()?;
        let arr = val.as_array()?;
        let mut best: Option<(bool, i64, WindowTuple)> = None;

        for item in arr {
            let Some(obj) = item.as_object() else {
                continue;
            };
            let mapped = obj.get("mapped").and_then(|v| v.as_bool()).unwrap_or(true);
            let hidden = obj.get("hidden").and_then(|v| v.as_bool()).unwrap_or(false);
            if !mapped || hidden {
                continue;
            }
            let pinned = obj.get("pinned").and_then(|v| v.as_bool()).unwrap_or(false);
            let ws_id = obj
                .get("workspace")
                .and_then(|w| w.get("id"))
                .and_then(|v| v.as_i64());
            if let (Some(active_ws), Some(client_ws)) = (active_workspace_id, ws_id) {
                if !pinned && client_ws != active_ws && client_ws > 0 {
                    continue;
                }
            }
            let Some(at) = obj.get("at").and_then(|v| v.as_array()) else {
                continue;
            };
            let Some(size) = obj.get("size").and_then(|v| v.as_array()) else {
                continue;
            };
            if at.len() < 2 || size.len() < 2 {
                continue;
            }
            let x = at[0].as_i64().unwrap_or(0);
            let y = at[1].as_i64().unwrap_or(0);
            let w = size[0].as_i64().unwrap_or(0);
            let h = size[1].as_i64().unwrap_or(0);
            if w <= 0 || h <= 0 || cx < x || cx >= x + w || cy < y || cy >= y + h {
                continue;
            }

            let class = obj
                .get("class")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let initial_class = obj
                .get("initialClass")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let title = obj
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            let effective_class = if !class.is_empty() {
                class
            } else {
                initial_class.clone()
            };
            if effective_class.is_empty() {
                continue;
            }
            if effective_class.to_ascii_lowercase().contains("duckydeck")
                || title.starts_with("duckyPad OLED —")
            {
                continue;
            }
            let res_name = if !initial_class.is_empty() {
                initial_class
            } else {
                effective_class.clone()
            };
            let floating = obj
                .get("floating")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let focus_history = obj
                .get("focusHistoryID")
                .and_then(|v| v.as_i64())
                .unwrap_or(9999);

            let candidate = (effective_class, res_name, title);
            let replace = match &best {
                None => true,
                Some((best_floating, best_hist, _)) => {
                    if floating != *best_floating {
                        floating
                    } else {
                        focus_history < *best_hist
                    }
                }
            };
            if replace {
                best = Some((floating, focus_history, candidate));
            }
        }

        best.map(|(_, _, win)| win)
    }

    pub fn query_hyprland_window_and_hover(
        last_cursor: &mut Option<(i64, i64)>,
    ) -> Option<(Option<WindowTuple>, Option<WindowTuple>)> {
        let sock = find_hyprland_socket_path()?;
        Some(query_hyprland_window_and_hover_from_socket(
            &sock,
            last_cursor,
        ))
    }

    pub fn query_hyprland_window_and_hover_from_socket(
        sock: &Path,
        last_cursor: &mut Option<(i64, i64)>,
    ) -> (Option<WindowTuple>, Option<WindowTuple>) {
        let active_win = hyprland_ipc_request(sock, "j/activewindow")
            .and_then(|raw| parse_hyprland_active_window_json(&raw));

        let mut hover_win = None;
        if let Some(cursor_raw) = hyprland_ipc_request(sock, "j/cursorpos") {
            if let Some(cur_xy) = parse_hyprland_cursorpos_json(&cursor_raw) {
                if *last_cursor != Some(cur_xy) {
                    *last_cursor = Some(cur_xy);
                    let active_ws = hyprland_ipc_request(sock, "j/activeworkspace")
                        .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw.trim()).ok())
                        .and_then(|v| v.get("id").and_then(|id| id.as_i64()));
                    if let Some(clients_raw) = hyprland_ipc_request(sock, "j/clients") {
                        hover_win = find_hyprland_client_at_cursor_json(
                            &clients_raw,
                            active_ws,
                            cur_xy.0,
                            cur_xy.1,
                        );
                    }
                }
            }
        }
        (active_win, hover_win)
    }

    pub fn query_hyprland_window_at_current_cursor() -> Option<WindowTuple> {
        let sock = find_hyprland_socket_path()?;
        let cursor_raw = hyprland_ipc_request(&sock, "j/cursorpos")?;
        let (cx, cy) = parse_hyprland_cursorpos_json(&cursor_raw)?;
        let active_ws = hyprland_ipc_request(&sock, "j/activeworkspace")
            .and_then(|raw| serde_json::from_str::<serde_json::Value>(raw.trim()).ok())
            .and_then(|v| v.get("id").and_then(|id| id.as_i64()));
        let clients_raw = hyprland_ipc_request(&sock, "j/clients")?;
        find_hyprland_client_at_cursor_json(&clients_raw, active_ws, cx, cy)
    }

    // =========================================================================
    // 2. COSMIC (`cosmic-comp`) / wlroots / Sway / Niri Wayland Toplevel Watcher
    //    (Pure-Rust Wayland wire protocol client for `zwlr_foreign_toplevel_manager_v1`
    //     and `zcosmic_toplevel_info_v1` / `ext_foreign_toplevel_list_v1`)
    // =========================================================================

    static WAYLAND_ACTIVE_TOPLEVEL: OnceLock<Mutex<(String, String, String, u64)>> =
        OnceLock::new();
    static WAYLAND_WATCHER_STARTED: OnceLock<()> = OnceLock::new();

    #[derive(Clone, Default)]
    struct ToplevelEntry {
        is_cosmic: bool,
        app_id: String,
        title: String,
        activated: bool,
    }

    pub fn parse_wayland_state_has_activated(state_bytes: &[u8]) -> bool {
        // In both `zwlr_foreign_toplevel_handle_v1::state` and `zcosmic_toplevel_handle_v1::state`,
        // `STATE_ACTIVATED = 2` is encoded as a 32-bit little-endian integer in the `wl_array`.
        state_bytes
            .as_chunks::<4>()
            .0
            .iter()
            .any(|chunk| u32::from_ne_bytes(*chunk) == 2)
    }

    fn encode_wl_str(s: &str) -> Vec<u8> {
        let bytes = s.as_bytes();
        let len_with_nul = (bytes.len() + 1) as u32;
        let mut out = Vec::with_capacity(4 + ((len_with_nul as usize + 3) & !3));
        out.extend_from_slice(&len_with_nul.to_ne_bytes());
        out.extend_from_slice(bytes);
        out.push(0);
        while out.len() % 4 != 0 {
            out.push(0);
        }
        out
    }

    fn send_wl_msg(stream: &mut UnixStream, sender_id: u32, opcode: u16, payload: &[u8]) -> bool {
        let total_size = (8 + payload.len()) as u16;
        let size_and_opcode = ((total_size as u32) << 16) | (opcode as u32);
        let mut buf = Vec::with_capacity(8 + payload.len());
        buf.extend_from_slice(&sender_id.to_ne_bytes());
        buf.extend_from_slice(&size_and_opcode.to_ne_bytes());
        buf.extend_from_slice(payload);
        stream.write_all(&buf).is_ok()
    }

    fn read_wl_str(payload: &[u8], offset: usize) -> Option<(String, usize)> {
        if offset + 4 > payload.len() {
            return None;
        }
        let len = u32::from_ne_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
        ]) as usize;
        let padded = (len + 3) & !3;
        let start = offset + 4;
        if len == 0 || start + padded > payload.len() {
            return Some((
                String::new(),
                start + padded.min(payload.len().saturating_sub(start)),
            ));
        }
        let raw = &payload[start..start + len.saturating_sub(1)];
        let s = String::from_utf8_lossy(raw).trim().to_string();
        Some((s, start + padded))
    }

    fn read_wl_array(payload: &[u8], offset: usize) -> Option<(&[u8], usize)> {
        if offset + 4 > payload.len() {
            return None;
        }
        let len = u32::from_ne_bytes([
            payload[offset],
            payload[offset + 1],
            payload[offset + 2],
            payload[offset + 3],
        ]) as usize;
        let padded = (len + 3) & !3;
        let start = offset + 4;
        if start + padded > payload.len() {
            return None;
        }
        Some((&payload[start..start + len], start + padded))
    }

    fn record_wayland_active_toplevel(app_id: &str, title: &str) {
        let cls = app_id.trim();
        if cls.is_empty() {
            return;
        }
        if cls.to_ascii_lowercase().contains("duckydeck") || title.starts_with("duckyPad OLED —")
        {
            return;
        }
        let lock = WAYLAND_ACTIVE_TOPLEVEL
            .get_or_init(|| Mutex::new((String::new(), String::new(), String::new(), 0)));
        if let Ok(mut guard) = lock.lock() {
            if guard.0 != cls || guard.2 != title {
                let next_seq = guard.3.wrapping_add(1);
                *guard = (
                    cls.to_string(),
                    cls.to_string(),
                    title.to_string(),
                    next_seq,
                );
            }
        }
    }

    pub fn ensure_wayland_toplevel_watcher_started() {
        WAYLAND_WATCHER_STARTED.get_or_init(|| {
            std::thread::Builder::new()
                .name("duckydeck-wl-toplevel".to_string())
                .spawn(|| {
                    run_wayland_toplevel_client();
                })
                .ok();
        });
    }

    pub fn latest_wayland_active_window() -> Option<(String, String, String, u64)> {
        ensure_wayland_toplevel_watcher_started();
        let lock = WAYLAND_ACTIVE_TOPLEVEL.get()?;
        let guard = lock.lock().ok()?;
        if guard.3 == 0 || guard.0.is_empty() {
            None
        } else {
            Some(guard.clone())
        }
    }

    fn run_wayland_toplevel_client() {
        let Ok(wl_disp) = std::env::var("WAYLAND_DISPLAY") else {
            return;
        };
        if wl_disp.trim().is_empty() {
            return;
        }
        let sock_path = if Path::new(&wl_disp).is_absolute() {
            PathBuf::from(&wl_disp)
        } else if let Ok(rt) = std::env::var("XDG_RUNTIME_DIR") {
            PathBuf::from(rt).join(&wl_disp)
        } else {
            return;
        };
        let Ok(stream) = UnixStream::connect(&sock_path) else {
            return;
        };
        run_wayland_toplevel_client_on_stream(stream);
    }

    pub fn run_wayland_toplevel_client_on_stream(mut stream: UnixStream) {
        // Object IDs: 1 = wl_display, 2 = wl_registry
        let mut next_client_id: u32 = 3;
        let registry_id: u32 = 2;
        if !send_wl_msg(&mut stream, 1, 1, &registry_id.to_ne_bytes()) {
            return;
        }

        let mut zwlr_mgr_id: Option<u32> = None;
        let mut cosmic_info_id: Option<u32> = None;
        let mut toplevels: std::collections::HashMap<u32, ToplevelEntry> =
            std::collections::HashMap::new();

        let mut hdr = [0u8; 8];
        loop {
            if stream.read_exact(&mut hdr).is_err() {
                break;
            }
            let sender_id = u32::from_ne_bytes([hdr[0], hdr[1], hdr[2], hdr[3]]);
            let size_and_op = u32::from_ne_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]);
            let size = (size_and_op >> 16) as usize;
            let opcode = (size_and_op & 0xFFFF) as u16;
            if size < 8 {
                break;
            }
            let mut payload = vec![0u8; size - 8];
            if !payload.is_empty() && stream.read_exact(&mut payload).is_err() {
                break;
            }

            if sender_id == registry_id && opcode == 0 && payload.len() >= 8 {
                // wl_registry::global(name: u32, interface: str, version: u32)
                let name = u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
                if let Some((iface, next_off)) = read_wl_str(&payload, 4) {
                    if next_off + 4 <= payload.len() {
                        let version = u32::from_ne_bytes([
                            payload[next_off],
                            payload[next_off + 1],
                            payload[next_off + 2],
                            payload[next_off + 3],
                        ]);
                        if iface == "zwlr_foreign_toplevel_manager_v1" && zwlr_mgr_id.is_none() {
                            let new_id = next_client_id;
                            next_client_id += 1;
                            let mut bind_payload = Vec::new();
                            bind_payload.extend_from_slice(&name.to_ne_bytes());
                            bind_payload.extend_from_slice(&encode_wl_str(&iface));
                            bind_payload.extend_from_slice(&version.clamp(1, 3).to_ne_bytes());
                            bind_payload.extend_from_slice(&new_id.to_ne_bytes());
                            if send_wl_msg(&mut stream, registry_id, 0, &bind_payload) {
                                zwlr_mgr_id = Some(new_id);
                            }
                        } else if iface == "zcosmic_toplevel_info_v1" && cosmic_info_id.is_none() {
                            let new_id = next_client_id;
                            next_client_id += 1;
                            let mut bind_payload = Vec::new();
                            bind_payload.extend_from_slice(&name.to_ne_bytes());
                            bind_payload.extend_from_slice(&encode_wl_str(&iface));
                            bind_payload.extend_from_slice(&1u32.to_ne_bytes());
                            bind_payload.extend_from_slice(&new_id.to_ne_bytes());
                            if send_wl_msg(&mut stream, registry_id, 0, &bind_payload) {
                                cosmic_info_id = Some(new_id);
                            }
                        }
                    }
                }
            } else if (Some(sender_id) == zwlr_mgr_id || Some(sender_id) == cosmic_info_id)
                && opcode == 0
                && payload.len() >= 4
            {
                // `toplevel(id: new_id)` event from manager
                let handle_id =
                    u32::from_ne_bytes([payload[0], payload[1], payload[2], payload[3]]);
                toplevels.insert(
                    handle_id,
                    ToplevelEntry {
                        is_cosmic: Some(sender_id) == cosmic_info_id,
                        ..ToplevelEntry::default()
                    },
                );
            } else if let Some(entry) = toplevels.get_mut(&sender_id) {
                if entry.is_cosmic {
                    // `zcosmic_toplevel_handle_v1` (bound at v1) event opcodes:
                    // 0 = closed(), 1 = done(), 2 = title(string), 3 = app_id(string), 8 = state(wl_array)
                    match opcode {
                        0 => {
                            toplevels.remove(&sender_id);
                        }
                        1 => {
                            if entry.activated && !entry.app_id.is_empty() {
                                let app_id = entry.app_id.clone();
                                let title = entry.title.clone();
                                record_wayland_active_toplevel(&app_id, &title);
                            }
                        }
                        2 => {
                            if let Some((title, _)) = read_wl_str(&payload, 0) {
                                entry.title = title;
                            }
                        }
                        3 => {
                            if let Some((app_id, _)) = read_wl_str(&payload, 0) {
                                entry.app_id = app_id;
                            }
                        }
                        8 => {
                            if let Some((arr, _)) = read_wl_array(&payload, 0) {
                                entry.activated = parse_wayland_state_has_activated(arr);
                            }
                        }
                        _ => {}
                    }
                } else {
                    // `zwlr_foreign_toplevel_handle_v1` event opcodes:
                    // 0 = title(string), 1 = app_id(string), 4 = state(wl_array), 5 = done(), 6 = closed()
                    match opcode {
                        0 => {
                            if let Some((title, _)) = read_wl_str(&payload, 0) {
                                entry.title = title;
                            }
                        }
                        1 => {
                            if let Some((app_id, _)) = read_wl_str(&payload, 0) {
                                entry.app_id = app_id;
                            }
                        }
                        4 => {
                            if let Some((arr, _)) = read_wl_array(&payload, 0) {
                                entry.activated = parse_wayland_state_has_activated(arr);
                            }
                        }
                        5 => {
                            if entry.activated && !entry.app_id.is_empty() {
                                let app_id = entry.app_id.clone();
                                let title = entry.title.clone();
                                record_wayland_active_toplevel(&app_id, &title);
                            }
                        }
                        6 => {
                            toplevels.remove(&sender_id);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // =========================================================================
    // 3. Cinnamon (Linux Mint) / MATE / Xfce / Xorg Backend (`libX11.so.6`)
    // =========================================================================

    type XErrorHandler = unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int;

    unsafe extern "C" fn ignore_x11_errors(_dpy: *mut c_void, _err: *mut c_void) -> c_int {
        0
    }

    struct X11Api {
        dpy: *mut c_void,
        root: c_ulong,
        atom_net_active_window: c_ulong,
        atom_net_wm_name: c_ulong,
        atom_utf8_string: c_ulong,
        atom_wm_class: c_ulong,
        atom_wm_name: c_ulong,
        atom_string: c_ulong,
        x_get_window_property: unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            c_ulong,
            c_long,
            c_long,
            c_int,
            c_ulong,
            *mut c_ulong,
            *mut c_int,
            *mut c_ulong,
            *mut c_ulong,
            *mut *mut c_uchar,
        ) -> c_int,
        x_free: unsafe extern "C" fn(*mut c_void) -> c_int,
        x_query_pointer: unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            *mut c_ulong,
            *mut c_ulong,
            *mut c_int,
            *mut c_int,
            *mut c_int,
            *mut c_int,
            *mut c_uint,
        ) -> c_int,
        x_query_tree: unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            *mut c_ulong,
            *mut c_ulong,
            *mut *mut c_ulong,
            *mut c_uint,
        ) -> c_int,
    }

    // SAFETY: Accessed exclusively through a global `Mutex<Option<X11Api>>`.
    unsafe impl Send for X11Api {}

    static X11_API: OnceLock<Mutex<Option<X11Api>>> = OnceLock::new();

    unsafe extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }

    const RTLD_LAZY: c_int = 1;

    #[allow(clippy::missing_transmute_annotations)]
    unsafe fn init_x11_api() -> Option<X11Api> {
        std::env::var_os("DISPLAY")?;
        let handle = dlopen(c"libX11.so.6".as_ptr(), RTLD_LAZY);
        if handle.is_null() {
            return None;
        }

        let sym_open = dlsym(handle, c"XOpenDisplay".as_ptr());
        let sym_root = dlsym(handle, c"XDefaultRootWindow".as_ptr());
        let sym_intern = dlsym(handle, c"XInternAtom".as_ptr());
        let sym_get_prop = dlsym(handle, c"XGetWindowProperty".as_ptr());
        let sym_free = dlsym(handle, c"XFree".as_ptr());
        let sym_qp = dlsym(handle, c"XQueryPointer".as_ptr());
        let sym_qt = dlsym(handle, c"XQueryTree".as_ptr());
        let sym_err = dlsym(handle, c"XSetErrorHandler".as_ptr());

        if sym_open.is_null()
            || sym_root.is_null()
            || sym_intern.is_null()
            || sym_get_prop.is_null()
            || sym_free.is_null()
            || sym_qp.is_null()
            || sym_qt.is_null()
        {
            return None;
        }

        if !sym_err.is_null() {
            let x_set_error_handler: unsafe extern "C" fn(XErrorHandler) -> *mut c_void =
                std::mem::transmute(sym_err);
            x_set_error_handler(ignore_x11_errors);
        }

        let x_open_display: unsafe extern "C" fn(*const c_char) -> *mut c_void =
            std::mem::transmute(sym_open);
        let x_default_root: unsafe extern "C" fn(*mut c_void) -> c_ulong =
            std::mem::transmute(sym_root);
        let x_intern_atom: unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_ulong =
            std::mem::transmute(sym_intern);

        let dpy = x_open_display(std::ptr::null());
        if dpy.is_null() {
            return None;
        }
        let root = x_default_root(dpy);
        let atom_net_active_window = x_intern_atom(dpy, c"_NET_ACTIVE_WINDOW".as_ptr(), 0);
        let atom_net_wm_name = x_intern_atom(dpy, c"_NET_WM_NAME".as_ptr(), 0);
        let atom_utf8_string = x_intern_atom(dpy, c"UTF8_STRING".as_ptr(), 0);
        let atom_wm_class = x_intern_atom(dpy, c"WM_CLASS".as_ptr(), 0);
        let atom_wm_name = x_intern_atom(dpy, c"WM_NAME".as_ptr(), 0);
        let atom_string = x_intern_atom(dpy, c"STRING".as_ptr(), 0);

        Some(X11Api {
            dpy,
            root,
            atom_net_active_window,
            atom_net_wm_name,
            atom_utf8_string,
            atom_wm_class,
            atom_wm_name,
            atom_string,
            x_get_window_property: std::mem::transmute(sym_get_prop),
            x_free: std::mem::transmute(sym_free),
            x_query_pointer: std::mem::transmute(sym_qp),
            x_query_tree: std::mem::transmute(sym_qt),
        })
    }

    pub fn parse_wm_class_bytes(bytes: &[u8]) -> Option<(String, String)> {
        let parts: Vec<&[u8]> = bytes.split(|&b| b == 0).filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return None;
        }
        let res_name = String::from_utf8_lossy(parts[0]).trim().to_string();
        let res_class = if parts.len() >= 2 {
            String::from_utf8_lossy(parts[1]).trim().to_string()
        } else {
            res_name.clone()
        };
        let effective_class = if !res_class.is_empty() {
            res_class
        } else {
            res_name.clone()
        };
        if effective_class.is_empty() {
            None
        } else {
            Some((effective_class, res_name))
        }
    }

    unsafe fn x11_get_property_bytes(
        api: &X11Api,
        win: c_ulong,
        prop: c_ulong,
        req_type: c_ulong,
    ) -> Option<Vec<u8>> {
        if win == 0 {
            return None;
        }
        let mut actual_type: c_ulong = 0;
        let mut actual_format: c_int = 0;
        let mut nitems: c_ulong = 0;
        let mut bytes_after: c_ulong = 0;
        let mut prop_ptr: *mut c_uchar = std::ptr::null_mut();

        let status = (api.x_get_window_property)(
            api.dpy,
            win,
            prop,
            0,
            1024,
            0,
            req_type,
            &mut actual_type,
            &mut actual_format,
            &mut nitems,
            &mut bytes_after,
            &mut prop_ptr,
        );
        if status != 0 || prop_ptr.is_null() {
            return None;
        }
        let byte_len = match actual_format {
            8 => nitems as usize,
            16 => (nitems as usize) * 2,
            32 => (nitems as usize) * std::mem::size_of::<c_ulong>(),
            _ => 0,
        };
        let slice = if byte_len > 0 {
            std::slice::from_raw_parts(prop_ptr, byte_len).to_vec()
        } else {
            Vec::new()
        };
        (api.x_free)(prop_ptr.cast());
        if slice.is_empty() {
            None
        } else {
            Some(slice)
        }
    }

    unsafe fn x11_read_window_title(api: &X11Api, win: c_ulong) -> String {
        if let Some(bytes) =
            x11_get_property_bytes(api, win, api.atom_net_wm_name, api.atom_utf8_string)
                .or_else(|| x11_get_property_bytes(api, win, api.atom_net_wm_name, 0))
        {
            let s = String::from_utf8_lossy(&bytes)
                .trim_end_matches('\0')
                .trim()
                .to_string();
            if !s.is_empty() {
                return s;
            }
        }
        if let Some(bytes) = x11_get_property_bytes(api, win, api.atom_wm_name, api.atom_string)
            .or_else(|| x11_get_property_bytes(api, win, api.atom_wm_name, 0))
        {
            let s = String::from_utf8_lossy(&bytes)
                .trim_end_matches('\0')
                .trim()
                .to_string();
            if !s.is_empty() {
                return s;
            }
        }
        String::new()
    }

    unsafe fn x11_inspect_window_recursive(
        api: &X11Api,
        win: c_ulong,
        depth: u8,
    ) -> Option<WindowTuple> {
        if win == 0 || win == api.root {
            return None;
        }
        if let Some(bytes) = x11_get_property_bytes(api, win, api.atom_wm_class, api.atom_string)
            .or_else(|| x11_get_property_bytes(api, win, api.atom_wm_class, 0))
        {
            if let Some((cls, rname)) = parse_wm_class_bytes(&bytes) {
                let title = x11_read_window_title(api, win);
                if !cls.to_ascii_lowercase().contains("duckydeck")
                    && !title.starts_with("duckyPad OLED —")
                {
                    return Some((cls, rname, title));
                }
                return None;
            }
        }
        if depth >= 3 {
            return None;
        }
        let mut root_ret: c_ulong = 0;
        let mut parent_ret: c_ulong = 0;
        let mut children_ptr: *mut c_ulong = std::ptr::null_mut();
        let mut nchildren: c_uint = 0;
        let ok = (api.x_query_tree)(
            api.dpy,
            win,
            &mut root_ret,
            &mut parent_ret,
            &mut children_ptr,
            &mut nchildren,
        );
        if ok == 0 || children_ptr.is_null() {
            return None;
        }
        let children = std::slice::from_raw_parts(children_ptr, nchildren as usize).to_vec();
        (api.x_free)(children_ptr.cast());
        for &child in children.iter().rev() {
            if let Some(found) = x11_inspect_window_recursive(api, child, depth + 1) {
                return Some(found);
            }
        }
        None
    }

    pub fn query_x11_window_and_hover(
        last_cursor: &mut Option<(i32, i32)>,
        inspect_cursor_always: bool,
    ) -> (Option<WindowTuple>, Option<WindowTuple>, bool) {
        let lock = X11_API.get_or_init(|| Mutex::new(unsafe { init_x11_api() }));
        let Ok(guard) = lock.lock() else {
            return (None, None, false);
        };
        let Some(ref api) = *guard else {
            return (None, None, false);
        };

        unsafe {
            let mut active_win = None;
            if let Some(bytes) =
                x11_get_property_bytes(api, api.root, api.atom_net_active_window, 0)
            {
                if bytes.len() >= std::mem::size_of::<c_ulong>() {
                    let mut arr = [0u8; std::mem::size_of::<c_ulong>()];
                    arr.copy_from_slice(&bytes[..std::mem::size_of::<c_ulong>()]);
                    let win_id = c_ulong::from_ne_bytes(arr);
                    active_win = x11_inspect_window_recursive(api, win_id, 0);
                }
            }

            let mut root_ret: c_ulong = 0;
            let mut child_ret: c_ulong = 0;
            let mut root_x: c_int = 0;
            let mut root_y: c_int = 0;
            let mut win_x: c_int = 0;
            let mut win_y: c_int = 0;
            let mut mask: c_uint = 0;
            let mut hover_win = None;
            let mut button1_down = false;

            if (api.x_query_pointer)(
                api.dpy,
                api.root,
                &mut root_ret,
                &mut child_ret,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask,
            ) != 0
            {
                // Button1Mask in X11 is (1 << 8) = 256
                button1_down = (mask & (1 << 8)) != 0;
                let cur_xy = (root_x, root_y);
                let moved = *last_cursor != Some(cur_xy);
                if moved {
                    *last_cursor = Some(cur_xy);
                }
                if (moved || inspect_cursor_always || button1_down) && child_ret != 0 {
                    hover_win = x11_inspect_window_recursive(api, child_ret, 0);
                }
            }

            (active_win, hover_win, button1_down)
        }
    }

    #[cfg(test)]
    pub fn inspect_x11_window_id(win: c_ulong) -> Option<WindowTuple> {
        let lock = X11_API.get_or_init(|| Mutex::new(unsafe { init_x11_api() }));
        let guard = lock.lock().ok()?;
        let api = guard.as_ref()?;
        unsafe { x11_inspect_window_recursive(api, win, 0) }
    }

    // =========================================================================
    // 4. GNOME (`mutter` Wayland & Xorg) Auto-Installed Shell Extension + Introspect
    // =========================================================================

    static GNOME_EXT_INSTALLED: OnceLock<()> = OnceLock::new();

    pub fn is_gnome_session() -> bool {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_default()
            .to_ascii_lowercase();
        let session = std::env::var("DESKTOP_SESSION")
            .unwrap_or_default()
            .to_ascii_lowercase();
        desktop.contains("gnome") || session.contains("gnome") || desktop.contains("ubuntu")
    }

    pub async fn ensure_gnome_shell_extension_installed(conn: &zbus::Connection) {
        if !is_gnome_session() {
            return;
        }
        let mut first_time = false;
        GNOME_EXT_INSTALLED.get_or_init(|| {
            first_time = true;
            if let Some(home) = std::env::var_os("HOME") {
                let ext_uuid = "duckydeck-autoswitcher@duckydeck.github.io";
                let ext_dir = PathBuf::from(home)
                    .join(".local/share/gnome-shell/extensions")
                    .join(ext_uuid);
                if fs::create_dir_all(&ext_dir).is_ok() {
                    let metadata_json = r#"{
  "uuid": "duckydeck-autoswitcher@duckydeck.github.io",
  "name": "DuckyDeck Studio Auto-Switcher Bridge",
  "description": "Reports active window focus changes to DuckyDeck Studio over session D-Bus",
  "shell-version": ["42", "43", "44", "45", "46", "47", "48", "49"],
  "version": 1
}
"#;
                    let extension_js = r#"import Gio from 'gi://Gio';
import GLib from 'gi://GLib';

export default class DuckyDeckAutoSwitcherExtension {
    enable() {
        this._focusSig = global.display.connect('notify::focus-window', () => {
            this._sendFocusWindow();
        });
        this._sendFocusWindow();
    }

    _sendFocusWindow() {
        try {
            const win = global.display.focus_window;
            if (!win) return;
            const wmClass = win.get_wm_class() || '';
            const wmInstance = win.get_wm_class_instance() || wmClass;
            const title = win.get_title() || '';
            Gio.DBus.session.call(
                'io.github.duckydeck.Studio',
                '/KWinBridge',
                'io.github.duckydeck.KWinBridge',
                'NotifyActiveWindow',
                new GLib.Variant('(sss)', [wmClass, wmInstance, title]),
                null,
                Gio.DBusCallFlags.NONE,
                -1,
                null,
                null
            );
        } catch (e) {}
    }

    disable() {
        if (this._focusSig) {
            global.display.disconnect(this._focusSig);
            this._focusSig = null;
        }
    }
}
"#;
                    let _ = fs::write(ext_dir.join("metadata.json"), metadata_json);
                    let _ = fs::write(ext_dir.join("extension.js"), extension_js);
                }
            }
        });

        if first_time {
            if let Ok(proxy) = super::build_uncached_proxy(
                conn,
                "org.gnome.Shell.Extensions",
                "/org/gnome/Shell/Extensions",
                "org.gnome.Shell.Extensions",
            )
            .await
            {
                let _: std::result::Result<bool, _> = proxy
                    .call(
                        "EnableExtension",
                        &("duckydeck-autoswitcher@duckydeck.github.io",),
                    )
                    .await;
            }
        }
    }

    pub async fn query_gnome_introspect_active_window(
        conn: &zbus::Connection,
    ) -> Option<WindowTuple> {
        if !is_gnome_session() {
            return None;
        }
        let proxy = super::build_uncached_proxy(
            conn,
            "org.gnome.Shell.Introspect",
            "/org/gnome/Shell/Introspect",
            "org.gnome.Shell.Introspect",
        )
        .await
        .ok()?;

        let windows: std::collections::HashMap<
            u64,
            std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
        > = proxy.call("GetWindows", &()).await.ok()?;

        for props in windows.values() {
            let focused = props
                .get("focus")
                .and_then(|v| bool::try_from(v).ok())
                .unwrap_or(false);
            if !focused {
                continue;
            }
            let extract_str = |key: &str| -> String {
                props
                    .get(key)
                    .and_then(|v| <&str>::try_from(v).ok().map(|s| s.trim().to_string()))
                    .unwrap_or_default()
            };
            let wm_class = extract_str("wm-class");
            let wm_instance = extract_str("wm-class-instance");
            let app_id = extract_str("app-id")
                .trim_end_matches(".desktop")
                .to_string();
            let title = extract_str("title");

            let effective_class = if !wm_class.is_empty() {
                wm_class
            } else if !app_id.is_empty() {
                app_id
            } else {
                wm_instance.clone()
            };
            if effective_class.is_empty()
                || effective_class.to_ascii_lowercase().contains("duckydeck")
                || title.starts_with("duckyPad OLED —")
            {
                continue;
            }
            let res_name = if !wm_instance.is_empty() {
                wm_instance
            } else {
                effective_class.clone()
            };
            return Some((effective_class, res_name, title));
        }
        None
    }
}
