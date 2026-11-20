use super::kde_integration::quote_desktop_exec_binary;
#[cfg(not(target_os = "windows"))]
use super::kwin_dbus::generate_kwin6_hook_js;
#[cfg(not(target_os = "windows"))]
use super::linux_backends::{
    latest_bridge_window, linux_desktop_window, record_last_bridge_window,
};
use super::rules::{decompress_mozlz4, MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES};
use super::*;
use std::collections::HashSet;

#[test]
fn test_hold_mode_activates_after_delay_and_restores_on_release() {
    let mut engine = HostKeyboardModifierEngine::new();
    let rules = vec![ModifierLayerRule {
        enabled: true,
        trigger_key: "RCTRL".to_string(),
        mode: "hold".to_string(),
        hold_delay_ms: 200,
        oneshot_timeout_sec: 6,
        target_profile: 8,
        resource_class: String::new(),
        title_contains: None,
        title_is_regex: false,
    }];

    let mut held = HashSet::new();
    held.insert(97u16); // KEY_RIGHTCTRL

    // Press at t=1000 -> pending hold
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[97], 0, 0, 1000),
        None
    );
    // At t=1150 (< 200ms) -> still waiting
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1150),
        None
    );
    // At t=1210 (>= 200ms) -> activates Profile #8
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1210),
        Some(ModifierLayerTransition::Activated {
            target_profile: 8,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
        })
    );

    // Release RCTRL at t=1500 -> deactivates
    held.clear();
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1500),
        Some(ModifierLayerTransition::Deactivated)
    );
}

#[test]
fn test_hold_mode_chord_guard_cancels_when_another_key_pressed() {
    let mut engine = HostKeyboardModifierEngine::new();
    let rules = vec![ModifierLayerRule {
        enabled: true,
        trigger_key: "LCTRL".to_string(),
        mode: "hold".to_string(),
        hold_delay_ms: 200,
        oneshot_timeout_sec: 6,
        target_profile: 8,
        resource_class: String::new(),
        title_contains: None,
        title_is_regex: false,
    }];

    let mut held = HashSet::new();
    held.insert(29u16); // KEY_LEFTCTRL

    // Press LCTRL at t=1000
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[29], 0, 0, 1000),
        None
    );

    // User presses 'C' (evdev 46) at t=1080 for Ctrl+C
    held.insert(46u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[46], 0, 0, 1080),
        None
    );

    // Release 'C' at t=1140 while still holding LCTRL past 200ms -> must NOT activate!
    held.remove(&46u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1250),
        None
    );
}

#[test]
fn test_oneshot_mode_activates_on_tap_and_completes_after_duckypad_key() {
    let mut engine = HostKeyboardModifierEngine::new();
    let rules = vec![ModifierLayerRule {
        enabled: true,
        trigger_key: "MENU".to_string(),
        mode: "oneshot".to_string(),
        hold_delay_ms: 200,
        oneshot_timeout_sec: 6,
        target_profile: 5,
        resource_class: String::new(),
        title_contains: None,
        title_is_regex: false,
    }];

    let mut held = HashSet::new();
    held.insert(127u16); // KEY_COMPOSE (Menu)

    // Press Menu at t=2000
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[127], 0, 0, 2000),
        None
    );

    // Release Menu cleanly at t=2090 -> One-Shot activates!
    held.clear();
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 2090),
        Some(ModifierLayerTransition::Activated {
            target_profile: 5,
            trigger_key: "MENU".to_string(),
            mode: "oneshot".to_string(),
        })
    );

    // User presses a duckyPad key at t=3000 (duckypad_keys_held = 1, last_duckypad_keydown_ms = 3000)
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 1, 3000, 3000),
        None
    );

    // User releases the duckyPad key at t=3100 (duckypad_keys_held = 0) -> One-Shot completes & restores!
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 3000, 3100),
        Some(ModifierLayerTransition::Deactivated)
    );
}

#[test]
fn test_toggle_mode_locks_and_unlocks() {
    let mut engine = HostKeyboardModifierEngine::new();
    let rules = vec![ModifierLayerRule {
        enabled: true,
        trigger_key: "MENU".to_string(),
        mode: "toggle".to_string(),
        hold_delay_ms: 200,
        oneshot_timeout_sec: 6,
        target_profile: 4,
        resource_class: String::new(),
        title_contains: None,
        title_is_regex: false,
    }];

    let mut held = HashSet::new();
    held.insert(127u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[127], 0, 0, 1000),
        None
    );
    held.clear();
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1080),
        Some(ModifierLayerTransition::Activated {
            target_profile: 4,
            trigger_key: "MENU".to_string(),
            mode: "toggle".to_string(),
        })
    );

    // Tap Menu again at t=5000 to unlock
    held.insert(127u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[127], 0, 0, 5000),
        None
    );
    held.clear();
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 5080),
        Some(ModifierLayerTransition::Deactivated)
    );
}

