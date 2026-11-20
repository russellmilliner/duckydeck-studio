use std::fs;
use std::path::{Path, PathBuf};

use super::{load_rules_bundle_from_dir, AutoSwitchRule, SwitcherState};
use crate::hardware::storage::DuckyProfile;

pub const ANY_BROWSER_CLASS: &str = "*browser*";

pub fn is_any_browser_wildcard(rule_class: &str) -> bool {
    matches!(
        rule_class.trim().to_ascii_lowercase().as_str(),
        "*browser*"
            | "*browsers*"
            | "any-browser"
            | "all-browsers"
            | "web-browser"
            | "web-browsers"
            | "browser"
            | "browsers"
    )
}

pub fn is_web_browser_class(resource_class: &str) -> bool {
    let trimmed = resource_class.trim();
    if trimmed.is_empty() {
        return false;
    }
    if is_any_browser_wildcard(trimmed) {
        return true;
    }

    for part in trimmed.split([',', '|']) {
        let token = part.trim().to_ascii_lowercase();
        if token.is_empty() {
            continue;
        }
        if is_any_browser_wildcard(&token) {
            return true;
        }
        let short = token.rsplit('.').next().unwrap_or(token.as_str());
        let is_browser = matches!(
            short,
            "chrome"
                | "google-chrome"
                | "google-chrome-stable"
                | "google-chrome-beta"
                | "google-chrome-unstable"
                | "chromium"
                | "chromium-browser"
                | "ungoogled-chromium"
                | "thorium"
                | "thorium-browser"
                | "zen"
                | "zen-alpha"
                | "zen-beta"
                | "zen-twilight"
                | "zen-browser"
                | "zen_browser"
                | "firefox"
                | "firefox-esr"
                | "firefox-devedition"
                | "firefox-nightly"
                | "navigator"
                | "librewolf"
                | "librewolf-community"
                | "waterfox"
                | "floorp"
                | "tor-browser"
                | "icecat"
                | "palemoon"
                | "seamonkey"
                | "basilisk"
                | "mullvadbrowser"
                | "mercury"
                | "edge"
                | "microsoft-edge"
                | "microsoft-edge-stable"
                | "microsoft-edge-beta"
                | "microsoft-edge-dev"
                | "msedge"
                | "brave"
                | "brave-browser"
                | "brave-bin"
                | "opera"
                | "opera-beta"
                | "opera-developer"
                | "vivaldi"
                | "vivaldi-stable"
                | "vivaldi-snapshot"
                | "falkon"
                | "konqueror"
                | "epiphany"
                | "midori"
                | "qutebrowser"
                | "nyxt"
                | "min"
                | "arc"
                | "ladybird"
                | "whale"
                | "naver-whale"
                | "yandex-browser"
                | "maxthon"
                | "slimjet"
                | "wavebox"
                | "sidekick"
                | "dillo"
                | "netsurf"
                | "luakit"
                | "vieb"
        ) || token.contains("chrome")
            || token.contains("chromium")
            || token.contains("firefox")
            || token.contains("zen_browser")
            || token.contains("zen-browser")
            || token.contains("brave")
            || token.contains("microsoft-edge")
            || token.contains("vivaldi")
            || token.contains("opera")
            || token.contains("librewolf")
            || token.contains("waterfox")
            || token.contains("floorp");
        if is_browser {
            return true;
        }
    }
    false
}

pub fn is_chromium_pwa_class(class_lower: &str) -> bool {
    for prefix in ["chrome-", "chromium-", "brave-", "msedge-"] {
        if let Some(rest) = class_lower.strip_prefix(prefix) {
            let app_id = rest.split('-').next().unwrap_or("");
            if (16..=40).contains(&app_id.len())
                && app_id.chars().all(|c| c.is_ascii_alphanumeric())
            {
                return true;
            }
        }
    }
    false
}

