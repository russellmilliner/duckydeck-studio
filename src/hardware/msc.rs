use std::fs;
use std::path::{Path, PathBuf};

use crate::hardware::profile::{
    compute_profiles_fingerprint, find_mounted_duckypad_sd, find_mounted_duckypad_sd_for_device,
    load_all_sd_profiles, mark_profiles_sd_synced, parse_profile_folder_name,
    profile_cache_dir_for_serial, record_mounted_sd_space, sync_profiles_to_device_cache,
    write_profiles_to_sd_root_for_model, DuckyProfile, DuckySdDeviceMeta, SdWriteSummary,
};
use crate::hardware::storage::{
    model_name_for_hw_model, open_duckypad_hid_for_serial, DP_MODEL_DUCKYPAD_PRO,
    DP_MODEL_OG_DUCKYPAD, HID_CMD_CLOSE_FILE, HID_CMD_CREATE_DIR, HID_CMD_DELETE_DIR,
    HID_CMD_DELETE_FILE, HID_CMD_DUMP_SD, HID_CMD_OPEN_FILE_FOR_READING,
    HID_CMD_OPEN_FILE_FOR_WRITING, HID_CMD_READ_FILE, HID_CMD_SOFTWARE_RESET, HID_CMD_WRITE_FILE,
    HID_IO_LOCK, HID_REPORT_ID_OUT, HID_STATUS_ERROR, MSC_SYNC_IN_PROGRESS,
};

pub fn run_storage_cmd(program: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    if Path::new("/.flatpak-info").exists() {
        let mut cmd = std::process::Command::new("flatpak-spawn");
        cmd.arg("--host").arg(program);
        for arg in args {
            cmd.arg(arg);
        }
        cmd.output()
    } else {
        std::process::Command::new(program).args(args).output()
    }
}

fn read_mounts_table() -> Option<String> {
    let mut table = fs::read_to_string("/proc/mounts").unwrap_or_default();
    if Path::new("/.flatpak-info").exists() {
        if let Ok(out) = run_storage_cmd("cat", &["/proc/mounts"]) {
            if out.status.success() {
                let host_mounts = String::from_utf8_lossy(&out.stdout);
                if !table.ends_with('\n') && !table.is_empty() {
                    table.push('\n');
                }
                table.push_str(&host_mounts);
            }
        }
    }
    if table.is_empty() {
        None
    } else {
        Some(table)
    }
}

fn parse_udisks_mount_path(stdout: &str) -> Option<PathBuf> {
    for line in stdout.lines() {
        if let Some((_, after_at)) = line.split_once(" at ") {
            let trimmed = after_at.trim();
            if trimmed.is_empty() {
                continue;
            }
            let direct = PathBuf::from(trimmed);
            if direct.is_dir() {
                return Some(direct);
            }
            let without_dot = PathBuf::from(trimmed.trim_end_matches('.'));
            if without_dot.is_dir() {
                return Some(without_dot);
            }
        }
    }
    None
}

