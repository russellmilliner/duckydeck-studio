use std::fs;
use std::path::PathBuf;

use super::{is_any_browser_wildcard, is_web_browser_class, strip_browser_title_suffix};

fn site_icon_cache_dir() -> PathBuf {
    let dir = crate::paths::cache_dir().join("site-icons");
    if let Err(e) = fs::create_dir_all(&dir) {
        crate::bridge::device_controller::record_system_event(
            "WARN",
            "CONFIG",
            format!("Failed creating site icon cache dir {}: {e}", dir.display()),
        );
    }
    dir
}

fn write_cached_site_svg(slug: &str, svg_content: &str) -> String {
    let dir = site_icon_cache_dir();
    let path = dir.join(format!("{slug}.svg"));
    let needs_write = match fs::read_to_string(&path) {
        Ok(existing) => existing != svg_content,
        Err(_) => true,
    };
    if needs_write {
        if let Err(e) = fs::write(&path, svg_content) {
            crate::bridge::device_controller::record_system_event(
                "WARN",
                "CONFIG",
                format!("Failed writing cached site SVG {}: {e}", path.display()),
            );
        }
    }
    crate::paths::path_to_file_uri(&path)
}

pub use crate::bridge::showcase::known_site_svg;

fn generate_custom_site_badge_svg(site_hint: &str) -> (String, String) {
    let cleaned = strip_browser_title_suffix(site_hint);
    let candidate = cleaned
        .rsplit(['-', '—', '|', ':'])
        .map(|s| s.trim())
        .find(|s| !s.is_empty())
        .unwrap_or_else(|| cleaned.trim());

    let mut hash: u32 = 2166136261;
    for b in candidate.to_ascii_lowercase().bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }

    let palette = [
        "#2563EB", "#0891B2", "#059669", "#7C3AED", "#DB2777", "#EA580C", "#DC2626", "#4F46E5",
    ];
    let bg = palette[(hash as usize) % palette.len()];

    let monogram: String = candidate
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase();
    let label = if monogram.is_empty() {
        "W".to_string()
    } else {
        monogram
    };

    let slug = format!("site_{hash:08x}");
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="{bg}"/>
  <circle cx="32" cy="32" r="22" fill="none" stroke="#FFFFFF" stroke-opacity="0.22" stroke-width="2"/>
  <ellipse cx="32" cy="32" rx="11" ry="22" fill="none" stroke="#FFFFFF" stroke-opacity="0.22" stroke-width="2"/>
  <line x1="10" y1="32" x2="54" y2="32" stroke="#FFFFFF" stroke-opacity="0.22" stroke-width="2"/>
  <text x="32" y="40" font-family="sans-serif" font-size="23" font-weight="bold" text-anchor="middle" fill="#FFFFFF">{label}</text>
</svg>"##
    );
    (slug, svg)
}

