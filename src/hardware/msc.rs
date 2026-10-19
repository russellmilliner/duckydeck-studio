use std::fs;
use std::path::{Path, PathBuf};

use crate::hardware::profile::{
    compute_profiles_fingerprint, find_mounted_duckypad_sd, load_all_sd_profiles,
    mark_profiles_sd_synced, record_mounted_sd_space, sync_profiles_to_device_cache,
    write_profiles_to_sd_root, DuckyProfile, SdWriteSummary,
};
use crate::hardware::storage::{
    open_duckypad_hid, HID_CMD_SOFTWARE_RESET, HID_IO_LOCK, HID_REPORT_ID_OUT, MSC_SYNC_IN_PROGRESS,
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
    if !disk_label.is_empty() {
        let by_label = PathBuf::from(format!("/dev/disk/by-label/{disk_label}"));
        if by_label.exists() {
            if let Ok(canon) = fs::canonicalize(&by_label) {
                return Some(canon);
            }
        }
    }

    if let Ok(entries) = fs::read_dir("/dev/disk/by-label") {
        for entry in entries.flatten() {
            let label = entry.file_name().to_string_lossy().to_string();
            if label.starts_with("DP24_")
                || label.starts_with("DP20_")
                || label.starts_with("DUCKYPAD")
            {
                if let Ok(canon) = fs::canonicalize(entry.path()) {
                    return Some(canon);
                }
            }
        }
    }

    if let Ok(entries) = fs::read_dir("/dev/disk/by-id") {
        let mut whole_disk_fallback: Option<PathBuf> = None;
        for entry in entries.flatten() {
            let id_name = entry.file_name().to_string_lossy().to_string();
            if id_name.contains("TinyUSB_Flash_Storage") || id_name.contains("duckyPad") {
                if let Ok(canon) = fs::canonicalize(entry.path()) {
                    if id_name.contains("-part") {
                        return Some(canon);
                    }
                    // Check if kernel has already created the first partition node (e.g. /dev/sdh1)
                    let part1 = PathBuf::from(format!("{}1", canon.display()));
                    if part1.exists() {
                        return Some(part1);
                    }
                    whole_disk_fallback = Some(canon);
                }
            }
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
    let mut last_udisks_err = String::new();
    let mut settled_dev_once = false;

    for attempt in 0..48 {
        if let Some(mounted) = find_mounted_duckypad_sd() {
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
                if let Some(mounted) = find_mounted_duckypad_sd() {
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
            if let Some(mounted) = find_mounted_duckypad_sd() {
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

pub(crate) fn unmount_duckypad_sd_cleanly(
    mount_root: &Path,
    known_dev: Option<&Path>,
) -> Result<(), String> {
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
        if let Ok(dev) = open_duckypad_hid() {
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
    if find_mounted_duckypad_sd().is_some() {
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
    if let Some(mounted) = find_mounted_duckypad_sd() {
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
        if let Some(mounted) = find_mounted_duckypad_sd() {
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

    // If a duckyPad Pro is connected over USB HID, switch it into USB MSC mode and mount
    let entered_msc = {
        let _guard = HID_IO_LOCK.lock().ok();
        if let Ok(device) = open_duckypad_hid() {
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
    if let Some(already_mounted) = find_mounted_duckypad_sd() {
        on_progress(
            35,
            "Reading profiles from mounted duckyPad microSD...",
            &format!("• Found mounted microSD at {}", already_mounted.display()),
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
            let dev = open_duckypad_hid()?;
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
            if let Ok(dev) = open_duckypad_hid() {
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
            open_duckypad_hid().is_ok()
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
    if let Some(already_mounted) = find_mounted_duckypad_sd() {
        on_progress(
            30,
            "Step 1/2: Writing profile changes & .dsb bytecode to mounted microSD...",
            &format!("• Found mounted microSD at {}", already_mounted.display()),
        );
        let summary = write_profiles_to_sd_root(&already_mounted, profiles)?;
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
        let dev = open_duckypad_hid()?;
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

            let mut res = write_profiles_to_sd_root(&mount_root, profiles);
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
            if let Ok(dev) = open_duckypad_hid() {
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
            open_duckypad_hid().is_ok()
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
    let dev = open_duckypad_hid()?;
    let mut tx = [0u8; 64];
    tx[0] = HID_REPORT_ID_OUT;
    tx[1] = 0x14;
    tx[2] = HID_CMD_SOFTWARE_RESET;
    tx[3] = 0;
    dev.write(&tx)
        .map_err(|e| format!("Failed sending software reset to duckyPad: {e}"))?;
    Ok(())
}
