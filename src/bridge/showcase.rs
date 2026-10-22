use cxx_qt_lib::QString;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::bridge::device_controller::{
    qobject, DeviceControllerRust, APP_AUTHOR, APP_GITHUB_URL, APP_VERSION,
};
use crate::bridge::event_log::{
    record_system_event, serialize_system_event_log_json, system_event_log_store,
};
#[cfg(feature = "demo")]
use crate::hardware::storage::{
    apply_app_launch_config_to_script, apply_web_api_config_to_script, AppLaunchConfig,
    WebApiConfig,
};
use crate::hardware::storage::{DuckyKeySlot, DuckyProfile};
use crate::kwin::auto_switcher::generate_rest_api_token;
use crate::kwin::auto_switcher::{strip_browser_title_suffix, AutoSwitchRule, SwitcherState};

pub fn create_default_demo_profile() -> DuckyProfile {
    let mut profile = DuckyProfile {
        number: 1,
        name: "Blender Sculpt".to_string(),
        bg_color: [0, 170, 255],
        keydown_color: Some([255, 255, 255]),
        dim_unused_keys: true,
        ..Default::default()
    };

    type DefaultSlotSpec<'a> = (usize, &'a str, &'a str, [u8; 3], &'a str, &'a str);
    let defaults: &[DefaultSlotSpec<'_>] = &[
        (
            1,
            "Front",
            "View",
            [0, 170, 255],
            "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Front View\nOLED_UPDATE\nKP_1\n",
            "",
        ),
        (
            2,
            "Side",
            "View",
            [0, 170, 255],
            "DEFAULTDELAY 20\nKP_3\n",
            "",
        ),
        (
            3,
            "Top",
            "View",
            [0, 170, 255],
            "DEFAULTDELAY 20\nKP_7\n",
            "",
        ),
        (
            4,
            "Wire",
            "Frame",
            [0, 220, 200],
            "DEFAULTDELAY 20\nKEYDOWN z\n",
            "KEYUP z\n",
        ),
        (
            5,
            "Grab",
            "Brush",
            [40, 220, 100],
            "DEFAULTDELAY 20\nSTRING g\n",
            "",
        ),
        (
            6,
            "Clay",
            "Strips",
            [40, 220, 100],
            "DEFAULTDELAY 20\nSTRING c\n",
            "",
        ),
        (
            7,
            "Crease",
            "Tool",
            [40, 220, 100],
            "DEFAULTDELAY 20\nSHIFT c\n",
            "",
        ),
        (
            8,
            "Smooth",
            "Hold",
            [40, 220, 100],
            "KEYDOWN SHIFT\n",
            "KEYUP SHIFT\n",
        ),
        (
            9,
            "Draw",
            "Sharp",
            [40, 220, 100],
            "DEFAULTDELAY 20\nSTRING x\n",
            "",
        ),
        (
            10,
            "Inflat",
            "Brush",
            [40, 220, 100],
            "DEFAULTDELAY 20\nSTRING i\n",
            "",
        ),
        (
            11,
            "Pinch",
            "Edge",
            [40, 220, 100],
            "DEFAULTDELAY 20\nSTRING p\n",
            "",
        ),
        (
            12,
            "Mask",
            "Lasso",
            [255, 180, 0],
            "DEFAULTDELAY 20\nCONTROL SHIFT LCLICK\n",
            "",
        ),
        (
            13,
            "DynTop",
            "Toggle",
            [255, 140, 0],
            "DEFAULTDELAY 20\nCONTROL d\n",
            "",
        ),
        (
            14,
            "Voxel",
            "Remesh",
            [255, 140, 0],
            "DEFAULTDELAY 20\nCONTROL r\n",
            "",
        ),
        (
            15,
            "Invert",
            "Mask",
            [255, 180, 0],
            "DEFAULTDELAY 20\nCONTROL i\n",
            "",
        ),
        (
            16,
            "Clear",
            "Mask",
            [255, 180, 0],
            "DEFAULTDELAY 20\nALT m\n",
            "",
        ),
        (
            17,
            "Undo",
            "Step",
            [255, 60, 60],
            "DEFAULTDELAY 20\nCONTROL z\n",
            "",
        ),
        (
            18,
            "Redo",
            "Step",
            [255, 60, 60],
            "DEFAULTDELAY 20\nCONTROL SHIFT z\n",
            "",
        ),
        (
            19,
            "Auto",
            "Click",
            [180, 80, 255],
            "WHILE _SW_BITFIELD != 0\n    LMOUSE\n    DELAY 50\nEND_WHILE\n",
            "",
        ),
        (
            20,
            "Save",
            "Incr+",
            [180, 80, 255],
            "DEFAULTDELAY 20\nCONTROL ALT s\n",
            "",
        ),
        (21, "Size -", "", [56, 189, 248], "STRING [\n", ""),
        (22, "Size +", "", [56, 189, 248], "STRING ]\n", ""),
        (23, "Invert", "", [56, 189, 248], "CONTROL i\n", ""),
        (24, "Zoom -", "", [56, 189, 248], "KP_MINUS\n", ""),
        (25, "Zoom +", "", [56, 189, 248], "KP_PLUS\n", ""),
        (26, "Frame", "", [56, 189, 248], "KP_DOT\n", ""),
    ];

    for &(slot_idx, l1, l2, rgb, press, release) in defaults {
        profile.slots.insert(
            slot_idx,
            DuckyKeySlot {
                slot: slot_idx,
                label_line1: l1.to_string(),
                label_line2: l2.to_string(),
                widget_label_line1: String::new(),
                widget_label_line2: String::new(),
                custom_rgb: Some(rgb),
                allow_abort: false,
                disable_autorepeat: false,
                on_press_script: press.to_string(),
                on_release_script: release.to_string(),
            },
        );
    }

    profile
}

#[cfg(not(feature = "demo"))]
pub fn create_showcase_profiles_and_rules() -> (Vec<DuckyProfile>, Vec<AutoSwitchRule>) {
    (vec![create_default_demo_profile()], Vec::new())
}

