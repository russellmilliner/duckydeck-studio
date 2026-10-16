import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami
import org.kde.syntaxhighlighting as KSyntax
import io.github.duckydeck 1.0

Kirigami.ApplicationWindow {
    id: root
    objectName: "duckydeckMainWindow"
    title: qsTr("DuckyDeck Studio v%1 — %2").arg(deviceController.appVersion).arg(deviceController.modelName)
    readonly property int compactWindowWidth: Math.max(
        profilesSidebar.QQC2.SplitView.preferredWidth
            + hardwareCanvas.requiredWidthForFullHeight
            + Kirigami.Units.largeSpacing * 2
            + 20,
        connRow.implicitWidth
            + fwBadgeRect.implicitWidth
            + syncStatusBadge.implicitWidth
            + (deviceController.sdMountPath.length > 0 ? 220 : 0)
            + Kirigami.Units.largeSpacing * 2
            + 180
    )
    readonly property int expandedWindowWidth: Math.max(
        1700,
        compactWindowWidth + inspectorCard.minContentWidth + 8
    )
    readonly property int requiredWindowWidth: root.hasSelectedSlot
        ? root.expandedWindowWidth
        : root.compactWindowWidth
    width: compactWindowWidth
    height: 800
    minimumWidth: root.hasSelectedSlot ? root.expandedWindowWidth : root.compactWindowWidth
    minimumHeight: 680

    function syncWindowWidthToEditorState() {
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

    readonly property real fontScale: (deviceController && deviceController.uiTextScale > 0
        ? deviceController.uiTextScale
        : 100) / 100.0
    readonly property real sidebarScale: (deviceController && deviceController.sidebarTextScale > 0
        ? deviceController.sidebarTextScale
        : 100) / 100.0
    readonly property real canvasScale: (deviceController && deviceController.canvasTextScale > 0
        ? deviceController.canvasTextScale
        : 100) / 100.0
    readonly property real inspectorScale: (deviceController && deviceController.inspectorTextScale > 0
        ? deviceController.inspectorTextScale
        : 100) / 100.0
    readonly property real editorScale: (deviceController && deviceController.editorTextScale > 0
        ? deviceController.editorTextScale
        : 100) / 100.0
    readonly property real statusBarScale: (deviceController && deviceController.statusBarTextScale > 0
        ? deviceController.statusBarTextScale
        : 100) / 100.0
    readonly property real eventLogScale: (deviceController && deviceController.eventLogTextScale > 0
        ? deviceController.eventLogTextScale
        : 100) / 100.0

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
    property bool syncingWebApiScript: false
    property bool webApiEnabled: false
    readonly property var webApiMethods: ["POST", "GET", "PUT", "PATCH", "DELETE"]
    readonly property var webApiFkeys: ["F13", "F14", "F15", "F16", "F17", "F18", "F19", "F20", "F21", "F22", "F23", "F24"]

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
        "SYSTEM"
    ]
    readonly property var eventLogLevels: ["ALL", "INFO", "WARN", "ERROR"]

    function levelColorForLog(lvl) {
        if (lvl === "ERROR") return Kirigami.Theme.negativeTextColor
        if (lvl === "WARN") return Kirigami.Theme.neutralTextColor
        return Kirigami.Theme.highlightColor
    }

    function categoryColorForLog(cat) {
        if (cat === "DEVICE") return "#38bdf8"
        if (cat === "PROFILE") return "#a78bfa"
        if (cat === "AUTOSWITCH") return "#4ade80"
        if (cat === "SD_SYNC") return "#f59e0b"
        if (cat === "SCRIPT") return "#2dd4bf"
        if (cat === "WEB_API") return "#f472b6"
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
                unsyncedItemsModel.append({
                    kind: it.kind || "modified",
                    profileNumber: it.profileNumber || 0,
                    profileName: it.profileName || "",
                    summary: it.summary || "",
                    detailsText: detailsArr.map(function(d) { return "  • " + d }).join("\n")
                })
            }
        } catch (e) {
            root.unsyncedHeadline = qsTr("Unable to parse unsynced changes summary.")
            root.unsyncedTotalChanges = 0
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

    function buildWebApiConfigJson() {
        const methodIdx = Math.max(0, webApiMethodCombo.currentIndex)
        const fkeyIdx = Math.max(0, webApiFkeyCombo.currentIndex)
        const cfg = {
            enabled: webApiEnabledCheck.checked,
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
            if (isEnabled) {
                root.showWebApiBuilder = true
            }
        } catch (e) {
            root.webApiEnabled = false
            webApiEnabledCheck.checked = false
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

    function loadSlotFromController(slotNumber, slotTitle, slotColor) {
        if (slotNumber <= 0 || slotNumber > 58) {
            root.selectedSlotNumber = 0
            return
        }
        root.loadingSlotData = true
        root.selectedSlotNumber = slotNumber
        root.selectedSlotTitle = slotTitle
        root.selectedSlotColor = slotColor

        try {
            const metaJson = deviceController.getSlotMetaJson(slotNumber)
            const meta = JSON.parse(metaJson)
            label1Input.text = meta.l1 || ""
            label2Input.text = meta.l2 || ""
            allowAbortCheck.checked = !!meta.allowAbort
            disableRepeatCheck.checked = !!meta.disableAutorepeat
        } catch (e) {
            label1Input.text = ""
            label2Input.text = ""
            allowAbortCheck.checked = false
            disableRepeatCheck.checked = false
        }

        root.pressScriptBuffer = deviceController.getSlotScript(slotNumber, false)
        root.releaseScriptBuffer = deviceController.getSlotScript(slotNumber, true)
        scriptEditor.text = root.editingReleaseScript ? root.releaseScriptBuffer : root.pressScriptBuffer
        root.loadWebApiBuilderFromScript(scriptEditor.text)
        root.loadingSlotData = false
        root.updateCompilerStatus()
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
    }

    function flushPendingSlotEditsIfModified() {
        if (!root.hasSelectedSlot || root.loadingSlotData || !root.compileOk) {
            return
        }
        const currentPress = root.editingReleaseScript ? root.pressScriptBuffer : scriptEditor.text
        const currentRelease = root.editingReleaseScript ? scriptEditor.text : root.releaseScriptBuffer
        const currentL1 = label1Input.text.trim()
        const currentL2 = label2Input.text.trim()
        const currentAbort = allowAbortCheck.checked
        const currentNoRepeat = disableRepeatCheck.checked

        try {
            const meta = JSON.parse(deviceController.getSlotMetaJson(root.selectedSlotNumber))
            const savedL1 = meta.l1 || ""
            const savedL2 = meta.l2 || ""
            const savedAbort = !!meta.allowAbort
            const savedNoRepeat = !!meta.disableAutorepeat
            const savedPress = deviceController.getSlotScript(root.selectedSlotNumber, false)
            const savedRelease = deviceController.getSlotScript(root.selectedSlotNumber, true)

            if (currentL1 !== savedL1 ||
                currentL2 !== savedL2 ||
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
        root.isDarkSystemTheme = Kirigami.Theme.backgroundColor.hslLightness < 0.5
        deviceController.notifySystemDarkTheme(root.isDarkSystemTheme)
        root.refreshCommandReferenceModel()
        deviceController.refreshDevice()
        root.syncSidebarProfilesFromController()
        hardwareCanvas.syncProfilesFromController()
        root.refreshSelectedSlotFromController()
        Qt.callLater(function() {
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncProfilesFromController()
            root.refreshSelectedSlotFromController()
            if (deviceController.connected) {
                root.ensureActiveProfileVisibleInList()
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
            if (!root.selectingProfileFromClick && deviceController.autoSwitchEnabled) {
                root.ensureActiveProfileVisibleInList()
                Qt.callLater(root.ensureActiveProfileVisibleInList)
            }
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
        if (hexColor && hexColor.length > 0) {
            profileDefaultColorDialog.selectedColor = hexColor
        } else if (deviceController && deviceController.activeProfileBgColor.length > 0) {
            profileDefaultColorDialog.selectedColor = deviceController.activeProfileBgColor
        } else {
            profileDefaultColorDialog.selectedColor = "#00aaff"
        }
        profileDefaultColorDialog.open()
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
        title: qsTr("Create New Profile Group — DuckyDeck Studio")
        width: 440
        height: 245
        minimumWidth: 400
        minimumHeight: 230
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
                wrapMode: Text.WordWrap
                font.pixelSize: 12
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
                    font.pixelSize: 12
                    color: Kirigami.Theme.textColor
                }

                QQC2.TextField {
                    id: newGroupNameInput
                    Layout.fillWidth: true
                    placeholderText: qsTr("e.g. Work & Coding")
                    selectByMouse: true
                    font.pixelSize: 12
                    onAccepted: newProfileGroupDialog.commitCreateNewGroup()
                    Keys.onEscapePressed: newProfileGroupDialog.close()
                }

                QQC2.Label {
                    text: qsTr("Initial Profile #1:")
                    font.bold: true
                    font.pixelSize: 12
                    color: Kirigami.Theme.textColor
                }

                QQC2.TextField {
                    id: newGroupFirstProfileInput
                    Layout.fillWidth: true
                    maximumLength: 16
                    placeholderText: qsTr("Default")
                    selectByMouse: true
                    font.pixelSize: 12
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
        id: profileDefaultColorDialog
        title: qsTr("Select Default Button Color (BG_COLOR) for Profile #%1 (%2)")
            .arg(root.contextMenuProfileNumber)
            .arg(root.contextMenuProfileName)
        selectedColor: deviceController.activeProfileBgColor.length > 0
            ? deviceController.activeProfileBgColor
            : "#00aaff"
        onAccepted: {
            const c = selectedColor
            deviceController.setProfileBgColor(
                root.contextMenuProfileNumber,
                Math.round(c.r * 255),
                Math.round(c.g * 255),
                Math.round(c.b * 255)
            )
            root.syncSidebarProfilesFromController()
            hardwareCanvas.syncSlotsFromController()
        }
    }

    ColorDialog {
        id: selectedKeyColorDialog
        title: qsTr("Select Custom Button Color (SWCOLOR) for Key %1")
            .arg(root.selectedSlotNumber)
        selectedColor: root.selectedSlotColor
        onAccepted: {
            const c = selectedColor
            hardwareCanvas.updateSelectedKeyRgb(
                Math.round(c.r * 255),
                Math.round(c.g * 255),
                Math.round(c.b * 255)
            )
            root.selectedSlotColor = c.toString()
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
            icon.name: "checkmark"
            text: qsTr("Select Profile")
            onTriggered: {
                const savedY = profileSidebarList.contentY
                root.selectingProfileFromClick = true
                deviceController.selectProfile(root.contextMenuProfileNumber)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
                root.selectingProfileFromClick = false
            }
        }

        QQC2.MenuItem {
            icon.name: "edit-rename"
            text: qsTr("Rename Profile...")
            onTriggered: {
                root.openRenameProfileDialog(root.contextMenuProfileNumber, root.contextMenuProfileName)
            }
        }

        QQC2.MenuItem {
            icon.name: "color-picker"
            text: qsTr("Change Default LED Color (BG_COLOR)...")
            onTriggered: {
                profileDefaultColorDialog.open()
            }
        }

        QQC2.MenuItem {
            icon.name: "crosshairs"
            text: qsTr("Bind Window to This Profile (Pick Window)...")
            onTriggered: {
                const savedY = profileSidebarList.contentY
                root.selectingProfileFromClick = true
                deviceController.selectProfile(root.contextMenuProfileNumber)
                root.refreshSelectedSlotFromController()
                profileSidebarList.contentY = savedY
                root.selectingProfileFromClick = false
                deviceController.armWindowPicker(true)
            }
        }

        QQC2.MenuItem {
            icon.name: "preferences-system-windows"
            text: qsTr("Auto-Switch Rules...")
            onTriggered: autoSwitchDrawer.open()
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
            icon.name: "edit-delete"
            text: qsTr("Delete Profile...")
            enabled: profileSidebarList.count > 1
            onTriggered: {
                deleteProfileDialog.open()
            }
        }
    }

    Window {
        id: renameProfileDialog
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
                font.pixelSize: 12
                text: qsTr("New name for Profile #%1 (max 16 characters):").arg(renameProfileDialog.targetProfileNumber)
                color: Kirigami.Theme.textColor
            }

            QQC2.TextField {
                id: renameProfileInput
                Layout.fillWidth: true
                maximumLength: 16
                placeholderText: qsTr("Profile name...")
                selectByMouse: true
                font.pixelSize: 13
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

    component AreaTextScaleRow: Rectangle {
        id: scaleRow
        required property string areaKey
        required property string areaTitle
        required property string areaDescription
        required property int scaleValue

        Layout.fillWidth: true
        implicitHeight: rowCol.implicitHeight + 16
        radius: 6
        color: Kirigami.Theme.backgroundColor
        border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        border.width: 1

        ColumnLayout {
            id: rowCol
            anchors.fill: parent
            anchors.margins: 8
            spacing: 4

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 1

                    QQC2.Label {
                        text: scaleRow.areaTitle
                        font.bold: true
                        font.pixelSize: 12
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        text: scaleRow.areaDescription
                        font.pixelSize: 10
                        color: Kirigami.Theme.disabledTextColor
                        elide: Text.ElideRight
                        Layout.fillWidth: true
                    }
                }

                QQC2.Button {
                    text: "A−"
                    implicitWidth: 32
                    implicitHeight: 26
                    font.bold: true
                    font.pixelSize: 11
                    enabled: scaleRow.scaleValue > 75
                    onClicked: deviceController.applyAreaTextScale(scaleRow.areaKey, Math.max(75, scaleRow.scaleValue - 5))
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Decrease %1 text size (-5%)").arg(scaleRow.areaTitle)
                }

                QQC2.Slider {
                    id: areaSlider
                    Layout.preferredWidth: 150
                    from: 75
                    to: 160
                    stepSize: 5
                    snapMode: QQC2.Slider.SnapAlways
                    value: scaleRow.scaleValue
                    onMoved: {
                        const stepped = Math.round(value / 5) * 5
                        if (stepped !== scaleRow.scaleValue) {
                            deviceController.applyAreaTextScale(scaleRow.areaKey, stepped)
                        }
                    }
                }

                QQC2.Button {
                    text: "A+"
                    implicitWidth: 32
                    implicitHeight: 26
                    font.bold: true
                    font.pixelSize: 11
                    enabled: scaleRow.scaleValue < 160
                    onClicked: deviceController.applyAreaTextScale(scaleRow.areaKey, Math.min(160, scaleRow.scaleValue + 5))
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Increase %1 text size (+5%)").arg(scaleRow.areaTitle)
                }

                Rectangle {
                    radius: 4
                    color: scaleRow.scaleValue === 100
                        ? Kirigami.Theme.alternateBackgroundColor
                        : Qt.alpha(Kirigami.Theme.highlightColor, 0.18)
                    border.color: scaleRow.scaleValue === 100
                        ? Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                        : Qt.alpha(Kirigami.Theme.highlightColor, 0.55)
                    border.width: 1
                    implicitWidth: 48
                    implicitHeight: 24

                    QQC2.Label {
                        anchors.centerIn: parent
                        text: qsTr("%1%").arg(scaleRow.scaleValue)
                        font.family: "monospace"
                        font.bold: true
                        font.pixelSize: 11
                        color: scaleRow.scaleValue === 100
                            ? Kirigami.Theme.textColor
                            : Kirigami.Theme.highlightColor
                    }
                }

                QQC2.Button {
                    text: qsTr("↺")
                    implicitWidth: 28
                    implicitHeight: 26
                    enabled: scaleRow.scaleValue !== 100
                    onClicked: deviceController.applyAreaTextScale(scaleRow.areaKey, 100)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Reset %1 text size to 100%").arg(scaleRow.areaTitle)
                }
            }
        }
    }

    // Settings Window (Appearance, System Integration, and Per-Area Text Sizes)
    Window {
        id: settingsDialog
        title: qsTr("Settings — DuckyDeck Studio")
        width: 660
        height: 620
        minimumWidth: 580
        minimumHeight: 540
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        function open() {
            settingsDialog.show()
            settingsDialog.raise()
            settingsDialog.requestActivate()
        }

        Shortcut {
            sequence: "Escape"
            onActivated: settingsDialog.close()
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
                    icon.name: "edit-reset"
                    text: qsTr("Reset All Text Sizes (100%)")
                    enabled: deviceController.sidebarTextScale !== 100
                        || deviceController.canvasTextScale !== 100
                        || deviceController.inspectorTextScale !== 100
                        || deviceController.editorTextScale !== 100
                        || deviceController.statusBarTextScale !== 100
                        || deviceController.eventLogTextScale !== 100
                    onClicked: deviceController.resetAllTextScales()
                }
            }

            QQC2.ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true

                ColumnLayout {
                    width: Math.max(520, settingsDialog.width - 48)
                    spacing: 12

                    // Section 1: Text Sizes by Application Area
                    Rectangle {
                        Kirigami.Theme.colorSet: Kirigami.Theme.View
                        Kirigami.Theme.inherit: false
                        Layout.fillWidth: true
                        implicitHeight: textSizesCol.implicitHeight + 20
                        radius: 8
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                        border.width: 1

                        ColumnLayout {
                            id: textSizesCol
                            anchors.fill: parent
                            anchors.margins: 10
                            spacing: 8

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 6

                                QQC2.Label {
                                    text: qsTr("TEXT SIZES BY APPLICATION AREA")
                                    font.bold: true
                                    font.pixelSize: 11
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                }

                                QQC2.Label {
                                    text: qsTr("Preset All:")
                                    font.pixelSize: 11
                                    color: Kirigami.Theme.disabledTextColor
                                }

                                QQC2.Button {
                                    text: "90%"
                                    implicitHeight: 24
                                    font.pixelSize: 10
                                    onClicked: deviceController.applyAreaTextScale("all", 90)
                                }

                                QQC2.Button {
                                    text: "100%"
                                    implicitHeight: 24
                                    font.pixelSize: 10
                                    onClicked: deviceController.applyAreaTextScale("all", 100)
                                }

                                QQC2.Button {
                                    text: "115%"
                                    implicitHeight: 24
                                    font.pixelSize: 10
                                    onClicked: deviceController.applyAreaTextScale("all", 115)
                                }

                                QQC2.Button {
                                    text: "130%"
                                    implicitHeight: 24
                                    font.pixelSize: 10
                                    onClicked: deviceController.applyAreaTextScale("all", 130)
                                }
                            }

                            AreaTextScaleRow {
                                areaKey: "sidebar"
                                areaTitle: qsTr("Profiles & Auto-Switcher Sidebar")
                                areaDescription: qsTr("Left column profile list, filter search bar, active window monitor, and sidebar buttons")
                                scaleValue: deviceController.sidebarTextScale
                            }

                            AreaTextScaleRow {
                                areaKey: "canvas"
                                areaTitle: qsTr("Hardware Pad & Key Labels")
                                areaDescription: qsTr("Center column keycap labels (L1/L2), slot numbers, rotary encoders, and expansion channels")
                                scaleValue: deviceController.canvasTextScale
                            }

                            AreaTextScaleRow {
                                areaKey: "inspector"
                                areaTitle: qsTr("Key Inspector & Action Controls")
                                areaDescription: qsTr("Right column slot header, L1/L2 fields, On Press / On Release / Web API builder, and footer")
                                scaleValue: deviceController.inspectorTextScale
                            }

                            AreaTextScaleRow {
                                areaKey: "editor"
                                areaTitle: qsTr("duckyScript Code Editor")
                                areaDescription: qsTr("Monospace duckyScript 3 code editor and synchronized line-number gutter")
                                scaleValue: deviceController.editorTextScale
                            }

                            AreaTextScaleRow {
                                areaKey: "statusbar"
                                areaTitle: qsTr("Bottom Status Bar")
                                areaDescription: qsTr("Connection status pill, Firmware/SN badge, SD Sync badge, SD path, and live status message")
                                scaleValue: deviceController.statusBarTextScale
                            }

                            AreaTextScaleRow {
                                areaKey: "eventlog"
                                areaTitle: qsTr("System Event Log")
                                areaDescription: qsTr("Event log viewer timestamps, level/category badges, filter controls, and diagnostic messages")
                                scaleValue: deviceController.eventLogTextScale
                            }
                        }
                    }

                    // Section 2: Appearance & System Integration
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
                                font.pixelSize: 11
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
                                    font.pixelSize: 12
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
                                    font.pixelSize: 12
                                    color: Kirigami.Theme.textColor
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.Button {
                                        text: qsTr("◐ Auto (KDE)")
                                        checkable: true
                                        checked: deviceController.trayIconMode === "auto"
                                        highlighted: checked
                                        onClicked: deviceController.applyTrayIconMode("auto")
                                    }

                                    QQC2.Button {
                                        text: qsTr("🌙 Dark Panel")
                                        checkable: true
                                        checked: deviceController.trayIconMode === "dark"
                                        highlighted: checked
                                        onClicked: deviceController.applyTrayIconMode("dark")
                                    }

                                    QQC2.Button {
                                        text: qsTr("☀ Light Panel")
                                        checkable: true
                                        checked: deviceController.trayIconMode === "light"
                                        highlighted: checked
                                        onClicked: deviceController.applyTrayIconMode("light")
                                    }

                                    Item { Layout.fillWidth: true }
                                }
                            }

                            Rectangle {
                                Layout.fillWidth: true
                                height: 1
                                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 16

                                QQC2.CheckBox {
                                    text: qsTr("Start on KDE System Login (Autostart)")
                                    checked: deviceController.autostartEnabled
                                    onToggled: deviceController.applyAutostartEnabled(checked)
                                }

                                QQC2.CheckBox {
                                    text: qsTr("Keep Running in Taskbar on Window Close")
                                    checked: deviceController.closeToTray
                                    onToggled: deviceController.setCloseToTrayEnabled(checked)
                                }

                                Item { Layout.fillWidth: true }
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
            enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
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
            enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
            onTriggered: root.triggerSyncToDeviceSd()
        }

        QQC2.MenuItem {
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
            icon.name: "folder-open"
            text: qsTr("Open SD Folder...")
            onTriggered: sdFolderDialog.open()
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
                ? qsTr("⬆ Check for Firmware Updates (v%1 Available)...").arg(deviceController.latestFirmwareVersion)
                : qsTr("Check for Firmware Updates...")
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
        title: qsTr("System Event Log — DuckyDeck Studio v%1").arg(deviceController.appVersion)
        width: 860
        height: 540
        minimumWidth: 680
        minimumHeight: 400
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
                            font.pixelSize: Math.round(15 * root.eventLogScale)
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
                                font.pixelSize: Math.round(11 * root.eventLogScale)
                                color: Kirigami.Theme.highlightColor
                            }
                        }

                        Item { Layout.fillWidth: true }

                        QQC2.Button {
                            icon.name: "view-refresh"
                            text: qsTr("Refresh")
                            implicitHeight: Math.round(26 * root.eventLogScale)
                            font.pixelSize: Math.round(11 * root.eventLogScale)
                            onClicked: {
                                deviceController.refreshEventLog()
                                root.refreshEventLogModel()
                            }
                        }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        text: qsTr("Log file: %1").arg(deviceController.logFilePath)
                        font.family: "monospace"
                        font.pixelSize: Math.round(11 * root.eventLogScale)
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
                    implicitHeight: Math.round(30 * root.eventLogScale)
                    font.pixelSize: Math.round(12 * root.eventLogScale)
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
                    font.pixelSize: Math.round(11 * root.eventLogScale)
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
                        font.pixelSize: Math.round(11 * root.eventLogScale)
                        font.bold: isSel
                        onClicked: {
                            root.eventLogLevelFilter = modelData
                            root.refreshEventLogModel()
                        }
                    }
                }
            }

            // Category Filter Pills Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 4

                QQC2.Label {
                    text: qsTr("Category:")
                    font.pixelSize: Math.round(11 * root.eventLogScale)
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
                        font.pixelSize: Math.round(10 * root.eventLogScale)
                        font.bold: isSelCat
                        onClicked: {
                            root.eventLogCategoryFilter = modelData
                            root.refreshEventLogModel()
                        }
                    }
                }

                Item { Layout.fillWidth: true }
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
                                    font.pixelSize: Math.round(11 * root.eventLogScale)
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
                                        font.pixelSize: Math.round(9 * root.eventLogScale)
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
                                        font.pixelSize: Math.round(9 * root.eventLogScale)
                                        color: root.categoryColorForLog(logRow.category)
                                    }
                                }

                                QQC2.Label {
                                    text: logRow.message
                                    font.family: "monospace"
                                    font.pixelSize: Math.round(11 * root.eventLogScale)
                                    color: logRow.level === "ERROR"
                                        ? Kirigami.Theme.negativeTextColor
                                        : Kirigami.Theme.textColor
                                    Layout.fillWidth: true
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
                    font.pixelSize: Math.round(11 * root.eventLogScale)
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
                    font.pixelSize: Math.round(11 * root.eventLogScale)
                    onClicked: exportEventLogFileDialog.open()
                }

                QQC2.Button {
                    icon.name: "edit-clear-history"
                    text: qsTr("Clear Log")
                    font.pixelSize: Math.round(11 * root.eventLogScale)
                    onClicked: {
                        deviceController.clearEventLog()
                        root.refreshEventLogModel()
                    }
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    text: qsTr("Close")
                    font.pixelSize: Math.round(11 * root.eventLogScale)
                    onClicked: eventLogDialog.close()
                }
            }
        }
    }

    Window {
        id: aboutDialog
        title: qsTr("About DuckyDeck Studio v%1").arg(deviceController.appVersion)
        width: 540
        height: 410
        minimumWidth: 500
        minimumHeight: 380
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

                        QQC2.Label {
                            text: qsTr("🦆 DuckyDeck Studio")
                            font.bold: true
                            font.pixelSize: 20
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
                                font.pixelSize: 12
                                color: Kirigami.Theme.highlightColor
                            }
                        }

                        Item { Layout.fillWidth: true }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        text: qsTr("KDE Plasma 6 / Kirigami Configurator, DSVM v2 Compiler & Wayland Auto-Switcher for duckyPad Pro & duckyPad (2020)")
                        font.pixelSize: 12
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

                GridLayout {
                    anchors.fill: parent
                    anchors.margins: 14
                    columns: 2
                    rowSpacing: 10
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
                    }

                    QQC2.Label {
                        text: qsTr("GitHub:")
                        font.bold: true
                        color: Kirigami.Theme.disabledTextColor
                    }
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            id: githubLinkLbl
                            text: deviceController.appGithubUrl
                            font.family: "monospace"
                            font.underline: githubMouse.containsMouse
                            color: Kirigami.Theme.highlightColor
                            elide: Text.ElideMiddle
                            Layout.fillWidth: true

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
                            font.pixelSize: 10
                            onClicked: root.copyTextToClipboard(deviceController.appGithubUrl)
                        }
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
                    }

                    QQC2.Label {
                        text: qsTr("Event Log:")
                        font.bold: true
                        color: Kirigami.Theme.disabledTextColor
                    }
                    QQC2.Label {
                        text: deviceController.logFilePath
                        font.family: "monospace"
                        font.pixelSize: 11
                        color: Kirigami.Theme.disabledTextColor
                        elide: Text.ElideMiddle
                        Layout.fillWidth: true
                    }
                }
            }

            // Footer Buttons
            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "internet-web-browser"
                    text: qsTr("Open GitHub Repository")
                    onClicked: Qt.openUrlExternally(deviceController.appGithubUrl)
                }

                QQC2.Button {
                    icon.name: "view-list-text"
                    text: qsTr("System Event Log...")
                    onClicked: {
                        aboutDialog.close()
                        eventLogDialog.openLogWindow()
                    }
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
        title: qsTr("Unsynced Profile Changes Summary — DuckyDeck Studio")
        width: Math.max(
            740,
            Math.ceil(unsyncedFooterRow.implicitWidth + 64),
            Math.ceil(unsyncedTitleMetrics.width + 180),
            Math.ceil(unsyncedBannerTitle.implicitWidth + 72)
        )
        height: 480
        minimumWidth: Math.max(
            700,
            Math.ceil(unsyncedFooterRow.implicitWidth + 56),
            Math.ceil(unsyncedTitleMetrics.width + 160),
            Math.ceil(unsyncedBannerTitle.implicitWidth + 64)
        )
        minimumHeight: 360
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
            font.pixelSize: Math.round(13 * root.fontScale)
            text: unsyncedChangesDialog.title
        }

        function open() {
            if (unsyncedChangesDialog.width < unsyncedChangesDialog.minimumWidth) {
                unsyncedChangesDialog.width = unsyncedChangesDialog.minimumWidth
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
                        font.pixelSize: Math.round(13 * root.fontScale)
                        text: deviceController.profileSyncState === "synced"
                            ? qsTr("✓ All Profiles Match duckyPad microSD Card")
                            : qsTr("⚠ Unsynced Changes Pending microSD Sync")
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        font.pixelSize: Math.round(12 * root.fontScale)
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
                    QQC2.ScrollBar.vertical: QQC2.ScrollBar { policy: QQC2.ScrollBar.AsNeeded }

                    delegate: Rectangle {
                        required property string kind
                        required property int profileNumber
                        required property string profileName
                        required property string summary
                        required property string detailsText

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
                            spacing: 4

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
                                        font.pixelSize: Math.round(10 * root.fontScale)
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
                                    font.pixelSize: Math.round(12 * root.fontScale)
                                    text: summary
                                    color: Kirigami.Theme.textColor
                                }
                            }

                            QQC2.Label {
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                                font.family: "monospace"
                                font.pixelSize: Math.round(11 * root.fontScale)
                                text: detailsText
                                color: Kirigami.Theme.textColor
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
                        font.pixelSize: Math.round(12 * root.fontScale)
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
                    enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
                    highlighted: deviceController.profileSyncState !== "synced"
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
        title: qsTr("duckyPad Firmware Update Check — DuckyDeck Studio")
        width: Math.max(
            700,
            Math.ceil(fwFooterRow.implicitWidth + 64),
            Math.ceil(fwTitleMetrics.width + 180)
        )
        height: 320
        minimumWidth: Math.max(
            660,
            Math.ceil(fwFooterRow.implicitWidth + 56),
            Math.ceil(fwTitleMetrics.width + 160)
        )
        minimumHeight: 290
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        onMinimumWidthChanged: {
            if (firmwareUpdateDialog.width < firmwareUpdateDialog.minimumWidth) {
                firmwareUpdateDialog.width = firmwareUpdateDialog.minimumWidth
            }
        }

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        TextMetrics {
            id: fwTitleMetrics
            font.bold: true
            font.pixelSize: Math.round(13 * root.fontScale)
            text: firmwareUpdateDialog.title
        }

        function open() {
            if (firmwareUpdateDialog.width < firmwareUpdateDialog.minimumWidth) {
                firmwareUpdateDialog.width = firmwareUpdateDialog.minimumWidth
            }
            firmwareUpdateDialog.show()
            firmwareUpdateDialog.raise()
            firmwareUpdateDialog.requestActivate()
            Qt.callLater(function() {
                if (firmwareUpdateDialog.width < firmwareUpdateDialog.minimumWidth) {
                    firmwareUpdateDialog.width = firmwareUpdateDialog.minimumWidth
                }
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
            anchors.margins: 16
            spacing: 14
            focus: true
            Keys.onEscapePressed: firmwareUpdateDialog.close()

            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: deviceController.firmwareUpdateAvailable
                    ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.16)
                    : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.14)
                border.color: deviceController.firmwareUpdateAvailable
                    ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.5)
                    : Qt.alpha(Kirigami.Theme.positiveTextColor, 0.45)
                border.width: 1
                implicitHeight: fwBannerCol.implicitHeight + 20

                ColumnLayout {
                    id: fwBannerCol
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 6

                    QQC2.Label {
                        Layout.fillWidth: true
                        font.bold: true
                        font.pixelSize: 13
                        text: deviceController.firmwareCheckBusy
                            ? qsTr("Checking GitHub for official firmware releases...")
                            : (deviceController.firmwareUpdateAvailable
                                ? qsTr("⬆ Firmware Update Available!")
                                : qsTr("✓ Firmware Status"))
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        font.pixelSize: 12
                        text: deviceController.firmwareCheckSummary.length > 0
                            ? deviceController.firmwareCheckSummary
                            : qsTr("Click 'Check Again' to query the official dekuNukem GitHub repository.")
                        color: Kirigami.Theme.textColor
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                implicitHeight: fwGrid.implicitHeight + 20

                GridLayout {
                    id: fwGrid
                    anchors.fill: parent
                    anchors.margins: 10
                    columns: 2
                    rowSpacing: 8
                    columnSpacing: 14

                    QQC2.Label {
                        text: qsTr("Device Model:")
                        color: Kirigami.Theme.disabledTextColor
                        font.pixelSize: 12
                    }
                    QQC2.Label {
                        Layout.fillWidth: true
                        text: deviceController.modelName
                        font.bold: true
                        font.pixelSize: 12
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        text: qsTr("Installed Firmware:")
                        color: Kirigami.Theme.disabledTextColor
                        font.pixelSize: 12
                    }
                    QQC2.Label {
                        Layout.fillWidth: true
                        text: deviceController.firmwareVersion !== "-"
                            ? qsTr("v%1").arg(deviceController.firmwareVersion)
                            : qsTr("Not Connected")
                        font.family: "monospace"
                        font.bold: true
                        font.pixelSize: 12
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        text: qsTr("Latest Official Release:")
                        color: Kirigami.Theme.disabledTextColor
                        font.pixelSize: 12
                    }
                    QQC2.Label {
                        Layout.fillWidth: true
                        text: deviceController.firmwareCheckBusy
                            ? qsTr("Checking...")
                            : (deviceController.latestFirmwareVersion !== "-"
                                ? qsTr("v%1").arg(deviceController.latestFirmwareVersion)
                                : qsTr("-"))
                        font.family: "monospace"
                        font.bold: true
                        font.pixelSize: 12
                        color: deviceController.firmwareUpdateAvailable
                            ? Kirigami.Theme.neutralTextColor
                            : Kirigami.Theme.positiveTextColor
                    }
                }
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                id: fwFooterRow
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "view-refresh"
                    text: qsTr("Check Again")
                    enabled: !deviceController.firmwareCheckBusy
                    onClicked: deviceController.checkFirmwareUpdate(true)
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    icon.name: "download"
                    text: qsTr("Download Firmware")
                    enabled: deviceController.latestFirmwareVersion !== "-"
                    highlighted: deviceController.firmwareUpdateAvailable
                    onClicked: deviceController.downloadFirmwareBinary()
                }

                QQC2.Button {
                    icon.name: "help-contents"
                    text: qsTr("Update Guide")
                    onClicked: deviceController.openFirmwareUpdatePage()
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
            font.pixelSize: Math.round(13 * root.fontScale)
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
            enabled: sdSyncDialog.visible && !deviceController.sdSyncBusy
            onActivated: sdSyncDialog.close()
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
                            font.pixelSize: 13
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
                            font.pixelSize: 12
                            color: deviceController.sdSyncResultState === "success"
                                ? Kirigami.Theme.positiveTextColor
                                : Kirigami.Theme.highlightColor
                        }
                    }

                    QQC2.Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        font.pixelSize: 12
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
            QQC2.ProgressBar {
                Layout.fillWidth: true
                from: 0
                to: 100
                value: deviceController.sdSyncProgress
                Behavior on value {
                    NumberAnimation { duration: 150 }
                }
            }

            QQC2.Label {
                text: deviceController.sdSyncLog.length > 0
                    ? qsTr("STEP-BY-STEP SYNC & VALIDATION LOG")
                    : qsTr("UNSYNCED CHANGES SUMMARY (PREVIEW BEFORE SYNC)")
                font.bold: true
                font.pixelSize: 11
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

                    QQC2.TextArea {
                        id: syncLogArea
                        readOnly: true
                        wrapMode: TextEdit.Wrap
                        font.family: "monospace"
                        font.pixelSize: Math.round(12 * root.fontScale)
                        color: Kirigami.Theme.textColor
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
                    enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
                    highlighted: deviceController.profileSyncState !== "synced"
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
                    font.pixelSize: 16
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
                            font.pixelSize: 12
                            color: Kirigami.Theme.textColor
                        }
                    }
                }

                QQC2.Button {
                    icon.name: "color-picker"
                    text: qsTr("Default Color")
                    implicitHeight: 28
                    font.pixelSize: 11
                    onClicked: root.openProfileDefaultColorPicker(
                        deviceController.activeProfile,
                        deviceController.activeProfileName,
                        deviceController.activeProfileBgColor
                    )
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Change Default Button Color (BG_COLOR) for Profile #%1 (%2)")
                        .arg(deviceController.activeProfile)
                        .arg(deviceController.activeProfileName)
                }

                Item { Layout.fillWidth: true }

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
                                font.pixelSize: Math.round(11 * root.sidebarScale)
                                color: Kirigami.Theme.disabledTextColor
                                Layout.fillWidth: true
                            }

                            QQC2.Button {
                                text: qsTr("↻ SD")
                                implicitHeight: Math.round(22 * root.sidebarScale)
                                font.pixelSize: Math.round(10 * root.sidebarScale)
                                onClicked: {
                                    const found = deviceController.scanMountedSdCard()
                                    if (found.length > 0) {
                                        root.refreshSelectedSlotFromController()
                                    } else {
                                        sdFolderDialog.open()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Scan / Reload duckyPad SD Card")
                            }
                        }

                        // Filter Search Input
                        QQC2.TextField {
                            id: profileFilterField
                            Layout.fillWidth: true
                            placeholderText: qsTr("Filter profiles or apps (drag ⠿ to reorder)...")
                            selectByMouse: true
                            font.pixelSize: Math.round(11 * root.sidebarScale)
                            onTextChanged: {
                                root.profileFilterText = text
                                root.syncSidebarProfilesFromController()
                            }
                        }

                        // Scrollable 64-Profile List with Drag-and-Drop Reordering
                        QQC2.ScrollView {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            clip: true

                            ListView {
                                id: profileSidebarList
                                model: sidebarProfileModel
                                spacing: 4
                                highlightFollowsCurrentItem: false

                                property int draggedProfileIndex: -1
                                property int dropTargetIndex: -1
                                readonly property bool canReorder: root.profileFilterText.trim().length === 0

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
                                        || quickColorBtn.hovered
                                        || quickRenameBtn.hovered
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

                                    width: profileSidebarList.width - 8
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
                                            : Qt.ArrowCursor

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
                                                : (profDelegate.isRowHovered ? Kirigami.Theme.alternateBackgroundColor : "transparent"))
                                        border.width: (profDelegate.isCurrent || profDelegate.isBeingDragged) ? 1.5 : 1
                                        border.color: (profDelegate.isCurrent || profDelegate.isBeingDragged)
                                            ? Kirigami.Theme.highlightColor
                                            : (profDelegate.isRowHovered ? profilesSidebar.frameBorder : "transparent")

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
                                                font.pixelSize: Math.round(14 * root.sidebarScale)
                                                color: profDelegate.isRowHovered || profDelegate.isCurrent || profDelegate.isBeingDragged
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
                                                font.pixelSize: Math.round(15 * root.sidebarScale)
                                                color: Kirigami.Theme.textColor
                                                elide: Text.ElideRight
                                                Layout.fillWidth: true
                                            }

                                            // Quick Color & Rename buttons on hover
                                            RowLayout {
                                                visible: profDelegate.isRowHovered && !profMouse.dragging
                                                spacing: 2

                                                QQC2.ToolButton {
                                                    id: quickColorBtn
                                                    implicitWidth: 24
                                                    implicitHeight: 24
                                                    padding: 0
                                                    contentItem: Text {
                                                        text: "🎨"
                                                        font.pixelSize: 12
                                                        horizontalAlignment: Text.AlignHCenter
                                                        verticalAlignment: Text.AlignVCenter
                                                    }
                                                    onClicked: {
                                                        root.openProfileDefaultColorPicker(
                                                            profDelegate.profNumber,
                                                            profDelegate.profName,
                                                            profDelegate.profColor.toString()
                                                        )
                                                    }
                                                    QQC2.ToolTip.visible: hovered
                                                    QQC2.ToolTip.text: qsTr("Change Default Button Color (BG_COLOR)")
                                                }

                                                QQC2.ToolButton {
                                                    id: quickRenameBtn
                                                    implicitWidth: 24
                                                    implicitHeight: 24
                                                    padding: 0
                                                    contentItem: Text {
                                                        text: "✎"
                                                        font.pixelSize: 13
                                                        color: Kirigami.Theme.textColor
                                                        horizontalAlignment: Text.AlignHCenter
                                                        verticalAlignment: Text.AlignVCenter
                                                    }
                                                    onClicked: {
                                                        root.openRenameProfileDialog(profDelegate.profNumber, profDelegate.profName)
                                                    }
                                                    QQC2.ToolTip.visible: hovered
                                                    QQC2.ToolTip.text: qsTr("Rename Profile")
                                                }
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
                                                    font.pixelSize: Math.round(10 * root.sidebarScale)
                                                    font.bold: true
                                                    color: Kirigami.Theme.highlightColor
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
                                                    font.pixelSize: Math.round(10 * root.sidebarScale)
                                                    color: Kirigami.Theme.disabledTextColor
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Inline New Profile Creator (when toggled)
                        RowLayout {
                            Layout.fillWidth: true
                            visible: root.showNewProfileInput
                            spacing: 4

                            QQC2.TextField {
                                id: newProfileNameField
                                Layout.fillWidth: true
                                maximumLength: 16
                                placeholderText: qsTr("Profile name...")
                                font.pixelSize: Math.round(11 * root.sidebarScale)
                                onAccepted: createProfBtn.clicked()
                            }

                            QQC2.Button {
                                id: createProfBtn
                                text: qsTr("Add")
                                implicitHeight: Math.round(26 * root.sidebarScale)
                                font.pixelSize: Math.round(11 * root.sidebarScale)
                                highlighted: true
                                enabled: newProfileNameField.text.trim().length > 0
                                onClicked: {
                                    if (deviceController.createNewProfile(newProfileNameField.text.trim())) {
                                        newProfileNameField.text = ""
                                        root.showNewProfileInput = false
                                        root.refreshSelectedSlotFromController()
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
                                font.pixelSize: Math.round(11 * root.sidebarScale)
                                onClicked: {
                                    root.showNewProfileInput = !root.showNewProfileInput
                                    if (root.showNewProfileInput) {
                                        newProfileNameField.forceActiveFocus()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Create a new profile (Right-click a profile for Export & actions)")
                            }

                            QQC2.Button {
                                Layout.fillWidth: true
                                icon.name: "document-import"
                                text: qsTr("Import")
                                font.pixelSize: Math.round(11 * root.sidebarScale)
                                onClicked: importProfileFileDialog.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Import a profile from .duckyprofile.json or config.txt")
                            }

                            QQC2.Button {
                                id: profileGroupsMenuBtn
                                Layout.fillWidth: true
                                icon.name: "folder"
                                text: qsTr("Groups ▾")
                                font.pixelSize: Math.round(11 * root.sidebarScale)
                                onClicked: profileGroupMenu.popup(
                                    profileGroupsMenuBtn,
                                    0,
                                    -profileGroupMenu.implicitHeight
                                )
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Create, Load, or Save All Profiles as a Profile Group (.duckygroup.json)")

                                QQC2.Menu {
                                    id: profileGroupMenu
                                    title: qsTr("Profile Groups")

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
                                    text: root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                        ? qsTr("AUTO-SWITCHER (PAUSED)")
                                        : qsTr("AUTO-SWITCHER")
                                    font.bold: true
                                    font.pixelSize: Math.round(11 * root.sidebarScale)
                                    color: root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                        ? Kirigami.Theme.neutralTextColor
                                        : Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                }

                                QQC2.Switch {
                                    checked: deviceController.autoSwitchEnabled && !root.hasSelectedSlot
                                    enabled: !root.hasSelectedSlot
                                    implicitHeight: 22
                                    onToggled: {
                                        deviceController.setAutoSwitchConfig(
                                            checked,
                                            deviceController.defaultFallbackProfile
                                        )
                                    }
                                    QQC2.ToolTip.visible: hovered
                                    QQC2.ToolTip.text: root.hasSelectedSlot
                                        ? qsTr("Auto-Switcher is disabled while the key editor is open to keep the edited key in context")
                                        : qsTr("Toggle automatic profile switching")
                                }
                            }

                            // Active Window Status Box
                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: activeWinCol.implicitHeight + 12
                                radius: 6
                                color: deviceController.windowPickerArmed
                                    ? Qt.tint(Kirigami.Theme.alternateBackgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.25))
                                    : Kirigami.Theme.alternateBackgroundColor
                                border.color: deviceController.windowPickerArmed
                                    ? Kirigami.Theme.highlightColor
                                    : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                        ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
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
                                                ? qsTr("⏸ Paused (Editing Key %1)").arg(root.selectedSlotNumber)
                                                : qsTr("Active: %1").arg(deviceController.activeWindowClass))
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pixelSize: Math.round(11 * root.sidebarScale)
                                        color: deviceController.windowPickerArmed
                                            ? Kirigami.Theme.highlightColor
                                            : (root.hasSelectedSlot && deviceController.autoSwitchEnabled
                                                ? Kirigami.Theme.neutralTextColor
                                                : Kirigami.Theme.textColor)
                                        elide: Text.ElideRight
                                        Layout.fillWidth: true
                                    }

                                    QQC2.Label {
                                        visible: deviceController.activeWindowTitle.length > 0
                                        text: deviceController.activeWindowTitle
                                        font.pixelSize: Math.round(9 * root.sidebarScale)
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
                                font.pixelSize: Math.round(11 * root.sidebarScale)
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
                    QQC2.SplitView.preferredWidth: requiredWidthForFullHeight
                    QQC2.SplitView.minimumWidth: requiredWidthForFullHeight
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
                        if (newSelectedSlot >= 1 && newSelectedSlot <= 58) {
                            root.loadSlotFromController(
                                newSelectedSlot,
                                hardwareCanvas.slotTitleFor(newSelectedSlot),
                                hardwareCanvas.slotColorFor(newSelectedSlot)
                            )
                        } else {
                            root.refreshSelectedSlotFromController()
                        }
                    }

                    onRenameProfileRequested: (profileNumber, currentName) => {
                        root.openRenameProfileDialog(profileNumber, currentName)
                    }
                }

                // Column 3: Right Inspector Card matching system View colorSet
                Rectangle {
                    id: inspectorCard
                    visible: root.hasSelectedSlot
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    Kirigami.Theme.inherit: false
                    readonly property int minContentWidth: Math.max(
                        560,
                        Math.ceil(Math.max(
                            inspectorHeaderRow.implicitWidth,
                            inspectorLabelsRow.implicitWidth,
                            inspectorTabsRow.implicitWidth,
                            inspectorFooterRow.implicitWidth
                        )) + Kirigami.Units.mediumSpacing * 2 + 12
                    )
                    QQC2.SplitView.fillWidth: true
                    QQC2.SplitView.fillHeight: true
                    QQC2.SplitView.minimumWidth: minContentWidth
                    radius: Kirigami.Units.smallSpacing
                    color: Kirigami.Theme.backgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                    border.width: 1
                    clip: true

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.mediumSpacing
                        spacing: Kirigami.Units.smallSpacing

                        // Inspector Header
                        RowLayout {
                            id: inspectorHeaderRow
                            Layout.fillWidth: true
                            spacing: 8

                            Rectangle {
                                width: Math.round(28 * root.inspectorScale)
                                height: Math.round(28 * root.inspectorScale)
                                radius: 6
                                color: Kirigami.Theme.alternateBackgroundColor
                                border.color: root.selectedSlotColor
                                border.width: 2

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    text: root.selectedSlotNumber
                                    font.bold: true
                                    font.pixelSize: Math.round(11 * root.inspectorScale)
                                    color: Kirigami.Theme.textColor
                                }
                            }

                            ColumnLayout {
                                spacing: 1
                                Layout.fillWidth: true
                                Layout.minimumWidth: 60

                                QQC2.Label {
                                    id: inspectorTitle
                                    font.weight: Font.DemiBold
                                    font.pixelSize: Math.round(15 * root.inspectorScale)
                                    text: qsTr("%1 — duckyScript 3").arg(root.selectedSlotTitle)
                                    color: Kirigami.Theme.textColor
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                }

                                QQC2.Label {
                                    text: deviceController.sdMountPath.length > 0
                                        ? qsTr("Profile #%1 (%2) • SD: %3")
                                            .arg(deviceController.activeProfile)
                                            .arg(deviceController.activeProfileName)
                                            .arg(deviceController.sdMountPath)
                                        : (deviceController.connected
                                            ? qsTr("Profile #%1 (%2) • %3 (SN: %4)")
                                                .arg(deviceController.activeProfile)
                                                .arg(deviceController.activeProfileName)
                                                .arg(deviceController.modelName)
                                                .arg(deviceController.serialNumber)
                                            : qsTr("RGB LED: %1 • Target Slot: %2 (Disconnected)")
                                                .arg(root.selectedSlotColor)
                                                .arg(root.selectedSlotNumber))
                                    font.pixelSize: Math.round(11 * root.inspectorScale)
                                    color: Kirigami.Theme.disabledTextColor
                                    Layout.fillWidth: true
                                    elide: Text.ElideMiddle
                                }
                            }

                            QQC2.Button {
                                text: qsTr("✨ AI")
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                onClicked: aiModal.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Open AI Macro Architect")
                            }

                            QQC2.ToolButton {
                                text: "✕"
                                implicitWidth: Math.round(28 * root.inspectorScale)
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pixelSize: Math.round(12 * root.inspectorScale)
                                onClicked: root.clearSlotSelection()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Close Key Editor (or click the selected key again)")
                            }
                        }

                        // Key Label & Flags Row (config.txt zN, xN, ab N, dr N)
                        RowLayout {
                            id: inspectorLabelsRow
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("L1:")
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.TextField {
                                id: label1Input
                                Layout.preferredWidth: Math.round(72 * root.inspectorScale)
                                Layout.minimumWidth: 52
                                maximumLength: 7
                                placeholderText: qsTr("Line 1")
                                selectByMouse: true
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                            }

                            QQC2.Label {
                                text: qsTr("L2:")
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                color: Kirigami.Theme.disabledTextColor
                            }

                            QQC2.TextField {
                                id: label2Input
                                Layout.preferredWidth: Math.round(72 * root.inspectorScale)
                                Layout.minimumWidth: 52
                                maximumLength: 7
                                placeholderText: qsTr("Line 2")
                                selectByMouse: true
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                            }

                            QQC2.CheckBox {
                                id: allowAbortCheck
                                text: qsTr("Allow Abort")
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                onToggled: root.updateCompilerStatus()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Allow any key press to abort this script early (ab %1 / $_ALLOW_ABORT = 1)").arg(root.selectedSlotNumber)
                            }

                            QQC2.CheckBox {
                                id: disableRepeatCheck
                                text: qsTr("No Repeat")
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                onToggled: root.updateCompilerStatus()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Disable auto-repeat while key is held down (dr %1 / $_DONT_REPEAT = 1)").arg(root.selectedSlotNumber)
                            }

                            Item { Layout.fillWidth: true }

                            QQC2.Button {
                                icon.name: "color-picker"
                                text: qsTr("Key Color")
                                enabled: root.selectedSlotNumber >= 1 && root.selectedSlotNumber <= 20
                                implicitHeight: Math.round(24 * root.inspectorScale)
                                font.pixelSize: Math.round(10 * root.inspectorScale)
                                onClicked: selectedKeyColorDialog.open()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Change custom LED color (SWCOLOR) for Key %1").arg(root.selectedSlotNumber)
                            }

                            QQC2.Button {
                                icon.name: "color-picker"
                                text: qsTr("Default Color")
                                implicitHeight: Math.round(24 * root.inspectorScale)
                                font.pixelSize: Math.round(10 * root.inspectorScale)
                                onClicked: root.openProfileDefaultColorPicker(
                                    deviceController.activeProfile,
                                    deviceController.activeProfileName,
                                    deviceController.activeProfileBgColor
                                )
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Change Default Button Color (BG_COLOR) for Profile #%1 (%2)")
                                    .arg(deviceController.activeProfile)
                                    .arg(deviceController.activeProfileName)
                            }
                        }

                        // Script Mode Tabs (keyN.txt vs keyN-release.txt) + Web API Builder Toggle + Snippet Quick Insert Bar
                        RowLayout {
                            id: inspectorTabsRow
                            Layout.fillWidth: true
                            spacing: 4

                            QQC2.ButtonGroup { id: scriptPhaseGroup }

                            QQC2.ToolButton {
                                QQC2.ButtonGroup.group: scriptPhaseGroup
                                checkable: true
                                checked: !root.editingReleaseScript
                                text: qsTr("▾ On Press (key%1)").arg(root.selectedSlotNumber)
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                font.bold: !root.editingReleaseScript
                                implicitHeight: Math.round(26 * root.inspectorScale)
                                onClicked: {
                                    if (root.editingReleaseScript) {
                                        root.releaseScriptBuffer = scriptEditor.text
                                        root.editingReleaseScript = false
                                        root.loadingSlotData = true
                                        scriptEditor.text = root.pressScriptBuffer
                                        root.loadWebApiBuilderFromScript(scriptEditor.text)
                                        root.loadingSlotData = false
                                        root.updateCompilerStatus()
                                    }
                                }
                            }

                            QQC2.ToolButton {
                                QQC2.ButtonGroup.group: scriptPhaseGroup
                                checkable: true
                                checked: root.editingReleaseScript
                                text: root.releaseScriptBuffer.trim().length > 0
                                    ? qsTr("▴ On Release (key%1-rel ●)").arg(root.selectedSlotNumber)
                                    : qsTr("▴ On Release (key%1-rel)").arg(root.selectedSlotNumber)
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                font.bold: root.editingReleaseScript
                                implicitHeight: Math.round(26 * root.inspectorScale)
                                onClicked: {
                                    if (!root.editingReleaseScript) {
                                        root.pressScriptBuffer = scriptEditor.text
                                        root.editingReleaseScript = true
                                        root.loadingSlotData = true
                                        scriptEditor.text = root.releaseScriptBuffer
                                        root.loadWebApiBuilderFromScript(scriptEditor.text)
                                        root.loadingSlotData = false
                                        root.updateCompilerStatus()
                                    }
                                }
                            }

                            QQC2.Button {
                                id: webApiToggleBtn
                                checkable: true
                                checked: root.showWebApiBuilder
                                highlighted: root.webApiEnabled
                                text: root.webApiEnabled
                                    ? qsTr("🌐 Web API ●")
                                    : qsTr("🌐 Web API")
                                implicitHeight: Math.round(26 * root.inspectorScale)
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                font.bold: root.webApiEnabled
                                onClicked: {
                                    root.showWebApiBuilder = checked
                                    if (checked && !root.webApiEnabled && scriptEditor.text.trim().length === 0) {
                                        webApiEnabledCheck.checked = true
                                        if (webApiOledField.text.trim().length === 0) {
                                            const lbl = (label1Input.text.trim() + " " + label2Input.text.trim()).trim()
                                            webApiOledField.text = lbl.length > 0 ? ("API: " + lbl) : ("API Key " + root.selectedSlotNumber)
                                        }
                                        root.syncWebApiBuilderToScript()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Configure an HTTP / Webhook API request triggered by pressing this key (without typing into the active window)")
                            }

                            QQC2.Button {
                                id: cmdRefToggleBtn
                                checkable: true
                                checked: root.showCommandReference
                                text: qsTr("📖 Reference")
                                implicitHeight: Math.round(26 * root.inspectorScale)
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                font.bold: root.showCommandReference
                                onClicked: {
                                    root.showCommandReference = checked
                                    if (checked && commandReferenceListView.count === 0) {
                                        root.refreshCommandReferenceModel()
                                    }
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Toggle interactive duckyScript 3 Command Reference & Snippet Browser")
                            }

                            Item { Layout.fillWidth: true }

                            QQC2.Button {
                                text: "+DELAY"
                                implicitHeight: Math.round(24 * root.inspectorScale)
                                font.pixelSize: Math.round(10 * root.inspectorScale)
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "DELAY 100\n")
                            }

                            QQC2.Button {
                                text: "+STRING"
                                implicitHeight: Math.round(24 * root.inspectorScale)
                                font.pixelSize: Math.round(10 * root.inspectorScale)
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "STRING Hello World\n")
                            }

                            QQC2.Button {
                                text: "+OLED"
                                implicitHeight: Math.round(24 * root.inspectorScale)
                                font.pixelSize: Math.round(10 * root.inspectorScale)
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "OLED_CLEAR\nOLED_CPRINT Active\nOLED_UPDATE\n")
                            }
                        }

                        // Collapsible Web API Call Action Builder (Option 4 Hybrid PGV _GV31 + F13..F24 Trigger)
                        Rectangle {
                            id: webApiBuilderCard
                            visible: root.showWebApiBuilder
                            Layout.fillWidth: true
                            implicitHeight: visible ? (webApiCol.implicitHeight + 16) : 0
                            radius: Kirigami.Units.smallSpacing
                            color: Qt.tint(
                                Kirigami.Theme.alternateBackgroundColor,
                                Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.08)
                            )
                            border.color: root.webApiEnabled
                                ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.65)
                                : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                            border.width: 1

                            ColumnLayout {
                                id: webApiCol
                                anchors.fill: parent
                                anchors.margins: 8
                                spacing: 6

                                // Row 1: Enable Toggle, Trigger Mechanism, and Test Button
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.CheckBox {
                                        id: webApiEnabledCheck
                                        text: root.editingReleaseScript
                                            ? qsTr("Trigger Web API on Key Release")
                                            : qsTr("Trigger Web API on Key Press")
                                        font.bold: true
                                        font.pixelSize: Math.round(11 * root.inspectorScale)
                                        onToggled: {
                                            if (checked && webApiOledField.text.trim().length === 0) {
                                                const lbl = (label1Input.text.trim() + " " + label2Input.text.trim()).trim()
                                                webApiOledField.text = lbl.length > 0 ? ("API: " + lbl) : ("API Key " + root.selectedSlotNumber)
                                            }
                                            root.syncWebApiBuilderToScript()
                                        }
                                    }

                                    Item { Layout.fillWidth: true }

                                    QQC2.Label {
                                        text: qsTr("Trigger:")
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.ComboBox {
                                        id: webApiTriggerModeCombo
                                        model: [
                                            qsTr("Silent Hardware PGV (_GV31)"),
                                            qsTr("Sentinel Key (Ctrl+Alt+Shift+F13–F24)")
                                        ]
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        Layout.preferredWidth: Math.round(195 * root.inspectorScale)
                                        onActivated: root.syncWebApiBuilderToScript()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: currentIndex === 0
                                            ? qsTr("Recommended for duckyPad Pro: signals DuckyDeck Studio silently via _GV31 over USB HID without sending any keystrokes to the OS.")
                                            : qsTr("Fallback for duckyPad (2020) or custom hotkeys: sends Ctrl+Alt+Shift+F13..F24 and intercepts it via Linux evdev.")
                                    }

                                    QQC2.ComboBox {
                                        id: webApiFkeyCombo
                                        visible: webApiTriggerModeCombo.currentIndex === 1
                                        model: root.webApiFkeys
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        Layout.preferredWidth: Math.round(68 * root.inspectorScale)
                                        onActivated: root.syncWebApiBuilderToScript()
                                    }

                                    QQC2.Button {
                                        text: deviceController.webApiTestBusy
                                            ? qsTr("⏳ Calling...")
                                            : qsTr("⚡ Test API Call")
                                        enabled: !deviceController.webApiTestBusy && webApiUrlField.text.trim().length > 0
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        font.pixelSize: Math.round(11 * root.inspectorScale)
                                        font.bold: true
                                        onClicked: {
                                            deviceController.testWebApiCall(
                                                root.buildWebApiConfigJson(),
                                                root.selectedSlotNumber
                                            )
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Fire this HTTP request right now from DuckyDeck Studio and flash Key %1's RGB LED on response").arg(root.selectedSlotNumber)
                                    }
                                }

                                // Row 2: HTTP Method + URL
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.ComboBox {
                                        id: webApiMethodCombo
                                        model: root.webApiMethods
                                        font.pixelSize: Math.round(11 * root.inspectorScale)
                                        font.bold: true
                                        implicitHeight: Math.round(28 * root.inspectorScale)
                                        Layout.preferredWidth: Math.round(88 * root.inspectorScale)
                                        onActivated: root.syncWebApiBuilderToScript()
                                    }

                                    QQC2.TextField {
                                        id: webApiUrlField
                                        Layout.fillWidth: true
                                        implicitHeight: Math.round(28 * root.inspectorScale)
                                        font.family: "monospace"
                                        font.pixelSize: Math.round(11 * root.inspectorScale)
                                        selectByMouse: true
                                        placeholderText: qsTr("https://api.example.com/webhook  or  http://localhost:8123/api/services/light/toggle")
                                        onTextEdited: {
                                            if (!webApiEnabledCheck.checked && text.trim().length > 0) {
                                                webApiEnabledCheck.checked = true
                                            }
                                            root.syncWebApiBuilderToScript()
                                        }
                                    }
                                }

                                // Row 3: Headers & JSON Body
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 6

                                    QQC2.TextField {
                                        id: webApiHeadersField
                                        Layout.fillWidth: true
                                        Layout.preferredWidth: 220
                                        implicitHeight: Math.round(26 * root.inspectorScale)
                                        font.family: "monospace"
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        selectByMouse: true
                                        placeholderText: qsTr("Headers (e.g. Authorization: Bearer <token>; X-Api-Key: 123)")
                                        onTextEdited: root.syncWebApiBuilderToScript()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Optional HTTP headers separated by ';' or newlines. Tip: 'Bearer <token>' automatically sets Authorization.")
                                    }

                                    QQC2.TextField {
                                        id: webApiBodyField
                                        visible: webApiMethodCombo.currentIndex !== 1 // Hide on GET
                                        Layout.fillWidth: true
                                        Layout.preferredWidth: 260
                                        implicitHeight: Math.round(26 * root.inspectorScale)
                                        font.family: "monospace"
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        selectByMouse: true
                                        placeholderText: qsTr("Request Body / JSON (e.g. {\"entity_id\": \"light.desk\"})")
                                        onTextEdited: root.syncWebApiBuilderToScript()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Request payload sent with POST/PUT/PATCH/DELETE. JSON objects automatically set Content-Type: application/json.")
                                    }
                                }

                                // Row 4: OLED Banner & RGB LED Status Flash
                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 8

                                    QQC2.Label {
                                        text: qsTr("OLED Text:")
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    QQC2.TextField {
                                        id: webApiOledField
                                        Layout.preferredWidth: Math.round(160 * root.inspectorScale)
                                        implicitHeight: Math.round(24 * root.inspectorScale)
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        selectByMouse: true
                                        placeholderText: qsTr("Optional OLED message")
                                        onTextEdited: root.syncWebApiBuilderToScript()
                                    }

                                    QQC2.CheckBox {
                                        id: webApiLedCheck
                                        checked: true
                                        text: qsTr("RGB LED Status Flash (Green = 2xx, Red = Error)")
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        onToggled: root.syncWebApiBuilderToScript()
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Flashes Key %1's physical RGB LED cyan while calling, green on HTTP 2xx, or red on error, then restores key color").arg(root.selectedSlotNumber)
                                    }

                                    Item { Layout.fillWidth: true }
                                }

                                // Row 5: Live Test / Execution Status Pill
                                Rectangle {
                                    visible: deviceController.webApiLastResult.length > 0
                                    Layout.fillWidth: true
                                    implicitHeight: Math.round(24 * root.inspectorScale)
                                    radius: 4
                                    color: deviceController.webApiTestBusy
                                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                                        : (deviceController.webApiLastOk
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                                            : Qt.alpha(Kirigami.Theme.negativeTextColor, 0.16))
                                    border.color: deviceController.webApiTestBusy
                                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                                        : (deviceController.webApiLastOk
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.5)
                                            : Qt.alpha(Kirigami.Theme.negativeTextColor, 0.5))
                                    border.width: 1

                                    QQC2.Label {
                                        anchors.fill: parent
                                        anchors.leftMargin: 8
                                        anchors.rightMargin: 8
                                        verticalAlignment: Text.AlignVCenter
                                        elide: Text.ElideRight
                                        font.family: "monospace"
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        color: deviceController.webApiTestBusy
                                            ? Kirigami.Theme.highlightColor
                                            : (deviceController.webApiLastOk
                                                ? Kirigami.Theme.positiveTextColor
                                                : Kirigami.Theme.negativeTextColor)
                                        text: deviceController.webApiLastResult
                                    }
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
                                        font.pixelSize: Math.round(11 * root.inspectorScale)
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
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
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
                                            font.pixelSize: Math.round(10 * root.inspectorScale)
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
                                        font.pixelSize: Math.round(10 * root.inspectorScale)
                                        color: Kirigami.Theme.disabledTextColor
                                    }
                                }

                                // Scrollable Command Reference List
                                QQC2.ScrollView {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    clip: true

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
                                                        font.pixelSize: Math.round(9 * root.inspectorScale)
                                                        color: root.badgeColorForType(refRow.badge)
                                                    }
                                                }

                                                QQC2.Label {
                                                    text: refRow.syntax
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pixelSize: Math.round(11 * root.inspectorScale)
                                                    color: Kirigami.Theme.textColor
                                                    Layout.preferredWidth: Math.round(185 * root.inspectorScale)
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    text: refRow.summary
                                                    font.pixelSize: Math.round(10 * root.inspectorScale)
                                                    color: Kirigami.Theme.disabledTextColor
                                                    Layout.fillWidth: true
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    visible: refRowMouse.containsMouse
                                                    text: qsTr("+ Insert")
                                                    font.bold: true
                                                    font.pixelSize: Math.round(10 * root.inspectorScale)
                                                    color: Kirigami.Theme.highlightColor
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

                        // Code Editor Area with Synchronized Line Number Gutter & Live Autocomplete Popup
                        Rectangle {
                            id: editorContainer
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: Kirigami.Units.smallSpacing
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: root.compileOk
                                ? Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                : Kirigami.Theme.negativeTextColor
                            border.width: 1
                            clip: true

                            QQC2.ScrollView {
                                id: scriptScrollView
                                anchors.fill: parent
                                anchors.margins: 2
                                clip: true

                                QQC2.TextArea {
                                    id: scriptEditor
                                    font.family: "monospace"
                                    font.pixelSize: Math.round(13 * root.editorScale)
                                    leftPadding: lineGutter.width + 10
                                    topPadding: 6
                                    bottomPadding: 6
                                    rightPadding: 8
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
                                        x: scriptScrollView.contentItem ? scriptScrollView.contentItem.contentX : 0
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
                                                    font.pixelSize: scriptEditor.font.pixelSize
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
                                visible: root.autocompleteVisible && autocompleteListView.count > 0
                                z: 25
                                readonly property real scrollX: (scriptScrollView.contentItem && scriptScrollView.contentItem.contentX)
                                    ? scriptScrollView.contentItem.contentX
                                    : 0
                                readonly property real scrollY: (scriptScrollView.contentItem && scriptScrollView.contentItem.contentY)
                                    ? scriptScrollView.contentItem.contentY
                                    : 0
                                readonly property real rowH: Math.round(26 * root.inspectorScale)
                                readonly property real footerH: Math.round(20 * root.inspectorScale)
                                readonly property real popupH: Math.min(autocompleteListView.count, 7) * rowH + footerH + 8
                                readonly property real cursorBottomY: scriptEditor.cursorRectangle.y - scrollY + scriptEditor.cursorRectangle.height + 6
                                readonly property real cursorTopY: scriptEditor.cursorRectangle.y - scrollY - popupH - 4

                                x: Math.max(
                                    lineGutter.width + 6,
                                    Math.min(
                                        scriptEditor.cursorRectangle.x - scrollX,
                                        editorContainer.width - width - 6
                                    )
                                )
                                y: (cursorBottomY + popupH <= editorContainer.height - 4 || cursorTopY < 4)
                                    ? Math.max(4, Math.min(cursorBottomY, editorContainer.height - popupH - 4))
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
                                                        font.pixelSize: Math.round(8 * root.inspectorScale)
                                                        color: root.badgeColorForType(acDelegate.badge)
                                                    }
                                                }

                                                QQC2.Label {
                                                    text: acDelegate.syntax
                                                    font.family: "monospace"
                                                    font.bold: true
                                                    font.pixelSize: Math.round(11 * root.inspectorScale)
                                                    color: acDelegate.isSelected
                                                        ? Kirigami.Theme.highlightColor
                                                        : Kirigami.Theme.textColor
                                                    Layout.preferredWidth: Math.round(150 * root.inspectorScale)
                                                    elide: Text.ElideRight
                                                }

                                                QQC2.Label {
                                                    text: acDelegate.summary
                                                    font.pixelSize: Math.round(10 * root.inspectorScale)
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
                                            font.pixelSize: Math.round(9 * root.inspectorScale)
                                            color: Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideRight
                                            Layout.fillWidth: true
                                        }
                                    }
                                }
                            }
                        }

                        // Inspector Footer Bar with Live DSVM v2 .dsb Bytecode Status
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

                                QQC2.Label {
                                    anchors.fill: parent
                                    anchors.leftMargin: 8
                                    anchors.rightMargin: 8
                                    verticalAlignment: Text.AlignVCenter
                                    elide: Text.ElideRight
                                    font.family: "monospace"
                                    font.pixelSize: Math.round(11 * root.inspectorScale)
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
                            }

                            QQC2.Button {
                                text: qsTr("↺ Revert")
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                onClicked: root.loadSlotFromController(
                                    root.selectedSlotNumber,
                                    root.selectedSlotTitle,
                                    root.selectedSlotColor
                                )
                            }

                            QQC2.Button {
                                text: deviceController.sdMountPath.length > 0
                                    ? qsTr("💾 Save Key %1 to SD").arg(root.selectedSlotNumber)
                                    : qsTr("💾 Save Key %1").arg(root.selectedSlotNumber)
                                implicitHeight: Math.round(28 * root.inspectorScale)
                                font.pixelSize: Math.round(11 * root.inspectorScale)
                                font.bold: true
                                highlighted: true
                                onClicked: root.saveCurrentSlotToController()
                            }
                        }
                    }
                }
            }
        }
    }

    // Bottom Status Bar (Connection Status, Firmware Version, Serial Number, SD Card Path & Live Status)
    footer: Rectangle {
        id: bottomStatusBar
        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        width: parent.width
        implicitHeight: Math.round(40 * root.statusBarScale)
        color: Kirigami.Theme.alternateBackgroundColor

        Rectangle {
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            height: 1
            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        }

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: Kirigami.Units.largeSpacing
            anchors.rightMargin: Kirigami.Units.largeSpacing
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
                        font.pixelSize: Math.round(12 * root.statusBarScale)
                        color: Kirigami.Theme.textColor
                    }
                }
            }

            // Firmware Version & Serial Number Badge (Click to check for firmware updates)
            Rectangle {
                id: fwBadgeRect
                radius: 4
                color: deviceController.firmwareUpdateAvailable
                    ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.18)
                    : Kirigami.Theme.backgroundColor
                border.color: deviceController.firmwareUpdateAvailable
                    ? Qt.alpha(Kirigami.Theme.neutralTextColor, 0.55)
                    : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                implicitWidth: fwLabel.implicitWidth + 16
                implicitHeight: Math.round(26 * root.statusBarScale)

                QQC2.Label {
                    id: fwLabel
                    anchors.centerIn: parent
                    text: {
                        const base = (deviceController.serialNumber && deviceController.serialNumber !== "-")
                            ? qsTr("FW: %1  •  SN: %2")
                                .arg(deviceController.firmwareVersion)
                                .arg(deviceController.serialNumber)
                            : qsTr("Firmware: %1").arg(deviceController.firmwareVersion)
                        return deviceController.firmwareUpdateAvailable
                            ? qsTr("%1  (⬆ v%2)").arg(base).arg(deviceController.latestFirmwareVersion)
                            : base
                    }
                    font.family: "monospace"
                    font.pixelSize: Math.round(12 * root.statusBarScale)
                    color: Kirigami.Theme.textColor
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
                QQC2.ToolTip.text: deviceController.firmwareUpdateAvailable
                    ? qsTr("Firmware update v%1 is available! Click for details.").arg(deviceController.latestFirmwareVersion)
                    : qsTr("Click to check for official duckyPad firmware updates")
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
                        font.pixelSize: Math.round(12 * root.statusBarScale)
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

            // Mounted SD Card Badge (when loaded)
            Rectangle {
                visible: deviceController.sdMountPath.length > 0
                radius: 4
                color: Kirigami.Theme.backgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1
                implicitWidth: Math.min(sdPathLabel.implicitWidth + 16, 280)
                implicitHeight: Math.round(26 * root.statusBarScale)

                QQC2.Label {
                    id: sdPathLabel
                    anchors.fill: parent
                    anchors.leftMargin: 8
                    anchors.rightMargin: 8
                    verticalAlignment: Text.AlignVCenter
                    text: qsTr("SD: %1").arg(deviceController.sdMountPath)
                    font.family: "monospace"
                    font.pixelSize: Math.round(12 * root.statusBarScale)
                    elide: Text.ElideMiddle
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            Rectangle {
                width: 1
                height: Math.round(18 * root.statusBarScale)
                color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            }

            // Live Status Message
            QQC2.Label {
                text: deviceController.statusMessage
                font.pixelSize: Math.round(13 * root.statusBarScale)
                color: Kirigami.Theme.textColor
                opacity: 0.92
                elide: Text.ElideRight
                Layout.fillWidth: true
            }
        }
    }
}
