use hidapi::HidApi;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use zbus::{interface, Result as ZbusResult};

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
}

pub struct SwitcherState {
    pub enabled: bool,
    pub default_profile: Option<u8>,
    pub rules: Vec<AutoSwitchRule>,
    pub last_switched_profile: u8,
    pub picker_armed: bool,
}

impl Default for SwitcherState {
    fn default() -> Self {
        Self {
            enabled: true,
            default_profile: Some(1),
            rules: vec![
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "org.kde.kdenlive".into(),
                    title_contains: None,
                    target_profile: 2,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "blender".into(),
                    title_contains: None,
                    target_profile: 3,
                },
                AutoSwitchRule {
                    enabled: true,
                    resource_class: "firefox".into(),
                    title_contains: None,
                    target_profile: 4,
                },
            ],
            last_switched_profile: 0,
            picker_armed: false,
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
        if resource_class.is_empty() || resource_class == "plasmashell" {
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
                });
                return;
            }

            if state.enabled {
                let class_lower = resource_class.to_lowercase();
                let caption_lower = caption.to_lowercase();

                for rule in &state.rules {
                    if !rule.enabled {
                        continue;
                    }
                    let rule_class = rule.resource_class.to_lowercase();
                    let class_matches =
                        class_lower == rule_class || class_lower.contains(&rule_class);

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
                        if send_hid_switch_profile(target) {
                            state.last_switched_profile = target;
                        }
                    }
                }
            }
        }

        let _ = self.ui_event_tx.send(WindowChangeEvent {
            resource_class,
            resource_name,
            caption,
            matched_profile,
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

    let script_id: i32 = kwin_scripting
        .call("loadScript", &(&script_path, plugin_name))
        .await?;

    let script_obj_path = format!("/Scripting/Script{script_id}");
    let kwin_script = zbus::Proxy::new(
        &conn,
        "org.kde.KWin",
        script_obj_path.as_str(),
        "org.kde.kwin.Script",
    )
    .await?;

    kwin_script.call_noreply("run", &()).await?;

    std::future::pending::<()>().await;
    Ok(())
}

fn write_kwin6_hook_script() -> std::io::Result<String> {
    let js_code = r#"
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

if (workspace.windowActivated) {
    workspace.windowActivated.connect(sendWindow);
    sendWindow(workspace.activeWindow);
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