fn find_mount_block_device(mount_point: &Path) -> Option<PathBuf> {
    let mounts = read_mounts_table()?;
    let target_canon = fs::canonicalize(mount_point).unwrap_or_else(|_| mount_point.to_path_buf());
    for line in mounts.lines() {
        let mut parts = line.split_whitespace();
        let Some(dev) = parts.next() else {
            continue;
        };
        let Some(mnt_raw) = parts.next() else {
            continue;
        };
        let mnt = PathBuf::from(mnt_raw.replace("\\040", " "));
        let mnt_canon = fs::canonicalize(&mnt).unwrap_or(mnt.clone());
        if mnt == mount_point || mnt_canon == target_canon {
            return Some(PathBuf::from(dev));
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn is_mount_point_mounted(mount_point: &Path, dev_path: Option<&Path>) -> bool {
    let Some(mounts) = read_mounts_table() else {
        return false;
    };
    let dev_canon =
        dev_path.and_then(|d| fs::canonicalize(d).ok().or_else(|| Some(d.to_path_buf())));
    for line in mounts.lines() {
        let mut parts = line.split_whitespace();
        let Some(dev) = parts.next() else {
            continue;
        };
        let Some(mnt_raw) = parts.next() else {
            continue;
        };
        let mnt = PathBuf::from(mnt_raw.replace("\\040", " "));
        if mnt == mount_point {
            return true;
        }
        if let Some(ref dc) = dev_canon {
            let line_dev = PathBuf::from(dev);
            let line_dev_canon = fs::canonicalize(&line_dev).unwrap_or(line_dev);
            if &line_dev_canon == dc {
                return true;
            }
        }
    }
    false
}

pub fn find_duckypad_msc_block_device(disk_label: &str, allow_whole_disk: bool) -> Option<PathBuf> {
    let clean_label = disk_label.trim();
    if !clean_label.is_empty() {
        let by_label = PathBuf::from(format!("/dev/disk/by-label/{clean_label}"));
        if by_label.exists() {
            if let Ok(canon) = fs::canonicalize(&by_label) {
                return Some(canon);
            }
        }
        // Case-insensitive scan in /dev/disk/by-label for exact disk_label
        if let Ok(entries) = fs::read_dir("/dev/disk/by-label") {
            for entry in entries.flatten() {
                let label = entry.file_name().to_string_lossy().to_string();
                if label.eq_ignore_ascii_case(clean_label) {
                    if let Ok(canon) = fs::canonicalize(entry.path()) {
                        return Some(canon);
                    }
                }
            }
        }
    }

    // Collect any block devices in /dev/disk/by-label that belong to a DIFFERENT duckyPad label
    // so we never accidentally grab another duckyPad's SD card when `disk_label` is specified.
    let mut foreign_duckypad_devs = std::collections::HashSet::new();
    if let Ok(entries) = fs::read_dir("/dev/disk/by-label") {
        for entry in entries.flatten() {
            let label = entry.file_name().to_string_lossy().to_string();
            let is_dp_label = label.starts_with("DP24_")
                || label.starts_with("DP20_")
                || label.starts_with("DUCKYPAD");
            if is_dp_label {
                if clean_label.is_empty() {
                    if let Ok(canon) = fs::canonicalize(entry.path()) {
                        return Some(canon);
                    }
                } else if !label.eq_ignore_ascii_case(clean_label) {
                    if let Ok(canon) = fs::canonicalize(entry.path()) {
                        foreign_duckypad_devs.insert(canon);
                    }
                }
            }
        }
    }

    let label_suffix = clean_label
        .split_once('_')
        .map(|(_, sfx)| sfx.to_ascii_uppercase())
        .filter(|s| !s.is_empty());

    if let Ok(entries) = fs::read_dir("/dev/disk/by-id") {
        let mut whole_disk_fallback: Option<PathBuf> = None;
        let mut generic_part_fallback: Option<PathBuf> = None;
        for entry in entries.flatten() {
            let id_name = entry.file_name().to_string_lossy().to_string();
            if id_name.contains("TinyUSB_Flash_Storage") || id_name.contains("duckyPad") {
                if let Ok(canon) = fs::canonicalize(entry.path()) {
                    if foreign_duckypad_devs.contains(&canon) {
                        continue;
                    }
                    let matches_suffix = label_suffix
                        .as_ref()
                        .map(|sfx| id_name.to_ascii_uppercase().contains(sfx))
                        .unwrap_or(true);

                    if id_name.contains("-part") {
                        if matches_suffix {
                            return Some(canon);
                        } else if generic_part_fallback.is_none() {
                            generic_part_fallback = Some(canon);
                        }
                        continue;
                    }
                    // Check if kernel has already created the first partition node (e.g. /dev/sdh1)
                    let part1 = PathBuf::from(format!("{}1", canon.display()));
                    if part1.exists() && !foreign_duckypad_devs.contains(&part1) {
                        if matches_suffix {
                            return Some(part1);
                        } else if generic_part_fallback.is_none() {
                            generic_part_fallback = Some(part1);
                        }
                        continue;
                    }
                    if matches_suffix || whole_disk_fallback.is_none() {
                        whole_disk_fallback = Some(canon);
                    }
                }
            }
        }
        if let Some(part) = generic_part_fallback {
            return Some(part);
        }
        if allow_whole_disk && whole_disk_fallback.is_some() {
            return whole_disk_fallback;
        }
    }

    None
}

pub(crate) fn wait_and_mount_duckypad_msc(
    disk_label: &str,
) -> Result<(PathBuf, Option<PathBuf>), String> {
    let target_label_opt = if disk_label.trim().is_empty() {
        None
    } else {
        Some(disk_label.trim())
    };
    let target_model_opt = if disk_label.starts_with("DP20_") {
        Some(DP_MODEL_OG_DUCKYPAD)
    } else if disk_label.starts_with("DP24_") {
        Some(DP_MODEL_DUCKYPAD_PRO)
    } else {
        None
    };

    #[cfg(target_os = "windows")]
    {
        for _ in 0..48 {
            if let Some(mounted) =
                find_mounted_duckypad_sd_for_device(None, target_label_opt, target_model_opt)
            {
                std::thread::sleep(std::time::Duration::from_millis(350));
                return Ok((mounted, None));
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        return Err("Timed out waiting for Windows to assign a drive letter to the duckyPad microSD card in USB Mass Storage mode".to_string());
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut last_udisks_err = String::new();
        let mut settled_dev_once = false;

        for attempt in 0..48 {
            if let Some(mounted) =
                find_mounted_duckypad_sd_for_device(None, target_label_opt, target_model_opt)
            {
                std::thread::sleep(std::time::Duration::from_millis(350));
                let dev = find_mount_block_device(&mounted)
                    .or_else(|| find_duckypad_msc_block_device(disk_label, true));
                return Ok((mounted, dev));
            }

            // Only allow superfloppy whole-disk fallback after ~4.5s (18 * 250ms) if no partition table exists
            let allow_whole_disk = attempt >= 18;
            if let Some(dev_path) = find_duckypad_msc_block_device(disk_label, allow_whole_disk) {
                if !settled_dev_once {
                    settled_dev_once = true;
                    // Give udev and udisksd a moment to finish probing the FAT32 partition before mounting
                    std::thread::sleep(std::time::Duration::from_millis(350));
                    if let Some(mounted) = find_mounted_duckypad_sd_for_device(
                        None,
                        target_label_opt,
                        target_model_opt,
                    ) {
                        let dev = find_mount_block_device(&mounted).or(Some(dev_path));
                        return Ok((mounted, dev));
                    }
                }

                let dev_str = dev_path.to_string_lossy().to_string();
                match run_storage_cmd(
                    "udisksctl",
                    &["mount", "--no-user-interaction", "-b", &dev_str],
                ) {
                    Ok(out) => {
                        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                        if !stderr.is_empty() && !stderr.contains("AlreadyMounted") {
                            last_udisks_err = format!("{} ({})", stderr, dev_path.display());
                        }
                        if let Some(parsed_mnt) = parse_udisks_mount_path(&stdout) {
                            std::thread::sleep(std::time::Duration::from_millis(300));
                            let dev = find_mount_block_device(&parsed_mnt).or(Some(dev_path));
                            return Ok((parsed_mnt, dev));
                        }
                    }
                    Err(e) => {
                        last_udisks_err =
                            format!("Failed invoking udisksctl on {}: {e}", dev_path.display());
                    }
                }

                std::thread::sleep(std::time::Duration::from_millis(300));
                if let Some(mounted) =
                    find_mounted_duckypad_sd_for_device(None, target_label_opt, target_model_opt)
                {
                    std::thread::sleep(std::time::Duration::from_millis(300));
                    let dev = find_mount_block_device(&mounted).or(Some(dev_path));
                    return Ok((mounted, dev));
                }
            } else {
                std::thread::sleep(std::time::Duration::from_millis(250));
            }
        }

        if last_udisks_err.is_empty() {
            Err("Timed out waiting for duckyPad microSD partition to appear and mount in USB Mass Storage mode".to_string())
        } else {
            Err(format!(
                "Timed out mounting duckyPad microSD in USB Mass Storage mode: {last_udisks_err}"
            ))
        }
    }
}

pub(crate) fn unmount_duckypad_sd_cleanly(
    mount_root: &Path,
    known_dev: Option<&Path>,
) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let _ = (mount_root, known_dev);
        std::thread::sleep(std::time::Duration::from_millis(1200));
        return Ok(());
    }
    #[cfg(not(target_os = "windows"))]
    {
        let block_dev = known_dev
            .map(|p| p.to_path_buf())
            .or_else(|| find_mount_block_device(mount_root));

        // Flush Linux filesystem buffers and allow USB 1.1 MSC pipeline to drain before unmounting
        if let Err(e) = run_storage_cmd("sync", &[]) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "STORAGE",
                format!("Failed running sync before unmount: {e}"),
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(600));

        for attempt in 0..24 {
            if !is_mount_point_mounted(mount_root, block_dev.as_deref()) {
                if let Err(e) = run_storage_cmd("sync", &[]) {
                    crate::bridge::device_controller::record_system_event(
                        "WARN",
                        "STORAGE",
                        format!("Failed running sync after unmount: {e}"),
                    );
                }
                // Give the STM32 TinyUSB MSC driver and SD card flash controller 1 full second
                // to finish internal FAT table / wear-leveling commits before software reset
                std::thread::sleep(std::time::Duration::from_millis(1000));
                return Ok(());
            }

            if let Some(ref dev_path) = block_dev {
                let dev_str = dev_path.to_string_lossy().to_string();
                if let Err(e) = run_storage_cmd(
                    "udisksctl",
                    &["unmount", "--no-user-interaction", "-b", &dev_str],
                ) {
                    crate::bridge::device_controller::record_system_event(
                        "WARN",
                        "STORAGE",
                        format!(
                            "Failed running udisksctl unmount on {}: {e}",
                            dev_path.display()
                        ),
                    );
                }
            }

            if attempt >= 3 && is_mount_point_mounted(mount_root, block_dev.as_deref()) {
                let mnt_str = mount_root.to_string_lossy().to_string();
                if let Err(e) = run_storage_cmd("umount", &[&mnt_str]) {
                    crate::bridge::device_controller::record_system_event(
                        "WARN",
                        "STORAGE",
                        format!("Failed running umount on {}: {e}", mount_root.display()),
                    );
                }
            }

            if let Err(e) = run_storage_cmd("sync", &[]) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "STORAGE",
                    format!("Failed running sync during unmount retry: {e}"),
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(300));
        }

        if !is_mount_point_mounted(mount_root, block_dev.as_deref()) {
            if let Err(e) = run_storage_cmd("sync", &[]) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "STORAGE",
                    format!("Failed running final sync after unmount: {e}"),
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
            Ok(())
        } else {
            Err(format!(
                "Volume {} is still busy and could not be unmounted cleanly",
                mount_root.display()
            ))
        }
    }
}

pub fn unmount_mounted_duckypad_sd(current_mount: Option<&Path>) -> Result<String, String> {
    let mount_root = current_mount
        .filter(|p| p.exists())
        .map(|p| p.to_path_buf())
        .or_else(find_mounted_duckypad_sd)
        .ok_or_else(|| "No mounted duckyPad microSD card detected".to_string())?;

    let block_dev = find_mount_block_device(&mount_root);
    unmount_duckypad_sd_cleanly(&mount_root, block_dev.as_deref())?;

    // If the duckyPad Pro was in USB Mass Storage mode, reset it back to normal HID mode
    if find_duckypad_msc_block_device("", true).is_some() {
        let _guard = HID_IO_LOCK.lock().ok();
        if let Ok(dev) = open_duckypad_hid_for_serial(None) {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x15;
            tx[2] = HID_CMD_SOFTWARE_RESET;
            tx[3] = 0;
            if let Err(e) = dev.write(&tx) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "HID",
                    format!("Failed sending software reset after unmount: {e}"),
                );
            }
        }
    }

    Ok(format!(
        "Unmounted microSD card at {}",
        mount_root.display()
    ))
}