#[test]
fn test_app_and_title_scoped_modifier_rules_override_global_all_rule() {
    let mut engine = HostKeyboardModifierEngine::new();
    let rules = vec![
        // 1. Global ("All Applications") fallback for RCTRL -> Profile #2
        ModifierLayerRule {
            enabled: true,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 2,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        },
        // 2. All Web Browsers (*browser*) rule for RCTRL -> Profile #4
        ModifierLayerRule {
            enabled: true,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 4,
            resource_class: "*browser*".to_string(),
            title_contains: None,
            title_is_regex: false,
        },
        // 3. Specific App ("firefox") rule for RCTRL -> Profile #5
        ModifierLayerRule {
            enabled: true,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 5,
            resource_class: "firefox".to_string(),
            title_contains: None,
            title_is_regex: false,
        },
        // 4. Specific App + Title ("firefox" + "YouTube") rule for RCTRL -> Profile #6
        ModifierLayerRule {
            enabled: true,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 6,
            resource_class: "firefox".to_string(),
            title_contains: Some("YouTube".to_string()),
            title_is_regex: false,
        },
    ];

    let mut held = HashSet::new();
    held.insert(97u16); // KEY_RIGHTCTRL

    // A. In Firefox on YouTube -> activates Profile #6 (App + Title specificity)
    assert_eq!(
        engine.evaluate_tick_for_window(
            true,
            false,
            &rules,
            "firefox",
            "Watch Rust Tutorials - YouTube — Mozilla Firefox",
            &held,
            &[97],
            0,
            0,
            1000,
        ),
        None
    );
    assert_eq!(
        engine.evaluate_tick_for_window(
            true,
            false,
            &rules,
            "firefox",
            "Watch Rust Tutorials - YouTube — Mozilla Firefox",
            &held,
            &[],
            0,
            0,
            1220,
        ),
        Some(ModifierLayerTransition::Activated {
            target_profile: 6,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
        })
    );

    // B. While still holding RCTRL, user switches tab in Firefox to GitHub -> shifts to Profile #5!
    assert_eq!(
        engine.evaluate_tick_for_window(
            true,
            false,
            &rules,
            "firefox",
            "duckydeck-studio — GitHub — Mozilla Firefox",
            &held,
            &[],
            0,
            0,
            1300,
        ),
        Some(ModifierLayerTransition::Activated {
            target_profile: 5,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
        })
    );

    // C. While still holding RCTRL, user focuses Chrome -> shifts to Profile #4 (*browser*)!
    assert_eq!(
        engine.evaluate_tick_for_window(
            true,
            false,
            &rules,
            "google-chrome",
            "Docs — Google Chrome",
            &held,
            &[],
            0,
            0,
            1400,
        ),
        Some(ModifierLayerTransition::Activated {
            target_profile: 4,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
        })
    );

    // D. While still holding RCTRL, user focuses Konsole -> falls back to Profile #2 (All Apps)!
    assert_eq!(
        engine.evaluate_tick_for_window(
            true,
            false,
            &rules,
            "org.kde.konsole",
            "millinr@eos-russdesk:~",
            &held,
            &[],
            0,
            0,
            1500,
        ),
        Some(ModifierLayerTransition::Activated {
            target_profile: 2,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
        })
    );

    // Release RCTRL -> deactivates
    held.clear();
    assert_eq!(
        engine.evaluate_tick_for_window(
            true,
            false,
            &rules,
            "org.kde.konsole",
            "millinr@eos-russdesk:~",
            &held,
            &[],
            0,
            0,
            1600,
        ),
        Some(ModifierLayerTransition::Deactivated)
    );
}

