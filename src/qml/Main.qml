import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import org.kde.syntaxhighlighting as KSyntax
import io.github.duckydeck 1.0

Kirigami.ApplicationWindow {
    id: root
    objectName: "duckydeckMainWindow"
    title: qsTr("DuckyDeck Studio v%1 — %2").arg(deviceController.appVersion).arg(deviceController.modelName)
    readonly property int currentScreenWidth: (root.screen && root.screen.width > 0)
        ? root.screen.width
        : (Screen.width > 0 ? Screen.width : 1920)
    readonly property int maxCenterColumnWidth: Math.max(
        460,
        Math.round(root.currentScreenWidth * 0.30)
    )
    readonly property int compactWindowWidth: Math.max(
        profilesSidebar.QQC2.SplitView.preferredWidth
            + Math.min(hardwareCanvas.requiredWidthForFullHeight, root.maxCenterColumnWidth)
            + Kirigami.Units.largeSpacing * 2
            + 20,
        connRow.implicitWidth
            + fwBadgeRect.implicitWidth
            + syncStatusBadge.implicitWidth
            + (deviceController.sdMountPath.length > 0 ? 220 : 0)
            + Kirigami.Units.largeSpacing * 2
            + 180
    )
    readonly property int expandedWindowWidth: Math.min(
        Math.max(1280, Math.round(root.currentScreenWidth * 0.96)),
        Math.max(
            1520,
            compactWindowWidth + inspectorCard.minContentWidth + 8
        )
    )
    readonly property int requiredWindowWidth: root.hasSelectedSlot
        ? root.expandedWindowWidth
        : root.compactWindowWidth
    visible: false
    width: (deviceController && deviceController.isDemoShowcaseMode()) ? 1920 : compactWindowWidth
    height: (deviceController && deviceController.isDemoShowcaseMode()) ? 1080 : 800
    minimumWidth: root.hasSelectedSlot ? root.expandedWindowWidth : root.compactWindowWidth
    minimumHeight: 680

    TextEdit {
        id: clipboardHelper
        visible: false
    }

    function copyTextToClipboard(textToCopy) {
        clipboardHelper.text = String(textToCopy !== undefined && textToCopy !== null ? textToCopy : "")
        clipboardHelper.selectAll()
        clipboardHelper.copy()
        clipboardHelper.deselect()
    }

    function syncWindowWidthToEditorState() {
        if (deviceController && deviceController.isDemoShowcaseMode()) {
            root.width = 1920
            root.height = 1080
            return
        }
        if (root.visibility === Window.Maximized || root.visibility === Window.FullScreen) {
            return
        }
        if (root.hasSelectedSlot) {
            if (root.width < root.minimumWidth) {
                root.width = root.minimumWidth
            }
        } else {
            root.width = root.minimumWidth
        }
    }

    onMinimumWidthChanged: {
        root.syncWindowWidthToEditorState()
        Qt.callLater(root.syncWindowWidthToEditorState)
    }

    color: Kirigami.Theme.backgroundColor
    readonly property bool isLightTheme: (0.2126 * root.color.r + 0.7152 * root.color.g + 0.0722 * root.color.b) > 0.45
    readonly property color highContrastHighlightText: isLightTheme ? "#0369a1" : Kirigami.Theme.highlightColor
    readonly property color highContrastPositiveText: isLightTheme ? "#15803d" : "#4ade80"
    readonly property color highContrastWarningText: isLightTheme ? "#b45309" : "#fbbf24"
    readonly property color highContrastOrangeText: isLightTheme ? "#c2410c" : "#fb923c"

    readonly property real fontScale: (deviceController && deviceController.uiTextScale > 0
        ? deviceController.uiTextScale
        : 100) / 100.0
    readonly property real sidebarScale: root.fontScale
    readonly property real canvasScale: root.fontScale
    readonly property real inspectorScale: root.fontScale
    readonly property real editorScale: root.fontScale
    readonly property real statusBarScale: root.fontScale
    readonly property real eventLogScale: root.fontScale

    function zoomIn() {
        if (deviceController) {
            deviceController.zoomInUiTextScale()
        }
    }

    function zoomOut() {
        if (deviceController) {
            deviceController.zoomOutUiTextScale()
        }
    }

    function resetZoom() {
        if (deviceController) {
            deviceController.resetAllTextScales()
        }
    }

    function allPopupWindows() {
        return [
            { name: "DuckyColorPickerWindow", win: duckyColorPickerWindow, openFn: function() { duckyColorPickerWindow.openForKey(1, "#38BDF8") } },
            { name: "OledIconPickerDialog", win: oledIconPickerDialog, openFn: function() { oledIconPickerDialog.openForInspector(0) } },
            { name: "InstalledAppPickerDialog", win: installedAppPickerDialog, openFn: function() { installedAppPickerDialog.show(); installedAppPickerDialog.raise(); installedAppPickerDialog.requestActivate() } },
            { name: "WebApiDestinationsDialog", win: webApiDestinationsDialog, openFn: function() { webApiDestinationsDialog.openForDestination("", "POST", "", "") } },
            { name: "NewProfileGroupDialog", win: newProfileGroupDialog, openFn: function() { newProfileGroupDialog.openWithDefaults() } },
            { name: "RenameProfileDialog", win: renameProfileDialog, openFn: function() { renameProfileDialog.openWithProfile(1, "Default") } },
            { name: "DeleteProfileDialog", win: deleteProfileDialog, openFn: function() { deleteProfileDialog.open() } },
            { name: "UnsyncedChangesDialog", win: unsyncedChangesDialog, openFn: function() { unsyncedChangesDialog.open() } },
            { name: "SdSyncDialog", win: sdSyncDialog, openFn: function() { sdSyncDialog.open() } },
            { name: "FirmwareUpdateDialog", win: firmwareUpdateDialog, openFn: function() { firmwareUpdateDialog.open() } },
            { name: "ProfileTemplatesDialog", win: profileTemplatesDialog, openFn: function() { profileTemplatesDialog.openGeneral() } },
            { name: "SettingsDialog", win: settingsDialog, openFn: function() { settingsDialog.open() } },
            { name: "AboutDialog", win: aboutDialog, openFn: function() { aboutDialog.openAboutWindow() } },
            { name: "EventLogDialog", win: eventLogDialog, openFn: function() { eventLogDialog.openLogWindow() } },
            { name: "LightingAnimatorDialog", win: lightingAnimatorDialog, openFn: function() { lightingAnimatorDialog.openFor("profile", 1) } },
            { name: "MacroRecorderDialog", win: macroRecorderDialog, openFn: function() { macroRecorderDialog.openRecorder() } },
            { name: "AppLaunchConfigWindow", win: appLaunchConfigWindow, openFn: function() { appLaunchConfigWindow.openWindow(1) } },
            { name: "WebApiKeyConfigWindow", win: webApiKeyConfigWindow, openFn: function() { webApiKeyConfigWindow.openWindow(1) } },
            { name: "KeyPropertiesWindow", win: keyPropertiesWindow, openFn: function() { keyPropertiesWindow.openForSlot(1) } },
            { name: "ProfilePropertiesWindow", win: profilePropertiesWindow, openFn: function() { profilePropertiesWindow.openForProfile(deviceController.activeProfile || 1) } },
            { name: "AiModal", win: aiModal, openFn: function() { aiModal.open() } },
            { name: "PgvDrawer", win: pgvDrawer, openFn: function() { pgvDrawer.open() } },
            { name: "AutoSwitchDrawer", win: autoSwitchDrawer, openFn: function() { autoSwitchDrawer.open() } },
            { name: "OledDesktopWidget", win: oledDesktopWidget, openFn: function() { oledDesktopWidget.show(); oledDesktopWidget.syncFromController() } }
        ]
    }

    function closeTopmostPopupWindow() {
        const popups = root.allPopupWindows()
        // 1. Prefer an actively focused popup window if one is active
        for (let i = 0; i < popups.length; ++i) {
            const w = popups[i].win
            if (w && w.visible && w.active === true) {
                if (w === macroRecorderDialog && macroRecorderDialog.isRecording) {
                    macroRecorderDialog.stopRecording()
                }
                w.close()
                return true
            }
        }
        // 2. Otherwise close the topmost visible popup window in priority order
        for (let i = 0; i < popups.length; ++i) {
            const w = popups[i].win
            if (w && w.visible) {
                if (w === macroRecorderDialog && macroRecorderDialog.isRecording) {
                    macroRecorderDialog.stopRecording()
                }
                w.close()
                return true
            }
        }
        if (root.autocompleteVisible) {
            root.autocompleteVisible = false
            return true
        }
        return false
    }

    Shortcut {
        sequences: ["Ctrl++", "Ctrl+=", "Ctrl+Shift++", "Ctrl+Shift+="]
        context: Qt.ApplicationShortcut
        onActivated: root.zoomIn()
        onAmbiguouslyActivated: root.zoomIn()
    }

    Shortcut {
        sequences: ["Ctrl+-", "Ctrl+_"]
        context: Qt.ApplicationShortcut
        onActivated: root.zoomOut()
        onAmbiguouslyActivated: root.zoomOut()
    }

    Shortcut {
        sequence: "Ctrl+0"
        context: Qt.ApplicationShortcut
        onActivated: root.resetZoom()
        onAmbiguouslyActivated: root.resetZoom()
    }


    property int selectedSlotNumber: 0
    readonly property bool hasSelectedSlot: root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 58
    onHasSelectedSlotChanged: {
        if (deviceController) {
            deviceController.setScriptEditorOpen(root.hasSelectedSlot)
        }
        root.syncWindowWidthToEditorState()
        Qt.callLater(root.syncWindowWidthToEditorState)
    }
    property string selectedSlotTitle: "Key 1 (Front View)"
    property color selectedSlotColor: "#00aaff"
    property bool editingReleaseScript: false
    property string pressScriptBuffer: ""
    property string releaseScriptBuffer: ""
    property bool loadingSlotData: false

    // Snapshot of slot state when first selected so Revert can undo auto-saved edits
    property int slotSnapshotSlotNumber: 0
    property int slotSnapshotProfileNumber: 0
    property string slotSnapshotL1: ""
    property string slotSnapshotL2: ""
    property string slotSnapshotWL1: ""
    property string slotSnapshotWL2: ""
    property bool slotSnapshotAbort: false
    property bool slotSnapshotNoRepeat: false
    property string slotSnapshotPressScript: ""
    property string slotSnapshotReleaseScript: ""
    property string slotSnapshotColor: "#00aaff"

    // Mutually exclusive inspector workspace mode: "code" | "app_launch" | "web_api"
    property string inspectorMode: "code"
    property bool showScriptPreviewInBuilder: true

    property string profileFilterText: ""
    property int totalProfileCount: 1
    property int totalRuleCount: 0
    property bool showNewProfileInput: false

    property bool compileOk: true
    property bool compileEmpty: false
    property int compileByteCount: 0
    property int compileErrorLine: 0
    property string compileErrorMsg: ""

    property bool showWebApiBuilder: false
    onShowWebApiBuilderChanged: {
        if (showWebApiBuilder) {
            if (showAppLaunchBuilder) showAppLaunchBuilder = false
            if (showCommandReference) showCommandReference = false
            if (inspectorMode !== "web_api") inspectorMode = "web_api"
        } else if (inspectorMode === "web_api" && !showAppLaunchBuilder) {
            inspectorMode = "code"
        }
    }
    property bool syncingWebApiScript: false
    property bool webApiEnabled: false
    property string webApiSelectedDestination: ""
    property var webApiDestinationsList: {
        if (!deviceController || !deviceController.webApiDestinationsJson) return []
        try {
            const parsed = JSON.parse(deviceController.webApiDestinationsJson)
            return Array.isArray(parsed) ? parsed : []
        } catch (e) {
            return []
        }
    }
    readonly property var webApiDestinationComboItems: {
        const items = [qsTr("Custom")]
        const list = root.webApiDestinationsList || []
        for (let i = 0; i < list.length; ++i) {
            const d = list[i]
            items.push(d.name)
        }
        return items
    }
    readonly property var webApiMethods: ["POST", "GET", "PUT", "PATCH", "DELETE"]
    readonly property var webApiFkeys: ["F13", "F14", "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24"]

    // Application Launch Builder state
    property bool showAppLaunchBuilder: false
    onShowAppLaunchBuilderChanged: {
        if (showAppLaunchBuilder) {
            if (showWebApiBuilder) showWebApiBuilder = false
            if (showCommandReference) showCommandReference = false
            if (inspectorMode !== "app_launch") inspectorMode = "app_launch"
        } else if (inspectorMode === "app_launch" && !showWebApiBuilder) {
            inspectorMode = "code"
        }
    }
    property bool syncingAppLaunchScript: false
    property bool appLaunchEnabled: false
    property string appLaunchAppName: ""
    property string appLaunchDesktopId: ""
    property string installedAppSearchText: ""
    readonly property var appLaunchTriggerModes: ["pgv", "fkey", "krunner", "guir"]

    ListModel {
        id: installedAppsModel
    }

    // DuckyScript 3 Autocomplete & Command Reference state
    property bool showCommandReference: false
    property string commandRefSearchText: ""
    property string commandRefCategory: "All"
    readonly property var commandRefCategories: [
        "All",
        "Text & Keys",
        "Flow & Loops",
        "OLED & RGB",
        "Variables & Math",
        "Profiles & System"
    ]
    property bool autocompleteEnabled: true
    property bool autocompleteVisible: false
    property int autocompleteIndex: 0
    property int autocompleteTokenStart: 0
    property int autocompleteTokenEnd: 0
    property string autocompletePrefix: ""
    property bool insertingCompletion: false

    ListModel {
        id: autocompleteModel
    }
    ListModel {
        id: commandReferenceModel
    }

    readonly property var duckyScriptCatalog: [
        // Text & Keys
        { token: "STRING", category: "Text & Keys", badge: "CMD", syntax: "STRING <text>", insertText: "STRING ", summary: "Type literal text without pressing Enter at the end (supports $var interpolation)." },
        { token: "STRINGLN", category: "Text & Keys", badge: "CMD", syntax: "STRINGLN <text>", insertText: "STRINGLN ", summary: "Type literal text followed by an automatic Enter keystroke." },
        { token: "STRING_BLOCK", category: "Text & Keys", badge: "CMD", syntax: "STRING_BLOCK ... END_STRING", insertText: "STRING_BLOCK\n\nEND_STRING", summary: "Multi-line literal text block typed without trailing Enter on each line." },
        { token: "END_STRING", category: "Text & Keys", badge: "CMD", syntax: "END_STRING", insertText: "END_STRING", summary: "Closes a STRING_BLOCK block." },
        { token: "STRINGLN_BLOCK", category: "Text & Keys", badge: "CMD", syntax: "STRINGLN_BLOCK ... END_STRINGLN", insertText: "STRINGLN_BLOCK\n\nEND_STRINGLN", summary: "Multi-line literal text block where each line is followed by Enter." },
        { token: "END_STRINGLN", category: "Text & Keys", badge: "CMD", syntax: "END_STRINGLN", insertText: "END_STRINGLN", summary: "Closes a STRINGLN_BLOCK block." },
        { token: "REM", category: "Text & Keys", badge: "CMD", syntax: "REM <comment>", insertText: "REM ", summary: "Single-line comment ignored by the compiler (or use //)." },
        { token: "REM_BLOCK", category: "Text & Keys", badge: "CMD", syntax: "REM_BLOCK ... END_REM", insertText: "REM_BLOCK\n\nEND_REM", summary: "Multi-line comment block ignored by the compiler." },
        { token: "END_REM", category: "Text & Keys", badge: "CMD", syntax: "END_REM", insertText: "END_REM", summary: "Closes a REM_BLOCK multi-line comment." },
        { token: "KEYDOWN", category: "Text & Keys", badge: "KEY", syntax: "KEYDOWN <key>", insertText: "KEYDOWN ", summary: "Hold down a key or modifier until KEYUP is called." },
        { token: "KEYUP", category: "Text & Keys", badge: "KEY", syntax: "KEYUP <key>", insertText: "KEYUP ", summary: "Release a key previously held with KEYDOWN." },
        { token: "INJECT_MOD", category: "Text & Keys", badge: "KEY", syntax: "INJECT_MOD <modifier>", insertText: "INJECT_MOD ", summary: "Inject a standalone modifier key press compatibility wrapper." },
        { token: "CTRL", category: "Text & Keys", badge: "KEY", syntax: "CTRL <key>", insertText: "CTRL ", summary: "Left Control modifier key (can be combined up to 6 keys, e.g. CTRL SHIFT c)." },
        { token: "RCTRL", category: "Text & Keys", badge: "KEY", syntax: "RCTRL <key>", insertText: "RCTRL ", summary: "Right Control modifier key." },
        { token: "SHIFT", category: "Text & Keys", badge: "KEY", syntax: "SHIFT <key>", insertText: "SHIFT ", summary: "Left Shift modifier key." },
        { token: "RSHIFT", category: "Text & Keys", badge: "KEY", syntax: "RSHIFT <key>", insertText: "RSHIFT ", summary: "Right Shift modifier key." },
        { token: "ALT", category: "Text & Keys", badge: "KEY", syntax: "ALT <key>", insertText: "ALT ", summary: "Left Alt / Option modifier key." },
        { token: "RALT", category: "Text & Keys", badge: "KEY", syntax: "RALT <key>", insertText: "RALT ", summary: "Right Alt / AltGr modifier key." },
        { token: "GUI", category: "Text & Keys", badge: "KEY", syntax: "GUI <key>", insertText: "GUI ", summary: "Left Super / Windows / Command modifier key (aliases: WINDOWS, COMMAND)." },
        { token: "WINDOWS", category: "Text & Keys", badge: "KEY", syntax: "WINDOWS <key>", insertText: "WINDOWS ", summary: "Alias for GUI (Super / Windows / Meta key)." },
        { token: "COMMAND", category: "Text & Keys", badge: "KEY", syntax: "COMMAND <key>", insertText: "COMMAND ", summary: "Alias for GUI (macOS Command / Super key)." },
        { token: "ENTER", category: "Text & Keys", badge: "KEY", syntax: "ENTER", insertText: "ENTER", summary: "Press the Return / Enter key." },
        { token: "ESC", category: "Text & Keys", badge: "KEY", syntax: "ESC", insertText: "ESC", summary: "Press the Escape key (alias: ESCAPE)." },
        { token: "BACKSPACE", category: "Text & Keys", badge: "KEY", syntax: "BACKSPACE", insertText: "BACKSPACE", summary: "Press the Backspace key." },
        { token: "TAB", category: "Text & Keys", badge: "KEY", syntax: "TAB", insertText: "TAB", summary: "Press the Tab key." },
        { token: "SPACE", category: "Text & Keys", badge: "KEY", syntax: "SPACE", insertText: "SPACE", summary: "Press the Spacebar." },
        { token: "DELETE", category: "Text & Keys", badge: "KEY", syntax: "DELETE", insertText: "DELETE", summary: "Press the forward Delete key." },
        { token: "INSERT", category: "Text & Keys", badge: "KEY", syntax: "INSERT", insertText: "INSERT", summary: "Press the Insert key." },
        { token: "HOME", category: "Text & Keys", badge: "KEY", syntax: "HOME", insertText: "HOME", summary: "Press the Home key." },
        { token: "END", category: "Text & Keys", badge: "KEY", syntax: "END", insertText: "END", summary: "Press the End key." },
        { token: "PAGEUP", category: "Text & Keys", badge: "KEY", syntax: "PAGEUP", insertText: "PAGEUP", summary: "Press the Page Up key." },
        { token: "PAGEDOWN", category: "Text & Keys", badge: "KEY", syntax: "PAGEDOWN", insertText: "PAGEDOWN", summary: "Press the Page Down key." },
        { token: "UP", category: "Text & Keys", badge: "KEY", syntax: "UP", insertText: "UP", summary: "Press the Up Arrow key (alias: UPARROW)." },
        { token: "DOWN", category: "Text & Keys", badge: "KEY", syntax: "DOWN", insertText: "DOWN", summary: "Press the Down Arrow key (alias: DOWNARROW)." },
        { token: "LEFT", category: "Text & Keys", badge: "KEY", syntax: "LEFT", insertText: "LEFT", summary: "Press the Left Arrow key (alias: LEFTARROW)." },
        { token: "RIGHT", category: "Text & Keys", badge: "KEY", syntax: "RIGHT", insertText: "RIGHT", summary: "Press the Right Arrow key (alias: RIGHTARROW)." },
        { token: "PRINTSCREEN", category: "Text & Keys", badge: "KEY", syntax: "PRINTSCREEN", insertText: "PRINTSCREEN", summary: "Press the Print Screen / SysRq key." },
        { token: "CAPSLOCK", category: "Text & Keys", badge: "KEY", syntax: "CAPSLOCK", insertText: "CAPSLOCK", summary: "Press the Caps Lock key." },
        { token: "NUMLOCK", category: "Text & Keys", badge: "KEY", syntax: "NUMLOCK", insertText: "NUMLOCK", summary: "Press the Num Lock key." },
        { token: "SCROLLLOCK", category: "Text & Keys", badge: "KEY", syntax: "SCROLLLOCK", insertText: "SCROLLLOCK", summary: "Press the Scroll Lock key." },
        { token: "PAUSE", category: "Text & Keys", badge: "KEY", syntax: "PAUSE", insertText: "PAUSE", summary: "Press the Pause / Break key." },
        { token: "MENU", category: "Text & Keys", badge: "KEY", syntax: "MENU", insertText: "MENU", summary: "Press the Application / Context Menu key (alias: APP)." },
        { token: "F1", category: "Text & Keys", badge: "KEY", syntax: "F1 .. F24", insertText: "F1", summary: "Function keys F1 through F24 (F13–F24 great for conflict-free shortcuts)." },
        { token: "F13", category: "Text & Keys", badge: "KEY", syntax: "F13 .. F24", insertText: "F13", summary: "Extended Function key F13 (F13–F24 supported)." },
        { token: "MK_VOLUP", category: "Text & Keys", badge: "KEY", syntax: "MK_VOLUP", insertText: "MK_VOLUP", summary: "Media key: Volume Up." },
        { token: "MK_VOLDOWN", category: "Text & Keys", badge: "KEY", syntax: "MK_VOLDOWN", insertText: "MK_VOLDOWN", summary: "Media key: Volume Down." },
        { token: "MK_MUTE", category: "Text & Keys", badge: "KEY", syntax: "MK_MUTE", insertText: "MK_MUTE", summary: "Media key: Mute Audio." },
        { token: "MK_PP", category: "Text & Keys", badge: "KEY", syntax: "MK_PP", insertText: "MK_PP", summary: "Media key: Play / Pause." },
        { token: "MK_NEXT", category: "Text & Keys", badge: "KEY", syntax: "MK_NEXT", insertText: "MK_NEXT", summary: "Media key: Next Track." },
        { token: "MK_PREV", category: "Text & Keys", badge: "KEY", syntax: "MK_PREV", insertText: "MK_PREV", summary: "Media key: Previous Track." },
        { token: "MK_STOP", category: "Text & Keys", badge: "KEY", syntax: "MK_STOP", insertText: "MK_STOP", summary: "Media key: Stop Playback." },
        { token: "MOUSE_MOVE", category: "Text & Keys", badge: "KEY", syntax: "MOUSE_MOVE <dx> <dy>", insertText: "MOUSE_MOVE 0 0", summary: "Move cursor relatively by dx (-127..127) and dy (-127..127) pixels." },
        { token: "MOUSE_SCROLL", category: "Text & Keys", badge: "KEY", syntax: "MOUSE_SCROLL <h> <v>", insertText: "MOUSE_SCROLL 0 1", summary: "Scroll mouse wheel horizontally (h) and vertically (v)." },
        { token: "MOUSE_WHEEL", category: "Text & Keys", badge: "KEY", syntax: "MOUSE_WHEEL <v>", insertText: "MOUSE_WHEEL 1", summary: "Scroll vertical mouse wheel (positive = up, negative = down)." },
        { token: "LMOUSE", category: "Text & Keys", badge: "KEY", syntax: "LMOUSE", insertText: "LMOUSE", summary: "Click Left Mouse Button (also works with KEYDOWN/KEYUP)." },
        { token: "RMOUSE", category: "Text & Keys", badge: "KEY", syntax: "RMOUSE", insertText: "RMOUSE", summary: "Click Right Mouse Button." },
        { token: "MMOUSE", category: "Text & Keys", badge: "KEY", syntax: "MMOUSE", insertText: "MMOUSE", summary: "Click Middle Mouse Button." },
        { token: "BMOUSE", category: "Text & Keys", badge: "KEY", syntax: "BMOUSE", insertText: "BMOUSE", summary: "Click Mouse Back Button." },
        { token: "FMOUSE", category: "Text & Keys", badge: "KEY", syntax: "FMOUSE", insertText: "FMOUSE", summary: "Click Mouse Forward Button." },

        // Flow & Loops
        { token: "DELAY", category: "Flow & Loops", badge: "FLOW", syntax: "DELAY <ms>", insertText: "DELAY 100", summary: "Pause script execution for <ms> milliseconds." },
        { token: "DEFAULTDELAY", category: "Flow & Loops", badge: "FLOW", syntax: "DEFAULTDELAY <ms>", insertText: "DEFAULTDELAY 20", summary: "Set delay (ms) automatically inserted before every subsequent command." },
        { token: "DEFAULTCHARDELAY", category: "Flow & Loops", badge: "FLOW", syntax: "DEFAULTCHARDELAY <ms>", insertText: "DEFAULTCHARDELAY 20", summary: "Set delay (ms) between each character typed by STRING/STRINGLN." },
        { token: "CHARJITTER", category: "Flow & Loops", badge: "FLOW", syntax: "CHARJITTER <ms>", insertText: "CHARJITTER 5", summary: "Add random jitter (0..ms) to character typing delay." },
        { token: "IF", category: "Flow & Loops", badge: "FLOW", syntax: "IF <cond> ... END_IF", insertText: "IF $_READKEY == 0\n    \nEND_IF", summary: "Conditional execution block (supports ==, !=, <, <=, >, >=, &&, ||, !)." },
        { token: "ELSE IF", category: "Flow & Loops", badge: "FLOW", syntax: "ELSE IF <cond>", insertText: "ELSE IF ", summary: "Secondary branch inside an IF ... END_IF block." },
        { token: "ELSE", category: "Flow & Loops", badge: "FLOW", syntax: "ELSE", insertText: "ELSE", summary: "Fallback branch inside an IF ... END_IF block." },
        { token: "END_IF", category: "Flow & Loops", badge: "FLOW", syntax: "END_IF", insertText: "END_IF", summary: "Closes an IF conditional block." },
        { token: "WHILE", category: "Flow & Loops", badge: "FLOW", syntax: "WHILE <cond> ... END_WHILE", insertText: "WHILE 1\n    \nEND_WHILE", summary: "Loop while condition evaluates to non-zero." },
        { token: "END_WHILE", category: "Flow & Loops", badge: "FLOW", syntax: "END_WHILE", insertText: "END_WHILE", summary: "Closes a WHILE loop." },
        { token: "LBREAK", category: "Flow & Loops", badge: "FLOW", syntax: "LBREAK", insertText: "LBREAK", summary: "Break out of the innermost WHILE loop immediately." },
        { token: "CONTINUE", category: "Flow & Loops", badge: "FLOW", syntax: "CONTINUE", insertText: "CONTINUE", summary: "Skip to the next iteration of the innermost WHILE loop." },
        { token: "LOOP0:", category: "Flow & Loops", badge: "FLOW", syntax: "LOOP0: .. LOOP9:", insertText: "LOOP0:\n    \nLOOP1:\n    ", summary: "Cyclic step label: each press of the key advances to the next LOOP<n>: block." },
        { token: "REPEAT", category: "Flow & Loops", badge: "FLOW", syntax: "REPEAT <count>", insertText: "REPEAT 1", summary: "Repeat the previous line <count> additional times." },
        { token: "FUN", category: "Flow & Loops", badge: "FLOW", syntax: "FUN <name>(<args>) ... END_FUN", insertText: "FUN my_func(arg1)\n    RETURN 0\nEND_FUN", summary: "Define a reusable function with optional arguments and RETURN value." },
        { token: "END_FUN", category: "Flow & Loops", badge: "FLOW", syntax: "END_FUN", insertText: "END_FUN", summary: "Closes a FUN function definition." },
        { token: "RETURN", category: "Flow & Loops", badge: "FLOW", syntax: "RETURN [expr]", insertText: "RETURN ", summary: "Return from the current function (optionally returning a value)." },
        { token: "HALT", category: "Flow & Loops", badge: "FLOW", syntax: "HALT", insertText: "HALT", summary: "Immediately stop script execution." },
        { token: "PASS", category: "Flow & Loops", badge: "FLOW", syntax: "PASS", insertText: "PASS", summary: "No-operation placeholder statement." },

        // OLED & RGB
        { token: "OLED_CLEAR", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CLEAR", insertText: "OLED_CLEAR", summary: "Clear the OLED display framebuffer (call OLED_UPDATE to render)." },
        { token: "OLED_UPDATE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_UPDATE", insertText: "OLED_UPDATE", summary: "Push the OLED framebuffer to the physical screen." },
        { token: "OLED_RESTORE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_RESTORE", insertText: "OLED_RESTORE", summary: "Restore the default profile/key grid view on the OLED screen." },
        { token: "OLED_CURSOR", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CURSOR <x> <y>", insertText: "OLED_CURSOR 0 0", summary: "Move the OLED text cursor to pixel coordinates (x: 0..127, y: 0..63)." },
        { token: "OLED_PRINT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_PRINT <text>", insertText: "OLED_PRINT ", summary: "Print left-aligned text at the current OLED cursor position (supports $var)." },
        { token: "OLED_CPRINT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CPRINT <text>", insertText: "OLED_CPRINT ", summary: "Print horizontally centered text on the OLED screen." },
        { token: "OLED_LPRINT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_LPRINT <text>", insertText: "OLED_LPRINT ", summary: "Print text at the current OLED cursor with automatic line wrapping." },
        { token: "OLED_LINE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_LINE <x1> <y1> <x2> <y2>", insertText: "OLED_LINE 0 0 127 63", summary: "Draw a line on the OLED framebuffer between (x1,y1) and (x2,y2)." },
        { token: "OLED_RECT", category: "OLED & RGB", badge: "OLED", syntax: "OLED_RECT <x1> <y1> <x2> <y2> <opt>", insertText: "OLED_RECT 0 0 127 63 1", summary: "Draw a rectangle on the OLED (opt: 0=clear, 1=outline, 2=filled)." },
        { token: "OLED_CIRCLE", category: "OLED & RGB", badge: "OLED", syntax: "OLED_CIRCLE <x> <y> <radius> <opt>", insertText: "OLED_CIRCLE 64 32 16 1", summary: "Draw a circle on the OLED (opt: 0=clear, 1=outline, 2=filled)." },
        { token: "SWCOLOR", category: "OLED & RGB", badge: "LED", syntax: "SWCOLOR <r> <g> <b>", insertText: "SWCOLOR 0 170 255", summary: "Set the current key's RGB LED color (0..255 per channel)." },
        { token: "SWC_SET", category: "OLED & RGB", badge: "LED", syntax: "SWC_SET <key> <r> <g> <b>", insertText: "SWC_SET $_THIS_KEYID 0 255 128", summary: "Set RGB LED color of a specific key (0 = current key, 1..20 = key slot)." },
        { token: "SWC_FILL", category: "OLED & RGB", badge: "LED", syntax: "SWC_FILL <r> <g> <b>", insertText: "SWC_FILL 0 170 255", summary: "Fill all key RGB LEDs with the specified (r, g, b) color." },
        { token: "SWC_RESET", category: "OLED & RGB", badge: "LED", syntax: "SWC_RESET <key>", insertText: "SWC_RESET 0", summary: "Reset key RGB LED(s) back to profile default colors (0 = all keys, 1..20 = slot)." },

        // Variables & Math
        { token: "VAR", category: "Variables & Math", badge: "VAR", syntax: "VAR $<name> = <expr>", insertText: "VAR $count = 0", summary: "Declare and initialize a 32-bit user variable ($ is optional)." },
        { token: "DEFINE", category: "Variables & Math", badge: "VAR", syntax: "DEFINE <NAME> <replacement>", insertText: "DEFINE MY_DELAY 50", summary: "Define a compile-time text replacement macro." },
        { token: "DPDSSTDLIB", category: "Variables & Math", badge: "FUNC", syntax: "DPDSSTDLIB", insertText: "DPDSSTDLIB", summary: "Import the standard library functions: WAITKEY(k), MEMSET(a,v,n), ABS(n), MIN(a,b), MAX(a,b)." },
        { token: "RANDINT", category: "Variables & Math", badge: "FUNC", syntax: "RANDINT(<min>, <max>)", insertText: "RANDINT(1, 100)", summary: "Return a random signed integer between min and max (inclusive)." },
        { token: "RANDUINT", category: "Variables & Math", badge: "FUNC", syntax: "RANDUINT(<min>, <max>)", insertText: "RANDUINT(0, 100)", summary: "Return a random unsigned integer between min and max (inclusive)." },
        { token: "RANDCHR", category: "Variables & Math", badge: "FUNC", syntax: "RANDCHR(<bitmask>)", insertText: "RANDCHR(0x103)", summary: "Type/print a random character (or use RANDOM_LETTER, RANDOM_NUMBER, RANDOM_CHAR)." },
        { token: "RANDOM_LETTER", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_LETTER", insertText: "RANDOM_LETTER", summary: "Type a random uppercase or lowercase ASCII letter." },
        { token: "RANDOM_NUMBER", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_NUMBER", insertText: "RANDOM_NUMBER", summary: "Type a random digit (0–9)." },
        { token: "RANDOM_CHAR", category: "Variables & Math", badge: "FUNC", syntax: "RANDOM_CHAR", insertText: "RANDOM_CHAR", summary: "Type a random printable ASCII character." },
        { token: "WAITKEY", category: "Variables & Math", badge: "FUNC", syntax: "WAITKEY(<key_id>)", insertText: "WAITKEY(1)", summary: "Block until the specified key_id (1..20) is pressed (requires DPDSSTDLIB)." },
        { token: "ABS", category: "Variables & Math", badge: "FUNC", syntax: "ABS(<n>)", insertText: "ABS(", summary: "Return absolute value of n (requires DPDSSTDLIB)." },
        { token: "MIN", category: "Variables & Math", badge: "FUNC", syntax: "MIN(<a>, <b>)", insertText: "MIN(", summary: "Return the smaller of a and b (requires DPDSSTDLIB)." },
        { token: "MAX", category: "Variables & Math", badge: "FUNC", syntax: "MAX(<a>, <b>)", insertText: "MAX(", summary: "Return the larger of a and b (requires DPDSSTDLIB)." },
        { token: "$_GV0", category: "Variables & Math", badge: "VAR", syntax: "$_GV0 .. $_GV31", insertText: "$_GV0 = 0", summary: "Persistent Global Variable (0..31) shared across profiles and saved to SD/flash." },
        { token: "$_DEFAULTDELAY", category: "Variables & Math", badge: "VAR", syntax: "$_DEFAULTDELAY", insertText: "$_DEFAULTDELAY = 20", summary: "Read/write the default delay (ms) between commands." },
        { token: "$_DEFAULTCHARDELAY", category: "Variables & Math", badge: "VAR", syntax: "$_DEFAULTCHARDELAY", insertText: "$_DEFAULTCHARDELAY = 20", summary: "Read/write the delay (ms) between typed characters." },
        { token: "$_CHARJITTER", category: "Variables & Math", badge: "VAR", syntax: "$_CHARJITTER", insertText: "$_CHARJITTER = 5", summary: "Read/write random character delay jitter (ms)." },
        { token: "$_TIME_MS", category: "Variables & Math", badge: "VAR", syntax: "$_TIME_MS", insertText: "$_TIME_MS", summary: "Milliseconds elapsed since duckyPad booted." },
        { token: "$_TIME_S", category: "Variables & Math", badge: "VAR", syntax: "$_TIME_S", insertText: "$_TIME_S", summary: "Seconds elapsed since duckyPad booted." },
        { token: "$_READKEY", category: "Variables & Math", badge: "VAR", syntax: "$_READKEY", insertText: "$_READKEY", summary: "Non-blocking read of currently pressed key ID (0 if none)." },
        { token: "$_BLOCKING_READKEY", category: "Variables & Math", badge: "VAR", syntax: "$_BLOCKING_READKEY", insertText: "$_BLOCKING_READKEY", summary: "Wait for any key press and return its key ID." },
        { token: "$_THIS_KEYID", category: "Variables & Math", badge: "VAR", syntax: "$_THIS_KEYID", insertText: "$_THIS_KEYID", summary: "Key slot ID (1..20) that triggered the currently running script." },
        { token: "$_KEYPRESS_COUNT", category: "Variables & Math", badge: "VAR", syntax: "$_KEYPRESS_COUNT", insertText: "$_KEYPRESS_COUNT", summary: "Number of times this key has been pressed." },
        { token: "$_ALLOW_ABORT", category: "Variables & Math", badge: "VAR", syntax: "$_ALLOW_ABORT = 1", insertText: "$_ALLOW_ABORT = 1", summary: "Set to 1 to allow pressing any key to abort the running script." },
        { token: "$_DONT_REPEAT", category: "Variables & Math", badge: "VAR", syntax: "$_DONT_REPEAT = 1", insertText: "$_DONT_REPEAT = 1", summary: "Set to 1 to disable auto-repeat when holding down the key." },
        { token: "$_SW_BITFIELD", category: "Variables & Math", badge: "VAR", syntax: "$_SW_BITFIELD", insertText: "$_SW_BITFIELD", summary: "Bitmask of all currently pressed physical switches on the duckyPad." },
        { token: "$_KBLED_BITFIELD", category: "Variables & Math", badge: "VAR", syntax: "$_KBLED_BITFIELD", insertText: "$_KBLED_BITFIELD", summary: "Host keyboard LED state bitmask (NumLock=1, CapsLock=2, ScrollLock=4)." },
        { token: "_IS_NUMLOCK_ON", category: "Variables & Math", badge: "VAR", syntax: "_IS_NUMLOCK_ON", insertText: "_IS_NUMLOCK_ON", summary: "Evaluates to 1 if host Num Lock LED is on, 0 otherwise." },
        { token: "_IS_CAPSLOCK_ON", category: "Variables & Math", badge: "VAR", syntax: "_IS_CAPSLOCK_ON", insertText: "_IS_CAPSLOCK_ON", summary: "Evaluates to 1 if host Caps Lock LED is on, 0 otherwise." },
        { token: "_IS_SCROLLLOCK_ON", category: "Variables & Math", badge: "VAR", syntax: "_IS_SCROLLLOCK_ON", insertText: "_IS_SCROLLLOCK_ON", summary: "Evaluates to 1 if host Scroll Lock LED is on, 0 otherwise." },
        { token: "$_RTC_HOUR", category: "Variables & Math", badge: "VAR", syntax: "$_RTC_YEAR / $_RTC_MONTH / $_RTC_DAY / $_RTC_HOUR / $_RTC_MINUTE / $_RTC_SECOND", insertText: "$_RTC_HOUR", summary: "Real-Time Clock fields synced from DuckyDeck Studio (also $_RTC_IS_VALID, $_RTC_WDAY)." },

        // Profiles & System
        { token: "GOTO_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "GOTO_PROFILE <name_or_num>", insertText: "GOTO_PROFILE ", summary: "Switch immediately to a profile by number or exact name." },
        { token: "NEXT_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "NEXT_PROFILE", insertText: "NEXT_PROFILE", summary: "Switch to the next profile in sequence." },
        { token: "PREV_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "PREV_PROFILE", insertText: "PREV_PROFILE", summary: "Switch to the previous profile in sequence." },
        { token: "SKIP_PROFILE", category: "Profiles & System", badge: "PROF", syntax: "SKIP_PROFILE <offset>", insertText: "SKIP_PROFILE 1", summary: "Jump forward/backward by <offset> profiles." },
        { token: "DP_SLEEP", category: "Profiles & System", badge: "PROF", syntax: "DP_SLEEP", insertText: "DP_SLEEP", summary: "Turn off OLED and RGB LEDs and enter low-power sleep mode." },
        { token: "BCLR", category: "Profiles & System", badge: "PROF", syntax: "BCLR", insertText: "BCLR", summary: "Clear the internal key press queue buffer." },
        { token: "HIDTX", category: "Profiles & System", badge: "PROF", syntax: "HIDTX <addr>", insertText: "HIDTX ", summary: "Transmit raw USB HID buffer from memory address <addr>." },
        { token: "PEEK8", category: "Profiles & System", badge: "FUNC", syntax: "PEEK8(<addr>) / PEEK16 / PEEK32", insertText: "PEEK8(", summary: "Read signed/unsigned 8, 16, or 32-bit value from VM memory address." },
        { token: "POKE8", category: "Profiles & System", badge: "FUNC", syntax: "POKE8(<addr>, <val>) / POKE16 / POKE32", insertText: "POKE8(", summary: "Write 8, 16, or 32-bit value to VM memory address." }
    ]

    function badgeColorForType(badge) {
        if (root.isLightTheme) {
            if (badge === "CMD") return "#0369a1"
            if (badge === "KEY") return "#6d28d9"
            if (badge === "FLOW") return "#b45309"
            if (badge === "OLED") return "#0f766e"
            if (badge === "LED") return "#be185d"
            if (badge === "VAR") return "#15803d"
            if (badge === "FUNC") return "#1d4ed8"
            if (badge === "PROF") return "#c2410c"
            return "#334155"
        }
        if (badge === "CMD") return "#38bdf8"
        if (badge === "KEY") return "#a78bfa"
        if (badge === "FLOW") return "#f59e0b"
        if (badge === "OLED") return "#2dd4bf"
        if (badge === "LED") return "#f472b6"
        if (badge === "VAR") return "#4ade80"
        if (badge === "FUNC") return "#60a5fa"
        if (badge === "PROF") return "#fb923c"
        return "#94a3b8"
    }

    function refreshCommandReferenceModel() {
        commandReferenceModel.clear()
        const query = (root.commandRefSearchText || "").trim().toLowerCase()
        const cat = root.commandRefCategory || "All"
        for (let i = 0; i < root.duckyScriptCatalog.length; ++i) {
            const item = root.duckyScriptCatalog[i]
            if (cat !== "All" && item.category !== cat) {
                continue
            }
            if (query.length > 0) {
                const hay = (item.token + " " + item.syntax + " " + item.summary + " " + item.badge).toLowerCase()
                if (hay.indexOf(query) === -1) {
                    continue
                }
            }
            commandReferenceModel.append(item)
        }
    }

    function collectDynamicScriptSymbols(scriptText) {
        const extra = []
        const seen = {}
        if (!scriptText) return extra
        const lines = scriptText.split("\n")
        for (let i = 0; i < lines.length; ++i) {
            const trimmed = lines[i].trim()
            if (trimmed.indexOf("VAR ") === 0) {
                const eqIdx = trimmed.indexOf("=")
                if (eqIdx > 4) {
                    let rawName = trimmed.slice(4, eqIdx).trim()
                    if (rawName.length > 0) {
                        const withDollar = rawName.charAt(0) === "$" ? rawName : ("$" + rawName)
                        if (!seen[withDollar]) {
                            seen[withDollar] = true
                            extra.push({
                                token: withDollar,
                                category: "Variables & Math",
                                badge: "USER",
                                syntax: withDollar,
                                insertText: withDollar,
                                summary: "User variable declared in this script (line " + (i + 1) + ")."
                            })
                        }
                    }
                }
            } else if (trimmed.indexOf("DEFINE ") === 0) {
                const rest = trimmed.slice(7).trim()
                const sp = rest.indexOf(" ")
                const defName = (sp > 0 ? rest.slice(0, sp) : rest).trim()
                if (defName.length > 0 && !seen[defName]) {
                    seen[defName] = true
                    extra.push({
                        token: defName,
                        category: "Variables & Math",
                        badge: "USER",
                        syntax: "DEFINE " + defName,
                        insertText: defName,
                        summary: "User DEFINE macro declared in this script (line " + (i + 1) + ")."
                    })
                }
            } else if (trimmed.indexOf("FUN ") === 0) {
                const rest = trimmed.slice(4).trim()
                const paren = rest.indexOf("(")
                const fnName = (paren > 0 ? rest.slice(0, paren) : rest).trim()
                if (fnName.length > 0 && !seen[fnName]) {
                    seen[fnName] = true
                    extra.push({
                        token: fnName,
                        category: "Flow & Loops",
                        badge: "USER",
                        syntax: rest,
                        insertText: fnName + "()",
                        summary: "User function defined in this script (line " + (i + 1) + ")."
                    })
                }
            }
        }
        return extra
    }

    function updateAutocompleteSuggestions(forcePopup) {
        if (root.loadingSlotData || root.insertingCompletion || (!root.autocompleteEnabled && !forcePopup)) {
            root.autocompleteVisible = false
            return
        }
        const text = scriptEditor.text || ""
        const cursor = scriptEditor.cursorPosition
        if (cursor < 0 || cursor > text.length) {
            root.autocompleteVisible = false
            return
        }

        // Find start of current line and start of current token
        let lineStart = cursor
        while (lineStart > 0 && text.charAt(lineStart - 1) !== "\n") {
            lineStart--
        }
        const lineBeforeCursor = text.slice(lineStart, cursor)
        const trimmedLine = lineBeforeCursor.replace(/^\s+/, "")

        // Suppress automatic popup inside literal text/comment lines unless typing a $variable or forcing via Ctrl+Space
        const firstSpaceIdx = trimmedLine.indexOf(" ")
        const firstWord = firstSpaceIdx >= 0 ? trimmedLine.slice(0, firstSpaceIdx) : ""
        const inLiteralOrComment = (
            trimmedLine.indexOf("//") === 0 ||
            firstWord === "REM" ||
            firstWord === "STRING" ||
            firstWord === "STRINGLN" ||
            firstWord === "OLED_PRINT" ||
            firstWord === "OLED_CPRINT" ||
            firstWord === "OLED_LPRINT"
        )

        let tokStart = cursor
        while (tokStart > lineStart) {
            const ch = text.charAt(tokStart - 1)
            if ((ch >= "A" && ch <= "Z") || (ch >= "a" && ch <= "z") || (ch >= "0" && ch <= "9") || ch === "_" || ch === "$") {
                tokStart--
            } else {
                break
            }
        }
        let tokEnd = cursor
        while (tokEnd < text.length) {
            const ch = text.charAt(tokEnd)
            if ((ch >= "A" && ch <= "Z") || (ch >= "a" && ch <= "z") || (ch >= "0" && ch <= "9") || ch === "_" || ch === ":") {
                tokEnd++
            } else {
                break
            }
        }

        const prefix = text.slice(tokStart, cursor)
        if (!forcePopup) {
            if (prefix.length < 1) {
                root.autocompleteVisible = false
                return
            }
            if (inLiteralOrComment && prefix.charAt(0) !== "$" && prefix.charAt(0) !== "_") {
                root.autocompleteVisible = false
                return
            }
        }

        const normPrefix = prefix.toUpperCase()
        const normNoDollar = normPrefix.indexOf("$") === 0 ? normPrefix.slice(1) : normPrefix
        const prefixMatches = []
        const substringMatches = []

        const dynamicSyms = root.collectDynamicScriptSymbols(text)
        const combined = dynamicSyms.concat(root.duckyScriptCatalog)

        // Also include $_GV0..$_GV31 when user types $_GV or _GV
        if (normNoDollar.indexOf("_GV") === 0) {
            for (let g = 1; g <= 31; ++g) {
                const gvTok = "$_GV" + g
                combined.push({
                    token: gvTok,
                    category: "Variables & Math",
                    badge: "VAR",
                    syntax: gvTok,
                    insertText: gvTok,
                    summary: "Persistent Global Variable #" + g + " (shared across profiles)."
                })
            }
        }

        for (let i = 0; i < combined.length; ++i) {
            const entry = combined[i]
            const tokUpper = entry.token.toUpperCase()
            const tokNoDollar = tokUpper.indexOf("$") === 0 ? tokUpper.slice(1) : tokUpper
            if (normPrefix.length === 0) {
                prefixMatches.push(entry)
            } else if (tokUpper === normPrefix || tokNoDollar === normNoDollar) {
                // If user already typed the exact full token and didn't press Ctrl+Space, don't keep showing single identical match
                if (forcePopup) {
                    prefixMatches.push(entry)
                }
            } else if (tokUpper.indexOf(normPrefix) === 0 || (normNoDollar.length > 0 && tokNoDollar.indexOf(normNoDollar) === 0)) {
                prefixMatches.push(entry)
            } else if (normPrefix.length >= 2 && (tokUpper.indexOf(normPrefix) > 0 || tokNoDollar.indexOf(normNoDollar) > 0)) {
                substringMatches.push(entry)
            }
        }

        const results = prefixMatches.concat(substringMatches).slice(0, 10)
        autocompleteModel.clear()
        for (let j = 0; j < results.length; ++j) {
            autocompleteModel.append(results[j])
        }

        root.autocompleteTokenStart = tokStart
        root.autocompleteTokenEnd = tokEnd
        root.autocompletePrefix = prefix
        root.autocompleteIndex = 0
        root.autocompleteVisible = autocompleteModel.count > 0
    }

    function applyAutocompleteSelection(idx) {
        if (idx < 0 || idx >= autocompleteModel.count) return
        const item = autocompleteModel.get(idx)
        if (!item) return
        const replacement = item.insertText || item.token
        const start = root.autocompleteTokenStart
        const end = root.autocompleteTokenEnd
        root.insertingCompletion = true
        scriptEditor.remove(start, end)
        scriptEditor.insert(start, replacement)
        scriptEditor.cursorPosition = start + replacement.length
        root.insertingCompletion = false
        root.autocompleteVisible = false
        scriptEditor.forceActiveFocus()
    }

    function insertCommandFromReference(snippetText) {
        if (!snippetText) return
        const text = scriptEditor.text || ""
        const cur = scriptEditor.cursorPosition
        const needsLeadingNewline = cur > 0 && text.charAt(cur - 1) !== "\n" && text.charAt(cur - 1) !== " "
        const needsTrailingNewline = snippetText.slice(-1) !== " " && snippetText.slice(-1) !== "(" && snippetText.slice(-1) !== "\n"
        const toInsert = (needsLeadingNewline ? "\n" : "") + snippetText + (needsTrailingNewline ? "\n" : "")
        root.insertingCompletion = true
        scriptEditor.insert(cur, toInsert)
        root.insertingCompletion = false
        root.autocompleteVisible = false
        scriptEditor.forceActiveFocus()
    }

    ListModel {
        id: sidebarProfileModel
    }
    ListModel {
        id: unsyncedItemsModel
    }
    ListModel {
        id: eventLogModel
    }
    property string eventLogSearchText: ""
    property string eventLogCategoryFilter: "ALL"
    property string eventLogLevelFilter: "ALL"
    readonly property var eventLogCategories: [
        "ALL",
        "DEVICE",
        "PROFILE",
        "AUTOSWITCH",
        "SD_SYNC",
        "SCRIPT",
        "WEB_API",
        "APP_LAUNCH",
        "SYSTEM"
    ]
    readonly property var eventLogLevels: ["ALL", "INFO", "WARN", "ERROR"]

    function levelColorForLog(lvl) {
        if (lvl === "ERROR") return root.isLightTheme ? "#b91c1c" : Kirigami.Theme.negativeTextColor
        if (lvl === "WARN") return root.isLightTheme ? "#b45309" : Kirigami.Theme.neutralTextColor
        return root.highContrastHighlightText
    }

    function categoryColorForLog(cat) {
        if (root.isLightTheme) {
            if (cat === "DEVICE") return "#0369a1"
            if (cat === "PROFILE") return "#6d28d9"
            if (cat === "AUTOSWITCH") return "#15803d"
            if (cat === "SD_SYNC") return "#b45309"
            if (cat === "SCRIPT") return "#0f766e"
            if (cat === "WEB_API") return "#be185d"
            if (cat === "APP_LAUNCH") return "#c2410c"
            return "#334155"
        }
        if (cat === "DEVICE") return "#38bdf8"
        if (cat === "PROFILE") return "#a78bfa"
        if (cat === "AUTOSWITCH") return "#4ade80"
        if (cat === "SD_SYNC") return "#f59e0b"
        if (cat === "SCRIPT") return "#2dd4bf"
        if (cat === "WEB_API") return "#f472b6"
        if (cat === "APP_LAUNCH") return "#fb923c"
        return "#94a3b8"
    }

    function refreshEventLogModel() {
        if (!deviceController) return
        eventLogModel.clear()
        const raw = deviceController.eventLogJson || "[]"
        const catFilter = (root.eventLogCategoryFilter || "ALL").toUpperCase()
        const lvlFilter = (root.eventLogLevelFilter || "ALL").toUpperCase()
        const query = (root.eventLogSearchText || "").trim().toLowerCase()
        try {
            const arr = JSON.parse(raw)
            if (!Array.isArray(arr)) return
            // Show newest events at the bottom or top; newest first or chronological? Chronological with auto-scroll to bottom (or newest at bottom)
            for (let i = 0; i < arr.length; ++i) {
                const it = arr[i]
                const lvl = (it.level || "INFO").toUpperCase()
                const cat = (it.category || "SYSTEM").toUpperCase()
                const msg = it.message || ""
                const ts = it.timestamp || ""
                if (catFilter !== "ALL" && cat !== catFilter) continue
                if (lvlFilter !== "ALL" && lvl !== lvlFilter) continue
                if (query.length > 0) {
                    const hay = (ts + " " + lvl + " " + cat + " " + msg).toLowerCase()
                    if (hay.indexOf(query) === -1) continue
                }
                eventLogModel.append({
                    timestamp: ts,
                    level: lvl,
                    category: cat,
                    message: msg
                })
            }
            if (eventLogListView.count > 0) {
                eventLogListView.positionViewAtEnd()
            }
        } catch (e) {}
    }

    property string unsyncedHeadline: ""
    property int unsyncedTotalChanges: 0

    function refreshUnsyncedSummaryModel() {
        if (!deviceController) return
        root.flushPendingSlotEditsIfModified()
        const raw = deviceController.refreshUnsyncedChanges()
        unsyncedItemsModel.clear()
        try {
            const parsed = JSON.parse(raw || "{}")
            root.unsyncedHeadline = parsed.headline || ""
            root.unsyncedTotalChanges = parsed.totalChanges || 0
            const items = Array.isArray(parsed.items) ? parsed.items : []
            for (let i = 0; i < items.length; ++i) {
                const it = items[i]
                const detailsArr = Array.isArray(it.details) ? it.details : []
                const detailItemsArr = Array.isArray(it.detailItems)
                    ? it.detailItems
                    : detailsArr.map(function(d) { return { changeKey: "", text: d } })
                unsyncedItemsModel.append({
                    kind: it.kind || "modified",
                    profileNumber: it.profileNumber || 0,
                    profileName: it.profileName || "",
                    summary: it.summary || "",
                    detailsText: detailsArr.map(function(d) { return "  • " + d }).join("\n"),
                    detailItemsJson: JSON.stringify(detailItemsArr)
                })
            }
        } catch (e) {
            root.unsyncedHeadline = qsTr("Unable to parse unsynced changes summary.")
            root.unsyncedTotalChanges = 0
        }
    }

    function discardSingleUnsyncedChange(profileNumber, profileName, changeKey) {
        if (!deviceController || !changeKey) return
        root.flushPendingSlotEditsIfModified()
        if (deviceController.discardUnsyncedChange(profileNumber, profileName, changeKey)) {
            hardwareCanvas.syncSlotsFromController()
            if (root.hasSelectedSlot) {
                root.loadSlotIntoInspector(root.selectedSlotNumber)
            }
            root.refreshUnsyncedSummaryModel()
        }
    }

    function formatUnsyncedSummaryPlainText() {
        if (!deviceController) return ""
        try {
            const parsed = JSON.parse(deviceController.unsyncedChangesJson || "{}")
            let lines = []
            if (parsed.headline) {
                lines.push(parsed.headline)
            }
            const items = Array.isArray(parsed.items) ? parsed.items : []
            for (let i = 0; i < items.length; ++i) {
                const it = items[i]
                lines.push("")
                lines.push("▸ " + (it.summary || ("Profile #" + it.profileNumber)))
                const detailsArr = Array.isArray(it.details) ? it.details : []
                for (let j = 0; j < detailsArr.length; ++j) {
                    lines.push("   • " + detailsArr[j])
                }
            }
            return lines.join("\n")
        } catch (e) {
            return ""
        }
    }

    function openUnsyncedChangesDialog() {
        root.refreshUnsyncedSummaryModel()
        unsyncedChangesDialog.open()
    }

    function findWebApiDestinationIndex(nameStr, methodStr, urlStr, headersStr) {
        const list = root.webApiDestinationsList || []
        const cleanName = (nameStr || "").trim().toLowerCase()
        if (cleanName.length > 0) {
            for (let i = 0; i < list.length; ++i) {
                if ((list[i].name || "").trim().toLowerCase() === cleanName) {
                    return i
                }
            }
        }
        const cleanUrl = (urlStr || "").trim().toLowerCase()
        const cleanMethod = ((methodStr || "POST").trim().toUpperCase())
        const cleanHeaders = (headersStr || "").trim()
        if (cleanUrl.length > 0) {
            for (let i = 0; i < list.length; ++i) {
                if ((list[i].url || "").trim().toLowerCase() === cleanUrl
                    && ((list[i].method || "POST").trim().toUpperCase()) === cleanMethod
                    && (list[i].headers || "").trim() === cleanHeaders) {
                    return i
                }
            }
        }
        return -1
    }

    function buildWebApiConfigJson() {
        const methodIdx = Math.max(0, webApiMethodCombo.currentIndex)
        const fkeyIdx = Math.max(0, webApiFkeyCombo.currentIndex)
        const cfg = {
            enabled: webApiEnabledCheck.checked,
            destination: (root.webApiSelectedDestination || "").trim(),
            method: root.webApiMethods[methodIdx] || "POST",
            url: webApiUrlField.text.trim(),
            headers: webApiHeadersField.text.trim(),
            body: webApiBodyField.text,
            oledText: webApiOledField.text.trim(),
            ledFeedback: webApiLedCheck.checked,
            triggerMode: webApiTriggerModeCombo.currentIndex === 1 ? "fkey" : "pgv",
            fkey: root.webApiFkeys[fkeyIdx] || "F13"
        }
        return JSON.stringify(cfg)
    }

    function loadWebApiBuilderFromScript(scriptText) {
        if (!deviceController) return
        root.syncingWebApiScript = true
        try {
            const rawJson = deviceController.extractScriptWebApiJson(scriptText, root.selectedSlotNumber)
            const cfg = JSON.parse(rawJson)
            const isEnabled = !!cfg.enabled
            root.webApiEnabled = isEnabled
            webApiEnabledCheck.checked = isEnabled
            const mIdx = root.webApiMethods.indexOf((cfg.method || "POST").toUpperCase())
            webApiMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
            webApiUrlField.text = cfg.url || ""
            webApiHeadersField.text = cfg.headers || ""
            webApiBodyField.text = cfg.body || ""
            webApiOledField.text = cfg.oledText || ""
            webApiLedCheck.checked = cfg.ledFeedback !== undefined ? !!cfg.ledFeedback : true
            webApiTriggerModeCombo.currentIndex = cfg.triggerMode === "fkey" ? 1 : 0
            const fIdx = root.webApiFkeys.indexOf((cfg.fkey || "F13").toUpperCase())
            webApiFkeyCombo.currentIndex = fIdx >= 0 ? fIdx : ((root.selectedSlotNumber - 1) % 12)

            const matchedIdx = root.findWebApiDestinationIndex(
                cfg.destination || "",
                cfg.method || "POST",
                cfg.url || "",
                cfg.headers || ""
            )
            if (matchedIdx >= 0 && matchedIdx < root.webApiDestinationsList.length) {
                const d = root.webApiDestinationsList[matchedIdx]
                root.webApiSelectedDestination = d.name
                webApiDestinationCombo.currentIndex = matchedIdx + 1
            } else {
                root.webApiSelectedDestination = (cfg.destination || "").trim()
                webApiDestinationCombo.currentIndex = 0
            }

            if (isEnabled) {
                root.showWebApiBuilder = true
            }
        } catch (e) {
            root.webApiEnabled = false
            root.webApiSelectedDestination = ""
            webApiEnabledCheck.checked = false
            webApiDestinationCombo.currentIndex = 0
        }
        root.syncingWebApiScript = false
    }

    function syncWebApiBuilderToScript() {
        if (root.loadingSlotData || root.syncingWebApiScript || !deviceController) return
        root.syncingWebApiScript = true
        root.webApiEnabled = webApiEnabledCheck.checked
        const cfgJson = root.buildWebApiConfigJson()
        const updated = deviceController.applyWebApiToScript(
            scriptEditor.text,
            cfgJson,
            root.selectedSlotNumber,
            root.editingReleaseScript
        )
        scriptEditor.text = updated
        if (root.editingReleaseScript) {
            root.releaseScriptBuffer = updated
        } else {
            root.pressScriptBuffer = updated
        }
        root.syncingWebApiScript = false
        root.updateCompilerStatus()
        if (root.compileOk) {
            root.saveCurrentSlotToController()
            hardwareCanvas.syncSlotsFromController()
        }
    }

    function buildAppLaunchConfigJson() {
        const modeIdx = Math.max(0, appLaunchTriggerModeCombo.currentIndex)
        const fkeyIdx = Math.max(0, appLaunchFkeyCombo.currentIndex)
        const cfg = {
            enabled: appLaunchEnabledCheck.checked,
            appName: root.appLaunchAppName || "",
            desktopId: root.appLaunchDesktopId || "",
            command: appLaunchCommandField.text.trim(),
            args: appLaunchArgsField.text.trim(),
            workingDir: appLaunchWorkDirField.text.trim(),
            runInTerminal: appLaunchTerminalCheck.checked,
            oledText: appLaunchOledField.text.trim(),
            ledFeedback: appLaunchLedCheck.checked,
            triggerMode: root.appLaunchTriggerModes[modeIdx] || "pgv",
            fkey: root.webApiFkeys[fkeyIdx] || "F13"
        }
        return JSON.stringify(cfg)
    }

    function loadAppLaunchBuilderFromScript(scriptText) {
        if (!deviceController) return
        root.syncingAppLaunchScript = true
        try {
            const rawJson = deviceController.extractScriptAppLaunchJson(scriptText, root.selectedSlotNumber)
            const cfg = JSON.parse(rawJson)
            const isEnabled = !!cfg.enabled
            root.appLaunchEnabled = isEnabled
            appLaunchEnabledCheck.checked = isEnabled
            root.appLaunchAppName = cfg.appName || ""
            root.appLaunchDesktopId = cfg.desktopId || ""
            appLaunchCommandField.text = cfg.command || ""
            appLaunchArgsField.text = cfg.args || ""
            appLaunchWorkDirField.text = cfg.workingDir || ""
            appLaunchTerminalCheck.checked = !!cfg.runInTerminal
            appLaunchOledField.text = cfg.oledText || ""
            appLaunchLedCheck.checked = cfg.ledFeedback !== undefined ? !!cfg.ledFeedback : true
            const tMode = (cfg.triggerMode || "pgv").toLowerCase()
            const mIdx = root.appLaunchTriggerModes.indexOf(tMode)
            appLaunchTriggerModeCombo.currentIndex = mIdx >= 0 ? mIdx : 0
            const fIdx = root.webApiFkeys.indexOf((cfg.fkey || "F13").toUpperCase())
            appLaunchFkeyCombo.currentIndex = fIdx >= 0 ? fIdx : ((root.selectedSlotNumber - 1) % 12)
            if (isEnabled) {
                root.showAppLaunchBuilder = true
            }
        } catch (e) {
            root.appLaunchEnabled = false
            appLaunchEnabledCheck.checked = false
            root.appLaunchAppName = ""
            root.appLaunchDesktopId = ""
        }
        root.syncingAppLaunchScript = false
    }

    function syncAppLaunchBuilderToScript() {
        if (root.loadingSlotData || root.syncingAppLaunchScript || !deviceController) return
        root.syncingAppLaunchScript = true
        root.appLaunchEnabled = appLaunchEnabledCheck.checked
        const cfgJson = root.buildAppLaunchConfigJson()
        const updated = deviceController.applyAppLaunchToScript(
            scriptEditor.text,
            cfgJson,
            root.selectedSlotNumber,
            root.editingReleaseScript
        )
        scriptEditor.text = updated
        if (root.editingReleaseScript) {
            root.releaseScriptBuffer = updated
        } else {
            root.pressScriptBuffer = updated
        }
        root.syncingAppLaunchScript = false
        root.updateCompilerStatus()
        if (root.compileOk) {
            root.saveCurrentSlotToController()
            hardwareCanvas.syncSlotsFromController()
        }
    }

    function refreshInstalledAppsModel() {
        if (!deviceController) return
        installedAppsModel.clear()
        const raw = deviceController.installedAppsJson || "[]"
        const query = (root.installedAppSearchText || "").trim().toLowerCase()
        try {
            const arr = JSON.parse(raw)
            if (!Array.isArray(arr)) return
            for (let i = 0; i < arr.length; ++i) {
                const app = arr[i]
                const name = app.name || ""
                const desktopId = app.desktopId || ""
                const execCmd = app.execCommand || app.execCmd || ""
                const execArgs = app.execArgs || ""
                const comment = app.comment || ""
                const categories = app.categories || ""
                if (query.length > 0) {
                    const hay = (name + " " + desktopId + " " + execCmd + " " + execArgs + " " + comment + " " + categories).toLowerCase()
                    if (!hay.includes(query)) continue
                }
                installedAppsModel.append({
                    desktopId: desktopId,
                    name: name,
                    execCmd: execCmd,
                    execArgs: execArgs,
                    icon: app.icon || "application-x-executable",
                    comment: comment,
                    terminal: !!app.terminal,
                    categories: categories
                })
            }
        } catch (e) {}
    }

    property var allProfileAnimationsStore: ({}) // { "P1": { profile: {...}, keys: { "1": {...} } } }

    function lightingStoreFilePath() {
        return (deviceController && deviceController.logFilePath)
            ? String(deviceController.logFilePath).replace(/duckydeck\.log$/, "lighting_animations.json")
            : ""
    }

    function defaultLightingConfig(isProfileScope) {
        return {
            enabled: false,
            trigger: isProfileScope ? "idle" : "keypress",
            preset: isProfileScope ? "wave_h" : "ripple",
            speed: 5,
            color1: "#00f0ff",
            color2: "#a855f7",
            idleTimeoutSec: 5,
            streamToHardware: true,
            embedInKeyScript: !isProfileScope
        }
    }

    function currentProfileAnimBucketKey() {
        const pnum = deviceController ? deviceController.activeProfile : 1
        const pname = (deviceController && deviceController.activeProfileName)
            ? String(deviceController.activeProfileName).trim()
            : ("Profile" + pnum)
        return "P" + pnum + ":" + pname
    }

    function getProfileLightingConfig() {
        const key = root.currentProfileAnimBucketKey()
        const bucket = (root.allProfileAnimationsStore || {})[key]
        if (bucket && bucket.profile) {
            return Object.assign(root.defaultLightingConfig(true), bucket.profile)
        }
        return root.defaultLightingConfig(true)
    }

    function getKeyLightingConfig(slotNum) {
        const key = root.currentProfileAnimBucketKey()
        const bucket = (root.allProfileAnimationsStore || {})[key]
        const sKey = String(slotNum)
        if (bucket && bucket.keys && bucket.keys[sKey]) {
            return Object.assign(root.defaultLightingConfig(false), bucket.keys[sKey])
        }
        // Also check if current slot script has an embedded // @DUCKYDECK_ANIM {...} comment
        if (slotNum === root.selectedSlotNumber && scriptEditor && scriptEditor.text) {
            const parsed = root.extractLightingAnimFromScript(scriptEditor.text)
            if (parsed) {
                return Object.assign(root.defaultLightingConfig(false), parsed)
            }
        }
        return root.defaultLightingConfig(false)
    }

    function extractLightingAnimFromScript(scriptText) {
        const lines = String(scriptText || "").split("\n")
        for (let i = 0; i < lines.length; ++i) {
            const trimmed = lines[i].trim()
            if (trimmed.indexOf("// @DUCKYDECK_ANIM ") === 0) {
                try {
                    const obj = JSON.parse(trimmed.slice(19).trim())
                    if (obj && typeof obj === "object") return obj
                } catch (e) {}
            }
        }
        return null
    }

    function buildKeypressDuckyScriptAnimationBlock(cfg, slotNum) {
        if (!cfg || !cfg.enabled) return ""
        const c1 = hardwareCanvas.parseHexRgb(cfg.color1 || "#00f0ff", 0, 240, 255)
        const c2 = hardwareCanvas.parseHexRgb(cfg.color2 || "#a855f7", 168, 85, 247)
        const stepDelay = Math.max(20, Math.round(135 - (cfg.speed || 5) * 11))
        const targetKey = (slotNum >= 1 && slotNum <= 20) ? slotNum : 1
        const originCol = (targetKey - 1) % 4
        const originRow = Math.floor((targetKey - 1) / 4)
        const preset = cfg.preset || "ripple"
        const metaJson = JSON.stringify({
            enabled: true,
            trigger: cfg.trigger || "keypress",
            preset: preset,
            speed: cfg.speed || 5,
            color1: cfg.color1 || "#00f0ff",
            color2: cfg.color2 || "#a855f7",
            idleTimeoutSec: cfg.idleTimeoutSec || 5,
            streamToHardware: cfg.streamToHardware !== false,
            embedInKeyScript: cfg.embedInKeyScript !== false
        })

        const out = [
            "// @DUCKYDECK_ANIM " + metaJson,
            "// --- DuckyDeck Studio RGB Keypress Animation (" + preset + ") ---"
        ]

        if (preset === "pulse" || preset === "starlight") {
            const steps = [1.0, 0.75, 0.45, 0.2, 0.65, 1.0, 0.35]
            for (let s = 0; s < steps.length; ++s) {
                const w = steps[s]
                const r = Math.round(c2[0] + (c1[0] - c2[0]) * w)
                const g = Math.round(c2[1] + (c1[1] - c2[1]) * w)
                const b = Math.round(c2[2] + (c1[2] - c2[2]) * w)
                out.push("SWC_SET " + targetKey + " " + r + " " + g + " " + b)
                out.push("DELAY " + stepDelay)
            }
            out.push("SWC_RESET " + targetKey)
        } else if (preset === "ripple") {
            for (let ring = 0; ring <= 3; ++ring) {
                for (let k = 1; k <= 20; ++k) {
                    const kc = (k - 1) % 4
                    const kr = Math.floor((k - 1) / 4)
                    const d = Math.round(Math.hypot(kc - originCol, kr - originRow))
                    if (d === ring) {
                        out.push("SWC_SET " + k + " " + c1[0] + " " + c1[1] + " " + c1[2])
                    }
                }
                out.push("DELAY " + stepDelay)
                if (ring > 0) {
                    for (let k = 1; k <= 20; ++k) {
                        const kc = (k - 1) % 4
                        const kr = Math.floor((k - 1) / 4)
                        const d = Math.round(Math.hypot(kc - originCol, kr - originRow))
                        if (d === ring - 1) {
                            out.push("SWC_RESET " + k)
                        }
                    }
                }
            }
            out.push("SWC_RESET 0")
        } else if (preset === "wave_h" || preset === "scanner") {
            for (let col = 0; col < 4; ++col) {
                for (let row = 0; row < 5; ++row) {
                    const k = row * 4 + col + 1
                    out.push("SWC_SET " + k + " " + c1[0] + " " + c1[1] + " " + c1[2])
                }
                out.push("DELAY " + stepDelay)
                for (let row = 0; row < 5; ++row) {
                    const k = row * 4 + col + 1
                    out.push("SWC_RESET " + k)
                }
            }
            out.push("SWC_RESET 0")
        } else if (preset === "wave_v") {
            for (let row = 0; row < 5; ++row) {
                for (let col = 0; col < 4; ++col) {
                    const k = row * 4 + col + 1
                    out.push("SWC_SET " + k + " " + c1[0] + " " + c1[1] + " " + c1[2])
                }
                out.push("DELAY " + stepDelay)
                for (let col = 0; col < 4; ++col) {
                    const k = row * 4 + col + 1
                    out.push("SWC_RESET " + k)
                }
            }
            out.push("SWC_RESET 0")
        } else if (preset === "spectrum") {
            for (let f = 0; f < 6; ++f) {
                const rgb = hardwareCanvas.hsvToRgb(f / 6.0, 0.95, 1.0)
                out.push("SWC_SET " + targetKey + " " + rgb[0] + " " + rgb[1] + " " + rgb[2])
                out.push("DELAY " + stepDelay)
            }
            out.push("SWC_RESET " + targetKey)
        }
        out.push("// --- End DuckyDeck Studio RGB Animation ---")
        return out.join("\n")
    }

    function applyLightingAnimBlockToScript(existingScript, cfg, slotNum) {
        const lines = String(existingScript || "").split("\n")
        const cleaned = []
        let skippingBlock = false
        for (let i = 0; i < lines.length; ++i) {
            const t = lines[i].trim()
            if (t.indexOf("// @DUCKYDECK_ANIM ") === 0) {
                continue
            }
            if (t.indexOf("// --- DuckyDeck Studio RGB Keypress Animation") === 0) {
                skippingBlock = true
                continue
            }
            if (skippingBlock && t.indexOf("// --- End DuckyDeck Studio RGB Animation ---") === 0) {
                skippingBlock = false
                continue
            }
            if (!skippingBlock) {
                cleaned.push(lines[i])
            }
        }
        while (cleaned.length > 0 && cleaned[0].trim() === "") {
            cleaned.shift()
        }
        const body = cleaned.join("\n")
        if (!cfg || !cfg.enabled || cfg.embedInKeyScript === false) {
            return body
        }
        const block = root.buildKeypressDuckyScriptAnimationBlock(cfg, slotNum)
        return body.length > 0 ? (block + "\n\n" + body) : block
    }

    function syncLightingAnimationsForActiveProfile() {
        if (!hardwareCanvas) return
        const bKey = root.currentProfileAnimBucketKey()
        const store = Object.assign({}, root.allProfileAnimationsStore || {})
        if (!store[bKey]) {
            store[bKey] = { profile: root.defaultLightingConfig(true), keys: {} }
        }
        // Scan active profile slots for any embedded // @DUCKYDECK_ANIM metadata
        if (deviceController) {
            for (let s = 1; s <= 20; ++s) {
                const script = deviceController.getSlotScript(s, false)
                const embedded = root.extractLightingAnimFromScript(script)
                if (embedded && !store[bKey].keys[String(s)]) {
                    store[bKey].keys[String(s)] = embedded
                }
            }
        }
        root.allProfileAnimationsStore = store
        hardwareCanvas.profileAnimationConfig = Object.assign(root.defaultLightingConfig(true), store[bKey].profile || {})
        hardwareCanvas.keyAnimationConfigs = Object.assign({}, store[bKey].keys || {})
        hardwareCanvas.syncSlotsFromController()
    }

    function saveLightingAnimationConfig(scope, slotNum, cfg) {
        const bKey = root.currentProfileAnimBucketKey()
        const store = Object.assign({}, root.allProfileAnimationsStore || {})
        if (!store[bKey]) {
            store[bKey] = { profile: root.defaultLightingConfig(true), keys: {} }
        }
        if (scope === "profile") {
            store[bKey].profile = Object.assign({}, cfg)
            hardwareCanvas.profileAnimationConfig = Object.assign({}, cfg)
        } else {
            const sNum = (slotNum >= 1 && slotNum <= 20) ? slotNum : (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20 ? root.selectedSlotNumber : 1)
            const keysMap = Object.assign({}, store[bKey].keys || {})
            keysMap[String(sNum)] = Object.assign({}, cfg)
            store[bKey].keys = keysMap
            hardwareCanvas.keyAnimationConfigs = keysMap

            // If embedInKeyScript is enabled (or disabled to remove previous block), update the slot's press script
            if (deviceController) {
                const existingPress = (root.hasSelectedSlot && root.selectedSlotNumber === sNum && !root.editingReleaseScript)
                    ? scriptEditor.text
                    : deviceController.getSlotScript(sNum, false)
                const updatedPress = root.applyLightingAnimBlockToScript(existingPress, cfg, sNum)
                if (updatedPress !== existingPress) {
                    if (root.hasSelectedSlot && root.selectedSlotNumber === sNum) {
                        root.pressScriptBuffer = updatedPress
                        if (!root.editingReleaseScript) {
                            scriptEditor.text = updatedPress
                        }
                        root.updateCompilerStatus()
                        root.saveCurrentSlotToController()
                    } else {
                        const meta = JSON.parse(deviceController.getSlotMetaJson(sNum) || "{}")
                        const rel = deviceController.getSlotScript(sNum, true)
                        deviceController.saveSlotConfig(
                            sNum,
                            meta.l1 || "",
                            meta.l2 || "",
                            updatedPress,
                            rel,
                            !!meta.allowAbort,
                            !!meta.disableAutorepeat
                        )
                    }
                }
            }
        }
        root.allProfileAnimationsStore = store
        hardwareCanvas.syncSlotsFromController()
        hardwareCanvas.notifyUserActivity(scope === "key" ? slotNum : 1)
        if (cfg.enabled && (cfg.trigger === "keypress" || cfg.trigger === "both")) {
            hardwareCanvas.triggerKeypressBurst(
                scope === "key" ? slotNum : 10,
                cfg,
                scope
            )
        } else if (cfg.enabled && cfg.trigger === "idle") {
            hardwareCanvas.idleAnimationActive = true
        } else if (!cfg.enabled) {
            hardwareCanvas.resetAnimatedKeyColors()
        }
    }

    function openRenameProfileDialog(profileNumber, currentName) {
        root.contextMenuProfileNumber = profileNumber
        root.contextMenuProfileName = currentName
        renameProfileDialog.openWithProfile(profileNumber, currentName)
    }

    function updateCompilerStatus() {
        if (!deviceController) return
        try {
            const raw = deviceController.checkScriptSyntax(
                scriptEditor.text,
                allowAbortCheck.checked,
                disableRepeatCheck.checked
            )
            const res = JSON.parse(raw)
            root.compileOk = !!res.ok
            root.compileEmpty = !!res.empty
            root.compileByteCount = res.bytes || 0
            root.compileErrorLine = res.errorLine || 0
            root.compileErrorMsg = res.errorMsg || ""
        } catch (e) {
            root.compileOk = true
            root.compileEmpty = false
            root.compileByteCount = 0
            root.compileErrorLine = 0
            root.compileErrorMsg = ""
        }
    }

    property bool selectingProfileFromClick: false

    function ensureActiveProfileVisibleInList() {
        if (!deviceController || !profileSidebarList) return
        const targetProf = deviceController.activeProfile
        for (let i = 0; i < sidebarProfileModel.count; ++i) {
            if (sidebarProfileModel.get(i).profNumber === targetProf) {
                profileSidebarList.positionViewAtIndex(i, ListView.Contain)
                break
            }
        }
    }

    function syncSidebarProfilesFromController() {
        if (!deviceController || !deviceController.profilesJson) return
        const savedContentY = profileSidebarList ? profileSidebarList.contentY : 0
        let rulesByProfile = {}
        try {
            const rArr = JSON.parse(deviceController.autoSwitchRulesJson || "[]")
            if (Array.isArray(rArr)) {
                root.totalRuleCount = rArr.length
                for (let j = 0; j < rArr.length; ++j) {
                    const r = rArr[j]
                    if (!r.enabled) continue
                    const tp = r.targetProfile
                    if (!rulesByProfile[tp]) {
                        rulesByProfile[tp] = []
                    }
                    rulesByProfile[tp].push(r)
                }
            }
        } catch (e) {}

        try {
            const arr = JSON.parse(deviceController.profilesJson)
            if (!Array.isArray(arr)) return
            root.totalProfileCount = arr.length

            const filter = root.profileFilterText.trim().toLowerCase()
            const newItems = []
            for (let i = 0; i < arr.length; ++i) {
                const p = arr[i]
                const nameStr = p.name || "Default"
                const profRules = rulesByProfile[p.number] || []
                const boundStr = p.boundClass || (profRules.length > 0 ? (profRules[0].resourceClass || "") : "")
                const boundIconStr = p.boundIcon || (profRules.length > 0 ? (profRules[0].icon || "") : "")
                const numStr = String(p.number)
                let ruleSearchText = boundStr.toLowerCase()
                let tooltipParts = []
                for (let k = 0; k < profRules.length; ++k) {
                    const pr = profRules[k]
                    const clsDisplay = pr.resourceClass && pr.resourceClass.toLowerCase() === "*browser*"
                        ? qsTr("All Web Browsers")
                        : pr.resourceClass
                    const tPart = pr.titleContains && pr.titleContains.length > 0
                        ? (" (title: \"" + pr.titleContains + "\")")
                        : ""
                    tooltipParts.push(clsDisplay + tPart)
                    ruleSearchText += " " + (pr.resourceClass || "") + " " + (pr.titleContains || "")
                    if (pr.resourceClass && pr.resourceClass.toLowerCase() === "*browser*") {
                        ruleSearchText += " all web browsers chrome zen firefox edge brave opera vivaldi"
                    }
                }
                if (filter.length > 0
                    && !nameStr.toLowerCase().includes(filter)
                    && !ruleSearchText.toLowerCase().includes(filter)
                    && !numStr.includes(filter)) {
                    continue
                }
                const tooltipStr = tooltipParts.length > 0
                    ? qsTr("Auto-Switch Application: %1").arg(tooltipParts.join(", "))
                    : (boundStr.length > 0 ? qsTr("Auto-Switch Application: %1").arg(boundStr) : "")

                const bg = Array.isArray(p.bgColor) && p.bgColor.length === 3 ? p.bgColor : [0, 170, 255]
                newItems.push({
                    profNumber: p.number,
                    profName: nameStr,
                    slotCount: p.slotCount || 0,
                    boundClass: boundStr,
                    boundIcon: boundIconStr,
                    boundRuleCount: profRules.length > 0 ? profRules.length : (boundStr.length > 0 ? 1 : 0),
                    boundTooltip: tooltipStr,
                    bgR: bg[0],
                    bgG: bg[1],
                    bgB: bg[2]
                })
            }

            for (let idx = 0; idx < newItems.length; ++idx) {
                if (idx < sidebarProfileModel.count) {
                    sidebarProfileModel.set(idx, newItems[idx])
                } else {
                    sidebarProfileModel.append(newItems[idx])
                }
            }
            if (sidebarProfileModel.count > newItems.length) {
                sidebarProfileModel.remove(newItems.length, sidebarProfileModel.count - newItems.length)
            }

            if (profileSidebarList && savedContentY > 0) {
                const maxScroll = Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                profileSidebarList.contentY = Math.min(savedContentY, maxScroll)
            }
        } catch (e) {}
    }

    function clearSlotSelection() {
        root.flushPendingSlotEditsIfModified()
        root.selectedSlotNumber = 0
        hardwareCanvas.selectedSlot = 0
    }

    function captureSlotSnapshot() {
        if (!root.hasSelectedSlot) return
        root.slotSnapshotSlotNumber = root.selectedSlotNumber
        root.slotSnapshotProfileNumber = deviceController ? deviceController.activeProfile : 1
        root.slotSnapshotL1 = label1Input.text
        root.slotSnapshotL2 = label2Input.text
        root.slotSnapshotWL1 = widgetLabel1Input.text
        root.slotSnapshotWL2 = widgetLabel2Input.text
        root.slotSnapshotAbort = allowAbortCheck.checked
        root.slotSnapshotNoRepeat = disableRepeatCheck.checked
        root.slotSnapshotPressScript = root.pressScriptBuffer
        root.slotSnapshotReleaseScript = root.releaseScriptBuffer
        root.slotSnapshotColor = String(root.selectedSlotColor)
    }

    function revertSlotToSnapshot() {
        if (!root.hasSelectedSlot) return
        root.loadingSlotData = true
        label1Input.text = root.slotSnapshotL1
        label2Input.text = root.slotSnapshotL2
        widgetLabel1Input.text = root.slotSnapshotWL1
        widgetLabel2Input.text = root.slotSnapshotWL2
        allowAbortCheck.checked = root.slotSnapshotAbort
        disableRepeatCheck.checked = root.slotSnapshotNoRepeat
        root.pressScriptBuffer = root.slotSnapshotPressScript
        root.releaseScriptBuffer = root.slotSnapshotReleaseScript
        scriptEditor.text = root.editingReleaseScript ? root.releaseScriptBuffer : root.pressScriptBuffer
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript(scriptEditor.text)
        root.loadAppLaunchBuilderFromScript(scriptEditor.text)
        root.loadingSlotData = false
        root.updateCompilerStatus()
        root.saveCurrentSlotToController()
        hardwareCanvas.syncSlotsFromController()
    }

    function clearSelectedSlotNow() {
        if (!root.hasSelectedSlot) return
        root.loadingSlotData = true
        label1Input.text = ""
        label2Input.text = ""
        widgetLabel1Input.text = ""
        widgetLabel2Input.text = ""
        allowAbortCheck.checked = false
        disableRepeatCheck.checked = false
        root.pressScriptBuffer = ""
        root.releaseScriptBuffer = ""
        scriptEditor.text = ""
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript("")
        root.loadAppLaunchBuilderFromScript("")
        root.loadingSlotData = false
        root.updateCompilerStatus()
        if (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20) {
            deviceController.clearKeyColor(root.selectedSlotNumber)
        }
        root.saveCurrentSlotToController()
        hardwareCanvas.syncSlotsFromController()
        root.selectedSlotColor = hardwareCanvas.slotColorFor(root.selectedSlotNumber)
        root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
    }

    function loadSlotFromController(slotNumber, slotTitle, slotColor) {
        if (slotNumber <= 0 || slotNumber > 58) {
            root.selectedSlotNumber = 0
            return
        }
        const isDifferentSlotOrProfile = (
            root.slotSnapshotSlotNumber !== slotNumber ||
            root.slotSnapshotProfileNumber !== (deviceController ? deviceController.activeProfile : 1)
        )
        root.loadingSlotData = true
        root.selectedSlotNumber = slotNumber
        root.selectedSlotTitle = slotTitle
        root.selectedSlotColor = slotColor

        try {
            const metaJson = deviceController.getSlotMetaJson(slotNumber)
            const meta = JSON.parse(metaJson)
            label1Input.text = meta.l1 || ""
            label2Input.text = meta.l2 || ""
            widgetLabel1Input.text = meta.wl1 || ""
            widgetLabel2Input.text = meta.wl2 || ""
            allowAbortCheck.checked = !!meta.allowAbort
            disableRepeatCheck.checked = !!meta.disableAutorepeat
        } catch (e) {
            label1Input.text = ""
            label2Input.text = ""
            widgetLabel1Input.text = ""
            widgetLabel2Input.text = ""
            allowAbortCheck.checked = false
            disableRepeatCheck.checked = false
        }

        root.pressScriptBuffer = deviceController.getSlotScript(slotNumber, false)
        root.releaseScriptBuffer = deviceController.getSlotScript(slotNumber, true)
        scriptEditor.text = root.editingReleaseScript ? root.releaseScriptBuffer : root.pressScriptBuffer
        root.showWebApiBuilder = false
        root.showAppLaunchBuilder = false
        root.inspectorMode = "code"
        root.loadWebApiBuilderFromScript(scriptEditor.text)
        root.loadAppLaunchBuilderFromScript(scriptEditor.text)
        if (isDifferentSlotOrProfile) {
            root.captureSlotSnapshot()
        }
        root.loadingSlotData = false
        root.updateCompilerStatus()
    }

    function isDuckyDeckStudioFirmware(verStr) {
        const raw = String(verStr || "").trim()
        if (!raw || raw === "-") return false
        const lower = raw.toLowerCase()
        if (lower.indexOf("duckydeck") >= 0 || lower.indexOf("studio") >= 0 || lower.indexOf("dds") >= 0) {
            return true
        }
        const clean = raw.replace(/^[vV]/, "").split(/\s+/)[0]
        const parts = clean.split(".")
        if (parts.length < 3) return false
        const maj = parseInt(parts[0], 10) || 0
        const min = parseInt(parts[1], 10) || 0
        if (maj === 1) return true
        return (maj > 3) || (maj === 3 && min >= 4)
    }

    function cleanFirmwareVersionLabel(verStr) {
        const raw = String(verStr || "-").trim()
        if (!raw || raw === "-") return "-"
        return raw.replace(/\s*\(DuckyDeck Studio\)/i, "").replace(/^[vV]/, "").trim()
    }

    function refreshSidebarTemplateCombo() {
        if (typeof newProfileTemplateCombo !== "undefined" && newProfileTemplateCombo) {
            newProfileTemplateCombo.reloadTemplates()
        }
    }

    function visibleOledChars(str) {
        if (!str) return []
        const raw = Array.from(String(str))
        const out = []
        for (let i = 0; i < raw.length; i++) {
            const cp = raw[i].codePointAt(0)
            if ((cp === 0xFE0E || cp === 0xFE0F || cp === 0x200D) && out.length > 0) {
                out[out.length - 1] += raw[i]
            } else {
                out.push(raw[i])
            }
        }
        return out
    }

    function oledLabelCharCount(str) {
        return visibleOledChars(str).length
    }

    function clampOledLabel(str) {
        return visibleOledChars(str).slice(0, 5).join("")
    }

    function isSingleIconLabel(str) {
        const s = String(str || "").trim()
        if (!s) return false
        const chars = visibleOledChars(s)
        return chars.length === 1 && chars[0].codePointAt(0) > 127
    }

    function saveCurrentSlotToController() {
        if (!root.hasSelectedSlot) {
            return
        }
        if (root.editingReleaseScript) {
            root.releaseScriptBuffer = scriptEditor.text
        } else {
            root.pressScriptBuffer = scriptEditor.text
        }

        deviceController.saveSlotConfig(
            root.selectedSlotNumber,
            label1Input.text.trim(),
            label2Input.text.trim(),
            root.pressScriptBuffer,
            root.releaseScriptBuffer,
            allowAbortCheck.checked,
            disableRepeatCheck.checked
        )
        deviceController.saveSlotWidgetLabels(
            root.selectedSlotNumber,
            widgetLabel1Input.text.trim(),
            widgetLabel2Input.text.trim()
        )
    }

    function flushPendingSlotEditsIfModified() {
        if (!root.hasSelectedSlot || root.loadingSlotData || !root.compileOk) {
            return
        }
        const currentPress = root.editingReleaseScript ? root.pressScriptBuffer : scriptEditor.text
        const currentRelease = root.editingReleaseScript ? scriptEditor.text : root.releaseScriptBuffer
        const currentL1 = label1Input.text.trim()
        const currentL2 = label2Input.text.trim()
        const currentWL1 = widgetLabel1Input.text.trim()
        const currentWL2 = widgetLabel2Input.text.trim()
        const currentAbort = allowAbortCheck.checked
        const currentNoRepeat = disableRepeatCheck.checked

        try {
            const meta = JSON.parse(deviceController.getSlotMetaJson(root.selectedSlotNumber))
            const savedL1 = meta.l1 || ""
            const savedL2 = meta.l2 || ""
            const savedWL1 = meta.wl1 || ""
            const savedWL2 = meta.wl2 || ""
            const savedAbort = !!meta.allowAbort
            const savedNoRepeat = !!meta.disableAutorepeat
            const savedPress = deviceController.getSlotScript(root.selectedSlotNumber, false)
            const savedRelease = deviceController.getSlotScript(root.selectedSlotNumber, true)

            if (currentL1 !== savedL1 ||
                currentL2 !== savedL2 ||
                currentWL1 !== savedWL1 ||
                currentWL2 !== savedWL2 ||
                currentAbort !== savedAbort ||
                currentNoRepeat !== savedNoRepeat ||
                currentPress !== savedPress ||
                currentRelease !== savedRelease) {
                root.saveCurrentSlotToController()
                hardwareCanvas.syncSlotsFromController()
            }
        } catch (e) {}
    }

    function refreshSelectedSlotFromController() {
        if (sidebarProfileModel.count === 0) {
            root.syncSidebarProfilesFromController()
        }
        hardwareCanvas.syncSlotsFromController()
        if (root.hasSelectedSlot) {
            root.loadSlotFromController(
                root.selectedSlotNumber,
                hardwareCanvas.slotTitleFor(root.selectedSlotNumber),
                hardwareCanvas.slotColorFor(root.selectedSlotNumber)
            )
        }
    }

    property bool isDarkSystemTheme: true

    Connections {
        target: Kirigami.Theme
        function onBackgroundColorChanged() {
            const dark = Kirigami.Theme.backgroundColor.hslLightness < 0.5
            if (root.isDarkSystemTheme !== dark) {
                root.isDarkSystemTheme = dark
                Qt.callLater(function() {
                    if (deviceController) {
                        deviceController.notifySystemDarkTheme(root.isDarkSystemTheme)
                    }
                })
            }
        }
    }

    onVisibilityChanged: (visibility) => {
        if (visibility === Window.Minimized) {
            Qt.callLater(function() {
                if (deviceController) {
                    deviceController.minimizeToTray()
                }
                root.hide()
            })
        }
    }

    onClosing: (close) => {
        if (deviceController && deviceController.closeToTray) {
            close.accepted = false
            deviceController.minimizeToTray()
            root.hide()
        }
    }

    Component.onCompleted: {
        if (deviceController.isStartMinimizedMode()) {
            root.visible = false
            root.hide()
            deviceController.minimizeToTray()
        }
        root.isDarkSystemTheme = Kirigami.Theme.backgroundColor.hslLightness < 0.5
        deviceController.notifySystemDarkTheme(root.isDarkSystemTheme)
        root.refreshCommandReferenceModel()
        deviceController.refreshDevice()

        const startProf = deviceController.startupProfileArg()
        if (startProf.length > 0) {
            const parsedNum = parseInt(startProf, 10)
            if (!isNaN(parsedNum) && parsedNum >= 1 && parsedNum <= 64) {
                deviceController.switchProfile(parsedNum)
            } else {
                deviceController.switchProfileByName(startProf)
            }
        }

        root.syncSidebarProfilesFromController()
        hardwareCanvas.syncProfilesFromController()
        root.refreshSelectedSlotFromController()
        root.syncLightingAnimationsForActiveProfile()
        Qt.callLater(function() {
            if (deviceController.isStartMinimizedMode()) {
                root.visible = false
                root.hide()
                deviceController.minimizeToTray()
            }
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncProfilesFromController()
            root.refreshSelectedSlotFromController()
            root.syncLightingAnimationsForActiveProfile()
            if (deviceController.connected) {
                root.ensureActiveProfileVisibleInList()
            }
            if (deviceController.isStartupSyncSdRequested()) {
                root.triggerSyncToDeviceSd()
            }
        })
    }

    function triggerSyncToDeviceSd() {
        root.flushPendingSlotEditsIfModified()
        sdSyncDialog.open()
        deviceController.syncProfilesToDeviceSd()
    }

    function triggerReloadFromDeviceSd() {
        sdSyncDialog.open()
        deviceController.reloadProfilesFromDeviceSd()
    }

    DeviceController {
        id: deviceController
        onSdSyncBusyChanged: {
            if (deviceController.sdSyncBusy) {
                sdSyncDialog.open()
            } else {
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                root.refreshSelectedSlotFromController()
            }
        }
        onActiveProfileChanged: {
            if (!root.selectingProfileFromClick) {
                root.ensureActiveProfileVisibleInList()
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
            root.syncLightingAnimationsForActiveProfile()
        }
        onActiveWindowClassChanged: {
            if (deviceController.autoSwitchEnabled) {
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
        }
        onAutoSwitchEnabledChanged: {
            if (deviceController.autoSwitchEnabled) {
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
        }
        onActiveProfileSlotsJsonChanged: {
            root.refreshSelectedSlotFromController()
        }
        onProfilesJsonChanged: {
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncProfilesFromController()
        }
        onAutoSwitchRulesJsonChanged: {
            root.syncSidebarProfilesFromController()
        }
        onGeneratedScriptPreviewChanged: {
            if (deviceController.generatedScriptPreview.length > 0) {
                scriptEditor.text = deviceController.generatedScriptPreview
                if (root.editingReleaseScript) {
                    root.releaseScriptBuffer = scriptEditor.text
                } else {
                    root.pressScriptBuffer = scriptEditor.text
                }
            }
        }
        onStatusMessageChanged: {
            if (deviceController.statusMessage.length > 0) {
                deviceController.recordStatusLog(deviceController.statusMessage)
            }
        }
        onEventLogJsonChanged: {
            if (eventLogDialog.visible) {
                root.refreshEventLogModel()
            }
        }
    }

    FolderDialog {
        id: sdFolderDialog
        title: qsTr("Select duckyPad microSD Card Root or Profile Directory")
        onAccepted: {
            if (deviceController.loadSdDirectory(selectedFolder.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }

    FileDialog {
        id: importProfileFileDialog
        title: qsTr("Import DuckyDeck Profile (.duckyprofile.json or config.txt)")
        fileMode: FileDialog.OpenFile
        nameFilters: [
            qsTr("DuckyDeck Profiles (*.duckyprofile.json *.json config.txt)"),
            qsTr("All Files (*)")
        ]
        onAccepted: {
            if (deviceController.importProfileFromPath(selectedFile.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }

    FolderDialog {
        id: importProfileFolderDialog
        title: qsTr("Select duckyPad Profile Folder (profile_<Name>) to Import")
        onAccepted: {
            if (deviceController.importProfileFromPath(selectedFolder.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }

    property int contextMenuProfileNumber: 1
    property string contextMenuProfileName: "Default"
    property int contextMenuProfileIndex: 0
    property int exportTargetProfileNumber: 0

    function openProfileDefaultColorPicker(profNumber, profName, hexColor) {
        root.contextMenuProfileNumber = profNumber
        root.contextMenuProfileName = profName
        let initialHex = "#00aaff"
        if (hexColor && hexColor.length > 0) {
            initialHex = hexColor
        } else if (deviceController && deviceController.activeProfileBgColor.length > 0) {
            initialHex = deviceController.activeProfileBgColor
        }
        duckyColorPickerWindow.openForProfileBg(profNumber, profName, initialHex)
    }

    function openKeyColorPicker(slotNumber, hexColor) {
        const targetSlot = (slotNumber >= 1 && slotNumber <= 20) ? slotNumber : root.selectedSlotNumber
        if (targetSlot < 1 || targetSlot > 20) return
        const initialHex = (hexColor && hexColor.length > 0)
            ? hexColor
            : (hardwareCanvas ? hardwareCanvas.slotColorFor(targetSlot) : root.selectedSlotColor)
        duckyColorPickerWindow.openForKey(targetSlot, initialHex || "#00aaff")
    }

    FileDialog {
        id: saveProfileGroupFileDialog
        title: qsTr("Save All Profiles as Group Bundle (.duckygroup.json)")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "duckygroup.json"
        nameFilters: [
            qsTr("DuckyDeck Profile Group (*.duckygroup.json *.json)")
        ]
        onAccepted: {
            root.flushPendingSlotEditsIfModified()
            deviceController.saveProfileGroup(selectedFile.toString(), "")
        }
    }

    FileDialog {
        id: loadProfileGroupFileDialog
        title: qsTr("Load DuckyDeck Profile Group (.duckygroup.json)")
        fileMode: FileDialog.OpenFile
        nameFilters: [
            qsTr("DuckyDeck Profile Group (*.duckygroup.json *.json)"),
            qsTr("All Files (*)")
        ]
        onAccepted: {
            if (deviceController.loadProfileGroup(selectedFile.toString())) {
                root.refreshSelectedSlotFromController()
            }
        }
    }

    Window {
        id: newProfileGroupDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Create New Profile Group — DuckyDeck Studio")
        width: 590
        height: 255
        minimumWidth: 560
        minimumHeight: 240
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openWithDefaults() {
            newGroupNameInput.text = qsTr("New Group")
            newGroupFirstProfileInput.text = qsTr("Default")
            newProfileGroupDialog.show()
            newProfileGroupDialog.raise()
            newProfileGroupDialog.requestActivate()
            Qt.callLater(function() {
                newGroupNameInput.forceActiveFocus()
                newGroupNameInput.selectAll()
            })
        }

        function commitCreateNewGroup() {
            const groupName = newGroupNameInput.text.trim() || qsTr("New Group")
            const firstProf = newGroupFirstProfileInput.text.trim() || qsTr("Default")
            if (deviceController.createNewProfileGroup(groupName, firstProf)) {
                root.refreshSelectedSlotFromController()
                newProfileGroupDialog.close()
            }
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: newProfileGroupDialog.visible
            onActivated: newProfileGroupDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10
            Keys.onEscapePressed: newProfileGroupDialog.close()

            QQC2.Label {
                Layout.fillWidth: true
                Layout.minimumWidth: 0
                Layout.preferredWidth: 1
                wrapMode: Text.WordWrap
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.textColor
                text: qsTr("Start a fresh profile group workspace. Tip: Use 'Save Current Group...' first if you want to back up your existing profiles and Auto-Switcher rules.")
            }

            GridLayout {
                Layout.fillWidth: true
                columns: 2
                columnSpacing: 10
                rowSpacing: 8

                QQC2.Label {
                    text: qsTr("Group Name:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                QQC2.TextField {
                    id: newGroupNameInput
                    Layout.fillWidth: true
                    Layout.minimumWidth: 120
                    Layout.preferredWidth: 1
                    placeholderText: qsTr("e.g. Work & Coding")
                    selectByMouse: true
                    font.pointSize: 10 * root.fontScale
                    onAccepted: newProfileGroupDialog.commitCreateNewGroup()
                    Keys.onEscapePressed: newProfileGroupDialog.close()
                }

                QQC2.Label {
                    text: qsTr("Initial Profile #1:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                QQC2.TextField {
                    id: newGroupFirstProfileInput
                    Layout.fillWidth: true
                    Layout.minimumWidth: 120
                    Layout.preferredWidth: 1
                    maximumLength: 16
                    placeholderText: qsTr("Default")
                    selectByMouse: true
                    font.pointSize: 10 * root.fontScale
                    onAccepted: newProfileGroupDialog.commitCreateNewGroup()
                    Keys.onEscapePressed: newProfileGroupDialog.close()
                }
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "document-save-all"
                    text: qsTr("Save Current Group...")
                    onClicked: saveProfileGroupFileDialog.open()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "dialog-cancel"
                    text: qsTr("Cancel")
                    onClicked: newProfileGroupDialog.close()
                }

                QQC2.Button {
                    icon.name: "document-new"
                    text: qsTr("Create New Group")
                    highlighted: true
                    onClicked: newProfileGroupDialog.commitCreateNewGroup()
                }
            }
        }
    }

    FileDialog {
        id: exportProfileFileDialog
        title: qsTr("Export Profile #%1 (%2) Bundle (.duckyprofile.json)")
            .arg(root.exportTargetProfileNumber > 0 ? root.exportTargetProfileNumber : deviceController.activeProfile)
            .arg(root.contextMenuProfileName)
        fileMode: FileDialog.SaveFile
        defaultSuffix: "duckyprofile.json"
        nameFilters: [
            qsTr("DuckyDeck Profile Bundle (*.duckyprofile.json *.json)")
        ]
        onAccepted: {
            const targetNum = root.exportTargetProfileNumber > 0
                ? root.exportTargetProfileNumber
                : deviceController.activeProfile
            deviceController.exportProfile(targetNum, selectedFile.toString())
        }
    }

    FolderDialog {
        id: exportProfileFolderDialog
        title: qsTr("Select Destination Folder to Export Profile #%1 (%2)")
            .arg(root.exportTargetProfileNumber > 0 ? root.exportTargetProfileNumber : deviceController.activeProfile)
            .arg(root.contextMenuProfileName)
        onAccepted: {
            const targetNum = root.exportTargetProfileNumber > 0
                ? root.exportTargetProfileNumber
                : deviceController.activeProfile
            deviceController.exportProfile(targetNum, selectedFolder.toString())
        }
    }

    ColorDialog {
        id: nativeSystemColorDialog
        title: qsTr("System Color Picker")
        selectedColor: duckyColorPickerWindow.currentColor
        onAccepted: {
            duckyColorPickerWindow.setColorFromQColor(selectedColor)
        }
    }

    Window {
        id: duckyColorPickerWindow
        readonly property bool closableWithEscape: true
        title: pickerMode === "key"
            ? qsTr("Set Key %1 Color — DuckyDeck Studio").arg(targetSlotNumber)
            : qsTr("Set Default Color (BG_COLOR) — Profile #%1 (%2)").arg(targetProfileNumber).arg(targetProfileName)
        width: Math.min(720, Screen.desktopAvailableWidth > 0 ? Screen.desktopAvailableWidth - 60 : 720)
        height: Math.min(780, Screen.desktopAvailableHeight > 0 ? Screen.desktopAvailableHeight - 60 : 780)
        minimumWidth: 640
        minimumHeight: 680
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property string pickerMode: "key" // "key" or "profile_bg"
        property int targetSlotNumber: 1
        property int targetProfileNumber: 1
        property string targetProfileName: "Default"

        property string initialHex: "#00aaff"
        property real hue: 0.55
        property real saturation: 1.0
        property real val: 1.0
        property int redVal: 0
        property int greenVal: 170
        property int blueVal: 255
        property bool updatingInternally: false

        property var savedColorsList: []
        property var usedColorsList: []

        readonly property color currentColor: Qt.rgba(redVal / 255.0, greenVal / 255.0, blueVal / 255.0, 1.0)
        readonly property string currentHex: formatRgbHex(redVal, greenVal, blueVal)

        function clampByte(n) {
            const v = Math.round(Number(n) || 0)
            return Math.max(0, Math.min(255, v))
        }

        function byteToHex(n) {
            const s = clampByte(n).toString(16)
            return s.length < 2 ? ("0" + s) : s
        }

        function formatRgbHex(r, g, b) {
            return "#" + byteToHex(r) + byteToHex(g) + byteToHex(b)
        }

        function refreshPaletteLists() {
            if (!deviceController) return
            deviceController.refreshColorPalettes()
            try {
                const sArr = JSON.parse(deviceController.savedColorsJson || "[]")
                savedColorsList = Array.isArray(sArr) ? sArr : []
            } catch (e) {
                savedColorsList = []
            }
            try {
                const uArr = JSON.parse(deviceController.usedColorsJson || "[]")
                usedColorsList = Array.isArray(uArr) ? uArr : []
            } catch (e) {
                usedColorsList = []
            }
        }

        function syncRgbFromHsv() {
            if (updatingInternally) return
            updatingInternally = true
            const c = Qt.hsva(
                Math.max(0.0, Math.min(0.9999, hue)),
                Math.max(0.0, Math.min(1.0, saturation)),
                Math.max(0.0, Math.min(1.0, val)),
                1.0
            )
            redVal = clampByte(c.r * 255)
            greenVal = clampByte(c.g * 255)
            blueVal = clampByte(c.b * 255)
            hexTextField.text = formatRgbHex(redVal, greenVal, blueVal)
            updatingInternally = false
        }

        function syncHsvFromRgb() {
            if (updatingInternally) return
            updatingInternally = true
            const c = Qt.rgba(redVal / 255.0, greenVal / 255.0, blueVal / 255.0, 1.0)
            if (c.hsvHue >= 0) {
                hue = c.hsvHue
            }
            saturation = c.hsvSaturation
            val = c.hsvValue
            hexTextField.text = formatRgbHex(redVal, greenVal, blueVal)
            updatingInternally = false
        }

        function setColorFromHex(hexStr) {
            if (!hexStr) return
            let clean = String(hexStr).trim()
            if (clean.startsWith("#")) {
                clean = clean.slice(1)
            }
            if (clean.length === 3) {
                clean = clean[0] + clean[0] + clean[1] + clean[1] + clean[2] + clean[2]
            }
            if (clean.length !== 6) return
            const r = parseInt(clean.slice(0, 2), 16)
            const g = parseInt(clean.slice(2, 4), 16)
            const b = parseInt(clean.slice(4, 6), 16)
            if (isNaN(r) || isNaN(g) || isNaN(b)) return
            redVal = clampByte(r)
            greenVal = clampByte(g)
            blueVal = clampByte(b)
            syncHsvFromRgb()
        }

        function setColorFromQColor(c) {
            if (!c) return
            redVal = clampByte(c.r * 255)
            greenVal = clampByte(c.g * 255)
            blueVal = clampByte(c.b * 255)
            syncHsvFromRgb()
        }

        function openForKey(slotNum, startHex) {
            pickerMode = "key"
            targetSlotNumber = slotNum
            initialHex = (startHex && startHex.length > 0) ? startHex : "#00aaff"
            refreshPaletteLists()
            setColorFromHex(initialHex)
            initialHex = currentHex
            show()
            raise()
            requestActivate()
        }

        function openForProfileBg(profNum, profName, startHex) {
            pickerMode = "profile_bg"
            targetProfileNumber = profNum
            targetProfileName = profName || "Default"
            initialHex = (startHex && startHex.length > 0) ? startHex : "#00aaff"
            refreshPaletteLists()
            setColorFromHex(initialHex)
            initialHex = currentHex
            show()
            raise()
            requestActivate()
        }

        function applySelectedColor() {
            if (pickerMode === "key") {
                hardwareCanvas.setKeyRgbAt(targetSlotNumber, redVal, greenVal, blueVal)
                if (root.selectedSlotNumber === targetSlotNumber) {
                    root.selectedSlotColor = currentHex
                }
                if (keyPropertiesWindow.visible && keyPropertiesWindow.slotNum === targetSlotNumber) {
                    keyPropertiesWindow.slotColor = currentHex
                    keyPropColorField.text = currentHex
                }
            } else {
                deviceController.setProfileBgColor(targetProfileNumber, redVal, greenVal, blueVal)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                if (root.hasSelectedSlot) {
                    root.selectedSlotColor = hardwareCanvas.slotColorFor(root.selectedSlotNumber)
                }
                if (profilePropertiesWindow.visible && profilePropertiesWindow.profNumber === targetProfileNumber) {
                    profilePropertiesWindow.profBgColor = currentHex
                    profBgColorField.text = currentHex
                }
            }
            refreshPaletteLists()
            close()
        }

        Connections {
            target: deviceController
            function onSavedColorsJsonChanged() {
                if (duckyColorPickerWindow.visible) {
                    try {
                        const sArr = JSON.parse(deviceController.savedColorsJson || "[]")
                        duckyColorPickerWindow.savedColorsList = Array.isArray(sArr) ? sArr : []
                    } catch (e) {}
                }
            }
            function onUsedColorsJsonChanged() {
                if (duckyColorPickerWindow.visible) {
                    try {
                        const uArr = JSON.parse(deviceController.usedColorsJson || "[]")
                        duckyColorPickerWindow.usedColorsList = Array.isArray(uArr) ? uArr : []
                    } catch (e) {}
                }
            }
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: duckyColorPickerWindow.visible
            onActivated: duckyColorPickerWindow.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            // Header bar with target info & Current vs New preview
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 2

                    QQC2.Label {
                        text: duckyColorPickerWindow.pickerMode === "key"
                            ? qsTr("Key %1 Color").arg(duckyColorPickerWindow.targetSlotNumber)
                            : qsTr("Profile #%1 (%2) Default Color").arg(duckyColorPickerWindow.targetProfileNumber).arg(duckyColorPickerWindow.targetProfileName)
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        text: duckyColorPickerWindow.pickerMode === "key"
                            ? qsTr("Pick a custom per-key RGB LED color (SWCOLOR) or choose from your Saved & Used palettes.")
                            : qsTr("Pick the default key background LED color (BG_COLOR) for this profile.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        elide: Text.ElideRight
                        Layout.fillWidth: true
                    }
                }

                // Current vs Selected Swatch comparison
                Rectangle {
                    Layout.preferredWidth: 116
                    Layout.preferredHeight: 38
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.25)
                    border.width: 1
                    clip: true

                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 3
                        spacing: 3

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 4
                            color: duckyColorPickerWindow.initialHex
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                            border.width: 1

                            QQC2.Label {
                                anchors.centerIn: parent
                                text: qsTr("Old")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: Qt.color(duckyColorPickerWindow.initialHex).hslLightness > 0.55 ? "#111111" : "#ffffff"
                            }

                            MouseArea {
                                anchors.fill: parent
                                cursorShape: Qt.PointingHandCursor
                                onClicked: duckyColorPickerWindow.setColorFromHex(duckyColorPickerWindow.initialHex)
                                QQC2.ToolTip.visible: containsMouse
                                QQC2.ToolTip.text: qsTr("Revert to previous color (%1)").arg(duckyColorPickerWindow.initialHex)
                            }
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 4
                            color: duckyColorPickerWindow.currentColor
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                            border.width: 1

                            QQC2.Label {
                                anchors.centerIn: parent
                                text: qsTr("New")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: duckyColorPickerWindow.currentColor.hslLightness > 0.55 ? "#111111" : "#ffffff"
                            }
                        }
                    }
                }
            }

            // Main Interactive Picker Area: 2D SV Square + Hue Strip + RGB Sliders & Hex
            RowLayout {
                Layout.fillWidth: true
                Layout.preferredHeight: 190
                spacing: 12

                // 2D Saturation / Value Square
                Item {
                    id: svSquareContainer
                    Layout.preferredWidth: 190
                    Layout.fillHeight: true

                    Rectangle {
                        id: svBaseHueRect
                        anchors.fill: parent
                        radius: 6
                        color: Qt.hsva(Math.max(0.0, Math.min(0.9999, duckyColorPickerWindow.hue)), 1.0, 1.0, 1.0)
                         clip: true

                        // Horizontal Saturation gradient (White -> Transparent)
                        Rectangle {
                            width: parent.height
                            height: parent.width
                            anchors.centerIn: parent
                            rotation: -90
                            gradient: Gradient {
                                GradientStop { position: 0.0; color: "#ffffff" }
                                GradientStop { position: 1.0; color: "#00ffffff" }
                            }
                        }

                        // Vertical Value gradient (Transparent -> Black)
                        Rectangle {
                            anchors.fill: parent
                            radius: 6
                            gradient: Gradient {
                                GradientStop { position: 0.0; color: "#00000000" }
                                GradientStop { position: 1.0; color: "#ff000000" }
                            }
                        }

                        Rectangle {
                            anchors.fill: parent
                            radius: 6
                            color: "transparent"
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.3)
                            border.width: 1
                        }

                        // Crosshair indicator
                        Rectangle {
                            x: Math.round(duckyColorPickerWindow.saturation * parent.width) - width / 2
                            y: Math.round((1.0 - duckyColorPickerWindow.val) * parent.height) - height / 2
                            width: 14
                            height: 14
                            radius: 7
                            color: "transparent"
                            border.color: duckyColorPickerWindow.val > 0.6 && duckyColorPickerWindow.saturation < 0.4 ? "#000000" : "#ffffff"
                            border.width: 2

                            Rectangle {
                                anchors.fill: parent
                                anchors.margins: 2
                                radius: 5
                                color: duckyColorPickerWindow.currentColor
                                border.color: "#000000"
                                border.width: 1
                            }
                        }

                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.CrossCursor
                            function updateSv(mx, my) {
                                duckyColorPickerWindow.saturation = Math.max(0.0, Math.min(1.0, mx / width))
                                duckyColorPickerWindow.val = Math.max(0.0, Math.min(1.0, 1.0 - (my / height)))
                                duckyColorPickerWindow.syncRgbFromHsv()
                            }
                            onPressed: (mouse) => updateSv(mouse.x, mouse.y)
                            onPositionChanged: (mouse) => {
                                if (pressed) updateSv(mouse.x, mouse.y)
                            }
                        }
                    }
                }

                // Vertical Hue Slider Bar
                Item {
                    id: hueBarContainer
                    Layout.preferredWidth: 24
                    Layout.fillHeight: true

                    Rectangle {
                        anchors.fill: parent
                        radius: 6
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.3)
                        border.width: 1
                        gradient: Gradient {
                            GradientStop { position: 0.00; color: "#ff0000" }
                            GradientStop { position: 0.17; color: "#ffff00" }
                            GradientStop { position: 0.33; color: "#00ff00" }
                            GradientStop { position: 0.50; color: "#00ffff" }
                            GradientStop { position: 0.67; color: "#0000ff" }
                            GradientStop { position: 0.83; color: "#ff00ff" }
                            GradientStop { position: 1.00; color: "#ff0000" }
                        }

                        Rectangle {
                            x: -2
                            y: Math.round(duckyColorPickerWindow.hue * (parent.height - height))
                            width: parent.width + 4
                            height: 8
                            radius: 3
                            color: "transparent"
                            border.color: "#ffffff"
                            border.width: 2

                            Rectangle {
                                anchors.fill: parent
                                anchors.margins: 1
                                radius: 2
                                color: "transparent"
                                border.color: "#000000"
                                border.width: 1
                            }
                        }

                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            function updateHue(my) {
                                duckyColorPickerWindow.hue = Math.max(0.0, Math.min(0.9999, my / height))
                                duckyColorPickerWindow.syncRgbFromHsv()
                            }
                            onPressed: (mouse) => updateHue(mouse.y)
                            onPositionChanged: (mouse) => {
                                if (pressed) updateHue(mouse.y)
                            }
                        }
                    }
                }

                // RGB Sliders + SpinBoxes + Hex Input + System Color Picker button
                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    spacing: 6

                    // Red Row
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        QQC2.Label {
                            text: "R"
                            font.bold: true
                            color: root.isLightTheme ? "#dc2626" : "#f87171"
                            Layout.preferredWidth: 14
                        }
                        QQC2.Slider {
                            Layout.fillWidth: true
                            from: 0
                            to: 255
                            stepSize: 1
                            value: duckyColorPickerWindow.redVal
                            onMoved: {
                                duckyColorPickerWindow.redVal = Math.round(value)
                                duckyColorPickerWindow.syncHsvFromRgb()
                            }
                        }
                        QQC2.SpinBox {
                            from: 0
                            to: 255
                            value: duckyColorPickerWindow.redVal
                            editable: true
                            Layout.preferredWidth: 68
                            onValueModified: {
                                duckyColorPickerWindow.redVal = value
                                duckyColorPickerWindow.syncHsvFromRgb()
                            }
                        }
                    }

                    // Green Row
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        QQC2.Label {
                            text: "G"
                            font.bold: true
                            color: root.highContrastPositiveText
                            Layout.preferredWidth: 14
                        }
                        QQC2.Slider {
                            Layout.fillWidth: true
                            from: 0
                            to: 255
                            stepSize: 1
                            value: duckyColorPickerWindow.greenVal
                            onMoved: {
                                duckyColorPickerWindow.greenVal = Math.round(value)
                                duckyColorPickerWindow.syncHsvFromRgb()
                            }
                        }
                        QQC2.SpinBox {
                            from: 0
                            to: 255
                            value: duckyColorPickerWindow.greenVal
                            editable: true
                            Layout.preferredWidth: 68
                            onValueModified: {
                                duckyColorPickerWindow.greenVal = value
                                duckyColorPickerWindow.syncHsvFromRgb()
                            }
                        }
                    }

                    // Blue Row
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6
                        QQC2.Label {
                            text: "B"
                            font.bold: true
                            color: root.isLightTheme ? "#2563eb" : "#60a5fa"
                            Layout.preferredWidth: 14
                        }
                        QQC2.Slider {
                            Layout.fillWidth: true
                            from: 0
                            to: 255
                            stepSize: 1
                            value: duckyColorPickerWindow.blueVal
                            onMoved: {
                                duckyColorPickerWindow.blueVal = Math.round(value)
                                duckyColorPickerWindow.syncHsvFromRgb()
                            }
                        }
                        QQC2.SpinBox {
                            from: 0
                            to: 255
                            value: duckyColorPickerWindow.blueVal
                            editable: true
                            Layout.preferredWidth: 68
                            onValueModified: {
                                duckyColorPickerWindow.blueVal = value
                                duckyColorPickerWindow.syncHsvFromRgb()
                            }
                        }
                    }

                    Item { Layout.fillHeight: true }

                    // Hex field + System Picker
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6

                        QQC2.Label {
                            text: qsTr("Hex:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }

                        QQC2.TextField {
                            id: hexTextField
                            Layout.preferredWidth: 96
                            text: duckyColorPickerWindow.currentHex
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            maximumLength: 7
                            selectByMouse: true
                            onEditingFinished: {
                                duckyColorPickerWindow.setColorFromHex(text)
                            }
                            onAccepted: {
                                duckyColorPickerWindow.setColorFromHex(text)
                            }
                        }

                        Item { Layout.fillWidth: true }

                        QQC2.Button {
                            icon.name: "color-picker"
                            text: qsTr("System Picker...")
                            font.pointSize: 10 * root.fontScale
                            onClicked: {
                                nativeSystemColorDialog.selectedColor = duckyColorPickerWindow.currentColor
                                nativeSystemColorDialog.open()
                            }
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Open native desktop color dialog / screen eyedropper")
                        }
                    }
                }
            }

            // Divider
            Rectangle {
                Layout.fillWidth: true
                height: 1
                color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
            }

            // SAVED COLORS Palette Section
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    QQC2.Label {
                        text: qsTr("SAVED COLORS (%1)").arg(duckyColorPickerWindow.savedColorsList.length)
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    QQC2.Label {
                        text: qsTr("• Left-click to select, right-click to remove")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }

                    QQC2.Button {
                        icon.name: "list-add"
                        text: qsTr("Save Current (%1)").arg(duckyColorPickerWindow.currentHex)
                        implicitHeight: 24
                        font.pointSize: 10 * root.fontScale
                        onClicked: {
                            deviceController.savePaletteColor(duckyColorPickerWindow.currentHex)
                            duckyColorPickerWindow.refreshPaletteLists()
                        }
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Save %1 to your persistent Saved Colors palette").arg(duckyColorPickerWindow.currentHex)
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: Math.min(130, Math.max(84, savedColorsFlow.implicitHeight + 16))
                    Layout.maximumHeight: 130
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                    border.width: 1
                    clip: true

                    QQC2.ScrollView {
                        anchors.fill: parent
                        anchors.margins: 6
                        clip: true
                        QQC2.ScrollBar.vertical: HighContrastScrollBar {}
                        QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                            orientation: Qt.Horizontal
                        }

                        Flow {
                            id: savedColorsFlow
                            width: parent.width
                            spacing: 6

                            Repeater {
                                model: duckyColorPickerWindow.savedColorsList

                                delegate: Rectangle {
                                    id: savedSwatch
                                    required property string modelData
                                    readonly property bool isCurrentMatch: modelData.toLowerCase() === duckyColorPickerWindow.currentHex.toLowerCase()
                                    width: 30
                                    height: 30
                                    radius: 5
                                    color: modelData
                                    border.width: isCurrentMatch ? 2 : (savedSwatchMouse.containsMouse ? 2 : 1)
                                    border.color: isCurrentMatch
                                        ? Kirigami.Theme.highlightColor
                                        : (savedSwatchMouse.containsMouse ? "#ffffff" : Qt.alpha("#000000", 0.45))

                                    // Inner ring when selected
                                    Rectangle {
                                        visible: savedSwatch.isCurrentMatch
                                        anchors.fill: parent
                                        anchors.margins: 2
                                        radius: 3
                                        color: "transparent"
                                        border.color: "#ffffff"
                                        border.width: 1
                                    }

                                    MouseArea {
                                        id: savedSwatchMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: (mouse) => {
                                            if (mouse.button === Qt.RightButton) {
                                                deviceController.removePaletteColor(savedSwatch.modelData)
                                                duckyColorPickerWindow.refreshPaletteLists()
                                            } else {
                                                duckyColorPickerWindow.setColorFromHex(savedSwatch.modelData)
                                            }
                                        }
                                        onDoubleClicked: (mouse) => {
                                            if (mouse.button === Qt.LeftButton) {
                                                duckyColorPickerWindow.setColorFromHex(savedSwatch.modelData)
                                                duckyColorPickerWindow.applySelectedColor()
                                            }
                                        }
                                    }

                                    QQC2.ToolTip.visible: savedSwatchMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("%1 • Left-click to select • Double-click to apply • Right-click to remove").arg(savedSwatch.modelData)
                                }
                            }
                        }
                    }
                }
            }

            // USED COLORS Palette Section
            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 6

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    QQC2.Label {
                        text: qsTr("USED COLORS IN PROFILES & KEYS (%1)").arg(duckyColorPickerWindow.usedColorsList.length)
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    QQC2.Label {
                        text: qsTr("• Left-click to select, right-click to save to palette")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.minimumHeight: 74
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                    border.width: 1
                    clip: true

                    QQC2.Label {
                        anchors.centerIn: parent
                        visible: duckyColorPickerWindow.usedColorsList.length === 0
                        text: qsTr("No profile or key colors loaded yet.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    QQC2.ScrollView {
                        anchors.fill: parent
                        anchors.margins: 6
                        clip: true
                        QQC2.ScrollBar.vertical: HighContrastScrollBar {}
                        QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                            orientation: Qt.Horizontal
                        }

                        Flow {
                            width: parent.width
                            spacing: 6

                            Repeater {
                                model: duckyColorPickerWindow.usedColorsList

                                delegate: Rectangle {
                                    id: usedSwatch
                                    required property string modelData
                                    readonly property bool isCurrentMatch: modelData.toLowerCase() === duckyColorPickerWindow.currentHex.toLowerCase()
                                    width: 30
                                    height: 30
                                    radius: 5
                                    color: modelData
                                    border.width: isCurrentMatch ? 2 : (usedSwatchMouse.containsMouse ? 2 : 1)
                                    border.color: isCurrentMatch
                                        ? Kirigami.Theme.highlightColor
                                        : (usedSwatchMouse.containsMouse ? "#ffffff" : Qt.alpha("#000000", 0.45))

                                    Rectangle {
                                        visible: usedSwatch.isCurrentMatch
                                        anchors.fill: parent
                                        anchors.margins: 2
                                        radius: 3
                                        color: "transparent"
                                        border.color: "#ffffff"
                                        border.width: 1
                                    }

                                    MouseArea {
                                        id: usedSwatchMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: (mouse) => {
                                            if (mouse.button === Qt.RightButton) {
                                                deviceController.savePaletteColor(usedSwatch.modelData)
                                                duckyColorPickerWindow.refreshPaletteLists()
                                            } else {
                                                duckyColorPickerWindow.setColorFromHex(usedSwatch.modelData)
                                            }
                                        }
                                        onDoubleClicked: (mouse) => {
                                            if (mouse.button === Qt.LeftButton) {
                                                duckyColorPickerWindow.setColorFromHex(usedSwatch.modelData)
                                                duckyColorPickerWindow.applySelectedColor()
                                            }
                                        }
                                    }

                                    QQC2.ToolTip.visible: usedSwatchMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("%1 • Left-click to select • Double-click to apply • Right-click to save to Saved Colors").arg(usedSwatch.modelData)
                                }
                            }
                        }
                    }
                }
            }

            // Footer Action Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    visible: duckyColorPickerWindow.pickerMode === "key"
                    icon.name: "edit-clear"
                    text: qsTr("Reset to Profile Default")
                    onClicked: {
                        hardwareCanvas.clearKeyRgbAt(duckyColorPickerWindow.targetSlotNumber)
                        if (root.selectedSlotNumber === duckyColorPickerWindow.targetSlotNumber) {
                            root.selectedSlotColor = hardwareCanvas.slotColorFor(duckyColorPickerWindow.targetSlotNumber)
                        }
                        duckyColorPickerWindow.close()
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Remove custom SWCOLOR from Key %1 and use profile BG_COLOR").arg(duckyColorPickerWindow.targetSlotNumber)
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "dialog-cancel"
                    text: qsTr("Cancel")
                    onClicked: duckyColorPickerWindow.close()
                }

                QQC2.Button {
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Apply Color")
                    highlighted: true
                    onClicked: duckyColorPickerWindow.applySelectedColor()
                }
            }
        }
    }

    // Profile Right-Click Context Menu
    QQC2.Menu {
        id: profileContextMenu
        title: qsTr("Profile Actions")

        QQC2.MenuItem {
            text: qsTr("Profile #%1 — %2")
                .arg(root.contextMenuProfileNumber)
                .arg(root.contextMenuProfileName)
            enabled: false
            font.bold: true
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-properties"
            text: qsTr("Properties...")
            onTriggered: {
                profilePropertiesWindow.openForProfile(root.contextMenuProfileNumber)
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "go-up"
            text: qsTr("Move Profile Up")
            enabled: profileSidebarList.canReorder && root.contextMenuProfileIndex > 0
            onTriggered: {
                const savedY = profileSidebarList.contentY
                deviceController.reorderProfile(root.contextMenuProfileIndex, root.contextMenuProfileIndex - 1)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
            }
        }

        QQC2.MenuItem {
            icon.name: "go-down"
            text: qsTr("Move Profile Down")
            enabled: profileSidebarList.canReorder && root.contextMenuProfileIndex < profileSidebarList.count - 1
            onTriggered: {
                const savedY = profileSidebarList.contentY
                deviceController.reorderProfile(root.contextMenuProfileIndex, root.contextMenuProfileIndex + 1)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-export"
            text: qsTr("Export Profile (.duckyprofile.json)...")
            onTriggered: {
                root.exportTargetProfileNumber = root.contextMenuProfileNumber
                exportProfileFileDialog.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "folder"
            text: qsTr("Export Profile to Folder...")
            onTriggered: {
                root.exportTargetProfileNumber = root.contextMenuProfileNumber
                exportProfileFolderDialog.open()
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "bookmark-new"
            text: qsTr("Save Profile as Template...")
            onTriggered: {
                profileTemplatesDialog.openSaveFromProfile(
                    root.contextMenuProfileNumber,
                    root.contextMenuProfileName
                )
            }
        }

        QQC2.MenuItem {
            icon.name: "document-new-from-template"
            text: qsTr("Apply Template to Profile...")
            onTriggered: {
                profileTemplatesDialog.openApplyToProfile(
                    root.contextMenuProfileNumber,
                    root.contextMenuProfileName
                )
            }
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "edit-delete"
            text: qsTr("Delete Profile...")
            enabled: profileSidebarList.count > 1
            onTriggered: {
                deleteProfileDialog.open()
            }
        }
    }

    Window {
        id: profileTemplatesDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Profile Templates Manager — DuckyDeck Studio")
        width: 840
        height: 660
        minimumWidth: 640
        minimumHeight: 520
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property var templatesList: []
        property int selectedIndex: 0
        property int targetProfileNumber: 0
        property string targetProfileName: ""
        property string statusMessage: ""

        function refreshTemplates() {
            let parsed = []
            try {
                parsed = JSON.parse(deviceController.listProfileTemplatesJson())
            } catch (e) {
                parsed = []
            }
            templatesList = parsed
            if (selectedIndex >= templatesList.length) {
                selectedIndex = Math.max(0, templatesList.length - 1)
            }
        }

        function selectedTemplate() {
            if (selectedIndex >= 0 && selectedIndex < templatesList.length) {
                return templatesList[selectedIndex]
            }
            return null
        }

        function deleteTemplateById(tplId, tplName) {
            if (!tplId) return
            if (deviceController.deleteProfileTemplate(tplId)) {
                profileTemplatesDialog.refreshTemplates()
                root.refreshSidebarTemplateCombo()
                profileTemplatesDialog.statusMessage = qsTr("✓ Deleted custom template '%1'.").arg(tplName || tplId)
            } else {
                profileTemplatesDialog.statusMessage = deviceController.statusMessage
            }
        }

        function openGeneral() {
            refreshTemplates()
            targetProfileNumber = deviceController.activeProfile
            targetProfileName = deviceController.activeProfileName
            saveSourceProfileCombo.syncChoices()
            tplSaveNameField.text = deviceController.activeProfileName + " Template"
            tplSaveDescField.text = qsTr("Saved from Profile #%1 (%2)")
                .arg(deviceController.activeProfile)
                .arg(deviceController.activeProfileName)
            tplNewProfileNameField.text = ""
            statusMessage = ""
            profileTemplatesDialog.show()
            profileTemplatesDialog.raise()
            profileTemplatesDialog.requestActivate()
        }

        function openSaveFromProfile(profNum, profName) {
            refreshTemplates()
            targetProfileNumber = profNum
            targetProfileName = profName
            saveSourceProfileCombo.syncChoices(profNum)
            tplSaveNameField.text = profName + " Template"
            tplSaveDescField.text = qsTr("Copied from Profile #%1 (%2)").arg(profNum).arg(profName)
            statusMessage = ""
            profileTemplatesDialog.show()
            profileTemplatesDialog.raise()
            profileTemplatesDialog.requestActivate()
            Qt.callLater(function() {
                tplSaveNameField.forceActiveFocus()
                tplSaveNameField.selectAll()
            })
        }

        function openApplyToProfile(profNum, profName) {
            refreshTemplates()
            targetProfileNumber = profNum
            targetProfileName = profName
            saveSourceProfileCombo.syncChoices(profNum)
            statusMessage = qsTr("Select a template below and click 'Apply to Profile #%1 (%2)'.").arg(profNum).arg(profName)
            profileTemplatesDialog.show()
            profileTemplatesDialog.raise()
            profileTemplatesDialog.requestActivate()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: profileTemplatesDialog.visible
            onActivated: profileTemplatesDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 2
                QQC2.Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: qsTr("Profile Templates — Create Profiles from Templates or Save Existing Profiles as Templates")
                    font.bold: true
                    font.pointSize: 11 * root.fontScale
                    color: Kirigami.Theme.textColor
                }
                QQC2.Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: qsTr("Choose a built-in or custom template to create a new profile or overwrite an existing profile, or copy any existing profile into a reusable template.")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            // Copy Existing Profile to Template Card
            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                border.width: 1
                implicitHeight: saveTplCol.implicitHeight + 16

                ColumnLayout {
                    id: saveTplCol
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.top: parent.top
                    anchors.margins: 8
                    spacing: 6

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        text: qsTr("Copy Existing Profile to Reusable Template:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Source Profile:")
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.ComboBox {
                            id: saveSourceProfileCombo
                            Layout.preferredWidth: 185
                            Layout.minimumWidth: 130
                            property var profileNums: []
                            property var profileNames: []
                            function syncChoices(preferredNum) {
                                let nums = []
                                let names = []
                                let labels = []
                                try {
                                    const arr = JSON.parse(deviceController.profilesJson)
                                    for (let i = 0; i < arr.length; i++) {
                                        nums.push(arr[i].number)
                                        names.push(arr[i].name)
                                        labels.push("#" + arr[i].number + " — " + arr[i].name)
                                    }
                                } catch (e) {}
                                profileNums = nums
                                profileNames = names
                                model = labels
                                const target = preferredNum || deviceController.activeProfile
                                const idx = nums.indexOf(target)
                                currentIndex = idx >= 0 ? idx : 0
                            }
                            onActivated: {
                                if (currentIndex >= 0 && currentIndex < profileNames.length) {
                                    tplSaveNameField.text = profileNames[currentIndex] + " Template"
                                    tplSaveDescField.text = qsTr("Copied from Profile #%1 (%2)")
                                        .arg(profileNums[currentIndex])
                                        .arg(profileNames[currentIndex])
                                }
                            }
                        }

                        QQC2.Label {
                            text: qsTr("Template Name:")
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.TextField {
                            id: tplSaveNameField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 140
                            maximumLength: 36
                            placeholderText: qsTr("Template Name...")
                            selectByMouse: true
                        }

                        QQC2.Button {
                            icon.name: "bookmark-new"
                            text: qsTr("Save as Template")
                            highlighted: true
                            enabled: tplSaveNameField.text.trim().length > 0 && saveSourceProfileCombo.profileNums.length > 0
                            onClicked: {
                                const srcIdx = Math.max(0, saveSourceProfileCombo.currentIndex)
                                const srcProf = saveSourceProfileCombo.profileNums[srcIdx] || deviceController.activeProfile
                                if (deviceController.saveProfileAsTemplate(
                                    srcProf,
                                    tplSaveNameField.text.trim(),
                                    tplSaveDescField.text.trim()
                                )) {
                                    profileTemplatesDialog.refreshTemplates()
                                    root.refreshSidebarTemplateCombo()
                                    profileTemplatesDialog.selectedIndex = Math.max(0, profileTemplatesDialog.templatesList.length - 1)
                                    profileTemplatesDialog.statusMessage = qsTr("✓ Saved Profile #%1 as template '%2'.")
                                        .arg(srcProf)
                                        .arg(tplSaveNameField.text.trim())
                                }
                            }
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Description:")
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.TextField {
                            id: tplSaveDescField
                            Layout.fillWidth: true
                            maximumLength: 140
                            placeholderText: qsTr("Description (optional)...")
                            selectByMouse: true
                        }
                    }
                }
            }

            // Templates List
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 200
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                border.width: 1
                clip: true

                ListView {
                    id: tplListView
                    anchors.fill: parent
                    anchors.margins: 6
                    spacing: 6
                    model: profileTemplatesDialog.templatesList
                    boundsBehavior: Flickable.StopAtBounds
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: tplListView
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: tplListView.contentHeight > tplListView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    delegate: Rectangle {
                        id: tplDelegate
                        width: Math.max(200, tplListView.width - (tplListView.contentHeight > tplListView.height ? 16 : 4))
                        implicitHeight: tplDelegateCol.implicitHeight + 16
                        radius: 5
                        readonly property bool isSel: profileTemplatesDialog.selectedIndex === index
                        readonly property bool isBuiltIn: Boolean(modelData.builtIn !== undefined ? modelData.builtIn : modelData.builtin)
                        readonly property int keyCount: Number(modelData.configuredSlotCount !== undefined ? modelData.configuredSlotCount : (modelData.slotCount || 0))
                        color: isSel
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.22)
                            : (tplRowMouse.containsMouse ? Qt.alpha(Kirigami.Theme.textColor, 0.06) : Kirigami.Theme.backgroundColor)
                        border.color: isSel
                            ? Kirigami.Theme.highlightColor
                            : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                        border.width: isSel ? 2 : 1

                        MouseArea {
                            id: tplRowMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            z: 0
                            onClicked: {
                                profileTemplatesDialog.selectedIndex = index
                            }
                        }

                        ColumnLayout {
                            id: tplDelegateCol
                            anchors.left: parent.left
                            anchors.right: parent.right
                            anchors.top: parent.top
                            anchors.margins: 8
                            spacing: 4
                            z: 2

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                Rectangle {
                                    width: 14
                                    height: 14
                                    radius: 3
                                    color: modelData.bgColor || "#38bdf8"
                                    border.color: "#ffffff66"
                                    border.width: 1
                                }

                                QQC2.Label {
                                    text: modelData.name || ""
                                    font.bold: true
                                    font.pointSize: 11 * root.fontScale
                                    color: Kirigami.Theme.textColor
                                    elide: Text.ElideRight
                                    Layout.maximumWidth: 260
                                }

                                Rectangle {
                                    radius: 4
                                    color: tplDelegate.isBuiltIn
                                        ? Qt.alpha("#0ea5e9", 0.20)
                                        : Qt.alpha("#a855f7", 0.22)
                                    border.color: tplDelegate.isBuiltIn
                                        ? Qt.alpha("#38bdf8", 0.65)
                                        : Qt.alpha("#c084fc", 0.70)
                                    border.width: 1
                                    implicitWidth: tplKindLbl.implicitWidth + 10
                                    implicitHeight: tplKindLbl.implicitHeight + 4

                                    QQC2.Label {
                                        id: tplKindLbl
                                        anchors.centerIn: parent
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        text: tplDelegate.isBuiltIn ? qsTr("BUILT-IN") : qsTr("CUSTOM TEMPLATE")
                                        color: Kirigami.Theme.textColor
                                    }
                                }

                                QQC2.Label {
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: tplDelegate.isSel
                                        ? Qt.alpha(Kirigami.Theme.textColor, 0.82)
                                        : Kirigami.Theme.disabledTextColor
                                    text: qsTr("%1 keys configured  •  OLED %2°")
                                        .arg(tplDelegate.keyCount)
                                        .arg(modelData.oledRotation || 0)
                                    elide: Text.ElideRight
                                    Layout.fillWidth: true
                                }

                                QQC2.Button {
                                    visible: !tplDelegate.isBuiltIn
                                    icon.name: "edit-delete"
                                    text: qsTr("Delete")
                                    implicitHeight: 26
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: {
                                        profileTemplatesDialog.deleteTemplateById(modelData.id, modelData.name)
                                    }
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                                font.pointSize: 10 * root.fontScale
                                text: modelData.description || ""
                                color: Qt.alpha(Kirigami.Theme.textColor, 0.85)
                            }
                        }
                    }
                }
            }

            // Status message
            QQC2.Label {
                visible: profileTemplatesDialog.statusMessage.length > 0
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
                text: profileTemplatesDialog.statusMessage
                font.bold: true
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.positiveTextColor
            }

            // Footer Actions: two clean rows to prevent horizontal overflow
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    QQC2.Label {
                        text: qsTr("New Profile Name:")
                        font.pointSize: 10 * root.fontScale
                    }

                    QQC2.TextField {
                        id: tplNewProfileNameField
                        Layout.preferredWidth: 180
                        Layout.minimumWidth: 120
                        maximumLength: 16
                        placeholderText: profileTemplatesDialog.selectedTemplate()
                            ? (profileTemplatesDialog.selectedTemplate().defaultProfileName || profileTemplatesDialog.selectedTemplate().name || qsTr("New profile name..."))
                            : qsTr("New profile name...")
                        selectByMouse: true
                    }

                    QQC2.Button {
                        icon.name: "list-add"
                        text: qsTr("Create New Profile from Selected")
                        highlighted: true
                        enabled: profileTemplatesDialog.selectedTemplate() !== null
                        onClicked: {
                            const tpl = profileTemplatesDialog.selectedTemplate()
                            if (!tpl) return
                            const pName = tplNewProfileNameField.text.trim().length > 0
                                ? tplNewProfileNameField.text.trim()
                                : (tpl.defaultProfileName || tpl.name)
                            if (deviceController.createProfileFromTemplate(pName, tpl.id)) {
                                root.syncSidebarProfilesFromController()
                                hardwareCanvas.syncProfilesFromController()
                                root.refreshSelectedSlotFromController()
                                profileTemplatesDialog.statusMessage = qsTr("✓ Created new profile '%1' from template '%2'.").arg(pName).arg(tpl.name)
                            }
                        }
                    }

                    Item { Layout.fillWidth: true }
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    QQC2.Button {
                        visible: profileTemplatesDialog.targetProfileNumber > 0
                        icon.name: "document-new-from-template"
                        text: qsTr("Apply to Profile #%1 (%2)")
                            .arg(profileTemplatesDialog.targetProfileNumber)
                            .arg(profileTemplatesDialog.targetProfileName || deviceController.activeProfileName)
                        enabled: profileTemplatesDialog.selectedTemplate() !== null
                        onClicked: {
                            const tpl = profileTemplatesDialog.selectedTemplate()
                            if (!tpl) return
                            if (deviceController.applyTemplateToProfile(profileTemplatesDialog.targetProfileNumber, tpl.id)) {
                                root.syncSidebarProfilesFromController()
                                hardwareCanvas.syncProfilesFromController()
                                root.refreshSelectedSlotFromController()
                                profileTemplatesDialog.statusMessage = qsTr("✓ Applied template '%1' to Profile #%2.")
                                    .arg(tpl.name)
                                    .arg(profileTemplatesDialog.targetProfileNumber)
                            }
                        }
                    }

                    QQC2.Button {
                        readonly property var curTpl: profileTemplatesDialog.selectedTemplate()
                        readonly property bool canDelete: curTpl !== null && !(curTpl.builtIn !== undefined ? curTpl.builtIn : curTpl.builtin)
                        visible: canDelete
                        enabled: canDelete
                        icon.name: "edit-delete"
                        text: qsTr("Delete Selected Template")
                        onClicked: {
                            const tpl = profileTemplatesDialog.selectedTemplate()
                            if (!tpl) return
                            profileTemplatesDialog.deleteTemplateById(tpl.id, tpl.name)
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        icon.name: "window-close"
                        text: qsTr("Close")
                        onClicked: profileTemplatesDialog.close()
                    }
                }
            }
        }
    }

    Window {
        id: renameProfileDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Rename Profile #%1 — DuckyDeck Studio").arg(renameProfileDialog.targetProfileNumber)
        width: 420
        height: 175
        minimumWidth: 380
        minimumHeight: 165
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        property int targetProfileNumber: 1
        property string initialProfileName: ""

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openWithProfile(profileNumber, currentName) {
            renameProfileDialog.targetProfileNumber = profileNumber
            renameProfileDialog.initialProfileName = currentName
            renameProfileInput.text = currentName
            renameProfileDialog.show()
            renameProfileDialog.raise()
            renameProfileDialog.requestActivate()
            Qt.callLater(function() {
                renameProfileInput.forceActiveFocus()
                renameProfileInput.selectAll()
            })
        }

        function commitRename() {
            const trimmed = renameProfileInput.text.trim()
            if (trimmed.length === 0) return
            const savedY = profileSidebarList.contentY
            root.selectingProfileFromClick = true
            if (deviceController.renameProfile(renameProfileDialog.targetProfileNumber, trimmed)) {
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
                renameProfileDialog.close()
            }
            root.selectingProfileFromClick = false
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: renameProfileDialog.visible
            onActivated: renameProfileDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12
            Keys.onEscapePressed: renameProfileDialog.close()

            QQC2.Label {
                Layout.fillWidth: true
                font.bold: true
                font.pointSize: 10 * root.fontScale
                text: qsTr("New name for Profile #%1 (max 16 characters):").arg(renameProfileDialog.targetProfileNumber)
                color: Kirigami.Theme.textColor
            }

            QQC2.TextField {
                id: renameProfileInput
                Layout.fillWidth: true
                maximumLength: 16
                placeholderText: qsTr("Profile name...")
                selectByMouse: true
                font.pointSize: 11 * root.fontScale
                onAccepted: renameProfileDialog.commitRename()
                Keys.onEscapePressed: renameProfileDialog.close()
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "dialog-cancel"
                    text: qsTr("Cancel")
                    onClicked: renameProfileDialog.close()
                }

                QQC2.Button {
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Rename")
                    highlighted: true
                    enabled: renameProfileInput.text.trim().length > 0
                    onClicked: renameProfileDialog.commitRename()
                }
            }
        }
    }

    QQC2.Dialog {
        id: deleteProfileDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: deleteProfileDialog.visible
            onActivated: deleteProfileDialog.close()
            onAmbiguouslyActivated: deleteProfileDialog.close()
        }
        anchors.centerIn: parent
        modal: true
        width: 400
        title: qsTr("Delete Profile #%1 — %2?")
            .arg(root.contextMenuProfileNumber)
            .arg(root.contextMenuProfileName)
        standardButtons: QQC2.Dialog.Yes | QQC2.Dialog.Cancel

        ColumnLayout {
            width: parent.width
            spacing: 8

            QQC2.Label {
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
                text: qsTr("Are you sure you want to delete Profile #%1 (%2)?\n\nAny Auto-Switcher rules targeting this profile will also be removed, and remaining profiles will be renumbered.")
                    .arg(root.contextMenuProfileNumber)
                    .arg(root.contextMenuProfileName)
            }
        }

        onAccepted: {
            const savedY = profileSidebarList.contentY
            root.selectingProfileFromClick = true
            if (deviceController.deleteProfile(root.contextMenuProfileNumber)) {
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = Math.min(
                    savedY,
                    Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                )
            }
            root.selectingProfileFromClick = false
        }
    }

    PgvInspectorDrawer {
        id: pgvDrawer
        controller: deviceController
    }

    AutoSwitchDrawer {
        id: autoSwitchDrawer
        transientParent: root
        controller: deviceController
    }

    AiArchitectModal {
        id: aiModal
        controller: deviceController
    }

    component HighContrastScrollBar: QQC2.ScrollBar {
        id: hcBar
        objectName: "highContrastScrollBar"
        readonly property bool isProfileBoxStyleScrollBar: true
        readonly property int styleTrackThickness: 11
        readonly property int styleThumbThickness: 7
        readonly property int styleTrackRadius: 5
        readonly property int styleThumbRadius: 4
        policy: size < 1.0 ? QQC2.ScrollBar.AlwaysOn : QQC2.ScrollBar.AsNeeded
        minimumSize: 0.14
        implicitWidth: orientation === Qt.Vertical ? 11 : (parent ? parent.width : 40)
        implicitHeight: orientation === Qt.Horizontal ? 11 : (parent ? parent.height : 40)
        x: orientation === Qt.Vertical ? (parent ? parent.width - width : 0) : 0
        y: orientation === Qt.Horizontal ? (parent ? parent.height - height : 0) : 0
        width: orientation === Qt.Vertical ? 11 : (parent ? parent.width : implicitWidth)
        height: orientation === Qt.Horizontal ? 11 : (parent ? parent.height : implicitHeight)
        z: 50
        padding: 1
        background: Rectangle {
            visible: hcBar.size < 1.0 || hcBar.policy === QQC2.ScrollBar.AlwaysOn
            implicitWidth: hcBar.orientation === Qt.Vertical ? 11 : 40
            implicitHeight: hcBar.orientation === Qt.Horizontal ? 11 : 40
            radius: 5
            color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.28)
            border.width: 1
        }
        contentItem: Rectangle {
            visible: hcBar.size < 1.0 || hcBar.policy === QQC2.ScrollBar.AlwaysOn
            implicitWidth: hcBar.orientation === Qt.Vertical ? 7 : 28
            implicitHeight: hcBar.orientation === Qt.Horizontal ? 7 : 28
            radius: 4
            color: hcBar.pressed
                ? Kirigami.Theme.highlightColor
                : (hcBar.hovered
                    ? Qt.tint(Kirigami.Theme.highlightColor, Qt.rgba(1, 1, 1, 0.22))
                    : Qt.alpha(Kirigami.Theme.textColor, 0.62))
            border.color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.55)
            border.width: 1
        }
    }

    // Settings Window (Appearance, System Integration, HTTP REST API, and Global Webhook)
    Window {
        id: settingsDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Settings — DuckyDeck Studio")
        readonly property int availScreenHeight: (root.screen && root.screen.desktopAvailableHeight > 0)
            ? root.screen.desktopAvailableHeight
            : (Screen.desktopAvailableHeight > 0 ? Screen.desktopAvailableHeight : 960)
        readonly property int targetFullHeight: Math.max(720, settingsContentCol.implicitHeight + 132)
        width: 780
        height: Math.min(availScreenHeight - 48, targetFullHeight)
        minimumWidth: 560
        minimumHeight: 440
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property bool showRestToken: false
        property string restApiFeedback: ""
        property string globalWebhookFeedback: ""

        function syncRestApiFieldsFromController() {
            restApiEnableCheck.checked = deviceController.restApiEnabled
            restApiIpField.text = deviceController.restApiBindIp || "127.0.0.1"
            restApiPortField.text = String(deviceController.restApiPort || 28491)
            restApiRequireAuthCheck.checked = deviceController.restApiRequireAuth
            restApiTokenField.text = deviceController.restApiToken || ""
        }

        function syncGlobalWebhookFieldsFromController() {
            try {
                const wh = JSON.parse(deviceController.getGlobalProfileWebhookJson())
                globalWebhookEnableCheck.checked = Boolean(wh.enabled)
                const mIdx = globalWebhookMethodCombo.indexOfValue((wh.method || "POST").toUpperCase())
                globalWebhookMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 1
                globalWebhookUrlField.text = wh.url || ""
                globalWebhookHeadersArea.text = wh.headers !== undefined ? String(wh.headers) : "Content-Type: application/json"
                globalWebhookBodyArea.text = wh.body !== undefined
                    ? String(wh.body)
                    : "{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\",\"previous_profile_number\":{{previous_profile_number}},\"previous_profile_name\":\"{{previous_profile_name}}\",\"source\":\"{{source}}\",\"device_serial\":\"{{device_serial}}\",\"timestamp\":{{timestamp}}}"
            } catch (e) {
                globalWebhookEnableCheck.checked = false
            }
        }

        function saveGlobalWebhookConfig() {
            const payload = {
                enabled: globalWebhookEnableCheck.checked,
                method: globalWebhookMethodCombo.currentText || "POST",
                url: globalWebhookUrlField.text.trim(),
                headers: globalWebhookHeadersArea.text,
                body: globalWebhookBodyArea.text
            }
            if (deviceController.saveGlobalProfileWebhookJson(JSON.stringify(payload))) {
                settingsDialog.globalWebhookFeedback = qsTr("✓ Saved global profile-change webhook configuration.")
            }
        }

        function open() {
            settingsDialog.syncRestApiFieldsFromController()
            settingsDialog.syncGlobalWebhookFieldsFromController()
            settingsDialog.restApiFeedback = ""
            settingsDialog.globalWebhookFeedback = ""
            settingsDialog.height = Math.min(settingsDialog.availScreenHeight - 48, settingsDialog.targetFullHeight)
            settingsDialog.show()
            settingsDialog.raise()
            settingsDialog.requestActivate()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: settingsDialog.visible
            onActivated: settingsDialog.close()
            onAmbiguouslyActivated: settingsDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Kirigami.Heading {
                    level: 2
                    text: qsTr("Application Settings")
                    color: Kirigami.Theme.textColor
                    Layout.fillWidth: true
                }

                QQC2.Button {
                    text: "−"
                    implicitWidth: 32
                    font.bold: true
                    enabled: deviceController.uiTextScale > 75
                    onClicked: root.zoomOut()
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Zoom Out (Ctrl+-)")
                }

                QQC2.Label {
                    text: qsTr("Zoom: %1%").arg(deviceController.uiTextScale)
                    font.family: "monospace"
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                QQC2.Button {
                    text: "+"
                    implicitWidth: 32
                    font.bold: true
                    enabled: deviceController.uiTextScale < 160
                    onClicked: root.zoomIn()
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Zoom In (Ctrl++ / Ctrl+=)")
                }

                QQC2.Button {
                    icon.name: "edit-reset"
                    text: qsTr("Reset Zoom (100%)")
                    enabled: deviceController.uiTextScale !== 100
                    onClicked: root.resetZoom()
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Reset Zoom to 100% (Ctrl+0)")
                }
            }

            Flickable {
                id: settingsFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                contentWidth: width
                contentHeight: settingsContentCol.implicitHeight + 12
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    id: settingsVerticalScrollBar
                    parent: settingsFlickable
                    anchors.top: settingsFlickable.top
                    anchors.right: settingsFlickable.right
                    anchors.bottom: settingsFlickable.bottom
                    policy: settingsFlickable.contentHeight > settingsFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: settingsContentCol
                    width: Math.max(
                        480,
                        settingsFlickable.width - (settingsFlickable.contentHeight > settingsFlickable.height ? 18 : 2)
                    )
                    spacing: 12

                    // Section 1: HTTP REST API Server (Remote Control & Programming)
                    Rectangle {
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: restApiCol.implicitHeight + 20
                        radius: 8
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: deviceController.restApiEnabled
                            ? Kirigami.Theme.positiveTextColor
                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                        border.width: 1

                        ColumnLayout {
                            id: restApiCol
                            anchors.fill: parent
                            anchors.margins: 10
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("🌐 HTTP REST API SERVER (REMOTE CONTROL & PROGRAMMING)")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                }

                                Rectangle {
                                    radius: 4
                                    color: deviceController.restApiEnabled
                                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.18)
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.1)
                                    border.color: deviceController.restApiEnabled
                                        ? Kirigami.Theme.positiveTextColor
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.3)
                                    border.width: 1
                                    implicitWidth: restStatusBadgeLbl.implicitWidth + 12
                                    implicitHeight: 22

                                    QQC2.Label {
                                        id: restStatusBadgeLbl
                                        anchors.centerIn: parent
                                        text: deviceController.restApiStatus
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: deviceController.restApiEnabled
                                            ? root.highContrastPositiveText
                                            : Kirigami.Theme.disabledTextColor
                                    }
                                }
                            }

                            // Prominent Security Implications Box
                            Rectangle {
                                Layout.fillWidth: true
                                radius: 6
                                color: Qt.alpha("#f59e0b", 0.14)
                                border.color: Qt.alpha("#f59e0b", 0.65)
                                border.width: 1
                                implicitHeight: restSecWarnCol.implicitHeight + 14

                                ColumnLayout {
                                    id: restSecWarnCol
                                    anchors.fill: parent
                                    anchors.margins: 7
                                    spacing: 4

                                    QQC2.Label {
                                        text: qsTr("⚠ Security Implications — Please Read Before Enabling")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: root.highContrastWarningText
                                        Layout.fillWidth: true
                                        wrapMode: Text.WordWrap
                                    }

                                    QQC2.Label {
                                        text: qsTr("• Full Device & Host Control: Any client with access to this API can switch/create/edit/delete profiles and keys, write to the microSD card, inject duckyScript keystroke macros onto the host PC, control RGB LEDs and the OLED display, and trigger host actions (APP_LAUNCH, WEB_API, SYS_ACTION) bound to keys.\n• Network Exposure (127.0.0.1 vs 0.0.0.0): Binding to 127.0.0.1 (Default) restricts access strictly to local processes on this computer. Binding to 0.0.0.0 or a LAN IP exposes an unencrypted plain-HTTP server to all devices on your local network.\n• Token Authentication: Keep Bearer Token Authentication enabled at all times (send 'Authorization: Bearer <token>' or 'X-API-Token: <token>'). Never expose this plain-HTTP port directly to the public internet; use an HTTPS reverse proxy or SSH/WireGuard tunnel if remote access is required.")
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.textColor
                                        Layout.fillWidth: true
                                        wrapMode: Text.WordWrap
                                    }
                                }
                            }

                            QQC2.CheckBox {
                                id: restApiEnableCheck
                                Layout.fillWidth: true
                                text: qsTr("Enable HTTP REST API Server (/api/v1/*)")
                                font.bold: true
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Listen IP:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    height: 28
                                    verticalAlignment: Text.AlignVCenter
                                }

                                QQC2.TextField {
                                    id: restApiIpField
                                    width: 140
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    placeholderText: "127.0.0.1"
                                }

                                QQC2.Button {
                                    text: qsTr("127.0.0.1 (Localhost Default)")
                                    implicitHeight: 28
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: restApiIpField.text = "127.0.0.1"
                                }

                                QQC2.Button {
                                    text: qsTr("0.0.0.0 (All Interfaces / LAN)")
                                    implicitHeight: 28
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: restApiIpField.text = "0.0.0.0"
                                }

                                QQC2.Label {
                                    text: qsTr("Port:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    height: 28
                                    verticalAlignment: Text.AlignVCenter
                                }

                                QQC2.TextField {
                                    id: restApiPortField
                                    width: 80
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    placeholderText: "28491"
                                    validator: IntValidator { bottom: 1024; top: 65535 }
                                }
                            }

                            QQC2.CheckBox {
                                id: restApiRequireAuthCheck
                                Layout.fillWidth: true
                                text: qsTr("Require Bearer Token Authentication (Recommended — header 'Authorization: Bearer <token>')")
                                font.bold: true
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.Label {
                                    text: qsTr("API Token:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.TextField {
                                    id: restApiTokenField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 140
                                    Layout.preferredWidth: 1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    echoMode: settingsDialog.showRestToken ? TextInput.Normal : TextInput.Password
                                    placeholderText: qsTr("Click 'Generate Token' to create a 160-bit API token")
                                }

                                QQC2.Button {
                                    text: settingsDialog.showRestToken ? qsTr("Hide") : qsTr("Show")
                                    implicitHeight: 28
                                    onClicked: settingsDialog.showRestToken = !settingsDialog.showRestToken
                                }

                                QQC2.Button {
                                    icon.name: "edit-copy"
                                    text: qsTr("Copy Token")
                                    implicitHeight: 28
                                    onClicked: {
                                        const prevShow = settingsDialog.showRestToken
                                        settingsDialog.showRestToken = true
                                        restApiTokenField.selectAll()
                                        restApiTokenField.copy()
                                        restApiTokenField.deselect()
                                        settingsDialog.showRestToken = prevShow
                                        settingsDialog.restApiFeedback = qsTr("✓ Copied API Bearer token to clipboard.")
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "view-refresh"
                                    text: qsTr("Generate Token")
                                    implicitHeight: 28
                                    onClicked: {
                                        const newTok = deviceController.regenerateRestApiToken()
                                        restApiTokenField.text = newTok
                                        settingsDialog.restApiFeedback = qsTr("✓ Generated new 160-bit API token (ddk_...).")
                                    }
                                }
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Button {
                                    icon.name: "dialog-ok-apply"
                                    text: qsTr("Apply & Restart REST API Server")
                                    highlighted: true
                                    font.bold: true
                                    onClicked: {
                                        const portNum = parseInt(restApiPortField.text.trim(), 10) || 28491
                                        if (deviceController.applyRestApiConfig(
                                            restApiEnableCheck.checked,
                                            restApiIpField.text.trim(),
                                            portNum,
                                            restApiRequireAuthCheck.checked,
                                            restApiTokenField.text.trim()
                                        )) {
                                            settingsDialog.syncRestApiFieldsFromController()
                                            settingsDialog.restApiFeedback = qsTr("✓ %1").arg(deviceController.restApiStatus)
                                        } else {
                                            settingsDialog.restApiFeedback = qsTr("✗ %1").arg(deviceController.statusMessage)
                                        }
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "edit-reset"
                                    text: qsTr("Reset Defaults (127.0.0.1:28491)")
                                    onClicked: {
                                        if (deviceController.resetRestApiDefaults()) {
                                            settingsDialog.syncRestApiFieldsFromController()
                                            settingsDialog.restApiFeedback = qsTr("✓ Reset REST API settings to defaults (127.0.0.1:28491, Auth Required).")
                                        }
                                    }
                                }
                            }

                            QQC2.Label {
                                visible: settingsDialog.restApiFeedback.length > 0
                                text: settingsDialog.restApiFeedback
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: settingsDialog.restApiFeedback.startsWith("✓")
                                    ? Kirigami.Theme.positiveTextColor
                                    : Kirigami.Theme.neutralTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }
                        }
                    }

                    // Section 2: Global Profile Change Webhook
                    Rectangle {
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: globalWebhookCol.implicitHeight + 20
                        radius: 8
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: globalWebhookEnableCheck.checked
                            ? Kirigami.Theme.highlightColor
                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                        border.width: 1

                        ColumnLayout {
                            id: globalWebhookCol
                            anchors.fill: parent
                            anchors.margins: 10
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("🔔 GLOBAL PROFILE CHANGE WEBHOOK")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                }

                                QQC2.Button {
                                    icon.name: "network-transmit-receive"
                                    text: deviceController.webApiTestBusy ? qsTr("Testing...") : qsTr("Test Webhook")
                                    enabled: !deviceController.webApiTestBusy && globalWebhookUrlField.text.trim().length > 0
                                    onClicked: {
                                        deviceController.testProfileWebhookJson(
                                            JSON.stringify({
                                                enabled: true,
                                                method: globalWebhookMethodCombo.currentText || "POST",
                                                url: globalWebhookUrlField.text.trim(),
                                                headers: globalWebhookHeadersArea.text,
                                                body: globalWebhookBodyArea.text
                                            }),
                                            0
                                        )
                                        settingsDialog.globalWebhookFeedback = qsTr("⏳ Testing webhook → %1 (see result below)").arg(globalWebhookUrlField.text.trim())
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "document-save"
                                    text: qsTr("Save Global Webhook")
                                    highlighted: true
                                    onClicked: settingsDialog.saveGlobalWebhookConfig()
                                }
                            }

                            QQC2.CheckBox {
                                id: globalWebhookEnableCheck
                                Layout.fillWidth: true
                                text: qsTr("Call this HTTP Webhook whenever the active profile changes (UI, Hardware buttons, Window Auto-Switcher, or REST API)")
                                onToggled: settingsDialog.saveGlobalWebhookConfig()
                                contentItem: QQC2.Label {
                                    leftPadding: (globalWebhookEnableCheck.indicator ? globalWebhookEnableCheck.indicator.width : 18) + globalWebhookEnableCheck.spacing
                                    text: globalWebhookEnableCheck.text
                                    font: globalWebhookEnableCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.ComboBox {
                                    id: globalWebhookMethodCombo
                                    Layout.preferredWidth: 105
                                    model: ["GET", "POST", "PUT", "PATCH", "DELETE"]
                                    currentIndex: 1
                                }

                                QQC2.TextField {
                                    id: globalWebhookUrlField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 120
                                    Layout.preferredWidth: 1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    placeholderText: qsTr("http://homeassistant.local:8123/api/webhook/duckypad_profile_changed")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Headers (Optional, one 'Header-Name: Value' per line):")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.TextArea {
                                id: globalWebhookHeadersArea
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 44
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                wrapMode: TextEdit.Wrap
                                clip: true
                                placeholderText: qsTr("Content-Type: application/json")
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Payload Body (Placeholders: {{profile_number}}, {{profile_name}}, {{previous_profile_number}}, {{previous_profile_name}}, {{source}}, {{device_serial}}, {{timestamp}}):")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.TextArea {
                                id: globalWebhookBodyArea
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 54
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                wrapMode: TextEdit.Wrap
                                clip: true
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }

                            QQC2.Label {
                                visible: settingsDialog.globalWebhookFeedback.length > 0
                                text: settingsDialog.globalWebhookFeedback
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: settingsDialog.globalWebhookFeedback.startsWith("✓")
                                    ? Kirigami.Theme.positiveTextColor
                                    : Kirigami.Theme.neutralTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }
                        }
                    }

                    // Section 3: Appearance & System Integration
                    Rectangle {
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: themeCol.implicitHeight + 20
                        radius: 8
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                        border.width: 1

                        ColumnLayout {
                            id: themeCol
                            anchors.fill: parent
                            anchors.margins: 10
                            spacing: 10

                            QQC2.Label {
                                text: qsTr("APPEARANCE & SYSTEM INTEGRATION")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                            }

                            GridLayout {
                                Layout.fillWidth: true
                                columns: 2
                                rowSpacing: 8
                                columnSpacing: 12

                                QQC2.Label {
                                    text: qsTr("Application Theme:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.textColor
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.Button {
                                        text: qsTr("◐ System")
                                        checkable: true
                                        checked: deviceController.appThemeMode === "system"
                                        highlighted: checked
                                        onClicked: deviceController.applyAppThemeMode("system")
                                    }

                                    QQC2.Button {
                                        text: qsTr("☀ Light")
                                        checkable: true
                                        checked: deviceController.appThemeMode === "light"
                                        highlighted: checked
                                        onClicked: deviceController.applyAppThemeMode("light")
                                    }

                                    QQC2.Button {
                                        text: qsTr("🌙 Dark")
                                        checkable: true
                                        checked: deviceController.appThemeMode === "dark"
                                        highlighted: checked
                                        onClicked: deviceController.applyAppThemeMode("dark")
                                    }

                                    Item { Layout.fillWidth: true }
                                }

                                QQC2.Label {
                                    text: qsTr("Taskbar Tray Icon:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.textColor
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.Button {
                                        text: qsTr("◐ System")
                                        checkable: true
                                        checked: deviceController.trayIconMode === "auto"
                                        highlighted: checked
                                        onClicked: deviceController.applyTrayIconMode("auto")
                                    }

                                    QQC2.Button {
                                        text: qsTr("☀ Light Panel")
                                        checkable: true
                                        checked: deviceController.trayIconMode === "light"
                                        highlighted: checked
                                        onClicked: deviceController.applyTrayIconMode("light")
                                    }

                                    QQC2.Button {
                                        text: qsTr("🌙 Dark Panel")
                                        checkable: true
                                        checked: deviceController.trayIconMode === "dark"
                                        highlighted: checked
                                        onClicked: deviceController.applyTrayIconMode("dark")
                                    }

                                    Item { Layout.fillWidth: true }
                                }
                            }

                            Rectangle {
                                Layout.fillWidth: true
                                height: 1
                                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.CheckBox {
                                    Layout.fillWidth: true
                                    text: qsTr("Start on KDE System Login (Autostart, Minimized to Tray)")
                                    checked: deviceController.autostartEnabled
                                    onToggled: deviceController.applyAutostartEnabled(checked)
                                }

                                QQC2.CheckBox {
                                    Layout.fillWidth: true
                                    text: qsTr("Keep Running in Taskbar on Window Close")
                                    checked: deviceController.closeToTray
                                    onToggled: deviceController.setCloseToTrayEnabled(checked)
                                }
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    highlighted: true
                    onClicked: settingsDialog.close()
                }
            }
        }
    }

    // Dedicated Window for Configuring Named Reusable Web API Destinations
    Window {
        id: webApiDestinationsDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Web API Destinations — DuckyDeck Studio")
        width: 740
        height: 560
        minimumWidth: 580
        minimumHeight: 420
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property string editingOriginalName: ""

        function openForDestination(destName, fallbackMethod, fallbackUrl, fallbackHeaders) {
            const clean = (destName || "").trim()
            const list = root.webApiDestinationsList || []
            let found = null
            if (clean.length > 0) {
                for (let i = 0; i < list.length; ++i) {
                    if ((list[i].name || "").trim().toLowerCase() === clean.toLowerCase()) {
                        found = list[i]
                        break
                    }
                }
            }
            if (found) {
                webApiDestinationsDialog.loadEntryIntoForm(found)
            } else {
                webApiDestinationsDialog.editingOriginalName = ""
                destManagerNameField.text = clean
                const mIdx = root.webApiMethods.indexOf(((fallbackMethod || "POST").trim().toUpperCase()))
                destManagerMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
                destManagerUrlField.text = (fallbackUrl || "").trim()
                destManagerHeadersField.text = (fallbackHeaders || "").trim()
            }
            webApiDestinationsDialog.show()
            webApiDestinationsDialog.raise()
            webApiDestinationsDialog.requestActivate()
        }

        function loadEntryIntoForm(entry) {
            if (!entry) return
            webApiDestinationsDialog.editingOriginalName = entry.name || ""
            destManagerNameField.text = entry.name || ""
            const mIdx = root.webApiMethods.indexOf(((entry.method || "POST").trim().toUpperCase()))
            destManagerMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
            destManagerUrlField.text = entry.url || ""
            destManagerHeadersField.text = entry.headers || ""
        }

        function clearForm() {
            webApiDestinationsDialog.editingOriginalName = ""
            destManagerNameField.text = ""
            destManagerMethodCombo.currentIndex = 0
            destManagerUrlField.text = ""
            destManagerHeadersField.text = ""
            destManagerNameField.forceActiveFocus()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: webApiDestinationsDialog.visible
            onActivated: webApiDestinationsDialog.close()
            onAmbiguouslyActivated: webApiDestinationsDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2

                    Kirigami.Heading {
                        level: 2
                        text: qsTr("Reusable Web API Destinations")
                        color: Kirigami.Theme.textColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }

                    QQC2.Label {
                        text: qsTr("Configure named HTTP endpoints (Method, URL, and Headers) once and reuse them across keys with different payloads.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                    }
                }

                QQC2.Button {
                    icon.name: "list-add"
                    text: qsTr("New Destination")
                    onClicked: webApiDestinationsDialog.clearForm()
                }
            }

            // Saved Destinations List
            Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 8
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1

                Flickable {
                    id: destListFlickable
                    anchors.fill: parent
                    anchors.margins: 8
                    clip: true
                    contentWidth: width
                    contentHeight: destListCol.implicitHeight + 4
                    boundsBehavior: Flickable.StopAtBounds
                    flickableDirection: Flickable.VerticalFlick

                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: destListFlickable
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: destListFlickable.contentHeight > destListFlickable.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    ColumnLayout {
                        id: destListCol
                        width: Math.max(200, destListFlickable.width - (destListFlickable.contentHeight > destListFlickable.height ? 16 : 0))
                        spacing: 6

                        QQC2.Label {
                            visible: (root.webApiDestinationsList || []).length === 0
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.topMargin: 28
                            horizontalAlignment: Text.AlignHCenter
                            wrapMode: Text.WordWrap
                            text: qsTr("No saved Web API destinations yet.\nEnter a Name, HTTP Method, URL, and optional Headers below and click Save Destination.")
                            color: Kirigami.Theme.disabledTextColor
                            font.pointSize: 10 * root.fontScale
                        }

                        Repeater {
                            model: root.webApiDestinationsList || []

                            delegate: Rectangle {
                                required property var modelData
                                required property int index
                                readonly property bool isBeingEdited: (webApiDestinationsDialog.editingOriginalName || "").toLowerCase() === (modelData.name || "").toLowerCase()
                                readonly property bool isSelectedOnCurrentKey: root.hasSelectedSlot && (root.webApiSelectedDestination || "").toLowerCase() === (modelData.name || "").toLowerCase()

                                Layout.fillWidth: true
                                implicitHeight: destRowLayout.implicitHeight + 14
                                radius: 6
                                color: isBeingEdited
                                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                                    : Qt.rgba(Kirigami.Theme.backgroundColor.r, Kirigami.Theme.backgroundColor.g, Kirigami.Theme.backgroundColor.b, 0.85)
                                border.color: isBeingEdited
                                    ? Kirigami.Theme.highlightColor
                                    : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                border.width: 1

                                RowLayout {
                                    id: destRowLayout
                                    anchors.fill: parent
                                    anchors.margins: 7
                                    spacing: 8

                                    Rectangle {
                                        radius: 4
                                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.55)
                                        border.width: 1
                                        implicitWidth: Math.max(48, methodBadgeLbl.implicitWidth + 12)
                                        implicitHeight: 22

                                        QQC2.Label {
                                            id: methodBadgeLbl
                                            anchors.centerIn: parent
                                            text: (modelData.method || "POST").toUpperCase()
                                            font.family: "monospace"
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: root.highContrastHighlightText
                                        }
                                    }

                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 0
                                        Layout.preferredWidth: 1
                                        spacing: 2

                                        RowLayout {
                                            Layout.fillWidth: true
                                            spacing: 6

                                            QQC2.Label {
                                                text: modelData.name || ""
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.textColor
                                                elide: Text.ElideRight
                                                Layout.fillWidth: true
                                                Layout.minimumWidth: 0
                                            }

                                            Rectangle {
                                                visible: isSelectedOnCurrentKey
                                                radius: 3
                                                color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.18)
                                                implicitWidth: activeBadgeText.implicitWidth + 10
                                                implicitHeight: 18

                                                QQC2.Label {
                                                    id: activeBadgeText
                                                    anchors.centerIn: parent
                                                    text: qsTr("Active on Key %1").arg(root.selectedSlotNumber)
                                                    font.pointSize: 10 * root.fontScale
                                                    font.bold: true
                                                    color: Kirigami.Theme.positiveTextColor
                                                }
                                            }
                                        }

                                        QQC2.Label {
                                            Layout.fillWidth: true
                                            Layout.minimumWidth: 0
                                            text: modelData.url || ""
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideMiddle
                                        }
                                    }

                                    QQC2.Button {
                                        visible: root.hasSelectedSlot
                                        text: isSelectedOnCurrentKey ? qsTr("✓ Selected") : qsTr("Use on Key %1").arg(root.selectedSlotNumber)
                                        highlighted: isSelectedOnCurrentKey
                                        implicitHeight: 26
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: {
                                            root.syncingWebApiScript = true
                                            root.webApiSelectedDestination = modelData.name || ""
                                            const mIdx = root.webApiMethods.indexOf((modelData.method || "POST").toUpperCase())
                                            webApiMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
                                            webApiUrlField.text = modelData.url || ""
                                            webApiHeadersField.text = modelData.headers || ""
                                            webApiDestinationCombo.currentIndex = index + 1
                                            if (!webApiEnabledCheck.checked) {
                                                webApiEnabledCheck.checked = true
                                            }
                                            root.syncingWebApiScript = false
                                            root.syncWebApiBuilderToScript()
                                        }
                                    }

                                    QQC2.Button {
                                        icon.name: "document-edit"
                                        text: qsTr("Edit")
                                        implicitHeight: 26
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: webApiDestinationsDialog.loadEntryIntoForm(modelData)
                                    }

                                    QQC2.Button {
                                        icon.name: "edit-delete"
                                        implicitHeight: 26
                                        implicitWidth: 30
                                        onClicked: {
                                            const delName = (modelData.name || "").trim()
                                            if (delName.length === 0) return
                                            deviceController.deleteWebApiDestination(delName)
                                            if ((root.webApiSelectedDestination || "").toLowerCase() === delName.toLowerCase()) {
                                                root.webApiSelectedDestination = ""
                                                webApiDestinationCombo.currentIndex = 0
                                                root.syncWebApiBuilderToScript()
                                            }
                                            if ((webApiDestinationsDialog.editingOriginalName || "").toLowerCase() === delName.toLowerCase()) {
                                                webApiDestinationsDialog.clearForm()
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Delete destination '%1'").arg(modelData.name || "")
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Destination Editor Card
            Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                Layout.fillWidth: true
                implicitHeight: destEditorFormCol.implicitHeight + 20
                radius: 8
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1

                ColumnLayout {
                    id: destEditorFormCol
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 8

                    QQC2.Label {
                        text: webApiDestinationsDialog.editingOriginalName.length > 0
                            ? qsTr("EDIT DESTINATION: %1").arg(webApiDestinationsDialog.editingOriginalName)
                            : qsTr("ADD / UPDATE DESTINATION")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Name:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.preferredWidth: 64
                        }

                        QQC2.TextField {
                            id: destManagerNameField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 80
                            Layout.preferredWidth: 1
                            selectByMouse: true
                            placeholderText: qsTr("e.g. Home Assistant — Light Toggle, OBS Webhook, Discord Bot")
                        }

                        QQC2.Label {
                            text: qsTr("Method:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.ComboBox {
                            id: destManagerMethodCombo
                            model: root.webApiMethods
                            font.bold: true
                            Layout.preferredWidth: 100
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("URL:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.preferredWidth: 64
                        }

                        QQC2.TextField {
                            id: destManagerUrlField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 80
                            Layout.preferredWidth: 1
                            font.family: "monospace"
                            selectByMouse: true
                            placeholderText: qsTr("https://api.example.com/webhook  or  http://homeassistant.local:8123/api/services/light/toggle")
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Headers:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.preferredWidth: 64
                            Layout.alignment: Qt.AlignTop
                        }

                        QQC2.TextArea {
                            id: destManagerHeadersField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 80
                            Layout.preferredWidth: 1
                            implicitHeight: 54
                            wrapMode: TextEdit.Wrap
                            clip: true
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            placeholderText: qsTr("Optional HTTP headers, one per line (e.g. Content-Type: application/json; charset=utf-8)")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.backgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        Item { Layout.fillWidth: true }

                        QQC2.Button {
                            text: qsTr("Clear")
                            enabled: destManagerNameField.text.length > 0 || destManagerUrlField.text.length > 0 || destManagerHeadersField.text.length > 0
                            onClicked: webApiDestinationsDialog.clearForm()
                        }

                        QQC2.Button {
                            icon.name: "document-save"
                            text: webApiDestinationsDialog.editingOriginalName.length > 0
                                ? qsTr("Save Changes to Destination")
                                : qsTr("Save Destination")
                            highlighted: true
                            enabled: destManagerNameField.text.trim().length > 0 && destManagerUrlField.text.trim().length > 0
                            onClicked: {
                                const newName = destManagerNameField.text.trim()
                                const oldName = webApiDestinationsDialog.editingOriginalName.trim()
                                const mIdx = Math.max(0, destManagerMethodCombo.currentIndex)
                                const methodStr = root.webApiMethods[mIdx] || "POST"
                                const urlStr = destManagerUrlField.text.trim()
                                const headersStr = destManagerHeadersField.text.trim()

                                if (oldName.length > 0 && oldName.toLowerCase() !== newName.toLowerCase()) {
                                    deviceController.deleteWebApiDestination(oldName)
                                }
                                deviceController.saveWebApiDestination(newName, methodStr, urlStr, headersStr)
                                webApiDestinationsDialog.editingOriginalName = newName

                                if (root.hasSelectedSlot && (
                                    root.webApiSelectedDestination.trim().length === 0
                                    || root.webApiSelectedDestination.trim().toLowerCase() === oldName.toLowerCase()
                                    || root.webApiSelectedDestination.trim().toLowerCase() === newName.toLowerCase()
                                )) {
                                    root.syncingWebApiScript = true
                                    root.webApiSelectedDestination = newName
                                    webApiMethodCombo.currentIndex = mIdx
                                    webApiUrlField.text = urlStr
                                    webApiHeadersField.text = headersStr
                                    const matchedIdx = root.findWebApiDestinationIndex(newName, methodStr, urlStr, headersStr)
                                    if (matchedIdx >= 0) {
                                        webApiDestinationCombo.currentIndex = matchedIdx + 1
                                    }
                                    if (!webApiEnabledCheck.checked && urlStr.length > 0) {
                                        webApiEnabledCheck.checked = true
                                    }
                                    root.syncingWebApiScript = false
                                    root.syncWebApiBuilderToScript()
                                }
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    highlighted: true
                    onClicked: webApiDestinationsDialog.close()
                }
            }
        }
    }

    // Profile Sync Status Menu
    QQC2.Menu {
        id: profileSyncMenu
        title: qsTr("Profile Sync Status")

        QQC2.MenuItem {
            text: deviceController.profileSyncState === "synced"
                ? qsTr("✓ Validated & Synced with duckyPad SD")
                : (deviceController.profileSyncState === "unsynced"
                    ? qsTr("⚠ Unsynced Profile Changes")
                    : qsTr("⚠ Profile Mismatch Detected"))
            enabled: false
            font.bold: true
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "view-list-details"
            text: qsTr("📋 Review Unsynced Changes Summary...")
            onTriggered: root.openUnsyncedChangesDialog()
        }

        QQC2.MenuItem {
            icon.name: "document-save"
            text: qsTr("⬆ Sync (Write) Changes to duckyPad SD")
            enabled: !deviceController.sdSyncBusy
                && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                && (deviceController.connected || deviceController.sdMountPath.length > 0)
            onTriggered: root.triggerSyncToDeviceSd()
        }

        QQC2.MenuItem {
            icon.name: "view-refresh"
            text: qsTr("⬇ Reload (Read) Profiles from duckyPad SD")
            enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
            onTriggered: root.triggerReloadFromDeviceSd()
        }

        QQC2.MenuItem {
            icon.name: "dialog-information"
            text: qsTr("View Sync Progress & Validation Details...")
            onTriggered: {
                root.refreshUnsyncedSummaryModel()
                sdSyncDialog.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "folder-open"
            text: qsTr("Open SD Folder...")
            onTriggered: sdFolderDialog.open()
        }
    }

    // Hamburger Menu for Action Buttons
    QQC2.Menu {
        id: hamburgerMenu
        title: qsTr("Menu")
        onAboutToShow: deviceController.refreshSdMountStatus()

        QQC2.MenuItem {
            icon.name: "view-refresh"
            text: qsTr("Scan USB HID")
            onTriggered: {
                deviceController.refreshDevice()
                root.refreshSelectedSlotFromController()
            }
        }

        QQC2.MenuItem {
            icon.name: "view-list-details"
            text: qsTr("📋 Review Unsynced Changes Summary...")
            onTriggered: root.openUnsyncedChangesDialog()
        }

        QQC2.MenuItem {
            icon.name: "document-save"
            text: deviceController.profileSyncState === "unsynced"
                ? qsTr("⬆ Sync Changes to duckyPad SD (Unsynced)")
                : qsTr("Sync Profiles to duckyPad SD")
            enabled: !deviceController.sdSyncBusy
                && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                && (deviceController.connected || deviceController.sdMountPath.length > 0)
            onTriggered: root.triggerSyncToDeviceSd()
        }

        QQC2.MenuItem {
            visible: deviceController.sdCardDetected && deviceController.sdMountPath.length === 0
            height: visible ? implicitHeight : 0
            icon.name: "media-mount"
            text: qsTr("Mount SD Card")
            enabled: !deviceController.sdSyncBusy && deviceController.sdCardDetected && deviceController.sdMountPath.length === 0
            onTriggered: {
                deviceController.mountSdCard()
            }
        }

        QQC2.MenuItem {
            icon.name: "media-mount"
            text: qsTr("Mount and Scan SD Card")
            enabled: !deviceController.sdSyncBusy && deviceController.sdMountPath.length === 0 && (deviceController.sdCardDetected || deviceController.connected)
            onTriggered: root.triggerReloadFromDeviceSd()
        }

        QQC2.MenuItem {
            visible: deviceController.sdMountPath.length > 0
            height: visible ? implicitHeight : 0
            icon.name: "media-flash-sd-mmc"
            text: qsTr("Scan Mounted SD Card")
            onTriggered: {
                const found = deviceController.scanMountedSdCard()
                if (found.length > 0) {
                    root.refreshSelectedSlotFromController()
                } else {
                    sdFolderDialog.open()
                }
            }
        }

        QQC2.MenuItem {
            icon.name: "media-eject"
            text: qsTr("Unmount SD Card")
            enabled: !deviceController.sdSyncBusy && deviceController.sdMountPath.length > 0
            onTriggered: {
                if (deviceController.unmountSdCard()) {
                    root.refreshSelectedSlotFromController()
                }
            }
        }

        QQC2.MenuItem {
            icon.name: "folder-open"
            text: qsTr("Open SD Folder...")
            onTriggered: sdFolderDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-new-from-template"
            text: qsTr("Profile Templates Manager...")
            onTriggered: profileTemplatesDialog.openGeneral()
        }

        QQC2.MenuItem {
            icon.name: "document-new"
            text: qsTr("New Profile Group...")
            onTriggered: newProfileGroupDialog.openWithDefaults()
        }

        QQC2.MenuItem {
            icon.name: "document-open"
            text: qsTr("Load Profile Group (.duckygroup.json)...")
            onTriggered: loadProfileGroupFileDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "document-save-all"
            text: qsTr("Save All Profiles as Group (.duckygroup.json)...")
            onTriggered: saveProfileGroupFileDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "document-import"
            text: qsTr("Import Single Profile (File / config.txt)...")
            onTriggered: importProfileFileDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "folder-open"
            text: qsTr("Import Single Profile Folder...")
            onTriggered: importProfileFolderDialog.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "preferences-system-windows"
            text: qsTr("Auto-Switcher Rules...")
            onTriggered: autoSwitchDrawer.open()
        }

        QQC2.MenuItem {
            icon.name: "preferences-system-time"
            text: qsTr("Sync RTC Clock")
            enabled: deviceController.connected
            onTriggered: deviceController.syncRtcClock()
        }

        QQC2.MenuItem {
            icon.name: "system-reboot"
            text: qsTr("Reboot duckyPad")
            enabled: deviceController.connected
            onTriggered: deviceController.rebootDevice()
        }

        QQC2.MenuItem {
            icon.name: "system-software-update"
            text: deviceController.firmwareUpdateAvailable
                ? qsTr("⬆ Firmware Manager & Updater (v%1 Available)...").arg(deviceController.latestFirmwareVersion)
                : qsTr("Firmware Manager & Updater...")
            onTriggered: {
                deviceController.checkFirmwareUpdate(true)
                firmwareUpdateDialog.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "format-list-ordered"
            text: qsTr("Variables (_GV)...")
            onTriggered: {
                pgvDrawer.readAllFromDevice()
                pgvDrawer.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "system-run"
            text: qsTr("AI Macro Architect...")
            onTriggered: aiModal.open()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "settings-configure"
            text: qsTr("Settings...")
            onTriggered: settingsDialog.open()
        }

        QQC2.MenuItem {
            icon.name: "view-list-text"
            text: qsTr("System Event Log (%1)...").arg(deviceController.eventLogCount)
            onTriggered: eventLogDialog.openLogWindow()
        }

        QQC2.MenuItem {
            icon.name: "help-contents"
            text: qsTr("User Documentation (Web Guide)...")
            onTriggered: Qt.openUrlExternally("file:///home/millinr/Builds/Ducky/duckydeck-studio/docs/user-guide.html")
        }

        QQC2.MenuItem {
            icon.name: "emblem-favorite"
            text: qsTr("☕ Buy Me a Coffee (Ko-fi)...")
            onTriggered: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
        }

        QQC2.MenuItem {
            icon.name: "help-about"
            text: qsTr("About DuckyDeck Studio...")
            onTriggered: aboutDialog.openAboutWindow()
        }

        QQC2.MenuSeparator {}

        QQC2.MenuItem {
            icon.name: "application-exit"
            text: qsTr("Quit DuckyDeck Studio")
            onTriggered: deviceController.quitApplication()
        }
    }

    FileDialog {
        id: exportEventLogFileDialog
        title: qsTr("Export System Event Log (.log)")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "log"
        nameFilters: [
            qsTr("Log Files (*.log *.txt)"),
            qsTr("All Files (*)")
        ]
        onAccepted: {
            deviceController.exportEventLog(selectedFile.toString())
        }
    }

    Window {
        id: eventLogDialog
        readonly property bool closableWithEscape: true
        title: qsTr("System Event Log — DuckyDeck Studio v%1").arg(deviceController.appVersion)
        width: 960
        height: 560
        minimumWidth: 780
        minimumHeight: 420
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openLogWindow() {
            deviceController.refreshEventLog()
            root.refreshEventLogModel()
            eventLogDialog.show()
            eventLogDialog.raise()
            eventLogDialog.requestActivate()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: eventLogDialog.visible
            onActivated: eventLogDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.mediumSpacing

            // Header Banner
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: logHeaderCol.implicitHeight + 20
                radius: Kirigami.Units.smallSpacing
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                border.width: 1

                ColumnLayout {
                    id: logHeaderCol
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 4

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("📋 System Event Log")
                            font.bold: true
                            font.pointSize: 12 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }

                        Rectangle {
                            radius: 4
                            color: Qt.alpha(Kirigami.Theme.highlightColor, 0.18)
                            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                            border.width: 1
                            implicitWidth: logCountBadgeLbl.implicitWidth + 12
                            implicitHeight: Math.round(22 * root.eventLogScale)

                            QQC2.Label {
                                id: logCountBadgeLbl
                                anchors.centerIn: parent
                                text: qsTr("%1 shown / %2 total")
                                    .arg(eventLogListView.count)
                                    .arg(deviceController.eventLogCount)
                                font.family: "monospace"
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: root.highContrastHighlightText
                            }
                        }

                        Item { Layout.fillWidth: true }

                        QQC2.Button {
                            icon.name: "view-refresh"
                            text: qsTr("Refresh")
                            implicitHeight: Math.round(26 * root.eventLogScale)
                            font.pointSize: 10 * root.fontScale
                            onClicked: {
                                deviceController.refreshEventLog()
                                root.refreshEventLogModel()
                            }
                        }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        Layout.preferredWidth: 1
                        text: qsTr("Log file: %1").arg(deviceController.logFilePath)
                        font.family: "monospace"
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        elide: Text.ElideMiddle
                    }
                }
            }

            // Search & Level Filter Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 6

                QQC2.TextField {
                    id: eventLogSearchField
                    Layout.fillWidth: true
                    Layout.minimumWidth: 140
                    Layout.preferredWidth: 1
                    implicitHeight: Math.round(30 * root.eventLogScale)
                    font.pointSize: 10 * root.fontScale
                    selectByMouse: true
                    placeholderText: qsTr("🔍 Filter events by keyword, timestamp, profile, window class, or error...")
                    text: root.eventLogSearchText
                    onTextEdited: {
                        root.eventLogSearchText = text
                        root.refreshEventLogModel()
                    }
                }

                QQC2.ToolButton {
                    visible: eventLogSearchField.text.length > 0
                    text: "✕"
                    implicitWidth: Math.round(28 * root.eventLogScale)
                    implicitHeight: Math.round(28 * root.eventLogScale)
                    onClicked: {
                        eventLogSearchField.text = ""
                        root.eventLogSearchText = ""
                        root.refreshEventLogModel()
                    }
                }

                QQC2.Label {
                    text: qsTr("Level:")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }

                Repeater {
                    model: root.eventLogLevels

                    delegate: QQC2.Button {
                        required property string modelData
                        readonly property bool isSel: root.eventLogLevelFilter === modelData
                        text: modelData
                        checkable: true
                        checked: isSel
                        highlighted: isSel
                        implicitHeight: Math.round(26 * root.eventLogScale)
                        font.pointSize: 10 * root.fontScale
                        font.bold: isSel
                        onClicked: {
                            root.eventLogLevelFilter = modelData
                            root.refreshEventLogModel()
                        }
                    }
                }
            }

            // Category Filter Pills Row (Flow wraps cleanly at any font scale)
            Flow {
                Layout.fillWidth: true
                spacing: 4

                QQC2.Label {
                    text: qsTr("Category:")
                    height: Math.round(24 * root.eventLogScale)
                    verticalAlignment: Text.AlignVCenter
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }

                Repeater {
                    model: root.eventLogCategories

                    delegate: QQC2.Button {
                        required property string modelData
                        readonly property bool isSelCat: root.eventLogCategoryFilter === modelData
                        text: modelData
                        checkable: true
                        checked: isSelCat
                        highlighted: isSelCat
                        implicitHeight: Math.round(24 * root.eventLogScale)
                        font.pointSize: 10 * root.fontScale
                        font.bold: isSelCat
                        onClicked: {
                            root.eventLogCategoryFilter = modelData
                            root.refreshEventLogModel()
                        }
                    }
                }
            }

            // Event Log List Container
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: Kirigami.Units.smallSpacing
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                QQC2.ScrollView {
                    anchors.fill: parent
                    anchors.margins: 4
                    clip: true
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        policy: eventLogListView.contentHeight > eventLogListView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }
                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                        orientation: Qt.Horizontal
                    }

                    ListView {
                        id: eventLogListView
                        model: eventLogModel
                        spacing: 3
                        boundsBehavior: Flickable.StopAtBounds

                        delegate: Rectangle {
                            id: logRow
                            required property int index
                            required property string timestamp
                            required property string level
                            required property string category
                            required property string message

                            width: eventLogListView.width - 8
                            implicitHeight: Math.max(Math.round(28 * root.eventLogScale), logRowLayout.implicitHeight + 8)
                            radius: 4
                            color: logRowMouse.containsMouse
                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                                : (index % 2 === 0
                                    ? Qt.alpha(Kirigami.Theme.backgroundColor, 0.55)
                                    : Qt.alpha(Kirigami.Theme.backgroundColor, 0.25))
                            border.color: logRow.level === "ERROR"
                                ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.45)
                                : (logRow.level === "WARN"
                                    ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.35)
                                    : "transparent")
                            border.width: 1

                            RowLayout {
                                id: logRowLayout
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 8
                                spacing: 8

                                QQC2.Label {
                                    text: logRow.timestamp
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.preferredWidth: Math.round(165 * root.eventLogScale)
                                }

                                Rectangle {
                                    radius: 3
                                    color: Qt.alpha(root.levelColorForLog(logRow.level), 0.18)
                                    border.color: Qt.alpha(root.levelColorForLog(logRow.level), 0.55)
                                    border.width: 1
                                    Layout.preferredWidth: Math.round(48 * root.eventLogScale)
                                    implicitHeight: Math.round(18 * root.eventLogScale)

                                    QQC2.Label {
                                        anchors.centerIn: parent
                                        text: logRow.level
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: root.levelColorForLog(logRow.level)
                                    }
                                }

                                Rectangle {
                                    radius: 3
                                    color: Qt.alpha(root.categoryColorForLog(logRow.category), 0.18)
                                    border.color: Qt.alpha(root.categoryColorForLog(logRow.category), 0.55)
                                    border.width: 1
                                    Layout.preferredWidth: Math.round(78 * root.eventLogScale)
                                    implicitHeight: Math.round(18 * root.eventLogScale)

                                    QQC2.Label {
                                        anchors.centerIn: parent
                                        text: logRow.category
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: root.categoryColorForLog(logRow.category)
                                    }
                                }

                                QQC2.Label {
                                    text: logRow.message
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: logRow.level === "ERROR"
                                        ? Kirigami.Theme.negativeTextColor
                                        : Kirigami.Theme.textColor
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    Layout.preferredWidth: 1
                                    wrapMode: Text.WrapAnywhere
                                }
                            }

                            MouseArea {
                                id: logRowMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: {
                                    const lineStr = "[" + logRow.timestamp + "] [" + logRow.level + "] [" + logRow.category + "] " + logRow.message
                                    root.copyTextToClipboard(lineStr)
                                }
                            }

                            QQC2.ToolTip.visible: logRowMouse.containsMouse
                            QQC2.ToolTip.delay: 400
                            QQC2.ToolTip.text: qsTr("Click to copy this log entry to clipboard")
                        }
                    }
                }
            }

            // Footer Actions Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "edit-copy"
                    text: qsTr("Copy Log to Clipboard")
                    font.pointSize: 10 * root.fontScale
                    onClicked: {
                        const txt = deviceController.formatEventLogText(
                            root.eventLogCategoryFilter,
                            root.eventLogLevelFilter,
                            root.eventLogSearchText
                        )
                        root.copyTextToClipboard(txt)
                    }
                }

                QQC2.Button {
                    icon.name: "document-export"
                    text: qsTr("Export Log (.log)...")
                    font.pointSize: 10 * root.fontScale
                    onClicked: exportEventLogFileDialog.open()
                }

                QQC2.Button {
                    icon.name: "edit-clear-history"
                    text: qsTr("Clear Log")
                    font.pointSize: 10 * root.fontScale
                    onClicked: {
                        deviceController.clearEventLog()
                        root.refreshEventLogModel()
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    text: qsTr("Close")
                    font.pointSize: 10 * root.fontScale
                    onClicked: eventLogDialog.close()
                }
            }
        }
    }

    Window {
        id: aboutDialog
        readonly property bool closableWithEscape: true
        title: qsTr("About DuckyDeck Studio v%1").arg(deviceController.appVersion)
        width: 720
        height: 550
        minimumWidth: 680
        minimumHeight: 510
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openAboutWindow() {
            aboutDialog.show()
            aboutDialog.raise()
            aboutDialog.requestActivate()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: aboutDialog.visible
            onActivated: aboutDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.mediumSpacing

            // App Header Banner
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: aboutHeaderCol.implicitHeight + 24
                radius: Kirigami.Units.smallSpacing
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                border.width: 1

                ColumnLayout {
                    id: aboutHeaderCol
                    anchors.fill: parent
                    anchors.margins: 12
                    spacing: 6

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 10

                        // Application Icon matching data/icons/io.github.duckydeck.Studio.svg
                        Rectangle {
                            width: 32
                            height: 32
                            radius: 6
                            color: "#16181d"
                            border.color: "#38bdf8"
                            border.width: 1.5

                            // 3x4 RGB key matrix
                            Repeater {
                                model: [
                                    { x: 4,  y: 5,  c: "#00aaff" }, { x: 10, y: 5,  c: "#00aaff" }, { x: 16, y: 5,  c: "#00aaff" },
                                    { x: 4,  y: 11, c: "#28dc64" }, { x: 10, y: 11, c: "#28dc64" }, { x: 16, y: 11, c: "#28dc64" },
                                    { x: 4,  y: 17, c: "#ffb400" }, { x: 10, y: 17, c: "#ffb400" }, { x: 16, y: 17, c: "#ffb400" },
                                    { x: 4,  y: 23, c: "#ff3c3c" }, { x: 10, y: 23, c: "#b450ff" }, { x: 16, y: 23, c: "#b450ff" }
                                ]
                                delegate: Rectangle {
                                    required property var modelData
                                    x: modelData.x
                                    y: modelData.y
                                    width: 4.5
                                    height: 4.5
                                    radius: 1
                                    color: modelData.c
                                }
                            }

                            // Rotary encoders & OLED on right
                            Rectangle { x: 22.5; y: 5;  width: 5.5; height: 5.5; radius: 2.75; color: "#262b36"; border.color: "#38bdf8"; border.width: 1 }
                            Rectangle { x: 22.5; y: 12; width: 5.5; height: 5.5; radius: 2.75; color: "#262b36"; border.color: "#38bdf8"; border.width: 1 }
                            Rectangle { x: 22;   y: 19; width: 6.5; height: 8.5; radius: 1;    color: "#000000"; border.color: "#94a3b8"; border.width: 1 }
                        }

                        QQC2.Label {
                            text: qsTr("DuckyDeck Studio")
                            font.bold: true
                            font.pointSize: 14 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }

                        Rectangle {
                            radius: 4
                            color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.6)
                            border.width: 1
                            implicitWidth: verPillLbl.implicitWidth + 14
                            implicitHeight: 24

                            QQC2.Label {
                                id: verPillLbl
                                anchors.centerIn: parent
                                text: qsTr("v%1").arg(deviceController.appVersion)
                                font.family: "monospace"
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: root.highContrastHighlightText
                            }
                        }

                        Item { Layout.fillWidth: true }

                        // Ko-fi Buy Me a Coffee Widget in About Header
                        Rectangle {
                            id: aboutKofiWidget
                            radius: 16
                            color: aboutKofiMouse.containsMouse ? "#c81e1e" : "#d93834"
                            border.color: "#ffffff33"
                            border.width: 1
                            implicitWidth: aboutKofiRow.implicitWidth + 22
                            implicitHeight: 30

                            RowLayout {
                                id: aboutKofiRow
                                anchors.centerIn: parent
                                spacing: 6

                                QQC2.Label {
                                    text: "☕"
                                    font.pointSize: 11 * root.fontScale
                                }

                                QQC2.Label {
                                    text: qsTr("Buy Me a Coffee")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: "#ffffff"
                                }
                            }

                            MouseArea {
                                id: aboutKofiMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
                            }

                            QQC2.ToolTip.visible: aboutKofiMouse.containsMouse
                            QQC2.ToolTip.text: qsTr("Support Russell Milliner on Ko-fi (https://ko-fi.com/russellmilliner)")
                        }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        Layout.preferredWidth: 1
                        text: qsTr("KDE Plasma 6 / Kirigami Configurator, DSVM v2 Compiler & Wayland Auto-Switcher for duckyPad Pro & duckyPad (2020)")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                    }
                }
            }

            // Metadata Details Card
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: Kirigami.Units.smallSpacing
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                Flickable {
                    id: aboutDetailsFlickable
                    anchors.fill: parent
                    anchors.margins: 14
                    clip: true
                    contentWidth: width
                    contentHeight: aboutMetadataGrid.implicitHeight + 4
                    boundsBehavior: Flickable.StopAtBounds
                    flickableDirection: Flickable.VerticalFlick

                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: aboutDetailsFlickable
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: aboutDetailsFlickable.contentHeight > aboutDetailsFlickable.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    GridLayout {
                        id: aboutMetadataGrid
                        width: Math.max(
                            200,
                            aboutDetailsFlickable.width - (aboutDetailsFlickable.contentHeight > aboutDetailsFlickable.height ? 16 : 0)
                        )
                        columns: 2
                        rowSpacing: 8
                        columnSpacing: 14

                        QQC2.Label {
                            text: qsTr("Version:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: qsTr("v%1 (DSVM v2 Bytecode Compiler)").arg(deviceController.appVersion)
                            font.family: "monospace"
                            font.bold: true
                            color: Kirigami.Theme.textColor
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            elide: Text.ElideRight
                        }

                        QQC2.Label {
                            text: qsTr("Author:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: deviceController.appAuthor
                            font.bold: true
                            color: Kirigami.Theme.textColor
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            elide: Text.ElideRight
                        }

                        QQC2.Label {
                            text: qsTr("GitHub:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            spacing: 8

                            QQC2.Label {
                                id: githubLinkLbl
                                text: deviceController.appGithubUrl
                                font.family: "monospace"
                                font.underline: githubMouse.containsMouse
                                color: root.highContrastHighlightText
                                elide: Text.ElideMiddle
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1

                                MouseArea {
                                    id: githubMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally(deviceController.appGithubUrl)
                                }
                            }

                            QQC2.Button {
                                icon.name: "edit-copy"
                                text: qsTr("Copy")
                                implicitHeight: 24
                                font.pointSize: 10 * root.fontScale
                                onClicked: root.copyTextToClipboard(deviceController.appGithubUrl)
                            }
                        }

                        QQC2.Label {
                            text: qsTr("Ko-fi Support:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            spacing: 8

                            QQC2.Label {
                                id: kofiLinkLbl
                                text: "https://ko-fi.com/russellmilliner"
                                font.family: "monospace"
                                font.underline: kofiLinkMouse.containsMouse
                                color: root.isLightTheme ? "#c81e1e" : "#ff5e5b"
                                elide: Text.ElideMiddle
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1

                                MouseArea {
                                    id: kofiLinkMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
                                }
                            }

                            QQC2.Button {
                                icon.name: "edit-copy"
                                text: qsTr("Copy")
                                implicitHeight: 24
                                font.pointSize: 10 * root.fontScale
                                onClicked: root.copyTextToClipboard("https://ko-fi.com/russellmilliner")
                            }
                        }

                        QQC2.Label {
                            text: qsTr("Special Thanks:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: qsTr("Thank you to dekuNukem for creating the duckyPad & duckyPad Pro!")
                            font.bold: true
                            color: root.highContrastPositiveText
                            wrapMode: Text.WordWrap
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                        }

                        QQC2.Label {
                            text: qsTr("duckyPad Links:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            spacing: 12

                            QQC2.Label {
                                text: "duckypad.com"
                                font.family: "monospace"
                                font.underline: duckySiteMouse.containsMouse
                                color: root.highContrastHighlightText

                                MouseArea {
                                    id: duckySiteMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally("https://www.duckypad.com")
                                }
                            }

                            QQC2.Label {
                                text: "•"
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.Label {
                                text: "Tindie Store (dekuNukem)"
                                font.family: "monospace"
                                font.underline: tindieSiteMouse.containsMouse
                                color: root.highContrastHighlightText

                                MouseArea {
                                    id: tindieSiteMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: Qt.openUrlExternally("https://www.tindie.com/stores/dekunukem/")
                                }
                            }

                            Item { Layout.fillWidth: true }
                        }

                        QQC2.Label {
                            text: qsTr("License:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: qsTr("GPL-3.0-or-later")
                            font.family: "monospace"
                            color: Kirigami.Theme.textColor
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                        }

                        QQC2.Label {
                            text: qsTr("Device Status:")
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            text: deviceController.connected
                                ? qsTr("%1 (FW: %2 • SN: %3)")
                                    .arg(deviceController.modelName)
                                    .arg(deviceController.firmwareVersion)
                                    .arg(deviceController.serialNumber)
                                : deviceController.modelName
                            font.family: "monospace"
                            color: Kirigami.Theme.textColor
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                        }
                    }
                }
            }

            // Footer Buttons
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "internet-web-browser"
                    text: qsTr("GitHub")
                    onClicked: Qt.openUrlExternally(deviceController.appGithubUrl)
                }

                QQC2.Button {
                    icon.name: "internet-web-browser"
                    text: qsTr("duckyPad Site")
                    onClicked: Qt.openUrlExternally("https://www.duckypad.com")
                }

                QQC2.Button {
                    icon.name: "internet-web-browser"
                    text: qsTr("Tindie Store")
                    onClicked: Qt.openUrlExternally("https://www.tindie.com/stores/dekunukem/")
                }

                QQC2.Button {
                    icon.name: "help-contents"
                    text: qsTr("User Guide")
                    onClicked: Qt.openUrlExternally("file:///home/millinr/Builds/Ducky/duckydeck-studio/docs/user-guide.html")
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    text: qsTr("Close")
                    highlighted: true
                    onClicked: aboutDialog.close()
                }
            }
        }
    }

    Window {
        id: unsyncedChangesDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Unsynced Profile Changes Summary — DuckyDeck Studio")
        width: Math.max(
            740,
            Math.ceil(unsyncedFooterRow.implicitWidth + 64),
            Math.ceil(unsyncedTitleMetrics.width + 180),
            Math.ceil(unsyncedBannerTitle.implicitWidth + 72)
        )
        height: Math.min(680, Screen.desktopAvailableHeight > 0 ? Screen.desktopAvailableHeight - 80 : 680)
        minimumWidth: Math.max(
            700,
            Math.ceil(unsyncedFooterRow.implicitWidth + 56),
            Math.ceil(unsyncedTitleMetrics.width + 160),
            Math.ceil(unsyncedBannerTitle.implicitWidth + 64)
        )
        minimumHeight: 480
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        onMinimumWidthChanged: {
            if (unsyncedChangesDialog.width < unsyncedChangesDialog.minimumWidth) {
                unsyncedChangesDialog.width = unsyncedChangesDialog.minimumWidth
            }
        }

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        TextMetrics {
            id: unsyncedTitleMetrics
            font.bold: true
            font.pointSize: 11 * root.fontScale
            text: unsyncedChangesDialog.title
        }

        function open() {
            if (unsyncedChangesDialog.width < unsyncedChangesDialog.minimumWidth) {
                unsyncedChangesDialog.width = unsyncedChangesDialog.minimumWidth
            }
            const targetH = Math.min(680, Screen.desktopAvailableHeight > 0 ? Screen.desktopAvailableHeight - 80 : 680)
            if (unsyncedChangesDialog.height < targetH) {
                unsyncedChangesDialog.height = targetH
            }
            unsyncedChangesDialog.show()
            unsyncedChangesDialog.raise()
            unsyncedChangesDialog.requestActivate()
            Qt.callLater(function() {
                if (unsyncedChangesDialog.width < unsyncedChangesDialog.minimumWidth) {
                    unsyncedChangesDialog.width = unsyncedChangesDialog.minimumWidth
                }
                unsyncedDialogRootLayout.forceActiveFocus()
            })
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: unsyncedChangesDialog.visible
            onActivated: unsyncedChangesDialog.close()
        }

        ColumnLayout {
            id: unsyncedDialogRootLayout
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12
            focus: true
            Keys.onEscapePressed: unsyncedChangesDialog.close()

            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: deviceController.profileSyncState === "synced"
                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.14)
                    : Qt.alpha(Kirigami.Theme.neutralTextColor, 0.16)
                border.color: deviceController.profileSyncState === "synced"
                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                    : Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
                border.width: 1
                implicitHeight: unsyncedBannerCol.implicitHeight + 20

                ColumnLayout {
                    id: unsyncedBannerCol
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 4

                    QQC2.Label {
                        id: unsyncedBannerTitle
                        Layout.fillWidth: true
                        font.bold: true
                        font.pointSize: 11 * root.fontScale
                        text: deviceController.profileSyncState === "synced"
                            ? qsTr("✓ All Profiles Match duckyPad microSD Card")
                            : qsTr("⚠ Unsynced Changes Pending microSD Sync")
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        font.pointSize: 10 * root.fontScale
                        text: root.unsyncedHeadline.length > 0
                            ? root.unsyncedHeadline
                            : deviceController.profileSyncDetail
                        color: Kirigami.Theme.textColor
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                ListView {
                    id: unsyncedListView
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 8
                    clip: true
                    model: unsyncedItemsModel
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: unsyncedListView
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: unsyncedListView.contentHeight > unsyncedListView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    delegate: Rectangle {
                        id: diffCardDelegate
                        required property string kind
                        required property int profileNumber
                        required property string profileName
                        required property string summary
                        required property string detailsText
                        required property string detailItemsJson

                        readonly property var parsedDetailItems: {
                            try {
                                const arr = JSON.parse(detailItemsJson || "[]")
                                return Array.isArray(arr) ? arr : []
                            } catch (e) {
                                return []
                            }
                        }

                        width: unsyncedListView.width - 14
                        implicitHeight: diffCardCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.backgroundColor
                        border.width: 1
                        border.color: kind === "added"
                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                            : (kind === "deleted"
                                ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.55)
                                : Qt.alpha(Kirigami.Theme.highlightColor, 0.45))

                        ColumnLayout {
                            id: diffCardCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                Rectangle {
                                    radius: 3
                                    color: kind === "added"
                                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.2)
                                        : (kind === "deleted"
                                            ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.2)
                                            : Qt.alpha(Kirigami.Theme.highlightColor, 0.2))
                                    implicitWidth: kindTagLbl.implicitWidth + 10
                                    implicitHeight: kindTagLbl.implicitHeight + 4

                                    QQC2.Label {
                                        id: kindTagLbl
                                        anchors.centerIn: parent
                                        text: kind === "added"
                                            ? qsTr("NEW")
                                            : (kind === "deleted"
                                                ? qsTr("DELETED")
                                                : (kind === "unverified" ? qsTr("UNVERIFIED") : qsTr("MODIFIED")))
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: kind === "added"
                                            ? Kirigami.Theme.positiveTextColor
                                            : (kind === "deleted"
                                                ? Kirigami.Theme.negativeTextColor
                                                : Kirigami.Theme.highlightColor)
                                    }
                                }

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    text: summary
                                    color: Kirigami.Theme.textColor
                                }

                                QQC2.Button {
                                    visible: kind === "modified" && diffCardDelegate.parsedDetailItems.length > 1
                                    icon.name: "edit-delete"
                                    text: qsTr("Delete All in Profile")
                                    implicitHeight: Math.round(24 * root.fontScale)
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: root.discardSingleUnsyncedChange(
                                        diffCardDelegate.profileNumber,
                                        diffCardDelegate.profileName,
                                        "profile_all"
                                    )
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Discard all unsynced changes in Profile #%1 and restore its microSD card state").arg(diffCardDelegate.profileNumber)
                                }
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                Repeater {
                                    model: diffCardDelegate.parsedDetailItems

                                    delegate: Rectangle {
                                        required property var modelData
                                        readonly property string itemKey: (modelData && modelData.changeKey) ? String(modelData.changeKey) : ""
                                        readonly property string itemText: (modelData && modelData.text) ? String(modelData.text) : ""

                                        Layout.fillWidth: true
                                        implicitHeight: detailRowLayout.implicitHeight + 8
                                        radius: 4
                                        color: Qt.alpha(Kirigami.Theme.alternateBackgroundColor, 0.7)
                                        border.width: 1
                                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.08)

                                        RowLayout {
                                            id: detailRowLayout
                                            anchors.fill: parent
                                            anchors.leftMargin: 8
                                            anchors.rightMargin: 6
                                            anchors.topMargin: 4
                                            anchors.bottomMargin: 4
                                            spacing: 8

                                            QQC2.Label {
                                                Layout.fillWidth: true
                                                wrapMode: Text.WordWrap
                                                font.family: "monospace"
                                                font.pointSize: 10 * root.fontScale
                                                text: "• " + itemText
                                                color: Kirigami.Theme.textColor
                                            }

                                            QQC2.Button {
                                                visible: itemKey.length > 0
                                                icon.name: "edit-delete"
                                                text: qsTr("Delete")
                                                implicitHeight: Math.round(24 * root.fontScale)
                                                font.pointSize: 10 * root.fontScale
                                                onClicked: root.discardSingleUnsyncedChange(
                                                    diffCardDelegate.profileNumber,
                                                    diffCardDelegate.profileName,
                                                    itemKey
                                                )
                                                QQC2.ToolTip.visible: hovered
                                                QQC2.ToolTip.text: qsTr("Delete this unsynced change and revert it back to the microSD card state")
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    QQC2.Label {
                        anchors.centerIn: parent
                        width: Math.max(200, unsyncedListView.width - 24)
                        horizontalAlignment: Text.AlignHCenter
                        wrapMode: Text.WordWrap
                        visible: unsyncedListView.count === 0
                        text: qsTr("✓ No unsynced changes — all profiles and keys match the microSD card.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }
                }
            }

            RowLayout {
                id: unsyncedFooterRow
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "view-refresh"
                    text: qsTr("Refresh Summary")
                    onClicked: root.refreshUnsyncedSummaryModel()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "document-save"
                    text: qsTr("⬆ Sync Changes to duckyPad SD Now")
                    enabled: !deviceController.sdSyncBusy
                        && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                        && (deviceController.connected || deviceController.sdMountPath.length > 0)
                    highlighted: deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges
                    onClicked: {
                        unsyncedChangesDialog.close()
                        root.triggerSyncToDeviceSd()
                    }
                }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: unsyncedChangesDialog.close()
                }
            }
        }
    }

    Window {
        id: firmwareUpdateDialog
        readonly property bool closableWithEscape: true
        title: qsTr("duckyPad Pro Firmware Manager & Updater — DuckyDeck Studio")
        width: 880
        height: 680
        minimumWidth: Math.max(740, Math.ceil(fwFooterRow.implicitWidth) + 32)
        minimumHeight: 540
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint

        property var availableFirmwares: []
        property string vendorFilter: "all" // "all" | "official" | "studio"
        readonly property var filteredFirmwares: {
            const all = firmwareUpdateDialog.availableFirmwares || []
            if (firmwareUpdateDialog.vendorFilter === "official") {
                return all.filter(function(item) { return !!item.is_official })
            }
            if (firmwareUpdateDialog.vendorFilter === "studio") {
                return all.filter(function(item) { return !item.is_official })
            }
            return all
        }
        property int selectedFirmwareIndex: 0
        property string customFirmwarePath: ""

        Connections {
            target: deviceController
            function onFirmwareFetchBusyChanged() {
                if (!deviceController.firmwareFetchBusy && firmwareUpdateDialog.visible) {
                    firmwareUpdateDialog.refreshCatalog()
                }
            }
        }

        function refreshCatalog() {
            let parsed = []
            try {
                parsed = JSON.parse(deviceController.listAvailableFirmwaresJson())
            } catch (e) {
                parsed = []
            }
            availableFirmwares = parsed
            if (filteredFirmwares.length > 0 && (selectedFirmwareIndex < 0 || selectedFirmwareIndex >= filteredFirmwares.length)) {
                selectedFirmwareIndex = 0
            }
        }

        function selectedFirmwareItem() {
            if (customFirmwarePath.length > 0) {
                return null
            }
            if (selectedFirmwareIndex >= 0 && selectedFirmwareIndex < filteredFirmwares.length) {
                return filteredFirmwares[selectedFirmwareIndex]
            }
            return null
        }

        function selectedTargetPath() {
            if (customFirmwarePath.length > 0) {
                return customFirmwarePath
            }
            const item = selectedFirmwareItem()
            return item ? (item.path || "") : ""
        }

        function canInstallSelected() {
            if (customFirmwarePath.length > 0) {
                return true
            }
            const item = selectedFirmwareItem()
            if (!item) return false
            return (item.path && item.path.length > 0) || (item.download_url && item.download_url.length > 0)
        }

        function selectedTargetLabel() {
            if (customFirmwarePath.length > 0) {
                const parts = customFirmwarePath.split("/")
                return parts[parts.length - 1]
            }
            const item = selectedFirmwareItem()
            if (item) {
                const tag = item.is_official ? "Official dekuNukem" : "DuckyDeck Studio"
                return "v" + item.version + " (" + tag + ")"
            }
            return ""
        }

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        FileDialog {
            id: customFirmwareFileDialog
            title: qsTr("Select duckyPad Pro Firmware Binary (DPP_FW_*.bin)")
            nameFilters: ["duckyPad Pro Firmware (*.bin)", "All Files (*)"]
            onAccepted: {
                let p = selectedFile.toString()
                if (p.startsWith("file://")) {
                    p = p.substring(7)
                }
                firmwareUpdateDialog.customFirmwarePath = p
            }
        }

        function open() {
            refreshCatalog()
            const hasOfficial = (availableFirmwares || []).some(function(item) { return !!item.is_official })
            if (!hasOfficial && !deviceController.firmwareFetchBusy) {
                deviceController.fetchOfficialFirmwaresFromSite()
            }
            firmwareUpdateDialog.show()
            firmwareUpdateDialog.raise()
            firmwareUpdateDialog.requestActivate()
            Qt.callLater(function() {
                fwDialogRootCol.forceActiveFocus()
            })
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: firmwareUpdateDialog.visible
            onActivated: firmwareUpdateDialog.close()
        }

        ColumnLayout {
            id: fwDialogRootCol
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10
            focus: true
            Keys.onEscapePressed: firmwareUpdateDialog.close()

            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: Qt.alpha(Kirigami.Theme.highlightColor, 0.12)
                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                border.width: 1
                implicitHeight: fwBannerCol.implicitHeight + 18

                ColumnLayout {
                    id: fwBannerCol
                    anchors.left: parent.left
                    anchors.right: parent.right
                    anchors.top: parent.top
                    anchors.margins: 10
                    spacing: 6

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 10

                        QQC2.Label {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.preferredWidth: 1
                            wrapMode: Text.WordWrap
                            font.bold: true
                            font.pointSize: 11 * root.fontScale
                            text: qsTr("duckyPad Pro Firmware Manager — Switch or Update Firmware Version")
                            color: Kirigami.Theme.textColor
                        }

                        Rectangle {
                            radius: 4
                            Layout.alignment: Qt.AlignTop | Qt.AlignRight
                            readonly property bool installedIsStudio: root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                            color: installedIsStudio
                                ? Qt.alpha("#a855f7", 0.22)
                                : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.18)
                            border.color: installedIsStudio
                                ? Qt.alpha("#c084fc", 0.7)
                                : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                            border.width: 1
                            implicitWidth: installedBadgeRow.implicitWidth + 14
                            implicitHeight: installedBadgeRow.implicitHeight + 6

                            RowLayout {
                                id: installedBadgeRow
                                anchors.centerIn: parent
                                spacing: 5

                                Image {
                                    visible: deviceController.firmwareVersion !== "-" && root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                                    source: root.effectiveThemeMode === "light"
                                        ? "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-light.svg"
                                        : "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg"
                                    width: 13
                                    height: 13
                                    sourceSize.width: 26
                                    sourceSize.height: 26
                                    fillMode: Image.PreserveAspectFit
                                }

                                QQC2.Label {
                                    id: installedBadgeLbl
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    text: deviceController.firmwareVersion !== "-"
                                        ? (root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                                            ? qsTr("Installed: v%1 [DuckyDeck Studio]").arg(root.cleanFirmwareVersionLabel(deviceController.firmwareVersion))
                                            : qsTr("Installed: v%1 [✔ Official dekuNukem]").arg(root.cleanFirmwareVersionLabel(deviceController.firmwareVersion)))
                                        : qsTr("Device Not Connected")
                                    color: Kirigami.Theme.textColor
                                }
                            }
                        }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        wrapMode: Text.WordWrap
                        font.pointSize: 10 * root.fontScale
                        text: qsTr("DuckyDeck Studio includes its custom DuckyDeck Studio Firmware v1.0.0 (with monochrome key icons & 0°/90°/180°/270° OLED orientation) and can fetch & download official dekuNukem firmware releases directly from github.com/dekuNukem/duckyPad-Pro.")
                        color: Kirigami.Theme.textColor
                    }
                }
            }

            // Filter & Action Toolbar (two responsive rows to prevent overflow)
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    QQC2.Label {
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        text: qsTr("Filter Origin:")
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Button {
                        text: qsTr("All (%1)").arg(firmwareUpdateDialog.availableFirmwares.length)
                        highlighted: firmwareUpdateDialog.vendorFilter === "all"
                        onClicked: {
                            firmwareUpdateDialog.vendorFilter = "all"
                            firmwareUpdateDialog.selectedFirmwareIndex = 0
                        }
                    }

                    QQC2.Button {
                        icon.source: root.effectiveThemeMode === "light"
                            ? "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-light.svg"
                            : "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg"
                        text: qsTr("DuckyDeck Studio")
                        highlighted: firmwareUpdateDialog.vendorFilter === "studio"
                        onClicked: {
                            firmwareUpdateDialog.vendorFilter = "studio"
                            firmwareUpdateDialog.selectedFirmwareIndex = 0
                        }
                    }

                    QQC2.Button {
                        text: qsTr("✔ Official dekuNukem")
                        highlighted: firmwareUpdateDialog.vendorFilter === "official"
                        onClicked: {
                            firmwareUpdateDialog.vendorFilter = "official"
                            firmwareUpdateDialog.selectedFirmwareIndex = 0
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        icon.name: "download"
                        text: deviceController.firmwareFetchBusy
                            ? qsTr("Fetching Official...")
                            : qsTr("⬇ Fetch Official List")
                        enabled: !deviceController.firmwareFetchBusy && !deviceController.sdSyncBusy
                        highlighted: true
                        onClicked: deviceController.fetchOfficialFirmwaresFromSite()
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Query dekuNukem/duckyPad-Pro on GitHub for all available official DPP_FW_*.bin firmware releases")
                    }
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    QQC2.Button {
                        icon.name: "internet-services"
                        text: qsTr("Open Official Releases Page (GitHub)")
                        onClicked: Qt.openUrlExternally("https://github.com/dekuNukem/duckyPad-Pro/releases")
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Open https://github.com/dekuNukem/duckyPad-Pro/releases in your web browser")
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        icon.name: "document-open"
                        text: firmwareUpdateDialog.customFirmwarePath.length > 0
                            ? qsTr("Custom: %1").arg(firmwareUpdateDialog.selectedTargetLabel())
                            : qsTr("Browse Custom .bin...")
                        onClicked: customFirmwareFileDialog.open()
                    }

                    QQC2.Button {
                        visible: firmwareUpdateDialog.customFirmwarePath.length > 0
                        icon.name: "edit-clear"
                        text: qsTr("Clear Custom")
                        onClicked: firmwareUpdateDialog.customFirmwarePath = ""
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 220
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                ColumnLayout {
                    anchors.centerIn: parent
                    width: Math.min(parent.width - 40, 560)
                    spacing: 10
                    visible: firmwareUpdateDialog.filteredFirmwares.length === 0

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        horizontalAlignment: Text.AlignHCenter
                        wrapMode: Text.WordWrap
                        font.bold: true
                        font.pointSize: 11 * root.fontScale
                        text: deviceController.firmwareFetchBusy
                            ? qsTr("Downloading Official Firmware List from dekuNukem/duckyPad-Pro...")
                            : qsTr("No Official Firmwares Cached Yet — Click 'Fetch Official List'")
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        horizontalAlignment: Text.AlignHCenter
                        wrapMode: Text.WordWrap
                        font.pointSize: 10 * root.fontScale
                        text: qsTr("Click 'Fetch Official List' to query dekuNukem's official duckyPad-Pro repository for available firmware releases, or browse a local DPP_FW_*.bin file.")
                        color: Kirigami.Theme.textColor
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 10

                        QQC2.Button {
                            icon.name: "download"
                            text: deviceController.firmwareFetchBusy
                                ? qsTr("Fetching...")
                                : qsTr("⬇ Fetch Official Firmware List")
                            enabled: !deviceController.firmwareFetchBusy
                            highlighted: true
                            onClicked: deviceController.fetchOfficialFirmwaresFromSite()
                        }

                        QQC2.Button {
                            icon.name: "internet-services"
                            text: qsTr("Open dekuNukem Releases Page")
                            onClicked: Qt.openUrlExternally("https://github.com/dekuNukem/duckyPad-Pro/releases")
                        }

                        QQC2.Button {
                            icon.name: "document-open"
                            text: qsTr("Browse Local .bin...")
                            onClicked: customFirmwareFileDialog.open()
                        }
                    }
                }

                ListView {
                    id: fwListView
                    anchors.fill: parent
                    anchors.margins: 6
                    spacing: 6
                    clip: true
                    model: firmwareUpdateDialog.filteredFirmwares
                    boundsBehavior: Flickable.StopAtBounds
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: fwListView
                        anchors.top: fwListView.top
                        anchors.right: fwListView.right
                        anchors.bottom: fwListView.bottom
                        policy: fwListView.contentHeight > fwListView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    delegate: Rectangle {
                        id: fwRowDelegate
                        width: Math.max(220, fwListView.width - (fwListView.contentHeight > fwListView.height ? 20 : 6))
                        height: fwRowCol.implicitHeight + 20
                        implicitHeight: fwRowCol.implicitHeight + 20
                        radius: 6
                        clip: true
                        readonly property bool isSelected: firmwareUpdateDialog.customFirmwarePath.length === 0
                            && firmwareUpdateDialog.selectedFirmwareIndex === index
                        readonly property bool isCurrentInstalled: modelData.version === root.cleanFirmwareVersionLabel(deviceController.firmwareVersion)
                            && (!fwRowDelegate.isOfficialFw === root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion))
                        readonly property bool isOfficialFw: modelData.is_official !== undefined ? Boolean(modelData.is_official) : !modelData.supports_monochrome_icons
                        readonly property bool isDownloaded: modelData.is_downloaded !== undefined ? Boolean(modelData.is_downloaded) : Boolean(modelData.path && modelData.path.length > 0)
                        readonly property string dlUrl: modelData.download_url || ""
                        color: isSelected
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.22)
                            : (fwRowMouse.containsMouse
                                ? Qt.alpha(Kirigami.Theme.textColor, 0.06)
                                : Kirigami.Theme.backgroundColor)
                        border.color: isSelected
                            ? Kirigami.Theme.highlightColor
                            : (!isOfficialFw
                                ? Qt.alpha("#a855f7", 0.65)
                                : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.40))
                        border.width: isSelected ? 2 : 1

                        MouseArea {
                            id: fwRowMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            z: 0
                            onClicked: {
                                firmwareUpdateDialog.customFirmwarePath = ""
                                firmwareUpdateDialog.selectedFirmwareIndex = index
                            }
                        }

                        ColumnLayout {
                            id: fwRowCol
                            anchors.left: parent.left
                            anchors.right: parent.right
                            anchors.top: parent.top
                            anchors.margins: 10
                            spacing: 5
                            z: 2

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                Flow {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    Layout.preferredWidth: 1
                                    spacing: 6

                                    QQC2.Label {
                                        height: 20
                                        verticalAlignment: Text.AlignVCenter
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 11 * root.fontScale
                                        text: "v" + modelData.version
                                        color: Kirigami.Theme.textColor
                                    }

                                    // Prominent Origin Badge: Official dekuNukem vs DuckyDeck Studio
                                    Rectangle {
                                        radius: 4
                                        color: fwRowDelegate.isOfficialFw
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.20)
                                            : Qt.alpha("#a855f7", 0.24)
                                        border.color: fwRowDelegate.isOfficialFw
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.65)
                                            : Qt.alpha("#c084fc", 0.80)
                                        border.width: 1
                                        width: vendorBadgeRow.implicitWidth + 12
                                        height: 20
                                        implicitWidth: width
                                        implicitHeight: height

                                        RowLayout {
                                            id: vendorBadgeRow
                                            anchors.centerIn: parent
                                            spacing: 4

                                            Image {
                                                visible: !fwRowDelegate.isOfficialFw
                                                source: root.effectiveThemeMode === "light"
                                                    ? "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-light.svg"
                                                    : "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg"
                                                width: 12
                                                height: 12
                                                sourceSize.width: 24
                                                sourceSize.height: 24
                                                fillMode: Image.PreserveAspectFit
                                            }

                                            QQC2.Label {
                                                id: vendorBadgeLbl
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                text: fwRowDelegate.isOfficialFw
                                                    ? qsTr("✔ OFFICIAL DEKUNUKEM")
                                                    : qsTr("DUCKYDECK STUDIO")
                                                color: Kirigami.Theme.textColor
                                            }
                                        }
                                    }

                                    Rectangle {
                                        radius: 4
                                        color: modelData.supports_monochrome_icons
                                            ? Qt.alpha("#0ea5e9", 0.20)
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.10)
                                        border.color: modelData.supports_monochrome_icons
                                            ? Qt.alpha("#38bdf8", 0.60)
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.25)
                                        border.width: 1
                                        width: chLbl.implicitWidth + 10
                                        height: 20
                                        implicitWidth: width
                                        implicitHeight: height

                                        QQC2.Label {
                                            id: chLbl
                                            anchors.centerIn: parent
                                            font.bold: modelData.supports_monochrome_icons
                                            font.pointSize: 10 * root.fontScale
                                            text: modelData.supports_monochrome_icons
                                                ? qsTr("★ Monochrome Icons & 0°/90°/180°/270° OLED")
                                                : (modelData.channel || qsTr("Official"))
                                            color: Kirigami.Theme.textColor
                                        }
                                    }

                                    Rectangle {
                                        radius: 4
                                        color: fwRowDelegate.isDownloaded
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                                            : Qt.alpha("#f59e0b", 0.18)
                                        border.color: fwRowDelegate.isDownloaded
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                                            : Qt.alpha("#f59e0b", 0.65)
                                        border.width: 1
                                        width: dlStateLbl.implicitWidth + 10
                                        height: 20
                                        implicitWidth: width
                                        implicitHeight: height

                                        QQC2.Label {
                                            id: dlStateLbl
                                            anchors.centerIn: parent
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            text: fwRowDelegate.isDownloaded ? qsTr("READY LOCALLY") : qsTr("REMOTE (ON SITE)")
                                            color: Kirigami.Theme.textColor
                                        }
                                    }

                                    Rectangle {
                                        visible: fwRowDelegate.isCurrentInstalled
                                        radius: 4
                                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.25)
                                        border.color: Kirigami.Theme.highlightColor
                                        border.width: 1
                                        width: instLbl.implicitWidth + 10
                                        height: 20
                                        implicitWidth: width
                                        implicitHeight: height

                                        QQC2.Label {
                                            id: instLbl
                                            anchors.centerIn: parent
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            text: qsTr("CURRENTLY INSTALLED")
                                            color: Kirigami.Theme.textColor
                                        }
                                    }
                                }

                                QQC2.Button {
                                    visible: !fwRowDelegate.isDownloaded && fwRowDelegate.dlUrl.length > 0
                                    enabled: !deviceController.firmwareFetchBusy && !deviceController.sdSyncBusy
                                    Layout.alignment: Qt.AlignTop | Qt.AlignRight
                                    icon.name: "download"
                                    text: qsTr("⬇ Download .bin")
                                    implicitHeight: 26
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: {
                                        firmwareUpdateDialog.customFirmwarePath = ""
                                        firmwareUpdateDialog.selectedFirmwareIndex = index
                                        deviceController.downloadOfficialFirmwareOnly(modelData.filename, fwRowDelegate.dlUrl)
                                    }
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                wrapMode: Text.WordWrap
                                text: qsTr("%1  •  %2  •  %3 KB  •  CRC32 %4")
                                    .arg(modelData.filename || "")
                                    .arg(modelData.release_date || "-")
                                    .arg(Math.max(1, Math.round((modelData.size_bytes || 0) / 1024)))
                                    .arg(modelData.crc32 || "-")
                                color: Kirigami.Theme.textColor
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                wrapMode: Text.WordWrap
                                font.pointSize: 10 * root.fontScale
                                text: modelData.notes || ""
                                color: Kirigami.Theme.textColor
                            }
                        }
                    }
                }
            }

            RowLayout {
                id: fwFooterRow
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "view-refresh"
                    text: qsTr("Refresh & Fetch Official")
                    enabled: !deviceController.firmwareCheckBusy && !deviceController.firmwareFetchBusy && !deviceController.sdSyncBusy
                    onClicked: {
                        firmwareUpdateDialog.refreshCatalog()
                        deviceController.fetchOfficialFirmwaresFromSite()
                        deviceController.checkFirmwareUpdate(true)
                    }
                }

                QQC2.Button {
                    icon.name: "help-contents"
                    text: qsTr("Official Guide")
                    onClicked: deviceController.openFirmwareUpdatePage()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "system-software-update"
                    readonly property var selItem: firmwareUpdateDialog.selectedFirmwareItem()
                    readonly property bool needsDownloadFirst: firmwareUpdateDialog.customFirmwarePath.length === 0
                        && selItem !== null
                        && (!selItem.path || selItem.path.length === 0)
                        && (selItem.download_url && selItem.download_url.length > 0)
                    text: firmwareUpdateDialog.selectedTargetLabel().length > 0
                        ? (needsDownloadFirst
                            ? qsTr("Download & Install %1").arg(firmwareUpdateDialog.selectedTargetLabel())
                            : qsTr("Install %1 on duckyPad Pro").arg(firmwareUpdateDialog.selectedTargetLabel()))
                        : qsTr("Install Selected Firmware")
                    enabled: !deviceController.sdSyncBusy && !deviceController.firmwareFetchBusy && firmwareUpdateDialog.canInstallSelected()
                    highlighted: true
                    onClicked: {
                        const targetPath = firmwareUpdateDialog.selectedTargetPath()
                        if (targetPath.length > 0) {
                            if (deviceController.installFirmwareToDevice(targetPath)) {
                                firmwareUpdateDialog.close()
                                sdSyncDialog.open()
                            }
                            return
                        }
                        const item = firmwareUpdateDialog.selectedFirmwareItem()
                        if (item && item.download_url && item.download_url.length > 0) {
                            if (deviceController.downloadAndInstallOfficialFirmware(item.filename, item.download_url)) {
                                firmwareUpdateDialog.close()
                                sdSyncDialog.open()
                            }
                        }
                    }
                }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: firmwareUpdateDialog.close()
                }
            }
        }
    }

    Window {
        id: sdSyncDialog
        readonly property bool closableWithEscape: true
        title: qsTr("duckyPad SD Sync & Validation — DuckyDeck Studio")
        width: Math.max(
            780,
            Math.ceil(sdSyncFooterRow.implicitWidth + 64),
            Math.ceil(sdSyncTitleMetrics.width + 180),
            Math.ceil(syncBannerTitleLbl.implicitWidth + 96)
        )
        height: 460
        minimumWidth: Math.max(
            740,
            Math.ceil(sdSyncFooterRow.implicitWidth + 56),
            Math.ceil(sdSyncTitleMetrics.width + 160),
            Math.ceil(syncBannerTitleLbl.implicitWidth + 80)
        )
        minimumHeight: 380
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        onMinimumWidthChanged: {
            if (sdSyncDialog.width < sdSyncDialog.minimumWidth) {
                sdSyncDialog.width = sdSyncDialog.minimumWidth
            }
        }

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        TextMetrics {
            id: sdSyncTitleMetrics
            font.bold: true
            font.pointSize: 11 * root.fontScale
            text: sdSyncDialog.title
        }

        function open() {
            if (sdSyncDialog.width < sdSyncDialog.minimumWidth) {
                sdSyncDialog.width = sdSyncDialog.minimumWidth
            }
            sdSyncDialog.show()
            sdSyncDialog.raise()
            sdSyncDialog.requestActivate()
            Qt.callLater(function() {
                if (sdSyncDialog.width < sdSyncDialog.minimumWidth) {
                    sdSyncDialog.width = sdSyncDialog.minimumWidth
                }
            })
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: sdSyncDialog.visible
            onActivated: sdSyncDialog.close()
            onAmbiguouslyActivated: sdSyncDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12

            // Status & Validation Banner
            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: deviceController.sdSyncBusy
                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                    : (deviceController.sdSyncResultState === "success"
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                        : (deviceController.sdSyncResultState === "error"
                            ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.18)
                            : Qt.alpha(Kirigami.Theme.neutralTextColor, 0.14)))
                border.color: deviceController.sdSyncBusy
                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.55)
                    : (deviceController.sdSyncResultState === "success"
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                        : (deviceController.sdSyncResultState === "error"
                            ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.6)
                            : Qt.alpha(Kirigami.Theme.neutralTextColor, 0.45)))
                border.width: 1
                implicitHeight: syncBannerCol.implicitHeight + 20

                ColumnLayout {
                    id: syncBannerCol
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 6

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            id: syncBannerTitleLbl
                            Layout.fillWidth: true
                            font.bold: true
                            font.pointSize: 11 * root.fontScale
                            text: deviceController.sdSyncBusy
                                ? ("⟳ " + deviceController.sdSyncStep)
                                : (deviceController.sdSyncResultState === "success"
                                    ? qsTr("✓ Validated: Changes Successfully Saved to duckyPad")
                                    : (deviceController.sdSyncResultState === "error"
                                        ? qsTr("⚠ Sync Error")
                                        : qsTr("duckyPad microSD Sync Status")))
                            color: Kirigami.Theme.textColor
                        }

                        QQC2.Label {
                            visible: deviceController.sdSyncBusy || deviceController.sdSyncResultState === "success"
                            text: qsTr("%1%").arg(deviceController.sdSyncProgress)
                            font.family: "monospace"
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: deviceController.sdSyncResultState === "success"
                                ? Kirigami.Theme.positiveTextColor
                                : Kirigami.Theme.highlightColor
                        }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        font.pointSize: 10 * root.fontScale
                        text: deviceController.sdSyncBusy
                            ? qsTr("Please wait while DuckyDeck Studio writes, flushes, and validates your profiles on the duckyPad microSD card...")
                            : (deviceController.sdSyncResultSummary.length > 0
                                ? deviceController.sdSyncResultSummary
                                : deviceController.profileSyncDetail)
                        color: Kirigami.Theme.textColor
                    }
                }
            }

            // Progress Bar
            Rectangle {
                id: sdSyncProgressBarTrack
                Layout.fillWidth: true
                Layout.preferredHeight: 8
                implicitHeight: 8
                radius: 4
                color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.24)
                border.width: 1
                clip: true

                property real progressFraction: Math.max(0.0, Math.min(1.0, deviceController.sdSyncProgress / 100.0))
                Behavior on progressFraction {
                    NumberAnimation { duration: 150 }
                }

                Rectangle {
                    anchors.left: parent.left
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    width: Math.round(parent.width * parent.progressFraction)
                    radius: 4
                    color: Kirigami.Theme.highlightColor
                }
            }

            QQC2.Label {
                text: deviceController.sdSyncLog.length > 0
                    ? qsTr("STEP-BY-STEP SYNC & VALIDATION LOG")
                    : qsTr("UNSYNCED CHANGES SUMMARY (PREVIEW BEFORE SYNC)")
                font.bold: true
                font.pointSize: 10 * root.fontScale
                color: Kirigami.Theme.disabledTextColor
            }

            // Step-by-Step Log Area (or Unsynced Changes Summary when idle)
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                QQC2.ScrollView {
                    anchors.fill: parent
                    anchors.margins: 8
                    clip: true
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {}
                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                        orientation: Qt.Horizontal
                    }

                    T.TextArea {
                        id: syncLogArea
                        readOnly: true
                        selectByMouse: true
                        wrapMode: TextEdit.Wrap
                        font.family: "monospace"
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.textColor
                        selectionColor: Kirigami.Theme.highlightColor
                        selectedTextColor: Kirigami.Theme.highlightedTextColor
                        background: null
                        text: deviceController.sdSyncLog.length > 0
                            ? deviceController.sdSyncLog
                            : (root.formatUnsyncedSummaryPlainText()
                                || qsTr("• Click 'Sync to duckyPad SD' to write and validate profile changes on hardware."))
                    }
                }
            }

            // Action Buttons
            RowLayout {
                id: sdSyncFooterRow
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "document-save"
                    text: qsTr("⬆ Sync to duckyPad SD")
                    enabled: !deviceController.sdSyncBusy
                        && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                        && (deviceController.connected || deviceController.sdMountPath.length > 0)
                    highlighted: deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges
                    onClicked: {
                        root.flushPendingSlotEditsIfModified()
                        deviceController.syncProfilesToDeviceSd()
                    }
                }

                QQC2.Button {
                    icon.name: "view-list-details"
                    text: qsTr("📋 Changes Summary")
                    enabled: !deviceController.sdSyncBusy
                    onClicked: root.openUnsyncedChangesDialog()
                }

                QQC2.Button {
                    icon.name: "view-refresh"
                    text: qsTr("⬇ Reload from SD")
                    enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
                    onClicked: deviceController.reloadProfilesFromDeviceSd()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: deviceController.sdSyncResultState === "success" ? "dialog-ok-apply" : "window-close"
                    text: deviceController.sdSyncBusy
                        ? qsTr("Syncing...")
                        : (deviceController.sdSyncResultState === "success" ? qsTr("Done") : qsTr("Close"))
                    enabled: !deviceController.sdSyncBusy
                    highlighted: deviceController.sdSyncResultState === "success"
                    onClicked: sdSyncDialog.close()
                }
            }
        }
    }

    pageStack.globalToolBar.style: Kirigami.ApplicationHeaderStyle.None

    pageStack.initialPage: Kirigami.Page {
        padding: 0

        background: Rectangle {
            color: Kirigami.Theme.backgroundColor
        }

        header: Rectangle {
            Kirigami.Theme.colorSet: Kirigami.Theme.Window
            Kirigami.Theme.inherit: false
            width: parent.width
            implicitHeight: 46
            color: Kirigami.Theme.backgroundColor

            Rectangle {
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.bottom: parent.bottom
                height: 1
                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: Kirigami.Units.largeSpacing
                anchors.rightMargin: Kirigami.Units.largeSpacing
                spacing: Kirigami.Units.mediumSpacing

                QQC2.Label {
                    text: qsTr("DuckyDeck Studio v%1").arg(deviceController.appVersion)
                    font.bold: true
                    font.pointSize: 12 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                Rectangle {
                    radius: 5
                    color: Qt.tint(
                        Kirigami.Theme.alternateBackgroundColor,
                        Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.16)
                    )
                    border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                    border.width: 1
                    implicitWidth: activeProfHeaderRow.implicitWidth + 16
                    implicitHeight: 28

                    RowLayout {
                        id: activeProfHeaderRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            id: headerProfColorSwatch
                            width: 12
                            height: 12
                            radius: 6
                            color: deviceController.activeProfileBgColor.length > 0
                                ? deviceController.activeProfileBgColor
                                : "#00aaff"
                            border.width: 1
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.55)

                            MouseArea {
                                id: headerSwatchMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: root.openProfileDefaultColorPicker(
                                    deviceController.activeProfile,
                                    deviceController.activeProfileName,
                                    deviceController.activeProfileBgColor
                                )
                            }

                            QQC2.ToolTip.visible: headerSwatchMouse.containsMouse
                            QQC2.ToolTip.text: qsTr("Click to change Default Button Color (BG_COLOR) for Profile #%1").arg(deviceController.activeProfile)
                        }

                        QQC2.Label {
                            id: activeProfHeaderLabel
                            text: qsTr("Profile #%1 — %2")
                                .arg(deviceController.activeProfile)
                                .arg(deviceController.activeProfileName)
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }
                }

                Item { Layout.fillWidth: true }

                // Ko-fi Buy Me a Coffee Widget beside Hamburger Menu
                Rectangle {
                    id: headerKofiWidget
                    radius: 15
                    color: headerKofiMouse.containsMouse ? "#c81e1e" : "#d93834"
                    border.color: "#ffffff33"
                    border.width: 1
                    implicitWidth: headerKofiRow.implicitWidth + 20
                    implicitHeight: 30

                    RowLayout {
                        id: headerKofiRow
                        anchors.centerIn: parent
                        spacing: 6

                        QQC2.Label {
                            text: "☕"
                            font.pointSize: 11 * root.fontScale
                        }

                        QQC2.Label {
                            text: qsTr("Buy Me a Coffee")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: "#ffffff"
                        }
                    }

                    MouseArea {
                        id: headerKofiMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: Qt.openUrlExternally("https://ko-fi.com/russellmilliner")
                    }

                    QQC2.ToolTip.visible: headerKofiMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Buy Russell Milliner a coffee on Ko-fi (https://ko-fi.com/russellmilliner)")
                }

                QQC2.Button {
                    id: hamburgerMenuBtn
                    icon.name: "application-menu"
                    display: QQC2.AbstractButton.IconOnly
                    implicitWidth: 36
                    implicitHeight: 32
                    onClicked: hamburgerMenu.popup(hamburgerMenuBtn, 0, hamburgerMenuBtn.height)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Menu (Device Actions, Profile Groups, Import/Export, Rules, Variables, AI & Settings)")
                }
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.mediumSpacing

            UdevBanner {
                controller: deviceController
            }

            QQC2.SplitView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                orientation: Qt.Horizontal

                handle: Rectangle {
                    implicitWidth: 8
                    color: "transparent"

                    Rectangle {
                        anchors.centerIn: parent
                        width: 2
                        height: 48
                        radius: 1
                        color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    }
                }

                // Column 1: Profiles & Auto-Switcher Sidebar
                Rectangle {
                    id: profilesSidebar
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    readonly property color frameBorder: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

                    QQC2.SplitView.preferredWidth: Math.max(
                        Math.round(300 * Math.max(1.0, root.sidebarScale * 0.95)),
                        profileActionsRow.implicitWidth + Kirigami.Units.mediumSpacing * 2 + 8
                    )
                    QQC2.SplitView.minimumWidth: Math.max(
                        Math.round(280 * Math.max(1.0, root.sidebarScale * 0.92)),
                        profileActionsRow.implicitWidth + Kirigami.Units.mediumSpacing * 2 + 8
                    )
                    QQC2.SplitView.maximumWidth: 460
                    QQC2.SplitView.fillHeight: true
                    radius: Kirigami.Units.smallSpacing
                    color: Kirigami.Theme.backgroundColor
                    border.color: profilesSidebar.frameBorder
                    border.width: 1
                    clip: true

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.mediumSpacing
                        spacing: Kirigami.Units.smallSpacing

                        // PROFILES Header
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("PROFILES (%1/64)").arg(root.totalProfileCount)
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                            }
                        }

                        // Filter Search Input
                        QQC2.TextField {
                            id: profileFilterField
                            Layout.fillWidth: true
                            placeholderText: qsTr("Filter profiles or apps (drag ⠿ to reorder)...")
                            selectByMouse: true
                            font.pointSize: 10 * root.fontScale
                            onTextChanged: {
                                root.profileFilterText = text
                                root.syncSidebarProfilesFromController()
                            }
                        }

                        // Scrollable 64-Profile List with Drag-and-Drop Reordering
                        Item {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            clip: true

                            ListView {
                                id: profileSidebarList
                                anchors.fill: parent
                                clip: true
                                model: sidebarProfileModel
                                spacing: 4
                                boundsBehavior: Flickable.StopAtBounds
                                highlightFollowsCurrentItem: false

                                property int draggedProfileIndex: -1
                                property int dropTargetIndex: -1
                                readonly property bool canReorder: root.profileFilterText.trim().length === 0

                                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                    id: profilesBoxScrollBar
                                    parent: profileSidebarList
                                    anchors.top: parent.top
                                    anchors.right: parent.right
                                    anchors.bottom: parent.bottom
                                    policy: profileSidebarList.contentHeight > profileSidebarList.height
                                        ? QQC2.ScrollBar.AlwaysOn
                                        : QQC2.ScrollBar.AsNeeded
                                }

                                delegate: Item {
                                    id: profDelegate
                                    required property int index
                                    required property int profNumber
                                    required property string profName
                                    required property int slotCount
                                    required property string boundClass
                                    required property string boundIcon
                                    required property int boundRuleCount
                                    required property string boundTooltip
                                    required property int bgR
                                    required property int bgG
                                    required property int bgB

                                    readonly property bool isCurrent: deviceController.activeProfile === profNumber
                                    readonly property bool isBeingDragged: profileSidebarList.draggedProfileIndex === index
                                    readonly property bool isDropTarget: profileSidebarList.draggedProfileIndex !== -1
                                        && profileSidebarList.dropTargetIndex === index
                                        && profileSidebarList.draggedProfileIndex !== index
                                    readonly property bool isRowHovered: profRowHover.hovered
                                        || profMouse.containsMouse
                                        || profSwatchMouse.containsMouse
                                    readonly property color profColor: Qt.rgba(bgR / 255.0, bgG / 255.0, bgB / 255.0, 1.0)
                                    readonly property real rowStride: height + profileSidebarList.spacing
                                    readonly property real shiftOffsetY: {
                                        const fromIdx = profileSidebarList.draggedProfileIndex
                                        const toIdx = profileSidebarList.dropTargetIndex
                                        if (fromIdx < 0 || toIdx < 0 || fromIdx === toIdx || index === fromIdx) {
                                            return 0
                                        }
                                        if (fromIdx < toIdx && index > fromIdx && index <= toIdx) {
                                            return -rowStride
                                        }
                                        if (fromIdx > toIdx && index >= toIdx && index < fromIdx) {
                                            return rowStride
                                        }
                                        return 0
                                    }

                                    width: Math.max(
                                        100,
                                        profileSidebarList.width - (profileSidebarList.contentHeight > profileSidebarList.height ? 14 : 2)
                                    )
                                    height: Math.round(40 * root.sidebarScale)
                                    z: isBeingDragged ? 100 : 1

                                    HoverHandler {
                                        id: profRowHover
                                    }

                                    // Drop target slot outline shown in the target gap while dragging
                                    Rectangle {
                                        visible: profileSidebarList.draggedProfileIndex !== -1
                                            && profileSidebarList.dropTargetIndex === profDelegate.index
                                        anchors.fill: parent
                                        radius: 6
                                        color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.10)
                                        border.width: 1.5
                                        border.color: Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.55)
                                    }

                                    MouseArea {
                                        id: profMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                                        cursorShape: profMouse.dragging
                                            ? Qt.ClosedHandCursor
                                            : Qt.PointingHandCursor

                                        property real pressX: 0
                                        property real pressY: 0
                                        property real dragOffsetX: 0
                                        property real dragOffsetY: 0
                                        property bool dragging: false

                                        onPressed: (mouse) => {
                                            if (mouse.button === Qt.RightButton) {
                                                dragging = false
                                                dragOffsetX = 0
                                                dragOffsetY = 0
                                                root.contextMenuProfileNumber = profDelegate.profNumber
                                                root.contextMenuProfileName = profDelegate.profName
                                                root.contextMenuProfileIndex = profDelegate.index
                                                root.exportTargetProfileNumber = profDelegate.profNumber
                                                profileContextMenu.popup()
                                                return
                                            }
                                            pressX = mouse.x
                                            pressY = mouse.y
                                            dragOffsetX = 0
                                            dragOffsetY = 0
                                            dragging = false
                                        }

                                        onPositionChanged: (mouse) => {
                                            if (!pressed || (mouse.buttons & Qt.RightButton) || !profileSidebarList.canReorder) return
                                            if (!dragging && Math.abs(mouse.y - pressY) > 5) {
                                                dragging = true
                                                profileSidebarList.interactive = false
                                                profileSidebarList.draggedProfileIndex = profDelegate.index
                                                profileSidebarList.dropTargetIndex = profDelegate.index
                                            }
                                            if (dragging) {
                                                // Auto-scroll when dragging near the top or bottom of the viewport
                                                const posInView = profMouse.mapToItem(profileSidebarList, mouse.x, mouse.y)
                                                if (posInView.y < 24 && profileSidebarList.contentY > 0) {
                                                    profileSidebarList.contentY = Math.max(0, profileSidebarList.contentY - 10)
                                                } else if (posInView.y > profileSidebarList.height - 24) {
                                                    const maxScroll = Math.max(0, profileSidebarList.contentHeight - profileSidebarList.height)
                                                    profileSidebarList.contentY = Math.min(maxScroll, profileSidebarList.contentY + 10)
                                                }

                                                dragOffsetX = Math.max(-8, Math.min(8, mouse.x - pressX))
                                                dragOffsetY = mouse.y - pressY

                                                const draggedCenterY = profDelegate.y + dragOffsetY + profDelegate.height * 0.5
                                                const targetIdx = Math.max(
                                                    0,
                                                    Math.min(
                                                        Math.max(0, profileSidebarList.count - 1),
                                                        Math.floor(draggedCenterY / profDelegate.rowStride)
                                                    )
                                                )
                                                profileSidebarList.dropTargetIndex = targetIdx
                                            }
                                        }

                                        onReleased: (mouse) => {
                                            if (mouse.button === Qt.RightButton) {
                                                return
                                            }
                                            if (dragging) {
                                                const fromIdx = profileSidebarList.draggedProfileIndex
                                                const toIdx = profileSidebarList.dropTargetIndex
                                                dragging = false
                                                dragOffsetX = 0
                                                dragOffsetY = 0
                                                profileSidebarList.draggedProfileIndex = -1
                                                profileSidebarList.dropTargetIndex = -1
                                                profileSidebarList.interactive = true
                                                if (fromIdx >= 0 && toIdx >= 0 && fromIdx !== toIdx) {
                                                    const savedY = profileSidebarList.contentY
                                                    deviceController.reorderProfile(fromIdx, toIdx)
                                                    root.refreshSelectedSlotFromController()
                                                    profileSidebarList.contentY = savedY
                                                }
                                            } else {
                                                root.flushPendingSlotEditsIfModified()
                                                const savedY = profileSidebarList.contentY
                                                root.selectingProfileFromClick = true
                                                deviceController.selectProfile(profDelegate.profNumber)
                                                root.refreshSelectedSlotFromController()
                                                profileSidebarList.contentY = savedY
                                                root.selectingProfileFromClick = false
                                            }
                                        }

                                        onDoubleClicked: (mouse) => {
                                            if (mouse.button === Qt.LeftButton) {
                                                dragging = false
                                                dragOffsetX = 0
                                                dragOffsetY = 0
                                                root.openRenameProfileDialog(profDelegate.profNumber, profDelegate.profName)
                                            }
                                        }

                                        onCanceled: {
                                            dragging = false
                                            dragOffsetX = 0
                                            dragOffsetY = 0
                                            profileSidebarList.draggedProfileIndex = -1
                                            profileSidebarList.dropTargetIndex = -1
                                            profileSidebarList.interactive = true
                                        }
                                    }

                                    Rectangle {
                                        id: profCard
                                        width: parent.width
                                        height: parent.height
                                        x: profDelegate.isBeingDragged ? profMouse.dragOffsetX : 0
                                        y: profDelegate.isBeingDragged ? profMouse.dragOffsetY : profDelegate.shiftOffsetY
                                        scale: profDelegate.isBeingDragged ? 1.02 : 1.0
                                        radius: 6
                                        opacity: profDelegate.isBeingDragged ? 0.96 : 1.0
                                        color: profDelegate.isBeingDragged
                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.32))
                                            : (profDelegate.isCurrent
                                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.22))
                                                : "transparent")
                                        border.width: (profDelegate.isCurrent || profDelegate.isBeingDragged) ? 1.5 : 0
                                        border.color: (profDelegate.isCurrent || profDelegate.isBeingDragged)
                                            ? Kirigami.Theme.highlightColor
                                            : "transparent"

                                        Behavior on y {
                                            enabled: profileSidebarList.draggedProfileIndex !== -1 && !profDelegate.isBeingDragged
                                            NumberAnimation {
                                                duration: 120
                                                easing.type: Easing.OutCubic
                                            }
                                        }

                                        Behavior on scale {
                                            NumberAnimation {
                                                duration: 100
                                                easing.type: Easing.OutCubic
                                            }
                                        }

                                        RowLayout {
                                            anchors.fill: parent
                                            anchors.leftMargin: 6
                                            anchors.rightMargin: 6
                                            spacing: 6

                                            // Drag Handle Grip
                                            QQC2.Label {
                                                visible: profileSidebarList.canReorder
                                                text: "⠿"
                                                font.pointSize: 11 * root.fontScale
                                                color: profDelegate.isCurrent || profDelegate.isBeingDragged
                                                    ? Kirigami.Theme.textColor
                                                    : Kirigami.Theme.disabledTextColor
                                            }

                                            // Profile Default Button Color Swatch (Click to change BG_COLOR)
                                            Rectangle {
                                                width: 6
                                                height: Math.round(24 * root.sidebarScale)
                                                radius: 3
                                                color: profDelegate.profColor
                                                border.width: profSwatchMouse.containsMouse ? 1 : 0
                                                border.color: Kirigami.Theme.textColor

                                                MouseArea {
                                                    id: profSwatchMouse
                                                    anchors.fill: parent
                                                    anchors.margins: -3
                                                    hoverEnabled: true
                                                    cursorShape: Qt.PointingHandCursor
                                                    onClicked: {
                                                        root.openProfileDefaultColorPicker(
                                                            profDelegate.profNumber,
                                                            profDelegate.profName,
                                                            profDelegate.profColor.toString()
                                                        )
                                                    }
                                                }

                                                QQC2.ToolTip.visible: profSwatchMouse.containsMouse
                                                QQC2.ToolTip.text: qsTr("Click to change Default Button Color (BG_COLOR) for Profile #%1").arg(profDelegate.profNumber)
                                            }

                                            QQC2.Label {
                                                text: profDelegate.profNumber + ". " + profDelegate.profName
                                                font.bold: profDelegate.isCurrent || profDelegate.isBeingDragged
                                                font.pointSize: 12 * root.fontScale
                                                color: Kirigami.Theme.textColor
                                                elide: Text.ElideRight
                                                Layout.fillWidth: true
                                            }

                                            // Auto-Switch Associated Application Icon on the right side of the profile
                                            RowLayout {
                                                visible: profDelegate.boundIcon.length > 0 || profDelegate.boundClass.length > 0
                                                spacing: 3
                                                Layout.alignment: Qt.AlignVCenter

                                                Kirigami.Icon {
                                                    source: profDelegate.boundIcon.length > 0
                                                        ? profDelegate.boundIcon
                                                        : "application-x-executable"
                                                    Layout.preferredWidth: Math.round(22 * root.sidebarScale)
                                                    Layout.preferredHeight: Math.round(22 * root.sidebarScale)
                                                    isMask: false

                                                    HoverHandler {
                                                        id: appIconHover
                                                    }

                                                    QQC2.ToolTip.visible: appIconHover.hovered && profDelegate.boundTooltip.length > 0
                                                    QQC2.ToolTip.text: profDelegate.boundTooltip
                                                }

                                                QQC2.Label {
                                                    visible: profDelegate.boundRuleCount > 1
                                                    text: "+" + (profDelegate.boundRuleCount - 1)
                                                    font.pointSize: 10 * root.fontScale
                                                    font.bold: true
                                                    color: root.highContrastHighlightText
                                                }
                                            }

                                            // Fallback Key Count Badge when no Auto-Switch App is associated
                                            Rectangle {
                                                visible: profDelegate.boundIcon.length === 0 && profDelegate.boundClass.length === 0
                                                radius: 4
                                                color: Kirigami.Theme.alternateBackgroundColor
                                                border.color: profilesSidebar.frameBorder
                                                border.width: 1
                                                implicitWidth: badgeText.implicitWidth + 8
                                                implicitHeight: Math.round(20 * root.sidebarScale)

                                                QQC2.Label {
                                                    id: badgeText
                                                    anchors.centerIn: parent
                                                    text: profDelegate.slotCount + "k"
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.disabledTextColor
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Inline New Profile Creator (when toggled)
                        ColumnLayout {
                            Layout.fillWidth: true
                            visible: root.showNewProfileInput
                            spacing: 4

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.ComboBox {
                                    id: newProfileTemplateCombo
                                    Layout.fillWidth: true
                                    font.pointSize: 10 * root.fontScale
                                    implicitHeight: Math.round(24 * root.sidebarScale)
                                    property var templateIds: ["builtin_blank"]
                                    property var templateDefaultNames: ["NewProfile"]

                                    function reloadTemplates() {
                                        let ids = []
                                        let defNames = []
                                        let labels = []
                                        try {
                                            const arr = JSON.parse(deviceController.listProfileTemplatesJson())
                                            for (let i = 0; i < arr.length; i++) {
                                                ids.push(arr[i].id)
                                                defNames.push(arr[i].defaultProfileName || arr[i].name)
                                                labels.push((arr[i].builtIn ? "★ " : "📋 ") + arr[i].name)
                                            }
                                        } catch (e) {}
                                        if (ids.length === 0) {
                                            ids = ["builtin_blank"]
                                            defNames = ["NewProfile"]
                                            labels = [qsTr("★ Blank Profile")]
                                        }
                                        templateIds = ids
                                        templateDefaultNames = defNames
                                        model = labels
                                        currentIndex = 0
                                    }

                                    Component.onCompleted: reloadTemplates()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Select a template for the new profile")
                                }

                                QQC2.Button {
                                    icon.name: "bookmark-new"
                                    text: qsTr("Templates...")
                                    implicitHeight: Math.round(24 * root.sidebarScale)
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: profileTemplatesDialog.openGeneral()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Manage profile templates or save an existing profile as a template")
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.TextField {
                                    id: newProfileNameField
                                    Layout.fillWidth: true
                                    maximumLength: 16
                                    placeholderText: qsTr("Profile name (optional)...")
                                    font.pointSize: 10 * root.fontScale
                                    onAccepted: createProfBtn.clicked()
                                }

                                QQC2.Button {
                                    id: createProfBtn
                                    text: qsTr("Add")
                                    implicitHeight: Math.round(26 * root.sidebarScale)
                                    font.pointSize: 10 * root.fontScale
                                    highlighted: true
                                    onClicked: {
                                        const tIdx = Math.max(0, newProfileTemplateCombo.currentIndex)
                                        const tplId = newProfileTemplateCombo.templateIds[tIdx] || "builtin_blank"
                                        const fallbackName = newProfileTemplateCombo.templateDefaultNames[tIdx] || "NewProfile"
                                        const chosenName = newProfileNameField.text.trim().length > 0
                                            ? newProfileNameField.text.trim()
                                            : fallbackName
                                        if (deviceController.createProfileFromTemplate(tplId, chosenName)) {
                                            newProfileNameField.text = ""
                                            root.showNewProfileInput = false
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncProfilesFromController()
                                            root.refreshSelectedSlotFromController()
                                        }
                                    }
                                }
                            }
                        }

                        // Single Row: + New Profile / Import Profile / Groups ▾ Menu
                        RowLayout {
                            id: profileActionsRow
                            Layout.fillWidth: true
                            spacing: 4

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "list-add"
                                text: qsTr("New")
                                font.pointSize: 10 * root.fontScale
                                onClicked: {
                                    root.showNewProfileInput = !root.showNewProfileInput
                                    if (root.showNewProfileInput) {
                                        newProfileTemplateCombo.reloadTemplates()
                                        newProfileNameField.forceActiveFocus()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Create a new profile from blank or from a template (Right-click a profile for Templates & Export)")
                            }

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "document-import"
                                text: qsTr("Import")
                                font.pointSize: 10 * root.fontScale
                                onClicked: importProfileFileDialog.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Import a profile from .duckyprofile.json or config.txt")
                            }

                            QQC2.Button {
                                id: profileGroupsMenuBtn
                                Layout.fillWidth: true
                                icon.name: "folder"
                                text: qsTr("Groups ▾")
                                font.pointSize: 10 * root.fontScale
                                onClicked: profileGroupMenu.popup(
                                    profileGroupsMenuBtn,
                                    0,
                                    -profileGroupMenu.implicitHeight
                                )
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Manage Profile Templates and Profile Groups (.duckygroup.json)")

                                QQC2.Menu {
                                    id: profileGroupMenu
                                    title: qsTr("Templates & Profile Groups")

                                    QQC2.MenuItem {
                                        icon.name: "document-new-from-template"
                                        text: qsTr("Profile Templates Manager...")
                                        onTriggered: profileTemplatesDialog.openGeneral()
                                    }

                                    QQC2.MenuItem {
                                        icon.name: "bookmark-new"
                                        text: qsTr("Save Active Profile as Template...")
                                        onTriggered: profileTemplatesDialog.openSaveFromProfile(
                                            deviceController.activeProfile,
                                            deviceController.activeProfileName
                                        )
                                    }

                                    QQC2.MenuSeparator {}

                                    QQC2.MenuItem {
                                        icon.name: "document-new"
                                        text: qsTr("New Profile Group...")
                                        onTriggered: newProfileGroupDialog.openWithDefaults()
                                    }

                                    QQC2.MenuItem {
                                        icon.name: "document-open"
                                        text: qsTr("Load Profile Group (.duckygroup.json)...")
                                        onTriggered: loadProfileGroupFileDialog.open()
                                    }

                                    QQC2.MenuItem {
                                        icon.name: "document-save-all"
                                        text: qsTr("Save All Profiles as Group (.duckygroup.json)...")
                                        onTriggered: saveProfileGroupFileDialog.open()
                                    }
                                }
                            }
                        }

                        // Divider
                        Rectangle {
                            Layout.fillWidth: true
                            height: 1
                            color: profilesSidebar.frameBorder
                        }

                        // AUTO-SWITCHER Section
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true

                                QQC2.Label {
                                    text: qsTr("AUTO-SWITCHER")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                }

                                QQC2.Switch {
                                    checked: deviceController.autoSwitchEnabled
                                    implicitHeight: 22
                                    onToggled: {
                                        deviceController.setAutoSwitchConfig(
                                            checked,
                                            deviceController.defaultFallbackProfile
                                        )
                                    }
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Toggle automatic profile switching")
                                }
                            }

                            // Active Window Status Box
                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: activeWinCol.implicitHeight + 12
                                radius: 6
                                color: deviceController.windowPickerArmed
                                    ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25))
                                    : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                        ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(0.96, 0.62, 0.04, 0.16))
                                        : Kirigami.Theme.alternateBackgroundColor)
                                border.color: deviceController.windowPickerArmed
                                    ? Kirigami.Theme.highlightColor
                                    : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                        ? "#f59e0b"
                                        : profilesSidebar.frameBorder)
                                border.width: 1

                                ColumnLayout {
                                    id: activeWinCol
                                    anchors.fill: parent
                                    anchors.margins: 6
                                    spacing: 1

                                    QQC2.Label {
                                        text: deviceController.windowPickerArmed
                                            ? qsTr("🎯 Click target window...")
                                            : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                                ? qsTr("⏸ Paused while Key Editor is open")
                                                : qsTr("Active: %1").arg(deviceController.activeWindowClass))
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: deviceController.windowPickerArmed
                                            ? root.highContrastHighlightText
                                            : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                                ? root.highContrastWarningText
                                                : Kirigami.Theme.textColor)
                                        elide: Text.ElideRight
                                        Layout.fillWidth: true
                                    }

                                    QQC2.Label {
                                        visible: deviceController.activeWindowTitle.length > 0 || (root.hasSelectedSlot && deviceController.autoSwitchEnabled)
                                        text: (root.hasSelectedSlot && deviceController.autoSwitchEnabled)
                                            ? qsTr("Auto-Switch resumes when Key Editor closes")
                                            : deviceController.activeWindowTitle
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                        elide: Text.ElideRight
                                        Layout.fillWidth: true
                                    }
                                }
                            }

                            // Rules Window Button
                            QQC2.Button {
                                Layout.fillWidth: true
                                Layout.minimumWidth: implicitWidth
                                icon.name: "configure"
                                text: qsTr("Rules")
                                font.pointSize: 10 * root.fontScale
                                onClicked: autoSwitchDrawer.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Open Auto-Switcher Rules")
                            }
                        }
                    }
                }

                // Column 2: Interactive Hardware Canvas
                HardwareCanvas {
                    id: hardwareCanvas
                    QQC2.SplitView.preferredWidth: Math.min(requiredWidthForFullHeight, root.maxCenterColumnWidth)
                    QQC2.SplitView.minimumWidth: Math.min(440, root.maxCenterColumnWidth)
                    QQC2.SplitView.maximumWidth: root.maxCenterColumnWidth
                    QQC2.SplitView.fillWidth: !root.hasSelectedSlot
                    QQC2.SplitView.fillHeight: true
                    controller: deviceController

                    onSlotSelected: (slotNumber, slotTitle, slotColor) => {
                        root.flushPendingSlotEditsIfModified()
                        root.loadSlotFromController(slotNumber, slotTitle, slotColor)
                    }

                    onBeforeSlotMutationRequested: {
                        root.flushPendingSlotEditsIfModified()
                    }

                    onSlotContentMutated: (newSelectedSlot) => {
                        hardwareCanvas.syncSlotsFromController()
                        if (root.hasSelectedSlot) {
                            if (newSelectedSlot >= 1 && newSelectedSlot <= 58 && root.selectedSlotNumber === newSelectedSlot) {
                                root.loadSlotFromController(
                                    newSelectedSlot,
                                    hardwareCanvas.slotTitleFor(newSelectedSlot),
                                    hardwareCanvas.slotColorFor(newSelectedSlot)
                                )
                            } else {
                                root.refreshSelectedSlotFromController()
                            }
                        }
                    }

                    onRenameProfileRequested: (profileNumber, currentName) => {
                        root.openRenameProfileDialog(profileNumber, currentName)
                    }

                    onKeyColorPickerRequested: (slotNumber, currentColorHex) => {
                        root.openKeyColorPicker(slotNumber, currentColorHex)
                    }

                    onProfileDefaultColorPickerRequested: (profileNumber, profileName, currentColorHex) => {
                        root.openProfileDefaultColorPicker(profileNumber, profileName, currentColorHex)
                    }

                    onOpenLightingAnimatorRequested: (targetScope, targetSlot) => {
                        lightingAnimatorDialog.openFor(targetScope, targetSlot)
                    }

                    onOpenKeyPropertiesRequested: (slotNumber) => {
                        keyPropertiesWindow.openForSlot(slotNumber)
                    }
                }

                // Column 3: Right Inspector Card matching system View colorSet
                Rectangle {
                    id: inspectorCard
                    visible: root.hasSelectedSlot
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    readonly property bool isMechanicalKey: root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20
                    readonly property bool isRotaryEncoder: root.selectedSlotNumber >= 21 && root.selectedSlotNumber <= 26
                    readonly property bool hasRevertibleChanges: (
                        root.hasSelectedSlot && (
                            label1Input.text !== root.slotSnapshotL1 ||
                            label2Input.text !== root.slotSnapshotL2 ||
                            allowAbortCheck.checked !== root.slotSnapshotAbort ||
                            disableRepeatCheck.checked !== root.slotSnapshotNoRepeat ||
                            root.pressScriptBuffer !== root.slotSnapshotPressScript ||
                            root.releaseScriptBuffer !== root.slotSnapshotReleaseScript ||
                            String(root.selectedSlotColor) !== root.slotSnapshotColor
                        )
                    )
                    readonly property int minContentWidth: Math.max(
                        440,
                        Math.ceil(Math.max(
                            inspectorHeaderRow.implicitWidth + 16,
                            inspectorLabelsRow.implicitWidth + 16,
                            inspectorPhaseRow.implicitWidth + 12,
                            holdFlagsRow.implicitWidth + 16,
                            inspectorTabsRow.implicitWidth + 12,
                            editorHeaderRow.implicitWidth + 16,
                            inspectorFooterRow.implicitWidth + 12
                        )) + Kirigami.Units.mediumSpacing * 2 + 8
                    )
                    QQC2.SplitView.fillWidth: true
                    QQC2.SplitView.fillHeight: true
                    QQC2.SplitView.minimumWidth: minContentWidth
                    radius: Kirigami.Units.smallSpacing
                    color: Kirigami.Theme.backgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    clip: true

                    Timer {
                        id: liveSlotAutoSaveTimer
                        interval: 350
                        repeat: false
                        onTriggered: {
                            if (root.hasSelectedSlot && !root.loadingSlotData && root.compileOk) {
                                root.saveCurrentSlotToController()
                                hardwareCanvas.syncSlotsFromController()
                                root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
                            }
                        }
                    }

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.mediumSpacing
                        spacing: 6

                        // =========================================================================
                        // ZONE 1: Identity & Key Appearance Card (Header + Live OLED Labels + Color)
                        // =========================================================================
                        Rectangle {
                            Layout.fillWidth: true
                            implicitHeight: zone1Col.implicitHeight + 16
                            radius: 6
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                            border.width: 1

                            ColumnLayout {
                                id: zone1Col
                                anchors.fill: parent
                                anchors.margins: 8
                                spacing: 6

                                // Row 1A: Interactive Keycap Badge, Slot Title, Slot Type Pill, Key Color Swatch & Close
                                RowLayout {
                                    id: inspectorHeaderRow
                                    Layout.fillWidth: true
                                    spacing: 6

                                    // Interactive Mini Keycap Badge (click to pick key color on slots 1..20)
                                    Rectangle {
                                        width: Math.round(32 * root.inspectorScale)
                                        height: Math.round(32 * root.inspectorScale)
                                        radius: 6
                                        color: inspectorBadgeSwatchMouse.containsMouse && inspectorCard.isMechanicalKey
                                            ? Qt.lighter(Kirigami.Theme.backgroundColor, 1.2)
                                            : Kirigami.Theme.backgroundColor
                                        border.color: root.selectedSlotColor
                                        border.width: 2

                                        // Mini top LED bar matching physical keycap
                                        Rectangle {
                                            anchors.top: parent.top
                                            anchors.topMargin: 3
                                            anchors.horizontalCenter: parent.horizontalCenter
                                            width: parent.width - 10
                                            height: 3
                                            radius: 1.5
                                            color: root.selectedSlotColor
                                        }

                                        QQC2.Label {
                                            anchors.centerIn: parent
                                            anchors.verticalCenterOffset: 2
                                            text: root.selectedSlotNumber
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.textColor
                                        }

                                        MouseArea {
                                            id: inspectorBadgeSwatchMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            enabled: inspectorCard.isMechanicalKey
                                            cursorShape: inspectorCard.isMechanicalKey ? Qt.PointingHandCursor : Qt.ArrowCursor
                                            onClicked: root.openKeyColorPicker(root.selectedSlotNumber, root.selectedSlotColor)
                                        }

                                        QQC2.ToolTip.visible: inspectorBadgeSwatchMouse.containsMouse && inspectorCard.isMechanicalKey
                                        QQC2.ToolTip.text: qsTr("Click to change Key %1 RGB Color (%2)")
                                            .arg(root.selectedSlotNumber)
                                            .arg(root.selectedSlotColor)
                                    }

                                    ColumnLayout {
                                        spacing: 1
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 60

                                        QQC2.Label {
                                            id: inspectorTitle
                                            font.weight: Font.Bold
                                            font.pointSize: 11 * root.fontScale
                                            text: root.selectedSlotTitle
                                            color: Kirigami.Theme.textColor
                                            elide: Text.ElideRight
                                            Layout.fillWidth: true
                                        }

                                        RowLayout {
                                            Layout.fillWidth: true
                                            spacing: 5

                                            // Slot Type Badge (MECHANICAL KEY / ROTARY ENCODER / EXPANSION)
                                            Rectangle {
                                                radius: 3
                                                color: inspectorCard.isMechanicalKey
                                                    ? Qt.alpha("#38bdf8", 0.16)
                                                    : (inspectorCard.isRotaryEncoder ? Qt.alpha("#a78bfa", 0.16) : Qt.alpha("#f59e0b", 0.16))
                                                border.color: inspectorCard.isMechanicalKey
                                                    ? Qt.alpha("#38bdf8", 0.5)
                                                    : (inspectorCard.isRotaryEncoder ? Qt.alpha("#a78bfa", 0.5) : Qt.alpha("#f59e0b", 0.5))
                                                border.width: 1
                                                implicitWidth: slotTypeBadgeLbl.implicitWidth + 8
                                                implicitHeight: Math.round(15 * root.inspectorScale)

                                                QQC2.Label {
                                                    id: slotTypeBadgeLbl
                                                    anchors.centerIn: parent
                                                    text: inspectorCard.isMechanicalKey
                                                        ? qsTr("KEY %1").arg(root.selectedSlotNumber)
                                                        : (inspectorCard.isRotaryEncoder
                                                            ? qsTr("ENCODER")
                                                            : qsTr("CH %1").arg(root.selectedSlotNumber - 26))
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: inspectorCard.isMechanicalKey
                                                        ? root.highContrastHighlightText
                                                        : (inspectorCard.isRotaryEncoder
                                                            ? (root.isLightTheme ? "#6d28d9" : "#a78bfa")
                                                            : root.highContrastWarningText)
                                                }
                                            }

                                            QQC2.Label {
                                                text: qsTr("P%1: %2")
                                                    .arg(deviceController.activeProfile)
                                                    .arg(deviceController.activeProfileName)
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.disabledTextColor
                                                elide: Text.ElideRight
                                                Layout.fillWidth: true
                                            }
                                        }
                                    }

                                    // Interactive Key RGB FX Animation Pill (Slots 1..20)
                                    Rectangle {
                                        visible: inspectorCard.isMechanicalKey
                                        readonly property bool hasKeyFx: {
                                            const _cfgs = hardwareCanvas.keyAnimationConfigs
                                            return hardwareCanvas.keyHasCustomAnimation(root.selectedSlotNumber)
                                        }
                                        radius: 5
                                        color: keyFxPillMouse.containsMouse
                                            ? Qt.alpha("#c084fc", 0.25)
                                            : (hasKeyFx ? Qt.alpha("#a855f7", 0.18) : Kirigami.Theme.backgroundColor)
                                        border.color: hasKeyFx
                                            ? "#c084fc"
                                            : (keyFxPillMouse.containsMouse ? "#a855f7" : Qt.alpha(Kirigami.Theme.textColor, 0.25))
                                        border.width: 1
                                        implicitWidth: keyFxPillRow.implicitWidth + 10
                                        implicitHeight: Math.round(26 * root.inspectorScale)

                                        RowLayout {
                                            id: keyFxPillRow
                                            anchors.centerIn: parent
                                            spacing: 3

                                            QQC2.Label {
                                                text: parent.parent.hasKeyFx ? qsTr("✨ Key FX ●") : qsTr("✨ Key FX")
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: parent.parent.hasKeyFx
                                                    ? (root.isLightTheme ? "#6d28d9" : "#e9d5ff")
                                                    : Kirigami.Theme.textColor
                                            }
                                        }

                                        MouseArea {
                                            id: keyFxPillMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: lightingAnimatorDialog.openFor("key", root.selectedSlotNumber)
                                        }

                                        QQC2.ToolTip.visible: keyFxPillMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Configure gaming RGB lighting animations (Pulse, Wave, Ripple, Starlight, Spectrum, Scanner) for Key %1 or Profile #%2")
                                            .arg(root.selectedSlotNumber)
                                            .arg(deviceController.activeProfile)
                                    }

                                    // Interactive Key RGB Color Swatch Button (Slots 1..20)
                                    Rectangle {
                                        visible: inspectorCard.isMechanicalKey
                                        radius: 5
                                        color: keyColorPillMouse.containsMouse
                                            ? Qt.lighter(Kirigami.Theme.backgroundColor, 1.25)
                                            : Kirigami.Theme.backgroundColor
                                        border.color: keyColorPillMouse.containsMouse
                                            ? root.selectedSlotColor
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.25)
                                        border.width: 1
                                        implicitWidth: keyColorPillRow.implicitWidth + 12
                                        implicitHeight: Math.round(26 * root.inspectorScale)

                                        RowLayout {
                                            id: keyColorPillRow
                                            anchors.centerIn: parent
                                            spacing: 5

                                            Rectangle {
                                                width: Math.round(12 * root.inspectorScale)
                                                height: Math.round(12 * root.inspectorScale)
                                                radius: 3
                                                color: root.selectedSlotColor
                                                border.color: "#ffffff44"
                                                border.width: 1
                                            }

                                            QQC2.Label {
                                                text: String(root.selectedSlotColor).toUpperCase()
                                                font.family: "monospace"
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.textColor
                                            }
                                        }

                                        MouseArea {
                                            id: keyColorPillMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: root.openKeyColorPicker(root.selectedSlotNumber, root.selectedSlotColor)
                                        }

                                        QQC2.ToolTip.visible: keyColorPillMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Set Key %1 Color (SWCOLOR) or pick from Saved & Used Palettes").arg(root.selectedSlotNumber)
                                    }

                                    QQC2.ToolButton {
                                        text: "✕"
                                        implicitWidth: Math.round(26 * root.inspectorScale)
                                        implicitHeight: Math.round(26 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: root.clearSlotSelection()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Close Key Inspector (or click the selected key again)")
                                    }
                                }

                                Rectangle {
                                    Layout.fillWidth: true
                                    height: 1
                                    color: Qt.alpha(Kirigami.Theme.textColor, 0.08)
                                }

                                // Row 1B: OLED Key Labels (Line 1 & Line 2 with 5-char indicators + Icon Picker) + Clear Slot Action
                                RowLayout {
                                    id: inspectorLabelsRow
                                    Layout.fillWidth: true
                                    spacing: 5

                                    QQC2.Label {
                                        text: inspectorCard.isMechanicalKey ? qsTr("OLED L1:") : qsTr("OLED Label:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    RowLayout {
                                        spacing: 3
                                        QQC2.TextField {
                                            id: label1Input
                                            Layout.preferredWidth: Math.round(66 * root.inspectorScale)
                                            Layout.minimumWidth: 52
                                            implicitHeight: Math.round(25 * root.inspectorScale)
                                            maximumLength: 10
                                            placeholderText: qsTr("Top (5ch)")
                                            selectByMouse: true
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            onTextChanged: {
                                                const clamped = root.clampOledLabel(text)
                                                if (text !== clamped) {
                                                    text = clamped
                                                }
                                            }
                                            onTextEdited: liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.Label {
                                            text: qsTr("%1/5").arg(root.oledLabelCharCount(label1Input.text))
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: root.oledLabelCharCount(label1Input.text) >= 5
                                                ? Kirigami.Theme.neutralTextColor
                                                : Kirigami.Theme.disabledTextColor
                                        }
                                    }

                                    QQC2.Label {
                                        visible: inspectorCard.isMechanicalKey
                                        text: qsTr("L2:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    RowLayout {
                                        visible: inspectorCard.isMechanicalKey
                                        spacing: 3
                                        QQC2.TextField {
                                            id: label2Input
                                            Layout.preferredWidth: Math.round(66 * root.inspectorScale)
                                            Layout.minimumWidth: 52
                                            implicitHeight: Math.round(25 * root.inspectorScale)
                                            maximumLength: 10
                                            placeholderText: qsTr("Bot (5ch)")
                                            selectByMouse: true
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            onTextChanged: {
                                                const clamped = root.clampOledLabel(text)
                                                if (text !== clamped) {
                                                    text = clamped
                                                }
                                            }
                                            onTextEdited: liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.Label {
                                            text: qsTr("%1/5").arg(root.oledLabelCharCount(label2Input.text))
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: root.oledLabelCharCount(label2Input.text) >= 5
                                                ? Kirigami.Theme.neutralTextColor
                                                : Kirigami.Theme.disabledTextColor
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("★ Icon")
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        onClicked: oledIconPickerDialog.openForInspector(1)
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Pick an OLED Button Icon or Symbol for Slot %1").arg(root.selectedSlotNumber)
                                    }

                                    Item { Layout.fillWidth: true }

                                    QQC2.Button {
                                        icon.name: "edit-clear"
                                        text: qsTr("Clear")
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: root.clearSelectedSlotNow()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Clear labels, scripts, and custom RGB color on Slot %1").arg(root.selectedSlotNumber)
                                    }
                                }

                                // Row 1C: Desktop Widget Alternate Labels (longer labels up to 24 chars for Desktop OLED Widget)
                                RowLayout {
                                    id: inspectorWidgetLabelsRow
                                    Layout.fillWidth: true
                                    spacing: 5

                                    QQC2.Label {
                                        text: inspectorCard.isMechanicalKey ? qsTr("💻 Widget L1:") : qsTr("💻 Widget Label:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: (widgetLabel1Input.text.trim().length > 0 || widgetLabel2Input.text.trim().length > 0)
                                            ? root.highContrastWarningText
                                            : Kirigami.Theme.disabledTextColor
                                        QQC2.ToolTip.visible: widgetLblHover.hovered
                                        QQC2.ToolTip.text: qsTr("💻 Host App Only (Not Hardware-Portable): Longer labels (up to 24 chars) displayed only on the Desktop OLED Widget while DuckyDeck Studio is running. Physical OLED always uses the 5-char L1/L2 labels above.")
                                        HoverHandler { id: widgetLblHover }
                                    }

                                    QQC2.TextField {
                                        id: widgetLabel1Input
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 70
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        maximumLength: 24
                                        placeholderText: inspectorCard.isMechanicalKey
                                            ? qsTr("Desktop Widget L1 (up to 24ch, host widget only)")
                                            : qsTr("Desktop Widget Label (up to 24ch, host widget only)")
                                        selectByMouse: true
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        onTextEdited: liveSlotAutoSaveTimer.restart()
                                    }

                                    QQC2.Label {
                                        visible: inspectorCard.isMechanicalKey
                                        text: qsTr("L2:")
                                        font.bold: true
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.TextField {
                                        id: widgetLabel2Input
                                        visible: inspectorCard.isMechanicalKey
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 70
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        maximumLength: 24
                                        placeholderText: qsTr("Widget L2 (host widget only)")
                                        selectByMouse: true
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        onTextEdited: liveSlotAutoSaveTimer.restart()
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 2: Event Phase Tabs (On Press / On Release) & Key Hold Behavior
                        // =========================================================================
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            Item {
                                Layout.fillWidth: true
                                implicitHeight: Math.round(30 * root.inspectorScale)

                                // Bottom tab baseline
                                Rectangle {
                                    anchors.left: parent.left
                                    anchors.right: parent.right
                                    anchors.bottom: parent.bottom
                                    height: 1
                                    color: Qt.alpha(Kirigami.Theme.textColor, 0.22)
                                }

                                RowLayout {
                                    id: inspectorPhaseRow
                                    anchors.fill: parent
                                    spacing: 2

                                    // On Press Tab
                                    Rectangle {
                                        readonly property bool isActiveTab: !root.editingReleaseScript
                                        Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                        implicitWidth: onPressTabLbl.implicitWidth + 24
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        bottomLeftRadius: 0
                                        bottomRightRadius: 0
                                        color: isActiveTab
                                            ? Kirigami.Theme.backgroundColor
                                            : (onPressTabMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                                                : Kirigami.Theme.alternateBackgroundColor)
                                        border.color: isActiveTab
                                            ? Kirigami.Theme.highlightColor
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                        border.width: 1

                                        // Active tab top indicator bar
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.top: parent.top
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            height: 3
                                            topLeftRadius: 6
                                            topRightRadius: 6
                                            color: Kirigami.Theme.highlightColor
                                        }

                                        // Mask bottom border when active so tab connects to panel below
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.bottom: parent.bottom
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            anchors.leftMargin: 1
                                            anchors.rightMargin: 1
                                            height: 2
                                            color: Kirigami.Theme.backgroundColor
                                        }

                                        QQC2.Label {
                                            id: onPressTabLbl
                                            anchors.centerIn: parent
                                            text: root.pressScriptBuffer.trim().length > 0
                                                ? qsTr("⬇ On Press ●")
                                                : qsTr("⬇ On Press")
                                            font.pointSize: 10 * root.fontScale
                                            font.bold: parent.isActiveTab
                                            color: parent.isActiveTab
                                                ? root.highContrastHighlightText
                                                : Kirigami.Theme.textColor
                                        }

                                        MouseArea {
                                            id: onPressTabMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: {
                                                if (root.editingReleaseScript) {
                                                    root.releaseScriptBuffer = scriptEditor.text
                                                    root.editingReleaseScript = false
                                                    root.loadingSlotData = true
                                                    scriptEditor.text = root.pressScriptBuffer
                                                    root.showWebApiBuilder = false
                                                    root.showAppLaunchBuilder = false
                                                    root.inspectorMode = "code"
                                                    root.loadWebApiBuilderFromScript(scriptEditor.text)
                                                    root.loadAppLaunchBuilderFromScript(scriptEditor.text)
                                                    root.loadingSlotData = false
                                                    root.updateCompilerStatus()
                                                }
                                            }
                                        }

                                        QQC2.ToolTip.visible: onPressTabMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Edit script executed when pressed down")
                                    }

                                    // On Release Tab
                                    Rectangle {
                                        readonly property bool isActiveTab: root.editingReleaseScript
                                        Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                        implicitWidth: onReleaseTabLbl.implicitWidth + 24
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        bottomLeftRadius: 0
                                        bottomRightRadius: 0
                                        color: isActiveTab
                                            ? Kirigami.Theme.backgroundColor
                                            : (onReleaseTabMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                                                : Kirigami.Theme.alternateBackgroundColor)
                                        border.color: isActiveTab
                                            ? Kirigami.Theme.highlightColor
                                            : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                        border.width: 1

                                        // Active tab top indicator bar
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.top: parent.top
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            height: 3
                                            topLeftRadius: 6
                                            topRightRadius: 6
                                            color: Kirigami.Theme.highlightColor
                                        }

                                        // Mask bottom border when active
                                        Rectangle {
                                            visible: parent.isActiveTab
                                            anchors.bottom: parent.bottom
                                            anchors.left: parent.left
                                            anchors.right: parent.right
                                            anchors.leftMargin: 1
                                            anchors.rightMargin: 1
                                            height: 2
                                            color: Kirigami.Theme.backgroundColor
                                        }

                                        QQC2.Label {
                                            id: onReleaseTabLbl
                                            anchors.centerIn: parent
                                            text: root.releaseScriptBuffer.trim().length > 0
                                                ? qsTr("⬆ On Release ●")
                                                : qsTr("⬆ On Release")
                                            font.pointSize: 10 * root.fontScale
                                            font.bold: parent.isActiveTab
                                            color: parent.isActiveTab
                                                ? root.highContrastHighlightText
                                                : Kirigami.Theme.textColor
                                        }

                                        MouseArea {
                                            id: onReleaseTabMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: {
                                                if (!root.editingReleaseScript) {
                                                    root.pressScriptBuffer = scriptEditor.text
                                                    root.editingReleaseScript = true
                                                    root.loadingSlotData = true
                                                    scriptEditor.text = root.releaseScriptBuffer
                                                    root.showWebApiBuilder = false
                                                    root.showAppLaunchBuilder = false
                                                    root.inspectorMode = "code"
                                                    root.loadWebApiBuilderFromScript(scriptEditor.text)
                                                    root.loadAppLaunchBuilderFromScript(scriptEditor.text)
                                                    root.loadingSlotData = false
                                                    root.updateCompilerStatus()
                                                }
                                            }
                                        }

                                        QQC2.ToolTip.visible: onReleaseTabMouse.containsMouse
                                        QQC2.ToolTip.text: qsTr("Edit script executed when released")
                                    }

                                    Item { Layout.fillWidth: true }
                                }
                            }

                            // Key Hold Behavior Bar: Compact No Repeat on Key Held Down (Run Once) + Allow Abort
                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: holdFlagsRow.implicitHeight + 6
                                radius: 5
                                color: disableRepeatCheck.checked
                                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.12)
                                    : Kirigami.Theme.alternateBackgroundColor
                                border.color: disableRepeatCheck.checked
                                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                                    : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                border.width: 1

                                RowLayout {
                                    id: holdFlagsRow
                                    anchors.fill: parent
                                    anchors.leftMargin: 6
                                    anchors.rightMargin: 6
                                    anchors.topMargin: 3
                                    anchors.bottomMargin: 3
                                    spacing: 6

                                    QQC2.CheckBox {
                                        id: disableRepeatCheck
                                        text: qsTr("No Repeat on Held (Run Once)")
                                        font.bold: checked
                                        font.pointSize: 10 * root.fontScale
                                        onToggled: {
                                            root.updateCompilerStatus()
                                            liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Execute this key action only once per press even if the key is held down — essential for Delete or destructive actions (dr %1 / $_DONT_REPEAT = 1)").arg(root.selectedSlotNumber)
                                    }

                                    Item { Layout.fillWidth: true }

                                    QQC2.CheckBox {
                                        id: allowAbortCheck
                                        text: qsTr("Allow Abort")
                                        font.pointSize: 10 * root.fontScale
                                        onToggled: {
                                            root.updateCompilerStatus()
                                            liveSlotAutoSaveTimer.restart()
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Allow any key press to abort this script early (ab %1 / $_ALLOW_ABORT = 1)").arg(root.selectedSlotNumber)
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 3: Folder-Style Action Mode Tabs (DuckyScript IDE / Launch App / Web API) + Tools
                        // =========================================================================
                        Item {
                            Layout.fillWidth: true
                            implicitHeight: Math.round(30 * root.inspectorScale)

                            // Bottom tab baseline
                            Rectangle {
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.bottom: parent.bottom
                                height: 1
                                color: Qt.alpha(Kirigami.Theme.textColor, 0.22)
                            }

                            RowLayout {
                                id: inspectorTabsRow
                                anchors.fill: parent
                                spacing: 2

                                // Tab 1: DuckyScript IDE
                                Rectangle {
                                    id: codeModeBtn
                                    readonly property bool isActiveTab: root.inspectorMode === "code"
                                    Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                    implicitWidth: codeTabLbl.implicitWidth + 18
                                    topLeftRadius: 6
                                    topRightRadius: 6
                                    bottomLeftRadius: 0
                                    bottomRightRadius: 0
                                    color: isActiveTab
                                        ? Kirigami.Theme.backgroundColor
                                        : (codeTabMouse.containsMouse
                                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.10)
                                            : Kirigami.Theme.alternateBackgroundColor)
                                    border.color: isActiveTab
                                        ? Kirigami.Theme.highlightColor
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                    border.width: 1

                                    Rectangle {
                                        visible: parent.isActiveTab
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        height: 3
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        color: Kirigami.Theme.highlightColor
                                    }

                                    Rectangle {
                                        visible: parent.isActiveTab
                                        anchors.bottom: parent.bottom
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        anchors.leftMargin: 1
                                        anchors.rightMargin: 1
                                        height: 2
                                        color: Kirigami.Theme.backgroundColor
                                    }

                                    QQC2.Label {
                                        id: codeTabLbl
                                        anchors.centerIn: parent
                                        text: qsTr("📜 DuckyScript IDE")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: parent.isActiveTab
                                        color: parent.isActiveTab
                                            ? root.highContrastHighlightText
                                            : Kirigami.Theme.textColor
                                    }

                                    MouseArea {
                                        id: codeTabMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            root.showAppLaunchBuilder = false
                                            root.showWebApiBuilder = false
                                            root.inspectorMode = "code"
                                        }
                                    }

                                    QQC2.ToolTip.visible: codeTabMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("Full-height DuckyScript 3 editor with syntax highlighting, autocomplete & command reference")
                                }

                                // Tab 2: Launch App (Opens Launch App Configuration Window)
                                Rectangle {
                                    id: appLaunchToggleBtn
                                    readonly property bool isActiveTab: root.inspectorMode === "app_launch" || appLaunchConfigWindow.visible
                                    Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                    implicitWidth: appLaunchTabLbl.implicitWidth + 18
                                    topLeftRadius: 6
                                    topRightRadius: 6
                                    bottomLeftRadius: 0
                                    bottomRightRadius: 0
                                    color: isActiveTab
                                        ? Kirigami.Theme.backgroundColor
                                        : (appLaunchTabMouse.containsMouse
                                            ? Qt.alpha("#fb923c", 0.14)
                                            : Kirigami.Theme.alternateBackgroundColor)
                                    border.color: isActiveTab || root.appLaunchEnabled
                                        ? "#fb923c"
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                    border.width: 1

                                    Rectangle {
                                        visible: parent.isActiveTab || root.appLaunchEnabled
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        height: 3
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        color: "#fb923c"
                                    }

                                    Rectangle {
                                        visible: parent.isActiveTab
                                        anchors.bottom: parent.bottom
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        anchors.leftMargin: 1
                                        anchors.rightMargin: 1
                                        height: 2
                                        color: Kirigami.Theme.backgroundColor
                                    }

                                    QQC2.Label {
                                        id: appLaunchTabLbl
                                        anchors.centerIn: parent
                                        text: root.appLaunchEnabled
                                            ? qsTr("🚀 Launch App ●")
                                            : qsTr("🚀 Launch App")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: parent.isActiveTab || root.appLaunchEnabled
                                        color: parent.isActiveTab || root.appLaunchEnabled
                                            ? root.highContrastOrangeText
                                            : Kirigami.Theme.textColor
                                    }

                                    MouseArea {
                                        id: appLaunchTabMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            root.showWebApiBuilder = false
                                            root.showAppLaunchBuilder = false
                                            root.inspectorMode = "app_launch"
                                        }
                                    }

                                    QQC2.ToolTip.visible: appLaunchTabMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("Switch to Launch App tab")
                                }

                                // Tab 3: Web API
                                Rectangle {
                                    id: webApiToggleBtn
                                    readonly property bool isActiveTab: root.inspectorMode === "web_api" || webApiKeyConfigWindow.visible
                                    Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                                    implicitWidth: webApiTabLbl.implicitWidth + 18
                                    topLeftRadius: 6
                                    topRightRadius: 6
                                    bottomLeftRadius: 0
                                    bottomRightRadius: 0
                                    color: isActiveTab
                                        ? Kirigami.Theme.backgroundColor
                                        : (webApiTabMouse.containsMouse
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.14)
                                            : Kirigami.Theme.alternateBackgroundColor)
                                    border.color: isActiveTab || root.webApiEnabled
                                        ? Kirigami.Theme.positiveTextColor
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.18)
                                    border.width: 1

                                    Rectangle {
                                        visible: parent.isActiveTab || root.webApiEnabled
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        height: 3
                                        topLeftRadius: 6
                                        topRightRadius: 6
                                        color: Kirigami.Theme.positiveTextColor
                                    }

                                    Rectangle {
                                        visible: parent.isActiveTab
                                        anchors.bottom: parent.bottom
                                        anchors.left: parent.left
                                        anchors.right: parent.right
                                        anchors.leftMargin: 1
                                        anchors.rightMargin: 1
                                        height: 2
                                        color: Kirigami.Theme.backgroundColor
                                    }

                                    QQC2.Label {
                                        id: webApiTabLbl
                                        anchors.centerIn: parent
                                        text: root.webApiEnabled
                                            ? qsTr("🌐 Web API ●")
                                            : qsTr("🌐 Web API")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: parent.isActiveTab || root.webApiEnabled
                                        color: parent.isActiveTab || root.webApiEnabled
                                            ? root.highContrastPositiveText
                                            : Kirigami.Theme.textColor
                                    }

                                    MouseArea {
                                        id: webApiTabMouse
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            root.showAppLaunchBuilder = false
                                            root.showWebApiBuilder = false
                                            root.inspectorMode = "web_api"
                                        }
                                    }

                                    QQC2.ToolTip.visible: webApiTabMouse.containsMouse
                                    QQC2.ToolTip.text: qsTr("Switch to Web API tab")
                                }

                                Item { Layout.fillWidth: true }

                                QQC2.Button {
                                    text: qsTr("🔴 Record")
                                    implicitHeight: Math.round(24 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    leftPadding: 6
                                    rightPadding: 6
                                    onClicked: macroRecorderDialog.openRecorder()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Record live keyboard keys, mouse clicks, and mouse movements into DuckyScript 3 (with Exact Timings or As Fast As Possible)")
                                }

                                QQC2.Button {
                                    text: qsTr("✨ AI")
                                    implicitHeight: Math.round(24 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    leftPadding: 6
                                    rightPadding: 6
                                    onClicked: aiModal.open()
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Generate DuckyScript 3 macros from natural language using AI Macro Architect")
                                }
                            }
                        }

                        // Portability / Host-Dependency Status Strip for Selected Key
                        Rectangle {
                            id: slotPortabilityStrip
                            readonly property bool requiresHostApp: root.appLaunchEnabled || root.webApiEnabled
                            Layout.fillWidth: true
                            implicitHeight: Math.round(24 * root.inspectorScale)
                            radius: 4
                            color: requiresHostApp
                                ? Qt.alpha("#f59e0b", 0.14)
                                : Qt.alpha("#22c55e", 0.12)
                            border.color: requiresHostApp
                                ? Qt.alpha("#f59e0b", 0.55)
                                : Qt.alpha("#22c55e", 0.45)
                            border.width: 1

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 8
                                spacing: 6

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: slotPortabilityStrip.requiresHostApp ? root.highContrastWarningText : root.highContrastPositiveText
                                    text: slotPortabilityStrip.requiresHostApp
                                        ? qsTr("💻 Requires DuckyDeck Studio Running on Host PC — Not Hardware-Portable (uses USB HID host listener)")
                                        : qsTr("✅ Hardware-Portable DuckyScript 3 — Runs standalone from microSD on any PC without DuckyDeck Studio")
                                }
                            }

                            HoverHandler { id: slotPortabilityHover }
                            QQC2.ToolTip.visible: slotPortabilityHover.hovered
                            QQC2.ToolTip.text: slotPortabilityStrip.requiresHostApp
                                ? qsTr("This key uses Launch App or Web API, which are executed by the DuckyDeck Studio host process over USB HID. If DuckyDeck Studio is closed or the keypad is plugged into another computer, host actions will not run.")
                                : qsTr("Standard DuckyScript 3 keystrokes, delays, mouse moves, and SWC_SET LED commands compile to bytecode (.dsb) on the microSD card and run 100% standalone on the duckyPad hardware.")
                        }

                        // Compact status strip when Launch App or Web API is configured on this key
                        Rectangle {
                            visible: root.appLaunchEnabled || root.inspectorMode === "app_launch"
                            Layout.fillWidth: true
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            radius: 5
                            color: Qt.alpha("#fb923c", 0.14)
                            border.color: Qt.alpha("#fb923c", 0.55)
                            border.width: 1

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 8
                                spacing: 6

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: root.highContrastOrangeText
                                    text: root.appLaunchEnabled
                                        ? qsTr("🚀 Launch App [💻 Host App Required]: %1").arg(root.appLaunchAppName || appLaunchCommandField.text || qsTr("Configured"))
                                        : qsTr("🚀 Launch Application Configurator [💻 Host App Required]")
                                }

                                QQC2.Button {
                                    text: qsTr("Configure Launch App...")
                                    implicitHeight: Math.round(22 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    onClicked: appLaunchConfigWindow.openForCurrentSlot()
                                }
                            }
                        }

                        Rectangle {
                            visible: root.webApiEnabled || root.inspectorMode === "web_api"
                            Layout.fillWidth: true
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            radius: 5
                            color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.14)
                            border.color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                            border.width: 1

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 8
                                anchors.rightMargin: 8
                                spacing: 6

                                QQC2.Label {
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: root.highContrastPositiveText
                                    text: root.webApiEnabled
                                        ? qsTr("🌐 Web API [💻 Host App Required]: %1").arg(root.webApiSelectedDestination || webApiUrlField.text || qsTr("Configured"))
                                        : qsTr("🌐 Web API Call Configurator [💻 Host App Required]")
                                }

                                QQC2.Button {
                                    text: qsTr("Configure Web API...")
                                    implicitHeight: Math.round(22 * root.inspectorScale)
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    onClicked: webApiKeyConfigWindow.openForCurrentSlot()
                                }
                            }
                        }

                        // Collapsible DuckyScript 3 Command Reference & Snippet Browser
                        Rectangle {
                            id: commandReferenceCard
                            visible: root.showCommandReference
                            Layout.fillWidth: true
                            implicitHeight: visible ? Math.round(220 * root.inspectorScale) : 0
                            radius: Kirigami.Units.smallSpacing
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                            border.width: 1
                            clip: true

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 8
                                spacing: 6

                                // Search & Autocomplete Toggle Row
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.TextField {
                                        id: commandRefSearchField
                                        Layout.fillWidth: true
                                        implicitHeight: Math.round(26 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        selectByMouse: true
                                        placeholderText: qsTr("🔍 Filter duckyScript 3 commands, keys, $_VARS, OLED, RGB...")
                                        text: root.commandRefSearchText
                                        onTextEdited: {
                                            root.commandRefSearchText = text
                                            root.refreshCommandReferenceModel()
                                        }
                                    }

                                    QQC2.ToolButton {
                                        visible: commandRefSearchField.text.length > 0
                                        text: "✕"
                                        implicitWidth: Math.round(24 * root.inspectorScale)
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        onClicked: {
                                            commandRefSearchField.text = ""
                                            root.commandRefSearchText = ""
                                            root.refreshCommandReferenceModel()
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Clear search filter")
                                    }

                                    QQC2.CheckBox {
                                        id: autocompleteToggleCheck
                                        checked: root.autocompleteEnabled
                                        text: qsTr("⚡ Live Autocomplete")
                                        font.pointSize: 10 * root.fontScale
                                        onToggled: {
                                            root.autocompleteEnabled = checked
                                            if (!checked) {
                                                root.autocompleteVisible = false
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Show inline duckyScript 3 suggestions while typing (or press Ctrl+Space anytime)")
                                    }
                                }

                                // Category Filter Pills Row
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 4

                                    Repeater {
                                        model: root.commandRefCategories

                                        delegate: QQC2.Button {
                                            required property string modelData
                                            readonly property bool isSelectedCat: root.commandRefCategory === modelData
                                            text: modelData
                                            checkable: true
                                            checked: isSelectedCat
                                            highlighted: isSelectedCat
                                            implicitHeight: Math.round(22 * root.inspectorScale)
                                            font.pointSize: 10 * root.fontScale
                                            font.bold: isSelectedCat
                                            onClicked: {
                                                root.commandRefCategory = modelData
                                                root.refreshCommandReferenceModel()
                                            }
                                        }
                                    }

                                    Item { Layout.fillWidth: true }

                                    QQC2.Label {
                                        text: qsTr("%1 items • Click to insert").arg(commandReferenceListView.count)
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                    }
                                }

                                // Scrollable Command Reference List
                                QQC2.ScrollView {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    clip: true
                                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                        policy: commandReferenceListView.contentHeight > commandReferenceListView.height
                                            ? QQC2.ScrollBar.AlwaysOn
                                            : QQC2.ScrollBar.AsNeeded
                                    }
                                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                                        orientation: Qt.Horizontal
                                    }

                                    ListView {
                                        id: commandReferenceListView
                                        model: commandReferenceModel
                                        spacing: 3
                                        boundsBehavior: Flickable.StopAtBounds

                                        delegate: Rectangle {
                                            id: refRow
                                            required property int index
                                            required property string token
                                            required property string category
                                            required property string badge
                                            required property string syntax
                                            required property string insertText
                                            required property string summary

                                            width: commandReferenceListView.width - 6
                                            implicitHeight: Math.max(Math.round(30 * root.inspectorScale), refRowLayout.implicitHeight + 8)
                                            radius: 4
                                            color: refRowMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                                                : Qt.alpha(Kirigami.Theme.backgroundColor, 0.65)
                                            border.color: refRowMouse.containsMouse
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                                                : Qt.alpha(Kirigami.Theme.textColor, 0.08)
                                            border.width: 1

                                            RowLayout {
                                                id: refRowLayout
                                                anchors.fill: parent
                                                anchors.leftMargin: 6
                                                anchors.rightMargin: 6
                                                spacing: 8

                                                Rectangle {
                                                    radius: 3
                                                    color: Qt.alpha(root.badgeColorForType(refRow.badge), 0.2)
                                                    border.color: Qt.alpha(root.badgeColorForType(refRow.badge), 0.6)
                                                    border.width: 1
                                                    Layout.preferredWidth: Math.round(42 * root.inspectorScale)
                                                    implicitHeight: Math.round(18 * root.inspectorScale)

                                                    QQC2.Label {
                                                        anchors.centerIn: parent
                                                        text: refRow.badge
                                                        font.family: "monospace"
                                                        font.bold: true
                                                        font.pointSize: 10 * root.fontScale
                                                        color: root.badgeColorForType(refRow.badge)
                                                    }
                                                }

                                                QQC2.Label {
                                                    text: refRow.syntax
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.textColor
                                                    Layout.preferredWidth: Math.round(185 * root.inspectorScale)
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    text: refRow.summary
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.disabledTextColor
                                                    Layout.fillWidth: true
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    visible: refRowMouse.containsMouse
                                                    text: qsTr("+ Insert")
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: root.highContrastHighlightText
                                                }
                                            }

                                            MouseArea {
                                                id: refRowMouse
                                                anchors.fill: parent
                                                hoverEnabled: true
                                                cursorShape: Qt.PointingHandCursor
                                                onClicked: root.insertCommandFromReference(refRow.insertText)
                                            }

                                            QQC2.ToolTip.visible: refRowMouse.containsMouse
                                            QQC2.ToolTip.delay: 400
                                            QQC2.ToolTip.text: refRow.syntax + " — " + refRow.summary + "\n" + qsTr("Click to insert at cursor")
                                        }
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 4: Code Editor Area with Dedicated IDE Header Toolbar,
                        //         Synchronized Line-Number Gutter & Live Autocomplete Popup
                        // =========================================================================
                        Rectangle {
                            id: editorContainer
                            readonly property bool editorExpanded: root.inspectorMode === "code" || root.showScriptPreviewInBuilder
                            Layout.fillWidth: true
                            Layout.fillHeight: editorExpanded
                            Layout.preferredHeight: editorExpanded ? -1 : (editorHeaderStrip.height + 2)
                            Layout.minimumHeight: editorExpanded ? Math.round(135 * root.inspectorScale) : (editorHeaderStrip.height + 2)
                            radius: Kirigami.Units.smallSpacing
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: root.compileOk
                                ? Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                : Kirigami.Theme.negativeTextColor
                            border.width: 1
                            clip: true

                            // Dedicated IDE Header Toolbar
                            Rectangle {
                                id: editorHeaderStrip
                                anchors.top: parent.top
                                anchors.left: parent.left
                                anchors.right: parent.right
                                height: Math.round(30 * root.inspectorScale)
                                z: 5
                                color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.85)

                                Rectangle {
                                    anchors.left: parent.left
                                    anchors.right: parent.right
                                    anchors.bottom: parent.bottom
                                    height: 1
                                    color: Qt.alpha(Kirigami.Theme.textColor, 0.12)
                                }

                                RowLayout {
                                    id: editorHeaderRow
                                    anchors.fill: parent
                                    anchors.leftMargin: 8
                                    anchors.rightMargin: 6
                                    spacing: 4

                                    // Collapse / Expand toggle when inside a visual builder
                                    QQC2.ToolButton {
                                        visible: root.inspectorMode !== "code"
                                        text: root.showScriptPreviewInBuilder ? qsTr("▾ Script") : qsTr("▸ Script")
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.showScriptPreviewInBuilder = !root.showScriptPreviewInBuilder
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Show or hide the live generated DuckyScript 3 source preview while using a visual builder")
                                    }

                                    // Active script filename + line count badge
                                    Rectangle {
                                        radius: 3
                                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.38)
                                        border.width: 1
                                        implicitWidth: fileBadgeRow.implicitWidth + 10
                                        implicitHeight: Math.round(20 * root.inspectorScale)

                                        RowLayout {
                                            id: fileBadgeRow
                                            anchors.centerIn: parent
                                            spacing: 5

                                            QQC2.Label {
                                                text: root.editingReleaseScript
                                                    ? ("key" + root.selectedSlotNumber + "-release.txt")
                                                    : ("key" + root.selectedSlotNumber + ".txt")
                                                font.family: "monospace"
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: root.highContrastHighlightText
                                            }

                                            QQC2.Label {
                                                text: qsTr("• %1 ln").arg(Math.max(1, scriptEditor.lineCount))
                                                font.family: "monospace"
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.disabledTextColor
                                            }
                                        }
                                    }

                                    Item { Layout.fillWidth: true }

                                    // Quick Snippet Insertion Buttons
                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("+DELAY")
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.insertCommandFromReference("DELAY 100")
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Insert DELAY 100 (pause 100ms)")
                                    }

                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("+STRING")
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.insertCommandFromReference("STRINGLN ")
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Insert STRINGLN command")
                                    }

                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("+OLED")
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: true
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: root.insertCommandFromReference("OLED_CLEAR\nOLED_CURSOR 10 24\nOLED_PRINT Hello!\nOLED_UPDATE")
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Insert OLED screen drawing block")
                                    }

                                    QQC2.ToolButton {
                                        visible: editorContainer.editorExpanded
                                        text: qsTr("⚡")
                                        font.pointSize: 10 * root.fontScale
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        onClicked: {
                                            scriptEditor.forceActiveFocus()
                                            root.updateAutocompleteSuggestions(true)
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Trigger DuckyScript 3 Autocomplete (Ctrl+Space)")
                                    }

                                    // Integrated Command Reference Toggle
                                    QQC2.Button {
                                        text: qsTr("📖 Reference")
                                        checkable: true
                                        checked: root.showCommandReference
                                        highlighted: root.showCommandReference
                                        implicitHeight: Math.round(22 * root.inspectorScale)
                                        font.pointSize: 10 * root.fontScale
                                        font.bold: root.showCommandReference
                                        onClicked: {
                                            root.showCommandReference = !root.showCommandReference
                                            if (root.showCommandReference) {
                                                if (!editorContainer.editorExpanded) {
                                                    root.showScriptPreviewInBuilder = true
                                                }
                                                root.refreshCommandReferenceModel()
                                                commandRefSearchField.forceActiveFocus()
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Toggle interactive DuckyScript 3 Command Reference browser")
                                    }
                                }
                            }

                            Flickable {
                                id: scriptScrollView
                                objectName: "scriptEditorFlickable"
                                visible: editorContainer.editorExpanded
                                anchors.top: editorHeaderStrip.bottom
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.bottom: parent.bottom
                                anchors.margins: 2
                                clip: true
                                boundsBehavior: Flickable.StopAtBounds
                                flickableDirection: Flickable.HorizontalAndVerticalFlick

                                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                    id: scriptEditorVerticalScrollBar
                                    objectName: "highContrastScrollBar"
                                    parent: scriptScrollView
                                    anchors.top: scriptScrollView.top
                                    anchors.right: scriptScrollView.right
                                    anchors.bottom: scriptScrollView.bottom
                                    policy: QQC2.ScrollBar.AlwaysOn
                                }

                                QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                                    id: scriptEditorHorizontalScrollBar
                                    objectName: "highContrastScrollBar"
                                    parent: scriptScrollView
                                    anchors.left: scriptScrollView.left
                                    anchors.right: scriptScrollView.right
                                    anchors.bottom: scriptScrollView.bottom
                                    orientation: Qt.Horizontal
                                    policy: scriptScrollView.contentWidth > scriptScrollView.width
                                        ? QQC2.ScrollBar.AlwaysOn
                                        : QQC2.ScrollBar.AsNeeded
                                }

                                T.TextArea.flickable: T.TextArea {
                                    id: scriptEditor
                                    objectName: "scriptEditorTextArea"
                                    font.family: "monospace"
                                    font.pointSize: 11 * root.fontScale
                                    leftPadding: lineGutter.width + 10
                                    topPadding: 6
                                    bottomPadding: 6
                                    rightPadding: 16
                                    background: Rectangle { color: "transparent" }
                                    color: Kirigami.Theme.textColor
                                    selectionColor: Kirigami.Theme.highlightColor
                                    selectedTextColor: Kirigami.Theme.highlightedTextColor
                                    selectByMouse: true
                                    wrapMode: TextEdit.NoWrap
                                    onTextChanged: {
                                        if (!root.loadingSlotData) {
                                            if (root.editingReleaseScript) {
                                                root.releaseScriptBuffer = text
                                            } else {
                                                root.pressScriptBuffer = text
                                            }
                                            root.updateCompilerStatus()
                                            if (activeFocus && !root.insertingCompletion) {
                                                root.updateAutocompleteSuggestions(false)
                                            }
                                            liveSlotAutoSaveTimer.restart()
                                        }
                                    }
                                    onCursorPositionChanged: {
                                        if (root.autocompleteVisible && !root.insertingCompletion) {
                                            if (cursorPosition < root.autocompleteTokenStart || cursorPosition > root.autocompleteTokenEnd + 1) {
                                                root.autocompleteVisible = false
                                            }
                                        }
                                    }
                                    onActiveFocusChanged: {
                                        if (!activeFocus) {
                                            root.autocompleteVisible = false
                                        }
                                    }
                                    Keys.onPressed: (event) => {
                                        if ((event.modifiers & Qt.ControlModifier) && event.key === Qt.Key_Space) {
                                            root.updateAutocompleteSuggestions(true)
                                            event.accepted = true
                                            return
                                        }
                                        if (root.autocompleteVisible && autocompleteListView.count > 0) {
                                            if (event.key === Qt.Key_Down) {
                                                root.autocompleteIndex = (root.autocompleteIndex + 1) % autocompleteListView.count
                                                autocompleteListView.positionViewAtIndex(root.autocompleteIndex, ListView.Contain)
                                                event.accepted = true
                                                return
                                            }
                                            if (event.key === Qt.Key_Up) {
                                                root.autocompleteIndex = (root.autocompleteIndex - 1 + autocompleteListView.count) % autocompleteListView.count
                                                autocompleteListView.positionViewAtIndex(root.autocompleteIndex, ListView.Contain)
                                                event.accepted = true
                                                return
                                            }
                                            if (event.key === Qt.Key_Tab || event.key === Qt.Key_Return || event.key === Qt.Key_Enter) {
                                                root.applyAutocompleteSelection(root.autocompleteIndex)
                                                event.accepted = true
                                                return
                                            }
                                            if (event.key === Qt.Key_Escape) {
                                                root.autocompleteVisible = false
                                                event.accepted = true
                                                return
                                            }
                                        }
                                    }

                                    // Synchronized Line-Number Gutter
                                    Rectangle {
                                        id: lineGutter
                                        x: scriptScrollView.contentX
                                        y: 0
                                        z: 2
                                        width: Math.round(38 * root.editorScale)
                                        height: Math.max(
                                            scriptEditor.contentHeight + scriptEditor.topPadding + scriptEditor.bottomPadding,
                                            scriptScrollView.height
                                        )
                                        color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.65)

                                        Rectangle {
                                            anchors.right: parent.right
                                            anchors.top: parent.top
                                            anchors.bottom: parent.bottom
                                            width: 1
                                            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                        }

                                        Column {
                                            y: scriptEditor.topPadding
                                            width: parent.width - 6

                                            Repeater {
                                                model: Math.max(1, scriptEditor.lineCount)

                                                Text {
                                                    required property int index
                                                    readonly property bool isErrLine: !root.compileOk && root.compileErrorLine === (index + 1)
                                                    width: lineGutter.width - 7
                                                    horizontalAlignment: Text.AlignRight
                                                    text: (index + 1)
                                                    font.family: scriptEditor.font.family
                                                    font.pointSize: scriptEditor.font.pointSize
                                                    font.bold: isErrLine
                                                    color: isErrLine
                                                        ? Kirigami.Theme.negativeTextColor
                                                        : Kirigami.Theme.disabledTextColor
                                                }
                                            }
                                        }
                                    }

                                    KSyntax.SyntaxHighlighter {
                                        textEdit: scriptEditor
                                        definition: "duckyScript 3"
                                        repository: KSyntax.Repository
                                        theme: Kirigami.Theme.backgroundColor.hslLightness < 0.5
                                            ? KSyntax.Repository.defaultTheme(KSyntax.Repository.DarkTheme)
                                            : KSyntax.Repository.defaultTheme(KSyntax.Repository.LightTheme)
                                    }
                                }
                            }

                            // Floating DuckyScript 3 Autocomplete Popup
                            Rectangle {
                                id: autocompletePopup
                                visible: editorContainer.editorExpanded && root.autocompleteVisible && autocompleteListView.count > 0
                                z: 25
                                readonly property real headerOffset: editorHeaderStrip.height
                                readonly property real scrollX: scriptScrollView.contentX || 0
                                readonly property real scrollY: scriptScrollView.contentY || 0
                                readonly property real rowH: Math.round(26 * root.inspectorScale)
                                readonly property real footerH: Math.round(20 * root.inspectorScale)
                                readonly property real popupH: Math.min(autocompleteListView.count, 7) * rowH + footerH + 8
                                readonly property real cursorBottomY: headerOffset + scriptEditor.cursorRectangle.y - scrollY + scriptEditor.cursorRectangle.height + 6
                                readonly property real cursorTopY: headerOffset + scriptEditor.cursorRectangle.y - scrollY - popupH - 4

                                x: Math.max(
                                    lineGutter.width + 6,
                                    Math.min(
                                        scriptEditor.cursorRectangle.x - scrollX,
                                        editorContainer.width - width - 6
                                    )
                                )
                                y: (cursorBottomY + popupH <= editorContainer.height - 4 || cursorTopY < headerOffset + 4)
                                    ? Math.max(headerOffset + 4, Math.min(cursorBottomY, editorContainer.height - popupH - 4))
                                    : cursorTopY
                                width: Math.min(
                                    Math.max(360, Math.round(460 * root.inspectorScale)),
                                    editorContainer.width - lineGutter.width - 12
                                )
                                height: popupH
                                radius: 6
                                color: Kirigami.Theme.backgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.7)
                                border.width: 1

                                ColumnLayout {
                                    anchors.fill: parent
                                    anchors.margins: 4
                                    spacing: 2

                                    ListView {
                                        id: autocompleteListView
                                        Layout.fillWidth: true
                                        Layout.fillHeight: true
                                        clip: true
                                        model: autocompleteModel
                                        currentIndex: root.autocompleteIndex
                                        boundsBehavior: Flickable.StopAtBounds

                                        QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                            parent: autocompleteListView
                                            anchors.top: parent.top
                                            anchors.right: parent.right
                                            anchors.bottom: parent.bottom
                                            policy: autocompleteListView.contentHeight > autocompleteListView.height
                                                ? QQC2.ScrollBar.AlwaysOn
                                                : QQC2.ScrollBar.AsNeeded
                                        }

                                        delegate: Rectangle {
                                            id: acDelegate
                                            required property int index
                                            required property string token
                                            required property string badge
                                            required property string syntax
                                            required property string insertText
                                            required property string summary

                                            readonly property bool isSelected: index === root.autocompleteIndex
                                            width: autocompleteListView.width
                                            height: autocompletePopup.rowH
                                            radius: 4
                                            color: isSelected
                                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.25)
                                                : (acMouse.containsMouse
                                                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.12)
                                                    : "transparent")

                                            RowLayout {
                                                anchors.fill: parent
                                                anchors.leftMargin: 6
                                                anchors.rightMargin: 6
                                                spacing: 6

                                                Rectangle {
                                                    radius: 3
                                                    color: Qt.alpha(root.badgeColorForType(acDelegate.badge), 0.2)
                                                    border.color: Qt.alpha(root.badgeColorForType(acDelegate.badge), 0.6)
                                                    border.width: 1
                                                    Layout.preferredWidth: Math.round(38 * root.inspectorScale)
                                                    implicitHeight: Math.round(16 * root.inspectorScale)

                                                    QQC2.Label {
                                                        anchors.centerIn: parent
                                                        text: acDelegate.badge
                                                        font.family: "monospace"
                                                        font.bold: true
                                                        font.pointSize: 10 * root.fontScale
                                                        color: root.badgeColorForType(acDelegate.badge)
                                                    }
                                                }

                                                QQC2.Label {
                                                    text: acDelegate.syntax
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pointSize: 10 * root.fontScale
                                                    color: acDelegate.isSelected
                                                        ? Kirigami.Theme.highlightColor
                                                        : Kirigami.Theme.textColor
                                                    Layout.preferredWidth: Math.round(150 * root.inspectorScale)
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    text: acDelegate.summary
                                                    font.pointSize: 10 * root.fontScale
                                                    color: Kirigami.Theme.disabledTextColor
                                                    Layout.fillWidth: true
                                                    elide: Text.ElideRight
                                                }
                                            }

                                            MouseArea {
                                                id: acMouse
                                                anchors.fill: parent
                                                hoverEnabled: true
                                                cursorShape: Qt.PointingHandCursor
                                                onEntered: root.autocompleteIndex = acDelegate.index
                                                onClicked: root.applyAutocompleteSelection(acDelegate.index)
                                            }
                                        }
                                    }

                                    Rectangle {
                                        Layout.fillWidth: true
                                        height: 1
                                        color: Qt.alpha(Kirigami.Theme.textColor, 0.1)
                                    }

                                    RowLayout {
                                        Layout.fillWidth: true
                                        Layout.preferredHeight: autocompletePopup.footerH - 3
                                        spacing: 6

                                        QQC2.Label {
                                            Layout.leftMargin: 4
                                            text: qsTr("↑↓ Navigate  •  Tab / Enter Insert  •  Esc Dismiss  •  Ctrl+Space Trigger")
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideRight
                                            Layout.fillWidth: true
                                        }
                                    }
                                }
                            }
                        }

                        // =========================================================================
                        // ZONE 5: Inspector Footer Bar — Live DSVM v2 Status, Snapshot Revert & Save
                        // =========================================================================
                        RowLayout {
                            id: inspectorFooterRow
                            Layout.fillWidth: true
                            spacing: 6

                            Rectangle {
                                radius: 4
                                color: !root.compileOk
                                    ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.15)
                                    : (root.compileEmpty
                                        ? Kirigami.Theme.alternateBackgroundColor
                                        : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.15))
                                border.color: !root.compileOk
                                    ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.5)
                                    : (root.compileEmpty
                                        ? Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                        : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5))
                                border.width: 1
                                Layout.fillWidth: true
                                Layout.minimumWidth: 60
                                implicitHeight: Math.round(28 * root.inspectorScale)

                                RowLayout {
                                    anchors.fill: parent
                                    anchors.leftMargin: 8
                                    anchors.rightMargin: 8
                                    spacing: 6

                                    QQC2.Label {
                                        Layout.fillWidth: true
                                        verticalAlignment: Text.AlignVCenter
                                        elide: Text.ElideRight
                                        font.family: "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        color: !root.compileOk
                                            ? Kirigami.Theme.negativeTextColor
                                            : (root.compileEmpty
                                                ? Kirigami.Theme.disabledTextColor
                                                : Kirigami.Theme.positiveTextColor)
                                        text: {
                                            const baseName = root.editingReleaseScript
                                                ? ("key" + root.selectedSlotNumber + "-release")
                                                : ("key" + root.selectedSlotNumber)
                                            if (!root.compileOk) {
                                                return qsTr("⚠ Line %1: %2").arg(root.compileErrorLine).arg(root.compileErrorMsg)
                                            }
                                            if (root.compileEmpty) {
                                                return qsTr("%1.txt — Empty script (0 B)").arg(baseName)
                                            }
                                            return qsTr("✓ DSVM v2: %1 B (%2.dsb)").arg(root.compileByteCount).arg(baseName)
                                        }
                                    }

                                    Rectangle {
                                        visible: inspectorCard.hasRevertibleChanges
                                        radius: 3
                                        color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.18)
                                        border.color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.5)
                                        border.width: 1
                                        implicitWidth: modifiedBadgeLabel.implicitWidth + 8
                                        implicitHeight: Math.round(18 * root.inspectorScale)

                                        QQC2.Label {
                                            id: modifiedBadgeLabel
                                            anchors.centerIn: parent
                                            text: qsTr("● Edited")
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.neutralTextColor
                                        }
                                    }
                                }
                            }

                            QQC2.Button {
                                text: qsTr("↺ Revert")
                                enabled: inspectorCard.hasRevertibleChanges
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pointSize: 10 * root.fontScale
                                onClicked: {
                                    liveSlotAutoSaveTimer.stop()
                                    root.revertSlotToSnapshot()
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: inspectorCard.hasRevertibleChanges
                                    ? qsTr("Restore Key %1 labels, color, flags, and scripts to their state when this key was selected").arg(root.selectedSlotNumber)
                                    : qsTr("No changes made to Key %1 since it was selected").arg(root.selectedSlotNumber)
                            }

                            QQC2.Button {
                                text: deviceController.sdMountPath.length > 0
                                    ? qsTr("✓ Apply Key %1 (SD)").arg(root.selectedSlotNumber)
                                    : qsTr("✓ Apply Key %1").arg(root.selectedSlotNumber)
                                enabled: inspectorCard.hasRevertibleChanges
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                highlighted: inspectorCard.hasRevertibleChanges
                                onClicked: {
                                    liveSlotAutoSaveTimer.stop()
                                    root.saveCurrentSlotToController()
                                    root.captureSlotSnapshot()
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: inspectorCard.hasRevertibleChanges
                                    ? qsTr("Compile Key %1 DSVM v2 bytecode and commit this key's current state as the new baseline").arg(root.selectedSlotNumber)
                                    : qsTr("No uncommitted changes on Key %1").arg(root.selectedSlotNumber)
                            }

                            QQC2.Button {
                                text: qsTr("⬆ Sync SD")
                                enabled: !deviceController.sdSyncBusy
                                    && (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                                    && (deviceController.connected || deviceController.sdMountPath.length > 0)
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                highlighted: deviceController.profileSyncState !== "synced"
                                onClicked: {
                                    liveSlotAutoSaveTimer.stop()
                                    root.saveCurrentSlotToController()
                                    root.captureSlotSnapshot()
                                    root.openUnsyncedChangesDialog()
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: (deviceController.profileSyncState !== "synced" || inspectorCard.hasRevertibleChanges)
                                    ? qsTr("Review and push unsynced local profile and bytecode changes to the duckyPad SD card")
                                    : qsTr("All profiles and keys are already synced with the duckyPad SD card")
                            }
                        }
                    }
                }
            }
        }
    }

    // Bottom Status Bar (2-Line Layout: Line 1 = Connection, Sync & Live Status; Line 2 = Firmware Version, SD Card Space & OLED Widget Toggle)
    footer: Rectangle {
        id: bottomStatusBar
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        width: parent.width
        implicitHeight: Math.round(72 * root.statusBarScale)
        color: Kirigami.Theme.alternateBackgroundColor

        Rectangle {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            height: 1
            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.leftMargin: Kirigami.Units.largeSpacing
            anchors.rightMargin: Kirigami.Units.largeSpacing
            anchors.topMargin: 4
            anchors.bottomMargin: 4
            spacing: 4

            // Status Bar Line 1: Connection Status Pill, Sync Status Badge & Live Status Message
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                // Connection Status Pill
                Rectangle {
                    radius: 4
                    color: deviceController.connected
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                        : (deviceController.sdMountPath.length > 0
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                            : Qt.alpha(Kirigami.Theme.neutralTextColor, 0.16))
                    border.color: deviceController.connected
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                        : (deviceController.sdMountPath.length > 0
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast))
                    border.width: 1
                    implicitWidth: connRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: connRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            width: 9
                            height: 9
                            radius: 5
                            color: deviceController.connected
                                ? Kirigami.Theme.positiveTextColor
                                : (deviceController.sdMountPath.length > 0
                                    ? Kirigami.Theme.highlightColor
                                    : Kirigami.Theme.neutralTextColor)
                        }

                        QQC2.Label {
                            text: deviceController.connected
                                ? qsTr("USB HID Connected — %1").arg(deviceController.modelName)
                                : (deviceController.sdMountPath.length > 0
                                    ? qsTr("SD Card Mode — %1").arg(deviceController.modelName)
                                    : qsTr("Disconnected"))
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }
                }

                // Profile Sync / Mismatch Status Indicator Badge
                Rectangle {
                    id: syncStatusBadge
                    radius: 4
                    color: deviceController.sdSyncBusy
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                        : (deviceController.profileSyncState === "synced"
                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                            : (deviceController.profileSyncState === "unsynced"
                                ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.18)
                                : Qt.alpha(Kirigami.Theme.negativeTextColor, 0.18)))
                    border.color: deviceController.sdSyncBusy
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.6)
                        : (deviceController.profileSyncState === "synced"
                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                            : (deviceController.profileSyncState === "unsynced"
                                ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
                                : Qt.alpha(Kirigami.Theme.negativeTextColor, 0.55)))
                    border.width: 1
                    implicitWidth: syncRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: syncRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            width: 9
                            height: 9
                            radius: 5
                            color: deviceController.sdSyncBusy
                                ? Kirigami.Theme.highlightColor
                                : (deviceController.profileSyncState === "synced"
                                    ? Kirigami.Theme.positiveTextColor
                                    : (deviceController.profileSyncState === "unsynced"
                                        ? Kirigami.Theme.neutralTextColor
                                        : Kirigami.Theme.negativeTextColor))
                        }

                        QQC2.Label {
                            text: deviceController.sdSyncBusy
                                ? qsTr("⟳ Syncing (%1%)").arg(deviceController.sdSyncProgress)
                                : (deviceController.profileSyncState === "synced"
                                    ? qsTr("✓ SD Validated & Synced")
                                    : (deviceController.profileSyncState === "unsynced"
                                        ? qsTr("⚠ Unsynced Changes")
                                        : qsTr("⚠ Profile Mismatch")))
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: syncBadgeMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                        cursorShape: Qt.PointingHandCursor
                        onClicked: (mouse) => {
                            if (deviceController.sdSyncBusy) {
                                sdSyncDialog.open()
                            } else if (mouse.button === Qt.RightButton) {
                                profileSyncMenu.popup(syncStatusBadge, 0, -profileSyncMenu.implicitHeight)
                            } else {
                                root.openUnsyncedChangesDialog()
                            }
                        }
                    }

                    QQC2.ToolTip.visible: syncBadgeMouse.containsMouse
                    QQC2.ToolTip.text: deviceController.sdSyncBusy
                        ? deviceController.sdSyncStep
                        : (deviceController.profileSyncDetail + "\n" + qsTr("Click to open Unsynced Changes Summary (Right-click for Sync Menu)"))
                }

                Rectangle {
                    width: 1
                    height: Math.round(18 * root.statusBarScale)
                    color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                }

                // Live Status Message (Click to open Event Log Viewer)
                Rectangle {
                    id: statusLogLineBox
                    Layout.fillWidth: true
                    implicitHeight: Math.round(26 * root.statusBarScale)
                    radius: 4
                    color: statusLogLineMouse.containsMouse
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                        : "transparent"
                    border.color: statusLogLineMouse.containsMouse
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                        : "transparent"
                    border.width: 1

                    QQC2.Label {
                        anchors.fill: parent
                        anchors.leftMargin: 6
                        anchors.rightMargin: 6
                        verticalAlignment: Text.AlignVCenter
                        text: deviceController.statusMessage
                        font.pointSize: 11 * root.fontScale
                        color: Kirigami.Theme.textColor
                        opacity: statusLogLineMouse.containsMouse ? 1.0 : 0.92
                        elide: Text.ElideRight
                    }

                    MouseArea {
                        id: statusLogLineMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: eventLogDialog.openLogWindow()
                    }

                    QQC2.ToolTip.visible: statusLogLineMouse.containsMouse
                    QQC2.ToolTip.text: deviceController.statusMessage.length > 0
                        ? (deviceController.statusMessage + "\n" + qsTr("Click to open the Event Log Viewer"))
                        : qsTr("Click to open the Event Log Viewer")
                }
            }

            Rectangle {
                Layout.fillWidth: true
                height: 1
                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast * 0.65)
            }

            // Status Bar Line 2: Firmware Version & Serial Number, microSD Card Space Data & Desktop OLED Widget Toggle
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                // Firmware Version & Serial Number Badge (Click to check for firmware updates)
                Rectangle {
                    id: fwBadgeRect
                    radius: 4
                    readonly property bool isStudioFw: root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                    color: deviceController.firmwareUpdateAvailable
                        ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.18)
                        : (isStudioFw
                            ? Qt.alpha("#a855f7", 0.14)
                            : Kirigami.Theme.backgroundColor)
                    border.color: deviceController.firmwareUpdateAvailable
                        ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
                        : (isStudioFw
                            ? Qt.alpha("#c084fc", 0.55)
                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast))
                    border.width: 1
                    implicitWidth: fwBadgeRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: fwBadgeRow
                        anchors.centerIn: parent
                        spacing: 5

                        Image {
                            visible: deviceController.firmwareVersion !== "-" && fwBadgeRect.isStudioFw
                            source: root.effectiveThemeMode === "light"
                                ? "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-light.svg"
                                : "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg"
                            width: Math.round(14 * root.statusBarScale)
                            height: Math.round(14 * root.statusBarScale)
                            sourceSize.width: 28
                            sourceSize.height: 28
                            fillMode: Image.PreserveAspectFit
                        }

                        QQC2.Label {
                            id: fwLabel
                            text: {
                                const rawVer = deviceController.firmwareVersion
                                const fwVer = root.cleanFirmwareVersionLabel(rawVer)
                                const vendorTag = (rawVer && rawVer !== "-")
                                    ? (fwBadgeRect.isStudioFw ? " [Studio]" : " [✔ Official]")
                                    : ""
                                const base = (deviceController.serialNumber && deviceController.serialNumber !== "-")
                                    ? qsTr("FW: %1%2  •  SN: %3")
                                        .arg(fwVer)
                                        .arg(vendorTag)
                                        .arg(deviceController.serialNumber)
                                    : qsTr("Firmware: %1%2").arg(fwVer).arg(vendorTag)
                                return deviceController.firmwareUpdateAvailable
                                    ? qsTr("%1  (⬆ v%2)").arg(base).arg(deviceController.latestFirmwareVersion)
                                    : base
                            }
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: fwBadgeMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            deviceController.checkFirmwareUpdate(true)
                            firmwareUpdateDialog.open()
                        }
                    }

                    QQC2.ToolTip.visible: fwBadgeMouse.containsMouse
                    QQC2.ToolTip.text: {
                        const originDesc = root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                            ? qsTr("DuckyDeck Studio Custom Firmware v1.0.0 (Monochrome Icons & 0°/90°/180°/270° OLED)")
                            : qsTr("Official dekuNukem Firmware Release")
                        return deviceController.firmwareUpdateAvailable
                            ? qsTr("%1 — Update v%2 is available! Click to open Firmware Manager.").arg(originDesc).arg(deviceController.latestFirmwareVersion)
                            : qsTr("%1 — Click to open Firmware Manager & switch between Official dekuNukem or DuckyDeck Studio firmware").arg(originDesc)
                    }
                }

                // microSD Card Used / Free Space Badge
                Rectangle {
                    id: sdSpaceBadge
                    visible: deviceController.sdSpaceSummary.length > 0
                    radius: 4
                    color: Kirigami.Theme.backgroundColor
                    border.color: deviceController.sdSpaceUsedPercent >= 90
                        ? Qt.alpha(Kirigami.Theme.negativeTextColor, 0.55)
                        : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    implicitWidth: sdSpaceRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: sdSpaceRow
                        anchors.centerIn: parent
                        spacing: 6

                        QQC2.Label {
                            text: "💾 " + deviceController.sdSpaceSummary
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: sdSpaceMouse
                        anchors.fill: parent
                        hoverEnabled: true
                    }

                    QQC2.ToolTip.visible: sdSpaceMouse.containsMouse
                    QQC2.ToolTip.text: deviceController.sdMountPath.length > 0
                        ? (deviceController.sdSpaceDetail + "\n" + qsTr("Mount Path: %1").arg(deviceController.sdMountPath))
                        : deviceController.sdSpaceDetail
                }

                Item {
                    Layout.fillWidth: true
                }

                // Desktop OLED Widget Toggle Badge
                Rectangle {
                    id: oledWidgetStatusBtn
                    radius: 4
                    color: oledDesktopWidget.visible
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                        : Kirigami.Theme.backgroundColor
                    border.color: oledDesktopWidget.visible
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                        : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    implicitWidth: oledWidgetBtnRow.implicitWidth + 16
                    implicitHeight: Math.round(26 * root.statusBarScale)

                    RowLayout {
                        id: oledWidgetBtnRow
                        anchors.centerIn: parent
                        spacing: 6

                        Rectangle {
                            width: 9
                            height: 9
                            radius: 5
                            color: oledDesktopWidget.visible
                                ? Kirigami.Theme.positiveTextColor
                                : Kirigami.Theme.disabledTextColor
                        }

                        Image {
                            source: "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/oled-widget-icon.svg"
                            Layout.preferredWidth: Math.round(16 * root.statusBarScale)
                            Layout.preferredHeight: Math.round(16 * root.statusBarScale)
                            sourceSize.width: 32
                            sourceSize.height: 32
                            fillMode: Image.PreserveAspectFit
                        }

                        QQC2.Label {
                            text: oledDesktopWidget.visible
                                ? qsTr("Desktop OLED: Active")
                                : qsTr("Desktop OLED Widget")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: oledWidgetBtnMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: {
                            deviceController.toggleOledDesktopWidget()
                        }
                    }

                    QQC2.ToolTip.visible: oledWidgetBtnMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Toggle the floating, resizable, always-on-top KDE Desktop OLED Widget")
                }
            }
        }
    }

    FileDialog {
        id: appExecutableFileDialog
        title: qsTr("Select Application, Script, or Executable")
        nameFilters: [
            qsTr("All Files (*)"),
            qsTr("Scripts & AppImages (*.sh *.py *.AppImage *.desktop)")
        ]
        onAccepted: {
            let filePath = selectedFile.toString()
            if (filePath.indexOf("file://") === 0) {
                filePath = decodeURIComponent(filePath.substring(7))
            }
            if (filePath.length > 0) {
                const parts = filePath.split("/")
                const baseName = parts.length > 0 ? parts[parts.length - 1] : filePath
                root.appLaunchDesktopId = ""
                root.appLaunchAppName = baseName
                appLaunchCommandField.text = filePath
                appLaunchEnabledCheck.checked = true
                if (appLaunchOledField.text.trim().length === 0) {
                    appLaunchOledField.text = "Run: " + baseName.substring(0, 14)
                }
                if (label1Input.text.trim().length === 0 && label2Input.text.trim().length === 0) {
                    label1Input.text = baseName.substring(0, 5)
                }
                root.syncAppLaunchBuilderToScript()
            }
        }
    }

    Window {
        id: installedAppPickerDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: installedAppPickerDialog.visible
            onActivated: installedAppPickerDialog.close()
            onAmbiguouslyActivated: installedAppPickerDialog.close()
        }
        title: qsTr("Select Installed Application — DuckyDeck Studio")
        width: 720
        height: 540
        minimumWidth: 560
        minimumHeight: 400
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function selectDesktopApp(name, desktopId, execCmd, execArgs, terminal) {
            root.appLaunchAppName = name || ""
            root.appLaunchDesktopId = desktopId || ""
            appLaunchCommandField.text = execCmd || ""
            appLaunchArgsField.text = execArgs || ""
            appLaunchTerminalCheck.checked = !!terminal
            appLaunchEnabledCheck.checked = true
            appLaunchOledField.text = "Run: " + (name || desktopId || "App").substring(0, 14)
            if (label1Input.text.trim().length === 0 && label2Input.text.trim().length === 0 && name && name.length > 0) {
                label1Input.text = name.substring(0, 5)
            }
            root.syncAppLaunchBuilderToScript()
            installedAppPickerDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("🚀 Choose an Installed Desktop Application")
                    font.bold: true
                    font.pointSize: 11 * root.fontScale
                }

                Item { Layout.fillWidth: true }

                QQC2.Label {
                    text: qsTr("%1 applications").arg(installedAppsModel.count)
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }

                QQC2.Button {
                    icon.name: "view-refresh"
                    text: qsTr("Rescan")
                    onClicked: {
                        deviceController.refreshInstalledApps()
                        root.refreshInstalledAppsModel()
                    }
                }
            }

            QQC2.TextField {
                id: installedAppSearchInput
                Layout.fillWidth: true
                placeholderText: qsTr("Search applications by name, command, category, or description...")
                selectByMouse: true
                font.pointSize: 10 * root.fontScale
                onTextChanged: {
                    root.installedAppSearchText = text
                    root.refreshInstalledAppsModel()
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                clip: true

                QQC2.ScrollView {
                    anchors.fill: parent
                    anchors.margins: 2
                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        policy: installedAppsListView.contentHeight > installedAppsListView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }
                    QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                        orientation: Qt.Horizontal
                    }

                    ListView {
                        id: installedAppsListView
                        model: installedAppsModel
                        clip: true
                        spacing: 2

                        delegate: Rectangle {
                            required property int index
                            required property string desktopId
                            required property string name
                            required property string execCmd
                            required property string execArgs
                            required property string icon
                            required property string comment
                            required property bool terminal
                            required property string categories

                            width: installedAppsListView.width
                            height: Math.round(52 * root.fontScale)
                            radius: 4
                            color: appRowMouse.containsMouse
                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.18)
                                : (index % 2 === 0 ? "transparent" : Qt.alpha(Kirigami.Theme.textColor, 0.03))

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 10
                                anchors.rightMargin: 10
                                spacing: 10

                                Kirigami.Icon {
                                    source: icon && icon.length > 0 ? icon : "application-x-executable"
                                    fallback: "application-x-executable"
                                    Layout.preferredWidth: Math.round(28 * root.fontScale)
                                    Layout.preferredHeight: Math.round(28 * root.fontScale)
                                }

                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 2

                                    RowLayout {
                                        Layout.fillWidth: true
                                        spacing: 6

                                        QQC2.Label {
                                            text: name
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            elide: Text.ElideRight
                                        }

                                        Rectangle {
                                            visible: terminal
                                            radius: 3
                                            color: Qt.alpha(Kirigami.Theme.neutralTextColor, 0.2)
                                            implicitWidth: termBadgeLbl.implicitWidth + 8
                                            implicitHeight: termBadgeLbl.implicitHeight + 2

                                            QQC2.Label {
                                                id: termBadgeLbl
                                                anchors.centerIn: parent
                                                text: qsTr("Terminal")
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.neutralTextColor
                                            }
                                        }

                                        QQC2.Label {
                                            Layout.fillWidth: true
                                            text: desktopId
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideRight
                                        }
                                    }

                                    QQC2.Label {
                                        Layout.fillWidth: true
                                        readonly property string fullExecText: execArgs.length > 0 ? (execCmd + " " + execArgs) : execCmd
                                        text: comment.length > 0 ? (comment + "  •  " + fullExecText) : fullExecText
                                        font.family: comment.length > 0 ? Kirigami.Theme.defaultFont.family : "monospace"
                                        font.pointSize: 10 * root.fontScale
                                        color: Kirigami.Theme.disabledTextColor
                                        elide: Text.ElideRight
                                    }
                                }

                                QQC2.Button {
                                    text: qsTr("Select")
                                    implicitHeight: Math.round(26 * root.fontScale)
                                    font.pointSize: 10 * root.fontScale
                                    onClicked: installedAppPickerDialog.selectDesktopApp(name, desktopId, execCmd, execArgs, terminal)
                                }
                            }

                            MouseArea {
                                id: appRowMouse
                                anchors.fill: parent
                                anchors.rightMargin: 80
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onDoubleClicked: installedAppPickerDialog.selectDesktopApp(name, desktopId, execCmd, execArgs, terminal)
                                onClicked: installedAppPickerDialog.selectDesktopApp(name, desktopId, execCmd, execArgs, terminal)
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    text: qsTr("📁 Browse File System...")
                    onClicked: {
                        installedAppPickerDialog.close()
                        appExecutableFileDialog.open()
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Cancel")
                    onClicked: installedAppPickerDialog.close()
                }
            }
        }
    }

    // =========================================================================
    // Standalone Key Launch App Configurator Window
    // =========================================================================
    Window {
        id: appLaunchConfigWindow
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: appLaunchConfigWindow.visible
            onActivated: appLaunchConfigWindow.close()
            onAmbiguouslyActivated: appLaunchConfigWindow.close()
        }
        title: qsTr("Launch Desktop Application — Key %1 (%2)").arg(root.selectedSlotNumber).arg(root.selectedSlotTitle)
        width: 740
        height: 640
        minimumWidth: 580
        minimumHeight: 480
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function openWindow(slotNumber) {
            const targetSlot = (slotNumber !== undefined && slotNumber > 0) ? slotNumber : root.selectedSlotNumber
            if (targetSlot <= 0) return
            if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
                root.loadAppLaunchBuilderFromScript(scriptEditor.text)
            } else if (keyPropertiesWindow.visible && keyPropertiesWindow.slotNum === targetSlot) {
                root.loadAppLaunchBuilderFromScript(keyPropPressScriptArea.text)
            }
            appLaunchConfigWindow.show()
            appLaunchConfigWindow.raise()
            appLaunchConfigWindow.requestActivate()
        }

        function openForCurrentSlot() {
            openWindow(root.selectedSlotNumber)
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                    border.color: Kirigami.Theme.highlightColor
                    border.width: 1
                    QQC2.Label {
                        anchors.centerIn: parent
                        text: "🚀"
                        font.pointSize: 13 * root.fontScale
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        elide: Text.ElideRight
                        text: qsTr("Desktop Application & Command Launcher — %1").arg(root.selectedSlotTitle)
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                    }
                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        text: qsTr("Launch any installed desktop application, shell script, or command when this duckyPad key is pressed.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                    }
                }

                QQC2.Switch {
                    id: appLaunchEnabledCheck
                    text: checked ? qsTr("Enabled") : qsTr("Disabled")
                    font.bold: true
                    onToggled: root.syncAppLaunchBuilderToScript()
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: 5
                color: Qt.alpha("#f59e0b", 0.14)
                border.color: Qt.alpha("#f59e0b", 0.55)
                border.width: 1
                implicitHeight: appLaunchHostWarnLbl.implicitHeight + 10

                QQC2.Label {
                    id: appLaunchHostWarnLbl
                    anchors.fill: parent
                    anchors.margins: 5
                    wrapMode: Text.WordWrap
                    font.pointSize: 10 * root.fontScale
                    font.bold: true
                    color: root.highContrastWarningText
                    text: qsTr("💻 Requires DuckyDeck Studio Running on Host PC (Not Hardware-Portable) — Application launching is executed by DuckyDeck Studio over USB HID. If DuckyDeck Studio is not running or the duckyPad is moved to another PC, only standard DuckyScript 3 commands in this key will run.")
                }
            }

            Flickable {
                id: appLaunchFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: appLaunchScrollCol.implicitHeight + 8
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: appLaunchFlickable
                    anchors.top: appLaunchFlickable.top
                    anchors.right: appLaunchFlickable.right
                    anchors.bottom: appLaunchFlickable.bottom
                    policy: appLaunchFlickable.contentHeight > appLaunchFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: appLaunchScrollCol
                    width: Math.max(200, appLaunchFlickable.width - (appLaunchFlickable.contentHeight > appLaunchFlickable.height ? 18 : 2))
                    spacing: 10

                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: appPickCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: appPickCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Button {
                                    icon.name: "applications-all"
                                    text: qsTr("Choose Installed Application...")
                                    highlighted: true
                                    font.bold: true
                                    onClicked: {
                                        root.installedAppSearchText = ""
                                        installedAppSearchInput.text = ""
                                        root.refreshInstalledAppsModel()
                                        installedAppPickerDialog.show()
                                        installedAppPickerDialog.raise()
                                        installedAppPickerDialog.requestActivate()
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "document-open"
                                    text: qsTr("Browse Script / Executable...")
                                    onClicked: appExecutableFileDialog.open()
                                }

                                Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }

                                QQC2.Label {
                                    visible: root.appLaunchAppName.length > 0
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    Layout.preferredWidth: 1
                                    horizontalAlignment: Text.AlignRight
                                    text: qsTr("Selected: %1").arg(root.appLaunchAppName)
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    color: root.highContrastPositiveText
                                    elide: Text.ElideRight
                                }
                            }

                            GridLayout {
                                Layout.fillWidth: true
                                columns: 2
                                columnSpacing: 10
                                rowSpacing: 8

                                QQC2.Label {
                                    text: qsTr("Command / Binary:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }
                                QQC2.TextField {
                                    id: appLaunchCommandField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 100
                                    Layout.preferredWidth: 1
                                    placeholderText: qsTr("e.g. dolphin, konsole, /usr/bin/obs, ~/scripts/deploy.sh")
                                    font.family: "monospace"
                                    selectByMouse: true
                                    onTextEdited: {
                                        if (text.trim().length > 0 && !appLaunchEnabledCheck.checked) {
                                            appLaunchEnabledCheck.checked = true
                                        }
                                        root.syncAppLaunchBuilderToScript()
                                    }
                                }

                                QQC2.Label {
                                    text: qsTr("Arguments:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }
                                QQC2.TextField {
                                    id: appLaunchArgsField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 100
                                    Layout.preferredWidth: 1
                                    placeholderText: qsTr("Optional CLI arguments (e.g. --new-window ~/Projects)")
                                    font.family: "monospace"
                                    selectByMouse: true
                                    onTextEdited: root.syncAppLaunchBuilderToScript()
                                }

                                QQC2.Label {
                                    text: qsTr("Working Dir:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }
                                QQC2.TextField {
                                    id: appLaunchWorkDirField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 100
                                    Layout.preferredWidth: 1
                                    placeholderText: qsTr("Optional working directory (e.g. /home/user/workspace)")
                                    font.family: "monospace"
                                    selectByMouse: true
                                    onTextEdited: root.syncAppLaunchBuilderToScript()
                                }
                            }
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: appTrigCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: appTrigCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Trigger Mode:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.ComboBox {
                                    id: appLaunchTriggerModeCombo
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 120
                                    Layout.preferredWidth: 1
                                    model: [
                                        qsTr("USB HID Listener (Instant host launch — Zero keystroke pollution)"),
                                        qsTr("F-Key Shortcut Fallback (Emits F13–F24 + host listener)")
                                    ]
                                    onActivated: root.syncAppLaunchBuilderToScript()
                                }

                                QQC2.ComboBox {
                                    id: appLaunchFkeyCombo
                                    visible: appLaunchTriggerModeCombo.currentIndex === 1
                                    Layout.preferredWidth: 96
                                    model: ["F13","F14","F15","F16","F17","F18","F19","F20","F21","F22","F23","F24"]
                                    currentIndex: 7
                                    onActivated: root.syncAppLaunchBuilderToScript()
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 12

                                QQC2.CheckBox {
                                    id: appLaunchTerminalCheck
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    text: qsTr("Run inside Terminal Emulator (konsole)")
                                    onToggled: root.syncAppLaunchBuilderToScript()
                                }

                                QQC2.CheckBox {
                                    id: appLaunchLedCheck
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 0
                                    checked: true
                                    text: qsTr("Flash Key LED Green on Launch")
                                    onToggled: root.syncAppLaunchBuilderToScript()
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("OLED Feedback Text:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.TextField {
                                    id: appLaunchOledField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 100
                                    Layout.preferredWidth: 1
                                    maximumLength: 20
                                    placeholderText: qsTr("Optional OLED status message on press (leave blank for none)")
                                    selectByMouse: true
                                    onTextEdited: root.syncAppLaunchBuilderToScript()
                                }
                            }
                        }
                    }

                    // Quick Presets
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 6

                        QQC2.Label {
                            text: qsTr("Quick Presets:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                        }

                        QQC2.Button {
                            text: qsTr("📁 Dolphin")
                            implicitHeight: 26
                            font.pointSize: 10 * root.fontScale
                            onClicked: {
                                root.appLaunchAppName = "Dolphin"
                                root.appLaunchDesktopId = "org.kde.dolphin.desktop"
                                appLaunchCommandField.text = "dolphin"
                                appLaunchArgsField.text = ""
                                appLaunchTerminalCheck.checked = false
                                appLaunchEnabledCheck.checked = true
                                appLaunchOledField.text = "Run: Dolphin"
                                root.syncAppLaunchBuilderToScript()
                            }
                        }

                        QQC2.Button {
                            text: qsTr("🖥 Konsole")
                            implicitHeight: 26
                            font.pointSize: 10 * root.fontScale
                            onClicked: {
                                root.appLaunchAppName = "Konsole"
                                root.appLaunchDesktopId = "org.kde.konsole.desktop"
                                appLaunchCommandField.text = "konsole"
                                appLaunchArgsField.text = ""
                                appLaunchTerminalCheck.checked = false
                                appLaunchEnabledCheck.checked = true
                                appLaunchOledField.text = "Run: Konsole"
                                root.syncAppLaunchBuilderToScript()
                            }
                        }

                        QQC2.Button {
                            text: qsTr("🌐 Browser")
                            implicitHeight: 26
                            font.pointSize: 10 * root.fontScale
                            onClicked: {
                                root.appLaunchAppName = "Web Browser"
                                root.appLaunchDesktopId = ""
                                appLaunchCommandField.text = "xdg-open"
                                appLaunchArgsField.text = "https://duckypad.com"
                                appLaunchTerminalCheck.checked = false
                                appLaunchEnabledCheck.checked = true
                                appLaunchOledField.text = "Open Browser"
                                root.syncAppLaunchBuilderToScript()
                            }
                        }

                        Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "media-playback-start"
                    text: qsTr("Test Launch Now")
                    enabled: appLaunchCommandField.text.trim().length > 0
                    onClicked: {
                        deviceController.testAppLaunch(
                            root.buildAppLaunchConfigJson(),
                            root.selectedSlotNumber
                        )
                    }
                }

                QQC2.Button {
                    icon.name: "edit-clear"
                    text: qsTr("Disable / Clear")
                    onClicked: {
                        appLaunchEnabledCheck.checked = false
                        appLaunchCommandField.text = ""
                        appLaunchArgsField.text = ""
                        appLaunchWorkDirField.text = ""
                        root.appLaunchAppName = ""
                        root.appLaunchDesktopId = ""
                        root.syncAppLaunchBuilderToScript()
                    }
                }

                Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }

                QQC2.Button {
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Save & Close")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        root.syncAppLaunchBuilderToScript()
                        appLaunchConfigWindow.close()
                    }
                }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: appLaunchConfigWindow.close()
                }
            }
        }
    }

    // =========================================================================
    // Standalone Key Web API / Webhook Configurator Window
    // =========================================================================
    Window {
        id: webApiKeyConfigWindow
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: webApiKeyConfigWindow.visible
            onActivated: webApiKeyConfigWindow.close()
            onAmbiguouslyActivated: webApiKeyConfigWindow.close()
        }
        title: qsTr("Web API / HTTP Webhook — Key %1 (%2)").arg(root.selectedSlotNumber).arg(root.selectedSlotTitle)
        width: 820
        height: 760
        minimumWidth: 580
        minimumHeight: 500
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property var haEntitiesModel: []
        property string haStatusText: ""
        property bool haShowToken: false

        function loadHomeAssistantSavedConfig() {
            try {
                const cfg = JSON.parse(deviceController.getHomeAssistantConfigJson())
                if (cfg) {
                    haBaseUrlField.text = cfg.base_url || cfg.baseUrl || "http://homeassistant.local:8123"
                    haTokenField.text = cfg.access_token || cfg.accessToken || ""
                }
            } catch (e) {
                if (haBaseUrlField.text.trim().length === 0) {
                    haBaseUrlField.text = "http://homeassistant.local:8123"
                }
            }
        }

        function searchHomeAssistantEntities() {
            const baseUrl = haBaseUrlField.text.trim()
            const token = haTokenField.text.trim()
            if (baseUrl.length === 0 || token.length === 0) {
                webApiKeyConfigWindow.haStatusText = qsTr("⚠ Enter your Home Assistant URL and Long-Lived Access Token first.")
                return
            }
            const domainVal = haDomainCombo.currentIndex <= 0 ? "" : haDomainCombo.currentText
            const raw = deviceController.searchHomeAssistantEntitiesJson(
                baseUrl,
                token,
                haSearchQueryField.text.trim(),
                domainVal
            )
            try {
                const parsed = JSON.parse(raw)
                const entitiesList = Array.isArray(parsed)
                    ? parsed
                    : (parsed && Array.isArray(parsed.entities) ? parsed.entities : null)
                if (parsed && parsed.ok === false) {
                    webApiKeyConfigWindow.haEntitiesModel = []
                    webApiKeyConfigWindow.haStatusText = qsTr("✗ %1").arg(
                        String(parsed.error || deviceController.statusMessage || "Home Assistant request failed.")
                    )
                } else if (Array.isArray(entitiesList)) {
                    webApiKeyConfigWindow.haEntitiesModel = entitiesList
                    webApiKeyConfigWindow.haStatusText = entitiesList.length > 0
                        ? qsTr("✓ Found %1 matching Home Assistant entities. Click an action below to bind it to Key %2.")
                            .arg(entitiesList.length)
                            .arg(root.selectedSlotNumber)
                        : qsTr("No matching entities found for that filter/query.")
                } else {
                    webApiKeyConfigWindow.haEntitiesModel = []
                    webApiKeyConfigWindow.haStatusText = String(
                        deviceController.statusMessage || qsTr("Unexpected Home Assistant response.")
                    )
                }
            } catch (e) {
                webApiKeyConfigWindow.haEntitiesModel = []
                webApiKeyConfigWindow.haStatusText = qsTr("Failed to parse Home Assistant response: %1").arg(String(e))
            }
        }

        function applyHomeAssistantEntityToKey(entityObj, serviceOverride) {
            const entityId = entityObj ? String(entityObj.entity_id || entityObj.entityId || "") : ""
            if (!entityId) return
            const cleanBase = haBaseUrlField.text.trim().replace(/\/+$/, "")
            const token = haTokenField.text.trim()
            const domain = String(entityObj.domain || entityId.split(".")[0] || "homeassistant")
            let rawService = (serviceOverride && serviceOverride.length > 0)
                ? String(serviceOverride)
                : String(entityObj.suggested_service || entityObj.suggestedService || "toggle")
            if (rawService.indexOf("/") >= 0) {
                rawService = rawService.split("/").pop()
            } else if (rawService.indexOf(".") >= 0) {
                rawService = rawService.split(".").pop()
            }
            const service = rawService || "toggle"

            root.syncingWebApiScript = true
            webApiEnabledCheck.checked = true
            const postIdx = root.webApiMethods.indexOf("POST")
            webApiMethodCombo.currentIndex = postIdx >= 0 ? postIdx : 1
            webApiUrlField.text = cleanBase + "/api/services/" + domain + "/" + service
            webApiHeadersField.text = "Authorization: Bearer " + token + "\nContent-Type: application/json"
            webApiBodyField.text = JSON.stringify({ entity_id: entityId })
            const friendlyRaw = String(entityObj.friendly_name || entityObj.friendlyName || "")
            const friendly = friendlyRaw.length > 0 ? friendlyRaw : entityId
            webApiOledField.text = ("HA:" + friendly).slice(0, 20)
            root.syncingWebApiScript = false
            root.syncWebApiBuilderToScript()
            webApiKeyConfigWindow.haStatusText = qsTr("✓ Configured Key %1 to call %2.%3 on '%4' (%5)")
                .arg(root.selectedSlotNumber)
                .arg(domain)
                .arg(service)
                .arg(friendly)
                .arg(entityId)
        }

        function openWindow(slotNumber) {
            const targetSlot = (slotNumber !== undefined && slotNumber > 0) ? slotNumber : root.selectedSlotNumber
            if (targetSlot <= 0) return
            webApiKeyConfigWindow.loadHomeAssistantSavedConfig()
            if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
                root.loadWebApiBuilderFromScript(scriptEditor.text)
            } else if (keyPropertiesWindow.visible && keyPropertiesWindow.slotNum === targetSlot) {
                root.loadWebApiBuilderFromScript(keyPropPressScriptArea.text)
            }
            webApiKeyConfigWindow.show()
            webApiKeyConfigWindow.raise()
            webApiKeyConfigWindow.requestActivate()
        }

        function openForCurrentSlot() {
            openWindow(root.selectedSlotNumber)
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                    border.color: Kirigami.Theme.highlightColor
                    border.width: 1
                    QQC2.Label {
                        anchors.centerIn: parent
                        text: "🌐"
                        font.pointSize: 13 * root.fontScale
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        elide: Text.ElideRight
                        text: qsTr("Web API & Outbound HTTP Webhook — %1").arg(root.selectedSlotTitle)
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                    }
                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        text: qsTr("Trigger Home Assistant devices/entities or send HTTP REST/Webhook requests when this key is pressed.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                    }
                }

                QQC2.Button {
                    icon.name: "network-server"
                    text: qsTr("Manage Destinations...")
                    onClicked: {
                        webApiDestinationsDialog.openForDestination(
                            root.webApiSelectedDestination,
                            webApiMethodCombo.currentText,
                            webApiUrlField.text,
                            webApiHeadersField.text
                        )
                    }
                }

                QQC2.Switch {
                    id: webApiEnabledCheck
                    text: checked ? qsTr("Enabled") : qsTr("Disabled")
                    font.bold: true
                    onToggled: root.syncWebApiBuilderToScript()
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: 5
                color: Qt.alpha("#f59e0b", 0.14)
                border.color: Qt.alpha("#f59e0b", 0.55)
                border.width: 1
                implicitHeight: webApiHostWarnLbl.implicitHeight + 10

                QQC2.Label {
                    id: webApiHostWarnLbl
                    anchors.fill: parent
                    anchors.margins: 5
                    wrapMode: Text.WordWrap
                    font.pointSize: 10 * root.fontScale
                    font.bold: true
                    color: root.highContrastWarningText
                    text: qsTr("💻 Requires DuckyDeck Studio Running on Host PC (Not Hardware-Portable) — Outbound HTTP REST / Webhook calls are dispatched by DuckyDeck Studio over USB HID. The duckyPad hardware has no network stack and cannot send HTTP requests when DuckyDeck Studio is closed.")
                }
            }

            Flickable {
                id: webApiFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: webApiScrollCol.implicitHeight + 12
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: webApiFlickable
                    anchors.top: webApiFlickable.top
                    anchors.right: webApiFlickable.right
                    anchors.bottom: webApiFlickable.bottom
                    policy: webApiFlickable.contentHeight > webApiFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: webApiScrollCol
                    width: Math.max(200, webApiFlickable.width - (webApiFlickable.contentHeight > webApiFlickable.height ? 18 : 2))
                    spacing: 10

                    // Home Assistant Web API Device & Entity Search Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: haCardCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha("#38bdf8", 0.45)
                        border.width: 1

                        ColumnLayout {
                            id: haCardCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("🏠 Home Assistant Web API — Search Devices & Entities to Trigger")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                }

                                QQC2.Button {
                                    icon.name: "document-save"
                                    text: qsTr("Save HA Connection")
                                    onClicked: {
                                        if (deviceController.saveHomeAssistantConfig(haBaseUrlField.text.trim(), haTokenField.text.trim())) {
                                            webApiKeyConfigWindow.haStatusText = qsTr("✓ Saved Home Assistant URL and Long-Lived Access Token.")
                                        }
                                    }
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Connect to your Home Assistant instance with a Long-Lived Access Token (Profile → Security → Long-Lived Access Tokens) to search and bind entities directly to this key.")
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("HA URL:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 70
                                }

                                QQC2.TextField {
                                    id: haBaseUrlField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 140
                                    Layout.preferredWidth: 1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    text: "http://homeassistant.local:8123"
                                    placeholderText: qsTr("http://homeassistant.local:8123")
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("HA Token:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 70
                                }

                                QQC2.TextField {
                                    id: haTokenField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 140
                                    Layout.preferredWidth: 1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    echoMode: webApiKeyConfigWindow.haShowToken ? TextInput.Normal : TextInput.Password
                                    placeholderText: qsTr("Paste Home Assistant Long-Lived Access Token...")
                                }

                                QQC2.Button {
                                    text: webApiKeyConfigWindow.haShowToken ? qsTr("Hide") : qsTr("Show")
                                    onClicked: webApiKeyConfigWindow.haShowToken = !webApiKeyConfigWindow.haShowToken
                                }
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.ComboBox {
                                    id: haDomainCombo
                                    width: 150
                                    model: [
                                        "All Domains",
                                        "light",
                                        "switch",
                                        "scene",
                                        "script",
                                        "automation",
                                        "button",
                                        "input_boolean",
                                        "media_player",
                                        "cover",
                                        "fan",
                                        "climate",
                                        "lock"
                                    ]
                                    onActivated: {
                                        if (haTokenField.text.trim().length > 0) {
                                            webApiKeyConfigWindow.searchHomeAssistantEntities()
                                        }
                                    }
                                }

                                QQC2.TextField {
                                    id: haSearchQueryField
                                    width: Math.max(200, haCardCol.width - 340)
                                    selectByMouse: true
                                    placeholderText: qsTr("Search devices/entities (e.g. desk light, studio, scene)...")
                                    onAccepted: webApiKeyConfigWindow.searchHomeAssistantEntities()
                                }

                                QQC2.Button {
                                    icon.name: "edit-find"
                                    text: qsTr("🔍 Search Entities")
                                    highlighted: true
                                    onClicked: webApiKeyConfigWindow.searchHomeAssistantEntities()
                                }
                            }

                            QQC2.Label {
                                visible: webApiKeyConfigWindow.haStatusText.length > 0
                                text: webApiKeyConfigWindow.haStatusText
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: webApiKeyConfigWindow.haStatusText.startsWith("✓")
                                    ? Kirigami.Theme.positiveTextColor
                                    : Kirigami.Theme.neutralTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            Rectangle {
                                visible: webApiKeyConfigWindow.haEntitiesModel.length > 0
                                Layout.fillWidth: true
                                implicitHeight: Math.min(220, haEntityRepeaterCol.implicitHeight + 12)
                                radius: 5
                                color: Kirigami.Theme.backgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                                clip: true

                                Flickable {
                                    id: haEntityListFlick
                                    anchors.fill: parent
                                    anchors.margins: 4
                                    clip: true
                                    contentWidth: width
                                    contentHeight: haEntityRepeaterCol.implicitHeight
                                    boundsBehavior: Flickable.StopAtBounds
                                    flickableDirection: Flickable.VerticalFlick

                                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                        parent: haEntityListFlick
                                        anchors.top: haEntityListFlick.top
                                        anchors.right: haEntityListFlick.right
                                        anchors.bottom: haEntityListFlick.bottom
                                        policy: haEntityListFlick.contentHeight > haEntityListFlick.height
                                            ? QQC2.ScrollBar.AlwaysOn
                                            : QQC2.ScrollBar.AsNeeded
                                    }

                                    ColumnLayout {
                                        id: haEntityRepeaterCol
                                        width: Math.max(160, haEntityListFlick.width - 16)
                                        spacing: 4

                                        Repeater {
                                            model: webApiKeyConfigWindow.haEntitiesModel
                                            delegate: Rectangle {
                                                required property var modelData
                                                readonly property string entId: String(modelData.entity_id || modelData.entityId || "")
                                                readonly property string entName: String(modelData.friendly_name || modelData.friendlyName || entId)
                                                readonly property string entDomain: String(modelData.domain || (entId.indexOf(".") >= 0 ? entId.split(".")[0] : "ha"))
                                                readonly property string entService: {
                                                    const rawSvc = String(modelData.suggested_service || modelData.suggestedService || "toggle")
                                                    if (rawSvc.indexOf("/") >= 0) return rawSvc.split("/").pop()
                                                    if (rawSvc.indexOf(".") >= 0) return rawSvc.split(".").pop()
                                                    return rawSvc
                                                }
                                                Layout.fillWidth: true
                                                implicitHeight: haRowInnerCol.implicitHeight + 10
                                                radius: 4
                                                color: Kirigami.Theme.alternateBackgroundColor
                                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
                                                border.width: 1

                                                ColumnLayout {
                                                    id: haRowInnerCol
                                                    anchors.fill: parent
                                                    anchors.margins: 5
                                                    spacing: 4

                                                    RowLayout {
                                                        Layout.fillWidth: true
                                                        spacing: 6

                                                        QQC2.Label {
                                                            text: entName
                                                            font.bold: true
                                                            font.pointSize: 10 * root.fontScale
                                                            Layout.fillWidth: true
                                                            wrapMode: Text.WordWrap
                                                        }

                                                        Rectangle {
                                                            radius: 3
                                                            color: Qt.alpha(Kirigami.Theme.highlightColor, 0.18)
                                                            implicitWidth: haStateLbl.implicitWidth + 8
                                                            implicitHeight: 18
                                                            QQC2.Label {
                                                                id: haStateLbl
                                                                anchors.centerIn: parent
                                                                text: String(modelData.state || "unknown")
                                                                font.family: "monospace"
                                                                font.pointSize: 10 * root.fontScale
                                                                font.bold: true
                                                            }
                                                        }
                                                    }

                                                    Flow {
                                                        Layout.fillWidth: true
                                                        spacing: 6

                                                        QQC2.Label {
                                                            text: entId
                                                            font.family: "monospace"
                                                            font.pointSize: 10 * root.fontScale
                                                            color: Kirigami.Theme.disabledTextColor
                                                            height: 24
                                                            verticalAlignment: Text.AlignVCenter
                                                        }

                                                        QQC2.Button {
                                                            text: qsTr("⚡ %1.%2")
                                                                .arg(entDomain)
                                                                .arg(entService)
                                                            highlighted: true
                                                            implicitHeight: 24
                                                            font.pointSize: 10 * root.fontScale
                                                            onClicked: webApiKeyConfigWindow.applyHomeAssistantEntityToKey(
                                                                modelData,
                                                                entService
                                                            )
                                                        }

                                                        QQC2.Button {
                                                            visible: entDomain === "light" || entDomain === "switch" || entDomain === "input_boolean" || entDomain === "fan" || entDomain === "automation"
                                                            text: qsTr("Turn On")
                                                            implicitHeight: 24
                                                            font.pointSize: 10 * root.fontScale
                                                            onClicked: webApiKeyConfigWindow.applyHomeAssistantEntityToKey(modelData, "turn_on")
                                                        }

                                                        QQC2.Button {
                                                            visible: entDomain === "light" || entDomain === "switch" || entDomain === "input_boolean" || entDomain === "fan" || entDomain === "automation"
                                                            text: qsTr("Turn Off")
                                                            implicitHeight: 24
                                                            font.pointSize: 10 * root.fontScale
                                                            onClicked: webApiKeyConfigWindow.applyHomeAssistantEntityToKey(modelData, "turn_off")
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

                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: apiEndpointCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: apiEndpointCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Destination:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.ComboBox {
                                    id: webApiDestinationCombo
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 120
                                    Layout.preferredWidth: 1
                                    model: root.webApiDestinationComboItems
                                    onActivated: (index) => {
                                        root.syncingWebApiScript = true
                                        if (index > 0 && (index - 1) < root.webApiDestinationsList.length) {
                                            const d = root.webApiDestinationsList[index - 1]
                                            root.webApiSelectedDestination = d.name || ""
                                            const mIdx = root.webApiMethods.indexOf((d.method || "POST").toUpperCase())
                                            webApiMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
                                            webApiUrlField.text = d.url || ""
                                            webApiHeadersField.text = d.headers || ""
                                            if (!webApiEnabledCheck.checked) {
                                                webApiEnabledCheck.checked = true
                                            }
                                        } else {
                                            root.webApiSelectedDestination = ""
                                        }
                                        root.syncingWebApiScript = false
                                        root.syncWebApiBuilderToScript()
                                    }
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.ComboBox {
                                    id: webApiMethodCombo
                                    Layout.preferredWidth: 105
                                    model: root.webApiMethods
                                    onActivated: root.syncWebApiBuilderToScript()
                                }

                                QQC2.TextField {
                                    id: webApiUrlField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 100
                                    Layout.preferredWidth: 1
                                    font.family: "monospace"
                                    placeholderText: qsTr("https://example.com/api/webhook or http://homeassistant.local:8123/api/...")
                                    selectByMouse: true
                                    onTextEdited: {
                                        if (text.trim().length > 0 && !webApiEnabledCheck.checked) {
                                            webApiEnabledCheck.checked = true
                                        }
                                        root.syncWebApiBuilderToScript()
                                    }
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                wrapMode: Text.WordWrap
                                text: qsTr("HTTP Headers (Optional — one 'Header-Name: value' per line):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            QQC2.TextArea {
                                id: webApiHeadersField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 60
                                wrapMode: TextEdit.Wrap
                                clip: true
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                placeholderText: qsTr("Content-Type: application/json\nAuthorization: Bearer <token>")
                                selectByMouse: true
                                onTextChanged: {
                                    if (!root.syncingWebApiScript) root.syncWebApiBuilderToScript()
                                }
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                wrapMode: Text.WordWrap
                                text: qsTr("Request Body / JSON Payload:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            QQC2.TextArea {
                                id: webApiBodyField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 76
                                wrapMode: TextEdit.Wrap
                                clip: true
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                placeholderText: qsTr("{\"entity_id\": \"light.studio_key_light\"}")
                                selectByMouse: true
                                onTextChanged: {
                                    if (!root.syncingWebApiScript) root.syncWebApiBuilderToScript()
                                }
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: webTrigCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: webTrigCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Trigger Mode:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.ComboBox {
                                    id: webApiTriggerModeCombo
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 120
                                    Layout.preferredWidth: 1
                                    model: [
                                        qsTr("USB HID Listener (Silent background HTTP call)"),
                                        qsTr("F-Key Shortcut Fallback (Emits F13–F24 + HTTP call)"),
                                        qsTr("Standalone Terminal curl (Types curl command on host)")
                                    ]
                                    onActivated: root.syncWebApiBuilderToScript()
                                }

                                QQC2.ComboBox {
                                    id: webApiFkeyCombo
                                    visible: webApiTriggerModeCombo.currentIndex === 1
                                    Layout.preferredWidth: 96
                                    model: ["F13","F14","F15","F16","F17","F18","F19","F20","F21","F22","F23","F24"]
                                    currentIndex: 7
                                    onActivated: root.syncWebApiBuilderToScript()
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 12

                                QQC2.Label {
                                    text: qsTr("OLED Feedback Text:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.TextField {
                                    id: webApiOledField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 100
                                    Layout.preferredWidth: 1
                                    maximumLength: 20
                                    placeholderText: qsTr("Optional OLED status text on trigger")
                                    selectByMouse: true
                                    onTextEdited: root.syncWebApiBuilderToScript()
                                }

                                QQC2.CheckBox {
                                    id: webApiLedCheck
                                    checked: true
                                    text: qsTr("Flash Key LED Cyan")
                                    onToggled: root.syncWebApiBuilderToScript()
                                }
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "media-playback-start"
                    text: qsTr("Test Webhook Now")
                    enabled: webApiUrlField.text.trim().length > 0
                    onClicked: {
                        deviceController.testWebApiCall(
                            root.buildWebApiConfigJson(),
                            root.selectedSlotNumber
                        )
                    }
                }

                QQC2.Button {
                    icon.name: "edit-clear"
                    text: qsTr("Disable / Clear")
                    onClicked: {
                        webApiEnabledCheck.checked = false
                        webApiUrlField.text = ""
                        webApiHeadersField.text = ""
                        webApiBodyField.text = ""
                        root.syncWebApiBuilderToScript()
                    }
                }

                Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }

                QQC2.Button {
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Save & Close")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        root.syncWebApiBuilderToScript()
                        webApiKeyConfigWindow.close()
                    }
                }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: webApiKeyConfigWindow.close()
                }
            }
        }
    }

    // =========================================================================
    // Profile Properties Window (Editable Fields for All Profile Properties)
    // =========================================================================
    Window {
        id: profilePropertiesWindow
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: profilePropertiesWindow.visible
            onActivated: profilePropertiesWindow.close()
            onAmbiguouslyActivated: profilePropertiesWindow.close()
        }
        title: qsTr("Profile #%1 Properties — %2").arg(profilePropertiesWindow.profNumber).arg(profilePropertiesWindow.profName)
        width: 820
        height: 740
        minimumWidth: 560
        minimumHeight: 480
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property int profNumber: 1
        property string profName: ""
        property string profFolderName: ""
        property string profBgColor: "#38bdf8"
        property bool profHasKeydownColor: false
        property string profKeydownColor: "#ffffff"
        property bool profDimUnusedKeys: true
        property bool profIsLandscape: false
        property int profOledRotation: 0 // 0 | 90 | 180 | 270
        property int profWidgetRotation: -1 // -1 (Follow OLED) | 0 | 90 | 180 | 270
        property bool profIsUpperHs: false
        property string profExtraConfigLines: ""
        property int profConfiguredSlots: 0
        property bool profWebhookEnabled: false
        property string profWebhookMethod: "POST"
        property string profWebhookUrl: ""
        property string profWebhookHeaders: "Content-Type: application/json"
        property string profWebhookBody: "{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\",\"source\":\"{{source}}\"}"
        property string webhookTestStatus: ""
        property string statusFeedback: ""

        function openForProfile(profileNumber) {
            const targetNum = profileNumber > 0 ? profileNumber : deviceController.activeProfile
            if (targetNum <= 0) return
            if (deviceController.activeProfile !== targetNum) {
                deviceController.selectProfile(targetNum)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                oledDesktopWidget.syncFromController()
            }
            profilePropertiesWindow.profNumber = targetNum
            const rawJson = deviceController.getProfilePropertiesJson(targetNum)
            try {
                const obj = JSON.parse(rawJson)
                profilePropertiesWindow.profName = obj.name || ("Profile " + targetNum)
                profilePropertiesWindow.profFolderName = obj.folderName || ("profile" + targetNum + "_" + profilePropertiesWindow.profName)
                profilePropertiesWindow.profBgColor = obj.bgColor || "#38bdf8"
                profilePropertiesWindow.profHasKeydownColor = Boolean(obj.hasKeydownColor)
                profilePropertiesWindow.profKeydownColor = obj.keydownColor || "#ffffff"
                profilePropertiesWindow.profDimUnusedKeys = obj.dimUnusedKeys !== undefined ? Boolean(obj.dimUnusedKeys) : true
                profilePropertiesWindow.profIsLandscape = Boolean(obj.isLandscape)
                profilePropertiesWindow.profOledRotation = obj.oledRotation !== undefined
                    ? Number(obj.oledRotation)
                    : (obj.isLandscape ? 90 : 0)
                profilePropertiesWindow.profWidgetRotation = obj.widgetRotation !== undefined
                    ? Number(obj.widgetRotation)
                    : -1
                profilePropertiesWindow.profIsUpperHs = Boolean(obj.isUpperHs)
                profilePropertiesWindow.profExtraConfigLines = Array.isArray(obj.extraConfigLines) ? obj.extraConfigLines.join("\n") : ""
                profilePropertiesWindow.profConfiguredSlots = obj.configuredSlotCount || 0
                const wh = obj.webhook || {}
                profilePropertiesWindow.profWebhookEnabled = Boolean(wh.enabled)
                profilePropertiesWindow.profWebhookMethod = wh.method || "POST"
                profilePropertiesWindow.profWebhookUrl = wh.url || ""
                profilePropertiesWindow.profWebhookHeaders = wh.headers !== undefined ? String(wh.headers) : "Content-Type: application/json"
                profilePropertiesWindow.profWebhookBody = wh.body !== undefined
                    ? String(wh.body)
                    : "{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\",\"source\":\"{{source}}\"}"
            } catch (e) {
                profilePropertiesWindow.profName = deviceController.activeProfileName
                profilePropertiesWindow.profBgColor = deviceController.activeProfileBgColor
                profilePropertiesWindow.profOledRotation = 0
                profilePropertiesWindow.profWidgetRotation = -1
                profilePropertiesWindow.profWebhookEnabled = false
                profilePropertiesWindow.profWebhookMethod = "POST"
                profilePropertiesWindow.profWebhookUrl = ""
            }
            profNameField.text = profilePropertiesWindow.profName
            profBgColorField.text = profilePropertiesWindow.profBgColor
            profKeydownEnableCheck.checked = profilePropertiesWindow.profHasKeydownColor
            profKeydownColorField.text = profilePropertiesWindow.profKeydownColor
            profDimUnusedCheck.checked = profilePropertiesWindow.profDimUnusedKeys
            profUpperHsCheck.checked = profilePropertiesWindow.profIsUpperHs
            profExtraLinesArea.text = profilePropertiesWindow.profExtraConfigLines
            profWebhookEnableCheck.checked = profilePropertiesWindow.profWebhookEnabled
            const mIdx = profWebhookMethodCombo.indexOfValue(profilePropertiesWindow.profWebhookMethod.toUpperCase())
            profWebhookMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 1
            profWebhookUrlField.text = profilePropertiesWindow.profWebhookUrl
            profWebhookHeadersArea.text = profilePropertiesWindow.profWebhookHeaders
            profWebhookBodyArea.text = profilePropertiesWindow.profWebhookBody
            profilePropertiesWindow.webhookTestStatus = ""
            profilePropertiesWindow.statusFeedback = ""

            profilePropertiesWindow.show()
            profilePropertiesWindow.raise()
            profilePropertiesWindow.requestActivate()
        }

        function saveAllProfileProperties() {
            const extraArr = profExtraLinesArea.text
                .split("\n")
                .map(s => s.trim())
                .filter(s => s.length > 0)
            const payload = {
                name: profNameField.text.trim(),
                bgColor: profBgColorField.text.trim(),
                hasKeydownColor: profKeydownEnableCheck.checked,
                keydownColor: profKeydownColorField.text.trim(),
                dimUnusedKeys: profDimUnusedCheck.checked,
                isLandscape: profilePropertiesWindow.profOledRotation === 90 || profilePropertiesWindow.profOledRotation === 270,
                oledRotation: profilePropertiesWindow.profOledRotation,
                widgetRotation: profilePropertiesWindow.profWidgetRotation,
                isUpperHs: profUpperHsCheck.checked,
                extraConfigLines: extraArr,
                webhook: {
                    enabled: profWebhookEnableCheck.checked,
                    method: profWebhookMethodCombo.currentText || "POST",
                    url: profWebhookUrlField.text.trim(),
                    headers: profWebhookHeadersArea.text,
                    body: profWebhookBodyArea.text
                }
            }
            const ok = deviceController.saveProfilePropertiesJson(
                profilePropertiesWindow.profNumber,
                JSON.stringify(payload)
            )
            if (ok) {
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                oledDesktopWidget.syncFromController()
                profilePropertiesWindow.statusFeedback = qsTr("✓ Saved all properties for Profile #%1.").arg(profilePropertiesWindow.profNumber)
            }
            return ok
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 14
            spacing: 10

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: profilePropertiesWindow.profBgColor
                    border.color: "#ffffff66"
                    border.width: 1
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("Profile #%1 Properties").arg(profilePropertiesWindow.profNumber)
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Folder: %1  •  Configured Slots: %2")
                            .arg(profilePropertiesWindow.profFolderName)
                            .arg(profilePropertiesWindow.profConfiguredSlots)
                        font.family: "monospace"
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        wrapMode: Text.WrapAnywhere
                    }
                }
            }

            Flickable {
                id: profPropsFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: profPropsScrollCol.implicitHeight + 12
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: profPropsFlickable
                    anchors.top: profPropsFlickable.top
                    anchors.right: profPropsFlickable.right
                    anchors.bottom: profPropsFlickable.bottom
                    policy: profPropsFlickable.contentHeight > profPropsFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: profPropsScrollCol
                    width: Math.max(
                        200,
                        profPropsFlickable.width - (profPropsFlickable.contentHeight > profPropsFlickable.height ? 18 : 2)
                    )
                    spacing: 10

                    // Identity & Colors Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profBasicCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profBasicCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10

                                QQC2.Label {
                                    text: qsTr("Profile Name (max 13 chars):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 195
                                    wrapMode: Text.WordWrap
                                }

                                QQC2.TextField {
                                    id: profNameField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 13
                                    selectByMouse: true
                                    font.bold: true
                                    placeholderText: qsTr("Profile Name")
                                }
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Default Key Color (BG_COLOR):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    width: 195
                                    height: 28
                                    verticalAlignment: Text.AlignVCenter
                                }

                                Rectangle {
                                    width: 26
                                    height: 26
                                    radius: 4
                                    color: /^#[0-9A-Fa-f]{6}$/.test(profBgColorField.text.trim())
                                        ? profBgColorField.text.trim()
                                        : profilePropertiesWindow.profBgColor
                                    border.color: "#ffffff88"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    id: profBgColorField
                                    width: 100
                                    maximumLength: 7
                                    font.family: "monospace"
                                    selectByMouse: true
                                    onTextEdited: {
                                        if (/^#[0-9A-Fa-f]{6}$/.test(text.trim())) {
                                            profilePropertiesWindow.profBgColor = text.trim()
                                        }
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "color-picker"
                                    text: qsTr("Open Color Picker...")
                                    onClicked: {
                                        if (deviceController.activeProfile !== profilePropertiesWindow.profNumber) {
                                            deviceController.selectProfile(profilePropertiesWindow.profNumber)
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncSlotsFromController()
                                        }
                                        duckyColorPickerWindow.openForProfileBg(
                                            profilePropertiesWindow.profNumber,
                                            profNameField.text.trim(),
                                            profBgColorField.text.trim()
                                        )
                                    }
                                }
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Keydown Color (KEYDOWN_COLOR):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    width: 195
                                    height: 28
                                    verticalAlignment: Text.AlignVCenter
                                }

                                QQC2.CheckBox {
                                    id: profKeydownEnableCheck
                                    text: qsTr("Enable Custom Keydown Color")
                                    onToggled: profilePropertiesWindow.saveAllProfileProperties()
                                }

                                Rectangle {
                                    visible: profKeydownEnableCheck.checked
                                    width: 26
                                    height: 26
                                    radius: 4
                                    color: /^#[0-9A-Fa-f]{6}$/.test(profKeydownColorField.text.trim())
                                        ? profKeydownColorField.text.trim()
                                        : "#ffffff"
                                    border.color: "#ffffff88"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    id: profKeydownColorField
                                    visible: profKeydownEnableCheck.checked
                                    width: 100
                                    maximumLength: 7
                                    font.family: "monospace"
                                    selectByMouse: true
                                    onEditingFinished: profilePropertiesWindow.saveAllProfileProperties()
                                }
                            }
                        }
                    }

                    // Hardware Flags & Orientation Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profFlagsCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profFlagsCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("Hardware LED & Orientation Directives (config.txt):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.CheckBox {
                                id: profDimUnusedCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("Dim Unused Keys (DIM_UNUSED_KEYS 1 — turn off LEDs on unconfigured keys)")
                                onToggled: profilePropertiesWindow.saveAllProfileProperties()
                                contentItem: QQC2.Label {
                                    leftPadding: (profDimUnusedCheck.indicator ? profDimUnusedCheck.indicator.width : 18) + profDimUnusedCheck.spacing
                                    text: profDimUnusedCheck.text
                                    font: profDimUnusedCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.Label {
                                    text: qsTr("OLED Orientation (Physical Screen):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                }

                                Flow {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.Button {
                                        text: qsTr("0° (Portrait)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 0
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 0
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("90° (Landscape CW)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 90
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 90
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("180° (Inverted)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 180
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 180
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("270° (Landscape CCW)")
                                        highlighted: profilePropertiesWindow.profOledRotation === 270
                                        onClicked: {
                                            profilePropertiesWindow.profOledRotation = 270
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }
                                }
                            }

                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4

                                QQC2.Label {
                                    text: qsTr("💻 Desktop Widget Orientation (Instant — No microSD Sync Required):")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                    QQC2.ToolTip.visible: dwRotHover.hovered
                                    QQC2.ToolTip.text: qsTr("💻 Host App Only: Configure the Desktop OLED Widget orientation separately from the physical hardware OLED screen. Applies immediately without requiring a microSD card sync.")
                                    HoverHandler { id: dwRotHover }
                                }

                                Flow {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.Button {
                                        text: qsTr("Follow OLED (%1°)").arg(profilePropertiesWindow.profOledRotation)
                                        highlighted: profilePropertiesWindow.profWidgetRotation < 0
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = -1
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("0°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 0
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 0
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("90°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 90
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 90
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("180°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 180
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 180
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }

                                    QQC2.Button {
                                        text: qsTr("270°")
                                        highlighted: profilePropertiesWindow.profWidgetRotation === 270
                                        onClicked: {
                                            profilePropertiesWindow.profWidgetRotation = 270
                                            profilePropertiesWindow.saveAllProfileProperties()
                                        }
                                    }
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                                font.pointSize: 10 * root.fontScale
                                color: Kirigami.Theme.disabledTextColor
                                text: qsTr("Note: Physical OLED 180° & 270° require DuckyDeck Studio Firmware v1.0.0+. Desktop Widget Orientation (💻) is a host-only setting that updates the on-screen Desktop OLED Widget immediately without requiring a microSD card sync.")
                            }

                            QQC2.CheckBox {
                                id: profUpperHsCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("Inverted / Upper Hot-Swap Orientation (IS_UPPER_HS 1)")
                                onToggled: profilePropertiesWindow.saveAllProfileProperties()
                                contentItem: QQC2.Label {
                                    leftPadding: (profUpperHsCheck.indicator ? profUpperHsCheck.indicator.width : 18) + profUpperHsCheck.spacing
                                    text: profUpperHsCheck.text
                                    font: profUpperHsCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }
                        }
                    }

                    // Related Profile Features Quick Access
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profQuickCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profQuickCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("Profile Integrations:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                            }

                            Flow {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.Button {
                                    text: qsTr("✨ Profile RGB Lighting FX...")
                                    onClicked: {
                                        if (deviceController.activeProfile !== profilePropertiesWindow.profNumber) {
                                            deviceController.selectProfile(profilePropertiesWindow.profNumber)
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncSlotsFromController()
                                        }
                                        lightingAnimatorDialog.openFor("profile", 0)
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "bookmark-new"
                                    text: qsTr("Save as Template...")
                                    onClicked: {
                                        profileTemplatesDialog.openSaveFromProfile(
                                            profilePropertiesWindow.profNumber,
                                            profNameField.text.trim()
                                        )
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "crosshairs"
                                    text: qsTr("Bind Window...")
                                    onClicked: {
                                        if (deviceController.activeProfile !== profilePropertiesWindow.profNumber) {
                                            deviceController.selectProfile(profilePropertiesWindow.profNumber)
                                            root.syncSidebarProfilesFromController()
                                            hardwareCanvas.syncSlotsFromController()
                                        }
                                        deviceController.armWindowPicker(true)
                                    }
                                }

                                QQC2.Button {
                                    icon.name: "preferences-system-windows"
                                    text: qsTr("Auto-Switch Rules...")
                                    onClicked: autoSwitchDrawer.open()
                                }
                            }
                        }
                    }

                    // Per-Profile Change Webhook Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: profWebhookCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: profWebhookEnableCheck.checked
                            ? Kirigami.Theme.highlightColor
                            : Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: profWebhookCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("🔔 Profile Change Webhook (On Profile Activated)")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.fillWidth: true
                                    wrapMode: Text.WordWrap
                                }

                                QQC2.Button {
                                    icon.name: "network-transmit-receive"
                                    text: deviceController.webApiTestBusy ? qsTr("Testing...") : qsTr("Test Webhook")
                                    enabled: !deviceController.webApiTestBusy && profWebhookUrlField.text.trim().length > 0
                                    onClicked: {
                                        deviceController.testProfileWebhookJson(
                                            JSON.stringify({
                                                enabled: true,
                                                method: profWebhookMethodCombo.currentText || "POST",
                                                url: profWebhookUrlField.text.trim(),
                                                headers: profWebhookHeadersArea.text,
                                                body: profWebhookBodyArea.text
                                            }),
                                            profilePropertiesWindow.profNumber
                                        )
                                        profilePropertiesWindow.webhookTestStatus = qsTr("⏳ Testing webhook → %1 (see status bar / event log)").arg(profWebhookUrlField.text.trim())
                                    }
                                }
                            }

                            QQC2.CheckBox {
                                id: profWebhookEnableCheck
                                Layout.fillWidth: true
                                text: qsTr("Trigger this HTTP Webhook whenever Profile #%1 becomes active (UI, Hardware, Auto-Switch, or REST API)").arg(profilePropertiesWindow.profNumber)
                                contentItem: QQC2.Label {
                                    leftPadding: (profWebhookEnableCheck.indicator ? profWebhookEnableCheck.indicator.width : 18) + profWebhookEnableCheck.spacing
                                    text: profWebhookEnableCheck.text
                                    font: profWebhookEnableCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.ComboBox {
                                    id: profWebhookMethodCombo
                                    Layout.preferredWidth: 105
                                    model: ["GET", "POST", "PUT", "PATCH", "DELETE"]
                                    currentIndex: 1
                                }

                                QQC2.TextField {
                                    id: profWebhookUrlField
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 120
                                    Layout.preferredWidth: 1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    selectByMouse: true
                                    placeholderText: qsTr("http://homeassistant.local:8123/api/webhook/duckypad_profile")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Headers (Optional, one 'Header-Name: Value' per line):")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.TextArea {
                                id: profWebhookHeadersArea
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 46
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                wrapMode: TextEdit.Wrap
                                clip: true
                                placeholderText: qsTr("Content-Type: application/json\nAuthorization: Bearer <token>")
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Payload Body (Template placeholders: {{profile_number}}, {{profile_name}}, {{previous_profile_number}}, {{previous_profile_name}}, {{source}}, {{device_serial}}, {{timestamp}}):")
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.TextArea {
                                id: profWebhookBodyArea
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: 58
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                wrapMode: TextEdit.Wrap
                                clip: true
                                placeholderText: qsTr("{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\"}")
                                background: Rectangle {
                                    radius: 4
                                    color: Kirigami.Theme.backgroundColor
                                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                    border.width: 1
                                }
                            }

                            QQC2.Label {
                                visible: profilePropertiesWindow.webhookTestStatus.length > 0
                                text: profilePropertiesWindow.webhookTestStatus
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                font.bold: true
                                color: profilePropertiesWindow.webhookTestStatus.startsWith("✓")
                                    ? Kirigami.Theme.positiveTextColor
                                    : Kirigami.Theme.neutralTextColor
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }
                        }
                    }

                    // Extra config.txt lines
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        QQC2.Label {
                            text: qsTr("Additional Custom config.txt Directives (Optional, one per line):")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        QQC2.TextArea {
                            id: profExtraLinesArea
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 68
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            wrapMode: TextEdit.Wrap
                            clip: true
                            placeholderText: qsTr("// Optional custom config.txt directives")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }
                    }

                    QQC2.Label {
                        visible: profilePropertiesWindow.statusFeedback.length > 0
                        text: profilePropertiesWindow.statusFeedback
                        color: Kirigami.Theme.positiveTextColor
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Save & Close")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        if (profilePropertiesWindow.saveAllProfileProperties()) {
                            profilePropertiesWindow.close()
                        }
                    }
                }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: profilePropertiesWindow.close()
                }
            }
        }
    }

    // =========================================================================
    // Key / Slot Properties Window (Editable Fields for All Key Properties)
    // =========================================================================
    Window {
        id: keyPropertiesWindow
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: keyPropertiesWindow.visible
            onActivated: keyPropertiesWindow.close()
            onAmbiguouslyActivated: keyPropertiesWindow.close()
        }
        title: qsTr("Key Properties — Slot %1 (%2)").arg(keyPropertiesWindow.slotNum).arg(keyPropertiesWindow.slotTitle)
        width: 780
        height: 680
        minimumWidth: 660
        minimumHeight: 540
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property int slotNum: 1
        property string slotTitle: "Key 1"
        property string slotColor: "#38bdf8"

        function openForSlot(slotNumber) {
            const targetSlot = slotNumber > 0 ? slotNumber : root.selectedSlotNumber
            if (targetSlot <= 0 || targetSlot > 58) return

            if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
                root.flushPendingSlotEditsIfModified()
            }

            keyPropertiesWindow.slotNum = targetSlot
            keyPropertiesWindow.slotTitle = hardwareCanvas.slotTitleFor(targetSlot)
            keyPropertiesWindow.slotColor = String(hardwareCanvas.slotColorFor(targetSlot))

            let l1 = ""
            let l2 = ""
            let wl1 = ""
            let wl2 = ""
            let allowAbort = false
            let disableRepeat = false
            try {
                const metaJson = deviceController.getSlotMetaJson(targetSlot)
                const meta = JSON.parse(metaJson)
                l1 = meta.l1 || ""
                l2 = meta.l2 || ""
                wl1 = meta.wl1 || ""
                wl2 = meta.wl2 || ""
                allowAbort = !!meta.allowAbort
                disableRepeat = !!meta.disableAutorepeat
            } catch (e) {}

            keyPropLabel1Field.text = root.clampOledLabel(l1)
            keyPropLabel2Field.text = root.clampOledLabel(l2)
            keyPropWidgetLabel1Field.text = wl1
            keyPropWidgetLabel2Field.text = wl2
            keyPropColorField.text = keyPropertiesWindow.slotColor
            keyPropDontRepeatCheck.checked = disableRepeat
            keyPropAllowAbortCheck.checked = allowAbort
            keyPropPressScriptArea.text = deviceController.getSlotScript(targetSlot, false)
            keyPropReleaseScriptArea.text = deviceController.getSlotScript(targetSlot, true)

            keyPropertiesWindow.show()
            keyPropertiesWindow.raise()
            keyPropertiesWindow.requestActivate()
        }

        function applyKeyProperties() {
            const targetSlot = keyPropertiesWindow.slotNum
            if (targetSlot <= 0 || targetSlot > 58) return

            const l1 = root.clampOledLabel(keyPropLabel1Field.text.trim())
            const l2 = root.clampOledLabel(keyPropLabel2Field.text.trim())
            const wl1 = keyPropWidgetLabel1Field.text.trim()
            const wl2 = keyPropWidgetLabel2Field.text.trim()
            const disableRepeat = keyPropDontRepeatCheck.checked
            const allowAbort = keyPropAllowAbortCheck.checked
            const pressScript = keyPropPressScriptArea.text
            const releaseScript = keyPropReleaseScriptArea.text

            const hexCandidate = keyPropColorField.text.trim()
            if (targetSlot <= 20 && /^#[0-9A-Fa-f]{6}$/.test(hexCandidate)) {
                const rVal = parseInt(hexCandidate.substring(1, 3), 16)
                const gVal = parseInt(hexCandidate.substring(3, 5), 16)
                const bVal = parseInt(hexCandidate.substring(5, 7), 16)
                if (hexCandidate.toLowerCase() !== keyPropertiesWindow.slotColor.toLowerCase()) {
                    hardwareCanvas.setKeyRgbAt(targetSlot, rVal, gVal, bVal)
                }
                keyPropertiesWindow.slotColor = hexCandidate
            }

            deviceController.saveSlotConfig(
                targetSlot,
                l1,
                l2,
                pressScript,
                releaseScript,
                allowAbort,
                disableRepeat
            )
            deviceController.saveSlotWidgetLabels(
                targetSlot,
                wl1,
                wl2
            )

            hardwareCanvas.syncSlotsFromController()

            if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
                root.loadSlotFromController(
                    targetSlot,
                    hardwareCanvas.slotTitleFor(targetSlot),
                    hardwareCanvas.slotColorFor(targetSlot)
                )
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: keyPropertiesWindow.slotColor
                    border.color: "#ffffff66"
                    border.width: 1
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("Key Properties — %1 (Slot %2)").arg(keyPropertiesWindow.slotTitle).arg(keyPropertiesWindow.slotNum)
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Profile #%1 (%2) • Edit OLED labels, Desktop Widget alternate labels, per-key RGB color, execution flags, and scripts.")
                            .arg(deviceController.activeProfile)
                            .arg(deviceController.activeProfileName)
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                    }
                }
            }

            Flickable {
                id: keyPropsFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: keyPropsScrollCol.implicitHeight + 8
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: keyPropsFlickable
                    anchors.top: keyPropsFlickable.top
                    anchors.right: keyPropsFlickable.right
                    anchors.bottom: keyPropsFlickable.bottom
                    policy: keyPropsFlickable.contentHeight > keyPropsFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: keyPropsScrollCol
                    width: Math.max(
                        200,
                        keyPropsFlickable.width - (keyPropsFlickable.contentHeight > keyPropsFlickable.height ? 18 : 2)
                    )
                    spacing: 10

                    // OLED Labels & Per-Key RGB Color Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: keyBasicGrid.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        GridLayout {
                            id: keyBasicGrid
                            anchors.fill: parent
                            anchors.margins: 9
                            columns: 2
                            columnSpacing: 12
                            rowSpacing: 8

                            QQC2.Label {
                                text: qsTr("OLED Line 1 (max 5 chars):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8

                                QQC2.TextField {
                                    id: keyPropLabel1Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 10
                                    font.family: "monospace"
                                    font.bold: true
                                    selectByMouse: true
                                    placeholderText: qsTr("Top line (max 5 chars)")
                                    onTextChanged: {
                                        const clamped = root.clampOledLabel(text)
                                        if (text !== clamped) {
                                            text = clamped
                                        }
                                    }
                                }

                                QQC2.Label {
                                    text: qsTr("%1/5").arg(root.oledLabelCharCount(keyPropLabel1Field.text))
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.Button {
                                    text: qsTr("★ Icon...")
                                    font.bold: true
                                    onClicked: oledIconPickerDialog.openForKeyProperties(1)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Pick an OLED Button Icon or Symbol for Line 1")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("OLED Line 2 (max 5 chars):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                visible: keyPropertiesWindow.slotNum <= 20
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8
                                visible: keyPropertiesWindow.slotNum <= 20

                                QQC2.TextField {
                                    id: keyPropLabel2Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 10
                                    font.family: "monospace"
                                    selectByMouse: true
                                    placeholderText: qsTr("Bottom line (max 5 chars, optional)")
                                    onTextChanged: {
                                        const clamped = root.clampOledLabel(text)
                                        if (text !== clamped) {
                                            text = clamped
                                        }
                                    }
                                }

                                QQC2.Label {
                                    text: qsTr("%1/5").arg(root.oledLabelCharCount(keyPropLabel2Field.text))
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.Button {
                                    text: qsTr("★ Icon...")
                                    font.bold: true
                                    onClicked: oledIconPickerDialog.openForKeyProperties(2)
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: qsTr("Pick an OLED Button Icon or Symbol for Line 2")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Desktop Widget Label L1 (optional):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8

                                QQC2.TextField {
                                    id: keyPropWidgetLabel1Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 24
                                    font.family: "monospace"
                                    selectByMouse: true
                                    placeholderText: qsTr("Longer label for Desktop OLED Widget (up to 24 chars, falls back to OLED L1)")
                                }

                                QQC2.Label {
                                    text: qsTr("%1/24").arg(keyPropWidgetLabel1Field.text.length)
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Desktop Widget Label L2 (optional):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                visible: keyPropertiesWindow.slotNum <= 20
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8
                                visible: keyPropertiesWindow.slotNum <= 20

                                QQC2.TextField {
                                    id: keyPropWidgetLabel2Field
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    maximumLength: 24
                                    font.family: "monospace"
                                    selectByMouse: true
                                    placeholderText: qsTr("Longer bottom line for Desktop OLED Widget (up to 24 chars, optional)")
                                }

                                QQC2.Label {
                                    text: qsTr("%1/24").arg(keyPropWidgetLabel2Field.text.length)
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            QQC2.Label {
                                text: qsTr("Key RGB Color (SWCOLOR):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                visible: keyPropertiesWindow.slotNum <= 20
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                Layout.preferredWidth: 1
                                spacing: 8
                                visible: keyPropertiesWindow.slotNum <= 20

                                Rectangle {
                                    width: 24
                                    height: 24
                                    radius: 4
                                    color: /^#[0-9A-Fa-f]{6}$/.test(keyPropColorField.text.trim())
                                        ? keyPropColorField.text.trim()
                                        : keyPropertiesWindow.slotColor
                                    border.color: "#ffffff88"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    id: keyPropColorField
                                    Layout.preferredWidth: 96
                                    maximumLength: 7
                                    font.family: "monospace"
                                    selectByMouse: true
                                }

                                QQC2.Button {
                                    icon.name: "color-picker"
                                    text: qsTr("Color Picker...")
                                    onClicked: duckyColorPickerWindow.openForKey(keyPropertiesWindow.slotNum, keyPropColorField.text.trim())
                                }

                                QQC2.Button {
                                    icon.name: "edit-clear"
                                    text: qsTr("Use Profile Default")
                                    onClicked: {
                                        deviceController.clearKeyColor(keyPropertiesWindow.slotNum)
                                        hardwareCanvas.syncSlotsFromController()
                                        const defCol = hardwareCanvas.slotColorFor(keyPropertiesWindow.slotNum)
                                        keyPropertiesWindow.slotColor = defCol
                                        keyPropColorField.text = defCol
                                        if (root.selectedSlotNumber === keyPropertiesWindow.slotNum) {
                                            root.selectedSlotColor = defCol
                                        }
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }
                        }
                    }

                    // Execution Behavior Flags Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: keyFlagsCol.implicitHeight + 18
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        ColumnLayout {
                            id: keyFlagsCol
                            anchors.fill: parent
                            anchors.margins: 9
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("Hardware Execution Flags (config.txt):")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.CheckBox {
                                id: keyPropDontRepeatCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("No Repeat on Key Held Down (dr%1 — run once per physical press instead of auto-repeating)").arg(keyPropertiesWindow.slotNum)
                                contentItem: QQC2.Label {
                                    leftPadding: (keyPropDontRepeatCheck.indicator ? keyPropDontRepeatCheck.indicator.width : 18) + keyPropDontRepeatCheck.spacing
                                    text: keyPropDontRepeatCheck.text
                                    font: keyPropDontRepeatCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            QQC2.CheckBox {
                                id: keyPropAllowAbortCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                text: qsTr("Allow Early Abort (ab%1 — pressing any key while this macro runs terminates it immediately)").arg(keyPropertiesWindow.slotNum)
                                contentItem: QQC2.Label {
                                    leftPadding: (keyPropAllowAbortCheck.indicator ? keyPropAllowAbortCheck.indicator.width : 18) + keyPropAllowAbortCheck.spacing
                                    text: keyPropAllowAbortCheck.text
                                    font: keyPropAllowAbortCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }
                        }
                    }

                    // Quick Action Windows Row
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: keyQuickWinsRow.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                        border.width: 1

                        RowLayout {
                            id: keyQuickWinsRow
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            QQC2.Button {
                                visible: keyPropertiesWindow.slotNum <= 20
                                text: qsTr("✨ Key RGB FX...")
                                onClicked: lightingAnimatorDialog.openFor("key", keyPropertiesWindow.slotNum)
                            }

                            QQC2.Button {
                                text: qsTr("🚀 Launch App Window...")
                                onClicked: appLaunchConfigWindow.openWindow(keyPropertiesWindow.slotNum)
                            }

                            QQC2.Button {
                                text: qsTr("🌐 Web API Window...")
                                onClicked: webApiKeyConfigWindow.openWindow(keyPropertiesWindow.slotNum)
                            }

                            QQC2.Button {
                                text: qsTr("🔴 Record Macro...")
                                onClicked: macroRecorderDialog.openRecorder()
                            }

                            Item { Layout.fillWidth: true }
                        }
                    }

                    // On Press & On Release Scripts
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        QQC2.Label {
                            text: qsTr("⬇ On Press DuckyScript 3 (key%1.txt):").arg(keyPropertiesWindow.slotNum)
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        QQC2.TextArea {
                            id: keyPropPressScriptArea
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 110
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            wrapMode: TextEdit.Wrap
                            clip: true
                            placeholderText: qsTr("// On Press DuckyScript 3 commands")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }

                        QQC2.Label {
                            visible: keyPropertiesWindow.slotNum <= 20
                            text: qsTr("⬆ On Release DuckyScript 3 (key%1-release.txt):").arg(keyPropertiesWindow.slotNum)
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        QQC2.TextArea {
                            id: keyPropReleaseScriptArea
                            visible: keyPropertiesWindow.slotNum <= 20
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 80
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            wrapMode: TextEdit.Wrap
                            clip: true
                            placeholderText: qsTr("// Optional On Release DuckyScript 3 commands")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Save & Close")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        keyPropertiesWindow.applyKeyProperties()
                        keyPropertiesWindow.close()
                    }
                }

                QQC2.Button {
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: keyPropertiesWindow.close()
                }
            }
        }
    }

    property int showcaseStep: 0

    function selectShowcaseSlot(slotNum) {
        const title = hardwareCanvas.slotTitleFor(slotNum)
        const color = hardwareCanvas.slotColorFor(slotNum)
        hardwareCanvas.selectedSlot = slotNum
        root.loadSlotFromController(slotNum, title, color)
    }

    Timer {
        id: showcaseRunnerTimer
        interval: 350
        repeat: true
        running: deviceController.isDemoShowcaseMode()
        onTriggered: {
            const screenshotsDir = "/home/millinr/Builds/Ducky/duckydeck-studio/docs/screenshots/"
            const framesDir = "/home/millinr/Builds/Ducky/duckydeck-studio/docs/video_frames/"
            const step = root.showcaseStep
            root.showcaseStep = step + 1

            if (step === 0) {
                deviceController.selectProfile(1)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 1) {
                deviceController.captureShowcaseFrame(screenshotsDir + "01-dolphin-overview.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_01.png",
                    "DuckyDeck Studio — KDE Plasma 6 Configurator for duckyPad Pro & 2020",
                    "Profile #1: Dolphin File Manager • 20 RGB Mechanical Keys, Dual Rotary Encoders & Live DSVM v2 Compiler"
                )
                deviceController.selectProfile(2)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 2) {
                deviceController.captureShowcaseFrame(screenshotsDir + "02-google-chrome.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_02.png",
                    "Profile #2: Google Chrome — Browser Navigation, Tab Management & DevTools",
                    "Instant KWin Wayland Auto-Switching when Google Chrome receives window focus"
                )
                root.selectShowcaseSlot(5)
            } else if (step === 3) {
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_03.png",
                    "Interactive 26-Slot Hardware Canvas & Real-Time DuckyScript 3 Editor",
                    "Click any mechanical key or rotary encoder action to inspect and compile DuckyScript 3 macros"
                )
                deviceController.selectProfile(3)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 4) {
                deviceController.captureShowcaseFrame(screenshotsDir + "03-vscode-ide.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_04.png",
                    "Profile #3: Visual Studio Code — Build, Debug, Git & Editor Macros",
                    "Syntax highlighting, line numbers, Ctrl+Space autocomplete & live .dsb bytecode verification"
                )
                duckyColorPickerWindow.openForKey(4, "#f97316")
            } else if (step === 5) {
                deviceController.captureShowcaseFrame(screenshotsDir + "04-color-picker-palette.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_05.png",
                    "Custom Key & Profile Background Color Picker",
                    "Interactive HSV color wheel, RGB/Hex sliders, Saved Colors palette & Used Colors across all profiles"
                )
                duckyColorPickerWindow.close()
                deviceController.selectProfile(4)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(17)
                root.showWebApiBuilder = true
            } else if (step === 6) {
                deviceController.captureShowcaseFrame(screenshotsDir + "05-obs-studio-web-api.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_06.png",
                    "Profile #4: OBS Studio — Scene Switching & Host-Side Web API Triggers",
                    "Trigger HTTP/REST webhooks (Home Assistant, OBS WebSocket, local APIs) directly from duckyPad keys"
                )
                root.showWebApiBuilder = false
                deviceController.selectProfile(5)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(7)
            } else if (step === 7) {
                deviceController.captureShowcaseFrame(screenshotsDir + "06-konsole-terminal.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_07.png",
                    "Profile #5: KDE Konsole — One-Key Terminal Commands, Git & Cargo Workflows",
                    "Automate frequent terminal commands, split panes, and system monitoring with zero typing lag"
                )
                deviceController.selectProfile(6)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(19)
            } else if (step === 8) {
                deviceController.captureShowcaseFrame(screenshotsDir + "07-blender-3d.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_08.png",
                    "Profile #6: Blender 3D — Viewport Navigation, Sculpting Brushes & Loops",
                    "Full DuckyScript 3 loop & conditional support with Dual Rotary Encoder zoom and brush sizing"
                )
                deviceController.selectProfile(7)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
            } else if (step === 9) {
                deviceController.captureShowcaseFrame(screenshotsDir + "08-spotify-media.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_09.png",
                    "Profile #7: Spotify & Media Controls — Hardware Playback & Volume Encoders",
                    "Dedicated media transport keys and rotary encoder volume/scrubbing controls"
                )
                deviceController.selectProfile(8)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(2)
                root.showAppLaunchBuilder = true
            } else if (step === 10) {
                deviceController.captureShowcaseFrame(screenshotsDir + "09-app-launcher.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_10.png",
                    "Profile #8: Desktop Application Launcher — Start Any App from a Key Press",
                    "Browse installed XDG desktop applications or custom scripts and launch them instantaneously"
                )
                root.showAppLaunchBuilder = false
                deviceController.selectProfile(1)
                root.syncSidebarProfilesFromController()
                hardwareCanvas.syncSlotsFromController()
                root.selectShowcaseSlot(1)
                autoSwitchDrawer.open()
            } else if (step === 11) {
                deviceController.captureShowcaseFrame(screenshotsDir + "10-wayland-auto-switcher.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_11.png",
                    "KWin Wayland D-Bus Auto-Switcher — Automatic Profile Switching",
                    "Bind window classes and titles (Dolphin, Chrome, VS Code, OBS, Konsole, Blender, Spotify) to profiles"
                )
                autoSwitchDrawer.close()
                eventLogDialog.openLogWindow()
            } else if (step === 12) {
                deviceController.captureShowcaseFrame(screenshotsDir + "11-system-event-log.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_12.png",
                    "Live System Event Log — Real-Time Diagnostics & Audit Trail",
                    "Filter USB HID, SD Sync, KWin Auto-Switcher, Web API, and Application Launch events by category"
                )
                eventLogDialog.close()
                aboutDialog.openAboutWindow()
            } else if (step === 13) {
                deviceController.captureShowcaseFrame(screenshotsDir + "12-about-duckydeck.png", "", "")
                deviceController.captureShowcaseFrame(
                    framesDir + "scene_13.png",
                    "Open Source for the duckyPad Community — Special Thanks to dekuNukem",
                    "Explore official duckyPad hardware at duckypad.com and dekuNukem's Tindie store"
                )
                aboutDialog.close()
                showcaseRunnerTimer.stop()
                deviceController.quitApplication()
            }
        }
    }

    // =========================================================================
    // RGB Lighting Animator Window (Profile-Wide & Per-Key Gaming FX)
    // =========================================================================
    Window {
        id: lightingAnimatorDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: lightingAnimatorDialog.visible
            onActivated: lightingAnimatorDialog.close()
            onAmbiguouslyActivated: lightingAnimatorDialog.close()
        }
        title: qsTr("RGB Lighting Effects & Animations — DuckyDeck Studio")
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint
        width: Math.min(820, Screen.desktopAvailableWidth - 80)
        height: Math.min(680, Screen.desktopAvailableHeight - 80)
        minimumWidth: 680
        minimumHeight: 500
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property string targetScope: "profile" // "profile" or "key"
        property int targetKeySlot: 1
        property bool animEnabled: false
        property string animPreset: "wave_h"
        property string animTrigger: "both" // "idle" | "keypress" | "both"
        property int animSpeed: 5
        property string animColor1: "#38bdf8"
        property string animColor2: "#a855f7"
        property int animIdleTimeoutSec: 5
        property bool animPushToHardware: true
        property bool animCompileIntoScript: true
        property string statusBannerText: ""

        readonly property var presetCatalog: [
            { id: "pulse", icon: "💓", title: qsTr("Breathing / Pulse"), desc: qsTr("Smooth sinusoidal breathing between Primary & Secondary RGB colors") },
            { id: "wave_h", icon: "🌊", title: qsTr("Horizontal Wave"), desc: qsTr("Sweeping RGB color wave flowing left-to-right across key columns") },
            { id: "wave_v", icon: "💧", title: qsTr("Vertical Cascade"), desc: qsTr("Cascading RGB color wave flowing top-to-bottom down key rows") },
            { id: "ripple", icon: "💥", title: qsTr("Ripple / Shockwave"), desc: qsTr("Radial shockwave ring expanding outward from pressed or center key") },
            { id: "starlight", icon: "✨", title: qsTr("Reactive Starlight"), desc: qsTr("Twinkling multi-speed stardust sparkles across keycaps") },
            { id: "spectrum", icon: "🌈", title: qsTr("Rainbow Spectrum"), desc: qsTr("Continuous 360° HSV gaming rainbow cycle across all keys") },
            { id: "scanner", icon: "🚨", title: qsTr("Knight Rider Scanner"), desc: qsTr("Sweeping Cylon/KITT beam with glowing aftertrail") }
        ]

        readonly property var quickColors: [
            "#38bdf8", "#a855f7", "#ec4899", "#ef4444",
            "#f97316", "#eab308", "#22c55e", "#14b8a6",
            "#3b82f6", "#6366f1", "#ffffff", "#0f172a"
        ]

        function loadTargetConfig() {
            let cfg = (lightingAnimatorDialog.targetScope === "key")
                ? root.getKeyLightingConfig(lightingAnimatorDialog.targetKeySlot)
                : root.getProfileLightingConfig()
            if (!cfg) {
                cfg = root.defaultLightingConfig(lightingAnimatorDialog.targetScope !== "key")
            }
            lightingAnimatorDialog.animEnabled = Boolean(cfg.enabled)
            lightingAnimatorDialog.animPreset = cfg.preset ? String(cfg.preset) : "wave_h"
            lightingAnimatorDialog.animTrigger = cfg.trigger ? String(cfg.trigger) : "both"
            lightingAnimatorDialog.animSpeed = cfg.speed !== undefined ? Number(cfg.speed) : 5
            lightingAnimatorDialog.animColor1 = cfg.color1 ? String(cfg.color1) : "#38bdf8"
            lightingAnimatorDialog.animColor2 = cfg.color2 ? String(cfg.color2) : "#a855f7"
            lightingAnimatorDialog.animIdleTimeoutSec = cfg.idleTimeoutSec !== undefined ? Number(cfg.idleTimeoutSec) : 5
            lightingAnimatorDialog.animPushToHardware = cfg.streamToHardware !== undefined
                ? Boolean(cfg.streamToHardware)
                : (cfg.pushToHardware !== undefined ? Boolean(cfg.pushToHardware) : true)
            lightingAnimatorDialog.animCompileIntoScript = cfg.embedInKeyScript !== undefined
                ? Boolean(cfg.embedInKeyScript)
                : (cfg.compileIntoScript !== undefined ? Boolean(cfg.compileIntoScript) : true)
            lightingAnimatorDialog.statusBannerText = ""
        }

        function currentConfigObject() {
            return {
                enabled: lightingAnimatorDialog.animEnabled,
                preset: lightingAnimatorDialog.animPreset,
                trigger: lightingAnimatorDialog.animTrigger,
                speed: lightingAnimatorDialog.animSpeed,
                color1: lightingAnimatorDialog.animColor1,
                color2: lightingAnimatorDialog.animColor2,
                idleTimeoutSec: lightingAnimatorDialog.animIdleTimeoutSec,
                streamToHardware: lightingAnimatorDialog.animPushToHardware,
                embedInKeyScript: lightingAnimatorDialog.animCompileIntoScript,
                pushToHardware: lightingAnimatorDialog.animPushToHardware,
                compileIntoScript: lightingAnimatorDialog.animCompileIntoScript
            }
        }

        function openFor(scope, slotNumber) {
            lightingAnimatorDialog.targetScope = (scope === "key") ? "key" : "profile"
            if (slotNumber >= 1 && slotNumber <= 20) {
                lightingAnimatorDialog.targetKeySlot = slotNumber
            } else if (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20) {
                lightingAnimatorDialog.targetKeySlot = root.selectedSlotNumber
            } else {
                lightingAnimatorDialog.targetKeySlot = 1
            }
            lightingAnimatorDialog.loadTargetConfig()
            lightingAnimatorDialog.show()
            lightingAnimatorDialog.raise()
            lightingAnimatorDialog.requestActivate()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10

            // Header
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: Qt.alpha("#a855f7", 0.2)
                    border.color: "#c084fc"
                    border.width: 1
                    QQC2.Label {
                        anchors.centerIn: parent
                        text: "✨"
                        font.pointSize: 13 * root.fontScale
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("Gaming RGB Lighting Effects & Key Animations")
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        color: Kirigami.Theme.textColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Attach idle or reactive keypress animations (Pulse, Waves, Ripple, Starlight, Spectrum, Scanner) to an entire profile or an individual key.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                    }
                }
            }

            // Scope Selector Bar (Profile-Wide vs Individual Key)
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: scopeRow.implicitHeight + 16
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                border.width: 1

                RowLayout {
                    id: scopeRow
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 8

                    QQC2.Label {
                        text: qsTr("Attach To:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                    }

                    QQC2.Button {
                        text: qsTr("🎛 Profile #%1 (%2)")
                            .arg(deviceController.activeProfile)
                            .arg(deviceController.activeProfileName)
                        checkable: true
                        checked: lightingAnimatorDialog.targetScope === "profile"
                        highlighted: lightingAnimatorDialog.targetScope === "profile"
                        font.bold: lightingAnimatorDialog.targetScope === "profile"
                        onClicked: {
                            lightingAnimatorDialog.targetScope = "profile"
                            lightingAnimatorDialog.loadTargetConfig()
                        }
                    }

                    QQC2.Button {
                        text: qsTr("⌨ Individual Key")
                        checkable: true
                        checked: lightingAnimatorDialog.targetScope === "key"
                        highlighted: lightingAnimatorDialog.targetScope === "key"
                        font.bold: lightingAnimatorDialog.targetScope === "key"
                        onClicked: {
                            lightingAnimatorDialog.targetScope = "key"
                            lightingAnimatorDialog.loadTargetConfig()
                        }
                    }

                    QQC2.ComboBox {
                        id: animKeySelectorCombo
                        visible: lightingAnimatorDialog.targetScope === "key"
                        Layout.preferredWidth: 150
                        model: {
                            let arr = []
                            for (let k = 1; k <= 20; k++) {
                                const t = hardwareCanvas.slotTitleFor(k)
                                arr.push(qsTr("Key %1 — %2").arg(k).arg(t))
                            }
                            return arr
                        }
                        currentIndex: Math.max(0, Math.min(19, lightingAnimatorDialog.targetKeySlot - 1))
                        onActivated: (idx) => {
                            lightingAnimatorDialog.targetKeySlot = idx + 1
                            lightingAnimatorDialog.loadTargetConfig()
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Switch {
                        checked: lightingAnimatorDialog.animEnabled
                        text: lightingAnimatorDialog.animEnabled ? qsTr("Enabled") : qsTr("Disabled")
                        font.bold: true
                        onToggled: lightingAnimatorDialog.animEnabled = checked
                    }
                }
            }

            // Scrollable Settings Body
            Flickable {
                id: lightingFlickable
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: width
                contentHeight: lightingScrollCol.implicitHeight + 8
                boundsBehavior: Flickable.StopAtBounds
                flickableDirection: Flickable.VerticalFlick

                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                    parent: lightingFlickable
                    anchors.top: lightingFlickable.top
                    anchors.right: lightingFlickable.right
                    anchors.bottom: lightingFlickable.bottom
                    policy: lightingFlickable.contentHeight > lightingFlickable.height
                        ? QQC2.ScrollBar.AlwaysOn
                        : QQC2.ScrollBar.AsNeeded
                }

                ColumnLayout {
                    id: lightingScrollCol
                    width: Math.max(
                        200,
                        lightingFlickable.width - (lightingFlickable.contentHeight > lightingFlickable.height ? 18 : 2)
                    )
                    spacing: 10

                    // Trigger Condition Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: triggerCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                        border.width: 1

                        ColumnLayout {
                            id: triggerCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Animation Trigger:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                }

                                QQC2.Button {
                                    text: qsTr("⚡ On Keypress + When Idle (Both)")
                                    checkable: true
                                    checked: lightingAnimatorDialog.animTrigger === "both"
                                    highlighted: lightingAnimatorDialog.animTrigger === "both"
                                    onClicked: {
                                        lightingAnimatorDialog.animTrigger = "both"
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }

                                QQC2.Button {
                                    text: qsTr("👆 On Keypress Only")
                                    checkable: true
                                    checked: lightingAnimatorDialog.animTrigger === "keypress"
                                    highlighted: lightingAnimatorDialog.animTrigger === "keypress"
                                    onClicked: {
                                        lightingAnimatorDialog.animTrigger = "keypress"
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }

                                QQC2.Button {
                                    text: qsTr("🌙 When Idle Only")
                                    checkable: true
                                    checked: lightingAnimatorDialog.animTrigger === "idle"
                                    highlighted: lightingAnimatorDialog.animTrigger === "idle"
                                    onClicked: {
                                        lightingAnimatorDialog.animTrigger = "idle"
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }

                            RowLayout {
                                visible: lightingAnimatorDialog.animTrigger === "idle" || lightingAnimatorDialog.animTrigger === "both"
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Start Idle Animation After:")
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.SpinBox {
                                    from: 0
                                    to: 300
                                    value: lightingAnimatorDialog.animIdleTimeoutSec
                                    onValueModified: lightingAnimatorDialog.animIdleTimeoutSec = value
                                }

                                QQC2.Label {
                                    text: qsTr("seconds of inactivity (0 = animate continuously)")
                                    font.pointSize: 10 * root.fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                }
                            }
                        }
                    }

                    // Preset Cards Grid
                    QQC2.Label {
                        text: qsTr("Select Animation Preset:")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                    }

                    GridLayout {
                        Layout.fillWidth: true
                        columns: 2
                        rowSpacing: 8
                        columnSpacing: 8

                        Repeater {
                            model: lightingAnimatorDialog.presetCatalog

                            delegate: Rectangle {
                                required property var modelData
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                implicitHeight: presetCardRow.implicitHeight + 16
                                radius: 6
                                readonly property bool isSelected: lightingAnimatorDialog.animPreset === modelData.id
                                color: isSelected
                                    ? Qt.alpha("#a855f7", 0.22)
                                    : (presetMouse.containsMouse ? Qt.lighter(Kirigami.Theme.alternateBackgroundColor, 1.15) : Kirigami.Theme.alternateBackgroundColor)
                                border.color: isSelected ? "#c084fc" : Qt.alpha(Kirigami.Theme.textColor, 0.16)
                                border.width: isSelected ? 2 : 1

                                RowLayout {
                                    id: presetCardRow
                                    anchors.fill: parent
                                    anchors.margins: 8
                                    spacing: 8

                                    QQC2.Label {
                                        text: modelData.icon
                                        font.pointSize: 14 * root.fontScale
                                    }

                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 0
                                        Layout.preferredWidth: 1
                                        spacing: 2
                                        QQC2.Label {
                                            text: modelData.title
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: isSelected
                                                ? (root.isLightTheme ? "#6b21a8" : "#f3e8ff")
                                                : Kirigami.Theme.textColor
                                            Layout.fillWidth: true
                                            elide: Text.ElideRight
                                        }
                                        QQC2.Label {
                                            text: modelData.desc
                                            font.pointSize: 10 * root.fontScale
                                            color: isSelected
                                                ? Qt.alpha(Kirigami.Theme.textColor, 0.82)
                                                : Kirigami.Theme.disabledTextColor
                                            wrapMode: Text.WordWrap
                                            Layout.fillWidth: true
                                            Layout.minimumWidth: 0
                                        }
                                    }
                                }

                                MouseArea {
                                    id: presetMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: {
                                        lightingAnimatorDialog.animPreset = modelData.id
                                        lightingAnimatorDialog.animEnabled = true
                                    }
                                }
                            }
                        }
                    }

                    // Speed & Colors Card
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: speedColorCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                        border.width: 1

                        ColumnLayout {
                            id: speedColorCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10

                                QQC2.Label {
                                    text: qsTr("Animation Speed:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 130
                                }

                                QQC2.Slider {
                                    Layout.fillWidth: true
                                    Layout.minimumWidth: 80
                                    Layout.preferredWidth: 1
                                    from: 1
                                    to: 10
                                    stepSize: 1
                                    snapMode: QQC2.Slider.SnapAlways
                                    value: lightingAnimatorDialog.animSpeed
                                    onMoved: lightingAnimatorDialog.animSpeed = Math.round(value)
                                }

                                QQC2.Label {
                                    text: qsTr("%1 / 10").arg(lightingAnimatorDialog.animSpeed)
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 48
                                }
                            }

                            // Primary Color Row
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Primary Color:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 130
                                }

                                Rectangle {
                                    width: 24
                                    height: 24
                                    radius: 4
                                    color: lightingAnimatorDialog.animColor1
                                    border.color: "#ffffff66"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    Layout.preferredWidth: 86
                                    text: lightingAnimatorDialog.animColor1
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    maximumLength: 7
                                    onTextEdited: {
                                        if (/^#[0-9A-Fa-f]{6}$/.test(text.trim())) {
                                            lightingAnimatorDialog.animColor1 = text.trim()
                                        }
                                    }
                                }

                                Repeater {
                                    model: lightingAnimatorDialog.quickColors
                                    delegate: Rectangle {
                                        required property string modelData
                                        width: 18
                                        height: 18
                                        radius: 3
                                        color: modelData
                                        border.color: lightingAnimatorDialog.animColor1.toLowerCase() === modelData.toLowerCase()
                                            ? "#ffffff"
                                            : "#00000055"
                                        border.width: lightingAnimatorDialog.animColor1.toLowerCase() === modelData.toLowerCase() ? 2 : 1
                                        MouseArea {
                                            anchors.fill: parent
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: lightingAnimatorDialog.animColor1 = modelData
                                        }
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }

                            // Secondary Color Row
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Secondary Color:")
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    Layout.preferredWidth: 130
                                }

                                Rectangle {
                                    width: 24
                                    height: 24
                                    radius: 4
                                    color: lightingAnimatorDialog.animColor2
                                    border.color: "#ffffff66"
                                    border.width: 1
                                }

                                QQC2.TextField {
                                    Layout.preferredWidth: 86
                                    text: lightingAnimatorDialog.animColor2
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    maximumLength: 7
                                    onTextEdited: {
                                        if (/^#[0-9A-Fa-f]{6}$/.test(text.trim())) {
                                            lightingAnimatorDialog.animColor2 = text.trim()
                                        }
                                    }
                                }

                                Repeater {
                                    model: lightingAnimatorDialog.quickColors
                                    delegate: Rectangle {
                                        required property string modelData
                                        width: 18
                                        height: 18
                                        radius: 3
                                        color: modelData
                                        border.color: lightingAnimatorDialog.animColor2.toLowerCase() === modelData.toLowerCase()
                                            ? "#ffffff"
                                            : "#00000055"
                                        border.width: lightingAnimatorDialog.animColor2.toLowerCase() === modelData.toLowerCase() ? 2 : 1
                                        MouseArea {
                                            anchors.fill: parent
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: lightingAnimatorDialog.animColor2 = modelData
                                        }
                                    }
                                }

                                Item { Layout.fillWidth: true }
                            }
                        }
                    }

                    // Hardware Streaming & Autonomous DuckyScript Compilation Options
                    Rectangle {
                        Layout.fillWidth: true
                        implicitHeight: hwOptsCol.implicitHeight + 16
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                        border.width: 1

                        ColumnLayout {
                            id: hwOptsCol
                            anchors.fill: parent
                            anchors.margins: 8
                            spacing: 6

                            QQC2.CheckBox {
                                id: animPushHwCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                checked: lightingAnimatorDialog.animPushToHardware
                                text: qsTr("💻 [Host App Required — Not Hardware-Portable] Stream live RGB animation frames to physical duckyPad LEDs over USB HID while DuckyDeck Studio is running")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                onToggled: lightingAnimatorDialog.animPushToHardware = checked
                                contentItem: QQC2.Label {
                                    leftPadding: (animPushHwCheck.indicator ? animPushHwCheck.indicator.width : 18) + animPushHwCheck.spacing
                                    text: animPushHwCheck.text
                                    font: animPushHwCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }

                            QQC2.CheckBox {
                                id: animCompileScriptCheck
                                Layout.fillWidth: true
                                Layout.minimumWidth: 0
                                visible: lightingAnimatorDialog.targetScope === "key" && (lightingAnimatorDialog.animTrigger === "keypress" || lightingAnimatorDialog.animTrigger === "both")
                                checked: lightingAnimatorDialog.animCompileIntoScript
                                text: qsTr("✅ [Hardware-Portable — Runs Without PC App] Embed standalone DuckyScript 3 SWC_SET / SWC_RESET animation block into Key %1's script on microSD")
                                    .arg(lightingAnimatorDialog.targetKeySlot)
                                font.pointSize: 10 * root.fontScale
                                onToggled: lightingAnimatorDialog.animCompileIntoScript = checked
                                contentItem: QQC2.Label {
                                    leftPadding: (animCompileScriptCheck.indicator ? animCompileScriptCheck.indicator.width : 18) + animCompileScriptCheck.spacing
                                    text: animCompileScriptCheck.text
                                    font: animCompileScriptCheck.font
                                    color: Kirigami.Theme.textColor
                                    verticalAlignment: Text.AlignVCenter
                                    wrapMode: Text.WordWrap
                                }
                            }
                        }
                    }

                    // Status feedback banner
                    Rectangle {
                        visible: lightingAnimatorDialog.statusBannerText.length > 0
                        Layout.fillWidth: true
                        implicitHeight: statusLbl.implicitHeight + 12
                        radius: 5
                        color: Qt.alpha("#22c55e", 0.16)
                        border.color: "#22c55e"
                        border.width: 1

                        QQC2.Label {
                            id: statusLbl
                            anchors.fill: parent
                            anchors.margins: 6
                            text: lightingAnimatorDialog.statusBannerText
                            color: "#86efac"
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            wrapMode: Text.WordWrap
                        }
                    }
                }
            }

            // Bottom Action Bar
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    text: qsTr("🗑 Disable / Clear Effect")
                    onClicked: {
                        lightingAnimatorDialog.animEnabled = false
                        const cfg = lightingAnimatorDialog.currentConfigObject()
                        root.saveLightingAnimationConfig(
                            lightingAnimatorDialog.targetScope,
                            lightingAnimatorDialog.targetKeySlot,
                            cfg
                        )
                        lightingAnimatorDialog.statusBannerText = qsTr("Cleared animation effect for %1.")
                            .arg(lightingAnimatorDialog.targetScope === "key"
                                ? qsTr("Key %1").arg(lightingAnimatorDialog.targetKeySlot)
                                : qsTr("Profile #%1").arg(deviceController.activeProfile))
                    }
                }

                QQC2.Button {
                    text: qsTr("▶ Preview Burst Now")
                    onClicked: {
                        lightingAnimatorDialog.animEnabled = true
                        const cfg = lightingAnimatorDialog.currentConfigObject()
                        root.saveLightingAnimationConfig(
                            lightingAnimatorDialog.targetScope,
                            lightingAnimatorDialog.targetKeySlot,
                            cfg
                        )
                        const previewSlot = lightingAnimatorDialog.targetScope === "key"
                            ? lightingAnimatorDialog.targetKeySlot
                            : (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20 ? root.selectedSlotNumber : 8)
                        hardwareCanvas.triggerKeypressBurst(previewSlot, cfg, lightingAnimatorDialog.targetScope)
                        lightingAnimatorDialog.statusBannerText = qsTr("Previewing '%1' on Hardware Canvas%2!")
                            .arg(lightingAnimatorDialog.animPreset)
                            .arg(deviceController.connected && lightingAnimatorDialog.animPushToHardware ? qsTr(" and physical duckyPad") : "")
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    text: qsTr("Close")
                    onClicked: lightingAnimatorDialog.close()
                }

                QQC2.Button {
                    text: qsTr("💾 Save & Apply Animation")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        const cfg = lightingAnimatorDialog.currentConfigObject()
                        root.saveLightingAnimationConfig(
                            lightingAnimatorDialog.targetScope,
                            lightingAnimatorDialog.targetKeySlot,
                            cfg
                        )
                        const previewSlot = lightingAnimatorDialog.targetScope === "key"
                            ? lightingAnimatorDialog.targetKeySlot
                            : (root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20 ? root.selectedSlotNumber : 8)
                        if (cfg.enabled) {
                            hardwareCanvas.triggerKeypressBurst(previewSlot, cfg, lightingAnimatorDialog.targetScope)
                        }
                        lightingAnimatorDialog.statusBannerText = qsTr("Saved lighting animation for %1.")
                            .arg(lightingAnimatorDialog.targetScope === "key"
                                ? qsTr("Key %1").arg(lightingAnimatorDialog.targetKeySlot)
                                : qsTr("Profile #%1 (%2)").arg(deviceController.activeProfile).arg(deviceController.activeProfileName))
                    }
                }
            }
        }
    }

    // =========================================================================
    // Live Macro Recorder Window (Creates DuckyScript 3 with Exact Timings,
    // As Fast As Possible, Fixed Delay, Keyboard Keys & Mouse Moves/Clicks)
    // =========================================================================
    Window {
        id: macroRecorderDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: macroRecorderDialog.visible
            onActivated: { if (macroRecorderDialog.isRecording) macroRecorderDialog.stopRecording(); macroRecorderDialog.close() }
            onAmbiguouslyActivated: { if (macroRecorderDialog.isRecording) macroRecorderDialog.stopRecording(); macroRecorderDialog.close() }
        }
        title: qsTr("Live Macro Recorder (DuckyScript 3) — DuckyDeck Studio")
        visible: false
        transientParent: root
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint
        width: Math.max(840, minimumWidth)
        height: 680
        minimumWidth: 780
        minimumHeight: 520
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property bool isRecording: false
        property bool globalEvdevActive: false
        property string timingMode: "exact" // "exact" | "fast" | "fixed"
        property int fixedDelayMs: 35
        property int minDelayThresholdMs: 12
        property bool captureKeyboard: true
        property bool captureMouseClicks: true
        property bool captureMouseMoves: false
        property var rawEvents: []
        property double recordingStartMs: 0
        property double lastInputMs: 0
        property real lastPadMouseX: -1
        property real lastPadMouseY: -1
        property double lastEvdevEventMs: 0

        function openRecorder() {
            macroRecorderDialog.show()
            macroRecorderDialog.raise()
            macroRecorderDialog.requestActivate()
            captureFocusPad.forceActiveFocus()
        }

        function startRecordingSession() {
            macroRecorderDialog.rawEvents = []
            macroRecorderDialog.recordingStartMs = Date.now()
            macroRecorderDialog.lastInputMs = macroRecorderDialog.recordingStartMs
            macroRecorderDialog.lastPadMouseX = -1
            macroRecorderDialog.lastPadMouseY = -1
            macroRecorderDialog.isRecording = true
            macroRecorderDialog.globalEvdevActive = deviceController.startGlobalMacroCapture(
                macroRecorderDialog.captureMouseMoves,
                macroRecorderDialog.captureMouseClicks
            )
            evdevPollTimer.start()
            macroRecorderDialog.rebuildGeneratedDuckyScript()
            captureFocusPad.forceActiveFocus()
        }

        function stopRecordingSession() {
            if (!macroRecorderDialog.isRecording) return
            evdevPollTimer.stop()
            macroRecorderDialog.pollEvdevOnce()
            deviceController.stopGlobalMacroCapture()
            macroRecorderDialog.isRecording = false
            macroRecorderDialog.globalEvdevActive = false
            macroRecorderDialog.rebuildGeneratedDuckyScript()
        }

        onClosing: {
            if (macroRecorderDialog.isRecording) {
                macroRecorderDialog.stopRecordingSession()
            }
        }

        function qtKeyToDuckyName(key, text) {
            switch (key) {
                case Qt.Key_Return:
                case Qt.Key_Enter: return "ENTER"
                case Qt.Key_Escape: return "ESC"
                case Qt.Key_Backspace: return "BACKSPACE"
                case Qt.Key_Tab: return "TAB"
                case Qt.Key_Space: return "SPACE"
                case Qt.Key_CapsLock: return "CAPSLOCK"
                case Qt.Key_Print: return "PRINTSCREEN"
                case Qt.Key_ScrollLock: return "SCROLLLOCK"
                case Qt.Key_Pause: return "PAUSE"
                case Qt.Key_Insert: return "INSERT"
                case Qt.Key_Home: return "HOME"
                case Qt.Key_PageUp: return "PAGEUP"
                case Qt.Key_Delete: return "DELETE"
                case Qt.Key_End: return "END"
                case Qt.Key_PageDown: return "PAGEDOWN"
                case Qt.Key_Right: return "RIGHT"
                case Qt.Key_Left: return "LEFT"
                case Qt.Key_Down: return "DOWN"
                case Qt.Key_Up: return "UP"
                case Qt.Key_NumLock: return "NUMLOCK"
                case Qt.Key_Menu: return "MENU"
                case Qt.Key_Shift: return "SHIFT"
                case Qt.Key_Control: return "CTRL"
                case Qt.Key_Alt: return "ALT"
                case Qt.Key_AltGr: return "RALT"
                case Qt.Key_Meta:
                case Qt.Key_Super_L:
                case Qt.Key_Super_R: return "WIN"
                case Qt.Key_F1: return "F1"
                case Qt.Key_F2: return "F2"
                case Qt.Key_F3: return "F3"
                case Qt.Key_F4: return "F4"
                case Qt.Key_F5: return "F5"
                case Qt.Key_F6: return "F6"
                case Qt.Key_F7: return "F7"
                case Qt.Key_F8: return "F8"
                case Qt.Key_F9: return "F9"
                case Qt.Key_F10: return "F10"
                case Qt.Key_F11: return "F11"
                case Qt.Key_F12: return "F12"
                case Qt.Key_F13: return "F13"
                case Qt.Key_F14: return "F14"
                case Qt.Key_F15: return "F15"
                case Qt.Key_F16: return "F16"
                case Qt.Key_F17: return "F17"
                case Qt.Key_F18: return "F18"
                case Qt.Key_F19: return "F19"
                case Qt.Key_F20: return "F20"
                case Qt.Key_F21: return "F21"
                case Qt.Key_F22: return "F22"
                case Qt.Key_F23: return "F23"
                case Qt.Key_F24: return "F24"
                case Qt.Key_Minus: return "MINUS"
                case Qt.Key_Equal: return "EQUAL"
                case Qt.Key_BracketLeft: return "LEFTBRACE"
                case Qt.Key_BracketRight: return "RIGHTBRACE"
                case Qt.Key_Backslash: return "BACKSLASH"
                case Qt.Key_Semicolon: return "SEMICOLON"
                case Qt.Key_Apostrophe: return "APOSTROPHE"
                case Qt.Key_QuoteLeft: return "GRAVE"
                case Qt.Key_Comma: return "COMMA"
                case Qt.Key_Period: return "DOT"
                case Qt.Key_Slash: return "SLASH"
                default: break
            }
            if (key >= Qt.Key_A && key <= Qt.Key_Z) {
                return String.fromCharCode(97 + (key - Qt.Key_A))
            }
            if (key >= Qt.Key_0 && key <= Qt.Key_9) {
                return String.fromCharCode(48 + (key - Qt.Key_0))
            }
            if (text && text.length === 1) {
                const c = text.toLowerCase()
                if (/[a-z0-9]/.test(c)) return c
            }
            return ""
        }

        function appendRecordedEvent(evt) {
            let list = macroRecorderDialog.rawEvents.slice()
            // Coalesce consecutive mouse_move events within 25ms
            if (evt.kind === "mouse_move" && list.length > 0) {
                let prev = list[list.length - 1]
                if (prev.kind === "mouse_move" && (evt.elapsed_ms - prev.elapsed_ms) <= 25) {
                    prev.dx = (prev.dx || 0) + (evt.dx || 0)
                    prev.dy = (prev.dy || 0) + (evt.dy || 0)
                    prev.elapsed_ms = evt.elapsed_ms
                    list[list.length - 1] = prev
                    macroRecorderDialog.rawEvents = list
                    macroRecorderDialog.rebuildGeneratedDuckyScript()
                    return
                }
            }
            list.push(evt)
            macroRecorderDialog.rawEvents = list
            macroRecorderDialog.rebuildGeneratedDuckyScript()
        }

        function pollEvdevOnce() {
            if (!macroRecorderDialog.isRecording) return
            const jsonStr = deviceController.pollGlobalMacroEventsJson()
            if (!jsonStr || jsonStr === "[]") return
            try {
                const arr = JSON.parse(jsonStr)
                if (!Array.isArray(arr) || arr.length === 0) return
                let list = macroRecorderDialog.rawEvents.slice()
                for (let i = 0; i < arr.length; i++) {
                    const e = arr[i]
                    if ((e.kind === "key_down" || e.kind === "key_up") && !macroRecorderDialog.captureKeyboard) continue
                    if ((e.kind === "mouse_click" || e.kind === "mouse_wheel") && !macroRecorderDialog.captureMouseClicks) continue
                    if (e.kind === "mouse_move" && !macroRecorderDialog.captureMouseMoves) continue
                    macroRecorderDialog.lastEvdevEventMs = Date.now()
                    if (e.kind === "mouse_move" && list.length > 0 && list[list.length - 1].kind === "mouse_move" && (e.elapsed_ms - list[list.length - 1].elapsed_ms) <= 25) {
                        list[list.length - 1].dx += e.dx
                        list[list.length - 1].dy += e.dy
                        list[list.length - 1].elapsed_ms = e.elapsed_ms
                    } else {
                        list.push(e)
                    }
                }
                macroRecorderDialog.rawEvents = list
                macroRecorderDialog.rebuildGeneratedDuckyScript()
            } catch (err) {}
        }

        Timer {
            id: evdevPollTimer
            interval: 40
            repeat: true
            running: false
            onTriggered: macroRecorderDialog.pollEvdevOnce()
        }

        function rebuildGeneratedDuckyScript() {
            const events = macroRecorderDialog.rawEvents
            if (!events || events.length === 0) {
                recordedScriptEditor.text = qsTr("// Click '🔴 Start Recording' above, then type keys or use the mouse.\n// Your recorded DuckyScript 3 macro will appear here in real time.")
                return
            }

            let lines = []
            const modeLabel = macroRecorderDialog.timingMode === "exact"
                ? "Exact Timings"
                : (macroRecorderDialog.timingMode === "fast" ? "As Fast As Possible" : ("Fixed Delay (" + macroRecorderDialog.fixedDelayMs + "ms)"))
            lines.push("// Recorded Macro (" + modeLabel + " • " + events.length + " events)")

            let prevElapsed = events[0].elapsed_ms || 0

            for (let i = 0; i < events.length; i++) {
                const ev = events[i]
                const dt = Math.max(0, Math.round((ev.elapsed_ms || 0) - prevElapsed))
                prevElapsed = ev.elapsed_ms || prevElapsed

                if (i > 0) {
                    if (macroRecorderDialog.timingMode === "exact") {
                        if (dt >= macroRecorderDialog.minDelayThresholdMs) {
                            lines.push("DELAY " + dt)
                        }
                    } else if (macroRecorderDialog.timingMode === "fixed") {
                        if (macroRecorderDialog.fixedDelayMs > 0) {
                            lines.push("DELAY " + macroRecorderDialog.fixedDelayMs)
                        }
                    }
                }

                if (ev.kind === "custom_delay") {
                    lines.push("DELAY " + Math.max(1, Math.round(ev.dx || 100)))
                } else if (ev.kind === "key_down") {
                    const nextEv = (i + 1 < events.length) ? events[i + 1] : null
                    const holdMs = nextEv ? Math.max(0, Math.round((nextEv.elapsed_ms || 0) - (ev.elapsed_ms || 0))) : 0
                    if (nextEv && nextEv.kind === "key_up" && nextEv.key === ev.key &&
                        (macroRecorderDialog.timingMode !== "exact" || holdMs < 95)) {
                        lines.push(ev.key)
                        prevElapsed = nextEv.elapsed_ms || prevElapsed
                        i++
                    } else {
                        lines.push("KEYDOWN " + ev.key)
                    }
                } else if (ev.kind === "key_up") {
                    lines.push("KEYUP " + ev.key)
                } else if (ev.kind === "mouse_click") {
                    lines.push(ev.key || "LMOUSE")
                } else if (ev.kind === "mouse_wheel") {
                    lines.push("MOUSE_SCROLL " + (ev.dx || 0) + " " + (ev.dy || 0))
                } else if (ev.kind === "mouse_move") {
                    let remX = Math.round(ev.dx || 0)
                    let remY = Math.round(ev.dy || 0)
                    while (remX !== 0 || remY !== 0) {
                        const stepX = Math.max(-127, Math.min(127, remX))
                        const stepY = Math.max(-127, Math.min(127, -remY))
                        lines.push("MOUSE_MOVE " + stepX + " " + stepY)
                        remX -= stepX
                        remY -= (-stepY)
                    }
                }
            }

            recordedScriptEditor.text = lines.join("\n")
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 10

            // Header Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Rectangle {
                    width: 34
                    height: 34
                    radius: 8
                    color: macroRecorderDialog.isRecording ? Qt.alpha("#ef4444", 0.25) : Qt.alpha("#38bdf8", 0.18)
                    border.color: macroRecorderDialog.isRecording ? "#ef4444" : "#38bdf8"
                    border.width: 1
                    QQC2.Label {
                        anchors.centerIn: parent
                        text: macroRecorderDialog.isRecording ? "🔴" : "⏺"
                        font.pointSize: 13 * root.fontScale
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2
                    QQC2.Label {
                        text: qsTr("DuckyScript 3 Live Macro Recorder")
                        font.bold: true
                        font.pointSize: 12 * root.fontScale
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }
                    QQC2.Label {
                        text: qsTr("Record keyboard strokes, key holds, mouse clicks, scroll wheel, and mouse movements directly into DuckyScript 3.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                    }
                }

                // Start / Stop Recording Button
                QQC2.Button {
                    text: macroRecorderDialog.isRecording
                        ? qsTr("⏹ Stop Recording (%1)").arg(macroRecorderDialog.rawEvents.length)
                        : qsTr("🔴 Start Recording")
                    highlighted: true
                    font.bold: true
                    implicitHeight: 34
                    onClicked: {
                        if (macroRecorderDialog.isRecording) {
                            macroRecorderDialog.stopRecordingSession()
                        } else {
                            macroRecorderDialog.startRecordingSession()
                        }
                    }
                }

                QQC2.Button {
                    text: qsTr("🧹 Clear")
                    enabled: macroRecorderDialog.rawEvents.length > 0 && !macroRecorderDialog.isRecording
                    implicitHeight: 34
                    onClicked: {
                        macroRecorderDialog.rawEvents = []
                        macroRecorderDialog.rebuildGeneratedDuckyScript()
                    }
                }
            }

            // Timing & Capture Filters Card
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: recOptionsCol.implicitHeight + 16
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
                border.width: 1

                ColumnLayout {
                    id: recOptionsCol
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 8

                    // Row 1: Timing Mode Options
                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Timing Mode:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            height: 28
                            verticalAlignment: Text.AlignVCenter
                        }

                        QQC2.Button {
                            text: qsTr("⏱ Exact Timings (Real Delays)")
                            highlighted: macroRecorderDialog.timingMode === "exact"
                            font.bold: macroRecorderDialog.timingMode === "exact"
                            onClicked: {
                                macroRecorderDialog.timingMode = "exact"
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Inserts DELAY <ms> commands matching the exact millisecond intervals between your recorded actions")
                        }

                        QQC2.Button {
                            text: qsTr("⚡ As Fast As Possible (No Delays)")
                            highlighted: macroRecorderDialog.timingMode === "fast"
                            font.bold: macroRecorderDialog.timingMode === "fast"
                            onClicked: {
                                macroRecorderDialog.timingMode = "fast"
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Omits all DELAY commands between actions and collapses quick key taps so the macro runs at maximum hardware speed")
                        }

                        QQC2.Button {
                            text: qsTr("📏 Fixed Step Delay")
                            highlighted: macroRecorderDialog.timingMode === "fixed"
                            font.bold: macroRecorderDialog.timingMode === "fixed"
                            onClicked: {
                                macroRecorderDialog.timingMode = "fixed"
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                        }

                        QQC2.SpinBox {
                            visible: macroRecorderDialog.timingMode === "fixed"
                            from: 5
                            to: 2000
                            stepSize: 5
                            value: macroRecorderDialog.fixedDelayMs
                            onValueModified: {
                                macroRecorderDialog.fixedDelayMs = value
                                macroRecorderDialog.rebuildGeneratedDuckyScript()
                            }
                        }

                        QQC2.Label {
                            visible: macroRecorderDialog.timingMode === "fixed"
                            text: qsTr("ms")
                            font.pointSize: 10 * root.fontScale
                            height: 28
                            verticalAlignment: Text.AlignVCenter
                            color: Kirigami.Theme.disabledTextColor
                        }
                    }

                    // Row 2: Input Sources (Keyboard, Mouse Clicks/Scroll, Mouse Movements)
                    Flow {
                        Layout.fillWidth: true
                        spacing: 10

                        QQC2.Label {
                            text: qsTr("Include Inputs:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            height: 26
                            verticalAlignment: Text.AlignVCenter
                        }

                        QQC2.CheckBox {
                            checked: macroRecorderDialog.captureKeyboard
                            text: qsTr("⌨ Keyboard Keys (Taps & Holds)")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            onToggled: macroRecorderDialog.captureKeyboard = checked
                        }

                        QQC2.CheckBox {
                            checked: macroRecorderDialog.captureMouseClicks
                            text: qsTr("🖱 Mouse Clicks & Scroll")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            onToggled: macroRecorderDialog.captureMouseClicks = checked
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Captures LMOUSE, RMOUSE, MMOUSE, and MOUSE_SCROLL events")
                        }

                        QQC2.CheckBox {
                            checked: macroRecorderDialog.captureMouseMoves
                            text: qsTr("📐 Mouse Moves (MOUSE_MOVE)")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            onToggled: macroRecorderDialog.captureMouseMoves = checked
                        }
                    }
                }
            }

            // Interactive Capture Area (Captures Keyboard + Mouse inside Window & Shows Global Evdev Status)
            Rectangle {
                id: captureFocusPad
                Layout.fillWidth: true
                Layout.preferredHeight: 96
                radius: 8
                focus: true
                activeFocusOnTab: true
                color: macroRecorderDialog.isRecording
                    ? Qt.alpha("#ef4444", 0.12)
                    : Qt.alpha(Kirigami.Theme.highlightColor, 0.08)
                border.color: macroRecorderDialog.isRecording
                    ? "#ef4444"
                    : (captureFocusPad.activeFocus ? Kirigami.Theme.highlightColor : Qt.alpha(Kirigami.Theme.textColor, 0.25))
                border.width: macroRecorderDialog.isRecording ? 2 : 1

                Keys.onPressed: (event) => {
                    if (!macroRecorderDialog.isRecording || event.isAutoRepeat) return
                    event.accepted = true
                    if (!macroRecorderDialog.captureKeyboard) return
                    if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                    const kName = macroRecorderDialog.qtKeyToDuckyName(event.key, event.text)
                    if (kName.length > 0) {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "key_down",
                            key: kName,
                            dx: 0,
                            dy: 0,
                            elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                        })
                    }
                }

                Keys.onReleased: (event) => {
                    if (!macroRecorderDialog.isRecording || event.isAutoRepeat) return
                    event.accepted = true
                    if (!macroRecorderDialog.captureKeyboard) return
                    if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                    const kName = macroRecorderDialog.qtKeyToDuckyName(event.key, event.text)
                    if (kName.length > 0) {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "key_up",
                            key: kName,
                            dx: 0,
                            dy: 0,
                            elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                        })
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    hoverEnabled: true
                    acceptedButtons: Qt.AllButtons
                    cursorShape: macroRecorderDialog.isRecording ? Qt.CrossCursor : Qt.PointingHandCursor

                    onPressed: (mouse) => {
                        captureFocusPad.forceActiveFocus()
                        macroRecorderDialog.lastPadMouseX = mouse.x
                        macroRecorderDialog.lastPadMouseY = mouse.y
                        if (!macroRecorderDialog.isRecording || !macroRecorderDialog.captureMouseClicks) return
                        if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                        let btnName = "LMOUSE"
                        if (mouse.button === Qt.RightButton) btnName = "RMOUSE"
                        else if (mouse.button === Qt.MiddleButton) btnName = "MMOUSE"
                        else if (mouse.button === Qt.BackButton) btnName = "BMOUSE"
                        else if (mouse.button === Qt.ForwardButton) btnName = "FMOUSE"
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "mouse_click",
                            key: btnName,
                            dx: 0,
                            dy: 0,
                            elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                        })
                    }

                    onWheel: (wheel) => {
                        if (!macroRecorderDialog.isRecording || !macroRecorderDialog.captureMouseClicks) return
                        if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                        const vStep = wheel.angleDelta.y > 0 ? 1 : (wheel.angleDelta.y < 0 ? -1 : 0)
                        const hStep = wheel.angleDelta.x > 0 ? 1 : (wheel.angleDelta.x < 0 ? -1 : 0)
                        if (vStep !== 0 || hStep !== 0) {
                            macroRecorderDialog.appendRecordedEvent({
                                kind: "mouse_wheel",
                                key: "MOUSE_SCROLL",
                                dx: hStep,
                                dy: vStep,
                                elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                            })
                        }
                    }

                    onPositionChanged: (mouse) => {
                        if (!macroRecorderDialog.isRecording || !macroRecorderDialog.captureMouseMoves) {
                            macroRecorderDialog.lastPadMouseX = mouse.x
                            macroRecorderDialog.lastPadMouseY = mouse.y
                            return
                        }
                        if (macroRecorderDialog.globalEvdevActive && (Date.now() - macroRecorderDialog.lastEvdevEventMs) < 35) return
                        if (macroRecorderDialog.lastPadMouseX >= 0 && macroRecorderDialog.lastPadMouseY >= 0) {
                            const dx = Math.round(mouse.x - macroRecorderDialog.lastPadMouseX)
                            const dy = Math.round(mouse.y - macroRecorderDialog.lastPadMouseY)
                            if (Math.abs(dx) >= 2 || Math.abs(dy) >= 2) {
                                macroRecorderDialog.lastPadMouseX = mouse.x
                                macroRecorderDialog.lastPadMouseY = mouse.y
                                macroRecorderDialog.appendRecordedEvent({
                                    kind: "mouse_move",
                                    key: "MOUSE_MOVE",
                                    dx: dx,
                                    dy: dy,
                                    elapsed_ms: Math.max(0, Date.now() - macroRecorderDialog.recordingStartMs)
                                })
                            }
                        } else {
                            macroRecorderDialog.lastPadMouseX = mouse.x
                            macroRecorderDialog.lastPadMouseY = mouse.y
                        }
                    }
                }

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 4

                    Item { Layout.fillHeight: true }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        horizontalAlignment: Text.AlignHCenter
                        wrapMode: Text.WordWrap
                        text: macroRecorderDialog.isRecording
                            ? qsTr("🔴 RECORDING LIVE — Type keys, click, scroll, or move the mouse here%1")
                                .arg(macroRecorderDialog.globalEvdevActive ? qsTr(" (or anywhere on desktop via evdev)") : "")
                            : qsTr("Click '🔴 Start Recording' then type keys or use mouse inside this Capture Pad")
                        font.bold: true
                        font.pointSize: 11 * root.fontScale
                        color: macroRecorderDialog.isRecording ? "#fca5a5" : Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                        horizontalAlignment: Text.AlignHCenter
                        elide: Text.ElideRight
                        text: qsTr("Captured Events: %1 • Mode: %2")
                            .arg(macroRecorderDialog.rawEvents.length)
                            .arg(macroRecorderDialog.timingMode === "exact"
                                ? qsTr("Exact Timings")
                                : (macroRecorderDialog.timingMode === "fast" ? qsTr("As Fast As Possible") : qsTr("Fixed %1ms Delay").arg(macroRecorderDialog.fixedDelayMs)))
                        font.family: "monospace"
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    Item { Layout.fillHeight: true }
                }
            }

            // Generated DuckyScript 3 Preview Header + Quick Manual Insertions
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("Generated DuckyScript 3 Output (Editable):")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                    Layout.fillWidth: true
                    elide: Text.ElideRight
                }

                QQC2.Button {
                    text: qsTr("+ DELAY 100")
                    implicitHeight: 26
                    font.pointSize: 10 * root.fontScale
                    onClicked: {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "custom_delay",
                            key: "DELAY",
                            dx: 100,
                            dy: 0,
                            elapsed_ms: macroRecorderDialog.rawEvents.length > 0
                                ? (macroRecorderDialog.rawEvents[macroRecorderDialog.rawEvents.length - 1].elapsed_ms || 0)
                                : 0
                        })
                    }
                }

                QQC2.Button {
                    text: qsTr("+ DELAY 500")
                    implicitHeight: 26
                    font.pointSize: 10 * root.fontScale
                    onClicked: {
                        macroRecorderDialog.appendRecordedEvent({
                            kind: "custom_delay",
                            key: "DELAY",
                            dx: 500,
                            dy: 0,
                            elapsed_ms: macroRecorderDialog.rawEvents.length > 0
                                ? (macroRecorderDialog.rawEvents[macroRecorderDialog.rawEvents.length - 1].elapsed_ms || 0)
                                : 0
                        })
                    }
                }
            }

            // Scrollable DuckyScript Preview Editor
            QQC2.ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 80
                Layout.preferredHeight: 1
                clip: true
                contentWidth: availableWidth
                QQC2.ScrollBar.vertical: HighContrastScrollBar {}
                QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                    orientation: Qt.Horizontal
                }

                QQC2.TextArea {
                    id: recordedScriptEditor
                    width: parent.width
                    font.family: "monospace"
                    font.pointSize: 10 * root.fontScale
                    selectByMouse: true
                    wrapMode: TextEdit.Wrap
                    clip: true
                    text: qsTr("// Click '🔴 Start Recording' above, then type keys or use the mouse.\n// Your recorded DuckyScript 3 macro will appear here in real time.")
                    background: Rectangle {
                        radius: 6
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                        border.width: 1
                    }
                }
            }

            // Footer Actions (Insert into Key Script / Replace Key Script)
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: root.hasSelectedSlot
                        ? qsTr("Target: %1 (%2)")
                            .arg(root.selectedSlotTitle)
                            .arg(root.editingReleaseScript ? qsTr("On Release") : qsTr("On Press"))
                        : qsTr("Select a key on the canvas to insert directly")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    elide: Text.ElideRight
                }

                QQC2.Button {
                    text: qsTr("Close")
                    onClicked: macroRecorderDialog.close()
                }

                QQC2.Button {
                    text: qsTr("📥 Append to Key Script")
                    enabled: root.hasSelectedSlot && recordedScriptEditor.text.trim().length > 0
                    onClicked: {
                        if (macroRecorderDialog.isRecording) {
                            macroRecorderDialog.stopRecordingSession()
                        }
                        const cur = scriptEditor.text.trim()
                        scriptEditor.text = cur.length > 0
                            ? (cur + "\n\n" + recordedScriptEditor.text.trim() + "\n")
                            : (recordedScriptEditor.text.trim() + "\n")
                        root.inspectorMode = "code"
                        macroRecorderDialog.close()
                    }
                }

                QQC2.Button {
                    text: qsTr("⚡ Replace Key Script")
                    highlighted: true
                    font.bold: true
                    enabled: root.hasSelectedSlot && recordedScriptEditor.text.trim().length > 0
                    onClicked: {
                        if (macroRecorderDialog.isRecording) {
                            macroRecorderDialog.stopRecordingSession()
                        }
                        scriptEditor.text = recordedScriptEditor.text.trim() + "\n"
                        root.inspectorMode = "code"
                        macroRecorderDialog.close()
                    }
                }
            }
        }
    }

    // =========================================================================
    // OLED Button Icon & Symbol Picker Window
    // =========================================================================
    Window {
        id: oledIconPickerDialog
        readonly property bool closableWithEscape: true
        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: oledIconPickerDialog.visible
            onActivated: oledIconPickerDialog.close()
            onAmbiguouslyActivated: oledIconPickerDialog.close()
        }
        title: qsTr("OLED Button Icon Picker — %1").arg(root.selectedSlotBadge)
        width: 680
        height: 540
        minimumWidth: 540
        minimumHeight: 420
        modality: Qt.NonModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint
        color: Kirigami.Theme.backgroundColor

        // "inspector" updates label1Input/label2Input directly; "keyProps" updates keyPropLabel1Field/keyPropLabel2Field
        property string targetSource: "inspector"
        property int placementMode: 0 // 0 = Icon (L1) + Short Label (L2), 1 = Big Icon Only (L1, clear L2), 2 = Set Line 1, 3 = Set Line 2, 4 = Prepend to Line 1
        property string selectedCategory: "All"
        property string searchQuery: ""

        readonly property bool targetSupportsTwoLines: targetSource === "keyProps"
            ? (keyPropertiesWindow.slotNum <= 20)
            : (root.selectedSlotNumber <= 20)

        readonly property var categories: [
            "All",
            "Media & Audio",
            "Arrows & Nav",
            "Editing & Clipboard",
            "Coding & Terminal",
            "Window & System",
            "Streaming & Apps",
            "Symbols & Shapes"
        ]

        readonly property var iconCatalog: [
            // Media & Audio
            { icon: "▶", label: "PLAY", name: "Play", cat: "Media & Audio" },
            { icon: "⏸", label: "PAUSE", name: "Pause", cat: "Media & Audio" },
            { icon: "⏯", label: "PLAY", name: "Play / Pause", cat: "Media & Audio" },
            { icon: "⏹", label: "STOP", name: "Stop", cat: "Media & Audio" },
            { icon: "⏭", label: "NEXT", name: "Next Track", cat: "Media & Audio" },
            { icon: "⏮", label: "PREV", name: "Prev Track", cat: "Media & Audio" },
            { icon: "⏩", label: "FFWD", name: "Fast Forward", cat: "Media & Audio" },
            { icon: "⏪", label: "RWND", name: "Rewind", cat: "Media & Audio" },
            { icon: "🔊", label: "VOL+", name: "Volume Up", cat: "Media & Audio" },
            { icon: "🔉", label: "VOL-", name: "Volume Down", cat: "Media & Audio" },
            { icon: "🔇", label: "MUTE", name: "Mute Audio", cat: "Media & Audio" },
            { icon: "🎤", label: "MIC", name: "Microphone", cat: "Media & Audio" },
            { icon: "🎧", label: "HEAD", name: "Headphones", cat: "Media & Audio" },
            { icon: "🎵", label: "MUSIC", name: "Music Note", cat: "Media & Audio" },
            { icon: "🎶", label: "SONG", name: "Music Notes", cat: "Media & Audio" },
            { icon: "⏺", label: "REC", name: "Record", cat: "Media & Audio" },

            // Arrows & Nav
            { icon: "↑", label: "UP", name: "Arrow Up", cat: "Arrows & Nav" },
            { icon: "↓", label: "DOWN", name: "Arrow Down", cat: "Arrows & Nav" },
            { icon: "←", label: "LEFT", name: "Arrow Left", cat: "Arrows & Nav" },
            { icon: "→", label: "RIGHT", name: "Arrow Right", cat: "Arrows & Nav" },
            { icon: "↖", label: "HOME", name: "Home / Top-Left", cat: "Arrows & Nav" },
            { icon: "↘", label: "END", name: "End / Bot-Right", cat: "Arrows & Nav" },
            { icon: "▲", label: "PGUP", name: "Triangle Up / Page Up", cat: "Arrows & Nav" },
            { icon: "▼", label: "PGDN", name: "Triangle Down / Page Down", cat: "Arrows & Nav" },
            { icon: "◀", label: "BACK", name: "Triangle Left / Back", cat: "Arrows & Nav" },
            { icon: "▶", label: "FWD", name: "Triangle Right / Forward", cat: "Arrows & Nav" },
            { icon: "↩", label: "UNDO", name: "Return / Undo", cat: "Arrows & Nav" },
            { icon: "↪", label: "REDO", name: "Forward / Redo", cat: "Arrows & Nav" },
            { icon: "↺", label: "RESET", name: "Counterclockwise / Reset", cat: "Arrows & Nav" },
            { icon: "↻", label: "SYNC", name: "Clockwise / Reload", cat: "Arrows & Nav" },
            { icon: "⇄", label: "SWAP", name: "Swap Left/Right", cat: "Arrows & Nav" },
            { icon: "⇅", label: "SORT", name: "Swap Up/Down", cat: "Arrows & Nav" },

            // Editing & Clipboard
            { icon: "✂", label: "CUT", name: "Cut / Scissors", cat: "Editing & Clipboard" },
            { icon: "📋", label: "COPY", name: "Copy / Clipboard", cat: "Editing & Clipboard" },
            { icon: "📌", label: "PASTE", name: "Pin / Paste", cat: "Editing & Clipboard" },
            { icon: "💾", label: "SAVE", name: "Save / Floppy", cat: "Editing & Clipboard" },
            { icon: "📁", label: "OPEN", name: "Folder / Open", cat: "Editing & Clipboard" },
            { icon: "📄", label: "NEW", name: "New File / Document", cat: "Editing & Clipboard" },
            { icon: "🔍", label: "FIND", name: "Search / Zoom", cat: "Editing & Clipboard" },
            { icon: "🗑", label: "DEL", name: "Delete / Trash", cat: "Editing & Clipboard" },
            { icon: "✏", label: "EDIT", name: "Edit / Pencil", cat: "Editing & Clipboard" },
            { icon: "🔖", label: "MARK", name: "Bookmark", cat: "Editing & Clipboard" },
            { icon: "📷", label: "SHOT", name: "Screenshot / Camera", cat: "Editing & Clipboard" },
            { icon: "🔗", label: "LINK", name: "Link / URL", cat: "Editing & Clipboard" },

            // Coding & Terminal
            { icon: "⌨", label: "TERM", name: "Keyboard / Terminal", cat: "Coding & Terminal" },
            { icon: "⚡", label: "BUILD", name: "Build / Flash / Fast", cat: "Coding & Terminal" },
            { icon: "🐞", label: "DEBUG", name: "Debug / Bug", cat: "Coding & Terminal" },
            { icon: "🚀", label: "RUN", name: "Run / Deploy / Rocket", cat: "Coding & Terminal" },
            { icon: "🧪", label: "TEST", name: "Unit Test / Lab", cat: "Coding & Terminal" },
            { icon: "🔧", label: "TOOL", name: "Wrench / Config", cat: "Coding & Terminal" },
            { icon: "⚙", label: "PREF", name: "Settings / Gear", cat: "Coding & Terminal" },
            { icon: "📦", label: "PKG", name: "Package / Cargo", cat: "Coding & Terminal" },
            { icon: "🌿", label: "GIT", name: "Branch / Git", cat: "Coding & Terminal" },
            { icon: "☁", label: "PUSH", name: "Cloud / Push", cat: "Coding & Terminal" },
            { icon: "🤖", label: "AI", name: "AI / Bot / Agent", cat: "Coding & Terminal" },
            { icon: "⌘", label: "CMD", name: "Command / Super Key", cat: "Coding & Terminal" },

            // Window & System
            { icon: "🖥", label: "DESK", name: "Desktop / Monitor", cat: "Window & System" },
            { icon: "🪟", label: "WIN", name: "Window / Tile", cat: "Window & System" },
            { icon: "🔒", label: "LOCK", name: "Lock Screen", cat: "Window & System" },
            { icon: "🔓", label: "UNLCK", name: "Unlock", cat: "Window & System" },
            { icon: "🌙", label: "SLEEP", name: "Sleep / Night", cat: "Window & System" },
            { icon: "☀", label: "BRITE", name: "Brightness / Sun", cat: "Window & System" },
            { icon: "🔋", label: "PWR", name: "Battery / Power", cat: "Window & System" },
            { icon: "🌐", label: "WEB", name: "Browser / Web", cat: "Window & System" },
            { icon: "📡", label: "WIFI", name: "Network / Wi-Fi", cat: "Window & System" },
            { icon: "⏱", label: "TIMER", name: "Stopwatch / Macro", cat: "Window & System" },
            { icon: "🔔", label: "BELL", name: "Notification / Alert", cat: "Window & System" },
            { icon: "🏠", label: "HOME", name: "Home Workspace", cat: "Window & System" },

            // Streaming & Apps
            { icon: "🎥", label: "SCENE", name: "Video / OBS Scene", cat: "Streaming & Apps" },
            { icon: "📺", label: "LIVE", name: "Stream / Monitor", cat: "Streaming & Apps" },
            { icon: "💬", label: "CHAT", name: "Chat / Discord", cat: "Streaming & Apps" },
            { icon: "🎮", label: "GAME", name: "Game Controller", cat: "Streaming & Apps" },
            { icon: "🎨", label: "COLOR", name: "Palette / Art", cat: "Streaming & Apps" },
            { icon: "🧮", label: "CALC", name: "Calculator", cat: "Streaming & Apps" },
            { icon: "📧", label: "MAIL", name: "Mail / Inbox", cat: "Streaming & Apps" },
            { icon: "📅", label: "CAL", name: "Calendar", cat: "Streaming & Apps" },

            // Symbols & Shapes
            { icon: "★", label: "STAR", name: "Solid Star", cat: "Symbols & Shapes" },
            { icon: "☆", label: "FAV", name: "Outline Star", cat: "Symbols & Shapes" },
            { icon: "♥", label: "LIKE", name: "Heart", cat: "Symbols & Shapes" },
            { icon: "✔", label: "OK", name: "Checkmark / Confirm", cat: "Symbols & Shapes" },
            { icon: "✖", label: "CLOSE", name: "Cross / Close", cat: "Symbols & Shapes" },
            { icon: "➕", label: "ADD", name: "Plus / Zoom In", cat: "Symbols & Shapes" },
            { icon: "➖", label: "SUB", name: "Minus / Zoom Out", cat: "Symbols & Shapes" },
            { icon: "⚠", label: "WARN", name: "Warning", cat: "Symbols & Shapes" },
            { icon: "🔥", label: "HOT", name: "Fire / Hotkey", cat: "Symbols & Shapes" },
            { icon: "💎", label: "PRO", name: "Diamond / Macro", cat: "Symbols & Shapes" },
            { icon: "🎯", label: "AIM", name: "Target / Focus", cat: "Symbols & Shapes" },
            { icon: "∞", label: "LOOP", name: "Infinity / Loop", cat: "Symbols & Shapes" }
        ]

        readonly property var filteredIcons: {
            const q = searchQuery.trim().toLowerCase()
            const cat = selectedCategory
            const out = []
            for (let i = 0; i < iconCatalog.length; i++) {
                const item = iconCatalog[i]
                if (cat !== "All" && item.cat !== cat) continue
                if (q.length > 0) {
                    const hay = (item.name + " " + item.label + " " + item.cat + " " + item.icon).toLowerCase()
                    if (hay.indexOf(q) === -1) continue
                }
                out.push(item)
            }
            return out
        }

        function openForInspector(defaultLine) {
            targetSource = "inspector"
            if (!targetSupportsTwoLines) {
                placementMode = 2 // Set Line 1
            } else if (defaultLine === 2) {
                placementMode = 3 // Set Line 2
            } else {
                placementMode = 0 // Icon (Line 1) + Short Label (Line 2)
            }
            show()
            raise()
            requestActivate()
            iconSearchInput.forceActiveFocus()
        }

        function openForKeyProperties(lineNum) {
            targetSource = "keyProps"
            if (!targetSupportsTwoLines) {
                placementMode = 2
            } else if (lineNum === 2) {
                placementMode = 3
            } else {
                placementMode = 2
            }
            show()
            raise()
            requestActivate()
            iconSearchInput.forceActiveFocus()
        }

        function currentLine1Text() {
            return targetSource === "keyProps" ? keyPropLabel1Field.text : label1Input.text
        }

        function currentLine2Text() {
            return targetSource === "keyProps" ? keyPropLabel2Field.text : label2Input.text
        }

        function applyValues(newL1, newL2) {
            const c1 = root.clampOledLabel(newL1)
            const c2 = root.clampOledLabel(newL2)
            if (targetSource === "keyProps") {
                keyPropLabel1Field.text = c1
                if (targetSupportsTwoLines) {
                    keyPropLabel2Field.text = c2
                }
            } else {
                label1Input.text = c1
                if (targetSupportsTwoLines) {
                    label2Input.text = c2
                }
                liveSlotAutoSaveTimer.stop()
                if (root.hasSelectedSlot && !root.loadingSlotData && root.compileOk) {
                    root.saveCurrentSlotToController()
                    hardwareCanvas.syncSlotsFromController()
                    root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
                } else {
                    liveSlotAutoSaveTimer.restart()
                }
            }
        }

        function applyIconItem(item) {
            if (!item) return
            const cur1 = currentLine1Text()
            const cur2 = currentLine2Text()
            if (!targetSupportsTwoLines) {
                if (placementMode === 4) {
                    applyValues(root.clampOledLabel(item.icon + cur1.trim()), "")
                } else {
                    applyValues(item.icon, "")
                }
                return
            }

            if (placementMode === 0) {
                // Icon on Line 1 + Short Label on Line 2 (or keep existing L2 if user already typed custom text)
                const nextL2 = cur2.trim().length > 0 && !root.isSingleIconLabel(cur2)
                    ? cur2
                    : item.label
                applyValues(item.icon, nextL2)
            } else if (placementMode === 1) {
                // Big Icon Only (Line 1 = icon, Line 2 = empty)
                applyValues(item.icon, "")
            } else if (placementMode === 2) {
                // Set Line 1 Only
                applyValues(item.icon, cur2)
            } else if (placementMode === 3) {
                // Set Line 2 Only
                applyValues(cur1, item.icon)
            } else if (placementMode === 4) {
                // Prepend Icon to Line 1 (e.g. "🔊VOL+")
                const stripped = root.isSingleIconLabel(cur1) ? "" : cur1.trim()
                const fallback = stripped.length > 0 ? stripped : item.label
                applyValues(root.clampOledLabel(item.icon + fallback), cur2)
            }
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 12
            spacing: 10

            // Top Header & Live OLED Key Preview Card
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: headerRow.implicitHeight + 16
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
                border.width: 1

                RowLayout {
                    id: headerRow
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 12

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 3

                        QQC2.Label {
                            text: qsTr("OLED Button Icon & Symbol Picker (%1)").arg(root.selectedSlotBadge)
                            font.bold: true
                            font.pointSize: 11 * root.fontScale
                            Layout.fillWidth: true
                            elide: Text.ElideRight
                        }

                        QQC2.Label {
                            text: qsTr("Click any icon below to assign it. Desktop OLED & Studio render the graphical symbol, and Sync to SD automatically writes the 5-char 7-bit ASCII glyph for the physical duckyPad OLED.")
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }
                    }

                    // Live 1:1 Desktop OLED Preview Box
                    ColumnLayout {
                        spacing: 2
                        QQC2.Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: qsTr("Desktop OLED")
                            font.pointSize: 10 * root.fontScale
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        Rectangle {
                            Layout.preferredWidth: 82
                            Layout.preferredHeight: 48
                            radius: 5
                            color: "#05070a"
                            border.color: "#38bdf8"
                            border.width: 1.5

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 4
                                spacing: 0

                                Text {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#ffffff"
                                    font.family: root.isSingleIconLabel(oledIconPickerDialog.currentLine1Text()) ? "sans-serif" : "monospace"
                                    font.bold: true
                                    font.pointSize: {
                                        const l1 = oledIconPickerDialog.currentLine1Text()
                                        const l2 = oledIconPickerDialog.currentLine2Text()
                                        if (root.isSingleIconLabel(l1)) {
                                            return ((!oledIconPickerDialog.targetSupportsTwoLines || l2.trim().length === 0) ? 14 : 12) * root.fontScale
                                        }
                                        return 10 * root.fontScale
                                    }
                                    text: oledIconPickerDialog.currentLine1Text().length > 0
                                        ? oledIconPickerDialog.currentLine1Text()
                                        : "—"
                                }

                                Text {
                                    visible: oledIconPickerDialog.targetSupportsTwoLines && oledIconPickerDialog.currentLine2Text().trim().length > 0
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: 15
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#d8f3ff"
                                    font.family: root.isSingleIconLabel(oledIconPickerDialog.currentLine2Text()) ? "sans-serif" : "monospace"
                                    font.bold: true
                                    font.pointSize: (root.isSingleIconLabel(oledIconPickerDialog.currentLine2Text()) ? 11 : 10) * root.fontScale
                                    text: oledIconPickerDialog.currentLine2Text()
                                }
                            }
                        }
                    }

                    // Live Physical duckyPad 6x8 ASCII OLED Preview Box
                    ColumnLayout {
                        spacing: 2
                        QQC2.Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: qsTr("duckyPad HW")
                            font.pointSize: 10 * root.fontScale
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        Rectangle {
                            Layout.preferredWidth: 82
                            Layout.preferredHeight: 48
                            radius: 5
                            color: "#05070a"
                            border.color: "#94a3b8"
                            border.width: 1.5

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 4
                                spacing: 0

                                Text {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#ffffff"
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    text: {
                                        const hw1 = deviceController.oledLabelToHardwareAscii(oledIconPickerDialog.currentLine1Text())
                                        return hw1.length > 0 ? hw1 : "—"
                                    }
                                }

                                Text {
                                    visible: oledIconPickerDialog.targetSupportsTwoLines && oledIconPickerDialog.currentLine2Text().trim().length > 0
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: 15
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    color: "#d8f3ff"
                                    font.family: "monospace"
                                    font.bold: true
                                    font.pointSize: 10 * root.fontScale
                                    text: deviceController.oledLabelToHardwareAscii(oledIconPickerDialog.currentLine2Text())
                                }
                            }
                        }
                    }
                }
            }

            // Placement Mode + Search & Category Filter Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("Apply Mode:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                }

                QQC2.ComboBox {
                    id: iconPlacementCombo
                    Layout.preferredWidth: 220
                    model: oledIconPickerDialog.targetSupportsTwoLines
                        ? [
                            qsTr("Icon (L1) + Short Label (L2)"),
                            qsTr("Big Icon Only (L1, clear L2)"),
                            qsTr("Set Line 1 Only"),
                            qsTr("Set Line 2 Only"),
                            qsTr("Icon + Text on Line 1")
                        ]
                        : [
                            qsTr("Set Icon Only"),
                            qsTr("Icon + Text (5 chars)")
                        ]
                    currentIndex: {
                        if (!oledIconPickerDialog.targetSupportsTwoLines) {
                            return oledIconPickerDialog.placementMode === 4 ? 1 : 0
                        }
                        return oledIconPickerDialog.placementMode
                    }
                    onActivated: function(index) {
                        if (!oledIconPickerDialog.targetSupportsTwoLines) {
                            oledIconPickerDialog.placementMode = (index === 1) ? 4 : 2
                        } else {
                            oledIconPickerDialog.placementMode = index
                        }
                    }
                }

                QQC2.ComboBox {
                    id: iconCategoryCombo
                    Layout.preferredWidth: 165
                    model: oledIconPickerDialog.categories
                    currentIndex: Math.max(0, oledIconPickerDialog.categories.indexOf(oledIconPickerDialog.selectedCategory))
                    onActivated: function(index) {
                        oledIconPickerDialog.selectedCategory = oledIconPickerDialog.categories[index]
                    }
                }

                QQC2.TextField {
                    id: iconSearchInput
                    Layout.fillWidth: true
                    placeholderText: qsTr("Search icons (e.g. play, vol, git, arrow)...")
                    selectByMouse: true
                    onTextChanged: oledIconPickerDialog.searchQuery = text
                }
            }

            // Direct Line 1 / Line 2 Editable Preview Bar
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("Line 1:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                }

                QQC2.TextField {
                    Layout.preferredWidth: 110
                    maximumLength: 10
                    font.family: "monospace"
                    font.bold: true
                    selectByMouse: true
                    text: oledIconPickerDialog.currentLine1Text()
                    onTextEdited: {
                        oledIconPickerDialog.applyValues(text, oledIconPickerDialog.currentLine2Text())
                    }
                }

                QQC2.Label {
                    visible: oledIconPickerDialog.targetSupportsTwoLines
                    text: qsTr("Line 2:")
                    font.bold: true
                    font.pointSize: 10 * root.fontScale
                }

                QQC2.TextField {
                    visible: oledIconPickerDialog.targetSupportsTwoLines
                    Layout.preferredWidth: 110
                    maximumLength: 10
                    font.family: "monospace"
                    selectByMouse: true
                    text: oledIconPickerDialog.currentLine2Text()
                    onTextEdited: {
                        oledIconPickerDialog.applyValues(oledIconPickerDialog.currentLine1Text(), text)
                    }
                }

                QQC2.Button {
                    visible: oledIconPickerDialog.targetSupportsTwoLines
                    text: qsTr("⇅ Swap L1/L2")
                    onClicked: {
                        oledIconPickerDialog.applyValues(
                            oledIconPickerDialog.currentLine2Text(),
                            oledIconPickerDialog.currentLine1Text()
                        )
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Label {
                    text: qsTr("%1 icons").arg(oledIconPickerDialog.filteredIcons.length)
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            // Scrollable Icon Grid
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 6
                color: Qt.darker(Kirigami.Theme.backgroundColor, 1.08)
                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
                border.width: 1
                clip: true

                GridView {
                    id: iconGridView
                    anchors.fill: parent
                    anchors.margins: 8
                    cellWidth: 92
                    cellHeight: 72
                    clip: true
                    boundsBehavior: Flickable.StopAtBounds
                    model: oledIconPickerDialog.filteredIcons

                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: iconGridView
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: iconGridView.contentHeight > iconGridView.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    delegate: Item {
                        width: iconGridView.cellWidth
                        height: iconGridView.cellHeight

                        required property var modelData

                        Rectangle {
                            anchors.fill: parent
                            anchors.margins: 3
                            radius: 6
                            color: iconTileMouse.containsMouse
                                ? Qt.alpha(Kirigami.Theme.highlightColor, 0.24)
                                : Kirigami.Theme.alternateBackgroundColor
                            border.color: iconTileMouse.containsMouse
                                ? Kirigami.Theme.highlightColor
                                : Qt.alpha(Kirigami.Theme.textColor, 0.14)
                            border.width: iconTileMouse.containsMouse ? 1.5 : 1

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 4
                                spacing: 1

                                Text {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    horizontalAlignment: Text.AlignHCenter
                                    verticalAlignment: Text.AlignVCenter
                                    text: modelData.icon
                                    font.pointSize: 14 * root.fontScale
                                    font.bold: true
                                    color: Kirigami.Theme.textColor
                                }

                                Text {
                                    Layout.fillWidth: true
                                    horizontalAlignment: Text.AlignHCenter
                                    text: modelData.label + " · " + deviceController.oledLabelToHardwareAscii(modelData.icon)
                                    font.family: "monospace"
                                    font.pointSize: 10 * root.fontScale
                                    font.bold: true
                                    color: Kirigami.Theme.disabledTextColor
                                    elide: Text.ElideRight
                                }
                            }

                            MouseArea {
                                id: iconTileMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: oledIconPickerDialog.applyIconItem(modelData)
                                onDoubleClicked: {
                                    oledIconPickerDialog.applyIconItem(modelData)
                                    oledIconPickerDialog.close()
                                }
                            }

                            QQC2.ToolTip.visible: iconTileMouse.containsMouse
                            QQC2.ToolTip.text: qsTr("%1 (%2 / HW: %3) — Click to apply, double-click to apply & close")
                                .arg(modelData.name)
                                .arg(modelData.icon)
                                .arg(deviceController.oledLabelToHardwareAscii(modelData.icon))
                        }
                    }
                }
            }

            // Footer Action Bar
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "edit-clear"
                    text: qsTr("Clear Labels")
                    onClicked: oledIconPickerDialog.applyValues("", "")
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    text: qsTr("Done")
                    highlighted: true
                    font.bold: true
                    onClicked: oledIconPickerDialog.close()
                }
            }
        }
    }

    OledDesktopWidget {
        id: oledDesktopWidget
        controller: deviceController
        selectedSlot: root.selectedSlotNumber
    }

    // =========================================================================
    // AUTOMATED UI TEST SUITE & SCENE-GRAPH AUDITOR (--ui-test-suite)
    // Verifies behavioral specifications, zero window/element overflow, and
    // WCAG color contrast across all windows/dialogs in Dark and Light themes.
    // =========================================================================
    Item {
        id: uiTestSuiteRunner
        visible: false
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false

        property int stepIndex: 0
        property string currentTheme: "dark"
        property var behavioralResults: []
        property var overflowViolations: []
        property var contrastViolations: []
        property var scrollBarStyleViolations: []
        property var minFontPointSizeViolations: []
        property var auditedWindows: []
        property int totalElementsInspected: 0
        property int totalTextNodesInspected: 0
        property int totalControlsInspected: 0
        property int totalScrollBarsInspected: 0

        function srgbChannelToLinear(c) {
            return (c <= 0.03928) ? (c / 12.92) : Math.pow((c + 0.055) / 1.055, 2.4)
        }

        function relativeLuminance(r, g, b) {
            return 0.2126 * srgbChannelToLinear(r)
                 + 0.7152 * srgbChannelToLinear(g)
                 + 0.0722 * srgbChannelToLinear(b)
        }

        function contrastRatio(c1, c2) {
            var l1 = relativeLuminance(c1.r, c1.g, c1.b)
            var l2 = relativeLuminance(c2.r, c2.g, c2.b)
            var lighter = Math.max(l1, l2)
            var darker = Math.min(l1, l2)
            return (lighter + 0.05) / (darker + 0.05)
        }

        function blendColors(fg, bg, extraOpacity) {
            var alpha = Math.max(0, Math.min(1, fg.a * (extraOpacity !== undefined ? extraOpacity : 1.0)))
            return {
                r: fg.r * alpha + bg.r * (1.0 - alpha),
                g: fg.g * alpha + bg.g * (1.0 - alpha),
                b: fg.b * alpha + bg.b * (1.0 - alpha),
                a: 1.0
            }
        }

        function isRectangleNode(node) {
            if (!node) return false
            var s = node.toString()
            return s.indexOf("QQuickRectangle") !== -1 || s.indexOf("ShadowedRectangle") !== -1
        }

        function resolveEffectiveBackground(item, fallbackWindowColor) {
            var layers = []
            var cur = item ? item.parent : null
            var guard = 0
            while (cur && guard < 64) {
                guard++
                // 1. Check if cur itself is a Rectangle with a visible fill
                if (isRectangleNode(cur) && cur.color !== undefined && cur.color.a !== undefined && cur.color.a > 0.01) {
                    var rectAlpha = cur.color.a * (cur.opacity !== undefined ? cur.opacity : 1.0)
                    layers.push({
                        r: cur.color.r,
                        g: cur.color.g,
                        b: cur.color.b,
                        a: rectAlpha
                    })
                    if (rectAlpha >= 0.98) break
                }
                // 2. Check if cur is a Control with a background item
                if (cur.background && cur.background.visible !== false && (cur.background.opacity === undefined || cur.background.opacity > 0.05)) {
                    if (isRectangleNode(cur.background) && cur.background.color !== undefined && cur.background.color.a !== undefined && cur.background.color.a > 0.01) {
                        var bgAlpha = cur.background.color.a * (cur.background.opacity !== undefined ? cur.background.opacity : 1.0)
                        layers.push({
                            r: cur.background.color.r,
                            g: cur.background.color.g,
                            b: cur.background.color.b,
                            a: bgAlpha
                        })
                        if (bgAlpha >= 0.98) break
                    } else {
                        // Native Breeze QQC2 style background (Button, ComboBox, TextField, SpinBox, etc.)
                        var curType = cur.toString()
                        var isFlatUnpressed = (cur.flat === true && !cur.down && !cur.checked && !cur.highlighted)
                        if (!isFlatUnpressed && (curType.indexOf("Button") !== -1
                                || curType.indexOf("TextField") !== -1
                                || curType.indexOf("TextArea") !== -1
                                || curType.indexOf("ComboBox") !== -1
                                || curType.indexOf("SpinBox") !== -1)) {
                            var themeBg = (cur.Kirigami && cur.Kirigami.Theme && cur.Kirigami.Theme.backgroundColor && cur.Kirigami.Theme.backgroundColor.a > 0.01)
                                ? cur.Kirigami.Theme.backgroundColor
                                : root.color
                            if (themeBg && themeBg.a > 0.01) {
                                layers.push({
                                    r: themeBg.r,
                                    g: themeBg.g,
                                    b: themeBg.b,
                                    a: themeBg.a
                                })
                                if (themeBg.a >= 0.98) break
                            }
                        }
                    }
                }
                cur = cur.parent
            }

            var hasValidFallback = (fallbackWindowColor
                && fallbackWindowColor.a !== undefined
                && fallbackWindowColor.a > 0.01
                && !(fallbackWindowColor.r === 0 && fallbackWindowColor.g === 0 && fallbackWindowColor.b === 0 && uiTestSuiteRunner.currentTheme === "light"))

            var base = hasValidFallback
                ? {
                    r: fallbackWindowColor.r,
                    g: fallbackWindowColor.g,
                    b: fallbackWindowColor.b,
                    a: 1.0
                }
                : (uiTestSuiteRunner.currentTheme === "light"
                    ? { r: 0.937, g: 0.941, b: 0.945, a: 1.0 }
                    : { r: 0.125, g: 0.137, b: 0.149, a: 1.0 })

            for (var i = layers.length - 1; i >= 0; i--) {
                base = blendColors(layers[i], base, 1.0)
            }
            return base
        }

        function isEffectivelyEnabled(item) {
            var cur = item
            var guard = 0
            while (cur && guard < 64) {
                guard++
                if (cur.enabled === false) return false
                if (cur.visible === false) return false
                if (cur.opacity !== undefined && cur.opacity < 0.45) return false
                cur = cur.parent
            }
            return true
        }

        function isPlaceholderTextNode(item) {
            if (!item || !item.parent) return false
            if (item.parent.placeholderText !== undefined && String(item.text) === String(item.parent.placeholderText)) {
                return true
            }
            return false
        }

        function hasAlphanumericText(s) {
            if (!s) return false
            return /[A-Za-z0-9]/.test(String(s))
        }

        function isInsideScrollableHorizontal(item, rootItem) {
            var cur = item ? item.parent : null
            var guard = 0
            while (cur && cur !== rootItem && guard < 64) {
                guard++
                var tName = cur.toString()
                if (cur.contentWidth !== undefined && cur.contentItem !== undefined) {
                    if (cur.contentWidth > cur.width + 2) return true
                }
                if (tName.indexOf("Flickable") !== -1 || tName.indexOf("ListView") !== -1 || tName.indexOf("GridView") !== -1) {
                    return true
                }
                cur = cur.parent
            }
            return false
        }

        function findEnclosingFlickable(item, rootItem) {
            var cur = item ? item.parent : null
            var guard = 0
            while (cur && cur !== rootItem && guard < 64) {
                guard++
                if (cur.contentWidth !== undefined && cur.contentHeight !== undefined && cur.contentItem !== undefined) {
                    return cur
                }
                cur = cur.parent
            }
            return null
        }

        function describeItem(item) {
            if (!item) return "<null>"
            var rawType = item.toString().split("(")[0]
            var txt = ""
            if (item.text !== undefined && String(item.text).length > 0) {
                txt = String(item.text).replace(/\s+/g, " ").substring(0, 48)
            } else if (item.title !== undefined && String(item.title).length > 0) {
                txt = String(item.title).replace(/\s+/g, " ").substring(0, 48)
            }
            return rawType + (txt.length > 0 ? "[\"" + txt + "\"]" : "")
        }

        function validateScrollBarStyle(windowName, sb) {
            if (!sb) return false
            uiTestSuiteRunner.totalScrollBarsInspected++

            var refBar = profilesBoxScrollBar
            var isVert = (sb.orientation === undefined || sb.orientation === Qt.Vertical)
            var bg = sb.background
            var thumb = sb.contentItem

            var expectedTrackThickness = refBar ? refBar.styleTrackThickness : 11
            var expectedThumbThickness = refBar ? refBar.styleThumbThickness : 7
            var expectedTrackRadius = refBar ? refBar.styleTrackRadius : 5
            var expectedThumbRadius = refBar ? refBar.styleThumbRadius : 4
            var expectedMinSize = refBar ? refBar.minimumSize : 0.14
            var expectedPadding = refBar ? refBar.padding : 1
            var expectedZ = refBar ? refBar.z : 50

            var actualTrackThickness = isVert ? Math.round(sb.implicitWidth) : Math.round(sb.implicitHeight)
            var actualThumbThickness = thumb
                ? (isVert ? Math.round(thumb.implicitWidth) : Math.round(thumb.implicitHeight))
                : -1
            var actualTrackRadius = (bg && bg.radius !== undefined) ? Math.round(bg.radius) : -1
            var actualThumbRadius = (thumb && thumb.radius !== undefined) ? Math.round(thumb.radius) : -1
            var actualTrackBorderWidth = (bg && bg.border) ? Math.round(bg.border.width) : -1
            var actualThumbBorderWidth = (thumb && thumb.border) ? Math.round(thumb.border.width) : -1
            var actualTrackAlpha = (bg && bg.color && bg.color.a !== undefined) ? bg.color.a : -1
            var actualTrackBorderAlpha = (bg && bg.border && bg.border.color && bg.border.color.a !== undefined) ? bg.border.color.a : -1
            var actualThumbBorderAlpha = (thumb && thumb.border && thumb.border.color && thumb.border.color.a !== undefined) ? thumb.border.color.a : -1

            var reasons = []
            if (sb.isProfileBoxStyleScrollBar !== true || sb.objectName !== "highContrastScrollBar") {
                reasons.push("not HighContrastScrollBar (objectName=" + sb.objectName + ")")
            }
            if (Math.abs((sb.minimumSize || 0) - expectedMinSize) > 0.005) {
                reasons.push("minimumSize=" + sb.minimumSize + " expected=" + expectedMinSize)
            }
            if (Math.round(sb.padding || 0) !== expectedPadding) {
                reasons.push("padding=" + sb.padding + " expected=" + expectedPadding)
            }
            if (Math.round(sb.z || 0) !== expectedZ) {
                reasons.push("z=" + sb.z + " expected=" + expectedZ)
            }
            if (actualTrackThickness !== expectedTrackThickness) {
                reasons.push("trackThickness=" + actualTrackThickness + " expected=" + expectedTrackThickness)
            }
            if (actualThumbThickness !== expectedThumbThickness) {
                reasons.push("thumbThickness=" + actualThumbThickness + " expected=" + expectedThumbThickness)
            }
            if (actualTrackRadius !== expectedTrackRadius) {
                reasons.push("trackRadius=" + actualTrackRadius + " expected=" + expectedTrackRadius)
            }
            if (actualThumbRadius !== expectedThumbRadius) {
                reasons.push("thumbRadius=" + actualThumbRadius + " expected=" + expectedThumbRadius)
            }
            if (actualTrackBorderWidth !== 1 || actualThumbBorderWidth !== 1) {
                reasons.push("borderWidth track=" + actualTrackBorderWidth + " thumb=" + actualThumbBorderWidth)
            }
            if (Math.abs(actualTrackAlpha - 0.14) > 0.025
                || Math.abs(actualTrackBorderAlpha - 0.28) > 0.025
                || Math.abs(actualThumbBorderAlpha - 0.55) > 0.025) {
                reasons.push("alpha mismatch track=" + actualTrackAlpha.toFixed(2)
                    + " trackBorder=" + actualTrackBorderAlpha.toFixed(2)
                    + " thumbBorder=" + actualThumbBorderAlpha.toFixed(2))
            }

            if (reasons.length > 0) {
                uiTestSuiteRunner.scrollBarStyleViolations.push({
                    theme: uiTestSuiteRunner.currentTheme,
                    window: windowName,
                    element: describeItem(sb),
                    parentElement: sb.parent ? describeItem(sb.parent) : "<none>",
                    reasons: reasons
                })
                return false
            }
            return true
        }

        function auditWindowScrollBars(windowName, rootItem) {
            if (!rootItem) return 0
            // OledDesktopWidget is a compact HUD overlay without scrollbars
            if (windowName.indexOf("OledDesktopWidget") === 0) return 0

            var stack = [{ node: rootItem, depth: 0 }]
            var visited = []
            var count = 0
            var guard = 0

            while (stack.length > 0 && guard < 10000) {
                var entry = stack.pop()
                var node = entry.node
                var depth = entry.depth
                if (!node || visited.indexOf(node) !== -1) continue
                visited.push(node)
                guard++

                var tStr = node.toString()
                if (tStr.indexOf("ScrollBar") !== -1 || node.isProfileBoxStyleScrollBar === true) {
                    count++
                    validateScrollBarStyle(windowName, node)
                }

                if (node.children && node.children.length > 0 && depth < 42) {
                    for (var i = node.children.length - 1; i >= 0; i--) {
                        stack.push({ node: node.children[i], depth: depth + 1 })
                    }
                }
                if (node.contentItem && node.contentItem !== node && depth < 42) {
                    stack.push({ node: node.contentItem, depth: depth + 1 })
                }
            }
            return count
        }

        function auditItemTree(windowName, rootItem, winWidth, winHeight, fallbackBgColor) {
            if (!rootItem || winWidth <= 0 || winHeight <= 0) return

            var windowScrollBarsCount = auditWindowScrollBars(windowName, rootItem)

            var stack = [{ node: rootItem, depth: 0 }]
            var visitedCount = 0

            while (stack.length > 0 && visitedCount < 8000) {
                var entry = stack.pop()
                var item = entry.node
                var depth = entry.depth
                if (!item) continue
                visitedCount++
                uiTestSuiteRunner.totalElementsInspected++

                if (item.visible === false || (item.opacity !== undefined && item.opacity <= 0.05)) {
                    continue
                }
                if (item.width !== undefined && item.height !== undefined && (item.width <= 0.5 || item.height <= 0.5)) {
                    continue
                }

                var typeStr = item.toString()
                var isButton = typeStr.indexOf("Button") !== -1 && typeStr.indexOf("ButtonGroup") === -1
                var isControl = isButton
                    || typeStr.indexOf("CheckBox") !== -1
                    || typeStr.indexOf("Switch") !== -1
                    || typeStr.indexOf("ComboBox") !== -1
                    || typeStr.indexOf("TextField") !== -1
                    || typeStr.indexOf("SpinBox") !== -1
                var isTextNode = (typeStr.indexOf("QQuickText") !== -1
                    || typeStr.indexOf("Label") !== -1
                    || (item.text !== undefined && item.font !== undefined && item.elide !== undefined))
                    && typeStr.indexOf("TextField") === -1
                    && typeStr.indexOf("TextArea") === -1
                    && typeStr.indexOf("TextEdit") === -1

                // 1. Check Control / Button Overflow
                if (isControl && item.width > 1 && item.height > 1) {
                    uiTestSuiteRunner.totalControlsInspected++
                    var flick = findEnclosingFlickable(item, rootItem)
                    if (!flick) {
                        var tl = item.mapToItem(rootItem, 0, 0)
                        var br = item.mapToItem(rootItem, item.width, item.height)
                        if (tl && br) {
                            if (br.x > winWidth + 4 || tl.x < -4 || br.y > winHeight + 4 || tl.y < -4) {
                                uiTestSuiteRunner.overflowViolations.push({
                                    theme: uiTestSuiteRunner.currentTheme,
                                    window: windowName,
                                    element: describeItem(item),
                                    kind: "control_outside_window_bounds",
                                    bounds: {
                                        left: Math.round(tl.x),
                                        top: Math.round(tl.y),
                                        right: Math.round(br.x),
                                        bottom: Math.round(br.y),
                                        windowWidth: Math.round(winWidth),
                                        windowHeight: Math.round(winHeight)
                                    }
                                })
                            }
                        }
                    } else if (flick.width > 10 && (!flick.contentWidth || flick.contentWidth <= flick.width + 4)) {
                        // Inside a vertical-only Flickable/ScrollView: verify horizontal bounds inside flickable
                        var ftl = item.mapToItem(flick, 0, 0)
                        var fbr = item.mapToItem(flick, item.width, item.height)
                        if (ftl && fbr && (fbr.x > flick.width + 6 || ftl.x < -6)) {
                            uiTestSuiteRunner.overflowViolations.push({
                                theme: uiTestSuiteRunner.currentTheme,
                                window: windowName,
                                element: describeItem(item),
                                kind: "control_overflows_scrollview_width",
                                bounds: {
                                    left: Math.round(ftl.x),
                                    right: Math.round(fbr.x),
                                    containerWidth: Math.round(flick.width)
                                }
                            })
                        }
                    }
                }

                // 2. Check Text / Label Overflow, Minimum 10pt Font, & WCAG Color Contrast
                if (isTextNode && hasAlphanumericText(item.text)) {
                    uiTestSuiteRunner.totalTextNodesInspected++

                    // Check minimum 10pt font size (scaled by root.fontScale)
                    if (item.font && item.font.pointSize !== undefined && item.font.pointSize > 0) {
                        var minExpectedPt = 9.95 * root.fontScale
                        if (item.font.pointSize < minExpectedPt) {
                            uiTestSuiteRunner.minFontPointSizeViolations.push({
                                theme: uiTestSuiteRunner.currentTheme,
                                window: windowName,
                                element: describeItem(item),
                                pointSize: Number(item.font.pointSize.toFixed(2)),
                                minimumRequiredPointSize: Number((10 * root.fontScale).toFixed(2))
                            })
                        }
                    }

                    // Check if un-elided single-line text overflows its parent container or window
                    var textWrap = item.wrapMode !== undefined ? item.wrapMode : Text.NoWrap
                    var textElide = item.elide !== undefined ? item.elide : Text.ElideNone
                    if (textWrap === Text.NoWrap && textElide === Text.ElideNone && item.implicitWidth > item.width + 8) {
                        var parentClip = item.parent && item.parent.clip === true
                        var ttl = item.mapToItem(rootItem, 0, 0)
                        var tbr = item.mapToItem(rootItem, item.implicitWidth, item.height)
                        if (parentClip || (tbr && tbr.x > winWidth + 6 && !isInsideScrollableHorizontal(item, rootItem))) {
                            uiTestSuiteRunner.overflowViolations.push({
                                theme: uiTestSuiteRunner.currentTheme,
                                window: windowName,
                                element: describeItem(item),
                                kind: "text_overflow_unelided",
                                details: {
                                    implicitWidth: Math.round(item.implicitWidth),
                                    allocatedWidth: Math.round(item.width),
                                    mappedRight: tbr ? Math.round(tbr.x) : -1,
                                    windowWidth: Math.round(winWidth)
                                }
                            })
                        }
                    }

                    // Check WCAG Color Contrast for enabled text nodes (excluding empty-input placeholder hints)
                    if (isEffectivelyEnabled(item) && !isPlaceholderTextNode(item) && item.color !== undefined) {
                        var bg = resolveEffectiveBackground(item, fallbackBgColor)
                        var fg = blendColors(item.color, bg, item.opacity !== undefined ? item.opacity : 1.0)
                        var ratio = contrastRatio(fg, bg)
                        var fontPt = (item.font && item.font.pointSize > 0) ? item.font.pointSize : (10 * root.fontScale)
                        var isBold = (item.font && item.font.bold) ? true : false
                        // Minimum contrast threshold: 3.0:1 across all UI controls, badges, headers, captions, and labels
                        var minRatio = 3.0
                        if (ratio < minRatio) {
                            uiTestSuiteRunner.contrastViolations.push({
                                theme: uiTestSuiteRunner.currentTheme,
                                window: windowName,
                                element: describeItem(item),
                                contrastRatio: Number(ratio.toFixed(2)),
                                requiredRatio: minRatio,
                                fgColor: Qt.rgba(fg.r, fg.g, fg.b, 1.0).toString(),
                                bgColor: Qt.rgba(bg.r, bg.g, bg.b, 1.0).toString(),
                                fontSize: fontPt,
                                bold: isBold
                            })
                        }
                    }
                }

                // Traverse visible children
                if (item.children && item.children.length > 0 && depth < 42) {
                    for (var c = item.children.length - 1; c >= 0; c--) {
                        stack.push({ node: item.children[c], depth: depth + 1 })
                    }
                }
                // Also traverse contentItem if present and not already in children
                if (item.contentItem && item.contentItem !== item && depth < 42) {
                    var alreadyChild = false
                    if (item.children) {
                        for (var k = 0; k < item.children.length; k++) {
                            if (item.children[k] === item.contentItem) {
                                alreadyChild = true
                                break
                            }
                        }
                    }
                    if (!alreadyChild) {
                        stack.push({ node: item.contentItem, depth: depth + 1 })
                    }
                }
            }

            uiTestSuiteRunner.auditedWindows.push({
                theme: uiTestSuiteRunner.currentTheme,
                window: windowName,
                width: Math.round(winWidth),
                height: Math.round(winHeight),
                elementsVisited: visitedCount,
                scrollBarsAudited: windowScrollBarsCount
            })
        }

        function recordBehavior(name, passed, details) {
            uiTestSuiteRunner.behavioralResults.push({
                name: name,
                passed: Boolean(passed),
                details: details || ""
            })
        }

        function ensureTestFixtures() {
            deviceController.resetAllTextScales()
            firmwareUpdateDialog.refreshCatalog()
        }

        function runBehavioralSuite() {
            ensureTestFixtures()

            // 1. Desktop Widget Profile Rotation (0°, 90°, 180°, 270°)
            deviceController.setActiveProfileWidgetRotation(0)
            oledDesktopWidget.syncFromController()
            var rot0Ok = (oledDesktopWidget.effectiveRotation === 0
                && oledDesktopWidget.gridCols === 4
                && oledDesktopWidget.gridRows === 5
                && oledDesktopWidget.slotForCell(0, 0) === 1
                && oledDesktopWidget.slotForCell(3, 0) === 4
                && oledDesktopWidget.slotForCell(0, 4) === 17
                && oledDesktopWidget.slotForCell(3, 4) === 20)
            recordBehavior("desktop_widget_rotation_0_deg", rot0Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows)

            deviceController.setActiveProfileWidgetRotation(90)
            oledDesktopWidget.syncFromController()
            var rot90Ok = (oledDesktopWidget.effectiveRotation === 90
                && oledDesktopWidget.gridCols === 5
                && oledDesktopWidget.gridRows === 4
                && oledDesktopWidget.slotForCell(0, 0) === 4
                && oledDesktopWidget.slotForCell(4, 0) === 20
                && oledDesktopWidget.slotForCell(0, 3) === 1
                && oledDesktopWidget.slotForCell(4, 3) === 17)
            recordBehavior("desktop_widget_rotation_90_deg", rot90Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows)

            deviceController.setActiveProfileWidgetRotation(180)
            oledDesktopWidget.syncFromController()
            var rot180Ok = (oledDesktopWidget.effectiveRotation === 180
                && oledDesktopWidget.gridCols === 4
                && oledDesktopWidget.gridRows === 5
                && oledDesktopWidget.slotForCell(0, 0) === 20
                && oledDesktopWidget.slotForCell(3, 0) === 17
                && oledDesktopWidget.slotForCell(0, 4) === 4
                && oledDesktopWidget.slotForCell(3, 4) === 1)
            recordBehavior("desktop_widget_rotation_180_deg", rot180Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows)

            deviceController.setActiveProfileWidgetRotation(270)
            oledDesktopWidget.syncFromController()
            var rot270Ok = (oledDesktopWidget.effectiveRotation === 270
                && oledDesktopWidget.gridCols === 5
                && oledDesktopWidget.gridRows === 4
                && oledDesktopWidget.slotForCell(0, 0) === 17
                && oledDesktopWidget.slotForCell(4, 0) === 1
                && oledDesktopWidget.slotForCell(0, 3) === 20
                && oledDesktopWidget.slotForCell(4, 3) === 4)
            recordBehavior("desktop_widget_rotation_270_deg", rot270Ok,
                "rot=" + oledDesktopWidget.effectiveRotation + " grid=" + oledDesktopWidget.gridCols + "x" + oledDesktopWidget.gridRows)

            // 2. Profile Properties Window hardware rotation + Follow Hardware (-1) propagation
            var activeProfNum = deviceController.activeProfile || 1
            profilePropertiesWindow.openForProfile(activeProfNum)
            profilePropertiesWindow.profOledRotation = 90
            profilePropertiesWindow.profWidgetRotation = -1
            profilePropertiesWindow.saveAllProfileProperties()
            var savedProps = {}
            try {
                savedProps = JSON.parse(deviceController.getProfilePropertiesJson(activeProfNum))
            } catch (e) {}
            var followHwOk = (savedProps.oledRotation === 90
                && savedProps.widgetRotation === -1
                && savedProps.effectiveWidgetRotation === 90
                && oledDesktopWidget.effectiveRotation === 90
                && oledDesktopWidget.gridCols === 5
                && oledDesktopWidget.gridRows === 4)
            recordBehavior("profile_properties_follow_hardware_rotation", followHwOk,
                "hwRot=" + savedProps.oledRotation
                + " widgetOverride=" + savedProps.widgetRotation
                + " effective=" + oledDesktopWidget.effectiveRotation)

            // Restore 0° rotation after rotation tests
            profilePropertiesWindow.profOledRotation = 0
            profilePropertiesWindow.profWidgetRotation = -1
            profilePropertiesWindow.saveAllProfileProperties()
            profilePropertiesWindow.close()

            // 3. Firmware Manager filtering & catalog behavior
            firmwareUpdateDialog.refreshCatalog()
            firmwareUpdateDialog.vendorFilter = "all"
            var allCount = firmwareUpdateDialog.filteredFirmwares.length
            firmwareUpdateDialog.vendorFilter = "studio"
            var studioCount = firmwareUpdateDialog.filteredFirmwares.length
            firmwareUpdateDialog.vendorFilter = "official"
            var officialCount = firmwareUpdateDialog.filteredFirmwares.length
            firmwareUpdateDialog.vendorFilter = "all"
            recordBehavior("firmware_manager_vendor_filtering",
                allCount >= 2 && studioCount >= 1 && officialCount >= 1 && (studioCount + officialCount === allCount),
                "all=" + allCount + " studio=" + studioCount + " official=" + officialCount)

            // 4. Resizable windows scrollbar style matches Profiles Box scrollbar (profilesBoxScrollBar)
            var resizableWindowsWithScrollBars = [
                { name: "MainWindow", root: root.contentItem },
                { name: "FirmwareUpdateDialog", root: firmwareUpdateDialog.contentItem },
                { name: "ProfilePropertiesWindow", root: profilePropertiesWindow.contentItem },
                { name: "KeyPropertiesWindow", root: keyPropertiesWindow.contentItem },
                { name: "DuckyColorPickerWindow", root: duckyColorPickerWindow.contentItem },
                { name: "ProfileTemplatesDialog", root: profileTemplatesDialog.contentItem },
                { name: "SettingsDialog", root: settingsDialog.contentItem },
                { name: "WebApiDestinationsDialog", root: webApiDestinationsDialog.contentItem },
                { name: "EventLogDialog", root: eventLogDialog.contentItem },
                { name: "AboutDialog", root: aboutDialog.contentItem },
                { name: "UnsyncedChangesDialog", root: unsyncedChangesDialog.contentItem },
                { name: "SdSyncDialog", root: sdSyncDialog.contentItem },
                { name: "AppLaunchConfigWindow", root: appLaunchConfigWindow.contentItem },
                { name: "InstalledAppPickerDialog", root: installedAppPickerDialog.contentItem },
                { name: "WebApiKeyConfigWindow", root: webApiKeyConfigWindow.contentItem },
                { name: "LightingAnimatorDialog", root: lightingAnimatorDialog.contentItem },
                { name: "MacroRecorderDialog", root: macroRecorderDialog.contentItem },
                { name: "OledIconPickerDialog", root: oledIconPickerDialog.contentItem },
                { name: "PgvDrawer", root: pgvDrawer.contentItem },
                { name: "AutoSwitchDrawer", root: autoSwitchDrawer.contentItem },
                { name: "AiModal", root: aiModal.contentItem }
            ]
            var missingScrollBarWindows = []
            var totalResizableBarsChecked = 0
            for (var wIdx = 0; wIdx < resizableWindowsWithScrollBars.length; wIdx++) {
                var wEntry = resizableWindowsWithScrollBars[wIdx]
                var c = auditWindowScrollBars(wEntry.name + "_BehavioralCheck", wEntry.root)
                totalResizableBarsChecked += c
                if (c === 0) {
                    missingScrollBarWindows.push(wEntry.name)
                }
            }
            var refBarOk = validateScrollBarStyle("ProfilesBox_Reference", profilesBoxScrollBar)
            recordBehavior(
                "resizable_windows_scrollbar_style_matches_profiles_box",
                refBarOk
                    && missingScrollBarWindows.length === 0
                    && uiTestSuiteRunner.scrollBarStyleViolations.length === 0
                    && totalResizableBarsChecked >= 20,
                "windowsVerified=" + (resizableWindowsWithScrollBars.length - missingScrollBarWindows.length)
                    + "/" + resizableWindowsWithScrollBars.length
                    + " scrollBarsVerified=" + totalResizableBarsChecked
                    + " violations=" + uiTestSuiteRunner.scrollBarStyleViolations.length
            )

            // 5. Script Editor Box Scrollbar (always-on vertical HighContrastScrollBar + multi-line scrolling)
            root.selectedSlotNumber = 1
            root.inspectorMode = "code"
            editorContainer.editorExpanded = true
            root.syncWindowWidthToEditorState()
            var savedScriptText = scriptEditor.text
            root.loadingSlotData = true
            var longLines = []
            for (var ln = 1; ln <= 60; ln++) {
                longLines.push("STRING Line " + ln + " — DuckyScript 3 Scrollbar Test")
            }
            scriptEditor.text = longLines.join("\n")
            root.loadingSlotData = false
            var scriptSbStyleOk = validateScrollBarStyle("ScriptEditor_VerticalScrollBar", scriptEditorVerticalScrollBar)
            var scriptSbVisibleOk = Boolean(
                scriptEditorVerticalScrollBar
                && scriptEditorVerticalScrollBar.visible
                && scriptEditorVerticalScrollBar.policy === QQC2.ScrollBar.AlwaysOn
                && scriptEditorVerticalScrollBar.background
                && scriptEditorVerticalScrollBar.background.visible
                && scriptEditorVerticalScrollBar.contentItem
                && scriptEditorVerticalScrollBar.contentItem.visible
            )
            var scriptScrollableOk = scriptScrollView.contentHeight > scriptScrollView.height && scriptEditorVerticalScrollBar.size < 1.0
            scriptScrollView.contentY = Math.max(20, scriptScrollView.contentHeight - scriptScrollView.height)
            var scrolledPos = scriptEditorVerticalScrollBar.position
            scriptScrollView.contentY = 0
            root.loadingSlotData = true
            scriptEditor.text = savedScriptText
            root.loadingSlotData = false
            recordBehavior(
                "script_editor_has_scrollbar",
                scriptSbStyleOk && scriptSbVisibleOk && scriptScrollableOk && scrolledPos > 0.01,
                "styleOk=" + scriptSbStyleOk
                    + " visibleOk=" + scriptSbVisibleOk
                    + " scrollableOk=" + scriptScrollableOk
                    + " scrolledPos=" + scrolledPos.toFixed(3)
            )

            // 6. Settings Dialog removed per-area text sizes & minimum 10pt base font
            settingsDialog.open()
            var forbiddenAreaStrings = [
                "TEXT SIZES BY APPLICATION AREA",
                "Profiles & Auto-Switcher Sidebar",
                "Hardware Pad & Key Labels",
                "Key Inspector & Action Controls",
                "duckyScript Code Editor",
                "Bottom Status Bar"
            ]
            var foundForbiddenSettingsTexts = []
            var sStack = [settingsDialog.contentItem]
            var sVisited = []
            while (sStack.length > 0 && sVisited.length < 4000) {
                var sNode = sStack.pop()
                if (!sNode || sVisited.indexOf(sNode) !== -1) continue
                sVisited.push(sNode)
                if (sNode.text !== undefined) {
                    var sTxt = String(sNode.text)
                    for (var fIdx = 0; fIdx < forbiddenAreaStrings.length; fIdx++) {
                        if (sTxt.indexOf(forbiddenAreaStrings[fIdx]) !== -1) {
                            foundForbiddenSettingsTexts.push(forbiddenAreaStrings[fIdx])
                        }
                    }
                }
                if (sNode.children) {
                    for (var sc = 0; sc < sNode.children.length; sc++) {
                        sStack.push(sNode.children[sc])
                    }
                }
                if (sNode.contentItem && sNode.contentItem !== sNode) {
                    sStack.push(sNode.contentItem)
                }
            }
            settingsDialog.close()
            var baseEditorPt = scriptEditor.font.pointSize / root.fontScale
            recordBehavior(
                "settings_removed_area_text_sizes_and_min_10pt_font",
                foundForbiddenSettingsTexts.length === 0 && baseEditorPt >= 10.0,
                "forbiddenSettingsFound=" + foundForbiddenSettingsTexts.length
                    + " baseEditorPt=" + baseEditorPt.toFixed(1)
            )

            // 7. Ctrl++, Ctrl+=, Ctrl+- Zoom Adjustment & Disk Persistence on Restart
            deviceController.resetAllTextScales()
            var initialZoom = deviceController.uiTextScale
            // Dispatch Ctrl++
            deviceController.sendKeyToActiveWindow(Qt.Key_Plus, Qt.ControlModifier, "+")
            var afterCtrlPlus = deviceController.uiTextScale
            // Dispatch Ctrl+=
            deviceController.sendKeyToActiveWindow(Qt.Key_Equal, Qt.ControlModifier, "=")
            var afterCtrlEqual = deviceController.uiTextScale
            // Verify disk persistence by mutating in-memory scale and reloading from disk (simulating app restart)
            var reloadedAfterZoomIn = deviceController.reloadZoomLevelFromDisk()
            // Dispatch Ctrl+-
            deviceController.sendKeyToActiveWindow(Qt.Key_Minus, Qt.ControlModifier, "-")
            var afterCtrlMinus = deviceController.uiTextScale
            var reloadedAfterZoomOut = deviceController.reloadZoomLevelFromDisk()
            // Dispatch Ctrl+0 to reset to 100%
            deviceController.sendKeyToActiveWindow(Qt.Key_0, Qt.ControlModifier, "0")
            var afterCtrlZero = deviceController.uiTextScale
            var reloadedAfterReset = deviceController.reloadZoomLevelFromDisk()

            var zoomAndPersistenceOk = (
                initialZoom === 100
                && afterCtrlPlus === 105
                && afterCtrlEqual === 110
                && reloadedAfterZoomIn === 110
                && afterCtrlMinus === 105
                && reloadedAfterZoomOut === 105
                && afterCtrlZero === 100
                && reloadedAfterReset === 100
            )
            recordBehavior(
                "ctrl_plus_equal_minus_zoom_and_restart_persistence",
                zoomAndPersistenceOk,
                "initial=" + initialZoom
                    + " ctrlPlus=" + afterCtrlPlus
                    + " ctrlEqual=" + afterCtrlEqual
                    + " reloaded110=" + reloadedAfterZoomIn
                    + " ctrlMinus=" + afterCtrlMinus
                    + " reloaded105=" + reloadedAfterZoomOut
                    + " ctrl0=" + afterCtrlZero
            )

            // 8. Every Popup Window Closable with ESC
            var popupEntries = root.allPopupWindows()
            var escFailedPopups = []
            var escVerifiedCount = 0
            // Ensure clean state before testing popups
            for (var pClean = 0; pClean < popupEntries.length; pClean++) {
                if (popupEntries[pClean].win && popupEntries[pClean].win.visible) {
                    popupEntries[pClean].win.close()
                }
            }
            for (var pIdx = 0; pIdx < popupEntries.length; pIdx++) {
                var pEntry = popupEntries[pIdx]
                var pWin = pEntry.win
                pEntry.openFn()
                var openedOk = Boolean(pWin && pWin.visible === true && pWin.closableWithEscape === true)
                deviceController.sendKeyToActiveWindow(Qt.Key_Escape, 0, "")
                var closedByEscOk = Boolean(pWin && pWin.visible === false)
                if (!openedOk || !closedByEscOk) {
                    escFailedPopups.push(pEntry.name + "(opened=" + openedOk + ",closed=" + closedByEscOk + ")")
                    if (pWin && pWin.visible) {
                        pWin.close()
                    }
                } else {
                    escVerifiedCount++
                }
            }
            root.selectedSlotNumber = 0
            root.syncWindowWidthToEditorState()
            recordBehavior(
                "all_popup_windows_closable_with_esc",
                escFailedPopups.length === 0 && escVerifiedCount === popupEntries.length && escVerifiedCount >= 24,
                "verified=" + escVerifiedCount + "/" + popupEntries.length
                    + (escFailedPopups.length > 0 ? " failed=" + escFailedPopups.join(",") : "")
            )
        }

        property var windowSteps: [
            {
                name: "MainWindow_Default",
                open: function() {
                    root.selectedSlotNumber = 0
                    root.syncWindowWidthToEditorState()
                },
                audit: function() {
                    auditItemTree("MainWindow_Default", root.contentItem, root.width, root.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {}
            },
            {
                name: "MainWindow_SlotInspector",
                open: function() {
                    root.selectedSlotNumber = 1
                    root.syncWindowWidthToEditorState()
                },
                audit: function() {
                    auditItemTree("MainWindow_SlotInspector", root.contentItem, root.width, root.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    root.selectedSlotNumber = 0
                    root.syncWindowWidthToEditorState()
                }
            },
            {
                name: "OledDesktopWidget_0deg",
                open: function() {
                    deviceController.setActiveProfileWidgetRotation(0)
                    oledDesktopWidget.show()
                    oledDesktopWidget.syncFromController()
                },
                audit: function() {
                    auditItemTree("OledDesktopWidget_0deg", oledDesktopWidget.contentItem, oledDesktopWidget.width, oledDesktopWidget.height, { r: 0.02, g: 0.03, b: 0.06, a: 1.0 })
                },
                close: function() {}
            },
            {
                name: "OledDesktopWidget_90deg",
                open: function() {
                    deviceController.setActiveProfileWidgetRotation(90)
                    oledDesktopWidget.show()
                    oledDesktopWidget.syncFromController()
                },
                audit: function() {
                    auditItemTree("OledDesktopWidget_90deg", oledDesktopWidget.contentItem, oledDesktopWidget.width, oledDesktopWidget.height, { r: 0.02, g: 0.03, b: 0.06, a: 1.0 })
                },
                close: function() {
                    deviceController.setActiveProfileWidgetRotation(0)
                    oledDesktopWidget.syncFromController()
                    oledDesktopWidget.hide()
                }
            },
            {
                name: "FirmwareUpdateDialog",
                open: function() {
                    ensureTestFixtures()
                    firmwareUpdateDialog.open()
                },
                audit: function() {
                    auditItemTree("FirmwareUpdateDialog", firmwareUpdateDialog.contentItem, firmwareUpdateDialog.width, firmwareUpdateDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    firmwareUpdateDialog.close()
                }
            },
            {
                name: "ProfilePropertiesWindow",
                open: function() {
                    profilePropertiesWindow.openForProfile(deviceController.activeProfile || 1)
                },
                audit: function() {
                    auditItemTree("ProfilePropertiesWindow", profilePropertiesWindow.contentItem, profilePropertiesWindow.width, profilePropertiesWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    profilePropertiesWindow.close()
                }
            },
            {
                name: "KeyPropertiesWindow",
                open: function() {
                    keyPropertiesWindow.openForSlot(1)
                },
                audit: function() {
                    auditItemTree("KeyPropertiesWindow", keyPropertiesWindow.contentItem, keyPropertiesWindow.width, keyPropertiesWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    keyPropertiesWindow.close()
                }
            },
            {
                name: "DuckyColorPickerWindow",
                open: function() {
                    duckyColorPickerWindow.openForKey(1, "#38BDF8")
                },
                audit: function() {
                    auditItemTree("DuckyColorPickerWindow", duckyColorPickerWindow.contentItem, duckyColorPickerWindow.width, duckyColorPickerWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    duckyColorPickerWindow.close()
                }
            },
            {
                name: "ProfileTemplatesDialog",
                open: function() {
                    profileTemplatesDialog.openGeneral()
                },
                audit: function() {
                    auditItemTree("ProfileTemplatesDialog", profileTemplatesDialog.contentItem, profileTemplatesDialog.width, profileTemplatesDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    profileTemplatesDialog.close()
                }
            },
            {
                name: "SettingsDialog",
                open: function() {
                    settingsDialog.open()
                },
                audit: function() {
                    auditItemTree("SettingsDialog", settingsDialog.contentItem, settingsDialog.width, settingsDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    settingsDialog.close()
                }
            },
            {
                name: "WebApiDestinationsDialog",
                open: function() {
                    webApiDestinationsDialog.openForDestination("", "POST", "", "")
                },
                audit: function() {
                    auditItemTree("WebApiDestinationsDialog", webApiDestinationsDialog.contentItem, webApiDestinationsDialog.width, webApiDestinationsDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    webApiDestinationsDialog.close()
                }
            },
            {
                name: "EventLogDialog",
                open: function() {
                    eventLogDialog.openLogWindow()
                },
                audit: function() {
                    auditItemTree("EventLogDialog", eventLogDialog.contentItem, eventLogDialog.width, eventLogDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    eventLogDialog.close()
                }
            },
            {
                name: "AboutDialog",
                open: function() {
                    aboutDialog.openAboutWindow()
                },
                audit: function() {
                    auditItemTree("AboutDialog", aboutDialog.contentItem, aboutDialog.width, aboutDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    aboutDialog.close()
                }
            },
            {
                name: "UnsyncedChangesDialog",
                open: function() {
                    unsyncedChangesDialog.open()
                },
                audit: function() {
                    auditItemTree("UnsyncedChangesDialog", unsyncedChangesDialog.contentItem, unsyncedChangesDialog.width, unsyncedChangesDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    unsyncedChangesDialog.close()
                }
            },
            {
                name: "SdSyncDialog",
                open: function() {
                    sdSyncDialog.open()
                },
                audit: function() {
                    auditItemTree("SdSyncDialog", sdSyncDialog.contentItem, sdSyncDialog.width, sdSyncDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    sdSyncDialog.close()
                }
            },
            {
                name: "AppLaunchConfigWindow",
                open: function() {
                    appLaunchConfigWindow.openWindow(1)
                },
                audit: function() {
                    auditItemTree("AppLaunchConfigWindow", appLaunchConfigWindow.contentItem, appLaunchConfigWindow.width, appLaunchConfigWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    appLaunchConfigWindow.close()
                }
            },
            {
                name: "InstalledAppPickerDialog",
                open: function() {
                    installedAppPickerDialog.show()
                },
                audit: function() {
                    auditItemTree("InstalledAppPickerDialog", installedAppPickerDialog.contentItem, installedAppPickerDialog.width, installedAppPickerDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    installedAppPickerDialog.close()
                }
            },
            {
                name: "WebApiKeyConfigWindow",
                open: function() {
                    webApiKeyConfigWindow.openWindow(1)
                },
                audit: function() {
                    auditItemTree("WebApiKeyConfigWindow", webApiKeyConfigWindow.contentItem, webApiKeyConfigWindow.width, webApiKeyConfigWindow.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    webApiKeyConfigWindow.close()
                }
            },
            {
                name: "LightingAnimatorDialog",
                open: function() {
                    lightingAnimatorDialog.openFor("profile", 1)
                },
                audit: function() {
                    auditItemTree("LightingAnimatorDialog", lightingAnimatorDialog.contentItem, lightingAnimatorDialog.width, lightingAnimatorDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    lightingAnimatorDialog.close()
                }
            },
            {
                name: "MacroRecorderDialog",
                open: function() {
                    macroRecorderDialog.openRecorder()
                },
                audit: function() {
                    auditItemTree("MacroRecorderDialog", macroRecorderDialog.contentItem, macroRecorderDialog.width, macroRecorderDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    macroRecorderDialog.close()
                }
            },
            {
                name: "OledIconPickerDialog",
                open: function() {
                    oledIconPickerDialog.openForInspector(0)
                },
                audit: function() {
                    auditItemTree("OledIconPickerDialog", oledIconPickerDialog.contentItem, oledIconPickerDialog.width, oledIconPickerDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    oledIconPickerDialog.close()
                }
            },
            {
                name: "NewProfileGroupDialog",
                open: function() {
                    newProfileGroupDialog.openWithDefaults()
                },
                audit: function() {
                    auditItemTree("NewProfileGroupDialog", newProfileGroupDialog.contentItem, newProfileGroupDialog.width, newProfileGroupDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    newProfileGroupDialog.close()
                }
            },
            {
                name: "RenameProfileDialog",
                open: function() {
                    renameProfileDialog.openWithProfile(1, "Default")
                },
                audit: function() {
                    auditItemTree("RenameProfileDialog", renameProfileDialog.contentItem, renameProfileDialog.width, renameProfileDialog.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    renameProfileDialog.close()
                }
            },
            {
                name: "PgvDrawer",
                open: function() {
                    pgvDrawer.open()
                },
                audit: function() {
                    auditItemTree("PgvDrawer", pgvDrawer.contentItem, pgvDrawer.width, pgvDrawer.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    pgvDrawer.close()
                }
            },
            {
                name: "AutoSwitchDrawer",
                open: function() {
                    autoSwitchDrawer.open()
                },
                audit: function() {
                    auditItemTree("AutoSwitchDrawer", autoSwitchDrawer.contentItem, autoSwitchDrawer.width, autoSwitchDrawer.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    autoSwitchDrawer.close()
                }
            },
            {
                name: "AiModal",
                open: function() {
                    aiModal.open()
                },
                audit: function() {
                    auditItemTree("AiModal", aiModal.contentItem, aiModal.width, aiModal.height, Kirigami.Theme.backgroundColor)
                },
                close: function() {
                    aiModal.close()
                }
            }
        ]

        property int phase: 0 // 0=init, 1=open window, 2=audit+close window

        Timer {
            id: uiTestStepTimer
            interval: 65
            repeat: true
            running: deviceController.isUiTestSuiteMode()
            onTriggered: {
                var totalStepsPerTheme = uiTestSuiteRunner.windowSteps.length
                if (uiTestSuiteRunner.phase === 0) {
                    // Run behavioral checks first, then switch to dark theme
                    uiTestSuiteRunner.runBehavioralSuite()
                    uiTestSuiteRunner.currentTheme = "dark"
                    deviceController.applyAppThemeMode("dark")
                    uiTestSuiteRunner.stepIndex = 0
                    uiTestSuiteRunner.phase = 1
                    return
                }

                if (uiTestSuiteRunner.stepIndex >= totalStepsPerTheme) {
                    if (uiTestSuiteRunner.currentTheme === "dark") {
                        // Switch to Light theme and audit all windows again
                        uiTestSuiteRunner.currentTheme = "light"
                        deviceController.applyAppThemeMode("light")
                        uiTestSuiteRunner.stepIndex = 0
                        uiTestSuiteRunner.phase = 1
                        return
                    } else {
                        // Finished both Dark and Light theme audits!
                        uiTestStepTimer.stop()
                        var failedBehaviors = []
                        for (var b = 0; b < uiTestSuiteRunner.behavioralResults.length; b++) {
                            if (!uiTestSuiteRunner.behavioralResults[b].passed) {
                                failedBehaviors.push(uiTestSuiteRunner.behavioralResults[b])
                            }
                        }
                        var totalFailures = failedBehaviors.length
                            + uiTestSuiteRunner.overflowViolations.length
                            + uiTestSuiteRunner.contrastViolations.length
                            + uiTestSuiteRunner.scrollBarStyleViolations.length
                            + uiTestSuiteRunner.minFontPointSizeViolations.length

                        var report = {
                            passed: totalFailures === 0,
                            summary: {
                                themesTested: ["dark", "light"],
                                windowsAuditedCount: uiTestSuiteRunner.auditedWindows.length,
                                totalElementsInspected: uiTestSuiteRunner.totalElementsInspected,
                                totalControlsInspected: uiTestSuiteRunner.totalControlsInspected,
                                totalTextNodesInspected: uiTestSuiteRunner.totalTextNodesInspected,
                                totalScrollBarsInspected: uiTestSuiteRunner.totalScrollBarsInspected,
                                behavioralChecksCount: uiTestSuiteRunner.behavioralResults.length,
                                behavioralFailuresCount: failedBehaviors.length,
                                overflowViolationsCount: uiTestSuiteRunner.overflowViolations.length,
                                contrastViolationsCount: uiTestSuiteRunner.contrastViolations.length,
                                scrollBarStyleViolationsCount: uiTestSuiteRunner.scrollBarStyleViolations.length,
                                minFontPointSizeViolationsCount: uiTestSuiteRunner.minFontPointSizeViolations.length
                            },
                            behavioralResults: uiTestSuiteRunner.behavioralResults,
                            overflowViolations: uiTestSuiteRunner.overflowViolations,
                            contrastViolations: uiTestSuiteRunner.contrastViolations,
                            scrollBarStyleViolations: uiTestSuiteRunner.scrollBarStyleViolations,
                            minFontPointSizeViolations: uiTestSuiteRunner.minFontPointSizeViolations,
                            auditedWindows: uiTestSuiteRunner.auditedWindows
                        }

                        deviceController.writeUiTestReport(JSON.stringify(report, null, 2), totalFailures === 0 ? 0 : 1)
                        Qt.quit()
                        return
                    }
                }

                var currentStep = uiTestSuiteRunner.windowSteps[uiTestSuiteRunner.stepIndex]
                if (uiTestSuiteRunner.phase === 1) {
                    try {
                        currentStep.open()
                    } catch (e) {
                        uiTestSuiteRunner.recordBehavior("open_" + currentStep.name + "_" + uiTestSuiteRunner.currentTheme, false, String(e))
                    }
                    uiTestSuiteRunner.phase = 2
                } else if (uiTestSuiteRunner.phase === 2) {
                    try {
                        currentStep.audit()
                    } catch (e2) {
                        uiTestSuiteRunner.recordBehavior("audit_" + currentStep.name + "_" + uiTestSuiteRunner.currentTheme, false, String(e2))
                    }
                    try {
                        currentStep.close()
                    } catch (e3) {}
                    uiTestSuiteRunner.stepIndex++
                    uiTestSuiteRunner.phase = 1
                }
            }
        }
    }
}