pub fn detect_unmounted_duckypad_sd(disk_label: &str) -> Option<PathBuf> {
    let target_label_opt = if disk_label.trim().is_empty() {
        None
    } else {
        Some(disk_label.trim())
    };
    if find_mounted_duckypad_sd_for_device(None, target_label_opt, None).is_some() {
        return None;
    }
    if let Some(dev) = find_duckypad_msc_block_device(disk_label, true) {
        return Some(dev);
    }
    if let Ok(entries) = fs::read_dir("/sys/block") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("mmcblk") {
                let p1 = PathBuf::from(format!("/dev/{name}p1"));
                if p1.exists() {
                    return Some(p1);
                }
                let whole = PathBuf::from(format!("/dev/{name}"));
                if whole.exists() {
                    return Some(whole);
                }
            }
        }
    }
    None
}

pub fn mount_duckypad_sd_only(disk_label: &str) -> Result<PathBuf, String> {
    let target_label_opt = if disk_label.trim().is_empty() {
        None
    } else {
        Some(disk_label.trim())
    };
    let target_model_opt = if disk_label.starts_with("DP20_") {
        Some(DP_MODEL_OG_DUCKYPAD)
    } else if disk_label.starts_with("DP24_") {
        Some(DP_MODEL_DUCKYPAD_PRO)
    } else {
        None
    };

    if let Some(mounted) =
        find_mounted_duckypad_sd_for_device(None, target_label_opt, target_model_opt)
    {
        return Ok(mounted);
    }
    if let Some(dev_path) = detect_unmounted_duckypad_sd(disk_label) {
        let dev_str = dev_path.to_string_lossy().to_string();
        let out = run_storage_cmd(
            "udisksctl",
            &["mount", "--no-user-interaction", "-b", &dev_str],
        )
        .map_err(|e| format!("Failed invoking udisksctl mount: {e}"))?;
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if let Some(parsed_mnt) = parse_udisks_mount_path(&stdout) {
            std::thread::sleep(std::time::Duration::from_millis(300));
            return Ok(parsed_mnt);
        }
        std::thread::sleep(std::time::Duration::from_millis(350));
        if let Some(mounted) =
            find_mounted_duckypad_sd_for_device(None, target_label_opt, target_model_opt)
        {
            return Ok(mounted);
        }
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if !stderr.is_empty() {
            return Err(format!(
                "udisksctl mount failed for {}: {stderr}",
                dev_path.display()
            ));
        }
    }

    if disk_label.starts_with("DP20_") {
        return Err(
            "duckyPad (2020) does not support USB Mass Storage mode — use Sync / Reload over USB HID or insert the microSD card into a card reader."
                .to_string(),
        );
    }

    // If a duckyPad Pro is connected over USB HID, switch it into USB MSC mode and mount
    let entered_msc = {
        let _guard = HID_IO_LOCK.lock().ok();
        if let Ok(device) = open_duckypad_hid_for_serial(None) {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x18;
            tx[2] = HID_CMD_SOFTWARE_RESET;
            tx[3] = 1; // reboot into USB MSC mode
            device.write(&tx).is_ok()
        } else {
            false
        }
    };

    if entered_msc {
        std::thread::sleep(std::time::Duration::from_millis(800));
        let (mounted, _dev) = wait_and_mount_duckypad_msc(disk_label)?;
        return Ok(mounted);
    }

    Err(
        "No unmounted duckyPad microSD card or connected duckyPad Pro detected to mount"
            .to_string(),
    )
}

pub fn build_hid_file_command_packet(seq: u8, cmd: u8, payload: &[u8]) -> [u8; 64] {
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = seq;
    tx[2] = cmd;
    let copy_len = payload.len().min(60);
    tx[3..3 + copy_len].copy_from_slice(&payload[..copy_len]);
    tx
}

const HID_CMD_LIST_FILES_V1: u8 = 0x0A;
const HID_CMD_OP_RESUME_V1: u8 = 0x0C;

fn hid_txrx(
    dev: &crate::hardware::storage::DuckyHidDevice,
    seq: &mut u8,
    cmd: u8,
    payload: &[u8],
    timeout_ms: i32,
) -> Result<[u8; 64], String> {
    *seq = seq.wrapping_add(1);
    if *seq == 0 {
        *seq = 1;
    }
    let tx = build_hid_file_command_packet(*seq, cmd, payload);
    dev.write(&tx)
        .map_err(|e| format!("HID write error (cmd 0x{cmd:02X}): {e}"))?;
    let mut rx = [0u8; 64];
    let n = dev
        .read_timeout(&mut rx, timeout_ms)
        .map_err(|e| format!("HID read error (cmd 0x{cmd:02X}): {e}"))?;
    if n == 0 {
        return Err(format!("HID response timed out for command 0x{cmd:02X}"));
    }
    if rx[2] == HID_STATUS_ERROR {
        return Err(format!(
            "duckyPad reported HID error on command 0x{cmd:02X}"
        ));
    }
    Ok(rx)
}