#[test]
fn test_rctrl_rshift_and_menu_rshift_modifier_chords() {
    assert_eq!(
        normalize_modifier_trigger_key("rightctrl-rightshift"),
        "RCTRL_RSHIFT"
    );
    assert_eq!(
        normalize_modifier_trigger_key("menu-rightshift"),
        "MENU_RSHIFT"
    );
    assert_eq!(
        modifier_trigger_key_label("RCTRL_RSHIFT"),
        "Right Ctrl + Right Shift Chord"
    );
    assert_eq!(
        modifier_trigger_key_label("MENU_RSHIFT"),
        "Menu + Right Shift Chord"
    );

    let mut engine = HostKeyboardModifierEngine::new();
    let rules = vec![
        ModifierLayerRule {
            enabled: true,
            trigger_key: "RCTRL".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 2,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        },
        ModifierLayerRule {
            enabled: true,
            trigger_key: "RCTRL_RSHIFT".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 10,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        },
        ModifierLayerRule {
            enabled: true,
            trigger_key: "MENU_RSHIFT".to_string(),
            mode: "hold".to_string(),
            hold_delay_ms: 200,
            oneshot_timeout_sec: 6,
            target_profile: 11,
            resource_class: String::new(),
            title_contains: None,
            title_is_regex: false,
        },
    ];

    // 1. Press Right Ctrl (97) at t=1000, then Right Shift (54) at t=1030
    let mut held = HashSet::new();
    held.insert(97u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[97], 0, 0, 1000),
        None
    );
    held.insert(54u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[54], 0, 0, 1030),
        None
    );
    // At t=1240 (>= 200ms after chord completed), RCTRL_RSHIFT activates Profile #10 (and RCTRL alone did not!)
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1240),
        Some(ModifierLayerTransition::Activated {
            target_profile: 10,
            trigger_key: "RCTRL_RSHIFT".to_string(),
            mode: "hold".to_string(),
        })
    );
    // Release Right Shift (54) at t=1500 while Right Ctrl (97) is still briefly held -> deactivates cleanly
    held.remove(&54u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1500),
        Some(ModifierLayerTransition::Deactivated)
    );
    held.clear();
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 1530),
        None
    );

    // 2. Press Menu (127) + Right Shift (54) at t=2000 -> activates Profile #11 after 200ms
    held.insert(127u16);
    held.insert(54u16);
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[127, 54], 0, 0, 2000),
        None
    );
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 2210),
        Some(ModifierLayerTransition::Activated {
            target_profile: 11,
            trigger_key: "MENU_RSHIFT".to_string(),
            mode: "hold".to_string(),
        })
    );
    held.clear();
    assert_eq!(
        engine.evaluate_tick(true, false, &rules, &held, &[], 0, 0, 2500),
        Some(ModifierLayerTransition::Deactivated)
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_hyprland_omarchy_active_window_and_cursor_hit_parsing() {
    let active_json = r#"{
        "address": "0x55d8f0a0",
        "mapped": true,
        "hidden": false,
        "at": [20, 48],
        "size": [1880, 1012],
        "workspace": {"id": 1, "name": "1"},
        "floating": false,
        "class": "Alacritty",
        "title": "nvim ~/Builds/duckydeck-studio",
        "initialClass": "Alacritty",
        "initialTitle": "Alacritty",
        "pid": 14322
    }"#;
    assert_eq!(
        linux_desktop_window::parse_hyprland_active_window_json(active_json),
        Some((
            "Alacritty".to_string(),
            "Alacritty".to_string(),
            "nvim ~/Builds/duckydeck-studio".to_string()
        ))
    );

    let clients_json = r#"[
        {
            "address": "0x1",
            "mapped": true,
            "hidden": false,
            "at": [0, 0],
            "size": [960, 1080],
            "workspace": {"id": 1, "name": "1"},
            "floating": false,
            "focusHistoryID": 1,
            "class": "firefox",
            "initialClass": "Navigator",
            "title": "GitHub — Mozilla Firefox"
        },
        {
            "address": "0x2",
            "mapped": true,
            "hidden": false,
            "at": [200, 200],
            "size": [500, 400],
            "workspace": {"id": 1, "name": "1"},
            "floating": true,
            "focusHistoryID": 2,
            "class": "org.gnome.Calculator",
            "initialClass": "gnome-calculator",
            "title": "Calculator"
        }
    ]"#;
    // Cursor at (300, 300) overlaps both tiled Firefox and floating Calculator; floating wins:
    assert_eq!(
        linux_desktop_window::find_hyprland_client_at_cursor_json(clients_json, Some(1), 300, 300),
        Some((
            "org.gnome.Calculator".to_string(),
            "gnome-calculator".to_string(),
            "Calculator".to_string()
        ))
    );
    // Cursor at (100, 100) hits only tiled Firefox:
    assert_eq!(
        linux_desktop_window::find_hyprland_client_at_cursor_json(clients_json, Some(1), 100, 100),
        Some((
            "firefox".to_string(),
            "Navigator".to_string(),
            "GitHub — Mozilla Firefox".to_string()
        ))
    );
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_x11_wm_class_and_wayland_state_parsing() {
    let wm_class_raw = b"Navigator\0firefox\0";
    assert_eq!(
        linux_desktop_window::parse_wm_class_bytes(wm_class_raw),
        Some(("firefox".to_string(), "Navigator".to_string()))
    );

    let mut state_bytes = Vec::new();
    state_bytes.extend_from_slice(&0u32.to_ne_bytes()); // STATE_MAXIMIZED
    assert!(!linux_desktop_window::parse_wayland_state_has_activated(
        &state_bytes
    ));
    state_bytes.extend_from_slice(&2u32.to_ne_bytes()); // STATE_ACTIVATED
    assert!(linux_desktop_window::parse_wayland_state_has_activated(
        &state_bytes
    ));
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_e2e_cosmic_and_zwlr_wayland_socket_protocol() {
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;

    fn wl_str(s: &str) -> Vec<u8> {
        let b = s.as_bytes();
        let len = (b.len() + 1) as u32;
        let mut v = Vec::new();
        v.extend_from_slice(&len.to_ne_bytes());
        v.extend_from_slice(b);
        v.push(0);
        while v.len() % 4 != 0 {
            v.push(0);
        }
        v
    }

    fn wl_msg(sender_id: u32, opcode: u16, payload: &[u8]) -> Vec<u8> {
        let size = (8 + payload.len()) as u16;
        let size_op = ((size as u32) << 16) | (opcode as u32);
        let mut v = Vec::with_capacity(8 + payload.len());
        v.extend_from_slice(&sender_id.to_ne_bytes());
        v.extend_from_slice(&size_op.to_ne_bytes());
        v.extend_from_slice(payload);
        v
    }

    let (mut server, client) = UnixStream::pair().expect("UnixStream::pair");
    server
        .set_read_timeout(Some(std::time::Duration::from_millis(500)))
        .unwrap();

    let client_thread = std::thread::spawn(move || {
        linux_desktop_window::run_wayland_toplevel_client_on_stream(client);
    });

    // 1. Read client's `wl_display.get_registry(2)` request (object 1, opcode 1, new_id = 2)
    let mut req = [0u8; 12];
    server.read_exact(&mut req).expect("read get_registry");
    assert_eq!(u32::from_ne_bytes([req[0], req[1], req[2], req[3]]), 1);
    assert_eq!(u32::from_ne_bytes([req[8], req[9], req[10], req[11]]), 2);

    // 2. Server advertises both `zcosmic_toplevel_info_v1` (global name 10) and `zwlr_foreign_toplevel_manager_v1` (global name 20)
    let mut g1 = Vec::new();
    g1.extend_from_slice(&10u32.to_ne_bytes());
    g1.extend_from_slice(&wl_str("zcosmic_toplevel_info_v1"));
    g1.extend_from_slice(&2u32.to_ne_bytes());
    server.write_all(&wl_msg(2, 0, &g1)).unwrap();

    let mut g2 = Vec::new();
    g2.extend_from_slice(&20u32.to_ne_bytes());
    g2.extend_from_slice(&wl_str("zwlr_foreign_toplevel_manager_v1"));
    g2.extend_from_slice(&3u32.to_ne_bytes());
    server.write_all(&wl_msg(2, 0, &g2)).unwrap();

    // Read the two `wl_registry.bind` requests from client (bound object IDs 3 and 4)
    let mut hdr = [0u8; 8];
    for expected_id in [3u32, 4u32] {
        server.read_exact(&mut hdr).expect("read bind hdr");
        let sz = (u32::from_ne_bytes([hdr[4], hdr[5], hdr[6], hdr[7]]) >> 16) as usize;
        let mut body = vec![0u8; sz - 8];
        server.read_exact(&mut body).expect("read bind body");
        let bound_id = u32::from_ne_bytes([
            body[body.len() - 4],
            body[body.len() - 3],
            body[body.len() - 2],
            body[body.len() - 1],
        ]);
        assert_eq!(bound_id, expected_id);
    }

    // 3. Emit a COSMIC toplevel on `cosmic_info_id` (3):
    //    - manager opcode 0: `toplevel(new_id = 0xFF000001)`
    //    - handle opcode 2: `title("COSMIC Terminal — ~/Builds")`
    //    - handle opcode 3: `app_id("com.system76.CosmicTerm")`
    //    - handle opcode 8: `state([2u32])` (`STATE_ACTIVATED = 2`)
    //    - handle opcode 1: `done()`
    let cosmic_handle = 0xFF00_0001u32;
    server
        .write_all(&wl_msg(3, 0, &cosmic_handle.to_ne_bytes()))
        .unwrap();
    server
        .write_all(&wl_msg(
            cosmic_handle,
            2,
            &wl_str("COSMIC Terminal — ~/Builds"),
        ))
        .unwrap();
    server
        .write_all(&wl_msg(
            cosmic_handle,
            3,
            &wl_str("com.system76.CosmicTerm"),
        ))
        .unwrap();
    let mut state_arr = Vec::new();
    state_arr.extend_from_slice(&4u32.to_ne_bytes());
    state_arr.extend_from_slice(&2u32.to_ne_bytes());
    server
        .write_all(&wl_msg(cosmic_handle, 8, &state_arr))
        .unwrap();
    server.write_all(&wl_msg(cosmic_handle, 1, &[])).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(30));
    let active_cosmic = linux_desktop_window::latest_wayland_active_window()
        .expect("COSMIC active window should be recorded");
    assert_eq!(active_cosmic.0, "com.system76.CosmicTerm");
    assert_eq!(active_cosmic.2, "COSMIC Terminal — ~/Builds");

    // 4. Emit a `zwlr_foreign_toplevel_manager_v1` toplevel on `zwlr_mgr_id` (4):
    //    - manager opcode 0: `toplevel(new_id = 0xFF000002)`
    //    - handle opcode 0: `title("GitHub — Mozilla Firefox")`
    //    - handle opcode 1: `app_id("firefox")`
    //    - handle opcode 4: `state([2u32])` (`STATE_ACTIVATED = 2`)
    //    - handle opcode 5: `done()`
    let zwlr_handle = 0xFF00_0002u32;
    server
        .write_all(&wl_msg(4, 0, &zwlr_handle.to_ne_bytes()))
        .unwrap();
    server
        .write_all(&wl_msg(zwlr_handle, 0, &wl_str("GitHub — Mozilla Firefox")))
        .unwrap();
    server
        .write_all(&wl_msg(zwlr_handle, 1, &wl_str("firefox")))
        .unwrap();
    server
        .write_all(&wl_msg(zwlr_handle, 4, &state_arr))
        .unwrap();
    server.write_all(&wl_msg(zwlr_handle, 5, &[])).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(30));
    let active_zwlr = linux_desktop_window::latest_wayland_active_window()
        .expect("zwlr active window should be recorded");
    assert_eq!(active_zwlr.0, "firefox");
    assert_eq!(active_zwlr.2, "GitHub — Mozilla Firefox");
    assert!(active_zwlr.3 > active_cosmic.3);

    drop(server);
    let _ = client_thread.join();
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_e2e_omarchy_hyprland_unix_socket_ipc_and_hover_hit_test() {
    use std::io::{Read, Write};
    use std::os::unix::net::UnixListener;

    let tmp_dir = std::env::temp_dir().join(format!("duckydeck-hypr-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).unwrap();
    let sock_path = tmp_dir.join(".socket.sock");
    let listener = UnixListener::bind(&sock_path).expect("bind mock Hyprland socket");

    let server_thread = std::thread::spawn(move || {
        for _ in 0..4 {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 128];
                let n = stream.read(&mut buf).unwrap_or(0);
                let cmd = String::from_utf8_lossy(&buf[..n]);
                let resp = match cmd.as_ref() {
                    "j/activewindow" => {
                        r#"{"class":"ghostty","initialClass":"com.mitchellh.ghostty","title":"omarchy — zsh"}"#
                    }
                    "j/cursorpos" => r#"{"x":1400,"y":500}"#,
                    "j/activeworkspace" => r#"{"id":2,"name":"2"}"#,
                    "j/clients" => {
                        r#"[
                            {"mapped":true,"hidden":false,"at":[0,0],"size":[960,1080],"workspace":{"id":2},"floating":false,"focusHistoryID":0,"class":"ghostty","initialClass":"com.mitchellh.ghostty","title":"omarchy — zsh"},
                            {"mapped":true,"hidden":false,"at":[960,0],"size":[960,1080],"workspace":{"id":2},"floating":false,"focusHistoryID":1,"class":"chromium","initialClass":"chromium","title":"Omarchy Manual — Chromium"}
                        ]"#
                    }
                    _ => "ok",
                };
                let _ = stream.write_all(resp.as_bytes());
            }
        }
    });

    let mut last_cursor = None;
    let (active_win, hover_win) = linux_desktop_window::query_hyprland_window_and_hover_from_socket(
        &sock_path,
        &mut last_cursor,
    );

    assert_eq!(
        active_win,
        Some((
            "ghostty".to_string(),
            "com.mitchellh.ghostty".to_string(),
            "omarchy — zsh".to_string()
        ))
    );
    assert_eq!(
        hover_win,
        Some((
            "chromium".to_string(),
            "chromium".to_string(),
            "Omarchy Manual — Chromium".to_string()
        ))
    );
    assert_eq!(last_cursor, Some((1400, 500)));

    let _ = server_thread.join();
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_e2e_cinnamon_x11_live_display_window_inspection() {
    use std::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_void};

    if std::env::var_os("DISPLAY").is_none() {
        return;
    }
    unsafe extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }
    unsafe {
        let handle = dlopen(c"libX11.so.6".as_ptr(), 1);
        if handle.is_null() {
            return;
        }
        let sym_open = dlsym(handle, c"XOpenDisplay".as_ptr());
        let sym_root = dlsym(handle, c"XDefaultRootWindow".as_ptr());
        let sym_create = dlsym(handle, c"XCreateSimpleWindow".as_ptr());
        let sym_intern = dlsym(handle, c"XInternAtom".as_ptr());
        let sym_change = dlsym(handle, c"XChangeProperty".as_ptr());
        let sym_flush = dlsym(handle, c"XFlush".as_ptr());
        let sym_destroy = dlsym(handle, c"XDestroyWindow".as_ptr());
        let sym_close = dlsym(handle, c"XCloseDisplay".as_ptr());
        if sym_open.is_null()
            || sym_root.is_null()
            || sym_create.is_null()
            || sym_intern.is_null()
            || sym_change.is_null()
            || sym_flush.is_null()
            || sym_destroy.is_null()
            || sym_close.is_null()
        {
            return;
        }

        type OpenFn = unsafe extern "C" fn(*const c_char) -> *mut c_void;
        type RootFn = unsafe extern "C" fn(*mut c_void) -> c_ulong;
        type CreateFn = unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            c_int,
            c_int,
            c_uint,
            c_uint,
            c_uint,
            c_ulong,
            c_ulong,
        ) -> c_ulong;
        type InternFn = unsafe extern "C" fn(*mut c_void, *const c_char, c_int) -> c_ulong;
        type ChangeFn = unsafe extern "C" fn(
            *mut c_void,
            c_ulong,
            c_ulong,
            c_ulong,
            c_int,
            c_int,
            *const c_uchar,
            c_int,
        ) -> c_int;
        type FlushFn = unsafe extern "C" fn(*mut c_void) -> c_int;
        type DestroyFn = unsafe extern "C" fn(*mut c_void, c_ulong) -> c_int;
        type CloseFn = unsafe extern "C" fn(*mut c_void) -> c_int;

        let x_open: OpenFn = std::mem::transmute(sym_open);
        let x_root: RootFn = std::mem::transmute(sym_root);
        let x_create: CreateFn = std::mem::transmute(sym_create);
        let x_intern: InternFn = std::mem::transmute(sym_intern);
        let x_change: ChangeFn = std::mem::transmute(sym_change);
        let x_flush: FlushFn = std::mem::transmute(sym_flush);
        let x_destroy: DestroyFn = std::mem::transmute(sym_destroy);
        let x_close: CloseFn = std::mem::transmute(sym_close);

        let dpy = x_open(std::ptr::null());
        if dpy.is_null() {
            return;
        }
        let root = x_root(dpy);
        let win = x_create(dpy, root, 0, 0, 100, 100, 0, 0, 0);
        let atom_wm_class = x_intern(dpy, c"WM_CLASS".as_ptr(), 0);
        let atom_string = x_intern(dpy, c"STRING".as_ptr(), 0);
        let atom_net_wm_name = x_intern(dpy, c"_NET_WM_NAME".as_ptr(), 0);
        let atom_utf8 = x_intern(dpy, c"UTF8_STRING".as_ptr(), 0);

        let wm_class_bytes = b"nemo\0Nemo\0";
        x_change(
            dpy,
            win,
            atom_wm_class,
            atom_string,
            8,
            0,
            wm_class_bytes.as_ptr(),
            wm_class_bytes.len() as c_int,
        );
        let title_bytes = b"Home \xE2\x80\x94 Nemo File Manager";
        x_change(
            dpy,
            win,
            atom_net_wm_name,
            atom_utf8,
            8,
            0,
            title_bytes.as_ptr(),
            title_bytes.len() as c_int,
        );
        x_flush(dpy);

        let inspected = linux_desktop_window::inspect_x11_window_id(win);
        x_destroy(dpy, win);
        x_flush(dpy);
        x_close(dpy);

        assert_eq!(
            inspected,
            Some((
                "Nemo".to_string(),
                "nemo".to_string(),
                "Home — Nemo File Manager".to_string()
            ))
        );
    }
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_e2e_gnome_bridge_notification_sequence_tracking() {
    let before_seq = latest_bridge_window().map(|w| w.3).unwrap_or(0);
    record_last_bridge_window(
        "org.gnome.Nautilus",
        "org.gnome.Nautilus",
        "Downloads — Files",
    );
    let after = latest_bridge_window().expect("bridge window should be recorded");
    assert_eq!(after.0, "org.gnome.Nautilus");
    assert_eq!(after.1, "org.gnome.Nautilus");
    assert_eq!(after.2, "Downloads — Files");
    assert!(after.3 > before_seq);
}

