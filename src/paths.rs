use std::env;
use std::fs::{self, DirBuilder};
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

const APP_DIR_NAME: &str = "duckydeck-studio";

pub fn is_flatpak() -> bool {
    Path::new("/.flatpak-info").exists() || env::var_os("FLATPAK_ID").is_some()
}

pub fn home_dir() -> Option<PathBuf> {
    for key in ["HOME", "USERPROFILE"] {
        if let Some(val) = env::var_os(key) {
            let p = PathBuf::from(&val);
            if !p.as_os_str().is_empty() && p.is_absolute() {
                return Some(p);
            }
        }
    }
    None
}

fn home_or_cwd() -> PathBuf {
    home_dir()
        .or_else(|| env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn xdg_config_home() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("APPDATA") {
        let p = PathBuf::from(appdata);
        if p.is_absolute() {
            return p;
        }
    }
    if let Ok(val) = env::var("XDG_CONFIG_HOME") {
        let trimmed = val.trim();
        if !trimmed.is_empty() && trimmed.starts_with('/') && !trimmed.contains(':') {
            return PathBuf::from(trimmed);
        }
    }
    home_or_cwd().join(".config")
}

pub fn xdg_data_home() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("LOCALAPPDATA").or_else(|| env::var_os("APPDATA")) {
        let p = PathBuf::from(appdata);
        if p.is_absolute() {
            return p;
        }
    }
    if let Ok(val) = env::var("XDG_DATA_HOME") {
        let trimmed = val.trim();
        if !trimmed.is_empty()
            && !trimmed.contains(':')
            && trimmed.starts_with('/')
            && !trimmed.starts_with("/var/lib/")
            && !trimmed.starts_with("/usr/")
        {
            return PathBuf::from(trimmed);
        }
    }
    home_or_cwd().join(".local").join("share")
}

pub fn xdg_cache_home() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(localappdata) = env::var_os("LOCALAPPDATA") {
        let p = PathBuf::from(localappdata);
        if p.is_absolute() {
            return p.join("Temp");
        }
    }
    if let Ok(val) = env::var("XDG_CACHE_HOME") {
        let trimmed = val.trim();
        if !trimmed.is_empty() && trimmed.starts_with('/') && !trimmed.contains(':') {
            return PathBuf::from(trimmed);
        }
    }
    home_or_cwd().join(".cache")
}

pub fn config_dir() -> PathBuf {
    xdg_config_home().join(APP_DIR_NAME)
}

pub fn host_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return config_dir();
    }
    #[cfg(not(target_os = "windows"))]
    home_or_cwd().join(".config").join(APP_DIR_NAME)
}

pub fn data_dir() -> PathBuf {
    xdg_data_home().join(APP_DIR_NAME)
}

pub fn host_data_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        return data_dir();
    }
    #[cfg(not(target_os = "windows"))]
    home_or_cwd()
        .join(".local")
        .join("share")
        .join(APP_DIR_NAME)
}

pub fn dekunukem_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("APPDATA") {
        return PathBuf::from(appdata)
            .join("dekuNukem")
            .join("duckypad_config");
    }
    home_or_cwd()
        .join(".local")
        .join("share")
        .join("duckypad_config")
}

pub fn dekunukem_autoswitcher_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    if let Some(appdata) = env::var_os("APPDATA") {
        return PathBuf::from(appdata)
            .join("dekuNukem")
            .join("duckypad_autoswitcher");
    }
    home_or_cwd()
        .join(".local")
        .join("share")
        .join("duckypad_autoswitcher")
}

pub fn cache_dir() -> PathBuf {
    xdg_cache_home().join(APP_DIR_NAME)
}

fn ensure_dir_0700(dir: &Path) -> bool {
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    builder.mode(0o700);
    if builder.create(dir).is_ok() {
        #[cfg(unix)]
        let _ = fs::set_permissions(dir, fs::Permissions::from_mode(0o700)); // ignore: best-effort mode enforcement after DirBuilder::mode(0o700)
        true
    } else {
        dir.is_dir()
    }
}

pub fn runtime_dir() -> PathBuf {
    if let Ok(val) = env::var("XDG_RUNTIME_DIR") {
        let trimmed = val.trim();
        if !trimmed.is_empty() && trimmed.starts_with('/') && !trimmed.contains(':') {
            let p = PathBuf::from(trimmed);
            if p.is_dir() || ensure_dir_0700(&p) {
                return p;
            }
        }
    }
    let fallback = data_dir().join("run");
    ensure_dir_0700(&fallback);
    fallback
}

pub fn kwin_runtime_dir() -> PathBuf {
    let dir = runtime_dir().join(APP_DIR_NAME);
    ensure_dir_0700(&dir);
    dir
}