fn query_dp20_fw_mode(
    dev: &crate::hardware::storage::DuckyHidDevice,
    seq: &mut u8,
) -> (u8, bool, String) {
    crate::hardware::storage::drain_pending_hid_reports(dev);
    if let Ok(rx) = hid_txrx(dev, seq, 0x00, &[], 500) {
        let major = rx[3];
        let minor = rx[4];
        let patch = rx[5];
        let is_dds = rx[20] == b'D' && rx[21] == b'D' && rx[22] == b'S';
        return (major, is_dds, format!("{major}.{minor}.{patch}"));
    }
    (1, false, String::new())
}

fn hid_list_dir_v1(
    dev: &crate::hardware::storage::DuckyHidDevice,
    seq: &mut u8,
    remote_dir: &str,
) -> Vec<(u8, String)> {
    let mut out = Vec::new();
    let mut payload = remote_dir.as_bytes().to_vec();
    payload.push(0);
    let Ok(mut rx) = hid_txrx(dev, seq, HID_CMD_LIST_FILES_V1, &payload, 1000) else {
        return out;
    };
    for _ in 0..256 {
        if rx[2] != 0 {
            break;
        }
        let ftype = rx[3];
        let raw = &rx[4..];
        let nul = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        if nul > 0 {
            let name = String::from_utf8_lossy(&raw[..nul]).trim().to_string();
            if !name.is_empty() && !name.starts_with('.') {
                out.push((ftype, name));
            }
        }
        let Ok(next_rx) = hid_txrx(dev, seq, HID_CMD_OP_RESUME_V1, &[], 1000) else {
            break;
        };
        rx = next_rx;
    }
    out
}

fn hid_read_file_bytes_v1(
    dev: &crate::hardware::storage::DuckyHidDevice,
    seq: &mut u8,
    remote_path: &str,
) -> Result<Vec<u8>, String> {
    let mut payload = remote_path.as_bytes().to_vec();
    payload.push(0);
    let mut rx = hid_txrx(dev, seq, HID_CMD_READ_FILE, &payload, 1200)?;
    let mut out = Vec::new();
    for _ in 0..2048 {
        if rx[2] != 0 {
            break;
        }
        let raw = &rx[3..63];
        let nul = raw.iter().position(|&b| b == 0).unwrap_or(raw.len());
        if nul > 0 {
            out.extend_from_slice(&raw[..nul]);
        }
        if nul < 60 {
            let _ = hid_txrx(dev, seq, HID_CMD_OP_RESUME_V1, &[], 800);
            break;
        }
        match hid_txrx(dev, seq, HID_CMD_OP_RESUME_V1, &[], 1200) {
            Ok(next_rx) => rx = next_rx,
            Err(_) => break,
        }
    }
    Ok(out)
}

fn hid_read_file_bytes_evo(
    dev: &crate::hardware::storage::DuckyHidDevice,
    seq: &mut u8,
    remote_path: &str,
) -> Result<Vec<u8>, String> {
    let _ = hid_txrx(
        dev,
        seq,
        HID_CMD_OPEN_FILE_FOR_READING,
        remote_path.as_bytes(),
        1200,
    )?;
    let mut out = Vec::new();
    for _ in 0..2048 {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0;
        tx[2] = HID_CMD_READ_FILE;
        dev.write(&tx)
            .map_err(|e| format!("HID write error (READ_FILE): {e}"))?;
        let mut rx = [0u8; 64];
        let n = dev
            .read_timeout(&mut rx, 1200)
            .map_err(|e| format!("HID read error (READ_FILE): {e}"))?;
        if n == 0 {
            break;
        }
        let chunk_len = (rx[2] as usize).min(60);
        if chunk_len == 0 {
            break;
        }
        out.extend_from_slice(&rx[3..3 + chunk_len]);
    }
    Ok(out)
}

fn hid_read_file_bytes_auto(
    dev: &crate::hardware::storage::DuckyHidDevice,
    seq: &mut u8,
    remote_path: &str,
    use_evo_protocol: bool,
) -> Result<Vec<u8>, String> {
    if use_evo_protocol {
        hid_read_file_bytes_evo(dev, seq, remote_path)
            .or_else(|_| hid_read_file_bytes_v1(dev, seq, remote_path))
    } else {
        hid_read_file_bytes_v1(dev, seq, remote_path)
            .or_else(|_| hid_read_file_bytes_evo(dev, seq, remote_path))
    }
}

fn hid_write_file_bytes(
    dev: &crate::hardware::storage::DuckyHidDevice,
    seq: &mut u8,
    remote_path: &str,
    content: &[u8],
) -> Result<(), String> {
    let _ = hid_txrx(
        dev,
        seq,
        HID_CMD_OPEN_FILE_FOR_WRITING,
        remote_path.as_bytes(),
        1500,
    )?;
    for chunk in content.chunks(60) {
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = chunk.len() as u8;
        tx[2] = HID_CMD_WRITE_FILE;
        tx[3..3 + chunk.len()].copy_from_slice(chunk);
        dev.write(&tx)
            .map_err(|e| format!("HID write error (WRITE_FILE): {e}"))?;
        let mut rx = [0u8; 64];
        let n = dev
            .read_timeout(&mut rx, 1500)
            .map_err(|e| format!("HID read error (WRITE_FILE): {e}"))?;
        if n == 0 || rx[2] == HID_STATUS_ERROR {
            return Err("duckyPad reported HID error while writing file chunk".to_string());
        }
    }
    let _ = hid_txrx(dev, seq, HID_CMD_CLOSE_FILE, &[], 1200)?;
    Ok(())
}