pub fn rule_matches_window_class(rule_class_raw: &str, active_window_class_lower: &str) -> bool {
    let trimmed = rule_class_raw.trim();
    if trimmed.is_empty() || active_window_class_lower.is_empty() {
        return false;
    }
    if trimmed == "*" || trimmed.eq_ignore_ascii_case("any") {
        return true;
    }
    if is_any_browser_wildcard(trimmed) {
        return is_web_browser_class(active_window_class_lower)
            || is_chromium_pwa_class(active_window_class_lower);
    }

    let is_pwa = is_chromium_pwa_class(active_window_class_lower);
    let norm_active: String = active_window_class_lower
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();

    for part in trimmed.split([',', '|']) {
        let token = part.trim().to_ascii_lowercase();
        if token.is_empty() {
            continue;
        }
        if token == "*" || token == "any" {
            return true;
        }
        if is_any_browser_wildcard(&token) {
            if is_web_browser_class(active_window_class_lower)
                || is_chromium_pwa_class(active_window_class_lower)
            {
                return true;
            }
            continue;
        }
        if active_window_class_lower == token {
            return true;
        }
        // Do not let generic browser rules (e.g. "chrome", "chromium") match standalone Chrome PWAs
        // like "chrome-hbmnbinfcdjocecbhmpfeapfnchehanm-Profile_1" unless the rule explicitly targets that PWA ID.
        if is_pwa && !is_chromium_pwa_class(&token) {
            continue;
        }
        let norm_token: String = token
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect();
        if !norm_token.is_empty() && norm_active == norm_token {
            return true;
        }
        if active_window_class_lower
            .split(['.', '-', '_'])
            .any(|seg| seg == token || (!norm_token.is_empty() && seg == norm_token))
        {
            return true;
        }
        if token.len() >= 5 && active_window_class_lower.contains(&token) {
            return true;
        }
    }
    false
}

pub fn extract_browser_title_suffix(caption: &str) -> Option<&'static str> {
    let trimmed = caption.trim();
    let lower = trimmed.to_lowercase();
    let suffixes: [&'static str; 38] = [
        " — Zen Browser",
        " - Zen Browser",
        " — Zen Twilight",
        " - Zen Twilight",
        " — Zen",
        " — Mozilla Firefox",
        " - Mozilla Firefox",
        " — Firefox Developer Edition",
        " - Firefox Developer Edition",
        " — Firefox Nightly",
        " - Firefox Nightly",
        " — Firefox",
        " - Firefox",
        " - Google Chrome",
        " — Google Chrome",
        " - Chromium",
        " — Chromium",
        " - Thorium",
        " — Thorium",
        " - Brave",
        " — Brave",
        " - Microsoft\u{200b} Edge",
        " - Microsoft Edge",
        " — Microsoft Edge",
        " - Opera",
        " — Opera",
        " - Vivaldi",
        " — Vivaldi",
        " — LibreWolf",
        " - LibreWolf",
        " — Waterfox",
        " - Waterfox",
        " — Floorp",
        " - Floorp",
        " — Falkon",
        " - Falkon",
        " — qutebrowser",
        " - qutebrowser",
    ];
    suffixes
        .into_iter()
        .find(|&suf| trimmed.len() > suf.len() && lower.ends_with(&suf.to_lowercase()))
}

pub fn strip_browser_title_suffix(caption: &str) -> String {
    let mut s = caption.trim().to_string();
    if let Some(suf) = extract_browser_title_suffix(&s) {
        let new_len = s.len() - suf.len();
        s.truncate(new_len);
        s = s.trim().to_string();
    }
    // Also strip leading unread notification badge like "(3) YouTube" -> "YouTube"
    if let Some(rest) = s.strip_prefix('(') {
        if let Some((digits, after)) = rest.split_once(')') {
            if !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
                let cleaned = after.trim();
                if !cleaned.is_empty() {
                    s = cleaned.to_string();
                }
            }
        }
    }
    s
}

static RESOLVED_BROWSER_TITLES: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, Option<String>>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

const MAX_BROWSER_RECOVERY_COMPRESSED_BYTES: u64 = 8 * 1024 * 1024;
pub(super) const MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES: usize = 16 * 1024 * 1024;
const MAX_BROWSER_SQLITE_SCAN_BYTES: u64 = 16 * 1024 * 1024;
const MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES: usize = 256;

static LAST_BROWSER_SITE_SUFFIX: std::sync::LazyLock<
    std::sync::Mutex<std::collections::HashMap<String, String>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

