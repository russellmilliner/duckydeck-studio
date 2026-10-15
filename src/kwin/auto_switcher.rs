use hidapi::HidApi;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use zbus::{interface, Result as ZbusResult};

use crate::hardware::storage::DuckyProfile;

const DUCKYPAD_VID: u16 = 0x0483;
const PID_DUCKYPAD_PRO: u16 = 0xd11d;
const PID_DUCKYPAD_2020: u16 = 0xd11c;
const HID_CMD_GOTO_PROFILE_BY_NUM: u8 = 0x01;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoSwitchRule {
    pub enabled: bool,
    pub resource_class: String,
    pub title_contains: Option<String>,
    pub target_profile: u8,
}

#[derive(Debug, Clone)]
pub struct WindowChangeEvent {
    pub resource_class: String,
    pub resource_name: String,
    pub caption: String,
    pub matched_profile: Option<u8>,
    pub picked: bool,
}

fn default_tray_icon_mode() -> String {
    "auto".to_string()
}

fn default_app_theme_mode() -> String {
    "system".to_string()
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedSwitcherConfig {
    pub enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    #[serde(default = "default_tray_icon_mode")]
    pub tray_icon_mode: String,
    #[serde(default = "default_app_theme_mode")]
    pub app_theme_mode: String,
    #[serde(default = "default_true")]
    pub close_to_tray: bool,
}

pub struct SwitcherState {
    pub enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    pub tray_icon_mode: String,
    pub app_theme_mode: String,
    pub close_to_tray: bool,
    pub last_switched_profile: u8,
    pub picker_armed: bool,
    pub loaded_from_disk: bool,
}

impl Default for SwitcherState {
    fn default() -> Self {
        if let Some(cfg) = Self::load_config_from_disk() {
            let mode = match cfg.tray_icon_mode.as_str() {
                "dark" | "light" | "auto" => cfg.tray_icon_mode,
                _ => "auto".to_string(),
            };
            let theme = match cfg.app_theme_mode.as_str() {
                "dark" | "light" | "system" => cfg.app_theme_mode,
                _ => "system".to_string(),
            };
            return Self {
                enabled: cfg.enabled,
                default_profile: cfg.default_profile,
                rules: cfg.rules,
                tray_icon_mode: mode,
                app_theme_mode: theme,
                close_to_tray: cfg.close_to_tray,
                last_switched_profile: 0,
                picker_armed: false,
                loaded_from_disk: true,
            };
        }

        Self {
            enabled: true,
            default_profile: Some(1),
            rules: vec![
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "org.kde.dolphin".into(),
                    title_contains: None,
                    target_profile: 2,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "firefox".into(),
                    title_contains: None,
                    target_profile: 3,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "code".into(),
                    title_contains: None,
                    target_profile: 4,
                },
            ],
            tray_icon_mode: "auto".to_string(),
            app_theme_mode: "system".to_string(),
            close_to_tray: true,
            last_switched_profile: 0,
            picker_armed: false,
            loaded_from_disk: false,
        }
    }
}

impl SwitcherState {
    pub fn config_file_path() -> PathBuf {
        let base = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                PathBuf::from(home).join(".config")
            });
        base.join("duckydeck-studio").join("kwin_rules.json")
    }

    pub fn load_config_from_disk() -> Option<PersistedSwitcherConfig> {
        let path = Self::config_file_path();
        let raw = fs::read_to_string(path).ok()?;
        serde_json::from_str(&raw).ok()
    }

    pub fn save_to_disk(&self) -> Result<(), String> {
        let path = Self::config_file_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed creating config dir: {e}"))?;
        }
        let payload = PersistedSwitcherConfig {
            enabled: self.enabled,
            default_profile: self.default_profile,
            rules: self.rules.clone(),
            tray_icon_mode: self.tray_icon_mode.clone(),
            app_theme_mode: self.app_theme_mode.clone(),
            close_to_tray: self.close_to_tray,
        };
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("Failed serializing KWin rules: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("Failed writing {}: {e}", path.display()))
    }
}

