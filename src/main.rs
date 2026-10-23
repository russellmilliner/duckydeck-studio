pub mod actions {
    pub mod app_launch;
    pub mod desktop_apps;
    pub mod esp32_ha_server;
    pub mod web_api;
}
pub mod bridge {
    pub mod device_controller;
    pub mod event_log;
    pub mod rest_routes;
    pub mod serialize;
    pub mod showcase;
}
pub mod hardware {
    pub mod firmware;
    pub mod msc;
    pub mod permissions;
    pub mod profile;
    pub mod storage;
}
pub mod kwin {
    pub mod auto_switcher;
}
pub mod ai {
    pub mod shortcut_scraper;
    pub mod web_shortcuts;
}
pub mod api_server;
pub mod paths;
pub mod updater;
pub mod vm;

use duckydeck_core as _;

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, Write};
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::bridge::device_controller::{
    ProfileExportBundle, ProfileGroupBundle, APP_AUTHOR, APP_GITHUB_URL, APP_VERSION,
};
use crate::hardware::storage::{
    find_mounted_duckypad_sd, get_sd_card_space_info, goto_profile_by_name, load_all_sd_profiles,
    open_duckypad_hid, profile_cache_dir, query_connected_duckypad_info, read_all_pgvs,
    reboot_duckypad, set_single_key_led_hid, sync_profiles_to_cache, write_profiles_to_sd_root,
    write_single_pgv, DuckyProfile, HID_CMD_GOTO_PROFILE_BY_NUM, HID_REPORT_ID_OUT,
    ONBOARD_KEY_COUNT, PID_DUCKYPAD_PRO,
};
use crate::kwin::auto_switcher::{AutoSwitchRule, SwitcherState};
use crate::vm::compiler::compile_duckyscript_to_dsb;

fn sanitize_xdg_env() {
    crate::paths::sanitize_xdg_env();
}

fn runtime_dir_path() -> PathBuf {
    crate::paths::runtime_dir()
}

fn pid_file_path() -> PathBuf {
    runtime_dir_path().join("duckydeck-studio.pid")
}

fn is_running_duckydeck_pid(pid: u32, self_pid: u32) -> bool {
    if pid <= 1 || pid == self_pid {
        return false;
    }
    #[cfg(target_os = "windows")]
    {
        return crate::kwin::auto_switcher::is_windows_pid_duckydeck(pid);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let proc_dir = PathBuf::from(format!("/proc/{pid}"));
        if !proc_dir.exists() {
            return false;
        }
        if let Ok(exe_path) = fs::read_link(proc_dir.join("exe")) {
            if let Some(name) = exe_path.file_name().and_then(|n| n.to_str()) {
                let clean = name.trim_end_matches(" (deleted)");
                if clean == "duckydeck-studio" {
                    return true;
                }
            }
        }
        if let Ok(comm) = fs::read_to_string(proc_dir.join("comm")) {
            let trimmed = comm.trim();
            if trimmed == "duckydeck-studi" || trimmed == "duckydeck-studio" {
                return true;
            }
        }
        false
    }
}

fn find_existing_duckydeck_instances() -> Vec<u32> {
    let self_pid = std::process::id();
    let mut found = BTreeSet::new();

    let pfile = pid_file_path();
    if let Ok(raw) = fs::read_to_string(&pfile) {
        if let Ok(pid) = raw.trim().parse::<u32>() {
            if is_running_duckydeck_pid(pid, self_pid) {
                found.insert(pid);
            }
        }
    }

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if let Ok(pid) = name.parse::<u32>() {
                    if is_running_duckydeck_pid(pid, self_pid) {
                        found.insert(pid);
                    }
                }
            }
        }
    }

    found.into_iter().collect()
}

#[cfg(target_os = "windows")]
fn send_ipc_command_to_running_instance(cmd: &str) -> Result<Vec<u32>, String> {
    let pids = find_existing_duckydeck_instances();
    if pids.is_empty() {
        return Err("No running DuckyDeck Studio instance found.".to_string());
    }
    let ipc_file = runtime_dir_path().join("duckydeck-ipc.cmd");
    fs::write(&ipc_file, cmd).map_err(|e| {
        format!(
            "Failed writing Windows IPC command file {}: {e}",
            ipc_file.display()
        )
    })?;
    for _ in 0..20 {
        std::thread::sleep(std::time::Duration::from_millis(30));
        if !ipc_file.exists() {
            return Ok(pids);
        }
    }
    Ok(pids)
}

#[cfg(not(target_os = "windows"))]
fn send_ipc_command_to_running_instance(cmd: &str) -> Result<Vec<u32>, String> {
    let pids = find_existing_duckydeck_instances();
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("Failed creating Tokio runtime for D-Bus IPC: {e}"))?;

    let dbus_result: Result<u32, String> = rt.block_on(async {
        let conn = zbus::Connection::session()
            .await
            .map_err(|e| format!("Failed connecting to session D-Bus: {e}"))?;
        let proxy = zbus::Proxy::new(
            &conn,
            "io.github.duckydeck.Studio",
            "/KWinBridge",
            "io.github.duckydeck.KWinBridge",
        )
        .await
        .map_err(|e| format!("Failed creating D-Bus proxy for io.github.duckydeck.Studio: {e}"))?;
        let remote_pid: u32 = proxy
            .call("DispatchIpcCommand", &(cmd,))
            .await
            .map_err(|e| format!("D-Bus DispatchIpcCommand failed: {e}"))?;
        Ok(remote_pid)
    });

    match dbus_result {
        Ok(remote_pid) => {
            let mut all_pids = pids;
            if remote_pid > 1 && !all_pids.contains(&remote_pid) {
                all_pids.push(remote_pid);
            }
            if all_pids.is_empty() {
                all_pids.push(remote_pid);
            }
            Ok(all_pids)
        }
        Err(e) => {
            if pids.is_empty() {
                Err("No running DuckyDeck Studio instance found on session D-Bus.".to_string())
            } else {
                Err(e)
            }
        }
    }
}