fn resolve_favicon_for_url(raw_url: &str) -> Option<String> {
    let trimmed = raw_url.trim();
    if trimmed.is_empty() {
        return None;
    }

    let dir = site_icon_cache_dir();
    let mut hash: u32 = 2166136261;
    for b in trimmed.to_ascii_lowercase().bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16777619);
    }
    let stem = format!("fav_{hash:08x}");

    for ext in ["png", "ico", "svg", "jpg", "webp"] {
        let existing = dir.join(format!("{stem}.{ext}"));
        if existing.is_file() {
            return Some(crate::paths::path_to_file_uri(&existing));
        }
    }

    let with_scheme = if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
        trimmed.to_string()
    } else {
        format!("https://{trimmed}")
    };

    let (scheme, rest) = with_scheme.split_once("://").unwrap_or(("https", trimmed));
    let host_port = rest.split('/').next().unwrap_or(rest).trim();
    let host_only = host_port
        .split(':')
        .next()
        .unwrap_or(host_port)
        .to_ascii_lowercase();

    let mut candidates = Vec::new();
    let lower_url = with_scheme.to_ascii_lowercase();
    if lower_url.ends_with(".ico")
        || lower_url.ends_with(".png")
        || lower_url.ends_with(".svg")
        || lower_url.ends_with(".jpg")
        || lower_url.ends_with(".jpeg")
        || lower_url.ends_with(".webp")
    {
        candidates.push(with_scheme.clone());
    }

    let is_local_or_lan = host_only == "localhost"
        || host_only.ends_with(".local")
        || host_only.ends_with(".lan")
        || host_only.starts_with("127.")
        || host_only.starts_with("10.")
        || host_only.starts_with("192.168.")
        || host_only.starts_with("172.");

    if !host_only.is_empty() {
        if !is_local_or_lan && host_only.contains('.') {
            candidates.push(format!(
                "https://www.google.com/s2/favicons?domain={host_only}&sz=64"
            ));
        }
        candidates.push(format!("{scheme}://{host_port}/favicon.ico"));
        candidates.push(format!("{scheme}://{host_port}/favicon.png"));
    }

    let failed_marker = dir.join(format!("{stem}.failed"));
    if !failed_marker.is_file() && !candidates.is_empty() {
        let should_spawn = INFLIGHT_FAVICONS
            .lock()
            .map(|mut g| g.insert(stem.clone()))
            .unwrap_or(false);
        if should_spawn {
            let dir_bg = dir.clone();
            let stem_bg = stem.clone();
            let failed_marker_bg = failed_marker.clone();
            let candidates_bg = candidates.clone();
            let spawned = crate::actions::web_api::spawn_shared_async(async move {
                let mut wrote_icon = false;
                if let Ok(client) = crate::actions::web_api::shared_http_client() {
                    for cand in &candidates_bg {
                        let Ok(resp) = client
                            .get(cand)
                            .timeout(std::time::Duration::from_millis(2200))
                            .send()
                            .await
                        else {
                            continue;
                        };
                        if !resp.status().is_success() {
                            continue;
                        }
                        let ctype = resp
                            .headers()
                            .get(reqwest::header::CONTENT_TYPE)
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or("")
                            .to_ascii_lowercase();
                        if ctype.contains("text/html") {
                            continue;
                        }
                        let Ok(bytes) = resp.bytes().await else {
                            continue;
                        };
                        if bytes.len() < 24 || bytes.len() > 2_000_000 {
                            continue;
                        }
                        let prefix_ascii = String::from_utf8_lossy(&bytes[..bytes.len().min(64)])
                            .to_ascii_lowercase();
                        if prefix_ascii.contains("<!doctype") || prefix_ascii.contains("<html") {
                            continue;
                        }
                        let ext = if ctype.contains("svg") || prefix_ascii.contains("<svg") {
                            "svg"
                        } else if bytes.starts_with(b"\x89PNG") || ctype.contains("png") {
                            "png"
                        } else if bytes.starts_with(&[0x00, 0x00, 0x01, 0x00])
                            || ctype.contains("icon")
                        {
                            "ico"
                        } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) || ctype.contains("jpeg") {
                            "jpg"
                        } else {
                            "png"
                        };
                        let out_path = dir_bg.join(format!("{stem_bg}.{ext}"));
                        if fs::write(&out_path, &bytes).is_ok() {
                            wrote_icon = true;
                            break;
                        }
                    }
                }
                if wrote_icon {
                    if let Ok(mut guard) = RULE_ICON_CACHE.lock() {
                        guard.clear();
                    }
                } else if let Err(e) = fs::write(&failed_marker_bg, b"1") {
                    crate::bridge::device_controller::record_system_event(
                        "WARN",
                        "CONFIG",
                        format!(
                            "Failed writing favicon failure marker {}: {e}",
                            failed_marker_bg.display()
                        ),
                    );
                }
                if let Ok(mut g) = INFLIGHT_FAVICONS.lock() {
                    g.remove(&stem_bg);
                }
            });
            if !spawned {
                if let Ok(mut g) = INFLIGHT_FAVICONS.lock() {
                    g.remove(&stem);
                }
            }
        }
    }

    if let Some((slug, svg)) = known_site_svg(trimmed) {
        return Some(write_cached_site_svg(slug, &svg));
    }
    let (slug, svg) = generate_custom_site_badge_svg(if !host_only.is_empty() {
        &host_only
    } else {
        trimmed
    });
    Some(write_cached_site_svg(&slug, &svg))
}

static RULE_ICON_CACHE: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

static INFLIGHT_FAVICONS: std::sync::LazyLock<std::sync::Mutex<std::collections::HashSet<String>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashSet::new()));

