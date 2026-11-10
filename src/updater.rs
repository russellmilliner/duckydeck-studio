use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const GITHUB_RELEASES_LATEST_API: &str =
    "https://api.github.com/repos/russellmilliner/duckydeck-studio/releases/latest";
pub const GITHUB_RELEASES_HTML_URL: &str =
    "https://github.com/russellmilliner/duckydeck-studio/releases/latest";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppUpdateCheckResult {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub release_url: String,
    pub download_url: String,
    pub release_notes: String,
}

pub fn parse_semver_tuple(v: &str) -> Option<(u32, u32, u32)> {
    let clean = v
        .trim()
        .trim_start_matches(['v', 'V'])
        .split(['-', '+'])
        .next()?
        .trim();
    let mut parts = clean.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next().unwrap_or("0").parse().ok()?;
    let patch: u32 = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}

pub fn is_newer_version(current: &str, latest: &str) -> bool {
    match (parse_semver_tuple(current), parse_semver_tuple(latest)) {
        (Some(curr), Some(lat)) => lat > curr,
        _ => false,
    }
}

pub fn select_platform_download_url_for_os(
    assets: &[serde_json::Value],
    is_windows: bool,
    prefer_flatpak: bool,
) -> Option<String> {
    let mut flatpak_url: Option<String> = None;
    let mut linux_tar_url: Option<String> = None;
    let mut linux_bin_url: Option<String> = None;
    let mut win_exe_url: Option<String> = None;
    let mut win_zip_url: Option<String> = None;

    for asset in assets {
        let name = asset.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let dl = asset
            .get("browser_download_url")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if name.is_empty() || dl.is_empty() || name.ends_with(".sha256") {
            continue;
        }
        let lower = name.to_ascii_lowercase();
        if lower.ends_with(".exe") {
            win_exe_url = Some(dl.to_string());
        } else if lower.ends_with(".zip") && lower.contains("windows") {
            win_zip_url = Some(dl.to_string());
        } else if lower.ends_with(".flatpak") {
            flatpak_url = Some(dl.to_string());
        } else if lower.ends_with(".tar.gz") && lower.contains("linux") {
            linux_tar_url = Some(dl.to_string());
        } else if lower == "duckydeck-studio" || lower.starts_with("duckydeck-studio-v") {
            linux_bin_url = Some(dl.to_string());
        }
    }

    if is_windows {
        win_exe_url.or(win_zip_url)
    } else if prefer_flatpak {
        flatpak_url.or(linux_tar_url).or(linux_bin_url)
    } else {
        linux_tar_url.or(flatpak_url).or(linux_bin_url)
    }
}

pub fn parse_github_release_json(
    raw_json: &str,
    current_version: &str,
    is_windows: bool,
    prefer_flatpak: bool,
) -> Result<AppUpdateCheckResult, String> {
    let parsed: serde_json::Value =
        serde_json::from_str(raw_json).map_err(|e| format!("Invalid GitHub JSON: {e}"))?;

    if let Some(msg) = parsed.get("message").and_then(|v| v.as_str()) {
        if parsed.get("tag_name").is_none() {
            return Err(format!("GitHub API error: {msg}"));
        }
    }

    let tag_name = parsed
        .get("tag_name")
        .and_then(|v| v.as_str())
        .or_else(|| parsed.get("name").and_then(|v| v.as_str()))
        .ok_or_else(|| "GitHub release response missing tag_name".to_string())?;

    let latest_clean = tag_name.trim().trim_start_matches(['v', 'V']).to_string();
    let curr_clean = current_version
        .trim()
        .trim_start_matches(['v', 'V'])
        .to_string();

    let release_url = parsed
        .get("html_url")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or(GITHUB_RELEASES_HTML_URL)
        .to_string();

    let release_notes = parsed
        .get("body")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    let assets = parsed
        .get("assets")
        .and_then(|v| v.as_array())
        .map(|a| a.as_slice())
        .unwrap_or(&[]);

    let download_url =
        select_platform_download_url_for_os(assets, is_windows, prefer_flatpak).unwrap_or_default();

    let update_available = is_newer_version(&curr_clean, &latest_clean);

    Ok(AppUpdateCheckResult {
        current_version: curr_clean,
        latest_version: latest_clean,
        update_available,
        release_url,
        download_url,
        release_notes,
    })
}