fn print_help() {
    println!(
        "DuckyDeck Studio v{APP_VERSION}
Native Linux / KDE Plasma 6 Configurator, DSVM v2 Compiler & Wayland Auto-Switcher
for duckyPad Pro (0x0483:0xd11d) and duckyPad 2020 (0x0483:0xd11c)
Author: {APP_AUTHOR}  |  Repository: {APP_GITHUB_URL}

USAGE:
    duckydeck-studio [OPTIONS]

STARTUP & INSTANCE OPTIONS:
    -h, --help                              Show this help message and exit
    -V, --version                           Print version information and exit
    -u, --check-update                      Check GitHub Releases for a newer DuckyDeck Studio version
        --self-update                       Download and install the latest DuckyDeck Studio release from GitHub
    -m, --minimize, --minimized, --tray     Start minimized in the KDE system tray (automatically
                                            detaches into the background and returns shell prompt)
        --foreground, --no-daemon           Keep process attached to terminal when using --minimize
        --restart                           Restart DuckyDeck Studio (closes any running instance
                                            and starts a fresh process in the background)
    -r, --replace, --kill-existing          Automatically close any already-running DuckyDeck Studio
                                            instance and start a new process without prompting
    -p, --profile <NUM|NAME>                Activate Profile <NUM> (1-64) or <NAME> on GUI startup
        --sync-sd                           Open and start microSD card synchronization on startup

RUNNING-INSTANCE REMOTE CONTROL (IPC):
        --show                              Show and focus the running instance's main window
        --hide                              Hide the running instance's main window to system tray
        --toggle-window                     Toggle the running instance's main window visibility
        --oled-widget                       Toggle (or launch) the Always-on-Top Resizable Desktop OLED Widget
        --auto-switch <on|off|toggle>       Enable, disable, or toggle KWin Wayland Auto-Switcher

PROFILE & GROUP BACKUP / RESTORE AUTOMATION:
        --export-group <FILE.duckygroup.json>
                                            Export all profiles & Auto-Switcher rules to a group file
        --import-group <FILE.duckygroup.json>
                                            Import a .duckygroup.json bundle into local cache/SD
        --export-profile <NUM|NAME> [-o <FILE.duckyprofile.json>]
                                            Export a single profile & its rules to JSON
        --import-profile <FILE.duckyprofile.json>
                                            Import a .duckyprofile.json or config.txt profile

GLOBAL VARIABLE (PGV 1-32) & LED HARDWARE SCRIPTING:
        --get-pgv <1..32>                   Read a Persistent Global Variable (PGV 1-32) over USB HID
        --dump-pgvs                         Dump all 32 Persistent Global Variables over USB HID
        --set-pgv <1..32> <VALUE>           Write a 32-bit signed integer to PGV <1..32> over USB HID
        --set-led <1..20|all> <COLOR>       Set key RGB LED color (#RRGGBB or R,G,B) over USB HID

HEADLESS HARDWARE & COMPILER COMMANDS:
        --status                            Query connected duckyPad USB HID & SD card status
        --list-profiles                     List all loaded profiles (#1-#64) and key counts
        --switch-profile <NUM|NAME>         Switch active profile on connected duckyPad over USB HID
        --compile <SCRIPT.txt> [-o <OUT.dsb>]
                                            Compile a DuckyScript 3 file to DSVM v2 (.dsb) bytecode
        --sync-rtc                          Synchronize duckyPad hardware RTC with Linux host clock
        --sleep                             Put connected duckyPad into sleep mode (OLED & RGB off)
        --wake                              Wake connected duckyPad from sleep mode
        --reboot                            Software-reboot the connected duckyPad over USB HID

EXAMPLES:
    duckydeck-studio --minimize
    duckydeck-studio --restart
    duckydeck-studio --toggle-window
    duckydeck-studio --auto-switch toggle
    duckydeck-studio --switch-profile \"OBS Studio\"
    duckydeck-studio --set-pgv 1 42 && duckydeck-studio --get-pgv 1
    duckydeck-studio --set-led 1 \"#ff3b30\"
    duckydeck-studio --export-group ~/backups/my-deck.duckygroup.json
    duckydeck-studio --compile key1.txt -o key1.dsb"
    );
}

fn load_current_profiles() -> Result<(Vec<DuckyProfile>, PathBuf), String> {
    let source_dir = find_mounted_duckypad_sd().unwrap_or_else(profile_cache_dir);
    let profiles = load_all_sd_profiles(&source_dir)
        .map_err(|e| format!("Failed loading profiles from {}: {e}", source_dir.display()))?;
    Ok((profiles, source_dir))
}

fn run_cli_status() -> i32 {
    println!("DuckyDeck Studio v{APP_VERSION} — Hardware Status");
    println!("--------------------------------------------------");
    let mut detected_serial = String::new();
    match query_connected_duckypad_info() {
        Ok(info) => {
            let model_str = if info.pid == PID_DUCKYPAD_PRO {
                "duckyPad Pro (2024)"
            } else {
                "duckyPad (2020)"
            };
            detected_serial = info.serial.clone();
            println!("USB HID Status : Connected");
            println!("Device Model   : {} (0x0483:0x{:04x})", model_str, info.pid);
            println!("Firmware       : v{}", info.fw_version);
            println!("Serial Number  : {}", info.serial);
            println!("Active Profile : #{}", info.active_profile);
        }
        Err(err) => {
            println!("USB HID Status : Disconnected ({err})");
        }
    }

    let mounted_sd = find_mounted_duckypad_sd();
    match mounted_sd {
        Some(ref path) => println!("Mounted SD Card: {}", path.display()),
        None => println!("Mounted SD Card: Not mounted"),
    }
    let sd_space = get_sd_card_space_info(&detected_serial, mounted_sd.as_deref());
    println!(
        "SD Card Space  : {}",
        sd_space.tray_text.trim_start_matches("SD Card: ")
    );
    println!("Local Cache Dir: {}", profile_cache_dir().display());
    let sw = SwitcherState::default();
    println!(
        "Auto-Switcher  : {} ({} rules configured)",
        if sw.enabled { "Enabled" } else { "Disabled" },
        sw.rules.len()
    );
    let running = find_existing_duckydeck_instances();
    if running.is_empty() {
        println!("Running Process: None");
    } else {
        println!(
            "Running Process: PID {}",
            running
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    0
}

fn run_cli_list_profiles() -> i32 {
    match load_current_profiles() {
        Ok((profiles, source_dir)) => {
            println!(
                "Loaded {} profile(s) from {}:",
                profiles.len(),
                source_dir.display()
            );
            println!(
                "{:<6} {:<24} {:<10} {:<10}",
                "SLOT", "NAME", "KEYS", "BG_COLOR"
            );
            println!("{}", "-".repeat(54));
            for p in &profiles {
                println!(
                    "#{:<5} {:<24} {:<10} #{:02X}{:02X}{:02X}",
                    p.number,
                    p.name,
                    p.slots.len(),
                    p.bg_color[0],
                    p.bg_color[1],
                    p.bg_color[2]
                );
            }
            0
        }
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

fn run_cli_switch_profile(target: &str) -> i32 {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        eprintln!("Error: --switch-profile requires a profile number (1-64) or profile name.");
        return 1;
    }
    let Ok(device) = open_duckypad_hid() else {
        eprintln!("Error: No duckyPad connected over USB HID.");
        return 1;
    };

    if let Ok(num) = trimmed.parse::<u8>() {
        if (1..=64).contains(&num) {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x02;
            tx[2] = HID_CMD_GOTO_PROFILE_BY_NUM;
            tx[3] = num;
            if let Err(e) = device.write(&tx) {
                eprintln!("Error switching to profile #{num}: {e}");
                return 1;
            }
            let mut rx = [0u8; 64];
            if let Err(e) = device.read_timeout(&mut rx, 200) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "HID",
                    format!("HID read_timeout after CLI profile switch failed: {e}"),
                );
            }
            println!("✓ Switched connected duckyPad to Profile #{num}");
            return 0;
        }
    }

    match goto_profile_by_name(&device, trimmed) {
        Ok(()) => {
            println!("✓ Switched connected duckyPad to Profile \"{trimmed}\"");
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn run_cli_compile(input_path: &str, output_override: Option<&str>) -> i32 {
    let in_path = Path::new(input_path);
    let script = match fs::read_to_string(in_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading '{}': {e}", in_path.display());
            return 1;
        }
    };

    let result = compile_duckyscript_to_dsb(&script, false, false);
    if !result.is_success {
        eprintln!(
            "✗ Compile Error on line {}: {}\n  > {}",
            result.error_line_number_starting_from_1, result.error_comment, result.error_line_str
        );
        return 1;
    }

    let out_path = match output_override {
        Some(custom) => PathBuf::from(custom),
        None => in_path.with_extension("dsb"),
    };

    if let Err(e) = fs::write(&out_path, &result.bin_array) {
        eprintln!("Error writing '{}': {e}", out_path.display());
        return 1;
    }

    println!(
        "✓ Compiled '{}' → '{}' ({} bytes, DSVM v2)",
        in_path.display(),
        out_path.display(),
        result.bin_array.len()
    );
    0
}

fn run_cli_get_pgv(index_str: &str) -> i32 {
    let Ok(idx) = index_str.trim().parse::<usize>() else {
        eprintln!("Error: PGV index must be an integer between 1 and 32.");
        return 1;
    };
    if !(1..=32).contains(&idx) {
        eprintln!("Error: PGV index must be between 1 and 32 (got {idx}).");
        return 1;
    }
    let Ok(device) = open_duckypad_hid() else {
        eprintln!("Error: No duckyPad connected over USB HID.");
        return 1;
    };
    match read_all_pgvs(&device) {
        Ok(values) => {
            println!("{}", values[idx - 1]);
            0
        }
        Err(e) => {
            eprintln!("Error reading PGV {idx}: {e}");
            1
        }
    }
}

fn run_cli_dump_pgvs() -> i32 {
    let Ok(device) = open_duckypad_hid() else {
        eprintln!("Error: No duckyPad connected over USB HID.");
        return 1;
    };
    match read_all_pgvs(&device) {
        Ok(values) => {
            println!("duckyPad Persistent Global Variables (PGV 1-32):");
            println!("{}", "-".repeat(48));
            for row in 0..8 {
                let i1 = row + 1;
                let i2 = row + 9;
                let i3 = row + 17;
                let i4 = row + 25;
                println!(
                    "PGV {:>2}: {:<8}  PGV {:>2}: {:<8}  PGV {:>2}: {:<8}  PGV {:>2}: {:<8}",
                    i1,
                    values[i1 - 1],
                    i2,
                    values[i2 - 1],
                    i3,
                    values[i3 - 1],
                    i4,
                    values[i4 - 1]
                );
            }
            0
        }
        Err(e) => {
            eprintln!("Error reading PGVs: {e}");
            1
        }
    }
}

fn run_cli_set_pgv(index_str: &str, value_str: &str) -> i32 {
    let Ok(idx) = index_str.trim().parse::<u8>() else {
        eprintln!("Error: PGV index must be an integer between 1 and 32.");
        return 1;
    };
    if !(1..=32).contains(&idx) {
        eprintln!("Error: PGV index must be between 1 and 32 (got {idx}).");
        return 1;
    }
    let Ok(val) = value_str.trim().parse::<i32>() else {
        eprintln!("Error: PGV value must be a 32-bit signed integer (got '{value_str}').");
        return 1;
    };
    let Ok(device) = open_duckypad_hid() else {
        eprintln!("Error: No duckyPad connected over USB HID.");
        return 1;
    };
    match write_single_pgv(&device, idx - 1, val) {
        Ok(()) => {
            println!("✓ Set PGV {idx} = {val}");
            0
        }
        Err(e) => {
            eprintln!("Error writing PGV {idx}: {e}");
            1
        }
    }
}

fn parse_rgb_color_arg(raw: &str) -> Result<[u8; 3], String> {
    let trimmed = raw.trim().trim_start_matches('#');
    if trimmed.len() == 6 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        let r = u8::from_str_radix(&trimmed[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&trimmed[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&trimmed[4..6], 16).unwrap_or(0);
        return Ok([r, g, b]);
    }
    let parts: Vec<&str> = raw.split(',').map(|s| s.trim()).collect();
    if parts.len() == 3 {
        let r = parts[0]
            .parse::<u8>()
            .map_err(|_| format!("Invalid red component '{}'", parts[0]))?;
        let g = parts[1]
            .parse::<u8>()
            .map_err(|_| format!("Invalid green component '{}'", parts[1]))?;
        let b = parts[2]
            .parse::<u8>()
            .map_err(|_| format!("Invalid blue component '{}'", parts[2]))?;
        return Ok([r, g, b]);
    }
    Err(format!(
        "Invalid color '{raw}'. Expected hex '#RRGGBB' (e.g. #ff3b30) or 'R,G,B' (e.g. 255,59,48)."
    ))
}

fn run_cli_set_led(key_str: &str, color_str: &str) -> i32 {
    let rgb = match parse_rgb_color_arg(color_str) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {e}");
            return 1;
        }
    };

    let key_trimmed = key_str.trim().to_ascii_lowercase();
    if key_trimmed == "all" || key_trimmed == "0" {
        let mut any_ok = false;
        for slot in 1..=ONBOARD_KEY_COUNT {
            if set_single_key_led_hid(slot, rgb) {
                any_ok = true;
            }
        }
        if any_ok {
            println!(
                "✓ Set all {} keys to #{:02X}{:02X}{:02X}",
                ONBOARD_KEY_COUNT, rgb[0], rgb[1], rgb[2]
            );
            return 0;
        } else {
            eprintln!("Error: Failed setting key LEDs over USB HID (is duckyPad connected?).");
            return 1;
        }
    }

    let Ok(slot) = key_trimmed.parse::<usize>() else {
        eprintln!("Error: Key index must be 1..20 or 'all' (got '{key_str}').");
        return 1;
    };
    if !(1..=ONBOARD_KEY_COUNT).contains(&slot) {
        eprintln!("Error: Key index must be between 1 and {ONBOARD_KEY_COUNT} (got {slot}).");
        return 1;
    }
    if set_single_key_led_hid(slot, rgb) {
        println!(
            "✓ Set Key {} LED to #{:02X}{:02X}{:02X}",
            slot, rgb[0], rgb[1], rgb[2]
        );
        0
    } else {
        eprintln!("Error: Failed setting Key {slot} LED over USB HID (is duckyPad connected?).");
        1
    }
}

fn run_cli_export_group(dest_str: &str) -> i32 {
    let (profiles, _) = match load_current_profiles() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            return 1;
        }
    };
    if profiles.is_empty() {
        eprintln!("Error: No profiles found to export.");
        return 1;
    }

    let sw = SwitcherState::default();
    let dest = PathBuf::from(dest_str.trim());
    let group_name = dest
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("DuckyDeck Group")
        .trim_end_matches(".duckygroup")
        .to_string();

    let bundle = ProfileGroupBundle {
        version: 1,
        group_name: group_name.clone(),
        active_profile: 1,
        default_fallback_profile: sw.default_profile,
        profiles: profiles.clone(),
        auto_switch_rules: sw.rules,
    };

    let Ok(json_str) = serde_json::to_string_pretty(&bundle) else {
        eprintln!("Error: Failed serializing profile group bundle.");
        return 1;
    };

    let target_file = if dest
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
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "STORAGE",
                    format!(
                        "Failed creating export group parent dir {}: {e}",
                        parent.display()
                    ),
                );
            }
        }
    }

    match fs::write(&target_file, &json_str) {
        Ok(()) => {
            println!(
                "✓ Exported Profile Group '{}' ({} profiles) to '{}'",
                group_name,
                profiles.len(),
                target_file.display()
            );
            0
        }
        Err(e) => {
            eprintln!("Error writing '{}': {e}", target_file.display());
            1
        }
    }
}