#[cfg(not(target_os = "windows"))]
#[test]
fn test_kwin6_hook_js_omits_cursor_pos_changed_by_default() {
    let js_default = generate_kwin6_hook_js(false, "null", "null");
    assert!(!js_default.contains("cursorPosChanged"));
    assert!(!js_default.contains("onCursorPosChanged"));
    assert!(!js_default.contains("topWindowAtPoint"));
    assert!(js_default.contains("workspace.windowActivated.connect(onWindowActivated)"));
    assert!(
        js_default.contains("workspace.activeWindow && !isDuckyDeckWindow(workspace.activeWindow)")
    );

    let js_hover = generate_kwin6_hook_js(true, "null", "null");
    assert!(js_hover.contains("workspace.cursorPosChanged.connect(onCursorPosChanged)"));
    assert!(js_hover.contains("function onCursorPosChanged()"));
    assert!(js_hover.contains("topWindowAtPoint(workspace.cursorPos)"));
}

#[test]
fn test_persisted_switcher_config_unfocused_hover_defaults_false() {
    let json_without_field = r#"{
        "enabled": true,
        "default_profile": 1,
        "rules": []
    }"#;
    let parsed: PersistedSwitcherConfig =
        serde_json::from_str(json_without_field).expect("deserialize legacy config");
    assert!(!parsed.auto_switch_unfocused_hover);
}

