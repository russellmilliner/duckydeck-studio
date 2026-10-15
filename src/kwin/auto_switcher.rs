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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedSwitcherConfig {
    pub enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
}

pub struct SwitcherState {
    pub enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    pub last_switched_profile: u8,
    pub picker_armed: bool,
    pub loaded_from_disk: bool,
}

impl Default for SwitcherState {
    fn default() -> Self {
        if let Some(cfg) = Self::load_config_from_disk() {
            return Self {
                enabled: cfg.enabled,
                default_profile: cfg.default_profile,
                rules: cfg.rules,
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
        };
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|e| format!("Failed serializing KWin rules: {e}"))?;
        fs::write(&path, json).map_err(|e| format!("Failed writing {}: {e}", path.display()))
    }

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
                let caption_lower = caption.to_lowercase();

                for rule in &state.rules {
                    if !rule.enabled {
                        continue;
                    }
                    let rule_class = rule.resource_class.to_lowercase();
                    let class_matches =
                        lower_class == rule_class || lower_class.contains(&rule_class);

                    let title_matches = match &rule.title_contains {
                        Some(sub) if !sub.trim().is_empty() => {
                            caption_lower.contains(&sub.to_lowercase())
                        }
                        _ => true,
                    };

                    if class_matches && title_matches {
                        matched_profile = Some(rule.target_profile);
                        break;
                    }
                }

                if matched_profile.is_none() {
                    matched_profile = state.default_profile;
                }

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