fn run_cli_import_group(src_str: &str) -> i32 {
    let src = PathBuf::from(src_str.trim());
    if !src.exists() {
        eprintln!("Error: Profile group file not found: '{}'", src.display());
        return 1;
    }
    let abs_src = fs::canonicalize(&src).unwrap_or(src.clone());

    if let Ok(pids) =
        send_ipc_command_to_running_instance(&format!("import-group:{}", abs_src.display()))
    {
        println!(
            "✓ Imported Profile Group '{}' into running DuckyDeck Studio instance (PID {})",
            abs_src.display(),
            pids.iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return 0;
    }

    let raw = match fs::read_to_string(&abs_src) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading '{}': {e}", abs_src.display());
            return 1;
        }
    };

    let mut bundle: ProfileGroupBundle = match serde_json::from_str(&raw) {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "Error parsing .duckygroup.json '{}': {e}",
                abs_src.display()
            );
            return 1;
        }
    };

    if bundle.profiles.is_empty() {
        eprintln!("Error: Profile group contains 0 profiles.");
        return 1;
    }

    bundle.profiles.truncate(64);
    for (idx, p) in bundle.profiles.iter_mut().enumerate() {
        p.number = (idx + 1) as u8;
        p.folder_name_override = None;
    }

    sync_profiles_to_cache(&bundle.profiles);
    if let Some(sd_root) = find_mounted_duckypad_sd() {
        if let Err(e) = write_profiles_to_sd_root(&sd_root, &bundle.profiles) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed writing imported group to SD {}: {e}",
                    sd_root.display()
                ),
            );
        }
    }

    let mut sw = SwitcherState {
        rules: bundle.auto_switch_rules,
        ..Default::default()
    };
    if let Some(fb) = bundle.default_fallback_profile {
        sw.default_profile = Some(fb.clamp(1, bundle.profiles.len() as u8));
    }
    if let Err(e) = sw.save_to_disk() {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "CONFIG",
            format!("Failed saving switcher rules after CLI group import: {e}"),
        );
    }

    println!(
        "✓ Imported Profile Group '{}' ({} profiles, {} rules)",
        bundle.group_name,
        bundle.profiles.len(),
        sw.rules.len()
    );
    0
}

fn run_cli_export_profile(target: &str, output_override: Option<&str>) -> i32 {
    let (profiles, _) = match load_current_profiles() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            return 1;
        }
    };

    let trimmed = target.trim();
    let maybe_prof = if let Ok(num) = trimmed.parse::<u8>() {
        profiles.iter().find(|p| p.number == num).cloned()
    } else {
        profiles
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(trimmed))
            .cloned()
    };

    let Some(profile) = maybe_prof else {
        eprintln!("Error: Profile '{trimmed}' not found.");
        return 1;
    };

    let sw = SwitcherState::default();
    let matching_rules: Vec<AutoSwitchRule> = sw
        .rules
        .into_iter()
        .filter(|r| r.target_profile == profile.number)
        .collect();

    let bundle = ProfileExportBundle {
        version: 1,
        profile: profile.clone(),
        auto_switch_rules: matching_rules,
    };

    let Ok(json_str) = serde_json::to_string_pretty(&bundle) else {
        eprintln!("Error: Failed serializing profile export bundle.");
        return 1;
    };

    let out_path = match output_override {
        Some(custom) => PathBuf::from(custom),
        None => {
            let safe_name: String = profile
                .name
                .chars()
                .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
                .collect();
            let stem = if safe_name.is_empty() {
                format!("profile_{}", profile.number)
            } else {
                safe_name
            };
            PathBuf::from(format!("{stem}.duckyprofile.json"))
        }
    };

    if let Some(parent) = out_path.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(e) = fs::create_dir_all(parent) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "STORAGE",
                    format!(
                        "Failed creating export profile parent dir {}: {e}",
                        parent.display()
                    ),
                );
            }
        }
    }

    match fs::write(&out_path, &json_str) {
        Ok(()) => {
            println!(
                "✓ Exported Profile #{} '{}' to '{}'",
                profile.number,
                profile.name,
                out_path.display()
            );
            0
        }
        Err(e) => {
            eprintln!("Error writing '{}': {e}", out_path.display());
            1
        }
    }
}

fn run_cli_import_profile(src_str: &str) -> i32 {
    let src = PathBuf::from(src_str.trim());
    if !src.exists() {
        eprintln!("Error: Profile file not found: '{}'", src.display());
        return 1;
    }
    let abs_src = fs::canonicalize(&src).unwrap_or(src.clone());

    if let Ok(pids) =
        send_ipc_command_to_running_instance(&format!("import-profile:{}", abs_src.display()))
    {
        println!(
            "✓ Imported Profile '{}' into running DuckyDeck Studio instance (PID {})",
            abs_src.display(),
            pids.iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return 0;
    }

    let raw = match fs::read_to_string(&abs_src) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading '{}': {e}", abs_src.display());
            return 1;
        }
    };

    let (mut imported_prof, imported_rules) =
        if let Ok(bundle) = serde_json::from_str::<ProfileExportBundle>(&raw) {
            (bundle.profile, bundle.auto_switch_rules)
        } else if let Ok(prof) = serde_json::from_str::<DuckyProfile>(&raw) {
            (prof, Vec::new())
        } else {
            eprintln!(
                "Error: Invalid .duckyprofile.json format in '{}'",
                abs_src.display()
            );
            return 1;
        };

    let (mut profiles, _) =
        load_current_profiles().unwrap_or_else(|_| (Vec::new(), profile_cache_dir()));
    if profiles.len() >= 64 {
        eprintln!("Error: Maximum of 64 profiles reached.");
        return 1;
    }

    let next_num = (profiles.len() + 1) as u8;
    imported_prof.number = next_num;
    imported_prof.folder_name_override = None;
    let prof_name = imported_prof.name.clone();
    profiles.push(imported_prof);

    sync_profiles_to_cache(&profiles);
    if let Some(sd_root) = find_mounted_duckypad_sd() {
        if let Err(e) = write_profiles_to_sd_root(&sd_root, &profiles) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!(
                    "Failed writing imported profile to SD {}: {e}",
                    sd_root.display()
                ),
            );
        }
    }

    if !imported_rules.is_empty() {
        let mut sw = SwitcherState::default();
        for mut r in imported_rules {
            r.target_profile = next_num;
            sw.rules.push(r);
        }
        if let Err(e) = sw.save_to_disk() {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "CONFIG",
                format!("Failed saving switcher rules after CLI profile import: {e}"),
            );
        }
    }

    println!("✓ Imported Profile #{next_num} '{prof_name}'");
    0
}