#[cfg(feature = "demo")]
pub fn create_showcase_profiles_and_rules() -> (Vec<DuckyProfile>, Vec<AutoSwitchRule>) {
    fn build_profile(
        number: u8,
        name: &str,
        bg_color: [u8; 3],
        slots: &[(usize, &str, &str, [u8; 3], &str, &str)],
    ) -> DuckyProfile {
        let mut p = DuckyProfile {
            number,
            name: name.to_string(),
            folder_name_override: Some(format!("profile_{}", name.replace(' ', "_"))),
            bg_color,
            keydown_color: Some([255, 255, 255]),
            dim_unused_keys: true,
            ..Default::default()
        };
        for &(slot_idx, l1, l2, rgb, press, release) in slots {
            p.slots.insert(
                slot_idx,
                DuckyKeySlot {
                    slot: slot_idx,
                    label_line1: l1.to_string(),
                    label_line2: l2.to_string(),
                    widget_label_line1: String::new(),
                    widget_label_line2: String::new(),
                    custom_rgb: Some(rgb),
                    allow_abort: press.contains("WHILE"),
                    disable_autorepeat: false,
                    on_press_script: press.to_string(),
                    on_release_script: release.to_string(),
                },
            );
        }
        p
    }

    // 1. Dolphin File Manager
    let dolphin = build_profile(
            1,
            "Dolphin",
            [0, 170, 255],
            &[
                (1, "New", "Tab", [0, 190, 255], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT New Dolphin Tab\nOLED_UPDATE\nCONTROL t\n", ""),
                (2, "Split", "View", [0, 190, 255], "DEFAULTDELAY 20\nF3\n", ""),
                (3, "Term", "Panel", [16, 185, 129], "DEFAULTDELAY 20\nF4\n", ""),
                (4, "Hidden", "Files", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL h\n", ""),
                (5, "Copy", "Path", [250, 204, 21], "DEFAULTDELAY 20\nCONTROL ALT c\n", ""),
                (6, "Filter", "Bar", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL i\n", ""),
                (7, "Search", "Files", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL f\n", ""),
                (8, "New", "Folder", [34, 197, 94], "DEFAULTDELAY 20\nF10\n", ""),
                (9, "Rename", "Item", [249, 115, 22], "DEFAULTDELAY 20\nF2\n", ""),
                (10, "Props", "Dialog", [168, 85, 247], "DEFAULTDELAY 20\nALT RETURN\n", ""),
                (11, "Back", "Dir", [99, 102, 241], "DEFAULTDELAY 20\nALT LEFT\n", ""),
                (12, "Up", "Folder", [99, 102, 241], "DEFAULTDELAY 20\nALT UP\n", ""),
                (13, "Fwd", "Dir", [99, 102, 241], "DEFAULTDELAY 20\nALT RIGHT\n", ""),
                (14, "Home", "Folder", [99, 102, 241], "DEFAULTDELAY 20\nALT HOME\n", ""),
                (15, "Move", "Trash", [239, 68, 68], "DEFAULTDELAY 20\nDELETE\n", ""),
                (16, "Open", "Konsole", [16, 185, 129], "DEFAULTDELAY 20\nSHIFT F4\n", ""),
                (17, "Select", "All", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL a\n", ""),
                (18, "Invert", "Select", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL SHIFT a\n", ""),
                (19, "Preview", "Pane", [168, 85, 247], "DEFAULTDELAY 20\nF12\n", ""),
                (20, "Close", "Tab", [239, 68, 68], "DEFAULTDELAY 20\nCONTROL w\n", ""),
                (21, "Icon -", "", [56, 189, 248], "CONTROL MINUS\n", ""),
                (22, "Icon +", "", [56, 189, 248], "CONTROL EQUAL\n", ""),
                (23, "Reset", "", [56, 189, 248], "CONTROL 0\n", ""),
                (24, "Tab <", "", [56, 189, 248], "CONTROL PAGEUP\n", ""),
                (25, "Tab >", "", [56, 189, 248], "CONTROL PAGEDOWN\n", ""),
                (26, "Reload", "", [56, 189, 248], "F5\n", ""),
            ],
        );

    // 2. Google Chrome
    let chrome = build_profile(
            2,
            "Google Chrome",
            [66, 133, 244],
            &[
                (1, "New", "Tab", [66, 133, 244], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Chrome: New Tab\nOLED_UPDATE\nCONTROL t\n", ""),
                (2, "Reopen", "Closed", [52, 168, 83], "DEFAULTDELAY 20\nCONTROL SHIFT t\n", ""),
                (3, "Close", "Tab", [234, 67, 53], "DEFAULTDELAY 20\nCONTROL w\n", ""),
                (4, "Incog", "Window", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL SHIFT n\n", ""),
                (5, "Dev", "Tools", [251, 188, 5], "DEFAULTDELAY 20\nCONTROL SHIFT i\n", ""),
                (6, "Address", "Bar", [66, 133, 244], "DEFAULTDELAY 20\nCONTROL l\n", ""),
                (7, "Find", "Page", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL f\n", ""),
                (8, "Book", "Mark", [251, 188, 5], "DEFAULTDELAY 20\nCONTROL d\n", ""),
                (9, "History", "List", [99, 102, 241], "DEFAULTDELAY 20\nCONTROL h\n", ""),
                (10, "Down", "Loads", [52, 168, 83], "DEFAULTDELAY 20\nCONTROL j\n", ""),
                (11, "Hard", "Reload", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT r\n", ""),
                (12, "Mute", "Tab", [234, 67, 53], "DEFAULTDELAY 20\nALT m\n", ""),
                (13, "Back", "Page", [66, 133, 244], "DEFAULTDELAY 20\nALT LEFT\n", ""),
                (14, "Next", "Page", [66, 133, 244], "DEFAULTDELAY 20\nALT RIGHT\n", ""),
                (15, "Tab", "Search", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT a\n", ""),
                (16, "Source", "View", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL u\n", ""),
                (17, "Full", "Screen", [52, 168, 83], "DEFAULTDELAY 20\nF11\n", ""),
                (18, "Clear", "Data", [234, 67, 53], "DEFAULTDELAY 20\nCONTROL SHIFT DELETE\n", ""),
                (19, "Task", "Mgr", [251, 188, 5], "DEFAULTDELAY 20\nSHIFT ESCAPE\n", ""),
                (20, "Print", "PDF", [14, 165, 233], "DEFAULTDELAY 20\nCONTROL p\n", ""),
                (21, "Tab <", "", [66, 133, 244], "CONTROL SHIFT TAB\n", ""),
                (22, "Tab >", "", [66, 133, 244], "CONTROL TAB\n", ""),
                (23, "Reload", "", [66, 133, 244], "F5\n", ""),
                (24, "Zoom -", "", [52, 168, 83], "CONTROL MINUS\n", ""),
                (25, "Zoom +", "", [52, 168, 83], "CONTROL EQUAL\n", ""),
                (26, "100%", "", [52, 168, 83], "CONTROL 0\n", ""),
            ],
        );

    // 3. VS Code
    let vscode = build_profile(
            3,
            "VS Code",
            [0, 122, 204],
            &[
                (1, "Cmd", "Palette", [0, 152, 255], "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Command Palette\nOLED_UPDATE\nCONTROL SHIFT p\n", ""),
                (2, "Quick", "Open", [0, 152, 255], "DEFAULTDELAY 20\nCONTROL p\n", ""),
                (3, "Toggle", "Term", [16, 185, 129], "DEFAULTDELAY 20\nCONTROL GRAVE\n", ""),
                (4, "Build", "Task", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT b\n", ""),
                (5, "Format", "Code", [52, 211, 153], "DEFAULTDELAY 20\nCONTROL SHIFT i\n", ""),
                (6, "Go To", "Def", [56, 189, 248], "DEFAULTDELAY 20\nF12\n", ""),
                (7, "Rename", "Symbol", [250, 204, 21], "DEFAULTDELAY 20\nF2\n", ""),
                (8, "Search", "All", [56, 189, 248], "DEFAULTDELAY 20\nCONTROL SHIFT f\n", ""),
                (9, "Source", "Git", [249, 115, 22], "DEFAULTDELAY 20\nCONTROL SHIFT g\n", ""),
                (10, "Debug", "Start", [34, 197, 94], "DEFAULTDELAY 20\nF5\n", ""),
                (11, "Step", "Over", [56, 189, 248], "DEFAULTDELAY 20\nF10\n", ""),
                (12, "Step", "Into", [56, 189, 248], "DEFAULTDELAY 20\nF11\n", ""),
                (13, "Break", "Point", [239, 68, 68], "DEFAULTDELAY 20\nF9\n", ""),
                (14, "Split", "Editor", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL BACKSLASH\n", ""),
                (15, "Side", "Bar", [99, 102, 241], "DEFAULTDELAY 20\nCONTROL b\n", ""),
                (16, "Zen", "Mode", [168, 85, 247], "DEFAULTDELAY 20\nCONTROL k\nDELAY 40\nSTRING z\n", ""),
                (17, "Comment", "Line", [34, 197, 94], "DEFAULTDELAY 20\nCONTROL SLASH\n", ""),
                (18, "Multi", "Cursor", [250, 204, 21], "DEFAULTDELAY 20\nCONTROL d\n", ""),
                (19, "Problem", "Panel", [239, 68, 68], "DEFAULTDELAY 20\nCONTROL SHIFT m\n", ""),
                (20, "Save", "All", [34, 197, 94], "DEFAULTDELAY 20\nCONTROL ALT s\n", ""),
                (21, "Font -", "", [0, 122, 204], "CONTROL MINUS\n", ""),
                (22, "Font +", "", [0, 122, 204], "CONTROL EQUAL\n", ""),
                (23, "Reset", "", [0, 122, 204], "CONTROL NUMPAD0\n", ""),
                (24, "Edit <", "", [0, 122, 204], "CONTROL PAGEUP\n", ""),
                (25, "Edit >", "", [0, 122, 204], "CONTROL PAGEDOWN\n", ""),
                (26, "Fold", "", [0, 122, 204], "CONTROL SHIFT [\n", ""),
            ],
        );

    // 4. OBS Studio (with Web API triggers)
    let obs_slot17 = apply_web_api_config_to_script(
        "",
        &WebApiConfig {
            enabled: true,
            destination: "Home Assistant — Light Toggle".to_string(),
            method: "POST".to_string(),
            url: "http://homeassistant.local:8123/api/services/light/toggle".to_string(),
            headers:
                "Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\nContent-Type: application/json"
                    .to_string(),
            body: "{\"entity_id\": \"light.studio_key_light\"}".to_string(),
            oled_text: "Key Light API".to_string(),
            led_feedback: true,
            trigger_mode: "pgv".to_string(),
            fkey: "F17".to_string(),
        },
        4,
        17,
        false,
    );
    let obs_slot18 = apply_web_api_config_to_script(
        "",
        &WebApiConfig {
            enabled: true,
            destination: "Home Assistant — Switch Turn On".to_string(),
            method: "POST".to_string(),
            url: "http://homeassistant.local:8123/api/services/switch/turn_on".to_string(),
            headers:
                "Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\nContent-Type: application/json"
                    .to_string(),
            body: "{\"entity_id\": \"switch.on_air_lamp\"}".to_string(),
            oled_text: "On-Air Lamp ON".to_string(),
            led_feedback: true,
            trigger_mode: "pgv".to_string(),
            fkey: "F18".to_string(),
        },
        4,
        18,
        false,
    );
    let obs = build_profile(
        4,
        "OBS Studio",
        [139, 92, 246],
        &[
            (
                1,
                "Scene",
                "Main",
                [56, 189, 248],
                "DEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT Scene: Main Cam\nOLED_UPDATE\nF13\n",
                "",
            ),
            (
                2,
                "Scene",
                "Screen",
                [56, 189, 248],
                "DEFAULTDELAY 20\nF14\n",
                "",
            ),
            (
                3,
                "Scene",
                "Pip",
                [56, 189, 248],
                "DEFAULTDELAY 20\nF15\n",
                "",
            ),
            (
                4,
                "Scene",
                "BRB",
                [250, 204, 21],
                "DEFAULTDELAY 20\nF16\n",
                "",
            ),
            (
                5,
                "Record",
                "Toggle",
                [239, 68, 68],
                "DEFAULTDELAY 20\nCONTROL SHIFT r\n",
                "",
            ),
            (
                6,
                "Pause",
                "Rec",
                [249, 115, 22],
                "DEFAULTDELAY 20\nCONTROL SHIFT p\n",
                "",
            ),
            (
                7,
                "Stream",
                "Live",
                [168, 85, 247],
                "DEFAULTDELAY 20\nCONTROL SHIFT s\n",
                "",
            ),
            (
                8,
                "Replay",
                "Buffer",
                [34, 197, 94],
                "DEFAULTDELAY 20\nCONTROL SHIFT b\n",
                "",
            ),
            (
                9,
                "Mic",
                "Mute",
                [239, 68, 68],
                "DEFAULTDELAY 20\nCONTROL SHIFT m\n",
                "",
            ),
            (
                10,
                "Desktop",
                "Mute",
                [249, 115, 22],
                "DEFAULTDELAY 20\nCONTROL SHIFT d\n",
                "",
            ),
            (
                11,
                "Studio",
                "Mode",
                [139, 92, 246],
                "DEFAULTDELAY 20\nCONTROL SHIFT t\n",
                "",
            ),
            (
                12,
                "Cut",
                "Trans",
                [56, 189, 248],
                "DEFAULTDELAY 20\nCONTROL SHIFT c\n",
                "",
            ),
            (
                13,
                "Fade",
                "Trans",
                [56, 189, 248],
                "DEFAULTDELAY 20\nCONTROL SHIFT f\n",
                "",
            ),
            (
                14,
                "Snap",
                "Shot",
                [34, 197, 94],
                "DEFAULTDELAY 20\nF17\n",
                "",
            ),
            (
                15,
                "Virtual",
                "Cam",
                [14, 165, 233],
                "DEFAULTDELAY 20\nCONTROL SHIFT v\n",
                "",
            ),
            (
                16,
                "Stats",
                "Dock",
                [99, 102, 241],
                "DEFAULTDELAY 20\nCONTROL SHIFT i\n",
                "",
            ),
            (17, "Key", "Light", [250, 204, 21], &obs_slot17, ""),
            (18, "On Air", "Lamp", [239, 68, 68], &obs_slot18, ""),
            (
                19,
                "Stream",
                "Marker",
                [34, 197, 94],
                "DEFAULTDELAY 20\nF18\n",
                "",
            ),
            (
                20,
                "Panic",
                "Mute",
                [239, 68, 68],
                "DEFAULTDELAY 20\nF19\n",
                "",
            ),
            (21, "Mic -", "", [139, 92, 246], "F20\n", ""),
            (22, "Mic +", "", [139, 92, 246], "F21\n", ""),
            (23, "Mute", "", [239, 68, 68], "CONTROL SHIFT m\n", ""),
            (24, "Desk -", "", [139, 92, 246], "F22\n", ""),
            (25, "Desk +", "", [139, 92, 246], "F23\n", ""),
            (26, "Mute", "", [249, 115, 22], "CONTROL SHIFT d\n", ""),
        ],
    );

    // 5. Konsole Terminal
    let konsole = build_profile(
        5,
        "Konsole",
        [16, 185, 129],
        &[
            (
                1,
                "New",
                "Tab",
                [16, 185, 129],
                "DEFAULTDELAY 20\nCONTROL SHIFT t\n",
                "",
            ),
            (
                2,
                "Split",
                "L / R",
                [56, 189, 248],
                "DEFAULTDELAY 20\nCONTROL (\n",
                "",
            ),
            (
                3,
                "Split",
                "T / B",
                [56, 189, 248],
                "DEFAULTDELAY 20\nCONTROL )\n",
                "",
            ),
            (
                4,
                "Clear",
                "Screen",
                [14, 165, 233],
                "DEFAULTDELAY 20\nCONTROL l\n",
                "",
            ),
            (
                5,
                "Git",
                "Status",
                [249, 115, 22],
                "DEFAULTDELAY 20\nSTRINGLN git status -sb\n",
                "",
            ),
            (
                6,
                "Git",
                "Log",
                [249, 115, 22],
                "DEFAULTDELAY 20\nSTRINGLN git log --oneline -n 15\n",
                "",
            ),
            (
                7,
                "Cargo",
                "Build",
                [250, 204, 21],
                "DEFAULTDELAY 20\nSTRINGLN cargo build --release\n",
                "",
            ),
            (
                8,
                "Cargo",
                "Test",
                [34, 197, 94],
                "DEFAULTDELAY 20\nSTRINGLN cargo test\n",
                "",
            ),
            (
                9,
                "Docker",
                "PS",
                [56, 189, 248],
                "DEFAULTDELAY 20\nSTRINGLN docker ps\n",
                "",
            ),
            (
                10,
                "System",
                "Btop",
                [168, 85, 247],
                "DEFAULTDELAY 20\nSTRINGLN btop\n",
                "",
            ),
            (
                11,
                "Journal",
                "Follow",
                [250, 204, 21],
                "DEFAULTDELAY 20\nSTRINGLN journalctl -f -n 40\n",
                "",
            ),
            (
                12,
                "Hist",
                "Search",
                [16, 185, 129],
                "DEFAULTDELAY 20\nCONTROL r\n",
                "",
            ),
            (
                13,
                "Copy",
                "Text",
                [56, 189, 248],
                "DEFAULTDELAY 20\nCONTROL SHIFT c\n",
                "",
            ),
            (
                14,
                "Paste",
                "Clip",
                [56, 189, 248],
                "DEFAULTDELAY 20\nCONTROL SHIFT v\n",
                "",
            ),
            (
                15,
                "Find",
                "Output",
                [99, 102, 241],
                "DEFAULTDELAY 20\nCONTROL SHIFT f\n",
                "",
            ),
            (
                16,
                "Cancel",
                "SIGINT",
                [239, 68, 68],
                "DEFAULTDELAY 20\nCONTROL c\n",
                "",
            ),
            (21, "Tab <", "", [16, 185, 129], "SHIFT LEFT\n", ""),
            (22, "Tab >", "", [16, 185, 129], "SHIFT RIGHT\n", ""),
            (23, "New", "", [16, 185, 129], "CONTROL SHIFT t\n", ""),
            (24, "Font -", "", [16, 185, 129], "CONTROL MINUS\n", ""),
            (25, "Font +", "", [16, 185, 129], "CONTROL PLUS\n", ""),
            (26, "100%", "", [16, 185, 129], "CONTROL ALT 0\n", ""),
        ],
    );

    // 6. Blender 3D
    let mut blender = create_default_demo_profile();
    blender.number = 6;
    blender.name = "Blender 3D".to_string();
    blender.folder_name_override = Some("profile_Blender_3D".to_string());
    blender.bg_color = [249, 115, 22];

    // 7. Spotify & Media
    let spotify = build_profile(
        7,
        "Spotify",
        [30, 215, 96],
        &[
            (
                1,
                "Play",
                "Pause",
                [30, 215, 96],
                "DEFAULTDELAY 20\nMK_PP\n",
                "",
            ),
            (
                2,
                "Prev",
                "Track",
                [56, 189, 248],
                "DEFAULTDELAY 20\nMK_PREV\n",
                "",
            ),
            (
                3,
                "Next",
                "Track",
                [56, 189, 248],
                "DEFAULTDELAY 20\nMK_NEXT\n",
                "",
            ),
            (
                4,
                "Stop",
                "Audio",
                [239, 68, 68],
                "DEFAULTDELAY 20\nMK_STOP\n",
                "",
            ),
            (
                5,
                "Mute",
                "Audio",
                [249, 115, 22],
                "DEFAULTDELAY 20\nMK_MUTE\n",
                "",
            ),
            (
                6,
                "Like",
                "Song",
                [236, 72, 153],
                "DEFAULTDELAY 20\nALT SHIFT b\n",
                "",
            ),
            (
                7,
                "Shuffle",
                "Queue",
                [30, 215, 96],
                "DEFAULTDELAY 20\nCONTROL s\n",
                "",
            ),
            (
                8,
                "Repeat",
                "Track",
                [30, 215, 96],
                "DEFAULTDELAY 20\nCONTROL r\n",
                "",
            ),
            (
                9,
                "Search",
                "Music",
                [56, 189, 248],
                "DEFAULTDELAY 20\nCONTROL l\n",
                "",
            ),
            (
                10,
                "Queue",
                "Panel",
                [168, 85, 247],
                "DEFAULTDELAY 20\nALT SHIFT q\n",
                "",
            ),
            (
                11,
                "Lyrics",
                "View",
                [250, 204, 21],
                "DEFAULTDELAY 20\nALT SHIFT l\n",
                "",
            ),
            (
                12,
                "Now",
                "Playing",
                [30, 215, 96],
                "DEFAULTDELAY 20\nALT SHIFT j\n",
                "",
            ),
            (21, "Vol -", "", [30, 215, 96], "MK_VOLDOWN\n", ""),
            (22, "Vol +", "", [30, 215, 96], "MK_VOLUP\n", ""),
            (23, "Mute", "", [239, 68, 68], "MK_MUTE\n", ""),
            (24, "Back", "", [56, 189, 248], "SHIFT LEFT\n", ""),
            (25, "Fwd", "", [56, 189, 248], "SHIFT RIGHT\n", ""),
            (26, "Play", "", [30, 215, 96], "MK_PP\n", ""),
        ],
    );

    // 8. App Launcher (with @DUCKYDECK_APP_LAUNCH triggers)
    let make_app_script =
        |slot_num: usize, name: &str, desktop_id: &str, cmd: &str, args: &str, oled: &str| {
            apply_app_launch_config_to_script(
                "",
                &AppLaunchConfig {
                    enabled: true,
                    app_name: name.to_string(),
                    desktop_id: desktop_id.to_string(),
                    command: cmd.to_string(),
                    args: args.to_string(),
                    working_dir: "/home/demo/Projects".to_string(),
                    run_in_terminal: false,
                    oled_text: oled.to_string(),
                    led_feedback: true,
                    trigger_mode: "pgv".to_string(),
                    fkey: format!("F{}", 12 + slot_num.min(12)),
                },
                8,
                slot_num,
                false,
            )
        };
    let app1 = make_app_script(
        1,
        "Dolphin",
        "org.kde.dolphin.desktop",
        "dolphin",
        "~/Projects",
        "Run: Dolphin",
    );
    let app2 = make_app_script(
        2,
        "Google Chrome",
        "google-chrome.desktop",
        "google-chrome-stable",
        "https://www.duckypad.com",
        "Run: Chrome",
    );
    let app3 = make_app_script(
        3,
        "Visual Studio Code",
        "code.desktop",
        "code",
        "~/Projects/duckydeck",
        "Run: VS Code",
    );
    let app4 = make_app_script(
        4,
        "OBS Studio",
        "com.obsproject.Studio.desktop",
        "obs",
        "",
        "Run: OBS Studio",
    );
    let app5 = make_app_script(
        5,
        "Konsole",
        "org.kde.konsole.desktop",
        "konsole",
        "",
        "Run: Konsole",
    );
    let app6 = make_app_script(
        6,
        "Blender",
        "blender.desktop",
        "blender",
        "",
        "Run: Blender",
    );
    let app7 = make_app_script(
        7,
        "Spotify",
        "spotify.desktop",
        "spotify",
        "",
        "Run: Spotify",
    );
    let app8 = make_app_script(8, "GIMP", "gimp.desktop", "gimp", "", "Run: GIMP");

    let app_launcher = build_profile(
        8,
        "App Launcher",
        [236, 72, 153],
        &[
            (1, "Dolphin", "Files", [0, 170, 255], &app1, ""),
            (2, "Google", "Chrome", [66, 133, 244], &app2, ""),
            (3, "VS", "Code", [0, 122, 204], &app3, ""),
            (4, "OBS", "Studio", [139, 92, 246], &app4, ""),
            (5, "Konsole", "Term", [16, 185, 129], &app5, ""),
            (6, "Blender", "3D", [249, 115, 22], &app6, ""),
            (7, "Spotify", "Music", [30, 215, 96], &app7, ""),
            (8, "GIMP", "Image", [250, 204, 21], &app8, ""),
        ],
    );

    let profiles = vec![
        dolphin,
        chrome,
        vscode,
        obs,
        konsole,
        blender,
        spotify,
        app_launcher,
    ];

    let rules = vec![
        AutoSwitchRule {
            enabled: true,
            resource_class: "org.kde.dolphin".to_string(),
            title_contains: None,
            title_is_regex: false,
            favicon_url: None,
            target_profile: 1,
        },
        AutoSwitchRule {
            enabled: true,
            resource_class: "google-chrome".to_string(),
            title_contains: None,
            title_is_regex: false,
            favicon_url: None,
            target_profile: 2,
        },
        AutoSwitchRule {
            enabled: true,
            resource_class: "code".to_string(),
            title_contains: None,
            title_is_regex: false,
            favicon_url: None,
            target_profile: 3,
        },
        AutoSwitchRule {
            enabled: true,
            resource_class: "com.obsproject.Studio".to_string(),
            title_contains: None,
            title_is_regex: false,
            favicon_url: None,
            target_profile: 4,
        },
        AutoSwitchRule {
            enabled: true,
            resource_class: "org.kde.konsole".to_string(),
            title_contains: None,
            title_is_regex: false,
            favicon_url: None,
            target_profile: 5,
        },
        AutoSwitchRule {
            enabled: true,
            resource_class: "blender".to_string(),
            title_contains: None,
            title_is_regex: false,
            favicon_url: None,
            target_profile: 6,
        },
        AutoSwitchRule {
            enabled: true,
            resource_class: "spotify".to_string(),
            title_contains: None,
            title_is_regex: false,
            favicon_url: None,
            target_profile: 7,
        },
    ];

    (profiles, rules)
}

pub fn create_showcase_instance() -> DeviceControllerRust {
    qobject::duckydeck_apply_ui_font_scale(100);
    let (profiles, rules) = create_showcase_profiles_and_rules();
    let saved_colors = crate::kwin::auto_switcher::default_saved_colors();
    let switcher = SwitcherState {
        enabled: true,
        paused_for_editing: false,
        default_profile: Some(8),
        rules: rules.clone(),
        mouse_settle_ms: 250,
        window_dwell_ms: 150,
        tray_icon_mode: "auto".to_string(),
        app_theme_mode: "dark".to_string(),
        close_to_tray: true,
        ui_text_scale: 100,
        sidebar_text_scale: 100,
        canvas_text_scale: 100,
        inspector_text_scale: 100,
        editor_text_scale: 100,
        status_bar_text_scale: 100,
        event_log_text_scale: 100,
        saved_colors: saved_colors.clone(),
        saved_color_names: std::collections::HashMap::new(),
        rest_api_enabled: false,
        rest_api_bind_ip: "127.0.0.1".to_string(),
        rest_api_port: 28491,
        rest_api_require_auth: true,
        rest_api_token: generate_rest_api_token(),
        profile_webhook_enabled: false,
        profile_webhook_method: "POST".to_string(),
        profile_webhook_url: String::new(),
        profile_webhook_headers: "Content-Type: application/json".to_string(),
        profile_webhook_body: String::new(),
        autoswitch_enable_section_expanded: true,
        autoswitch_rule_editor_expanded: true,
        google_search_shortcuts_permitted: true,
        multi_click_interval_ms: 250,
        last_switched_profile: 1,
        picker_armed: false,
        loaded_from_disk: true,
    };

    if let Ok(mut guard) = system_event_log_store().lock() {
        guard.clear();
    }
    record_system_event(
        "INFO",
        "SYSTEM",
        format!(
            "DuckyDeck Studio v{} started (KDE Plasma 6 Wayland)",
            APP_VERSION
        ),
    );
    record_system_event(
        "INFO",
        "DEVICE",
        "Connected to duckyPad Pro (FW v2.3.1, SN DP24-8F4A1C9E) — 8 profile(s) loaded",
    );
    record_system_event(
            "INFO",
            "AUTOSWITCH",
            "KWin Wayland D-Bus Auto-Switcher active (7 application rules loaded, fallback=#8 App Launcher)",
        );
    record_system_event(
        "INFO",
        "AUTOSWITCH",
        "Window focus matched rule 'org.kde.dolphin' → switched hardware to Profile #1 (Dolphin)",
    );
    record_system_event(
        "INFO",
        "WEB_API",
        "Web API PGV[31] listener armed for Profile #4 (OBS Studio): Slots 17 & 18 ready",
    );
    record_system_event(
            "INFO",
            "APP_LAUNCH",
            "Application Launcher PGV[31] listener armed for Profile #8 (App Launcher): 8 desktop apps ready",
        );

    let rules_json = DeviceControllerRust::serialize_rules(&rules, &profiles);
    let profiles_json = DeviceControllerRust::serialize_profiles_summary(&profiles, &rules);
    let active_prof_obj = profiles.first().unwrap();
    let slots_json = DeviceControllerRust::serialize_profile_slots(active_prof_obj);
    let saved_colors_json =
        serde_json::to_string(&saved_colors).unwrap_or_else(|_| "[]".to_string());
    let used_colors_json = DeviceControllerRust::collect_used_colors_json(&profiles);
    let (init_log_json, init_log_count) = serialize_system_event_log_json();

    let demo_apps = json!([
        {"name": "Dolphin", "exec": "dolphin", "icon": "system-file-manager", "desktopId": "org.kde.dolphin.desktop", "comment": "KDE File Manager"},
        {"name": "Google Chrome", "exec": "google-chrome-stable", "icon": "google-chrome", "desktopId": "google-chrome.desktop", "comment": "Access the Internet"},
        {"name": "Visual Studio Code", "exec": "code", "icon": "visual-studio-code", "desktopId": "code.desktop", "comment": "Code Editing. Redefined."},
        {"name": "OBS Studio", "exec": "obs", "icon": "com.obsproject.Studio", "desktopId": "com.obsproject.Studio.desktop", "comment": "Live Streaming and Screen Recorder"},
        {"name": "Konsole", "exec": "konsole", "icon": "utilities-terminal", "desktopId": "org.kde.konsole.desktop", "comment": "KDE Terminal Emulator"},
        {"name": "Blender", "exec": "blender", "icon": "blender", "desktopId": "blender.desktop", "comment": "3D modeling, animation, rendering and post-production"}
    ]);

    let demo_onboarding = json!({
        "firstRunPending": false,
        "deckDetected": true,
        "modelName": "duckyPad Pro",
        "serialNumber": "DP24-8F4A1C9E",
        "conflicts": [],
        "profileSources": [
            {
                "id": "deck_sd",
                "label": "Connected duckyPad Hardware / microSD (8 profiles)",
                "detail": "Load existing profiles directly from connected duckyPad Pro (SN DP24-8F4A1C9E)",
                "profileCount": 8,
                "recommended": true
            },
            {
                "id": "dekunukem_backup:/home/demo/.local/share/duckypad_config/profile_backups/duckyPad_Pro_backup",
                "label": "dekuNukem Configurator Backup — duckyPad_Pro_backup (8 profiles)",
                "detail": "~/.local/share/duckypad_config/profile_backups/duckyPad_Pro_backup",
                "profileCount": 8,
                "recommended": false
            },
            {
                "id": "keep_current",
                "label": "Keep Currently Loaded Profiles (8 profiles)",
                "detail": "Continue with the profiles currently loaded in DuckyDeck Studio",
                "profileCount": 8,
                "recommended": false
            }
        ],
        "ruleSources": [
            {
                "id": "existing_duckydeck_rules",
                "label": "Existing DuckyDeck Studio / SD Rules (7 rules)",
                "detail": "Preserves your configured DuckyDeck Studio KWin auto-switcher rules",
                "ruleCount": 7,
                "recommended": true
            },
            {
                "id": "dekunukem_rules:/home/demo/.local/share/duckypad_autoswitcher/config.txt",
                "label": "dekuNukem Auto-Switcher — config.txt (21 rules)",
                "detail": "~/.local/share/duckypad_autoswitcher/config.txt",
                "ruleCount": 21,
                "recommended": false
            }
        ]
    });

    DeviceControllerRust {
            connected: true,
            udev_rule_missing: false,
            model_name: QString::from("duckyPad Pro"),
            firmware_version: QString::from("1.1.1 (DuckyDeck Studio)"),
            serial_number: QString::from("DP24-8F4A1C9E"),
            active_profile: 1,
            active_profile_name: QString::from("Dolphin"),
            sd_mount_path: QString::from("/run/media/ducky/DUCKYPAD"),
            sd_card_detected: true,
            sd_space_summary: QString::from("SD: 1.42 MB used • 3.74 GB free"),
            sd_space_detail: QString::from(
                "duckyPad microSD Card Capacity\n• Used Space: 1.42 MB (0.04%)\n• Free Space: 3.74 GB (99.96%)\n• Total Capacity: 3.74 GB\n• Source: Mounted at /run/media/ducky/DUCKYPAD",
            ),
            sd_space_used_percent: 1,
            profiles_json: QString::from(&profiles_json),
            active_profile_slots_json: QString::from(&slots_json),
            status_message: QString::from(
                "Connected to duckyPad Pro (FW v1.1.1 DuckyDeck Studio) — 8 profile(s) synced & verified",
            ),
            ai_busy: false,
            generated_script_preview: QString::from(
                "// Slot 1: New Tab\nDEFAULTDELAY 20\nOLED_CLEAR\nOLED_CPRINT New Dolphin Tab\nOLED_UPDATE\nCONTROL t\n",
            ),
            auto_switch_enabled: true,
            default_fallback_profile: 8,
            auto_switch_settle_ms: 250,
            auto_switch_dwell_ms: 150,
            autoswitch_enable_section_expanded: true,
            autoswitch_rule_editor_expanded: true,
            active_window_class: QString::from("org.kde.dolphin"),
            active_window_title: QString::from("Projects — Dolphin"),
            window_picker_armed: false,
            last_picked_window_class: QString::from("google-chrome"),
            last_picked_window_title: QString::from("duckyPad Pro Documentation"),
            auto_switch_rules_json: QString::from(&rules_json),
            autostart_enabled: true,
            tray_icon_mode: QString::from("auto"),
            app_theme_mode: QString::from("dark"),
            close_to_tray: true,
            system_dark_theme: true,
            latest_firmware_version: QString::from("1.1.1"),
            firmware_update_available: false,
            firmware_check_busy: false,
            firmware_update_url: QString::from(
                "https://dekuNukem.github.io/duckyPad-Pro/doc/fw_update.html",
            ),
            firmware_check_summary: QString::from("DuckyDeck Studio Firmware v1.1.1 is up to date"),
            profile_sync_state: QString::from("synced"),
            profile_sync_detail: QString::from(
                "All 8 profiles verified in sync with duckyPad Pro microSD [hash #8f4a1c9e]",
            ),
            sd_sync_busy: false,
            sd_sync_progress: 100,
            sd_sync_step: QString::from("Ready"),
            sd_sync_log: QString::from(""),
            sd_sync_result_state: QString::from("idle"),
            sd_sync_result_summary: QString::from(""),
            web_api_test_busy: false,
            web_api_last_ok: true,
            web_api_last_result: QString::from("HTTP 200 OK (18 ms) — light.studio_key_light toggled"),
            web_api_destinations_json: QString::from(
                "[{\"name\":\"Home Assistant — Light Toggle\",\"method\":\"POST\",\"url\":\"http://homeassistant.local:8123/api/services/light/toggle\",\"headers\":\"Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\\nContent-Type: application/json\"},{\"name\":\"Home Assistant — Switch Turn On\",\"method\":\"POST\",\"url\":\"http://homeassistant.local:8123/api/services/switch/turn_on\",\"headers\":\"Authorization: Bearer eyJhbGciOiJIUzI1NiIs...\\nContent-Type: application/json\"}]",
            ),
            app_launch_last_ok: true,
            app_launch_last_result: QString::from("Launched Dolphin (PID 48210)"),
            installed_apps_json: QString::from(&demo_apps.to_string()),
            ui_text_scale: 100,
            sidebar_text_scale: 100,
            canvas_text_scale: 100,
            inspector_text_scale: 100,
            editor_text_scale: 100,
            status_bar_text_scale: 100,
            event_log_text_scale: 100,
            active_profile_bg_color: QString::from("#00aaff"),
            unsynced_changes_json: QString::from("[]"),
            has_copied_slot: false,
            copied_slot_summary: QString::from(""),
            app_version: QString::from(APP_VERSION),
            app_author: QString::from(APP_AUTHOR),
            app_github_url: QString::from(APP_GITHUB_URL),
            log_file_path: QString::from("/home/demo/.local/share/duckydeck-studio/duckydeck.log"),
            event_log_json: QString::from(&init_log_json),
            event_log_count: init_log_count,
            saved_colors_json: QString::from(&saved_colors_json),
            saved_color_names_json: QString::from("{}"),
            used_colors_json: QString::from(&used_colors_json),
            rest_api_enabled: false,
            rest_api_bind_ip: QString::from("127.0.0.1"),
            rest_api_port: 28491,
            rest_api_require_auth: true,
            rest_api_token: QString::from("ddk_9f8e7d6c5b4a3f2e1d0c9b8a7f6e5d4c3b2a1f0e"),
            rest_api_status: QString::from("Disabled"),
            firmware_fetch_busy: false,
            show_first_run_dialog: false,
            onboarding_state_json: QString::from(&demo_onboarding.to_string()),
            google_search_shortcuts_permitted: true,
            search_shortcuts_busy: false,
            discovered_shortcuts_json: QString::from("[]"),
            auto_switch_error_message: QString::from(""),
            multi_click_interval_ms: 250,
            copied_key_slot: None,
            firmware_download_url: String::new(),
            sd_root_path: None,
            profiles,
            switcher_state: Arc::new(Mutex::new(switcher)),
            kwin_listener_started: true,
            picker_bind_to_active_profile: false,
            conflict_warning_dismissed: false,
            pending_rule_import_source: None,
        }
}

pub fn capture_showcase_frame_impl(
    output_path: QString,
    banner_title: QString,
    banner_subtitle: QString,
) -> bool {
    let out_str = output_path.to_string();
    if let Some(parent) = Path::new(&out_str).parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            record_system_event(
                "WARN",
                "SYSTEM",
                format!(
                    "Failed creating capture parent dir {}: {e}",
                    parent.display()
                ),
            );
        }
    }
    qobject::duckydeck_capture_composite_frame(&output_path, &banner_title, &banner_subtitle)
}

pub fn demo_capture_dir_impl() -> QString {
    let dir = std::env::var("DUCKYDECK_CAPTURE_DIR")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| crate::paths::cache_dir().join("captures"));
    if let Err(e) = fs::create_dir_all(dir.join("screenshots")) {
        record_system_event(
            "WARN",
            "SYSTEM",
            format!("Failed creating screenshots capture dir: {e}"),
        );
    }
    if let Err(e) = fs::create_dir_all(dir.join("video_frames")) {
        record_system_event(
            "WARN",
            "SYSTEM",
            format!("Failed creating video_frames capture dir: {e}"),
        );
    }
    QString::from(dir.to_string_lossy().as_ref())
}

pub fn known_site_svg(query_raw: &str) -> Option<(&'static str, String)> {
    let cleaned = strip_browser_title_suffix(query_raw);
    let q = cleaned.trim().to_ascii_lowercase();
    if q.is_empty() {
        return None;
    }

    let has = |kw: &str| -> bool { q.contains(kw) };

    if has("youtube") || has("youtu.be") {
        return Some((
            "youtube",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="12" width="56" height="40" rx="12" fill="#FF0000"/>
  <polygon points="26,21 26,43 45,32" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("gmail") || has("mail.google") || has("google mail") {
        return Some((
            "gmail",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="10" width="56" height="44" rx="8" fill="#FFFFFF" stroke="#E0E0E0" stroke-width="2"/>
  <path d="M8 16 L8 50 L18 50 L18 28 L32 39 L46 28 L46 50 L56 50 L56 16 L46 16 L32 27 L18 16 Z" fill="#EA4335"/>
  <path d="M8 16 L18 24 L18 50 L10 50 C8.9 50 8 49.1 8 48 Z" fill="#4285F4"/>
  <path d="M56 16 L46 24 L46 50 L54 50 C55.1 50 56 49.1 56 48 Z" fill="#34A853"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("twitch") {
        return Some((
            "twitch",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <path d="M12 8 L8 18 L8 50 L20 50 L20 58 L28 58 L36 50 L46 50 L56 40 L56 8 Z" fill="#9146FF"/>
  <path d="M18 14 L50 14 L50 37 L43 44 L33 44 L26 51 L26 44 L18 44 Z" fill="#FFFFFF"/>
  <rect x="27" y="21" width="5" height="14" fill="#9146FF"/>
  <rect x="38" y="21" width="5" height="14" fill="#9146FF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("github") {
        return Some((
            "github",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#24292F"/>
  <path d="M32 12 C20.95 12 12 20.95 12 32 C12 40.84 17.73 48.33 25.68 50.98 C26.68 51.16 27.05 50.55 27.05 50.02 C27.05 49.55 27.03 48.0 27.02 46.2 C21.45 47.41 20.28 43.8 20.28 43.8 C19.37 41.49 18.05 40.88 18.05 40.88 C16.23 39.64 18.19 39.66 18.19 39.66 C20.2 39.8 21.26 41.73 21.26 41.73 C23.05 44.79 25.95 43.91 27.1 43.4 C27.28 42.1 27.8 41.22 28.37 40.72 C23.93 40.22 19.26 38.5 19.26 30.83 C19.26 28.64 20.04 26.86 21.32 25.46 C21.11 24.96 20.43 22.92 21.52 20.16 C21.52 20.16 23.2 19.62 27.02 22.21 C28.62 21.77 30.32 21.55 32.02 21.54 C33.72 21.55 35.42 21.77 37.02 22.21 C40.84 19.62 42.52 20.16 42.52 20.16 C43.61 22.92 42.93 24.96 42.72 25.46 C44.0 26.86 44.78 28.64 44.78 30.83 C44.78 38.52 40.1 40.21 35.64 40.7 C36.36 41.32 37.0 42.54 37.0 44.41 C37.0 47.09 36.98 49.25 36.98 50.02 C36.98 50.55 37.35 51.17 38.36 50.98 C46.3 48.32 52 40.84 52 32 C52 20.95 43.05 12 32 12 Z" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("gitlab") {
        return Some((
            "gitlab",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#292961"/>
  <polygon points="32,52 44,24 20,24" fill="#E24329"/>
  <polygon points="32,52 20,24 10,24" fill="#FC6D26"/>
  <polygon points="10,24 15,10 20,24" fill="#FCA326"/>
  <polygon points="32,52 44,24 54,24" fill="#FC6D26"/>
  <polygon points="54,24 49,10 44,24" fill="#FCA326"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("reddit") {
        return Some((
            "reddit",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#FF4500"/>
  <ellipse cx="32" cy="37" rx="16" ry="11" fill="#FFFFFF"/>
  <circle cx="17" cy="31" r="4.5" fill="#FFFFFF"/>
  <circle cx="47" cy="31" r="4.5" fill="#FFFFFF"/>
  <circle cx="26" cy="35" r="3" fill="#FF4500"/>
  <circle cx="38" cy="35" r="3" fill="#FF4500"/>
  <path d="M25 42 Q32 46 39 42" stroke="#FF4500" stroke-width="2.5" fill="none" stroke-linecap="round"/>
  <path d="M32 26 L36 16 L44 18" stroke="#FFFFFF" stroke-width="2.5" fill="none" stroke-linecap="round"/>
  <circle cx="45" cy="18" r="3.5" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("netflix") {
        return Some((
            "netflix",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="6" y="6" width="52" height="52" rx="12" fill="#141414"/>
  <rect x="18" y="13" width="8" height="38" fill="#B1060F"/>
  <rect x="38" y="13" width="8" height="38" fill="#B1060F"/>
  <polygon points="18,13 26,13 46,51 38,51" fill="#E50914"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("spotify") {
        return Some((
            "spotify",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#1DB954"/>
  <path d="M18 25 Q32 20 46 26" stroke="#121212" stroke-width="4.5" fill="none" stroke-linecap="round"/>
  <path d="M20 34 Q32 30 43 35" stroke="#121212" stroke-width="4" fill="none" stroke-linecap="round"/>
  <path d="M22 42 Q32 39 40 43" stroke="#121212" stroke-width="3.5" fill="none" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("discord") {
        return Some((
            "discord",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="8" width="56" height="48" rx="14" fill="#5865F2"/>
  <path d="M22 20 C26 18 38 18 42 20 C47 26 49 34 47 42 C43 45 39 46 37 44 L35 41 C29 41 29 41 27 44 C25 46 21 45 17 42 C15 34 17 26 22 20 Z" fill="#FFFFFF"/>
  <circle cx="26" cy="32" r="3.5" fill="#5865F2"/>
  <circle cx="38" cy="32" r="3.5" fill="#5865F2"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("slack") {
        return Some((
            "slack",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#4A154B"/>
  <rect x="14" y="25" width="15" height="6" rx="3" fill="#36C5F0"/>
  <rect x="25" y="14" width="6" height="15" rx="3" fill="#2EB67D"/>
  <rect x="35" y="33" width="15" height="6" rx="3" fill="#ECB22E"/>
  <rect x="33" y="35" width="6" height="15" rx="3" fill="#E01E5A"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("chatgpt") || has("openai") || has("chat.com") {
        return Some((
            "chatgpt",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#10A37F"/>
  <circle cx="32" cy="32" r="14" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <circle cx="32" cy="22" r="8" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <circle cx="23" cy="37" r="8" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <circle cx="41" cy="37" r="8" fill="none" stroke="#FFFFFF" stroke-width="3"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("claude") || has("anthropic") {
        return Some((
            "claude",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#D97757"/>
  <g stroke="#FFF7F0" stroke-width="4" stroke-linecap="round">
    <line x1="32" y1="12" x2="32" y2="52"/>
    <line x1="12" y1="32" x2="52" y2="32"/>
    <line x1="18" y1="18" x2="46" y2="46"/>
    <line x1="46" y1="18" x2="18" y2="46"/>
  </g>
</svg>"##
                .to_string(),
        ));
    }
    if has("gemini") || has("aistudio.google") {
        return Some((
            "gemini",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#1A1F35"/>
  <path d="M32 10 Q34 30 54 32 Q34 34 32 54 Q30 34 10 32 Q30 30 32 10 Z" fill="#669DF6"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google docs") || has("docs.google") {
        return Some((
            "google-docs",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="12" y="6" width="40" height="52" rx="6" fill="#4285F4"/>
  <rect x="20" y="22" width="24" height="4" rx="2" fill="#FFFFFF"/>
  <rect x="20" y="31" width="24" height="4" rx="2" fill="#FFFFFF"/>
  <rect x="20" y="40" width="16" height="4" rx="2" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google sheets") || has("sheets.google") {
        return Some((
            "google-sheets",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="12" y="6" width="40" height="52" rx="6" fill="#0F9D58"/>
  <rect x="19" y="22" width="26" height="22" rx="2" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="19" y1="33" x2="45" y2="33" stroke="#FFFFFF" stroke-width="3"/>
  <line x1="30" y1="22" x2="30" y2="44" stroke="#FFFFFF" stroke-width="3"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google slides") || has("slides.google") {
        return Some((
            "google-slides",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="12" y="6" width="40" height="52" rx="6" fill="#F4B400"/>
  <rect x="19" y="23" width="26" height="18" rx="2" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google drive") || has("drive.google") {
        return Some((
            "google-drive",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <polygon points="24,10 40,10 56,38 40,38" fill="#FBBC04"/>
  <polygon points="8,38 24,10 32,24 16,52" fill="#0F9D58"/>
  <polygon points="16,52 50,52 56,38 24,38" fill="#4285F4"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google calendar") || has("calendar.google") {
        return Some((
            "google-calendar",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="8" y="8" width="48" height="48" rx="10" fill="#4285F4"/>
  <rect x="16" y="20" width="32" height="28" rx="4" fill="#FFFFFF"/>
  <text x="32" y="41" font-family="sans-serif" font-size="18" font-weight="bold" text-anchor="middle" fill="#1A73E8">31</text>
</svg>"##
                .to_string(),
        ));
    }
    if has("google meet") || has("meet.google") {
        return Some((
            "google-meet",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="6" y="14" width="36" height="36" rx="8" fill="#00897B"/>
  <polygon points="42,24 58,14 58,50 42,40" fill="#34A853"/>
  <rect x="14" y="22" width="20" height="20" rx="4" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("google maps") || has("maps.google") {
        return Some((
            "google-maps",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <path d="M32 6 C20 6 12 15 12 26 C12 41 32 58 32 58 C32 58 52 41 52 26 C52 15 44 6 32 6 Z" fill="#EA4335"/>
  <circle cx="32" cy="26" r="8" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("printables") || has("prusa") {
        return Some((
            "printables",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#FA6831"/>
  <polygon points="32,12 50,22 50,42 32,52 14,42 14,22" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <polyline points="14,22 32,32 50,22" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <line x1="32" y1="32" x2="32" y2="52" stroke="#FFFFFF" stroke-width="4"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("makerworld") || has("bambu") {
        return Some((
            "makerworld",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#00AE42"/>
  <path d="M16 46 L16 18 L32 32 L48 18 L48 46" fill="none" stroke="#FFFFFF" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("thingiverse") {
        return Some((
            "thingiverse",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#248BFB"/>
  <circle cx="32" cy="32" r="21" fill="none" stroke="#FFFFFF" stroke-width="3"/>
  <path d="M21 22 L43 22 M32 22 L32 45" stroke="#FFFFFF" stroke-width="6" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("onshape") || has("tinkercad") {
        return Some((
            "onshape",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#1B5AA0"/>
  <polygon points="32,12 50,22 50,42 32,52 14,42 14,22" fill="#4CAF50" stroke="#FFFFFF" stroke-width="3"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("octoprint") || has("mainsail") || has("fluidd") || has("klipper") {
        return Some((
            "klipper-web",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#D32F2F"/>
  <polygon points="22,14 42,14 42,30 35,40 29,40 22,30" fill="#FFFFFF"/>
  <rect x="16" y="46" width="32" height="5" rx="2.5" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("home assistant") || has("homeassistant") {
        return Some((
            "home-assistant",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#18BCF2"/>
  <polygon points="32,12 10,32 18,32 18,52 46,52 46,32 54,32" fill="#FFFFFF"/>
  <circle cx="32" cy="36" r="5" fill="#18BCF2"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("proxmox") {
        return Some((
            "proxmox",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#222222"/>
  <path d="M16 16 L32 32 L16 48 M48 16 L32 32 L48 48" stroke="#E57000" stroke-width="6" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("plex") {
        return Some((
            "plex",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#1F2326"/>
  <polygon points="22,14 34,14 46,32 34,50 22,50 34,32" fill="#E5A00D"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("jellyfin") {
        return Some((
            "jellyfin",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#101828"/>
  <path d="M32 12 L50 48 L14 48 Z" fill="none" stroke="#AA5CC3" stroke-width="5" stroke-linejoin="round"/>
  <path d="M32 25 L40 41 L24 41 Z" fill="#00A4DC"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("figma") {
        return Some((
            "figma",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#1E1E1E"/>
  <circle cx="26" cy="18" r="7" fill="#F24E1E"/>
  <circle cx="38" cy="18" r="7" fill="#FF7262"/>
  <circle cx="26" cy="32" r="7" fill="#A259FF"/>
  <circle cx="38" cy="32" r="7" fill="#1ABCFE"/>
  <circle cx="26" cy="46" r="7" fill="#0ACF83"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("notion") {
        return Some((
            "notion",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="6" y="6" width="52" height="52" rx="10" fill="#FFFFFF" stroke="#191919" stroke-width="4"/>
  <path d="M22 44 L22 20 L42 44 L42 20" fill="none" stroke="#191919" stroke-width="5" stroke-linecap="round" stroke-linejoin="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("stack overflow") || has("stackoverflow") {
        return Some((
            "stackoverflow",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#222426"/>
  <path d="M18 38 L18 50 L46 50 L46 38" fill="none" stroke="#BCBBBB" stroke-width="4"/>
  <rect x="23" y="41" width="18" height="4" fill="#F48024"/>
  <rect x="24" y="33" width="18" height="4" transform="rotate(10 33 35)" fill="#F48024"/>
  <rect x="26" y="25" width="18" height="4" transform="rotate(24 35 27)" fill="#F48024"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("wikipedia") {
        return Some((
            "wikipedia",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#F8F9FA" stroke="#54595D" stroke-width="3"/>
  <text x="32" y="42" font-family="serif" font-size="30" font-weight="bold" text-anchor="middle" fill="#202122">W</text>
</svg>"##
                .to_string(),
        ));
    }
    if has("amazon") {
        return Some((
            "amazon",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#232F3E"/>
  <text x="32" y="38" font-family="sans-serif" font-size="30" font-weight="bold" text-anchor="middle" fill="#FFFFFF">a</text>
  <path d="M16 45 Q32 54 48 44" stroke="#FF9900" stroke-width="4" fill="none" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("twitter") || has("x.com") || q == "x" {
        return Some((
            "x-twitter",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0F1419"/>
  <path d="M18 16 L28 16 L46 48 L36 48 Z M45 16 L19 48" stroke="#FFFFFF" stroke-width="4" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("bluesky") || has("bsky") {
        return Some((
            "bluesky",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0285FF"/>
  <path d="M32 28 C25 16 12 12 12 24 C12 34 24 36 28 36 C20 38 14 44 20 50 C25 54 30 46 32 40 C34 46 39 54 44 50 C50 44 44 38 36 36 C40 36 52 34 52 24 C52 12 39 16 32 28 Z" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("linkedin") {
        return Some((
            "linkedin",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0A66C2"/>
  <text x="32" y="44" font-family="sans-serif" font-size="32" font-weight="bold" text-anchor="middle" fill="#FFFFFF">in</text>
</svg>"##
                .to_string(),
        ));
    }
    if has("whatsapp") {
        return Some((
            "whatsapp",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#25D366"/>
  <circle cx="32" cy="32" r="18" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <path d="M25 26 C25 34 30 39 38 39" stroke="#FFFFFF" stroke-width="5" fill="none" stroke-linecap="round"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("telegram") {
        return Some((
            "telegram",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <circle cx="32" cy="32" r="28" fill="#26A5E4"/>
  <polygon points="14,31 48,17 41,47 30,38 24,43 26,35" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("linear") {
        return Some((
            "linear",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#5E6AD2"/>
  <circle cx="32" cy="32" r="18" fill="none" stroke="#FFFFFF" stroke-width="4"/>
  <line x1="20" y1="44" x2="44" y2="20" stroke="#FFFFFF" stroke-width="4"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("jira") || has("atlassian") {
        return Some((
            "jira",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="14" fill="#0052CC"/>
  <polygon points="32,14 48,30 32,46 24,38 32,30 24,22" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("trello") {
        return Some((
            "trello",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#0079BF"/>
  <rect x="14" y="14" width="14" height="34" rx="4" fill="#FFFFFF"/>
  <rect x="36" y="14" width="14" height="22" rx="4" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }
    if has("soundcloud") {
        return Some((
            "soundcloud",
            r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect x="4" y="4" width="56" height="56" rx="12" fill="#FF5500"/>
  <path d="M16 42 L48 42 C53 42 54 33 48 32 C47 23 35 21 32 29 C28 28 24 31 24 35 L16 35 Z" fill="#FFFFFF"/>
</svg>"##
                .to_string(),
        ));
    }

    None
}