pub(super) fn decompress_mozlz4(data: &[u8]) -> Option<Vec<u8>> {
    if data.len() < 12 || &data[..8] != b"mozLz40\0" {
        return None;
    }
    let uncomp_len = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
    if uncomp_len == 0 || uncomp_len > MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES {
        return None;
    }
    let src = &data[12..];
    let mut out = Vec::with_capacity(uncomp_len);
    let mut i = 0usize;
    let n = src.len();
    while i < n {
        let token = src[i] as usize;
        i += 1;
        let mut lit_len = token >> 4;
        if lit_len == 15 {
            while i < n {
                let b = src[i] as usize;
                i += 1;
                lit_len = lit_len.checked_add(b)?;
                if lit_len > uncomp_len {
                    return None;
                }
                if b != 255 {
                    break;
                }
            }
        }
        let lit_end = i.checked_add(lit_len)?;
        if lit_end > n {
            break;
        }
        if out.len().checked_add(lit_len)? > uncomp_len {
            return None;
        }
        out.extend_from_slice(&src[i..lit_end]);
        i = lit_end;
        if i >= n {
            break;
        }
        if i + 2 > n {
            break;
        }
        let offset = (src[i] as usize) | ((src[i + 1] as usize) << 8);
        i += 2;
        if offset == 0 || offset > out.len() {
            return None;
        }
        let mut match_len = (token & 0x0f) + 4;
        if (token & 0x0f) == 15 {
            while i < n {
                let b = src[i] as usize;
                i += 1;
                match_len = match_len.checked_add(b)?;
                if match_len > uncomp_len {
                    return None;
                }
                if b != 255 {
                    break;
                }
            }
        }
        if out.len().checked_add(match_len)? > uncomp_len {
            return None;
        }
        let start = out.len() - offset;
        for j in 0..match_len {
            let b = out[start + j];
            out.push(b);
        }
    }
    Some(out)
}

fn clean_sqlite_extracted_title(raw: &str) -> String {
    let mut s = raw.trim().to_string();
    // In Firefox/Zen moz_places records, the `rev_host` column (e.g. `moc.elgoog.liam.`)
    // immediately follows `title` in the raw row payload.
    if let Some(pos) = s.find("moc.elgoog.liam.") {
        s.truncate(pos);
        if !s.to_lowercase().contains("gmail") {
            s.push_str(" - Gmail");
        }
        return s.trim().to_string();
    }
    for rev_marker in [
        "moc.elgoog.",
        "moc.ebotuoy.",
        "moc.kooltuo.",
        "moc.tfosorcim.",
        "moc.buhtig.",
    ] {
        if let Some(pos) = s.find(rev_marker) {
            s.truncate(pos);
            return s.trim().to_string();
        }
    }
    // Strip any generic trailing reversed hostname like `moc.elpmaxe.www.` if glued to "- Gmail"
    if let Some(gmail_end) = s.rfind(" - Gmail") {
        let after = &s[gmail_end + " - Gmail".len()..];
        if after.ends_with('.')
            && after
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '-')
        {
            s.truncate(gmail_end + " - Gmail".len());
        }
    }
    s.trim().to_string()
}