#[test]
fn test_kde_notifyrc_schema_and_user_override_parsing() {
    assert!(DESKTOP_NOTIFICATIONS_NOTIFYRC.contains("[Global]"));
    assert!(DESKTOP_NOTIFICATIONS_NOTIFYRC.contains("DesktopEntry=io.github.duckydeck.Studio"));
    for event_id in [
        "updateAvailable",
        "sdSyncComplete",
        "sdSyncError",
        "deviceConnection",
        "profileSwitched",
        "autoSwitcherState",
        "actionExecuted",
    ] {
        let section = format!("[Event/{event_id}]");
        assert!(
            DESKTOP_NOTIFICATIONS_NOTIFYRC.contains(&section),
            "Missing notifyrc section {section}"
        );
    }

    // Default notifyrc enables Popup for all 7 events by default
    for event_id in [
        "updateAvailable",
        "sdSyncComplete",
        "sdSyncError",
        "deviceConnection",
        "profileSwitched",
        "autoSwitcherState",
        "actionExecuted",
    ] {
        assert!(parse_notifyrc_event_popup_enabled(
            DESKTOP_NOTIFICATIONS_NOTIFYRC,
            event_id
        ));
    }

    let user_override = "\
[Event/updateAvailable]\n\
Action=\n\
\n\
[Event/profileSwitched]\n\
Action=Sound|Popup\n";
    assert!(!parse_notifyrc_event_popup_enabled(
        user_override,
        "updateAvailable"
    ));
    assert!(parse_notifyrc_event_popup_enabled(
        user_override,
        "profileSwitched"
    ));
    assert!(parse_notifyrc_event_popup_enabled(
        user_override,
        "sdSyncComplete"
    ));
}

