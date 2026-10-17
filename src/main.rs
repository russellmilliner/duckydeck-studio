pub mod bridge {
    pub mod device_controller;
}
pub mod hardware {
    pub mod permissions;
    pub mod storage;
}
pub mod kwin {
    pub mod auto_switcher;
}
pub mod ai {
    pub mod shortcut_scraper;
}
pub mod vm;

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

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
    let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let default_data_home = format!("{home}/.local/share");
    let needs_fix = match env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let trimmed = val.trim();
            trimmed.is_empty()
                || trimmed.contains(':')
                || !trimmed.starts_with('/')
                || trimmed.starts_with("/var/")
                || trimmed.starts_with("/usr/")
        }
        Err(_) => true,
    };
    if needs_fix {
        env::set_var("XDG_DATA_HOME", &default_data_home);
    }

    let current_dirs = env::var("XDG_DATA_DIRS").unwrap_or_default();
    if !current_dirs.split(':').any(|p| p == "/usr/share") {
        let mut parts: Vec<String> = current_dirs
            .split(':')
            .filter(|s| !s.is_empty() && *s != ".")
            .map(|s| s.to_string())
            .collect();
        for std_dir in [
            format!("{home}/.local/share/flatpak/exports/share"),
            "/var/lib/flatpak/exports/share".to_string(),
            "/usr/local/share".to_string(),
            "/usr/share".to_string(),
        ] {
            if !parts.contains(&std_dir) {
                parts.push(std_dir);
            }
        }
        env::set_var("XDG_DATA_DIRS", parts.join(":"));
    }

    let quiet_rules = "union.*=false;qt.core.qobject.connect=false";
    match env::var("QT_LOGGING_RULES") {
        Ok(existing) if !existing.trim().is_empty() => {
            if !existing.contains("union.*=false") {
                env::set_var("QT_LOGGING_RULES", format!("{existing};{quiet_rules}"));
            }
        }
        _ => {
            env::set_var("QT_LOGGING_RULES", quiet_rules);
        }
    }
}

fn runtime_dir_path() -> PathBuf {
    if let Ok(runtime_dir) = env::var("XDG_RUNTIME_DIR") {
        let trimmed = runtime_dir.trim();
        if !trimmed.is_empty() && Path::new(trimmed).is_dir() {
            return PathBuf::from(trimmed);
        }
    }
    PathBuf::from("/tmp")
}

fn pid_file_path() -> PathBuf {
    runtime_dir_path().join("duckydeck-studio.pid")
}

fn ipc_file_path() -> PathBuf {
    runtime_dir_path().join("duckydeck-studio.ipc")
}