fn lookup_full_title_in_browser_storage(search_prefix: &str) -> Option<String> {
    let home_path = crate::paths::home_dir()?;
    let roots = [
        home_path.join(".var/app/app.zen_browser.zen/.zen"),
        home_path.join(".zen"),
        home_path.join(".mozilla/firefox"),
        home_path.join(".var/app/org.mozilla.firefox/.mozilla/firefox"),
    ];

    let prefix_bytes = search_prefix.as_bytes();
    if prefix_bytes.len() < 32 {
        return None;
    }

    let mut profile_dirs: Vec<(std::time::SystemTime, PathBuf)> = Vec::new();
    for root in &roots {
        let Ok(entries) = fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if !p.is_dir() {
                continue;
            }
            let mtime = p
                .join("places.sqlite-wal")
                .metadata()
                .or_else(|_| p.join("places.sqlite").metadata())
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            profile_dirs.push((mtime, p));
        }
    }
    profile_dirs.sort_by_key(|a| std::cmp::Reverse(a.0));

    // 1. Check recovery.jsonlz4 first (live sessionstore with exact tab title & URL),
    //    checking the 12-byte mozLz40 header first so multi-tens-of-MB session files never spike RSS.
    for (_, prof_dir) in &profile_dirs {
        let recovery_path = prof_dir.join("sessionstore-backups/recovery.jsonlz4");
        let Ok(mut file) = fs::File::open(&recovery_path) else {
            continue;
        };
        let Ok(meta) = file.metadata() else {
            continue;
        };
        if meta.len() < 12 || meta.len() > MAX_BROWSER_RECOVERY_COMPRESSED_BYTES {
            continue;
        }
        let mut hdr = [0u8; 12];
        if std::io::Read::read_exact(&mut file, &mut hdr).is_err() || &hdr[..8] != b"mozLz40\0" {
            continue;
        }
        let uncomp_len = u32::from_le_bytes([hdr[8], hdr[9], hdr[10], hdr[11]]) as usize;
        if uncomp_len == 0 || uncomp_len > MAX_BROWSER_RECOVERY_UNCOMPRESSED_BYTES {
            continue;
        }
        let mut compressed = Vec::with_capacity(meta.len() as usize);
        compressed.extend_from_slice(&hdr);
        if std::io::Read::read_to_end(&mut file, &mut compressed).is_ok() {
            if let Some(raw_json) = decompress_mozlz4(&compressed) {
                if let Some(pos) = raw_json
                    .windows(prefix_bytes.len())
                    .rposition(|w| w == prefix_bytes)
                {
                    let slice = &raw_json[pos..raw_json.len().min(pos + 1024)];
                    let mut end = 0usize;
                    while end < slice.len() {
                        if slice[end] == b'"' && (end == 0 || slice[end - 1] != b'\\') {
                            break;
                        }
                        if slice[end] < 0x20 {
                            break;
                        }
                        end += 1;
                    }
                    let extracted = String::from_utf8_lossy(&slice[..end]).to_string();
                    if extracted.len() > search_prefix.len() {
                        return Some(extracted);
                    }
                }
            }
        }
    }

    // 2. Check places.sqlite-wal and places.sqlite (bounded to <= 16 MB to prevent large heap spikes)
    for (_, prof_dir) in &profile_dirs {
        for fname in ["places.sqlite-wal", "places.sqlite"] {
            let db_path = prof_dir.join(fname);
            let Ok(meta) = fs::metadata(&db_path) else {
                continue;
            };
            if meta.len() == 0 || meta.len() > MAX_BROWSER_SQLITE_SCAN_BYTES {
                continue;
            }
            let Ok(data) = fs::read(&db_path) else {
                continue;
            };
            if let Some(pos) = data
                .windows(prefix_bytes.len())
                .rposition(|w| w == prefix_bytes)
            {
                let slice = &data[pos..data.len().min(pos + 1024)];
                let text = String::from_utf8_lossy(slice);
                let printable: String = text.chars().take_while(|&c| (c as u32) >= 0x20).collect();
                let cleaned = clean_sqlite_extracted_title(&printable);
                if cleaned.len() > search_prefix.len() {
                    return Some(cleaned);
                }
            }
        }
    }

    None
}