#[test]
fn test_kde_desktop_entry_actions_and_global_shortcut_ipc_mapping() {
    let content = generate_desktop_entry_content("duckydeck-studio");
    assert!(content.contains("X-KDE-Notifications=io.github.duckydeck.Studio"));
    assert!(content.contains("X-GNOME-UsesNotifications=true"));
    assert!(content.contains("Actions=ToggleMainWindow;ShowMainWindow;MinimizeToTray;ToggleOledWidget;ToggleAutoSwitcher;NextProfile;PrevProfile;SyncSdCard;SleepDevice;WakeDevice;OpenAiArchitect;OpenEventLog;ArmWindowPicker;SwitchProfile1;SwitchProfile2;SwitchProfile3;SwitchProfile4;"));

    assert_eq!(GLOBAL_SHORTCUT_ACTIONS.len(), 17);
    for (action_id, friendly_name, cli_flag, expected_ipc) in GLOBAL_SHORTCUT_ACTIONS {
        let header = format!("[Desktop Action {action_id}]");
        assert!(
            content.contains(&header),
            "Generated desktop file missing {header}"
        );
        let exec_line = format!("Exec=duckydeck-studio {cli_flag}");
        assert!(
            content.contains(&exec_line),
            "Generated desktop file missing {exec_line}"
        );
        assert_eq!(
            global_shortcut_action_to_ipc_command(action_id),
            Some(*expected_ipc)
        );
        assert_eq!(
            global_shortcut_action_to_ipc_command(friendly_name),
            Some(*expected_ipc)
        );
        assert!(
            is_allowed_ipc_command(expected_ipc),
            "Expected IPC command '{expected_ipc}' to pass allowlist"
        );
    }

    // Verify IPC allowlist rejects multi-line injection and unknown commands
    assert!(is_allowed_ipc_command("switch-profile:OBS Studio"));
    assert!(is_allowed_ipc_command(
        "import-profile:/tmp/my.duckyprofile.json"
    ));
    assert!(!is_allowed_ipc_command("show\nquit"));
    assert!(!is_allowed_ipc_command("switch-profile:1\nrestart"));
    assert!(!is_allowed_ipc_command("arbitrary-command"));
    assert!(!is_allowed_ipc_command(""));

    // Verify desktop Exec quoting escapes special XDG characters
    assert_eq!(
        quote_desktop_exec_binary("/home/user/My $App \"Test\"/duckydeck-studio"),
        "\"/home/user/My \\$App \\\"Test\\\"/duckydeck-studio\""
    );
}