pub fn sanitize_xdg_env() {
    let resolved_data_home = xdg_data_home();
    let needs_fix = match env::var("XDG_DATA_HOME") {
        Ok(val) => {
            let trimmed = val.trim();
            trimmed.is_empty()
                || trimmed.contains(':')
                || !trimmed.starts_with('/')
                || trimmed.starts_with("/var/lib/")
                || trimmed.starts_with("/usr/")
        }
        Err(_) => true,
    };
    if needs_fix {
        env::set_var("XDG_DATA_HOME", &resolved_data_home);
    }

    let current_dirs = env::var("XDG_DATA_DIRS").unwrap_or_default();
    if !current_dirs.split(':').any(|p| p == "/usr/share") {
        let mut parts: Vec<String> = current_dirs
            .split(':')
            .filter(|s| !s.is_empty() && *s != ".")
            .map(|s| s.to_string())
            .collect();
        let flatpak_user_share = resolved_data_home
            .join("flatpak/exports/share")
            .to_string_lossy()
            .to_string();
        for std_dir in [
            flatpak_user_share,
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

/// Strip `file://` prefix from a QML FileDialog URI and normalize Windows `/C:/...` drive paths.
pub fn clean_file_uri_to_path_str(raw: &str) -> String {
    let trimmed = raw.trim();
    let without_scheme = trimmed.strip_prefix("file://").unwrap_or(trimmed).trim();
    let bytes = without_scheme.as_bytes();
    if bytes.len() >= 3 && bytes[0] == b'/' && bytes[1].is_ascii_alphabetic() && bytes[2] == b':' {
        without_scheme[1..].to_string()
    } else {
        without_scheme.to_string()
    }
}

/// Convert a local filesystem path into a valid `file://` URI for QML `Image` / `Icon`.
pub fn path_to_file_uri(path: &Path) -> String {
    let s = path.to_string_lossy().replace('\\', "/");
    if s.starts_with('/') {
        format!("file://{s}")
    } else {
        format!("file:///{s}")
    }
}

/// Open a URL or local file path in the user's default OS handler.
pub fn open_url_or_path_in_os(target: &str) -> std::io::Result<std::process::Child> {
    use std::process::{Command, Stdio};
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd.exe")
            .args(["/C", "start", "", target])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
    }
    #[cfg(not(target_os = "windows"))]
    {
        if is_flatpak() {
            // Try flatpak-spawn --host xdg-open first (works for both URLs and host file paths);
            // if the user revoked org.freedesktop.Flatpak in Flatseal, fall back to the sandbox's
            // portal-backed /usr/bin/xdg-open.
            let is_web_url =
                target.starts_with("http://") || target.starts_with("https://") || target.starts_with("mailto:");
            if !is_web_url {
                if let Ok(child) = Command::new("flatpak-spawn")
                    .args(["--host", "xdg-open", target])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                {
                    return Ok(child);
                }
            }
            Command::new("xdg-open")
                .arg(target)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
        } else {
            Command::new("xdg-open")
                .arg(target)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
        }
    }
}

/// Atomically writes `contents` to `path` with owner-only `0600` permissions on Unix.
pub fn write_private_file_atomic(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        ensure_dir_0700(parent);
    }
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("config.json");
    let pid = std::process::id();
    let tmp_path = path.with_file_name(format!(".{file_name}.tmp.{pid}"));

    let mut opts = fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    #[cfg(unix)]
    opts.mode(0o600);

    {
        let mut f = opts.open(&tmp_path)?;
        f.write_all(contents)?;
        let _ = f.sync_all();
    }
    #[cfg(unix)]
    {
        let _ = fs::set_permissions(&tmp_path, fs::Permissions::from_mode(0o600));
    }

    if let Err(e) = fs::rename(&tmp_path, path) {
        let _ = fs::remove_file(&tmp_path);
        return Err(e);
    }
    #[cfg(unix)]
    {
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o600));
    }
    Ok(())
}

/// Constant-time string equality check for authentication tokens and cryptographic digests.
pub fn constant_time_str_eq(a: &str, b: &str) -> bool {
    let ab = a.as_bytes();
    let bb = b.as_bytes();
    if ab.len() != bb.len() || ab.is_empty() {
        return false;
    }
    let mut diff = 0u8;
    for (&x, &y) in ab.iter().zip(bb.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
    0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
    0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
    0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
    0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
    0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
    0xc67178f2,
];

/// Computes a standard FIPS 180-4 SHA-256 digest over `data`.
pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut padded = Vec::with_capacity(data.len() + 72);
    padded.extend_from_slice(data);
    padded.push(0x80);
    while (padded.len() % 64) != 56 {
        padded.push(0x00);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// Computes a lowercase 64-character hexadecimal SHA-256 digest over `data`.
pub fn sha256_hex(data: &[u8]) -> String {
    let digest = sha256_bytes(data);
    let mut out = String::with_capacity(64);
    for b in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{b:02x}");
    }
    out
}

/// Computes an RFC 2104 HMAC-SHA256 lowercase hexadecimal tag over `message` using `key`.
pub fn hmac_sha256_hex(key: &[u8], message: &[u8]) -> String {
    let mut key_block = [0u8; 64];
    if key.len() > 64 {
        let hashed = sha256_bytes(key);
        key_block[..32].copy_from_slice(&hashed);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }

    let mut ipad = [0x36u8; 64];
    let mut opad = [0x5cu8; 64];
    for i in 0..64 {
        ipad[i] ^= key_block[i];
        opad[i] ^= key_block[i];
    }

    let mut inner_input = Vec::with_capacity(64 + message.len());
    inner_input.extend_from_slice(&ipad);
    inner_input.extend_from_slice(message);
    let inner_hash = sha256_bytes(&inner_input);

    let mut outer_input = Vec::with_capacity(96);
    outer_input.extend_from_slice(&opad);
    outer_input.extend_from_slice(&inner_hash);
    sha256_hex(&outer_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_and_hmac_sha256_nist_vectors() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        // RFC 4231 Test Case 2
        assert_eq!(
            hmac_sha256_hex(b"Jefe", b"what do ya want for nothing?"),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_write_private_file_atomic_sets_0600_mode() {
        let dir = std::env::temp_dir().join(format!("duckydeck_priv_test_{}", std::process::id()));
        let file = dir.join("secret.json");
        write_private_file_atomic(&file, br#"{"token":"secret"}"#).expect("write private file");
        let meta = fs::metadata(&file).expect("stat private file");
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
        let _ = fs::remove_dir_all(&dir);
    }
}