/// Resolves the full untruncated title of a browser window when the browser (e.g. Zen Browser / Firefox)
/// truncates `<title>` at 255/256 UTF-8 bytes before appending the site name (such as `- user@gmail.com - Gmail`).
pub fn resolve_full_browser_caption(resource_class: &str, caption: &str) -> String {
    if !is_web_browser_class(resource_class) {
        return caption.to_string();
    }
    let stripped = strip_browser_title_suffix(caption);
    let browser_suffix = extract_browser_title_suffix(caption).unwrap_or("");
    let lower_class = resource_class.trim().to_ascii_lowercase();

    // If this normal (non-truncated) browser title explicitly shows Gmail, remember that this browser window
    // is currently on a Gmail tab in case the user clicks an email with a 256+ byte subject next.
    if stripped.len() < 200 {
        if let Ok(mut last_map) = LAST_BROWSER_SITE_SUFFIX.lock() {
            if stripped.to_ascii_lowercase().contains("gmail") {
                last_map.insert(lower_class, " - Gmail".to_string());
            } else if !stripped.is_empty()
                && !stripped.eq_ignore_ascii_case("zen browser")
                && !stripped.eq_ignore_ascii_case("mozilla firefox")
            {
                last_map.remove(&lower_class);
            }
        }
        return caption.to_string();
    }

    // Build a stable 96-character search prefix from the truncated title
    let search_prefix: String = stripped.chars().take(96).collect();
    if search_prefix.len() < 40 {
        return caption.to_string();
    }

    if let Ok(cache) = RESOLVED_BROWSER_TITLES.lock() {
        if let Some(cached_entry) = cache.get(&search_prefix) {
            return match cached_entry {
                Some(full_stripped) => format!("{full_stripped}{browser_suffix}"),
                None => caption.to_string(),
            };
        }
    }

    if let Some(full_stripped) = lookup_full_title_in_browser_storage(&search_prefix) {
        if let Ok(mut cache) = RESOLVED_BROWSER_TITLES.lock() {
            if cache.len() >= MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES {
                cache.clear();
            }
            cache.insert(search_prefix, Some(full_stripped.clone()));
        }
        if full_stripped.to_ascii_lowercase().contains("gmail") {
            if let Ok(mut last_map) = LAST_BROWSER_SITE_SUFFIX.lock() {
                last_map.insert(lower_class, " - Gmail".to_string());
            }
        }
        return format!("{full_stripped}{browser_suffix}");
    }

    // Fallback for brand-new long email clicked while already inside Gmail before SQLite WAL flushes to disk:
    // Gecko truncates long `<title>` strings at ~254-256 UTF-8 bytes.
    if stripped.len() >= 230 {
        if let Ok(last_map) = LAST_BROWSER_SITE_SUFFIX.lock() {
            if let Some(site_suf) = last_map.get(&lower_class) {
                let augmented = format!("{stripped} …{site_suf}");
                if let Ok(mut cache) = RESOLVED_BROWSER_TITLES.lock() {
                    if cache.len() >= MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES {
                        cache.clear();
                    }
                    cache.insert(search_prefix, Some(augmented.clone()));
                }
                return format!("{augmented}{browser_suffix}");
            }
        }
    }

    if let Ok(mut cache) = RESOLVED_BROWSER_TITLES.lock() {
        if cache.len() >= MAX_RESOLVED_BROWSER_TITLE_CACHE_ENTRIES {
            cache.clear();
        }
        cache.insert(search_prefix, None);
    }

    caption.to_string()
}

pub fn discover_dekunukem_autoswitcher_configs() -> Vec<(PathBuf, usize)> {
    let mut dirs = Vec::new();
    let host_dir = crate::paths::dekunukem_autoswitcher_dir();
    if host_dir.is_dir() {
        dirs.push(host_dir);
    }
    let xdg_dir = crate::paths::xdg_data_home().join("duckypad_autoswitcher");
    if xdg_dir.is_dir() && !dirs.contains(&xdg_dir) {
        dirs.push(xdg_dir);
    }

    let mut found: Vec<(PathBuf, usize)> = Vec::new();
    for dir in dirs {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
        paths.sort_by(|a, b| {
            let a_name = a.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let b_name = b.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let rank = |n: &str| -> u8 {
                if n.eq_ignore_ascii_case("config-LinuxPro.txt") {
                    0
                } else if n.eq_ignore_ascii_case("config-Pro.txt") {
                    1
                } else if n.eq_ignore_ascii_case("config.txt") {
                    2
                } else {
                    3
                }
            };
            rank(a_name)
                .cmp(&rank(b_name))
                .then_with(|| a_name.cmp(b_name))
        });
        for p in paths {
            if !p.is_file() {
                continue;
            }
            let Some(fname) = p.file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if fname.eq_ignore_ascii_case("debug_log.txt") {
                continue;
            }
            let Ok(raw) = fs::read_to_string(&p) else {
                continue;
            };
            let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) else {
                continue;
            };
            if let Some(arr) = val.get("rules_list").and_then(|v| v.as_array()) {
                if !arr.is_empty() {
                    found.push((p, arr.len()));
                }
            }
        }
    }
    found
}