pub fn resolve_rule_icon(
    resource_class: &str,
    title_contains: Option<&str>,
    favicon_url: Option<&str>,
    target_profile_name: Option<&str>,
) -> String {
    let cache_key = format!(
        "{}\x1f{}\x1f{}\x1f{}",
        resource_class.trim(),
        title_contains.unwrap_or("").trim(),
        favicon_url.unwrap_or("").trim(),
        target_profile_name.unwrap_or("").trim()
    );
    if let Ok(guard) = RULE_ICON_CACHE.lock() {
        if let Some(cached) = guard.get(&cache_key) {
            return cached.clone();
        }
    }

    let resolved = resolve_rule_icon_uncached(
        resource_class,
        title_contains,
        favicon_url,
        target_profile_name,
    );
    if let Ok(mut guard) = RULE_ICON_CACHE.lock() {
        guard.insert(cache_key, resolved.clone());
    }
    resolved
}

fn resolve_rule_icon_uncached(
    resource_class: &str,
    title_contains: Option<&str>,
    favicon_url: Option<&str>,
    target_profile_name: Option<&str>,
) -> String {
    if let Some(raw_fav) = favicon_url {
        let fav_trimmed = raw_fav.trim();
        if !fav_trimmed.is_empty() {
            if let Some(icon_uri) = resolve_favicon_for_url(fav_trimmed) {
                return icon_uri;
            }
        }
    }

    let trimmed_class = resource_class.trim();
    if is_web_browser_class(trimmed_class) {
        let title_str = title_contains.unwrap_or("").trim();
        let cleaned_title = strip_browser_title_suffix(title_str);

        // 1. If the browser rule specifies a title substring, always resolve a site icon!
        if !cleaned_title.is_empty() {
            if let Some((slug, svg)) = known_site_svg(&cleaned_title) {
                return write_cached_site_svg(slug, &svg);
            }
            if cleaned_title.contains('.') && !cleaned_title.contains(' ') {
                if let Some(uri) = resolve_favicon_for_url(&cleaned_title) {
                    return uri;
                }
            }
            let (slug, svg) = generate_custom_site_badge_svg(&cleaned_title);
            return write_cached_site_svg(&slug, &svg);
        }

        // 2. If title_contains is empty, check if the target profile name matches a known site
        if let Some(prof_name) = target_profile_name {
            let p_trim = prof_name.trim();
            if !p_trim.is_empty() && !is_web_browser_class(p_trim) {
                if let Some((slug, svg)) = known_site_svg(p_trim) {
                    return write_cached_site_svg(slug, &svg);
                }
            }
        }

        // 3. If the rule matches all browsers (*browser*) and has no site title, show a multi-browser globe icon
        if is_any_browser_wildcard(trimmed_class) {
            let all_browsers_svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#0284C7"/>
  <circle cx="32" cy="32" r="21" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <ellipse cx="32" cy="32" rx="11" ry="21" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="11" y1="32" x2="53" y2="32" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="15" y1="21" x2="49" y2="21" stroke="#FFFFFF" stroke-width="2.5"/>
  <line x1="15" y1="43" x2="49" y2="43" stroke="#FFFFFF" stroke-width="2.5"/>
</svg>"##;
            return write_cached_site_svg("all-browsers", all_browsers_svg);
        }
    }

    resolve_app_icon_for_class(trimmed_class)
}

pub fn resolve_app_icon_for_class(resource_class: &str) -> String {
    static APP_ICON_CACHE: std::sync::OnceLock<
        std::sync::Mutex<std::collections::HashMap<String, String>>,
    > = std::sync::OnceLock::new();

    let trimmed = resource_class.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if is_any_browser_wildcard(trimmed) {
        return resolve_rule_icon(trimmed, None, None, None);
    }
    let lower = trimmed.to_ascii_lowercase();
    let cache =
        APP_ICON_CACHE.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    if let Ok(guard) = cache.lock() {
        if let Some(cached) = guard.get(&lower) {
            return cached.clone();
        }
    }

    let resolved = resolve_app_icon_for_class_uncached(trimmed, &lower);
    if let Ok(mut guard) = cache.lock() {
        guard.insert(lower, resolved.clone());
    }
    resolved
}