#[test]
fn test_cycle_profile_number_contiguous_sparse_and_wraparound() {
    // Empty list falls back to 1..=64 wrap-around
    assert_eq!(cycle_profile_number(&[], 1, true), 2);
    assert_eq!(cycle_profile_number(&[], 64, true), 1);
    assert_eq!(cycle_profile_number(&[], 1, false), 64);

    // Single profile stays on itself
    assert_eq!(cycle_profile_number(&[5], 5, true), 5);
    assert_eq!(cycle_profile_number(&[5], 5, false), 5);

    // Sparse unsorted profile numbers [12, 1, 4, 4] -> sorted [1, 4, 12]
    let sparse = [12, 1, 4, 4];
    assert_eq!(cycle_profile_number(&sparse, 1, true), 4);
    assert_eq!(cycle_profile_number(&sparse, 4, true), 12);
    assert_eq!(cycle_profile_number(&sparse, 12, true), 1);
    assert_eq!(cycle_profile_number(&sparse, 1, false), 12);
    assert_eq!(cycle_profile_number(&sparse, 12, false), 4);
    assert_eq!(cycle_profile_number(&sparse, 4, false), 1);
    // Current profile not in list snaps to first available profile
    assert_eq!(cycle_profile_number(&sparse, 7, true), 1);
}