fn resolve_dekunukem_switch_to(
    switch_to: &serde_json::Value,
    profiles: &[DuckyProfile],
) -> Option<u8> {
    if let Some(num) = switch_to.as_u64() {
        if (1..=64).contains(&num) {
            return Some(num as u8);
        }
    }
    let s = switch_to.as_str()?.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(p) = profiles.iter().find(|p| p.name.eq_ignore_ascii_case(s)) {
        return Some(p.number);
    }
    let norm = |txt: &str| -> String {
        txt.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase()
    };
    let target_norm = norm(s);
    if !target_norm.is_empty() {
        if let Some(p) = profiles.iter().find(|p| norm(&p.name) == target_norm) {
            return Some(p.number);
        }
        // Common shorthand aliases used on 5-16 char duckyPad profile names
        let aliases: &[(&str, &[&str])] = &[
            ("ytmusic", &["youtubemusic", "ytmusic", "music"]),
            ("homeassist", &["homeassistant", "homeassist", "hass"]),
            ("msteams", &["microsoftteams", "msteams", "teams"]),
            ("gcalendar", &["googlecalendar", "gcalendar", "calendar"]),
            ("gmaps", &["googlemaps", "gmaps", "maps"]),
            (
                "ageofemiresii",
                &["ageofempiresii", "ageofemiresii", "aoe2"],
            ),
        ];
        for &(key, group) in aliases {
            if target_norm == key || group.contains(&target_norm.as_str()) {
                if let Some(p) = profiles
                    .iter()
                    .find(|p| group.contains(&norm(&p.name).as_str()))
                {
                    return Some(p.number);
                }
            }
        }
        if target_norm.len() >= 4 {
            if let Some(p) = profiles.iter().find(|p| {
                let pn = norm(&p.name);
                pn.len() >= 4 && (pn.starts_with(&target_norm) || target_norm.starts_with(&pn))
            }) {
                return Some(p.number);
            }
        }
    }
    if let Ok(num) = s.parse::<u8>() {
        if (1..=64).contains(&num) {
            return Some(num);
        }
    }
    None
}