pub fn resolve_app_icon_for_class(resource_class: &str) -> String {
    let trimmed = resource_class.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    let lower = trimmed.to_ascii_lowercase();
    let short_lower = lower
        .rsplit('.')
        .next()
        .unwrap_or(lower.as_str())
        .to_string();

    let mut search_dirs = Vec::new();
    if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
        search_dirs.push(PathBuf::from(xdg_data).join("applications"));
    } else if let Ok(home) = std::env::var("HOME") {
        search_dirs.push(PathBuf::from(&home).join(".local/share/applications"));
        search_dirs.push(PathBuf::from(&home).join(".local/share/flatpak/exports/share/applications"));
    }
    search_dirs.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
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
            }
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

pub fn autostart_desktop_path() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".config")
        });
    base.join("autostart")
        .join("io.github.duckydeck.Studio.desktop")
}

pub fn is_kde_autostart_enabled() -> bool {
    let path = autostart_desktop_path();
    let Ok(content) = fs::read_to_string(&path) else {
        return false;
    };
    for line in content.lines() {
        let t = line.trim();
        if t.eq_ignore_ascii_case("Hidden=true")
            || t.eq_ignore_ascii_case("X-GNOME-Autostart-enabled=false")
        {
            return false;
        }
    }
    true
}

pub fn sync_local_desktop_and_icon(use_light_on_dark_icon: bool) {
    let data_home = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".local").join("share")
        });

    let icon_dir = data_home
        .join("icons")
        .join("hicolor")
        .join("scalable")
        .join("apps");
    if fs::create_dir_all(&icon_dir).is_ok() {
        let svg_content = if use_light_on_dark_icon {
            include_str!("../qml/icons/tray-icon-dark.svg")
        } else {
            include_str!("../qml/icons/tray-icon-light.svg")
        };
        let _ = fs::write(icon_dir.join("io.github.duckydeck.Studio.svg"), svg_content);
    }

    let apps_dir = data_home.join("applications");
    if fs::create_dir_all(&apps_dir).is_ok() {
        let exe_path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "duckydeck-studio".to_string());
        let desktop_content = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=DuckyDeck Studio\n\
             Comment=KDE Plasma 6 Configurator & Auto-Switcher for duckyPad Pro\n\
             Exec=\"{exe_path}\"\n\
             Icon=io.github.duckydeck.Studio\n\
             Terminal=false\n\
             Categories=Utility;HardwareSettings;Qt;KDE;\n\
             StartupWMClass=io.github.duckydeck.Studio\n"
        );
        let _ = fs::write(
            apps_dir.join("io.github.duckydeck.Studio.desktop"),
            desktop_content,
        );
    }
}

pub fn set_kde_autostart_enabled(enabled: bool) -> Result<PathBuf, String> {
    let path = autostart_desktop_path();
    if enabled {
        sync_local_desktop_and_icon(true);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed creating autostart directory: {e}"))?;
        }
        let exe_path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "duckydeck-studio".to_string());
        let desktop_entry = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=DuckyDeck Studio\n\
             Comment=KDE Plasma 6 Configurator & Auto-Switcher for duckyPad Pro\n\
             Exec=\"{exe_path}\" --minimized\n\
             Icon=io.github.duckydeck.Studio\n\
             Terminal=false\n\
             Categories=Utility;HardwareSettings;Qt;KDE;\n\
             StartupWMClass=io.github.duckydeck.Studio\n\
             X-KDE-AutostartScript=false\n\
             X-KDE-autostart-after=panel\n"
        );
        fs::write(&path, desktop_entry)
            .map_err(|e| format!("Failed writing {}: {e}", path.display()))?;
        Ok(path)
    } else {
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed removing {}: {e}", path.display()))?;
        }
        Ok(path)
    }
}

impl SwitcherState {