#[test]
fn test_decompress_mozlz4_blocks_oversized_match_bomb() {
    // Declared uncompressed size > MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES must be rejected
    let mut huge_hdr = b"mozLz40\0".to_vec();
    huge_hdr
        .extend_from_slice(&((MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES as u32) + 1).to_le_bytes());
    assert!(decompress_mozlz4(&huge_hdr).is_none());

    // Header claims 8 uncompressed bytes; block emits 4 literals ("abcd") followed by a match of length 19 (> 8)
    let mut bomb = b"mozLz40\0".to_vec();
    bomb.extend_from_slice(&8u32.to_le_bytes());
    bomb.push(0x4F); // lit_len = 4, match_len = 15 + 4 (+ ext)
    bomb.extend_from_slice(b"abcd");
    bomb.extend_from_slice(&4u16.to_le_bytes()); // offset = 4
    bomb.push(0x00); // match_len = 19 total -> out len would become 23 > 8
    assert!(decompress_mozlz4(&bomb).is_none());

    // Valid 8-byte payload ("abcdabcd") succeeds
    let mut valid = b"mozLz40\0".to_vec();
    valid.extend_from_slice(&8u32.to_le_bytes());
    valid.push(0x40); // lit_len = 4, match_len = 4
    valid.extend_from_slice(b"abcd");
    valid.extend_from_slice(&4u16.to_le_bytes());
    assert_eq!(decompress_mozlz4(&valid), Some(b"abcdabcd".to_vec()));
}
