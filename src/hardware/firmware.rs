use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::hardware::msc::{
    reboot_duckypad, unmount_duckypad_sd_cleanly, wait_and_mount_duckypad_msc,
};
use crate::hardware::profile::find_mounted_duckypad_sd;
use crate::hardware::storage::{
    open_duckypad_hid, HID_CMD_SOFTWARE_RESET, HID_IO_LOCK, HID_REPORT_ID_OUT, MSC_SYNC_IN_PROGRESS,
};

#[derive(Debug, Clone)]
pub struct FirmwareUpdateCheckResult {
    pub is_pro: bool,
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub download_url: String,
    pub guide_url: String,
}

fn parse_semver(v: &str) -> Option<(u32, u32, u32)> {
    let clean = v.trim().trim_start_matches(['v', 'V']);
    let mut parts = clean.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next()?.parse().ok()?;
    let patch: u32 = parts.next()?.parse().ok()?;
    Some((major, minor, patch))
}

pub async fn check_latest_duckypad_firmware(
    is_pro: bool,
    current_version: &str,
) -> Result<FirmwareUpdateCheckResult, String> {
    let (api_url, guide_url) = if is_pro {
        (
            "https://api.github.com/repos/dekuNukem/duckyPad-Pro/contents/firmware?ref=master",
            "https://dekuNukem.github.io/duckyPad-Pro/doc/fw_update.html",
        )
    } else {
        (
            "https://api.github.com/repos/dekuNukem/duckyPad/contents/firmware?ref=master",
            "https://github.com/dekuNukem/duckyPad/blob/master/firmware_updates_and_version_history.md",
        )
    };

    let client = reqwest::Client::builder()
        .user_agent(concat!("DuckyDeck-Studio/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(6))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(api_url)
        .send()
        .await
        .map_err(|e| format!("Failed to reach GitHub firmware repository: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API returned status {}", resp.status()));
    }

    let entries: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Invalid GitHub API response: {e}"))?;

    let arr = entries
        .as_array()
        .ok_or_else(|| "Unexpected GitHub firmware listing format".to_string())?;

    let mut best: Option<((u32, u32, u32), String, String)> = None;

    for item in arr {
        let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
        if item_type != "file" {
            continue;
        }
        let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let dl_url = item
            .get("download_url")
            .and_then(|v| v.as_str())
            .or_else(|| item.get("html_url").and_then(|v| v.as_str()))
            .unwrap_or(guide_url);

        let ver_opt = if is_pro {
            if name.starts_with("DPP_FW_") && name.ends_with(".bin") {
                name.strip_prefix("DPP_FW_")
                    .and_then(|s| s.strip_suffix(".bin"))
                    .and_then(|s| s.split('_').next())
            } else {
                None
            }
        } else if name.starts_with("duckypad_v") && name.ends_with(".dfu") {
            name.strip_prefix("duckypad_v")
                .and_then(|s| s.strip_suffix(".dfu"))
        } else {
            None
        };

        if let Some(ver_str) = ver_opt {
            if let Some(semver) = parse_semver(ver_str) {
                let replace = match &best {
                    Some((best_ver, _, _)) => semver > *best_ver,
                    None => true,
                };
                if replace {
                    best = Some((semver, ver_str.to_string(), dl_url.to_string()));
                }
            }
        }
    }

    let (remote_semver, latest_version, download_url) =
        best.ok_or_else(|| "No firmware image found in repository".to_string())?;

    let update_available = if is_duckydeck_studio_firmware_version(current_version) {
        remote_semver > (3, 3, 5)
    } else {
        match parse_semver(current_version) {
            Some(curr_semver) => remote_semver > curr_semver,
            None => false,
        }
    };

    Ok(FirmwareUpdateCheckResult {
        is_pro,
        current_version: current_version.to_string(),
        latest_version,
        update_available,
        download_url,
        guide_url: guide_url.to_string(),
    })
}

fn default_true_bool() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareVersionEntry {
    pub version: String,
    pub filename: String,
    pub path: String,
    pub crc32: String,
    pub size_bytes: u64,
    pub vendor: String,
    pub is_official: bool,
    pub channel: String,
    pub release_date: String,
    pub notes: String,
    pub is_installed: bool,
    pub supports_monochrome_icons: bool,
    #[serde(default)]
    pub download_url: String,
    #[serde(default = "default_true_bool")]
    pub is_downloaded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRemoteFirmwareItem {
    pub version: String,
    pub filename: String,
    pub crc32: String,
    pub size_bytes: u64,
    pub download_url: String,
    pub release_date: String,
    pub notes: String,
}

pub fn official_firmware_cache_dir() -> PathBuf {
    crate::paths::data_dir().join("firmware_official")
}

pub fn official_firmware_manifest_path() -> PathBuf {
    official_firmware_cache_dir().join("manifest.json")
}

pub fn load_cached_remote_firmwares() -> Vec<CachedRemoteFirmwareItem> {
    let path = official_firmware_manifest_path();
    let Ok(raw) = fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<CachedRemoteFirmwareItem>>(&raw).unwrap_or_default()
}

pub fn save_cached_remote_firmwares(items: &[CachedRemoteFirmwareItem]) {
    let dir = official_firmware_cache_dir();
    if fs::create_dir_all(&dir).is_ok() {
        if let Ok(json) = serde_json::to_string_pretty(items) {
            let path = official_firmware_manifest_path();
            if let Err(e) = fs::write(&path, json) {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "FIRMWARE",
                    format!("Failed writing {}: {e}", path.display()),
                );
            }
        }
    }
}

pub fn is_duckydeck_studio_firmware_version(version: &str) -> bool {
    let trimmed = version.trim();
    if trimmed.is_empty() || trimmed == "-" {
        return false;
    }
    let lower = trimmed.to_ascii_lowercase();
    if lower.contains("official") || lower.contains("dekunukem") {
        return false;
    }
    if lower.contains("studio") || lower.contains("dds") {
        return true;
    }
    let clean = trimmed.trim_start_matches(['v', 'V']);
    clean == "1.0.0" || clean == "3.4.0"
}

pub fn compute_ieee_crc32(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

fn firmware_release_metadata(
    version: &str,
    is_studio_custom: bool,
) -> (&'static str, &'static str, &'static str) {
    if is_studio_custom {
        return (
            "DuckyDeck Studio Custom (v1.0.0)",
            "Oct 17, 2026",
            "DuckyDeck Studio custom firmware v1.0.0 (built on duckyPad Pro 3.3.5 / DSVM v2): adds native 1-bit monochrome pixel-art OLED icons (16×16 full-cell, 10×10 top-icon + caption, and 7×10 / 6×8 inline glyphs), 0°/90°/180°/270° OLED orientation, UTF-8 key label decoding, and DuckyDeck Studio HID identification.",
        );
    }
    match version {
        "3.3.5" => (
            "Official Release",
            "Mar 15, 2026",
            "Official dekuNukem release: increased Anykey-to-Abort check frequency during script execution.",
        ),
        "3.3.4" => (
            "Official Release",
            "Mar 11, 2026",
            "Official dekuNukem release: adjusted PWM default level to reduce power consumption.",
        ),
        "3.3.3" => (
            "Official Release",
            "Feb 9, 2026",
            "Official dekuNukem release: disabled PWM output; stable public release.",
        ),
        "3.3.0" => (
            "Official Release",
            "Jan 2026",
            "Official dekuNukem release: DSVM v2 bytecode engine and expanded HID command support.",
        ),
        _ => (
            "Official dekuNukem Firmware",
            "Official Release",
            "Official dekuNukem duckyPad Pro ESP32-S3 OTA firmware binary from dekuNukem's repository.",
        ),
    }
}

pub fn list_available_duckypad_pro_firmwares(current_version: &str) -> Vec<FirmwareVersionEntry> {
    let mut candidate_dirs: Vec<(PathBuf, bool)> = Vec::new();
    for sys_fw_dir in [
        "/app/share/duckydeck-studio/firmware",
        "/usr/local/share/duckydeck-studio/firmware",
        "/usr/share/duckydeck-studio/firmware",
    ] {
        candidate_dirs.push((PathBuf::from(sys_fw_dir), true));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(bin_dir) = exe.parent() {
            candidate_dirs.push((bin_dir.join("../share/duckydeck-studio/firmware"), true));
        }
    }
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    candidate_dirs.push((manifest_dir.join("firmware/releases"), true));
    candidate_dirs.push((manifest_dir.join("firmware/duckyPad-Pro/firmware"), true));
    candidate_dirs.push((official_firmware_cache_dir(), false));
    candidate_dirs.push((crate::paths::data_dir().join("firmware"), false));

    let mut by_key: std::collections::BTreeMap<(u8, u32, u32, u32, String), FirmwareVersionEntry> =
        std::collections::BTreeMap::new();

    for (dir, is_bundled_studio_dir) in candidate_dirs {
        let Ok(rd) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(fname) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !fname.starts_with("DPP_FW_") || !fname.ends_with(".bin") {
                continue;
            }
            let inner = &fname["DPP_FW_".len()..fname.len() - ".bin".len()];
            let mut parts = inner.split('_');
            let Some(ver_str) = parts.next() else {
                continue;
            };
            let crc_str = parts.next().unwrap_or("").to_lowercase();
            let Some((maj, min, pat)) = parse_semver(ver_str) else {
                continue;
            };
            if ver_str == "3.4.0" {
                continue;
            }
            let size_bytes = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let is_studio_custom = is_bundled_studio_dir || matches!(crc_str.as_str(), "ad2acc81");
            let is_official = !is_studio_custom;
            let (channel, release_date, notes) =
                firmware_release_metadata(ver_str, is_studio_custom);
            let vendor = if is_official {
                "dekuNukem (Official)".to_string()
            } else {
                "DuckyDeck Studio".to_string()
            };
            let supports_monochrome_icons = is_studio_custom;
            let is_installed = if is_studio_custom {
                is_duckydeck_studio_firmware_version(current_version)
                    && (current_version.trim().trim_start_matches(['v', 'V']) == ver_str
                        || current_version.trim() == "3.4.0")
            } else {
                !is_duckydeck_studio_firmware_version(current_version)
                    && current_version.trim().starts_with(ver_str)
            };

            let studio_rank = if is_studio_custom { 1u8 } else { 0u8 };
            let key = (studio_rank, maj, min, pat, crc_str.clone());
            by_key.entry(key).or_insert_with(|| FirmwareVersionEntry {
                version: ver_str.to_string(),
                filename: fname.to_string(),
                path: path.to_string_lossy().to_string(),
                crc32: crc_str,
                size_bytes,
                vendor,
                is_official,
                channel: channel.to_string(),
                release_date: release_date.to_string(),
                notes: notes.to_string(),
                is_installed,
                supports_monochrome_icons,
                download_url: String::new(),
                is_downloaded: true,
            });
        }
    }

    // Merge remote official firmwares fetched from dekuNukem's site
    let official_dir = official_firmware_cache_dir();
    for remote in load_cached_remote_firmwares() {
        let Some((maj, min, pat)) = parse_semver(&remote.version) else {
            continue;
        };
        let crc_lower = remote.crc32.to_lowercase();
        let key = (0u8, maj, min, pat, crc_lower.clone());
        let local_path = official_dir.join(&remote.filename);
        let is_downloaded = local_path.is_file();
        let size_bytes = if is_downloaded {
            local_path
                .metadata()
                .map(|m| m.len())
                .unwrap_or(remote.size_bytes)
        } else {
            remote.size_bytes
        };
        let (default_channel, default_date, default_notes) =
            firmware_release_metadata(&remote.version, false);
        let is_installed = !is_duckydeck_studio_firmware_version(current_version)
            && current_version.trim().starts_with(&remote.version);

        by_key
            .entry(key)
            .and_modify(|existing| {
                if existing.download_url.is_empty() && !remote.download_url.is_empty() {
                    existing.download_url = remote.download_url.clone();
                }
            })
            .or_insert_with(|| FirmwareVersionEntry {
                version: remote.version.clone(),
                filename: remote.filename.clone(),
                path: if is_downloaded {
                    local_path.to_string_lossy().to_string()
                } else {
                    String::new()
                },
                crc32: crc_lower,
                size_bytes,
                vendor: "dekuNukem (Official)".to_string(),
                is_official: true,
                channel: default_channel.to_string(),
                release_date: if remote.release_date.trim().is_empty() {
                    default_date.to_string()
                } else {
                    remote.release_date.clone()
                },
                notes: if remote.notes.trim().is_empty() {
                    default_notes.to_string()
                } else {
                    remote.notes.clone()
                },
                is_installed,
                supports_monochrome_icons: false,
                download_url: remote.download_url.clone(),
                is_downloaded,
            });
    }

    let mut out: Vec<FirmwareVersionEntry> = by_key.into_values().collect();
    out.reverse();
    out
}

pub async fn fetch_official_firmwares_from_site(
    current_version: &str,
) -> Result<Vec<FirmwareVersionEntry>, String> {
    let client = reqwest::Client::builder()
        .user_agent(concat!("DuckyDeck-Studio/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("HTTP client init failed: {e}"))?;

    let mut discovered: std::collections::BTreeMap<
        (u32, u32, u32, String),
        CachedRemoteFirmwareItem,
    > = std::collections::BTreeMap::new();

    // Seed with existing cached entries so we never lose previously discovered items
    for item in load_cached_remote_firmwares() {
        if let Some((maj, min, pat)) = parse_semver(&item.version) {
            discovered.insert((maj, min, pat, item.crc32.to_lowercase()), item);
        }
    }

    let mut parse_and_insert =
        |fname: &str, dl_url: &str, size_bytes: u64, date_hint: &str, note_hint: &str| {
            if !fname.starts_with("DPP_FW_") || !fname.ends_with(".bin") {
                return;
            }
            let inner = &fname["DPP_FW_".len()..fname.len() - ".bin".len()];
            let mut parts = inner.split('_');
            let Some(ver_str) = parts.next() else {
                return;
            };
            let crc_str = parts.next().unwrap_or("").to_lowercase();
            if crc_str.is_empty() || crc_str == "ad2acc81" {
                return;
            }
            let Some((maj, min, pat)) = parse_semver(ver_str) else {
                return;
            };
            let (_, meta_date, meta_notes) = firmware_release_metadata(ver_str, false);
            let release_date = if !date_hint.trim().is_empty() {
                date_hint.trim().to_string()
            } else {
                meta_date.to_string()
            };
            let notes = if !note_hint.trim().is_empty() {
                note_hint.trim().to_string()
            } else {
                meta_notes.to_string()
            };
            discovered.insert(
                (maj, min, pat, crc_str.clone()),
                CachedRemoteFirmwareItem {
                    version: ver_str.to_string(),
                    filename: fname.to_string(),
                    crc32: crc_str,
                    size_bytes,
                    download_url: dl_url.to_string(),
                    release_date,
                    notes,
                },
            );
        };

    // 1. Fetch active firmware directory from dekuNukem/duckyPad-Pro
    let contents_url =
        "https://api.github.com/repos/dekuNukem/duckyPad-Pro/contents/firmware?ref=master";
    let resp = client
        .get(contents_url)
        .send()
        .await
        .map_err(|e| format!("Failed reaching dekuNukem firmware repository: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Official firmware repository returned HTTP {}",
            resp.status()
        ));
    }

    if let Ok(entries) = resp.json::<serde_json::Value>().await {
        if let Some(arr) = entries.as_array() {
            for item in arr {
                let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
                if item_type != "file" {
                    continue;
                }
                let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let size = item.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
                let dl_url = item
                    .get("download_url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if !name.is_empty() && !dl_url.is_empty() {
                    parse_and_insert(name, dl_url, size, "", "");
                }
            }
        }
    }

    // 2. Also check GitHub Releases for any attached DPP_FW_*.bin assets
    let releases_url = "https://api.github.com/repos/dekuNukem/duckyPad-Pro/releases?per_page=20";
    if let Ok(rel_resp) = client.get(releases_url).send().await {
        if rel_resp.status().is_success() {
            if let Ok(rel_json) = rel_resp.json::<serde_json::Value>().await {
                if let Some(rel_arr) = rel_json.as_array() {
                    for rel in rel_arr {
                        let published = rel
                            .get("published_at")
                            .and_then(|v| v.as_str())
                            .and_then(|s| s.get(..10))
                            .unwrap_or("");
                        let body = rel
                            .get("body")
                            .and_then(|v| v.as_str())
                            .map(|s| s.lines().next().unwrap_or("").trim())
                            .unwrap_or("");
                        if let Some(assets) = rel.get("assets").and_then(|v| v.as_array()) {
                            for asset in assets {
                                let name = asset.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                let size = asset.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
                                let dl_url = asset
                                    .get("browser_download_url")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("");
                                if !name.is_empty() && !dl_url.is_empty() {
                                    parse_and_insert(name, dl_url, size, published, body);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Also inspect recent commits to `/firmware` in dekuNukem/duckyPad-Pro so historical versions in git history are listed
    let commits_url =
        "https://api.github.com/repos/dekuNukem/duckyPad-Pro/commits?path=firmware&per_page=10";
    if let Ok(com_resp) = client.get(commits_url).send().await {
        if com_resp.status().is_success() {
            if let Ok(com_json) = com_resp.json::<serde_json::Value>().await {
                if let Some(com_arr) = com_json.as_array() {
                    for com in com_arr.iter().take(6) {
                        let sha = com.get("sha").and_then(|v| v.as_str()).unwrap_or("");
                        if sha.is_empty() {
                            continue;
                        }
                        let commit_date = com
                            .get("commit")
                            .and_then(|c| c.get("committer"))
                            .and_then(|cm| cm.get("date"))
                            .and_then(|d| d.as_str())
                            .and_then(|s| s.get(..10))
                            .unwrap_or("");
                        let commit_msg = com
                            .get("commit")
                            .and_then(|c| c.get("message"))
                            .and_then(|m| m.as_str())
                            .map(|s| s.lines().next().unwrap_or("").trim())
                            .unwrap_or("");
                        let tree_url = format!(
                            "https://api.github.com/repos/dekuNukem/duckyPad-Pro/contents/firmware?ref={sha}"
                        );
                        if let Ok(t_resp) = client.get(&tree_url).send().await {
                            if t_resp.status().is_success() {
                                if let Ok(t_json) = t_resp.json::<serde_json::Value>().await {
                                    if let Some(t_arr) = t_json.as_array() {
                                        for item in t_arr {
                                            let name = item
                                                .get("name")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("");
                                            let size = item
                                                .get("size")
                                                .and_then(|v| v.as_u64())
                                                .unwrap_or(0);
                                            let dl_url = item
                                                .get("download_url")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("");
                                            if !name.is_empty() && !dl_url.is_empty() {
                                                let note = if !commit_msg.is_empty() {
                                                    format!(
                                                        "Official dekuNukem release ({commit_msg})"
                                                    )
                                                } else {
                                                    String::new()
                                                };
                                                parse_and_insert(
                                                    name,
                                                    dl_url,
                                                    size,
                                                    commit_date,
                                                    &note,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let remote_list: Vec<CachedRemoteFirmwareItem> = discovered.into_values().collect();
    save_cached_remote_firmwares(&remote_list);

    Ok(list_available_duckypad_pro_firmwares(current_version))
}

pub async fn download_official_firmware_binary(
    filename: &str,
    download_url: &str,
) -> Result<PathBuf, String> {
    let clean_name = filename.trim();
    if !clean_name.starts_with("DPP_FW_")
        || !clean_name.ends_with(".bin")
        || clean_name.contains('/')
        || clean_name.contains('\\')
    {
        return Err(format!(
            "Invalid official firmware filename: '{clean_name}'"
        ));
    }
    let dir = official_firmware_cache_dir();
    fs::create_dir_all(&dir).map_err(|e| {
        format!(
            "Failed creating firmware cache directory {}: {e}",
            dir.display()
        )
    })?;
    let target_path = dir.join(clean_name);

    // Extract expected CRC32 from filename `DPP_FW_<ver>_<crc32>.bin`
    let inner = &clean_name["DPP_FW_".len()..clean_name.len() - ".bin".len()];
    let expected_crc_hex = inner.split('_').nth(1).unwrap_or("").to_lowercase();

    // If already downloaded and CRC matches, return immediately
    if target_path.is_file() {
        if let Ok(existing_bytes) = fs::read(&target_path) {
            if existing_bytes.len() > 64 * 1024 {
                let crc = format!("{:08x}", compute_ieee_crc32(&existing_bytes));
                if expected_crc_hex.is_empty() || crc == expected_crc_hex {
                    return Ok(target_path);
                }
            }
        }
    }

    let url = if !download_url.trim().is_empty() {
        download_url.trim().to_string()
    } else {
        format!(
            "https://raw.githubusercontent.com/dekuNukem/duckyPad-Pro/master/firmware/{clean_name}"
        )
    };

    let client = reqwest::Client::builder()
        .user_agent(concat!("DuckyDeck-Studio/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed downloading {clean_name} from {url}: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "Download of {clean_name} failed with HTTP {}",
            resp.status()
        ));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed reading firmware bytes: {e}"))?;

    if bytes.len() < 64 * 1024 {
        return Err(format!(
            "Downloaded firmware file is unexpectedly small ({} bytes)",
            bytes.len()
        ));
    }

    let actual_crc = format!("{:08x}", compute_ieee_crc32(&bytes));
    if !expected_crc_hex.is_empty() && actual_crc != expected_crc_hex {
        return Err(format!(
            "CRC32 verification failed for {clean_name}: expected {expected_crc_hex}, got {actual_crc}"
        ));
    }

    fs::write(&target_path, &bytes)
        .map_err(|e| format!("Failed saving firmware to {}: {e}", target_path.display()))?;

    Ok(target_path)
}

fn write_and_verify_firmware_on_sd(
    sd_root: &Path,
    target_filename: &str,
    fw_bytes: &[u8],
    expected_crc: u32,
) -> Result<PathBuf, String> {
    if let Ok(rd) = fs::read_dir(sd_root) {
        for entry in rd.flatten() {
            let p = entry.path();
            if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("DPP_FW_") && name.ends_with(".bin") {
                    if let Err(e) = fs::remove_file(&p) {
                        crate::bridge::device_controller::record_system_event(
                            "WARN",
                            "FIRMWARE",
                            format!("Failed removing old firmware {}: {e}", p.display()),
                        );
                    }
                }
            }
        }
    }

    let dest_path = sd_root.join(target_filename);
    {
        let mut f = fs::File::create(&dest_path).map_err(|e| {
            format!(
                "Failed creating firmware file {} on microSD: {e}",
                dest_path.display()
            )
        })?;
        f.write_all(fw_bytes).map_err(|e| {
            format!(
                "Failed writing firmware bytes to {}: {e}",
                dest_path.display()
            )
        })?;
        f.sync_all()
            .map_err(|e| format!("Failed flushing firmware file to microSD: {e}"))?;
    }

    let read_back = fs::read(&dest_path).map_err(|e| {
        format!(
            "Failed reading back staged firmware {} for verification: {e}",
            dest_path.display()
        )
    })?;
    if read_back.len() != fw_bytes.len() {
        return Err(format!(
            "Staged firmware size mismatch on microSD (expected {} bytes, got {} bytes)",
            fw_bytes.len(),
            read_back.len()
        ));
    }
    let read_crc = compute_ieee_crc32(&read_back);
    if read_crc != expected_crc {
        return Err(format!(
            "Staged firmware CRC32 mismatch on microSD (expected {:08x}, got {:08x})",
            expected_crc, read_crc
        ));
    }

    Ok(dest_path)
}

pub fn stage_duckypad_pro_firmware_via_msc_with_progress<F>(
    disk_label: &str,
    fw_source_path: &Path,
    mut on_progress: F,
) -> Result<String, String>
where
    F: FnMut(i32, &str, &str),
{
    let clean_str = fw_source_path
        .to_string_lossy()
        .trim()
        .trim_start_matches("file://")
        .to_string();
    let src_path = PathBuf::from(&clean_str);
    if !src_path.is_file() {
        return Err(format!(
            "Firmware binary file not found: {}",
            src_path.display()
        ));
    }

    let fw_bytes = fs::read(&src_path)
        .map_err(|e| format!("Failed reading firmware binary {}: {e}", src_path.display()))?;
    if fw_bytes.len() < 64 * 1024 {
        return Err(format!(
            "File {} is too small ({} bytes) to be a valid duckyPad Pro firmware image",
            src_path.display(),
            fw_bytes.len()
        ));
    }

    let crc = compute_ieee_crc32(&fw_bytes);
    let crc_hex = format!("{crc:x}");
    let raw_name = src_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("DPP_FW_custom.bin");

    let (ver_label, target_filename) = if raw_name.starts_with("DPP_FW_")
        && raw_name.ends_with(".bin")
        && raw_name.to_lowercase().contains(&crc_hex)
    {
        let ver = raw_name
            .strip_prefix("DPP_FW_")
            .and_then(|s| s.strip_suffix(".bin"))
            .and_then(|s| s.split('_').next())
            .unwrap_or("custom")
            .to_string();
        (ver, raw_name.to_string())
    } else {
        let ver = raw_name
            .strip_prefix("DPP_FW_")
            .and_then(|s| s.strip_suffix(".bin"))
            .and_then(|s| s.split('_').next())
            .unwrap_or("3.4.0")
            .to_string();
        (ver.clone(), format!("DPP_FW_{ver}_{crc_hex}.bin"))
    };

    on_progress(
        10,
        &format!("Validating duckyPad Pro firmware v{ver_label} (CRC32: {crc_hex})..."),
        &format!(
            "• Verified {} ({} KB, IEEE CRC32: 0x{crc_hex})\n• Target OTA image name: {target_filename}",
            src_path.display(),
            fw_bytes.len() / 1024
        ),
    );

    if let Some(already_mounted) = find_mounted_duckypad_sd() {
        on_progress(
            45,
            &format!("Staging {target_filename} to mounted microSD card..."),
            &format!(
                "• Found mounted microSD at {}\n• Removing old DPP_FW_*.bin and writing {target_filename}...",
                already_mounted.display()
            ),
        );
        let staged_path =
            write_and_verify_firmware_on_sd(&already_mounted, &target_filename, &fw_bytes, crc)?;
        if let Err(e) = crate::hardware::msc::run_storage_cmd("sync", &[]) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "FIRMWARE",
                format!("Failed running sync after staging firmware: {e}"),
            );
        }
        let rebooted = reboot_duckypad().is_ok();
        let msg = if rebooted {
            format!(
                "Staged {target_filename} ({}) and rebooted duckyPad Pro — press ANY KEY on your duckyPad Pro to complete the v{ver_label} update!",
                staged_path.display()
            )
        } else {
            format!(
                "Staged {target_filename} to {} — safely eject/insert the microSD into your duckyPad Pro and press ANY KEY on the keypad to install v{ver_label}!",
                staged_path.display()
            )
        };
        on_progress(100, &msg, &format!("✓ {msg}"));
        return Ok(msg);
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
        20,
        "Step 1/5: Switching duckyPad Pro to USB Mass Storage mode...",
        &format!("• Sending USB MSC command (0x14) to duckyPad Pro ({disk_label})..."),
    );

    {
        let _guard = HID_IO_LOCK.lock().ok();
        let dev = open_duckypad_hid()?;
        let mut tx = [0u8; 64];
        tx[0] = HID_REPORT_ID_OUT;
        tx[1] = 0x13;
        tx[2] = HID_CMD_SOFTWARE_RESET;
        tx[3] = 1;
        dev.write(&tx)
            .map_err(|e| format!("Failed to switch duckyPad Pro into USB storage mode: {e}"))?;
    }

    on_progress(
        38,
        "Step 2/5: Waiting for FAT32 partition & mounting microSD...",
        "• Waiting for duckyPad Pro microSD partition to enumerate on USB bus...",
    );

    let mount_res = wait_and_mount_duckypad_msc(disk_label);
    let stage_res: Result<PathBuf, String> = match mount_res {
        Ok((mount_root, real_dev)) => {
            on_progress(
                60,
                &format!("Step 3/5: Writing {target_filename} & verifying CRC32 on microSD..."),
                &format!(
                    "✓ Mounted microSD at {}\n• Writing {} bytes and verifying read-back CRC32 (0x{crc_hex})...",
                    mount_root.display(),
                    fw_bytes.len()
                ),
            );
            let mut res =
                write_and_verify_firmware_on_sd(&mount_root, &target_filename, &fw_bytes, crc);
            if res.is_ok() {
                on_progress(
                    80,
                    "Step 4/5: CRC32 verified! Flushing FAT32 buffers & unmounting...",
                    &format!("✓ Read-back CRC32 (0x{crc_hex}) verified on microSD; unmounting cleanly..."),
                );
            }
            if let Err(unmount_err) = unmount_duckypad_sd_cleanly(&mount_root, real_dev.as_deref())
            {
                if res.is_ok() {
                    res = Err(unmount_err);
                }
            } else if res.is_ok() {
                on_progress(
                    92,
                    "Step 5/5: Rebooting duckyPad Pro into OTA Firmware Updater...",
                    "✓ Unmounted cleanly; rebooting duckyPad Pro into OTA update prompt...",
                );
            }
            res
        }
        Err(e) => Err(e),
    };

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

    drop(_msc_guard);

    match stage_res {
        Ok(_) => {
            let msg = format!(
                "Firmware v{ver_label} ({target_filename}) staged to duckyPad Pro! Press ANY KEY on your duckyPad Pro to confirm and install v{ver_label}."
            );
            on_progress(
                100,
                &format!(
                    "Ready on duckyPad Pro: Press ANY KEY on the device to install v{ver_label}!"
                ),
                &format!("✓ {msg}"),
            );
            Ok(msg)
        }
        Err(e) => Err(e),
    }
}