    /// When SD profiles are loaded for the first time and the user hasn't saved custom rules yet,
    /// automatically align default KWin rules with matching profile names on their duckyPad.
    pub fn align_rules_with_profiles(&mut self, profiles: &[DuckyProfile]) {
        if self.loaded_from_disk || profiles.len() <= 1 {
            return;
        }

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
            ("YouTube", "chrome", Some("YouTube")),
            ("Gmail", "chrome", Some("Gmail")),
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
                    target_profile: prof.number,
                });
            }
        }

        if !aligned.is_empty() {
            self.rules = aligned;
        }
    }

    pub fn match_profile_for_window(&self, resource_class: &str, caption: &str) -> Option<u8> {
        let lower_class = resource_class.trim().to_lowercase();
        if lower_class.is_empty() || lower_class == "none" {
            return None;
        }
        let caption_lower = caption.to_lowercase();

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            let rule_class = rule.resource_class.to_lowercase();
            let class_matches = lower_class == rule_class || lower_class.contains(&rule_class);

            let title_matches = match &rule.title_contains {
                Some(sub) if !sub.trim().is_empty() => caption_lower.contains(&sub.to_lowercase()),
                _ => true,
            };

            if class_matches && title_matches {
                return Some(rule.target_profile);
            }
        }

        self.default_profile
    }
}

pub struct KWinBridgeService {
    state: Arc<Mutex<SwitcherState>>,
    ui_event_tx: mpsc::UnboundedSender<WindowChangeEvent>,
}

#[interface(name = "io.github.duckydeck.KWinBridge")]
impl KWinBridgeService {
    fn notify_active_window(
        &mut self,
        resource_class: String,
        resource_name: String,
        caption: String,
    ) {
        let lower_class = resource_class.to_lowercase();
        if resource_class.is_empty()
            || lower_class == "plasmashell"
            || lower_class.contains("duckydeck")
        {
            return;
        }

        let mut matched_profile: Option<u8> = None;

        if let Ok(mut state) = self.state.lock() {
            if state.picker_armed {
                state.picker_armed = false;
                let _ = self.ui_event_tx.send(WindowChangeEvent {
                    resource_class,
                    resource_name,
                    caption,
                    matched_profile: None,
                    picked: true,
                });
                return;
            }

            if state.enabled {
                matched_profile = state.match_profile_for_window(&resource_class, &caption);

                if let Some(target) = matched_profile {
                    if target != state.last_switched_profile {
                        let _ = send_hid_switch_profile(target);
                        state.last_switched_profile = target;
                    }
                }
            }
        }

        let _ = self.ui_event_tx.send(WindowChangeEvent {
            resource_class,
            resource_name,
            caption,
            matched_profile,
            picked: false,
        });
    }
}

pub fn send_hid_switch_profile(profile_number: u8) -> bool {
    let Ok(api) = HidApi::new() else {
        return false;
    };

    for pid in [PID_DUCKYPAD_PRO, PID_DUCKYPAD_2020] {
        if let Ok(device) = api.open(DUCKYPAD_VID, pid) {
            let mut tx = [0u8; 64];
            tx[0] = 0x00;
            tx[1] = 0x77;
            tx[2] = HID_CMD_GOTO_PROFILE_BY_NUM;
            tx[3] = profile_number.clamp(1, 64);

            if device.write(&tx).is_ok() {
                return true;
            }
        }
    }
    false
}

pub async fn pick_window_via_kwin_dbus() -> std::result::Result<WindowChangeEvent, String> {
    let conn = zbus::Connection::session()
        .await
        .map_err(|e| format!("D-Bus session error: {e}"))?;

    let kwin_proxy = zbus::Proxy::new(&conn, "org.kde.KWin", "/KWin", "org.kde.KWin")
        .await
        .map_err(|e| format!("KWin proxy error: {e}"))?;

    let info: std::collections::HashMap<String, zbus::zvariant::OwnedValue> = kwin_proxy
        .call("queryWindowInfo", &())
        .await
        .map_err(|e| format!("Window picker canceled or failed: {e}"))?;

    let extract_str = |key: &str| -> String {
        info.get(key)
            .and_then(|v| <&str>::try_from(v).ok().map(|s| s.to_string()))
            .unwrap_or_default()
    };

    let resource_class = extract_str("resourceClass");
    let resource_name = extract_str("resourceName");
    let caption = extract_str("caption");

    if resource_class.is_empty() && resource_name.is_empty() {
        return Err("Window picker canceled".to_string());
    }

    Ok(WindowChangeEvent {
        resource_class: if !resource_class.is_empty() {
            resource_class
        } else {
            resource_name.clone()
        },
        resource_name,
        caption,
        matched_profile: None,
        picked: true,
    })
}