fn should_prefer_flatpak_on_linux() -> bool {
    if Path::new("/.flatpak-info").exists() {
        return true;
    }
    if let Ok(exe) = std::env::current_exe() {
        let exe_str = exe.to_string_lossy();
        if exe_str.contains("/.local/bin/") || exe_str.contains("/target/") {
            return false;
        }
    }
    Command::new("flatpak")
        .args(["info", "io.github.duckydeck.Studio"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub async fn check_for_app_updates_async(
    current_version: &str,
) -> Result<AppUpdateCheckResult, String> {
    let client = reqwest::Client::builder()
        .user_agent(format!(
            "DuckyDeck-Studio/{}",
            current_version.trim_start_matches(['v', 'V'])
        ))
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(GITHUB_RELEASES_LATEST_API)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("Failed to reach GitHub Releases API: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API returned status {}", resp.status()));
    }

    let body = resp
        .text()
        .await
        .map_err(|e| format!("Failed reading GitHub API response: {e}"))?;

    parse_github_release_json(
        &body,
        current_version,
        cfg!(target_os = "windows"),
        should_prefer_flatpak_on_linux(),
    )
}

pub fn check_for_app_updates_blocking(
    current_version: &str,
) -> Result<AppUpdateCheckResult, String> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| format!("Failed creating async runtime: {e}"))?;
    rt.block_on(check_for_app_updates_async(current_version))
}

fn is_system_flatpak_install() -> bool {
    if let Ok(info) = fs::read_to_string("/.flatpak-info") {
        if info.contains("/var/lib/flatpak/") {
            return true;
        }
        if info.contains(".local/share/flatpak/") {
            return false;
        }
    }
    false
}

#[cfg(not(target_os = "windows"))]
fn run_flatpak_bundle_install(pkg_str: &str, scope_flag: &str) -> bool {
    let status = if Path::new("/.flatpak-info").exists() {
        Command::new("flatpak-spawn")
            .args([
                "--host",
                "flatpak",
                "install",
                scope_flag,
                "-y",
                "--reinstall",
                pkg_str,
            ])
            .status()
    } else {
        Command::new("flatpak")
            .args(["install", scope_flag, "-y", "--reinstall", pkg_str])
            .status()
    };
    matches!(status, Ok(st) if st.success())
}

pub fn update_staging_dir(latest_version: &str) -> PathBuf {
    let ver_clean = latest_version.trim_start_matches(['v', 'V']);
    #[cfg(not(target_os = "windows"))]
    {
        // Inside a Flatpak sandbox, `/tmp` (`std::env::temp_dir()`) is a private tmpfs invisible
        // to `flatpak-spawn --host flatpak install` and host `xdg-open` / `kioclient`.
        // Stage updates inside `~/.local/share/duckydeck-studio/updates/`, which is shared with
        // the host via `--filesystem=xdg-data/duckydeck-studio:create`.
        let updates_root = crate::paths::host_data_dir().join("updates");
        if let Ok(entries) = fs::read_dir(&updates_root) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    let _ = fs::remove_dir_all(&p);
                }
            }
        }
        updates_root.join(format!("duckydeck-studio-update-{ver_clean}"))
    }
    #[cfg(target_os = "windows")]
    {
        std::env::temp_dir().join(format!("duckydeck-studio-update-{ver_clean}"))
    }
}

/// Parses a `.sha256` manifest (single-line or multi-line `sha256sum` output) and verifies
/// that the SHA-256 digest of `asset_bytes` matches the expected 64-character hex digest.
pub fn verify_sha256_manifest_text(
    asset_bytes: &[u8],
    asset_filename: &str,
    sha256_text: &str,
) -> Result<String, String> {
    let actual_hex = crate::paths::sha256_hex(asset_bytes);
    let clean_name = asset_filename.trim();
    let mut matched_expected: Option<String> = None;
    let mut single_line_candidate: Option<String> = None;

    for raw_line in sha256_text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(first) = parts.next() else {
            continue;
        };
        let is_hex64 = first.len() == 64 && first.bytes().all(|b| b.is_ascii_hexdigit());
        if !is_hex64 {
            continue;
        }
        let hex_lower = first.to_ascii_lowercase();
        if let Some(second) = parts.next() {
            let entry_file = second
                .trim_start_matches('*')
                .rsplit(['/', '\\'])
                .next()
                .unwrap_or(second);
            if entry_file.eq_ignore_ascii_case(clean_name) {
                matched_expected = Some(hex_lower);
                break;
            }
            if single_line_candidate.is_none() {
                single_line_candidate = Some(hex_lower);
            }
        } else if single_line_candidate.is_none() {
            single_line_candidate = Some(hex_lower);
        }
    }

    let expected_hex = matched_expected
        .or(single_line_candidate)
        .ok_or_else(|| "Could not parse a valid 64-character SHA-256 digest from .sha256 file".to_string())?;

    if !crate::paths::constant_time_str_eq(&actual_hex, &expected_hex) {
        return Err(format!(
            "SHA-256 checksum mismatch for {clean_name}: expected {expected_hex}, computed {actual_hex}"
        ));
    }
    Ok(actual_hex)
}