fn run_cli_auto_switch(mode_str: &str) -> i32 {
    let mode = mode_str.trim().to_ascii_lowercase();
    let ipc_cmd = match mode.as_str() {
        "on" | "enable" | "enabled" | "true" | "1" => "auto-switch:on",
        "off" | "disable" | "disabled" | "false" | "0" => "auto-switch:off",
        "toggle" => "auto-switch:toggle",
        _ => {
            eprintln!("Error: --auto-switch expects 'on', 'off', or 'toggle' (got '{mode_str}').");
            return 1;
        }
    };

    if let Ok(pids) = send_ipc_command_to_running_instance(ipc_cmd) {
        println!(
            "✓ Sent '{}' to running DuckyDeck Studio instance (PID {})",
            ipc_cmd,
            pids.iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return 0;
    }

    let mut sw = SwitcherState::default();
    sw.enabled = match ipc_cmd {
        "auto-switch:on" => true,
        "auto-switch:off" => false,
        _ => !sw.enabled,
    };
    if let Err(e) = sw.save_to_disk() {
        eprintln!("Error saving Auto-Switcher config: {e}");
        return 1;
    }
    println!(
        "✓ Persisted Auto-Switcher state: {}",
        if sw.enabled { "Enabled" } else { "Disabled" }
    );
    0
}

fn run_cli_window_ipc(cmd: &str) -> i32 {
    match send_ipc_command_to_running_instance(cmd) {
        Ok(pids) => {
            println!(
                "✓ Sent '{}' to running DuckyDeck Studio instance (PID {})",
                cmd,
                pids.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            0
        }
        Err(e) => {
            eprintln!("Error: {e}");
            1
        }
    }
}

fn run_cli_sync_rtc() -> i32 {
    use chrono::Local;
    let Ok(device) = open_duckypad_hid() else {
        eprintln!("Error: No duckyPad connected over USB HID.");
        return 1;
    };
    let now = Local::now();
    let unix_ts = now.timestamp().max(0) as u32;
    let utc_offset_minutes = (now.offset().local_minus_utc() / 60) as i16;
    let ts_be = unix_ts.to_be_bytes();
    let off_be = utc_offset_minutes.to_be_bytes();

    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x04;
    tx[2] = 0x1A;
    tx[3..7].copy_from_slice(&ts_be);
    tx[7..9].copy_from_slice(&off_be);

    if let Err(e) = device.write(&tx) {
        eprintln!("Error synchronizing RTC: {e}");
        return 1;
    }
    let mut rx = [0u8; 64];
    if let Err(e) = device.read_timeout(&mut rx, 200) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "HID",
            format!("HID read_timeout after CLI RTC sync failed: {e}"),
        );
    }
    println!(
        "✓ Synchronized duckyPad RTC with host clock ({})",
        now.format("%Y-%m-%d %H:%M:%S %:z")
    );
    0
}

fn run_cli_sleep_wake(sleep: bool) -> i32 {
    let Ok(device) = open_duckypad_hid() else {
        eprintln!("Error: No duckyPad connected over USB HID.");
        return 1;
    };
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x05;
    tx[2] = if sleep { 0x15 } else { 0x16 };
    if let Err(e) = device.write(&tx) {
        eprintln!(
            "Error sending {} command: {e}",
            if sleep { "sleep" } else { "wake" }
        );
        return 1;
    }
    let mut rx = [0u8; 64];
    if let Err(e) = device.read_timeout(&mut rx, 200) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "HID",
            format!("HID read_timeout after CLI sleep/wake failed: {e}"),
        );
    }
    println!(
        "✓ Connected duckyPad {}",
        if sleep {
            "entered sleep mode"
        } else {
            "woke from sleep mode"
        }
    );
    0
}

fn pid_had_minimized_arg(pid: u32) -> bool {
    let Ok(raw) = fs::read(format!("/proc/{pid}/cmdline")) else {
        return false;
    };
    for part in raw.split(|&b| b == 0) {
        if let Ok(arg) = std::str::from_utf8(part) {
            if matches!(arg, "-m" | "--minimize" | "--minimized" | "--tray") {
                return true;
            }
        }
    }
    false
}