pub fn read_duckypad_2020_profiles_via_hid_with_progress<F>(
    serial: &str,
    mut on_progress: F,
) -> Result<Vec<DuckyProfile>, String>
where
    F: FnMut(i32, &str, &str),
{
    struct HidSyncGuard;
    impl Drop for HidSyncGuard {
        fn drop(&mut self) {
            MSC_SYNC_IN_PROGRESS.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
    MSC_SYNC_IN_PROGRESS.store(true, std::sync::atomic::Ordering::SeqCst);
    let _hid_sync_guard = HidSyncGuard;

    let serial_opt = if serial.is_empty() || serial == "-" {
        None
    } else {
        Some(serial)
    };
    let staging_dir = profile_cache_dir_for_serial(serial);
    fs::create_dir_all(&staging_dir)
        .map_err(|e| format!("Failed creating staging dir {}: {e}", staging_dir.display()))?;

    on_progress(
        15,
        "Step 1/3: Connecting to duckyPad (2020) over USB HID...",
        &format!("• Opening duckyPad (2020) HID channel (SN: {serial})..."),
    );

    let mut seq = 0x20u8;
    let _guard = HID_IO_LOCK
        .lock()
        .map_err(|_| "HID lock poisoned".to_string())?;
    let dev = open_duckypad_hid_for_serial(serial_opt)?;
    let (fw_major, is_dds_fw, fw_ver_str) = query_dp20_fw_mode(&dev, &mut seq);
    let use_evo_protocol = fw_major >= 2 || is_dds_fw;

    // Discover profile folders via HID_CMD_LIST_FILES (v1.x) or HID_CMD_DUMP_SD (EVO/DDS)
    on_progress(
        35,
        "Step 2/3: Scanning duckyPad (2020) microSD card over USB HID...",
        "• Querying profile directories and config.txt files over USB HID...",
    );

    // Clear old profile directories and stale profile_info.txt in staging_dir before dumping fresh ones
    let _ = fs::remove_file(staging_dir.join("profile_info.txt"));
    if let Ok(entries) = fs::read_dir(&staging_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
                    if parse_profile_folder_name(fname).is_some() {
                        let _ = fs::remove_dir_all(&p);
                    }
                }
            }
        }
    }

    let mut discovered_dirs: Vec<String> = Vec::new();
    if !use_evo_protocol {
        for (ftype, name) in hid_list_dir_v1(&dev, &mut seq, "") {
            if ftype == 1
                && parse_profile_folder_name(&name).is_some()
                && !discovered_dirs.contains(&name)
            {
                discovered_dirs.push(name);
            }
        }
    }

    if discovered_dirs.is_empty() {
        // Walk root entries using EVO/DDS HID_CMD_DUMP_SD (0x20)
        for _ in 0..512 {
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0;
            tx[2] = HID_CMD_DUMP_SD;
            if dev.write(&tx).is_err() {
                break;
            }
            let mut rx = [0u8; 64];
            let Ok(n) = dev.read_timeout(&mut rx, 1200) else {
                break;
            };
            if n == 0 || rx[1] == 4 || rx[2] == HID_STATUS_ERROR {
                break;
            }
            if rx[1] == 1 {
                let raw_slice = &rx[2..];
                let nul_pos = raw_slice
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap_or(raw_slice.len());
                if nul_pos > 0 {
                    let name = String::from_utf8_lossy(&raw_slice[..nul_pos])
                        .trim_start_matches('/')
                        .trim()
                        .to_string();
                    if parse_profile_folder_name(&name).is_some()
                        && !discovered_dirs.contains(&name)
                    {
                        discovered_dirs.push(name);
                    }
                }
            }
        }
    }

    // Try reading profile_info.txt if present
    if let Ok(info_bytes) =
        hid_read_file_bytes_auto(&dev, &mut seq, "/profile_info.txt", use_evo_protocol)
    {
        if !info_bytes.is_empty() {
            let _ = fs::write(staging_dir.join("profile_info.txt"), &info_bytes);
        }
    }

    // Also include any folders referenced in profile_info.txt
    if let Ok(info_text) = fs::read_to_string(staging_dir.join("profile_info.txt")) {
        for line in info_text.lines() {
            if let Some((num_s, pname)) = line.trim().split_once(char::is_whitespace) {
                let pname = pname.trim();
                let num_s = num_s.trim();
                let c1 = format!("profile{num_s}_{pname}");
                let c2 = format!("profile_{pname}");
                if !discovered_dirs.contains(&c1) {
                    discovered_dirs.push(c1);
                }
                if !discovered_dirs.contains(&c2) {
                    discovered_dirs.push(c2);
                }
            }
        }
    }

    for (idx, folder) in discovered_dirs.iter().enumerate() {
        let remote_cfg = format!("/{folder}/config.txt");
        let Ok(cfg_bytes) = hid_read_file_bytes_auto(&dev, &mut seq, &remote_cfg, use_evo_protocol)
        else {
            continue;
        };
        if cfg_bytes.is_empty() {
            continue;
        }
        let local_prof_dir = staging_dir.join(folder);
        let _ = fs::create_dir_all(&local_prof_dir);
        let _ = fs::write(local_prof_dir.join("config.txt"), &cfg_bytes);

        let pct = 40 + ((idx + 1) * 45 / discovered_dirs.len().max(1)) as i32;
        on_progress(
            pct.min(88),
            &format!("Reading {folder} over USB HID..."),
            &format!("• Loaded {remote_cfg} ({} bytes)", cfg_bytes.len()),
        );

        let cfg_text = String::from_utf8_lossy(&cfg_bytes);
        if !use_evo_protocol {
            // On v1.x firmware, list the profile directory once to know exactly which key files exist
            let dir_files: std::collections::HashSet<String> =
                hid_list_dir_v1(&dev, &mut seq, folder)
                    .into_iter()
                    .filter(|(ft, _)| *ft == 0)
                    .map(|(_, fname)| fname)
                    .collect();
            for key_idx in 1..=15usize {
                for suffix in ["txt", "dsb", "release.txt", "release.dsb"] {
                    let fname = if suffix.starts_with("release") {
                        format!("key{key_idx}-{suffix}")
                    } else {
                        format!("key{key_idx}.{suffix}")
                    };
                    if !dir_files.is_empty() && !dir_files.contains(&fname) {
                        continue;
                    }
                    let remote_file = format!("/{folder}/{fname}");
                    if let Ok(bytes) = hid_read_file_bytes_auto(&dev, &mut seq, &remote_file, false)
                    {
                        if !bytes.is_empty() {
                            let _ = fs::write(local_prof_dir.join(&fname), &bytes);
                        }
                    }
                }
            }
        } else {
            for key_idx in 1..=15usize {
                let z_prefix = format!("z{key_idx} ");
                let sw_prefix = format!("SWCOLOR_{key_idx} ");
                let maybe_configured = cfg_text
                    .lines()
                    .any(|l| l.starts_with(&z_prefix) || l.starts_with(&sw_prefix));
                if !maybe_configured {
                    continue;
                }
                for suffix in ["txt", "dsb", "release.txt", "release.dsb"] {
                    let fname = if suffix.starts_with("release") {
                        format!("key{key_idx}-{suffix}")
                    } else {
                        format!("key{key_idx}.{suffix}")
                    };
                    let remote_file = format!("/{folder}/{fname}");
                    if let Ok(bytes) = hid_read_file_bytes_auto(&dev, &mut seq, &remote_file, true)
                    {
                        if !bytes.is_empty() {
                            let _ = fs::write(local_prof_dir.join(&fname), &bytes);
                        }
                    }
                }
            }
        }
    }
    drop(_guard);

    let profiles = load_all_sd_profiles(&staging_dir)
        .map_err(|e| format!("Failed parsing duckyPad (2020) profiles: {e}"))?;
    if profiles.is_empty() {
        return Err(
            "No profiles could be read over USB HID from duckyPad (2020). You can also insert the microSD card directly into your computer."
                .to_string(),
        );
    }
    let meta = DuckySdDeviceMeta {
        serial_number: serial.to_string(),
        hw_model: DP_MODEL_OG_DUCKYPAD,
        model_name: model_name_for_hw_model(DP_MODEL_OG_DUCKYPAD).to_string(),
        firmware_version: fw_ver_str,
        disk_label: crate::hardware::storage::format_disk_label_for_device(
            DP_MODEL_OG_DUCKYPAD,
            serial,
        ),
    };
    let fp = compute_profiles_fingerprint(&profiles);
    crate::hardware::profile::sync_profiles_to_device_cache_with_meta(
        serial,
        &profiles,
        Some(&meta),
    );
    mark_profiles_sd_synced(serial, &profiles);
    on_progress(
        100,
        "Validated: Profiles loaded from duckyPad (2020) over USB HID!",
        &format!(
            "✓ Loaded {} profile(s) from duckyPad (2020) [hash #{}]",
            profiles.len(),
            &fp[..8.min(fp.len())]
        ),
    );
    Ok(profiles)
}