pub async fn download_and_install_app_update_async(
    download_url: &str,
    latest_version: &str,
) -> Result<String, String> {
    if download_url.trim().is_empty() {
        return Err("No release asset download URL available for this platform".to_string());
    }

    let filename = download_url
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(if cfg!(target_os = "windows") {
            "DuckyDeck-Studio-Setup.exe"
        } else {
            "duckydeck-studio-update.tar.gz"
        });

    let tmp_dir = update_staging_dir(latest_version);
    fs::create_dir_all(&tmp_dir)
        .map_err(|e| format!("Failed creating temporary update directory: {e}"))?;
    let pkg_path = tmp_dir.join(filename);

    let client = reqwest::Client::builder()
        .user_agent(concat!("DuckyDeck-Studio/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(180))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let resp = client
        .get(download_url)
        .send()
        .await
        .map_err(|e| format!("Failed downloading update asset: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("Download returned HTTP {}", resp.status()));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed reading downloaded asset bytes: {e}"))?;

    // Fetch and verify the companion `{download_url}.sha256` checksum published by the release pipeline
    let sha256_url = format!("{}.sha256", download_url.trim());
    let sha_resp = client
        .get(&sha256_url)
        .send()
        .await
        .map_err(|e| format!("Failed downloading SHA-256 checksum file ({sha256_url}): {e}"))?;
    if !sha_resp.status().is_success() {
        return Err(format!(
            "Refusing to install unverified update: SHA-256 checksum file ({sha256_url}) returned HTTP {}",
            sha_resp.status()
        ));
    }
    let sha_text = sha_resp
        .text()
        .await
        .map_err(|e| format!("Failed reading SHA-256 checksum response: {e}"))?;
    verify_sha256_manifest_text(&bytes, filename, &sha_text)?;

    fs::write(&pkg_path, &bytes).map_err(|e| {
        format!(
            "Failed writing update package to {}: {e}",
            pkg_path.display()
        )
    })?;

    let ver_clean = latest_version.trim_start_matches(['v', 'V']);

    #[cfg(target_os = "windows")]
    {
        let pkg_str = pkg_path.to_string_lossy().to_string();
        if filename.to_ascii_lowercase().ends_with(".exe") {
            Command::new("cmd")
                .args([
                    "/C",
                    "start",
                    "",
                    &pkg_str,
                    "/SILENT",
                    "/CLOSEAPPLICATIONS",
                    "/RESTARTAPPLICATIONS",
                ])
                .spawn()
                .map_err(|e| format!("Failed launching Windows Setup installer: {e}"))?;
            return Ok(format!(
                "Launched DuckyDeck Studio v{ver_clean} Windows Setup installer."
            ));
        }
        let _ = crate::paths::open_url_or_path_in_os(&pkg_str);
        return Ok(format!("Downloaded update archive to {pkg_str}"));
    }

    #[cfg(not(target_os = "windows"))]
    {
        let pkg_str = pkg_path.to_string_lossy().to_string();
        if filename.ends_with(".flatpak") {
            let scopes = if is_system_flatpak_install() {
                ["--system", "--user"]
            } else {
                ["--user", "--system"]
            };

            let installed = scopes
                .iter()
                .any(|scope| run_flatpak_bundle_install(&pkg_str, scope));

            if installed {
                let _ = fs::remove_dir_all(&tmp_dir);
                return Ok(format!(
                    "Installed DuckyDeck Studio v{ver_clean} Flatpak update. Restart DuckyDeck Studio to apply."
                ));
            }

            let _ = crate::paths::open_url_or_path_in_os(&pkg_str);
            return Ok(format!(
                "Downloaded Flatpak bundle to {pkg_str} (opened in software installer)."
            ));
        }

        if filename.ends_with(".tar.gz") {
            let status = Command::new("tar")
                .args(["-xzf", &pkg_str, "-C", &tmp_dir.to_string_lossy()])
                .status()
                .map_err(|e| format!("Failed running tar to extract update: {e}"))?;
            if !status.success() {
                return Err(format!("tar extraction exited with {status}"));
            }

            let mut extracted_bin = None;
            if let Ok(entries) = fs::read_dir(&tmp_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.is_dir() {
                        let candidate = p.join("duckydeck-studio");
                        if candidate.is_file() {
                            extracted_bin = Some(candidate);
                            break;
                        }
                    } else if p.file_name().and_then(|n| n.to_str()) == Some("duckydeck-studio") {
                        extracted_bin = Some(p);
                        break;
                    }
                }
            }

            let Some(new_bin) = extracted_bin else {
                return Err("Could not find duckydeck-studio binary in update archive".to_string());
            };

            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            let local_bin_dir = Path::new(&home).join(".local/bin");
            let _ = fs::create_dir_all(&local_bin_dir);
            let target_bin = std::env::current_exe()
                .ok()
                .filter(|p| {
                    let s = p.to_string_lossy();
                    !s.contains("/target/") && !s.starts_with("/app/") && !s.starts_with("/usr/")
                })
                .unwrap_or_else(|| local_bin_dir.join("duckydeck-studio"));

            let staged_bin = target_bin.with_extension("new");
            fs::copy(&new_bin, &staged_bin).map_err(|e| {
                format!(
                    "Failed staging updated binary to {}: {e}",
                    staged_bin.display()
                )
            })?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(meta) = fs::metadata(&staged_bin) {
                    let mut perms = meta.permissions();
                    perms.set_mode(0o755);
                    let _ = fs::set_permissions(&staged_bin, perms);
                }
            }
            fs::rename(&staged_bin, &target_bin)
                .map_err(|e| format!("Failed replacing binary at {}: {e}", target_bin.display()))?;

            return Ok(format!(
                "Installed DuckyDeck Studio v{ver_clean} to {}. Restart DuckyDeck Studio to apply.",
                target_bin.display()
            ));
        }

        let _ = crate::paths::open_url_or_path_in_os(&pkg_str);
        Ok(format!("Downloaded update to {pkg_str}"))
    }
}