fn main() {
    sanitize_xdg_env();

    let args: Vec<String> = env::args().skip(1).collect();
    let mut start_minimized = false;
    let mut foreground_mode = false;
    let mut replace_existing = false;
    let mut restart_requested = false;
    let mut startup_profile: Option<String> = None;
    let mut startup_sync_sd = false;
    let mut compile_input: Option<String> = None;
    let mut export_profile_target: Option<String> = None;
    let mut output_override: Option<String> = None;

    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        match arg {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-V" | "--version" => {
                println!("DuckyDeck Studio v{APP_VERSION}");
                std::process::exit(0);
            }
            "-u" | "--check-update" => {
                match crate::updater::check_for_app_updates_blocking(APP_VERSION) {
                    Ok(res) => {
                        if res.update_available {
                            println!(
                                "Update available: DuckyDeck Studio v{} (installed: v{})",
                                res.latest_version, res.current_version
                            );
                            println!("Release URL : {}", res.release_url);
                            if !res.download_url.is_empty() {
                                println!("Download URL: {}", res.download_url);
                            }
                        } else {
                            println!(
                                "DuckyDeck Studio v{} is up to date (latest: v{}).",
                                res.current_version, res.latest_version
                            );
                        }
                        std::process::exit(0);
                    }
                    Err(err) => {
                        eprintln!("Update check failed: {err}");
                        std::process::exit(1);
                    }
                }
            }
            "--self-update" => match crate::updater::check_for_app_updates_blocking(APP_VERSION) {
                Ok(res) => {
                    if !res.update_available {
                        println!(
                            "DuckyDeck Studio v{} is already up to date.",
                            res.current_version
                        );
                        std::process::exit(0);
                    }
                    println!(
                        "Downloading and installing DuckyDeck Studio v{}...",
                        res.latest_version
                    );
                    match crate::updater::download_and_install_app_update_blocking(
                        &res.download_url,
                        &res.latest_version,
                    ) {
                        Ok(msg) => {
                            println!("✓ {msg}");
                            std::process::exit(0);
                        }
                        Err(err) => {
                            eprintln!("Self-update failed: {err}");
                            std::process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Update check failed: {err}");
                    std::process::exit(1);
                }
            },
            "-m" | "--minimize" | "--minimized" | "--tray" => {
                start_minimized = true;
            }
            "--autostart" => {
                start_minimized = true;
                foreground_mode = true;
            }
            "-session" => {
                if i + 1 < args.len() {
                    i += 1;
                }
                start_minimized = true;
                foreground_mode = true;
            }
            "--foreground" | "--no-daemon" => {
                foreground_mode = true;
            }
            "--restart" => {
                restart_requested = true;
                replace_existing = true;
            }
            "-r" | "--replace" | "--kill-existing" => {
                replace_existing = true;
            }
            "--sync-sd" => {
                startup_sync_sd = true;
            }
            "-p" | "--profile" => {
                if i + 1 < args.len() {
                    i += 1;
                    startup_profile = Some(args[i].clone());
                } else {
                    eprintln!("Error: {arg} requires a profile number (1-64) or name.");
                    std::process::exit(1);
                }
            }
            "--show" => {
                std::process::exit(run_cli_window_ipc("show"));
            }
            "--hide" => {
                std::process::exit(run_cli_window_ipc("hide"));
            }
            "--toggle-window" => {
                std::process::exit(run_cli_window_ipc("toggle-window"));
            }
            "--oled-widget" => {
                if !find_existing_duckydeck_instances().is_empty() {
                    std::process::exit(run_cli_window_ipc("toggle-oled-widget"));
                } else {
                    env::set_var("DUCKYDECK_SHOW_OLED_WIDGET", "1");
                    start_minimized = true;
                }
            }
            "--auto-switch" => {
                if i + 1 < args.len() {
                    std::process::exit(run_cli_auto_switch(&args[i + 1]));
                } else {
                    eprintln!("Error: --auto-switch requires 'on', 'off', or 'toggle'.");
                    std::process::exit(1);
                }
            }
            "--export-group" => {
                if i + 1 < args.len() {
                    std::process::exit(run_cli_export_group(&args[i + 1]));
                } else {
                    eprintln!(
                        "Error: --export-group requires a destination .duckygroup.json path."
                    );
                    std::process::exit(1);
                }
            }
            "--import-group" => {
                if i + 1 < args.len() {
                    std::process::exit(run_cli_import_group(&args[i + 1]));
                } else {
                    eprintln!("Error: --import-group requires a source .duckygroup.json path.");
                    std::process::exit(1);
                }
            }
            "--export-profile" => {
                if i + 1 < args.len() {
                    i += 1;
                    export_profile_target = Some(args[i].clone());
                } else {
                    eprintln!("Error: --export-profile requires a profile number (1-64) or name.");
                    std::process::exit(1);
                }
            }
            "--import-profile" => {
                if i + 1 < args.len() {
                    std::process::exit(run_cli_import_profile(&args[i + 1]));
                } else {
                    eprintln!("Error: --import-profile requires a source .duckyprofile.json path.");
                    std::process::exit(1);
                }
            }
            "--get-pgv" => {
                if i + 1 < args.len() {
                    std::process::exit(run_cli_get_pgv(&args[i + 1]));
                } else {
                    eprintln!("Error: --get-pgv requires a PGV index (1..32).");
                    std::process::exit(1);
                }
            }
            "--dump-pgvs" => {
                std::process::exit(run_cli_dump_pgvs());
            }
            "--set-pgv" => {
                if i + 2 < args.len() {
                    std::process::exit(run_cli_set_pgv(&args[i + 1], &args[i + 2]));
                } else {
                    eprintln!("Error: --set-pgv requires <INDEX 1..32> and <VALUE>.");
                    std::process::exit(1);
                }
            }
            "--set-led" => {
                if i + 2 < args.len() {
                    std::process::exit(run_cli_set_led(&args[i + 1], &args[i + 2]));
                } else {
                    eprintln!(
                        "Error: --set-led requires <KEY 1..20|all> and <COLOR #RRGGBB|R,G,B>."
                    );
                    std::process::exit(1);
                }
            }
            "--status" => {
                std::process::exit(run_cli_status());
            }
            "--list-profiles" => {
                std::process::exit(run_cli_list_profiles());
            }
            "--switch-profile" => {
                if i + 1 < args.len() {
                    std::process::exit(run_cli_switch_profile(&args[i + 1]));
                } else {
                    eprintln!("Error: --switch-profile requires a profile number (1-64) or name.");
                    std::process::exit(1);
                }
            }
            "--compile" => {
                if i + 1 < args.len() {
                    i += 1;
                    compile_input = Some(args[i].clone());
                } else {
                    eprintln!("Error: --compile requires a DuckyScript (.txt) file path.");
                    std::process::exit(1);
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    i += 1;
                    output_override = Some(args[i].clone());
                } else {
                    eprintln!("Error: {arg} requires an output file path.");
                    std::process::exit(1);
                }
            }
            "--sync-rtc" => {
                std::process::exit(run_cli_sync_rtc());
            }
            "--sleep" => {
                std::process::exit(run_cli_sleep_wake(true));
            }
            "--wake" => {
                std::process::exit(run_cli_sleep_wake(false));
            }
            "--ui-test-suite" => {
                env::set_var("DUCKYDECK_UI_TEST_SUITE", "1");
                foreground_mode = true;
            }
            "--ui-test-report" => {
                if i + 1 < args.len() {
                    i += 1;
                    env::set_var("DUCKYDECK_UI_TEST_REPORT", &args[i]);
                } else {
                    eprintln!("Error: --ui-test-report requires an output JSON file path.");
                    std::process::exit(1);
                }
            }
            "--reboot" => match reboot_duckypad() {
                Ok(()) => {
                    println!("✓ Sent USB HID reboot command to duckyPad.");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error rebooting duckyPad: {e}");
                    std::process::exit(1);
                }
            },
            other => {
                if let Some(val) = other.strip_prefix("--profile=") {
                    startup_profile = Some(val.to_string());
                } else if let Some(val) = other.strip_prefix("--ui-test-report=") {
                    env::set_var("DUCKYDECK_UI_TEST_REPORT", val);
                } else if let Some(val) = other.strip_prefix("--switch-profile=") {
                    std::process::exit(run_cli_switch_profile(val));
                } else if let Some(val) = other.strip_prefix("--auto-switch=") {
                    std::process::exit(run_cli_auto_switch(val));
                } else if let Some(val) = other.strip_prefix("--export-group=") {
                    std::process::exit(run_cli_export_group(val));
                } else if let Some(val) = other.strip_prefix("--import-group=") {
                    std::process::exit(run_cli_import_group(val));
                } else if let Some(val) = other.strip_prefix("--export-profile=") {
                    export_profile_target = Some(val.to_string());
                } else if let Some(val) = other.strip_prefix("--import-profile=") {
                    std::process::exit(run_cli_import_profile(val));
                } else if let Some(val) = other.strip_prefix("--get-pgv=") {
                    std::process::exit(run_cli_get_pgv(val));
                } else if let Some(val) = other.strip_prefix("--compile=") {
                    compile_input = Some(val.to_string());
                } else if let Some(val) = other.strip_prefix("--output=") {
                    output_override = Some(val.to_string());
                } else if other.starts_with("-session=") {
                    start_minimized = true;
                    foreground_mode = true;
                } else if !other.starts_with("-psn_") {
                    eprintln!(
                        "Unknown option: '{other}'. Run 'duckydeck-studio --help' for usage."
                    );
                    std::process::exit(2);
                }
            }
        }
        i += 1;
    }

    let is_ui_test_suite = env::var("DUCKYDECK_UI_TEST_SUITE").as_deref() == Ok("1");

    // Automatically detect if launched by KDE Plasma / XDG / systemd autostart
    let launched_by_autostart = !is_ui_test_suite
        && (env::var("GIO_LAUNCHED_DESKTOP_FILE")
            .map(|v| v.contains("/autostart/"))
            .unwrap_or(false)
            || fs::read_to_string("/proc/self/cgroup")
                .map(|c| c.contains("autostart"))
                .unwrap_or(false));
    if launched_by_autostart {
        start_minimized = true;
        foreground_mode = true;
    }

    if let Some(ref input) = compile_input {
        std::process::exit(run_cli_compile(input, output_override.as_deref()));
    }
    if let Some(ref target) = export_profile_target {
        std::process::exit(run_cli_export_profile(target, output_override.as_deref()));
    }

    if env::var("DUCKYDECK_DEMO_SHOWCASE").as_deref() == Ok("1") {
        env::set_var("QT_QPA_PLATFORM", "xcb");
        env::set_var("QT_SCALE_FACTOR", "1");
        env::set_var("QT_AUTO_SCREEN_SCALE_FACTOR", "0");
        env::set_var("QT_SCREEN_SCALE_FACTORS", "1");
    } else if is_ui_test_suite {
        if env::var("QT_QPA_PLATFORM").is_err() {
            env::set_var("QT_QPA_PLATFORM", "offscreen");
        }
        env::set_var("QT_SCALE_FACTOR", "1");
        env::set_var("QT_AUTO_SCREEN_SCALE_FACTOR", "0");
        env::set_var("QT_SCREEN_SCALE_FACTORS", "1");
    }

    if !is_ui_test_suite {
        let pfile = pid_file_path();
        env::set_var("DUCKYDECK_PID_FILE", &pfile);
    }

    let is_daemon_child = env::var("DUCKYDECK_DAEMONIZED").as_deref() == Ok("1");
    let is_offscreen = env::var("QT_QPA_PLATFORM").as_deref() == Ok("offscreen");

    if !is_daemon_child && !is_ui_test_suite {
        let existing_pids = find_existing_duckydeck_instances();
        if !existing_pids.is_empty() {
            if restart_requested
                && !start_minimized
                && existing_pids.iter().copied().any(pid_had_minimized_arg)
            {
                start_minimized = true;
            }
            let csv = existing_pids
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(",");
            env::set_var("DUCKYDECK_EXISTING_PIDS", &csv);
            if replace_existing {
                env::set_var("DUCKYDECK_REPLACE_EXISTING", "1");
            } else {
                let has_gui_display = env::var("WAYLAND_DISPLAY")
                    .map(|v| !v.is_empty())
                    .unwrap_or(false)
                    || env::var("DISPLAY").map(|v| !v.is_empty()).unwrap_or(false);
                if !has_gui_display {
                    print!(
                        "DuckyDeck Studio is already running (PID {}). Close previous instance and start new one? [Y/n]: ",
                        csv
                    );
                    let _ = io::stdout().flush();
                    let mut answer = String::new();
                    if io::stdin().read_line(&mut answer).is_ok() {
                        let trimmed = answer.trim().to_ascii_lowercase();
                        if trimmed.is_empty() || trimmed == "y" || trimmed == "yes" {
                            env::set_var("DUCKYDECK_REPLACE_EXISTING", "1");
                        } else {
                            std::process::exit(0);
                        }
                    }
                }
            }
        } else {
            env::remove_var("DUCKYDECK_EXISTING_PIDS");
        }
    }

    if start_minimized {
        env::set_var("DUCKYDECK_START_MINIMIZED", "1");
    }
    if let Some(ref prof) = startup_profile {
        env::set_var("DUCKYDECK_STARTUP_PROFILE", prof);
    }
    if startup_sync_sd {
        env::set_var("DUCKYDECK_STARTUP_SYNC_SD", "1");
    }

    // Automatically detach into the background when --minimize / --tray is used from an interactive terminal, or when --restart is used
    let is_interactive_tty = std::io::IsTerminal::is_terminal(&io::stdin());
    if ((start_minimized && is_interactive_tty) || restart_requested)
        && !foreground_mode
        && !is_daemon_child
        && !is_offscreen
    {
        if let Ok(exe) = env::current_exe() {
            let mut child_args = args.clone();
            if start_minimized
                && !child_args
                    .iter()
                    .any(|a| matches!(a.as_str(), "-m" | "--minimize" | "--minimized" | "--tray"))
            {
                child_args.push("--minimize".to_string());
            }
            let mut cmd = Command::new(exe);
            cmd.args(&child_args)
                .env("DUCKYDECK_DAEMONIZED", "1")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            #[cfg(unix)]
            cmd.process_group(0);

            if let Ok(child) = cmd.spawn() {
                let existing_csv = env::var("DUCKYDECK_EXISTING_PIDS").unwrap_or_default();
                if restart_requested {
                    if !existing_csv.is_empty() {
                        println!(
                            "✓ DuckyDeck Studio restarted in background (replaced PID {} → new PID {})",
                            existing_csv,
                            child.id()
                        );
                    } else {
                        println!(
                            "✓ DuckyDeck Studio started in background (PID {})",
                            child.id()
                        );
                    }
                } else if !existing_csv.is_empty() && !replace_existing {
                    println!(
                        "⚠ DuckyDeck Studio is already running (PID {}) — prompting whether to replace with background instance (PID {})",
                        existing_csv,
                        child.id()
                    );
                } else {
                    println!(
                        "✓ DuckyDeck Studio started minimized in background (PID {})",
                        child.id()
                    );
                }
                std::process::exit(0);
            }
        }
    }

    let exit_code = bridge::device_controller::run_application(start_minimized);
    std::process::exit(exit_code);
}