pub fn write_duckypad_2020_profiles_via_hid_with_progress<F>(
    disk_label: &str,
    serial: &str,
    profiles: &[DuckyProfile],
    restore_profile: u8,
    mut on_progress: F,
) -> Result<SdWriteSummary, String>
where
    F: FnMut(i32, &str, &str),
{
    struct HidSyncGuard;
    impl Drop for HidSyncGuard {
        fn drop(&mut self) {
            MSC_SYNC_IN_PROGRESS.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
    MSC_SYNC_IN_PROGRESS.store(true, std::sync::atomic::Ordering::SeqCst);
    let _hid_sync_guard = HidSyncGuard;

    let serial_opt = if serial.is_empty() || serial == "-" {
        None
    } else {
        Some(serial)
    };
    let staging_dir = profile_cache_dir_for_serial(serial);
    fs::create_dir_all(&staging_dir)
        .map_err(|e| format!("Failed creating staging dir {}: {e}", staging_dir.display()))?;

    on_progress(
        20,
        "Step 1/4: Compiling 15-key duckyPad (2020) profiles & .dsb bytecode...",
        &format!(
            "• Staging {} profile(s) for duckyPad (2020) ({disk_label})...",
            profiles.len()
        ),
    );

    let meta = DuckySdDeviceMeta {
        serial_number: serial.to_string(),
        hw_model: DP_MODEL_OG_DUCKYPAD,
        model_name: model_name_for_hw_model(DP_MODEL_OG_DUCKYPAD).to_string(),
        firmware_version: String::new(),
        disk_label: disk_label.to_string(),
    };
    let summary = write_profiles_to_sd_root_for_model(
        &staging_dir,
        profiles,
        DP_MODEL_OG_DUCKYPAD,
        Some(&meta),
    )?;

    on_progress(
        45,
        "Step 2/4: Transferring profile files to duckyPad (2020) over USB HID...",
        "• Connected to duckyPad (2020) HID file interface...",
    );

    let mut seq = 0x40u8;
    {
        let _guard = HID_IO_LOCK
            .lock()
            .map_err(|_| "HID lock poisoned".to_string())?;
        let dev = open_duckypad_hid_for_serial(serial_opt)?;
        let (fw_major, is_dds_fw, _) = query_dp20_fw_mode(&dev, &mut seq);
        let use_pinfo_folders = fw_major >= 2 || is_dds_fw;

        // Write profile_info.txt
        if use_pinfo_folders {
            if let Ok(info_bytes) = fs::read(staging_dir.join("profile_info.txt")) {
                let _ = hid_txrx(
                    &dev,
                    &mut seq,
                    HID_CMD_DELETE_FILE,
                    b"profile_info.txt",
                    800,
                );
                let _ = hid_write_file_bytes(&dev, &mut seq, "profile_info.txt", &info_bytes);
            }
        }

        for (idx, prof) in profiles.iter().enumerate() {
            let local_folder = prof.folder_name_for_model(DP_MODEL_OG_DUCKYPAD);
            let remote_folder = if use_pinfo_folders {
                local_folder.clone()
            } else {
                format!("profile{}_{}", prof.number, prof.name)
            };
            let local_dir = staging_dir.join(&local_folder);
            if !local_dir.is_dir() {
                continue;
            }
            let pct = 50 + ((idx + 1) * 35 / profiles.len().max(1)) as i32;
            on_progress(
                pct.min(88),
                &format!("Step 3/4: Writing {remote_folder} over USB HID..."),
                &format!(
                    "• Syncing profile #{} ({}) to duckyPad (2020)...",
                    prof.number, prof.name
                ),
            );

            let _ = hid_txrx(
                &dev,
                &mut seq,
                HID_CMD_DELETE_DIR,
                remote_folder.as_bytes(),
                1200,
            );
            let _ = hid_txrx(
                &dev,
                &mut seq,
                HID_CMD_CREATE_DIR,
                remote_folder.as_bytes(),
                1200,
            );

            if let Ok(entries) = fs::read_dir(&local_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if !path.is_file() {
                        continue;
                    }
                    let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
                        continue;
                    };
                    if let Ok(bytes) = fs::read(&path) {
                        let remote_path = format!("/{remote_folder}/{fname}");
                        hid_write_file_bytes(&dev, &mut seq, &remote_path, &bytes)?;
                    }
                }
            }
        }

        on_progress(
            92,
            "Step 4/4: Rebooting duckyPad (2020) to reload updated profiles...",
            "• Sending software reset (0x14) to duckyPad (2020)...",
        );
        let reset_pkt = build_hid_file_command_packet(0x99, HID_CMD_SOFTWARE_RESET, &[0]);
        let _ = dev.write(&reset_pkt);
    }

    std::thread::sleep(std::time::Duration::from_millis(1200));
    if restore_profile > 0 {
        let _ = crate::kwin::auto_switcher::send_hid_switch_profile(restore_profile);
    }

    crate::hardware::profile::sync_profiles_to_device_cache_with_meta(
        serial,
        profiles,
        Some(&meta),
    );
    mark_profiles_sd_synced(serial, profiles);

    let short_fp = &summary.validated_fingerprint[..8.min(summary.validated_fingerprint.len())];
    on_progress(
        100,
        "Validated: All changes synced to duckyPad (2020)!",
        &format!(
            "✓ Synced {} profile(s) over USB HID to duckyPad (2020) [hash #{short_fp}]",
            summary.profiles_count
        ),
    );
    Ok(summary)
}

pub fn read_duckypad_pro_profiles_via_msc(
    disk_label: &str,
    serial: &str,
    restore_profile: u8,
) -> Result<Vec<DuckyProfile>, String> {
    read_duckypad_pro_profiles_via_msc_with_progress(
        disk_label,
        serial,
        restore_profile,
        |_, _, _| {},
    )
}