fn is_running_duckydeck_pid(pid: u32, self_pid: u32) -> bool {
    if pid <= 1 || pid == self_pid {
        return false;
    }
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

fn send_ipc_command_to_running_instance(cmd: &str) -> Result<Vec<u32>, String> {
    let pids = find_existing_duckydeck_instances();
    if pids.is_empty() {
        return Err("No running DuckyDeck Studio instance found.".to_string());
    }
    let ipc_path = ipc_file_path();
    fs::write(&ipc_path, format!("{cmd}\n"))
        .map_err(|e| format!("Failed writing IPC command to {}: {e}", ipc_path.display()))?;

    for _ in 0..20 {
        if !ipc_path.exists() {
            break;
        }
        thread::sleep(Duration::from_millis(35));
    }
    Ok(pids)
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
            println!("{:<6} {:<24} {:<10} {:<10}", "SLOT", "NAME", "KEYS", "BG_COLOR");
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
            let _ = device.read_timeout(&mut rx, 200);
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
            result.error_line_number_starting_from_1,
            result.error_comment,
            result.error_line_str
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
            let _ = fs::create_dir_all(parent);
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

    if let Ok(pids) = send_ipc_command_to_running_instance(&format!(
        "import-group:{}",
        abs_src.display()
    )) {
        println!(
            "✓ Imported Profile Group '{}' into running DuckyDeck Studio instance (PID {})",
            abs_src.display(),
            pids.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
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
            eprintln!("Error parsing .duckygroup.json '{}': {e}", abs_src.display());
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
        let _ = write_profiles_to_sd_root(&sd_root, &bundle.profiles);
    }

    let mut sw = SwitcherState::default();
    sw.rules = bundle.auto_switch_rules;
    if let Some(fb) = bundle.default_fallback_profile {
        sw.default_profile = Some(fb.clamp(1, bundle.profiles.len() as u8));
    }
    let _ = sw.save_to_disk();

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
            let _ = fs::create_dir_all(parent);
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

    if let Ok(pids) = send_ipc_command_to_running_instance(&format!(
        "import-profile:{}",
        abs_src.display()
    )) {
        println!(
            "✓ Imported Profile '{}' into running DuckyDeck Studio instance (PID {})",
            abs_src.display(),
            pids.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
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
            eprintln!("Error: Invalid .duckyprofile.json format in '{}'", abs_src.display());
            return 1;
        };

    let (mut profiles, _) = load_current_profiles().unwrap_or_else(|_| (Vec::new(), profile_cache_dir()));
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
        let _ = write_profiles_to_sd_root(&sd_root, &profiles);
    }

    if !imported_rules.is_empty() {
        let mut sw = SwitcherState::default();
        for mut r in imported_rules {
            r.target_profile = next_num;
            sw.rules.push(r);
        }
        let _ = sw.save_to_disk();
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
            pids.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
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
                pids.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
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
    let _ = device.read_timeout(&mut rx, 200);
    println!("✓ Synchronized duckyPad RTC with host clock ({})", now.format("%Y-%m-%d %H:%M:%S %:z"));
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
        eprintln!("Error sending {} command: {e}", if sleep { "sleep" } else { "wake" });
        return 1;
    }
    let mut rx = [0u8; 64];
    let _ = device.read_timeout(&mut rx, 200);
    println!(
        "✓ Connected duckyPad {}",
        if sleep { "entered sleep mode" } else { "woke from sleep mode" }
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
                    eprintln!("Error: --export-group requires a destination .duckygroup.json path.");
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
                    eprintln!("Error: --set-led requires <KEY 1..20|all> and <COLOR #RRGGBB|R,G,B>.");
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
            "--reboot" => {
                match reboot_duckypad() {
                    Ok(()) => {
                        println!("✓ Sent USB HID reboot command to duckyPad.");
                        std::process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error rebooting duckyPad: {e}");
                        std::process::exit(1);
                    }
                }
            }
            other => {
                if let Some(val) = other.strip_prefix("--profile=") {
                    startup_profile = Some(val.to_string());
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
                    eprintln!("Unknown option: '{other}'. Run 'duckydeck-studio --help' for usage.");
                    std::process::exit(2);
                }
            }
        }
        i += 1;
    }

    // Automatically detect if launched by KDE Plasma / XDG / systemd autostart
    let launched_by_autostart = env::var("GIO_LAUNCHED_DESKTOP_FILE")
        .map(|v| v.contains("/autostart/"))
        .unwrap_or(false)
        || fs::read_to_string("/proc/self/cgroup")
            .map(|c| c.contains("autostart"))
            .unwrap_or(false);
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
    }

    let pfile = pid_file_path();
    env::set_var("DUCKYDECK_PID_FILE", &pfile);
    let ipcfile = ipc_file_path();
    env::set_var("DUCKYDECK_IPC_FILE", &ipcfile);

    let is_daemon_child = env::var("DUCKYDECK_DAEMONIZED").as_deref() == Ok("1");
    let is_offscreen = env::var("QT_QPA_PLATFORM").as_deref() == Ok("offscreen");

    if !is_daemon_child {
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
                    || env::var("DISPLAY")
                        .map(|v| !v.is_empty())
                        .unwrap_or(false);
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
            if start_minimized && !child_args.iter().any(|a| matches!(a.as_str(), "-m" | "--minimize" | "--minimized" | "--tray")) {
                child_args.push("--minimize".to_string());
            }
            let mut cmd = Command::new(exe);
            cmd.args(&child_args)
                .env("DUCKYDECK_DAEMONIZED", "1")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .process_group(0);

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