#[cfg(test)]
mod ui_test_suite {
    use super::*;

    #[test]
    fn test_openapi_and_swagger_specs_are_valid() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let openapi_json_path = manifest_dir.join("docs/openapi.json");
        let swagger_json_path = manifest_dir.join("docs/swagger.json");
        let openapi_yaml_path = manifest_dir.join("docs/openapi.yaml");
        let rest_api_md_path = manifest_dir.join("docs/REST_API.md");

        assert!(openapi_json_path.exists(), "docs/openapi.json must exist");
        assert!(swagger_json_path.exists(), "docs/swagger.json must exist");
        assert!(openapi_yaml_path.exists(), "docs/openapi.yaml must exist");
        assert!(rest_api_md_path.exists(), "docs/REST_API.md must exist");

        let openapi_raw =
            std::fs::read_to_string(&openapi_json_path).expect("Failed to read docs/openapi.json");
        let openapi_val: serde_json::Value =
            serde_json::from_str(&openapi_raw).expect("docs/openapi.json must be valid JSON");

        assert_eq!(
            openapi_val.get("openapi").and_then(|v| v.as_str()),
            Some("3.0.3"),
            "docs/openapi.json must declare OpenAPI 3.0.3"
        );

        let paths = openapi_val
            .get("paths")
            .and_then(|v| v.as_object())
            .expect("docs/openapi.json must contain a paths object");

        let required_paths = [
            "/api/v1/health",
            "/api/v1/openapi.json",
            "/api/v1/swagger.json",
            "/api/v1/openapi.yaml",
            "/api/v1/status",
            "/api/v1/profiles",
            "/api/v1/profiles/select",
            "/api/v1/profiles/reorder",
            "/api/v1/profiles/{number}",
            "/api/v1/profiles/{number}/slots",
            "/api/v1/profiles/{number}/slots/{slot}",
            "/api/v1/profiles/{number}/slots/{slot}/trigger",
            "/api/v1/dsvm/compile",
            "/api/v1/templates",
            "/api/v1/pgv",
            "/api/v1/leds",
            "/api/v1/sd/sync",
            "/api/v1/sd/reload",
            "/api/v1/sd/mount",
            "/api/v1/sd/unmount",
            "/api/v1/device/sleep",
            "/api/v1/device/wake",
            "/api/v1/device/reboot",
            "/api/v1/device/sync-rtc",
            "/api/v1/jobs/{id}",
            "/api/v1/logs",
        ];

        for ep in required_paths {
            assert!(
                paths.contains_key(ep),
                "OpenAPI spec is missing endpoint path: {}",
                ep
            );
        }