pub fn read_duckypad_pro_profiles_via_msc_with_progress<F>(
    disk_label: &str,
    serial: &str,
    restore_profile: u8,
    mut on_progress: F,
) -> Result<Vec<DuckyProfile>, String>
where
    F: FnMut(i32, &str, &str),
{
    let serial_opt = if serial.is_empty() || serial == "-" {
        None
    } else {
        Some(serial)
    };
    let label_opt = if disk_label.trim().is_empty() {
        None
    } else {
        Some(disk_label.trim())
    };
    let is_dp20 = disk_label.starts_with("DP20_");
    let model_opt = if is_dp20 {
        Some(DP_MODEL_OG_DUCKYPAD)
    } else if disk_label.starts_with("DP24_") {
        Some(DP_MODEL_DUCKYPAD_PRO)
    } else {
        None
    };

    if let Some(already_mounted) =
        find_mounted_duckypad_sd_for_device(serial_opt, label_opt, model_opt)
    {
        on_progress(
            35,
            "Reading profiles from mounted duckyPad microSD...",
            &format!("• Found matching microSD at {}", already_mounted.display()),
        );
        let profiles = load_all_sd_profiles(&already_mounted)
            .map_err(|e| format!("Failed reading mounted microSD: {e}"))?;
        if !profiles.is_empty() {
            let fp = compute_profiles_fingerprint(&profiles);
            sync_profiles_to_device_cache(serial, &profiles);
            mark_profiles_sd_synced(serial, &profiles);
            record_mounted_sd_space(serial, &already_mounted);
            on_progress(
                100,
                "Validated: Profiles loaded from duckyPad microSD!",
                &format!(
                    "✓ Loaded & verified {} profile(s) from {} [hash #{}]",
                    profiles.len(),
                    already_mounted.display(),
                    &fp[..8.min(fp.len())]
                ),
            );
            return Ok(profiles);
        }
    }

    if is_dp20 {
        return read_duckypad_2020_profiles_via_hid_with_progress(serial, on_progress);
    }

    struct MscGuard;
    impl Drop for MscGuard {
        fn drop(&mut self) {
            MSC_SYNC_IN_PROGRESS.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
    MSC_SYNC_IN_PROGRESS.store(true, std::sync::atomic::Ordering::SeqCst);
    let _msc_guard = MscGuard;

    let already_has_block_dev = find_duckypad_msc_block_device(disk_label, true).is_some();
    if !already_has_block_dev {
        on_progress(
            15,
            "Step 1/4: Switching duckyPad Pro to USB Mass Storage mode...",
            &format!("• Sending USB MSC command (0x14) to duckyPad Pro ({disk_label})..."),
        );

        // 1. Reboot duckyPad Pro into USB Mass Storage mode (0x14, tx[3] = 1)
        {
            let _guard = HID_IO_LOCK.lock().ok();
            let dev = open_duckypad_hid_for_serial(serial_opt)?;
            let mut tx = [0u8; 64];
            tx[0] = HID_REPORT_ID_OUT;
            tx[1] = 0x11;
            tx[2] = HID_CMD_SOFTWARE_RESET;
            tx[3] = 1;
            dev.write(&tx)
                .map_err(|e| format!("Failed to enter USB storage mode: {e}"))?;
        }
    } else {
        on_progress(
            20,
            "Step 1/4: Detected unmounted duckyPad microSD block device...",
            "• Mounting duckyPad microSD partition via udisksctl...",
        );
    }

    on_progress(
        35,
        "Step 2/4: Waiting for microSD block device & mounting...",
        "• Waiting for duckyPad Pro microSD card to enumerate and mount...",
    );

    // 2. Wait for block device and mount
    let mount_res = wait_and_mount_duckypad_msc(disk_label);
    let loaded_profiles: Result<Vec<DuckyProfile>, String> = match mount_res {
        Ok((mount_root, real_dev)) => {
            let dev_str = real_dev
                .as_ref()
                .map(|d| d.display().to_string())
                .unwrap_or_else(|| disk_label.to_string());
            on_progress(
                60,
                "Step 3/4: Reading profiles & scripts from microSD...",
                &format!("✓ Mounted microSD ({dev_str}) at {}", mount_root.display()),
            );

            let res = match load_all_sd_profiles(&mount_root) {
                Ok(profiles) if !profiles.is_empty() => {
                    let fp = compute_profiles_fingerprint(&profiles);
                    sync_profiles_to_device_cache(serial, &profiles);
                    mark_profiles_sd_synced(serial, &profiles);
                    record_mounted_sd_space(serial, &mount_root);
                    on_progress(
                        80,
                        "Step 4/4: Flushing & unmounting microSD before returning to HID mode...",
                        &format!(
                            "✓ Read {} profile(s) from microSD [hash #{}]",
                            profiles.len(),
                            &fp[..8.min(fp.len())]
                        ),
                    );
                    Ok(profiles)
                }
                Ok(_) => Err(format!(
                    "No profile folders found on {}",
                    mount_root.display()
                )),
                Err(e) => Err(format!("Error reading profiles from SD card: {e}")),
            };

            // 3. Cleanly unmount and verify in /proc/mounts before resetting
            if let Err(e) = unmount_duckypad_sd_cleanly(&mount_root, real_dev.as_deref()) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "STORAGE",
                    format!("Unmount warning after MSC read: {e}"),
                );
            }
            res
        }
        Err(e) => Err(e),
    };

    // 4. Reset duckyPad Pro back to normal HID keypad mode (0x14, tx[3] = 0)
    for _ in 0..20 {
        let reset_ok = {
            let _guard = HID_IO_LOCK.lock().ok();
            if let Ok(dev) = open_duckypad_hid_for_serial(serial_opt) {
                let mut tx = [0u8; 64];
                tx[0] = HID_REPORT_ID_OUT;
                tx[1] = 0x12;
                tx[2] = HID_CMD_SOFTWARE_RESET;
                tx[3] = 0;
                dev.write(&tx).is_ok()
            } else {
                false
            }
        };
        if reset_ok {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(180));
    }

    // 5. Wait for normal HID mode to re-enumerate and restore active profile
    std::thread::sleep(std::time::Duration::from_millis(1200));
    for _ in 0..15 {
        let ready = {
            let _guard = HID_IO_LOCK.lock().ok();
            open_duckypad_hid_for_serial(serial_opt).is_ok()
        };
        if ready {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    drop(_msc_guard);
    if restore_profile > 0 && !crate::kwin::auto_switcher::send_hid_switch_profile(restore_profile)
    {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "HID",
            format!("Failed restoring active profile #{restore_profile} after MSC read"),
        );
    }

    if let Ok(ref profs) = loaded_profiles {
        on_progress(
            100,
            "Validated: Profiles reloaded from duckyPad Pro!",
            &format!(
                "✓ Cleanly unmounted microSD and restored duckyPad Pro to HID mode ({} profiles loaded)",
                profs.len()
            ),
        );
    }

    loaded_profiles
}

pub fn write_duckypad_pro_profiles_via_msc(
    disk_label: &str,
    serial: &str,
    profiles: &[DuckyProfile],
    restore_profile: u8,
) -> Result<SdWriteSummary, String> {
    write_duckypad_pro_profiles_via_msc_with_progress(
        disk_label,
        serial,
        profiles,
        restore_profile,
        |_, _, _| {},
    )
}

pub fn write_duckypad_pro_profiles_via_msc_with_progress<F>(
    disk_label: &str,
    serial: &str,
    profiles: &[DuckyProfile],
    restore_profile: u8,
    mut on_progress: F,
) -> Result<SdWriteSummary, String>
where
    F: FnMut(i32, &str, &str),
{
    let serial_opt = if serial.is_empty() || serial == "-" {
        None
    } else {
        Some(serial)
    };
    let label_opt = if disk_label.trim().is_empty() {
        None
    } else {
        Some(disk_label.trim())
    };
    let is_dp20 = disk_label.starts_with("DP20_");
    let target_hw_model = if is_dp20 {
        DP_MODEL_OG_DUCKYPAD
    } else {
        DP_MODEL_DUCKYPAD_PRO
    };
    let model_opt = if is_dp20 {
        Some(DP_MODEL_OG_DUCKYPAD)
    } else if disk_label.starts_with("DP24_") {
        Some(DP_MODEL_DUCKYPAD_PRO)
    } else {
        None
    };

    let device_meta = DuckySdDeviceMeta {
        serial_number: serial.to_string(),
        hw_model: target_hw_model,
        model_name: model_name_for_hw_model(target_hw_model).to_string(),
        firmware_version: String::new(),
        disk_label: disk_label.to_string(),
    };

    if let Some(already_mounted) =
        find_mounted_duckypad_sd_for_device(serial_opt, label_opt, model_opt)
    {
        on_progress(
            30,
            "Step 1/2: Writing profile changes & .dsb bytecode to mounted microSD...",
            &format!("• Found matching microSD at {}", already_mounted.display()),
        );
        let summary = write_profiles_to_sd_root_for_model(
            &already_mounted,
            profiles,
            target_hw_model,
            Some(&device_meta),
        )?;
        sync_profiles_to_device_cache(serial, profiles);
        mark_profiles_sd_synced(serial, profiles);
        record_mounted_sd_space(serial, &already_mounted);
        let short_fp = &summary.validated_fingerprint[..8.min(summary.validated_fingerprint.len())];
        on_progress(
            100,
            "Validated: All changes saved to mounted duckyPad microSD!",
            &format!(
                "✓ Wrote {} updated file(s) ({} unchanged skipped) & verified {} profile(s) [hash #{short_fp}]",
                summary.files_written, summary.files_verified_unchanged, summary.profiles_count
            ),
        );
        return Ok(summary);
    }

    if is_dp20 {
        return write_duckypad_2020_profiles_via_hid_with_progress(
            disk_label,
            serial,
            profiles,
            restore_profile,
            on_progress,
        );
    }

    struct MscGuard;
    impl Drop for MscGuard {
        fn drop(&mut self) {
            MSC_SYNC_IN_PROGRESS.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }
    MSC_SYNC_IN_PROGRESS.store(true, std::sync::atomic::Ordering::SeqCst);
    let _msc_guard = MscGuard;

    on_progress(
        12,
        "Step 1/5: Switching duckyPad Pro to USB Mass Storage mode...",
        &format!("• Sending USB MSC command (0x14) to duckyPad Pro ({disk_label})..."),
    );

    // 1. Reboot duckyPad Pro into USB Mass Storage mode (0x14, tx[3] = 1)
    {
        let _guard = HID_IO_LOCK.lock().ok();
        let dev = open_duckypad_hid_for_serial(serial_opt)?;
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x13;
        tx[2] = HID_CMD_SOFTWARE_RESET;
        tx[3] = 1;
        dev.write(&tx)
            .map_err(|e| format!("Failed to enter USB storage mode: {e}"))?;
    }

    on_progress(
        30,
        "Step 2/5: Waiting for FAT32 partition & mounting microSD...",
        "• Waiting for duckyPad Pro microSD partition to enumerate on USB bus...",
    );

    // 2. Wait for block device and mount
    let mount_res = wait_and_mount_duckypad_msc(disk_label);
    let write_res: Result<SdWriteSummary, String> = match mount_res {
        Ok((mount_root, real_dev)) => {
            let dev_str = real_dev
                .as_ref()
                .map(|d| d.display().to_string())
                .unwrap_or_else(|| disk_label.to_string());
            on_progress(
                52,
                "Step 3/5: Writing changed profile files & compiled .dsb bytecode...",
                &format!("✓ Mounted microSD ({dev_str}) at {}", mount_root.display()),
            );

            let mut res = write_profiles_to_sd_root_for_model(
                &mount_root,
                profiles,
                target_hw_model,
                Some(&device_meta),
            );
            if let Ok(ref summary) = res {
                sync_profiles_to_device_cache(serial, profiles);
                mark_profiles_sd_synced(serial, profiles);
                record_mounted_sd_space(serial, &mount_root);
                let short_fp =
                    &summary.validated_fingerprint[..8.min(summary.validated_fingerprint.len())];
                on_progress(
                    76,
                    "Step 4/5: Read-back validation passed! Flushing FAT buffers & unmounting...",
                    &format!(
                        "✓ Wrote {} updated file(s) ({} unchanged verified) & validated {} profile(s) on microSD [hash #{short_fp}]",
                        summary.files_written,
                        summary.files_verified_unchanged,
                        summary.profiles_count
                    ),
                );
            }

            // 3. Cleanly unmount and verify in /proc/mounts BEFORE resetting back to HID mode
            if let Err(unmount_err) = unmount_duckypad_sd_cleanly(&mount_root, real_dev.as_deref())
            {
                if res.is_ok() {
                    res = Err(unmount_err);
                }
            } else if res.is_ok() {
                on_progress(
                    90,
                    "Step 5/5: Rebooting duckyPad Pro back to normal HID mode...",
                    "✓ FAT32 filesystem flushed, unmounted, and settled; rebooting duckyPad Pro...",
                );
            }
            res
        }
        Err(e) => Err(e),
    };

    // 4. Reset duckyPad Pro back to normal HID keypad mode (0x14, tx[3] = 0)
    for _ in 0..20 {
        let reset_ok = {
            let _guard = HID_IO_LOCK.lock().ok();
            if let Ok(dev) = open_duckypad_hid_for_serial(serial_opt) {
                let mut tx = [0u8; 64];
                tx[0] = HID_REPORT_ID_OUT;
                tx[1] = 0x14;
                tx[2] = HID_CMD_SOFTWARE_RESET;
                tx[3] = 0;
                dev.write(&tx).is_ok()
            } else {
                false
            }
        };
        if reset_ok {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(180));
    }

    // 5. Wait for normal HID mode to re-enumerate and restore active profile
    std::thread::sleep(std::time::Duration::from_millis(1200));
    for _ in 0..15 {
        let ready = {
            let _guard = HID_IO_LOCK.lock().ok();
            open_duckypad_hid_for_serial(serial_opt).is_ok()
        };
        if ready {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(150));
    }
    drop(_msc_guard);
    if restore_profile > 0 && !crate::kwin::auto_switcher::send_hid_switch_profile(restore_profile)
    {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "HID",
            format!("Failed restoring active profile #{restore_profile} after MSC write"),
        );
    }

    if let Ok(ref summary) = write_res {
        let short_fp = &summary.validated_fingerprint[..8.min(summary.validated_fingerprint.len())];
        on_progress(
            100,
            "Validated: All changes successfully saved to duckyPad!",
            &format!(
                "✓ duckyPad Pro reconnected in HID mode (Profile #{restore_profile}) — {} profile(s) verified [hash #{short_fp}]",
                summary.profiles_count
            ),
        );
    }

    write_res
}

pub fn reboot_duckypad() -> Result<(), String> {
    let _guard = HID_IO_LOCK
        .lock()
        .map_err(|_| "HID lock poisoned".to_string())?;
    let dev = open_duckypad_hid_for_serial(None)?;
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x14;
    tx[2] = HID_CMD_SOFTWARE_RESET;
    tx[3] = 0;
    dev.write(&tx)
        .map_err(|e| format!("Failed sending software reset to duckyPad: {e}"))?;
    Ok(())
}