fn resolve_app_icon_for_class_uncached(trimmed: &str, lower: &str) -> String {
    let short_lower = lower.rsplit('.').next().unwrap_or(lower).to_string();

    let mut search_dirs = Vec::new();
    let data_home = crate::paths::xdg_data_home();
    search_dirs.push(data_home.join("applications"));
    search_dirs.push(data_home.join("flatpak/exports/share/applications"));
    if let Some(home) = crate::paths::home_dir() {
        for candidate in [
            home.join(".local/share/applications"),
            home.join(".local/share/flatpak/exports/share/applications"),
        ] {
            if !search_dirs.contains(&candidate) {
                search_dirs.push(candidate);
            }
        }
    }
    search_dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
    search_dirs.push(PathBuf::from("/run/host/usr/local/share/applications"));
    search_dirs.push(PathBuf::from("/run/host/usr/share/applications"));
    search_dirs.push(PathBuf::from("/usr/local/share/applications"));
    search_dirs.push(PathBuf::from("/usr/share/applications"));

    fn extract_icon_from_desktop(content: &str) -> Option<String> {
        let mut in_entry = false;
        for line in content.lines() {
            let l = line.trim();
            if l.starts_with('[') {
                in_entry = l == "[Desktop Entry]";
                continue;
            }
            if in_entry {
                if let Some(icon) = l.strip_prefix("Icon=") {
                    let val = icon.trim();
                    if !val.is_empty() {
                        return Some(val.to_string());
                    }
                }
            }
        }
        None
    }

    // 1. Direct desktop filename match
    for dir in &search_dirs {
        for cand in [
            format!("{trimmed}.desktop"),
            format!("{lower}.desktop"),
            format!("org.kde.{short_lower}.desktop"),
        ] {
            let p = dir.join(&cand);
            if p.is_file() {
                if let Ok(content) = fs::read_to_string(&p) {
                    if let Some(icon) = extract_icon_from_desktop(&content) {
                        return icon;
                    }
                }
            }
        }
    }

    // 2. Scan desktop files for StartupWMClass, filename suffix, or Exec binary match
    for dir in &search_dirs {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(fname) = path.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if !fname.ends_with(".desktop") {
                continue;
            };
            let stem_lower = fname.trim_end_matches(".desktop").to_ascii_lowercase();
            let stem_matches = stem_lower == lower
                || stem_lower == short_lower
                || stem_lower.ends_with(&format!(".{short_lower}"));

            let Ok(content) = fs::read_to_string(&path) else {
                continue;
            };

            if stem_matches {
                if let Some(icon) = extract_icon_from_desktop(&content) {
                    return icon;
                }
            }

            let mut wm_match = false;
            let mut exec_match = false;
            let mut in_entry = false;
            for line in content.lines() {
                let l = line.trim();
                if l.starts_with('[') {
                    in_entry = l == "[Desktop Entry]";
                    continue;
                }
                if !in_entry {
                    continue;
                }
                if let Some(wm) = l.strip_prefix("StartupWMClass=") {
                    let wm_l = wm.trim().to_ascii_lowercase();
                    if wm_l == lower || wm_l == short_lower {
                        wm_match = true;
                        break;
                    }
                } else if let Some(exec) = l.strip_prefix("Exec=") {
                    let first_tok = exec
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .trim_matches('"');
                    let bin = std::path::Path::new(first_tok)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("")
                        .to_ascii_lowercase();
                    if !bin.is_empty() && (bin == lower || bin == short_lower) {
                        exec_match = true;
                    }
                }
            }

            if wm_match || exec_match {
                if let Some(icon) = extract_icon_from_desktop(&content) {
                    return icon;
                }
            }
        }
    }

    // 3. Fallback common icon names
    match short_lower.as_str() {
        "dolphin" => "system-file-manager".to_string(),
        "konsole" | "alacritty" | "kitty" | "wezterm" => "utilities-terminal".to_string(),
        "code" | "code-oss" | "codium" | "vscodium" => "com.visualstudio.code".to_string(),
        "firefox" | "navigator" => "firefox".to_string(),
        "chrome" | "google-chrome" => "google-chrome".to_string(),
        "chromium" => "chromium".to_string(),
        _ => short_lower,
    }
}