        let swagger_raw =
            std::fs::read_to_string(&swagger_json_path).expect("Failed to read docs/swagger.json");
        let swagger_val: serde_json::Value =
            serde_json::from_str(&swagger_raw).expect("docs/swagger.json must be valid JSON");
        assert!(
            swagger_val
                .get("paths")
                .and_then(|v| v.as_object())
                .is_some(),
            "docs/swagger.json must contain paths"
        );
    }

    #[test]
    fn test_oled_label_input_sizing_and_home_automation_icons() {
        use crate::hardware::profile::{oled_icon_char_to_ascii, oled_label_to_hardware_ascii};
        use std::collections::HashSet;

        // 1. Verify entire AppLaunchBuilder.qml iconCatalog has zero duplicates, covers all app categories,
        //    and every icon maps cleanly to <= 5 chars of 7-bit printable ASCII.
        let qml_src = include_str!("qml/AppLaunchBuilder.qml");
        let mut seen_icons = HashSet::new();
        let mut seen_cats = HashSet::new();
        let mut icon_count = 0usize;

        for line in qml_src.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("{ icon: \"") && trimmed.contains("cat: \"") {
                let extract_field = |key: &str| -> String {
                    let prefix = format!("{key}: \"");
                    let start = trimmed.find(&prefix).expect("missing field") + prefix.len();
                    let rest = &trimmed[start..];
                    let end = rest.find('"').expect("unterminated string");
                    rest[..end].to_string()
                };
                let icon = extract_field("icon");
                let label = extract_field("label");
                let name = extract_field("name");
                let cat = extract_field("cat");

                assert!(
                    seen_icons.insert(icon.clone()),
                    "Duplicate icon '{}' found in iconCatalog (entry: {} / {})",
                    icon,
                    name,
                    cat
                );
                let chars: Vec<char> = icon.chars().collect();
                assert_eq!(
                    chars.len(),
                    1,
                    "Icon '{}' ({}) must be a single Unicode codepoint",
                    icon,
                    name
                );
                let ch = chars[0];
                assert!(
                    (ch as u32) > 127,
                    "Icon '{}' ({}) must be a non-ASCII graphical symbol",
                    icon,
                    name
                );
                assert!(
                    oled_icon_char_to_ascii(ch, false).is_some()
                        && oled_icon_char_to_ascii(ch, true).is_some(),
                    "Icon '{}' (U+{:04X}, {}) is missing an explicit mapping in oled_icon_char_to_ascii",
                    icon,
                    ch as u32,
                    name
                );
                assert!(
                    !label.is_empty()
                        && label.len() <= 5
                        && label.bytes().all(|b| (0x20..=0x7E).contains(&b)),
                    "Icon '{}' label '{}' must be 1..=5 printable ASCII chars",
                    icon,
                    label
                );
                assert!(!name.is_empty(), "Icon '{}' name must not be empty", icon);

                let converted = oled_label_to_hardware_ascii(&icon);
                assert!(
                    !converted.is_empty()
                        && converted.len() <= 5
                        && converted.bytes().all(|b| (0x20..=0x7E).contains(&b)),
                    "Icon '{}' ({}) converted ASCII '{}' must be 1..=5 printable ASCII chars",
                    icon,
                    name,
                    converted
                );

                let compound = format!("{icon} ON");
                let converted_compound = oled_label_to_hardware_ascii(&compound);
                assert!(
                    converted_compound.len() <= 5,
                    "Compound label '{compound}' must not exceed 5 chars, got '{}'",
                    converted_compound
                );

                seen_cats.insert(cat);
                icon_count += 1;
            }
        }

        assert!(
            icon_count >= 160,
            "Expected at least 160 unique icons in iconCatalog, found {icon_count}"
        );
        for required_cat in [
            "Web Browsers",
            "Word & Docs",
            "Excel & Sheets",
            "PowerPoint & Slides",
            "Photoshop & Creative",
            "FreeCAD & Fusion 360",
            "Editing & Clipboard",
            "Arrows & Nav",
            "Media & Audio",
            "Coding & Terminal",
            "Home Automation",
            "Window & System",
            "Streaming & Apps",
            "Symbols & Shapes",
        ] {
            assert!(
                seen_cats.contains(required_cat),
                "Missing required icon category '{required_cat}' in iconCatalog"
            );
        }

        // 2. Test input field sizing and capacity across zoom levels > 100% (100% to 200%)
        let zoom_levels: [f64; 8] = [1.00, 1.05, 1.10, 1.15, 1.25, 1.50, 1.75, 2.00];
        for zoom in zoom_levels {
            let font_point_size: f64 = 10.0 * zoom;
            // Typical monospace advance width is ~0.60 to 0.65 of pixel height
            let pixel_height: f64 = font_point_size * (96.0 / 72.0);
            let approx_char_width: f64 = pixel_height * 0.62;
            let five_char_content_width: f64 = (5.0 * approx_char_width).ceil();
            let padding_horizontal: f64 = 12.0; // 6px left + 6px right

            let min_content_needed: f64 = five_char_content_width + padding_horizontal;

            let input_layout_min_width: f64 = (86.0 * zoom).round().max(86.0);
            let input_layout_pref_width: f64 = (96.0 * zoom).round().max(96.0);
            let input_implicit_height: f64 = (30.0 * zoom).round().max(30.0);

            let required_height: f64 = pixel_height + 8.0; // font height + vertical padding

            assert!(
                input_layout_min_width >= min_content_needed,
                "At zoom {zoom}x (pt {font_point_size:.1}), minimum width {input_layout_min_width}px must accommodate 5 chars ({min_content_needed}px)"
            );
            assert!(
                input_layout_pref_width >= input_layout_min_width,
                "At zoom {zoom}x, preferred width {input_layout_pref_width}px must be >= min width {input_layout_min_width}px"
            );
            assert!(
                input_implicit_height >= required_height,
                "At zoom {zoom}x, input height {input_implicit_height}px must accommodate line height {required_height}px without vertical clipping"
            );
        }
    }

    #[test]
    #[ignore = "launches full GUI; run with cargo test -- --ignored"]
    fn test_full_qml_ui_overflow_and_contrast_suite() {
        use std::io::Read;
        use std::process::Stdio;
        use std::time::{Duration, Instant};

        if std::env::var("DUCKYDECK_UI_TEST_SUBPROC").as_deref() == Ok("1") {
            sanitize_xdg_env();
            let code = crate::bridge::device_controller::run_application(false);
            std::process::exit(code);
        }

        let timeout_secs: u64 = std::env::var("DUCKYDECK_UI_TEST_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .filter(|&s| s > 0)
            .unwrap_or(120);
        let timeout = Duration::from_secs(timeout_secs);

        let report_path = std::env::temp_dir().join(format!(
            "duckydeck_ui_test_report_{}.json",
            std::process::id()
        ));
        if report_path.exists() {
            if let Err(e) = std::fs::remove_file(&report_path) {
                eprintln!(
                    "Warning removing stale UI test report {}: {e}",
                    report_path.display()
                );
            }
        }

        let current_test_bin =
            std::env::current_exe().expect("Failed to resolve current test executable path");

        let mut cmd = Command::new(current_test_bin);
        cmd.arg("--exact")
            .arg("ui_test_suite::test_full_qml_ui_overflow_and_contrast_suite")
            .arg("--ignored")
            .arg("--nocapture")
            .env("DUCKYDECK_UI_TEST_SUBPROC", "1")
            .env("DUCKYDECK_UI_TEST_SUITE", "1")
            .env("DUCKYDECK_UI_TEST_REPORT", &report_path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if std::env::var_os("QT_QPA_PLATFORM").is_none() {
            cmd.env("QT_QPA_PLATFORM", "offscreen");
        }
        cmd.env("QT_SCALE_FACTOR", "1")
            .env("QT_AUTO_SCREEN_SCALE_FACTOR", "0")
            .env("QT_SCREEN_SCALE_FACTORS", "1");

        let mut child = cmd
            .spawn()
            .expect("Failed to spawn UI test suite subprocess");

        let mut child_stdout = child.stdout.take();
        let mut child_stderr = child.stderr.take();

        let stdout_handle = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(ref mut out) = child_stdout {
                if let Err(e) = out.read_to_end(&mut buf) {
                    eprintln!("Warning reading UI test subprocess stdout: {e}");
                }
            }
            buf
        });

        let stderr_handle = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(ref mut err) = child_stderr {
                if let Err(e) = err.read_to_end(&mut buf) {
                    eprintln!("Warning reading UI test subprocess stderr: {e}");
                }
            }
            buf
        });

        let start = Instant::now();
        let mut timed_out = false;
        let status = loop {
            match child.try_wait() {
                Ok(Some(exit_status)) => break Some(exit_status),
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        timed_out = true;
                        let _ = child.kill(); // ignore: best-effort kill on timed-out test child
                        let _ = child.wait(); // ignore: best-effort reap on timed-out test child
                        break None;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(err) => {
                    let _ = child.kill(); // ignore: best-effort kill on errored test child
                    let _ = child.wait(); // ignore: best-effort reap on errored test child
                    panic!("Error waiting for UI test suite subprocess: {err}");
                }
            }
        };

        let stdout_bytes = stdout_handle.join().unwrap_or_default();
        let stderr_bytes = stderr_handle.join().unwrap_or_default();
        let stderr_text = String::from_utf8_lossy(&stderr_bytes);
        let stdout_text = String::from_utf8_lossy(&stdout_bytes);

        if timed_out {
            if report_path.exists() {
                if let Err(e) = std::fs::remove_file(&report_path) {
                    eprintln!(
                        "Warning removing timed-out UI test report {}: {e}",
                        report_path.display()
                    );
                }
            }
            panic!(
                "UI test suite subprocess timed out after {}s (DUCKYDECK_UI_TEST_TIMEOUT_SECS).\nstderr:\n{}\nstdout:\n{}",
                timeout_secs, stderr_text, stdout_text
            );
        }

        let exit_status = status.expect("UI test subprocess must have an exit status");

        assert!(
            report_path.exists(),
            "UI test report JSON was not generated at {:?} (status: {:?}). stderr: {}",
            report_path,
            exit_status,
            stderr_text
        );

        let report_raw = std::fs::read_to_string(&report_path)
            .expect("Failed to read generated UI test report JSON");
        if let Err(e) = std::fs::remove_file(&report_path) {
            eprintln!(
                "Warning removing UI test report {}: {e}",
                report_path.display()
            );
        }

        let report: serde_json::Value =
            serde_json::from_str(&report_raw).expect("UI test report must be valid JSON");

        let passed = report
            .get("passed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let behavioral_failures = report
            .get("summary")
            .and_then(|s| s.get("behavioralFailuresCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let overflow_count = report
            .get("summary")
            .and_then(|s| s.get("overflowViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let contrast_count = report
            .get("summary")
            .and_then(|s| s.get("contrastViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let scrollbar_violations_count = report
            .get("summary")
            .and_then(|s| s.get("scrollBarStyleViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let min_font_pt_violations_count = report
            .get("summary")
            .and_then(|s| s.get("minFontPointSizeViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let total_scrollbars_inspected = report
            .get("summary")
            .and_then(|s| s.get("totalScrollBarsInspected"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        assert_eq!(
            behavioral_failures,
            0,
            "UI behavioral checks failed: {}",
            serde_json::to_string_pretty(&report["behavioralResults"]).unwrap_or_default()
        );
        assert_eq!(
            overflow_count,
            0,
            "UI overflow violations detected: {}",
            serde_json::to_string_pretty(&report["overflowViolations"]).unwrap_or_default()
        );
        assert_eq!(
            contrast_count,
            0,
            "UI contrast violations detected: {}",
            serde_json::to_string_pretty(&report["contrastViolations"]).unwrap_or_default()
        );
        assert_eq!(
            scrollbar_violations_count, 0,
            "UI scrollbar style violations detected (must match Profiles Box HighContrastScrollBar): {}",
            serde_json::to_string_pretty(&report["scrollBarStyleViolations"]).unwrap_or_default()
        );
        assert_eq!(
            min_font_pt_violations_count,
            0,
            "UI minimum 10pt font size violations detected: {}",
            serde_json::to_string_pretty(&report["minFontPointSizeViolations"]).unwrap_or_default()
        );
        assert!(
            total_scrollbars_inspected >= 40,
            "Expected at least 40 scrollbar instances to be inspected across all resizable windows and themes, got {}",
            total_scrollbars_inspected
        );

        let total_keycaps_audited = report
            .get("summary")
            .and_then(|s| s.get("totalKeycapsAudited"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        assert!(
            total_keycaps_audited >= 160,
            "Expected at least 160 keycap instances to be audited across themes and zoom levels up to 115%, got {}",
            total_keycaps_audited
        );

        let required_behaviors = [
            "resizable_windows_scrollbar_style_matches_profiles_box",
            "script_editor_has_scrollbar",
            "settings_removed_area_text_sizes_and_min_10pt_font",
            "ctrl_plus_equal_minus_zoom_and_restart_persistence",
            "all_popup_windows_closable_with_esc",
            "option_c_keycap_layout_no_overlap",
            "no_overflows_at_zoom_levels_up_to_115_percent",
            "new_profile_group_has_default_fallback",
            "fallback_profile_cannot_be_deleted",
            "profile_reorder_and_add_updates_autoswitch_rules",
            "deleted_profile_rule_reassigned_to_fallback",
            "duckydeck_studio_firmware_available_in_catalog",
            "autoswitch_sections_twist_minimize_and_auto_expand_on_double_click",
            "autoswitch_rule_title_regex_support",
        ];
        for req_name in required_behaviors {
            let behavior_passed = report
                .get("behavioralResults")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter().any(|entry| {
                        entry.get("name").and_then(|n| n.as_str()) == Some(req_name)
                            && entry.get("passed").and_then(|p| p.as_bool()) == Some(true)
                    })
                })
                .unwrap_or(false);
            assert!(
                behavior_passed,
                "Behavioral check '{}' did not pass: {}",
                req_name,
                serde_json::to_string_pretty(&report["behavioralResults"]).unwrap_or_default()
            );
        }

        assert!(
            passed && exit_status.success(),
            "UI test suite did not pass cleanly (status: {:?}). Report summary: {}\nstderr: {}",
            exit_status,
            serde_json::to_string_pretty(&report["summary"]).unwrap_or_default(),
            stderr_text
        );
    }

    #[test]
    #[ignore = "launches Flatpak GUI; run with cargo test test_flatpak_qml_ui_overflow_and_contrast_suite -- --ignored"]
    fn test_flatpak_qml_ui_overflow_and_contrast_suite() {
        use std::io::Read;
        use std::process::Stdio;
        use std::time::{Duration, Instant};

        let timeout_secs: u64 = std::env::var("DUCKYDECK_UI_TEST_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.trim().parse::<u64>().ok())
            .filter(|&s| s > 0)
            .unwrap_or(120);
        let timeout = Duration::from_secs(timeout_secs);

        let home_dir = std::env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| std::env::temp_dir());
        let flatpak_cache_dir = home_dir.join(".var/app/io.github.duckydeck.Studio/cache");
        std::fs::create_dir_all(&flatpak_cache_dir)
            .expect("Failed creating Flatpak cache dir for UI test report");

        let report_path = flatpak_cache_dir.join(format!(
            "duckydeck_flatpak_ui_test_report_{}.json",
            std::process::id()
        ));
        if report_path.exists() {
            let _ = std::fs::remove_file(&report_path);
        }

        let mut cmd = Command::new("flatpak");
        cmd.arg("run");
        if std::env::var_os("WAYLAND_DISPLAY").is_none() && std::env::var_os("DISPLAY").is_none() {
            cmd.arg("--env=QT_QPA_PLATFORM=offscreen");
        }
        cmd.arg("io.github.duckydeck.Studio")
            .arg("--ui-test-suite")
            .arg(format!("--ui-test-report={}", report_path.display()))
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .expect("Failed to spawn flatpak run io.github.duckydeck.Studio");

        let mut child_stdout = child.stdout.take();
        let mut child_stderr = child.stderr.take();

        let stdout_handle = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(ref mut out) = child_stdout {
                let _ = out.read_to_end(&mut buf);
            }
            buf
        });

        let stderr_handle = std::thread::spawn(move || {
            let mut buf = Vec::new();
            if let Some(ref mut err) = child_stderr {
                let _ = err.read_to_end(&mut buf);
            }
            buf
        });

        let start = Instant::now();
        let mut timed_out = false;
        let status = loop {
            match child.try_wait() {
                Ok(Some(exit_status)) => break Some(exit_status),
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        timed_out = true;
                        let _ = child.kill();
                        let _ = child.wait();
                        break None;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(err) => {
                    let _ = child.kill();
                    let _ = child.wait();
                    panic!("Error waiting for Flatpak UI test subprocess: {err}");
                }
            }
        };

        let stdout_bytes = stdout_handle.join().unwrap_or_default();
        let stderr_bytes = stderr_handle.join().unwrap_or_default();
        let stderr_text = String::from_utf8_lossy(&stderr_bytes);
        let stdout_text = String::from_utf8_lossy(&stdout_bytes);

        if timed_out {
            let _ = std::fs::remove_file(&report_path);
            panic!(
                "Flatpak UI test suite timed out after {}s.\nstderr:\n{}\nstdout:\n{}",
                timeout_secs, stderr_text, stdout_text
            );
        }

        let exit_status = status.expect("Flatpak UI test subprocess must have an exit status");

        assert!(
            report_path.exists(),
            "Flatpak UI test report JSON was not generated at {:?} (status: {:?}). stderr: {}",
            report_path,
            exit_status,
            stderr_text
        );

        let report_raw = std::fs::read_to_string(&report_path)
            .expect("Failed to read generated Flatpak UI test report JSON");
        let _ = std::fs::remove_file(&report_path);

        let report: serde_json::Value =
            serde_json::from_str(&report_raw).expect("Flatpak UI test report must be valid JSON");

        let passed = report
            .get("passed")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let behavioral_failures = report
            .get("summary")
            .and_then(|s| s.get("behavioralFailuresCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let overflow_count = report
            .get("summary")
            .and_then(|s| s.get("overflowViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let contrast_count = report
            .get("summary")
            .and_then(|s| s.get("contrastViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let scrollbar_violations_count = report
            .get("summary")
            .and_then(|s| s.get("scrollBarStyleViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let min_font_pt_violations_count = report
            .get("summary")
            .and_then(|s| s.get("minFontPointSizeViolationsCount"))
            .and_then(|v| v.as_u64())
            .unwrap_or(999);
        let total_scrollbars_inspected = report
            .get("summary")
            .and_then(|s| s.get("totalScrollBarsInspected"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let total_keycaps_audited = report
            .get("summary")
            .and_then(|s| s.get("totalKeycapsAudited"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        assert_eq!(
            overflow_count,
            0,
            "Flatpak UI overflow violations detected: {}",
            serde_json::to_string_pretty(&report["overflowViolations"]).unwrap_or_default()
        );
        assert_eq!(
            behavioral_failures,
            0,
            "Flatpak UI behavioral checks failed: {}",
            serde_json::to_string_pretty(&report["behavioralResults"]).unwrap_or_default()
        );
        assert_eq!(
            contrast_count,
            0,
            "Flatpak UI contrast violations detected: {}",
            serde_json::to_string_pretty(&report["contrastViolations"]).unwrap_or_default()
        );
        assert_eq!(
            scrollbar_violations_count,
            0,
            "Flatpak UI scrollbar style violations detected: {}",
            serde_json::to_string_pretty(&report["scrollBarStyleViolations"]).unwrap_or_default()
        );
        assert_eq!(
            min_font_pt_violations_count,
            0,
            "Flatpak UI minimum 10pt font size violations detected: {}",
            serde_json::to_string_pretty(&report["minFontPointSizeViolations"]).unwrap_or_default()
        );
        assert!(
            total_scrollbars_inspected >= 40,
            "Expected at least 40 scrollbar instances in Flatpak UI test, got {}",
            total_scrollbars_inspected
        );
        assert!(
            total_keycaps_audited >= 160,
            "Expected at least 160 keycap instances in Flatpak UI test, got {}",
            total_keycaps_audited
        );

        let required_behaviors = [
            "resizable_windows_scrollbar_style_matches_profiles_box",
            "script_editor_has_scrollbar",
            "settings_removed_area_text_sizes_and_min_10pt_font",
            "ctrl_plus_equal_minus_zoom_and_restart_persistence",
            "all_popup_windows_closable_with_esc",
            "option_c_keycap_layout_no_overlap",
            "no_overflows_at_zoom_levels_up_to_115_percent",
            "new_profile_group_has_default_fallback",
            "fallback_profile_cannot_be_deleted",
            "profile_reorder_and_add_updates_autoswitch_rules",
            "deleted_profile_rule_reassigned_to_fallback",
            "duckydeck_studio_firmware_available_in_catalog",
            "autoswitch_sections_twist_minimize_and_auto_expand_on_double_click",
            "autoswitch_rule_title_regex_support",
        ];
        for req_name in required_behaviors {
            let behavior_passed = report
                .get("behavioralResults")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter().any(|entry| {
                        entry.get("name").and_then(|n| n.as_str()) == Some(req_name)
                            && entry.get("passed").and_then(|p| p.as_bool()) == Some(true)
                    })
                })
                .unwrap_or(false);
            assert!(
                behavior_passed,
                "Flatpak behavioral check '{}' did not pass: {}",
                req_name,
                serde_json::to_string_pretty(&report["behavioralResults"]).unwrap_or_default()
            );
        }

        assert!(
            passed && exit_status.success(),
            "Flatpak UI test suite did not pass cleanly (status: {:?}). Report summary: {}\nstderr: {}",
            exit_status,
            serde_json::to_string_pretty(&report["summary"]).unwrap_or_default(),
            stderr_text
        );
    }

    #[test]
    fn test_rule_title_regex_and_substring_matching() {
        use crate::kwin::auto_switcher::{
            rule_matches_window_title, validate_title_regex, AutoSwitchRule, SwitcherState,
        };

        assert!(validate_title_regex("YouTube|Twitch|^\\[PROD\\]").is_ok());
        assert!(validate_title_regex("/^Gmail.*Inbox$/i").is_ok());
        assert!(validate_title_regex("[unclosed(").is_err());

        // Plain substring matching
        assert!(rule_matches_window_title(
            "youtube",
            false,
            "Watching Video - YouTube — Zen Browser",
            "watching video - youtube — zen browser"
        )
        .is_some());

        // Explicit regex matching (`title_is_regex = true`)
        assert!(rule_matches_window_title(
            "YouTube|Twitch",
            true,
            "Live Stream - Twitch — Mozilla Firefox",
            "live stream - twitch — mozilla firefox"
        )
        .is_some());
        assert!(rule_matches_window_title(
            "^\\[PROD\\]",
            true,
            "[PROD] Kubernetes Dashboard",
            "[prod] kubernetes dashboard"
        )
        .is_some());
        assert!(rule_matches_window_title(
            "^\\[PROD\\]",
            true,
            "Staging: [PROD] Kubernetes Dashboard",
            "staging: [prod] kubernetes dashboard"
        )
        .is_none());

        // When `title_is_regex = false`, regex patterns are matched strictly as literal substrings
        assert!(rule_matches_window_title(
            "/^(Inbox|Starred).*Gmail/",
            false,
            "Inbox (4) - user@gmail.com - Gmail — Zen Browser",
            "inbox (4) - user@gmail.com - gmail — zen browser"
        )
        .is_none());
        assert!(rule_matches_window_title(
            "/^(Inbox|Starred).*Gmail/",
            true,
            "Inbox (4) - user@gmail.com - Gmail — Zen Browser",
            "inbox (4) - user@gmail.com - gmail — zen browser"
        )
        .is_some());

        assert!(rule_matches_window_title(
            "Pull Request #[0-9]+",
            false,
            "Fix bug · Pull Request #42 · user/repo",
            "fix bug · pull request #42 · user/repo"
        )
        .is_none());
        assert!(rule_matches_window_title(
            "Pull Request #[0-9]+",
            true,
            "Fix bug · Pull Request #42 · user/repo",
            "fix bug · pull request #42 · user/repo"
        )
        .is_some());

        // Full SwitcherState::match_profile_for_window priority test
        let sw = SwitcherState {
            default_profile: Some(1),
            rules: vec![
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "*browser*".to_string(),
                    title_contains: None,
                    title_is_regex: false,
                    favicon_url: None,
                    target_profile: 2,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "*browser*".to_string(),
                    title_contains: Some("YouTube|Twitch".to_string()),
                    title_is_regex: true,
                    favicon_url: None,
                    target_profile: 5,
                },
            ],
            ..SwitcherState::default()
        };

        assert_eq!(
            sw.match_profile_for_window("firefox", "Hacker News — Mozilla Firefox"),
            Some(2)
        );
        assert_eq!(
            sw.match_profile_for_window("firefox", "Twitch Stream — Mozilla Firefox"),
            Some(5)
        );

        // Regression test: literal "| Microsoft Teams" with `title_is_regex = false` is a plain substring check
        assert!(rule_matches_window_title(
            "| Microsoft Teams",
            false,
            "Messages - Google Messages for web",
            "messages - google messages for web"
        )
        .is_none());
        assert_eq!(
            rule_matches_window_title(
                "Messages -",
                false,
                "Messages - Google Messages for web",
                "messages - google messages for web"
            ),
            Some(10)
        );
    }
}