pub async fn run_kwin_auto_switcher(
    state: Arc<Mutex<SwitcherState>>,
    ui_event_tx: mpsc::UnboundedSender<WindowChangeEvent>,
) -> ZbusResult<()> {
    let service = KWinBridgeService { state, ui_event_tx };
    let conn = zbus::conn::Builder::session()?
        .name("io.github.duckydeck.Studio")?
        .serve_at("/KWinBridge", service)?
        .build()
        .await?;

    let script_path = write_kwin6_hook_script()
        .map_err(|e| zbus::Error::Failure(format!("Failed to write KWin JS hook: {e}")))?;

    let kwin_scripting = zbus::Proxy::new(
        &conn,
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
    )
    .await?;

    let plugin_name = "duckydeck_kwin6_autoswitcher";
    let _: std::result::Result<bool, _> =
        kwin_scripting.call("unloadScript", &(plugin_name,)).await;

    let mut script_id: i32 = kwin_scripting
        .call("loadScript", &(&script_path, plugin_name))
        .await?;

    if script_id < 0 {
        let fallback_name = format!("duckydeck_kwin6_autoswitcher_{}", std::process::id());
        let _: std::result::Result<bool, _> = kwin_scripting
            .call("unloadScript", &(fallback_name.as_str(),))
            .await;
        script_id = kwin_scripting
            .call("loadScript", &(&script_path, fallback_name.as_str()))
            .await?;
    }

    if script_id >= 0 {
        let script_obj_path = format!("/Scripting/Script{script_id}");
        if let Ok(kwin_script) = zbus::Proxy::new(
            &conn,
            "org.kde.KWin",
            script_obj_path.as_str(),
            "org.kde.kwin.Script",
        )
        .await
        {
            let _ = kwin_script.call_noreply("run", &()).await;
        }
    }

    std::future::pending::<()>().await;
    Ok(())
}

fn write_kwin6_hook_script() -> std::io::Result<String> {
    let js_code = r#"
function isIgnoredWindow(win) {
    if (!win) return true;
    var cls = (win.resourceClass ? win.resourceClass.toString() : "").toLowerCase();
    return cls === "" || cls === "plasmashell" || cls.indexOf("duckydeck") !== -1;
}

function sendWindow(win) {
    if (!win) return;
    callDBus(
        "io.github.duckydeck.Studio",
        "/KWinBridge",
        "io.github.duckydeck.KWinBridge",
        "NotifyActiveWindow",
        win.resourceClass ? win.resourceClass.toString() : "",
        win.resourceName ? win.resourceName.toString() : "",
        win.caption ? win.caption.toString() : ""
    );
}

function sendInitialTopWindow() {
    if (workspace.activeWindow && !isIgnoredWindow(workspace.activeWindow)) {
        sendWindow(workspace.activeWindow);
        return;
    }
    var order = workspace.stackingOrder;
    if (order && order.length) {
        for (var i = order.length - 1; i >= 0; --i) {
            var w = order[i];
            if (!isIgnoredWindow(w) && (!w.minimized)) {
                sendWindow(w);
                return;
            }
        }
    }
}

if (workspace.windowActivated) {
    workspace.windowActivated.connect(sendWindow);
    sendInitialTopWindow();
} else if (workspace.clientActivated) {
    workspace.clientActivated.connect(sendWindow);
    sendWindow(workspace.activeClient);
}
"#;

    let runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".to_string());
    let file_path = format!("{runtime_dir}/duckydeck-kwin6-hook.js");

    let mut file = std::fs::File::create(&file_path)?;
    file.write_all(js_code.as_bytes())?;
    Ok(file_path)
}