pub fn parse_dekunukem_autoswitcher_file(
    path: &Path,
    profiles: &[DuckyProfile],
) -> Result<(bool, Option<u8>, Vec<AutoSwitchRule>), String> {
    let raw =
        fs::read_to_string(path).map_err(|e| format!("Failed reading {}: {e}", path.display()))?;
    let val: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed parsing {}: {e}", path.display()))?;
    let autoswitch_enabled = val
        .get("autoswitch_enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let arr = val
        .get("rules_list")
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("No 'rules_list' array found in {}", path.display()))?;

    let mut default_profile: Option<u8> = None;
    let mut rules: Vec<AutoSwitchRule> = Vec::new();

    for item in arr {
        let app_raw = item
            .get("app_name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let title_raw = item
            .get("window_title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        let enabled = item
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let Some(switch_val) = item.get("switch_to") else {
            continue;
        };
        let Some(target_profile) = resolve_dekunukem_switch_to(switch_val, profiles) else {
            continue;
        };

        if app_raw.is_empty() && title_raw.is_empty() {
            if enabled {
                default_profile = Some(target_profile);
            }
            continue;
        }

        let app_lower = app_raw.to_ascii_lowercase();
        if app_lower == "kthreadd" || app_lower == "kded5" {
            continue;
        }

        let (resource_class, title_opt) = if app_raw.is_empty() {
            let cleaned_t = strip_browser_title_suffix(title_raw)
                .trim_matches(|c: char| c == '-' || c.is_whitespace())
                .to_string();
            let final_t = if cleaned_t.is_empty() {
                title_raw.to_string()
            } else {
                cleaned_t
            };
            (ANY_BROWSER_CLASS.to_string(), Some(final_t))
        } else {
            let norm_class = match app_lower.as_str() {
                "dolphin" => "org.kde.dolphin".to_string(),
                "yakuake" => "org.kde.yakuake".to_string(),
                "zen" => "app.zen_browser.zen".to_string(),
                "orcaslicer" => "OrcaSlicer".to_string(),
                "upnote" => "upnote".to_string(),
                _ => app_raw.to_string(),
            };
            let cleaned_t = if title_raw.is_empty() {
                String::new()
            } else if is_web_browser_class(&norm_class) {
                strip_browser_title_suffix(title_raw)
                    .trim_matches(|c: char| c == '-' || c.is_whitespace())
                    .to_string()
            } else {
                let tl = title_raw.to_ascii_lowercase();
                if (app_lower == "code" && tl.contains("visual studio code"))
                    || (app_lower == "codium" && tl.contains("vscodium"))
                    || (app_lower == "upnote" && tl == "upnote")
                {
                    String::new()
                } else {
                    title_raw.to_string()
                }
            };
            (
                norm_class,
                if cleaned_t.is_empty() {
                    None
                } else {
                    Some(cleaned_t)
                },
            )
        };

        let duplicate = rules.iter().any(|r| {
            r.resource_class.eq_ignore_ascii_case(&resource_class)
                && r.title_contains
                    .as_deref()
                    .unwrap_or("")
                    .eq_ignore_ascii_case(title_opt.as_deref().unwrap_or(""))
                && r.target_profile == target_profile
        });
        if !duplicate {
            rules.push(AutoSwitchRule {
                enabled,
                resource_class,
                title_contains: title_opt,
                title_is_regex: false,
                favicon_url: None,
                target_profile,
            });
        }
    }

    Ok((autoswitch_enabled, default_profile, rules))
}

pub fn build_auto_aligned_rules(profiles: &[DuckyProfile]) -> Vec<AutoSwitchRule> {
    let known_mappings: &[(&str, &str, Option<&str>)] = &[
        ("Firefox", "firefox", None),
        ("Chrome", "chrome", None),
        ("Dolphin", "org.kde.dolphin", None),
        ("Yakuake", "org.kde.yakuake", None),
        ("VS Code", "code", None),
        ("OBS", "com.obsproject.Studio", None),
        ("Discord", "discord", None),
        ("Slack", "Slack", None),
        ("Zoom", "zoom", None),
        ("OrcaSlicer", "OrcaSlicer", None),
        ("UpNote", "upnote", None),
        ("YouTube", ANY_BROWSER_CLASS, Some("YouTube")),
        ("Gmail", ANY_BROWSER_CLASS, Some("Gmail")),
    ];

    let mut aligned = Vec::new();
    for &(prof_name_pat, class_name, title_sub) in known_mappings {
        if let Some(prof) = profiles
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(prof_name_pat))
        {
            aligned.push(AutoSwitchRule {
                enabled: true,
                resource_class: class_name.to_string(),
                title_contains: title_sub.map(|s| s.to_string()),
                title_is_regex: false,
                favicon_url: None,
                target_profile: prof.number,
            });
        }
    }
    aligned
}

pub fn extract_inline_regex_pattern(raw: &str) -> Option<(&str, bool)> {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix('/') {
        if let Some(body) = rest.strip_suffix("/i") {
            if !body.is_empty() {
                return Some((body, true));
            }
        } else if let Some(body) = rest.strip_suffix("/s") {
            if !body.is_empty() {
                return Some((body, false));
            }
        } else if let Some(body) = rest.strip_suffix('/') {
            if !body.is_empty() {
                return Some((body, true));
            }
        }
    }
    None
}

pub fn validate_title_regex(pattern_raw: &str) -> Result<(), String> {
    let trimmed = pattern_raw.trim();
    if trimmed.is_empty() {
        return Ok(());
    }
    let (body, case_insensitive) = extract_inline_regex_pattern(trimmed).unwrap_or((trimmed, true));
    regex::RegexBuilder::new(body)
        .case_insensitive(case_insensitive)
        .size_limit(256 * 1024)
        .build()
        .map(|_| ())
        .map_err(|e| e.to_string())
}

type CompiledTitleRegexMap = std::collections::HashMap<(String, bool), Option<regex::Regex>>;

static COMPILED_TITLE_REGEX_CACHE: std::sync::LazyLock<std::sync::Mutex<CompiledTitleRegexMap>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

fn get_cached_title_regex(sanitized: &str, case_insensitive: bool) -> Option<regex::Regex> {
    let key = (sanitized.to_string(), case_insensitive);
    if let Ok(guard) = COMPILED_TITLE_REGEX_CACHE.lock() {
        if let Some(cached) = guard.get(&key) {
            return cached.clone();
        }
    }
    let compiled = regex::RegexBuilder::new(sanitized)
        .case_insensitive(case_insensitive)
        .size_limit(256 * 1024)
        .build()
        .ok();
    if let Ok(mut guard) = COMPILED_TITLE_REGEX_CACHE.lock() {
        if guard.len() >= 512 {
            guard.clear();
        }
        guard.insert(key, compiled.clone());
    }
    compiled
}

pub fn rule_matches_window_title(
    pattern_raw: &str,
    title_is_regex: bool,
    caption: &str,
    caption_lower: &str,
) -> Option<usize> {
    let sub = pattern_raw.trim();
    if sub.is_empty() {
        return Some(0);
    }

    if title_is_regex {
        let (regex_body, case_insensitive) =
            extract_inline_regex_pattern(sub).unwrap_or((sub, true));
        let sanitized = regex_body.trim_matches('|').trim();
        if sanitized.is_empty() {
            return None;
        }
        if let Some(re) = get_cached_title_regex(sanitized, case_insensitive) {
            if let Some(mat) = re.find(caption) {
                if !mat.is_empty() || sanitized == "^$" {
                    return Some(mat.len().max(sanitized.len()).max(1));
                }
            }
        }
        return None;
    }

    if caption_lower.contains(&sub.to_lowercase()) {
        return Some(sub.len());
    }

    None
}

impl SwitcherState {
    /// When SD profiles are loaded and the user hasn't saved custom rules yet (or only had auto-aligned defaults),
    /// first check if full rules exist in host config or SD/cache `duckydeck_rules.json`, and otherwise align default rules.
    pub fn align_rules_with_profiles(&mut self, profiles: &[DuckyProfile]) {
        if profiles.len() <= 1 {
            return;
        }

        // 1. Check if host config or SD card rules bundle has a fuller rule set than what is currently in memory
        if let Some(disk_cfg) = Self::load_config_from_disk() {
            if disk_cfg.rules.len() > self.rules.len() && self.rules.len() <= 14 {
                self.enabled = disk_cfg.enabled;
                self.default_profile = Some(disk_cfg.default_profile.unwrap_or(1).max(1));
                self.rules = disk_cfg.rules;
                self.loaded_from_disk = true;
                let _ = self.save_to_disk(); // ignore: best-effort persist
                return;
            }
        }

        for candidate_dir in [
            crate::hardware::profile::profile_cache_dir(),
            crate::paths::host_data_dir().join("profile_cache"),
        ] {
            if let Some((en, fb, sd_rules)) = load_rules_bundle_from_dir(&candidate_dir, profiles) {
                if !self.loaded_from_disk
                    || (sd_rules.len() > self.rules.len() && self.rules.len() <= 14)
                {
                    self.enabled = en;
                    if fb.is_some() {
                        self.default_profile = fb;
                    }
                    self.rules = sd_rules;
                    self.loaded_from_disk = true;
                    let _ = self.save_to_disk(); // ignore: best-effort persist
                    return;
                }
            }
        }

        if self.loaded_from_disk {
            return;
        }

        let aligned = build_auto_aligned_rules(profiles);
        if !aligned.is_empty() {
            self.rules = aligned;
            if let Err(e) = self.save_to_disk() {
                crate::bridge::device_controller::record_system_event(
                    "WARN",
                    "CONFIG",
                    format!("Failed saving aligned auto-switch rules: {e}"),
                );
            }
        }
    }

    pub fn match_profile_for_window(&self, resource_class: &str, caption: &str) -> Option<u8> {
        match_profile_with_rules(&self.rules, self.default_profile, resource_class, caption)
    }
}

pub fn match_profile_with_rules(
    rules: &[AutoSwitchRule],
    default_profile: Option<u8>,
    resource_class: &str,
    caption: &str,
) -> Option<u8> {
    let lower_class = resource_class.trim().to_lowercase();
    if lower_class.is_empty() || lower_class == "none" {
        return default_profile;
    }
    let resolved_caption = resolve_full_browser_caption(resource_class, caption);
    let caption_lower = resolved_caption.to_lowercase();

    let mut best_match: Option<(usize, u8)> = None;

    for rule in rules {
        if !rule.enabled {
            continue;
        }
        if !rule_matches_window_class(&rule.resource_class, &lower_class) {
            continue;
        }

        let title_specificity = match rule.title_contains.as_ref().map(|s| s.trim()) {
            Some(sub) if !sub.is_empty() => match rule_matches_window_title(
                sub,
                rule.title_is_regex,
                &resolved_caption,
                &caption_lower,
            ) {
                Some(spec) => spec,
                None => continue,
            },
            _ => 0,
        };

        if best_match.is_none_or(|(best_len, _)| title_specificity > best_len) {
            best_match = Some((title_specificity, rule.target_profile));
        }
    }

    if let Some((_, target)) = best_match {
        return Some(target);
    }

    default_profile
}