pub fn download_and_install_app_update_blocking(
    download_url: &str,
    latest_version: &str,
) -> Result<String, String> {
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| format!("Failed creating async runtime: {e}"))?;
    rt.block_on(download_and_install_app_update_async(
        download_url,
        latest_version,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_comparison() {
        assert!(is_newer_version("0.1.8", "0.1.9"));
        assert!(is_newer_version("v0.1.8", "v0.2.0"));
        assert!(is_newer_version("0.1.8", "1.0.0"));
        assert!(!is_newer_version("0.1.8", "0.1.8"));
        assert!(!is_newer_version("0.1.9", "0.1.8"));
    }

    #[test]
    fn test_parse_github_release_json_selects_assets() {
        let sample = r#"{
            "tag_name": "v0.1.9",
            "html_url": "https://github.com/russellmilliner/duckydeck-studio/releases/tag/v0.1.9",
            "body": "Release notes: Added automatic update checker",
            "assets": [
                {
                    "name": "duckydeck-studio-v0.1.9-x86_64.flatpak",
                    "browser_download_url": "https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.9/duckydeck-studio-v0.1.9-x86_64.flatpak"
                },
                {
                    "name": "duckydeck-studio-v0.1.9-linux-x86_64.tar.gz",
                    "browser_download_url": "https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.9/duckydeck-studio-v0.1.9-linux-x86_64.tar.gz"
                },
                {
                    "name": "DuckyDeck-Studio-Setup-v0.1.9-x86_64.exe",
                    "browser_download_url": "https://github.com/russellmilliner/duckydeck-studio/releases/download/v0.1.9/DuckyDeck-Studio-Setup-v0.1.9-x86_64.exe"
                }
            ]
        }"#;

        let res_flatpak = parse_github_release_json(sample, "0.1.8", false, true).unwrap();
        assert!(res_flatpak.update_available);
        assert_eq!(res_flatpak.latest_version, "0.1.9");
        assert!(res_flatpak.download_url.ends_with(".flatpak"));

        let res_tar = parse_github_release_json(sample, "0.1.8", false, false).unwrap();
        assert!(res_tar.update_available);
        assert!(res_tar.download_url.ends_with(".tar.gz"));

        let res_win = parse_github_release_json(sample, "0.1.8", true, false).unwrap();
        assert!(res_win.update_available);
        assert!(res_win.download_url.ends_with(".exe"));

        let res_current = parse_github_release_json(sample, "0.1.9", false, true).unwrap();
        assert!(!res_current.update_available);
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_update_staging_dir_is_host_shared_not_private_tmp() {
        let dir = update_staging_dir("v0.1.19");
        let dir_str = dir.to_string_lossy();
        assert!(dir_str.contains("duckydeck-studio/updates/duckydeck-studio-update-0.1.19"));
        assert!(!dir.starts_with("/tmp/"));
    }

    #[test]
    fn test_verify_sha256_manifest_text_accepts_matching_and_rejects_tampered() {
        let payload = b"duckydeck-studio-release-binary-payload";
        let expected = crate::paths::sha256_hex(payload);
        let manifest = format!(
            "{expected}  duckydeck-studio-v0.1.19-x86_64.flatpak\n0000000000000000000000000000000000000000000000000000000000000000  other.tar.gz\n"
        );
        assert_eq!(
            verify_sha256_manifest_text(
                payload,
                "duckydeck-studio-v0.1.19-x86_64.flatpak",
                &manifest
            )
            .unwrap(),
            expected
        );
        assert!(verify_sha256_manifest_text(
            b"tampered-payload",
            "duckydeck-studio-v0.1.19-x86_64.flatpak",
            &manifest
        )
        .is_err());
    }
}
