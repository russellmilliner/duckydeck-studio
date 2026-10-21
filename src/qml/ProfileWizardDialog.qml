import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import org.kde.kirigami as Kirigami

Window {
    id: wizardDialog
    title: qsTr("New Profile Wizard — DuckyDeck Studio")
    width: 980
    height: 700
    minimumWidth: 860
    minimumHeight: 600
    visible: false
    transientParent: root
    modality: Qt.WindowModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMinMaxButtonsHint | ((root && root.visible) ? Qt.WindowStaysOnTopHint : 0)

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor

    property var root: null
    property var controller: null
    readonly property real fontScale: (controller && controller.uiTextScale > 0 ? controller.uiTextScale : 100) / 100.0
    readonly property bool isLightTheme: (0.2126 * Kirigami.Theme.backgroundColor.r + 0.7152 * Kirigami.Theme.backgroundColor.g + 0.0722 * Kirigami.Theme.backgroundColor.b) > 0.45
    readonly property color frameBorderColor: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

    property int currentStep: 0

    // Step 0: Basics
    property string profileName: ""
    property int targetSlotNumber: 1
    property string selectedTemplateId: "builtin_blank"

    // Step 1: Target App & Auto-Switch
    property string targetAppClass: ""
    property string targetAppTitle: ""
    property string targetAppName: ""
    property string lastSearchedApp: ""
    property bool autoSwitchEnabled: true
    property bool allowGoogleSearch: controller ? controller.googleSearchShortcutsPermitted : false

    // Step 2: Appearance & Orientation
    property color profileBgColor: "#00aaff"
    property color keydownColor: "#ffffff"
    property bool dimUnusedKeys: true
    property int oledRotation: 0

    // Step 3: Assigned Shortcuts Map: slotNumber (1..20) -> { slot, label1, label2, script, action, shortcut, category }
    property var keyAssignments: ({})
    property int selectedMatrixSlot: 1
    property string shortcutSearchFilter: ""

    // Drag & Drop State (both matrix-to-matrix swap and shortcut-menu-to-matrix assign)
    property int draggingMatrixSlot: -1
    property var draggingShortcutItem: null
    property real dragOverlayX: 0
    property real dragOverlayY: 0
    property int dropTargetSlot: -1

    // Discovered shortcuts parsed from controller
    property var discoveredShortcutsList: []

    onCurrentStepChanged: {
        const eff = resolveEffectiveAppName()
        if ((currentStep === 1 || currentStep === 3) && eff.length > 0) {
            if (!targetAppName) {
                targetAppName = eff
            }
            if (discoveredShortcutsList.length === 0 || lastSearchedApp.toLowerCase() !== eff.toLowerCase()) {
                triggerShortcutsSearch(eff)
            }
        }
    }

    function resetWizard() {
        currentStep = 0
        profileName = ""
        targetAppClass = ""
        targetAppTitle = ""
        targetAppName = ""
        lastSearchedApp = ""
        autoSwitchEnabled = true
        allowGoogleSearch = controller ? controller.googleSearchShortcutsPermitted : false
        profileBgColor = "#00aaff"
        keydownColor = "#ffffff"
        dimUnusedKeys = true
        oledRotation = 0
        keyAssignments = {}
        selectedMatrixSlot = 1
        shortcutSearchFilter = ""
        draggingMatrixSlot = -1
        draggingShortcutItem = null
        dropTargetSlot = -1
        discoveredShortcutsList = []

        // Pick next available profile number
        if (controller && controller.profilesJson) {
            try {
                const list = JSON.parse(controller.profilesJson)
                const used = new Set(list.map(p => p.number))
                let nextNum = 1
                while (used.has(nextNum) && nextNum <= 64) {
                    nextNum++
                }
                targetSlotNumber = nextNum
            } catch (e) {
                targetSlotNumber = 1
            }
        }
    }

    function openWizard() {
        resetWizard()
        wizardDialog.show()
        wizardDialog.raise()
        wizardDialog.requestActivate()
    }

    // Refresh shortcuts list when controller property updates
    Connections {
        target: controller
        function onDiscoveredShortcutsJsonChanged() {
            if (!controller) return
            try {
                const raw = controller.discoveredShortcutsJson
                const arr = JSON.parse(raw || "[]")
                wizardDialog.discoveredShortcutsList = Array.isArray(arr) ? arr : []
            } catch (e) {
                wizardDialog.discoveredShortcutsList = []
            }
        }
        function onLastPickedWindowClassChanged() {
            if (wizardDialog.visible && controller && controller.lastPickedWindowClass) {
                wizardDialog.targetAppClass = controller.lastPickedWindowClass
                wizardDialog.targetAppTitle = controller.lastPickedWindowTitle || ""
                const resolved = wizardDialog.resolveAppNameFromWindow(
                    controller.lastPickedWindowClass,
                    controller.lastPickedWindowTitle || "",
                    wizardDialog.profileName
                )
                if (!wizardDialog.profileName || wizardDialog.profileName === "New Profile") {
                    wizardDialog.profileName = resolved
                }
                wizardDialog.targetAppName = resolved
                triggerShortcutsSearch(resolved)
            }
        }
    }

    function cleanAppNameFromClass(wmClass) {
        if (!wmClass) return ""
        const trimmed = wmClass.trim()
        if (!trimmed) return ""
        const lower = trimmed.toLowerCase()
        if (lower.includes("minecraft") || lower.includes("prismlauncher") || lower.includes("lunarclient") || lower.includes("badlion") || lower.includes("modrinth")) {
            return "Minecraft"
        }
        const parts = trimmed.split(".")
        const raw = parts[parts.length - 1] || trimmed
        return raw.charAt(0).toUpperCase() + raw.slice(1)
    }

    function resolveAppNameFromWindow(wmClass, wmTitle, profName) {
        const combined = ((wmClass || "") + " " + (wmTitle || "") + " " + (profName || "")).toLowerCase()
        if (combined.includes("minecraft") || combined.includes("prismlauncher") || combined.includes("lunarclient") || combined.includes("badlion")) {
            return "Minecraft"
        }
        const cleanedClass = cleanAppNameFromClass(wmClass)
        const lowerClass = cleanedClass.toLowerCase()
        if ((!cleanedClass || lowerClass === "java" || lowerClass === "javaw" || lowerClass === "glfw" || lowerClass === "steam") && wmTitle) {
            return wmTitle.trim()
        }
        return cleanedClass || (profName ? profName.trim() : "App")
    }

    function resolveEffectiveAppName() {
        if (targetAppName && targetAppName.trim().length > 0) {
            return targetAppName.trim()
        }
        const fromWin = resolveAppNameFromWindow(targetAppClass, targetAppTitle, profileName)
        if (fromWin && fromWin !== "App") {
            return fromWin
        }
        if (profileName && profileName.trim().length > 0) {
            return profileName.trim()
        }
        return ""
    }

    function triggerShortcutsSearch(appName) {
        const query = (appName && appName.trim().length > 0) ? appName.trim() : resolveEffectiveAppName()
        if (!controller || !query) return
        lastSearchedApp = query
        if (targetAppName !== query) {
            targetAppName = query
        }
        controller.searchAppShortcuts(query, allowGoogleSearch)
    }

    function categoryLedColor(category) {
        const cat = String(category || "").toLowerCase()
        if (cat.includes("debug")) return Qt.rgba(0.96, 0.62, 0.04, 1.0)      // Amber
        if (cat.includes("combat") || cat.includes("action")) return Qt.rgba(0.94, 0.27, 0.27, 1.0) // Red
        if (cat.includes("move")) return Qt.rgba(0.13, 0.77, 0.37, 1.0)       // Green
        if (cat.includes("hotbar") || cat.includes("creative")) return Qt.rgba(0.66, 0.33, 0.97, 1.0) // Purple
        if (cat.includes("camera") || cat.includes("view") || cat.includes("hud")) return Qt.rgba(0.02, 0.71, 0.83, 1.0) // Cyan
        if (cat.includes("chat") || cat.includes("multi")) return Qt.rgba(0.93, 0.28, 0.60, 1.0) // Pink
        return wizardDialog.profileBgColor
    }

    function hitTestMatrixSlot(sourceItem, mx, my) {
        if (!sourceItem || !wizardSwitchMatrix) return -1
        const pt = sourceItem.mapToItem(wizardSwitchMatrix, mx, my)
        const stride = 72
        if (pt.x >= 0 && pt.x < wizardSwitchMatrix.width && pt.y >= 0 && pt.y < wizardSwitchMatrix.height) {
            const col = Math.min(3, Math.max(0, Math.floor(pt.x / stride)))
            const row = Math.min(4, Math.max(0, Math.floor(pt.y / stride)))
            return row * 4 + col + 1
        }
        return -1
    }

    function assignShortcutToSlot(slotNum, shortcutItem) {
        if (slotNum < 1 || slotNum > 20 || !shortcutItem) return
        const newMap = Object.assign({}, keyAssignments)
        newMap[String(slotNum)] = {
            slot: slotNum,
            label1: shortcutItem.label_l1 || "",
            label2: shortcutItem.label_l2 || "",
            script: shortcutItem.duckyscript || "",
            action: shortcutItem.action || "",
            shortcut: shortcutItem.shortcut || "",
            category: shortcutItem.category || ""
        }
        keyAssignments = newMap
        selectedMatrixSlot = slotNum
    }

    function swapMatrixSlots(slotA, slotB) {
        if (slotA < 1 || slotA > 20 || slotB < 1 || slotB > 20 || slotA === slotB) return
        const newMap = Object.assign({}, keyAssignments)
        const a = newMap[String(slotA)] ? Object.assign({}, newMap[String(slotA)], { slot: slotB }) : null
        const b = newMap[String(slotB)] ? Object.assign({}, newMap[String(slotB)], { slot: slotA }) : null
        if (a) newMap[String(slotB)] = a; else delete newMap[String(slotB)]
        if (b) newMap[String(slotA)] = b; else delete newMap[String(slotA)]
        keyAssignments = newMap
        selectedMatrixSlot = slotB
    }

    function clearSlotAssignment(slotNum) {
        const newMap = Object.assign({}, keyAssignments)
        delete newMap[String(slotNum)]
        keyAssignments = newMap
    }

    function clearAllAssignments() {
        keyAssignments = ({})
    }

    function autoFillEmptySlots() {
        if (!discoveredShortcutsList || discoveredShortcutsList.length === 0) return
        const newMap = Object.assign({}, keyAssignments)
        let scIdx = 0
        for (let slot = 1; slot <= 20; slot++) {
            if (!newMap[String(slot)]) {
                while (scIdx < discoveredShortcutsList.length) {
                    const item = discoveredShortcutsList[scIdx++]
                    if (item && item.action) {
                        newMap[String(slot)] = {
                            slot: slot,
                            label1: item.label_l1 || "",
                            label2: item.label_l2 || "",
                            script: item.duckyscript || "",
                            action: item.action || "",
                            shortcut: item.shortcut || "",
                            category: item.category || ""
                        }
                        break
                    }
                }
            }
        }
        keyAssignments = newMap
    }

    function finishAndCreateProfile() {
        if (!controller) return

        const finalName = profileName.trim() || targetAppName.trim() || ("Profile " + targetSlotNumber)
        const slotsArray = []
        for (const slotKey in keyAssignments) {
            const s = keyAssignments[slotKey]
            if (s) {
                slotsArray.push({
                    slot: s.slot,
                    label1: s.label1,
                    label2: s.label2,
                    script: s.script,
                    action: s.action,
                    shortcut: s.shortcut
                })
            }
        }

        function colorToRgb(c) {
            return [
                Math.round(c.r * 255),
                Math.round(c.g * 255),
                Math.round(c.b * 255)
            ]
        }

        const payload = {
            name: finalName,
            slot_number: targetSlotNumber,
            template_id: selectedTemplateId,
            bg_color: colorToRgb(profileBgColor),
            keydown_color: colorToRgb(keydownColor),
            dim_unused_keys: dimUnusedKeys,
            auto_switch_class: targetAppClass,
            auto_switch_title: targetAppTitle,
            auto_switch_enabled: autoSwitchEnabled && !!targetAppClass,
            slots: slotsArray
        }

        const jsonStr = JSON.stringify(payload)
        const ok = controller.createWizardProfile(jsonStr)
        if (ok) {
            wizardDialog.close()
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 12

        // Top Stepper Header
        Rectangle {
            Layout.fillWidth: true
            height: 50
            radius: 8
            color: Qt.alpha(Kirigami.Theme.alternateBackgroundColor, 0.75)
            border.width: 1
            border.color: wizardDialog.frameBorderColor

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 20
                anchors.rightMargin: 20
                spacing: 10

                Repeater {
                    model: [
                        { step: 0, title: qsTr("1. Basics") },
                        { step: 1, title: qsTr("2. Target App") },
                        { step: 2, title: qsTr("3. Appearance") },
                        { step: 3, title: qsTr("4. Shortcuts Layout") }
                    ]

                    delegate: Item {
                        Layout.fillWidth: true
                        Layout.fillHeight: true

                        RowLayout {
                            anchors.centerIn: parent
                            spacing: 8

                            Rectangle {
                                width: 24
                                height: 24
                                radius: 12
                                color: wizardDialog.currentStep === modelData.step
                                    ? Kirigami.Theme.highlightColor
                                    : (wizardDialog.currentStep > modelData.step
                                        ? Kirigami.Theme.positiveTextColor
                                        : Qt.alpha(Kirigami.Theme.textColor, 0.2))

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    text: wizardDialog.currentStep > modelData.step ? "✓" : String(modelData.step + 1)
                                    font.bold: true
                                    font.pointSize: 9.5 * fontScale
                                    color: (wizardDialog.currentStep >= modelData.step) ? "#ffffff" : Kirigami.Theme.textColor
                                }
                            }

                            QQC2.Label {
                                text: modelData.title
                                font.bold: wizardDialog.currentStep === modelData.step
                                font.pointSize: 10.5 * fontScale
                                color: wizardDialog.currentStep === modelData.step
                                    ? Kirigami.Theme.highlightColor
                                    : Kirigami.Theme.textColor
                                opacity: wizardDialog.currentStep >= modelData.step ? 1.0 : 0.65
                            }
                        }

                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                wizardDialog.currentStep = modelData.step
                            }
                        }
                    }
                }
            }
        }

        // Main Wizard Content Area (Stacked by Step)
        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: wizardDialog.currentStep

            // ==============================================================
            // Step 0: Basics
            // ==============================================================
            QQC2.ScrollView {
                clip: true
                ColumnLayout {
                    width: parent.width - 20
                    spacing: 16

                    Kirigami.Heading {
                        level: 2
                        text: qsTr("Profile Basics")
                    }

                    QQC2.Label {
                        text: qsTr("Choose a clear name and starting profile slot on your duckyPad.")
                        font.pointSize: 10 * fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        height: 1
                        color: wizardDialog.frameBorderColor
                    }

                    // Profile Name
                    ColumnLayout {
                        spacing: 4
                        Layout.fillWidth: true

                        QQC2.Label {
                            text: qsTr("Profile / Application Name:")
                            font.bold: true
                            font.pointSize: 10 * fontScale
                        }

                        QQC2.TextField {
                            id: nameField
                            Layout.fillWidth: true
                            implicitHeight: Math.round(32 * fontScale)
                            font.pointSize: 10.5 * fontScale
                            placeholderText: qsTr("e.g., Minecraft, Firefox, VS Code, Blender, OBS Studio")
                            text: wizardDialog.profileName
                            onTextChanged: {
                                wizardDialog.profileName = text
                                if (!wizardDialog.targetAppClass) {
                                    wizardDialog.targetAppName = text.trim()
                                }
                            }
                        }
                    }

                    // Slot Number
                    RowLayout {
                        spacing: 16
                        Layout.fillWidth: true

                        ColumnLayout {
                            spacing: 4
                            Layout.preferredWidth: 200

                            QQC2.Label {
                                text: qsTr("duckyPad Slot # (1..64):")
                                font.bold: true
                                font.pointSize: 10 * fontScale
                            }

                            QQC2.SpinBox {
                                id: slotSpin
                                implicitHeight: Math.round(32 * fontScale)
                                font.pointSize: 10 * fontScale
                                from: 1
                                to: 64
                                value: wizardDialog.targetSlotNumber
                                onValueChanged: wizardDialog.targetSlotNumber = value
                            }
                        }

                        ColumnLayout {
                            spacing: 4
                            Layout.fillWidth: true

                            QQC2.Label {
                                text: qsTr("Base Template:")
                                font.bold: true
                                font.pointSize: 10 * fontScale
                            }

                            QQC2.ComboBox {
                                id: templateCombo
                                Layout.fillWidth: true
                                implicitHeight: Math.round(32 * fontScale)
                                font.pointSize: 10 * fontScale
                                textRole: "text"
                                valueRole: "id"
                                model: [
                                    { id: "builtin_blank", text: qsTr("Blank Profile (Fresh Start)") },
                                    { id: "builtin_browser", text: qsTr("Web Browser (Chrome, Firefox, Edge)") },
                                    { id: "builtin_editor", text: qsTr("Code & Text Editor (VS Code, Kate)") },
                                    { id: "builtin_media", text: qsTr("Media & Audio Controller") },
                                    { id: "builtin_blender", text: qsTr("3D Modeling & CAD (Blender, FreeCAD)") },
                                    { id: "builtin_numpad", text: qsTr("Numeric Keypad & Calculator") }
                                ]
                                currentIndex: 0
                                onActivated: wizardDialog.selectedTemplateId = currentValue
                            }
                        }
                    }
                }
            }

            // ==============================================================
            // Step 1: Target App & Auto-Switch
            // ==============================================================
            QQC2.ScrollView {
                clip: true
                ColumnLayout {
                    width: parent.width - 20
                    spacing: 16

                    Kirigami.Heading {
                        level: 2
                        text: qsTr("Target Application & Auto-Switch")
                    }

                    QQC2.Label {
                        text: qsTr("Bind this profile to a desktop window so duckyPad activates automatically when you switch tasks.")
                        font.pointSize: 10 * fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    // Window Picker Card
                    Rectangle {
                        Layout.fillWidth: true
                        radius: 8
                        color: Kirigami.Theme.alternateBackgroundColor
                        border.width: 1
                        border.color: wizardDialog.frameBorderColor
                        implicitHeight: pickerCol.implicitHeight + 24

                        ColumnLayout {
                            id: pickerCol
                            anchors.fill: parent
                            anchors.margins: 14
                            spacing: 12

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 12

                                QQC2.Button {
                                    text: controller && controller.windowPickerArmed ? qsTr("🎯 Click Target Window...") : qsTr("🎯 Pick Active Window")
                                    implicitHeight: Math.round(30 * fontScale)
                                    font.pointSize: 10 * fontScale
                                    highlighted: controller && controller.windowPickerArmed
                                    icon.name: "edit-find"
                                    onClicked: {
                                        if (controller) {
                                            controller.armWindowPicker(false)
                                        }
                                    }
                                }

                                QQC2.Label {
                                    text: qsTr("Click button, then click on the target app's window anywhere on screen.")
                                    wrapMode: Text.WordWrap
                                    Layout.fillWidth: true
                                    font.pointSize: 9.5 * fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            // Detected class/title
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10

                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 2
                                    QQC2.Label {
                                        text: qsTr("Window Class / WM_CLASS:")
                                        font.bold: true
                                        font.pointSize: 9.5 * fontScale
                                    }
                                    QQC2.TextField {
                                        Layout.fillWidth: true
                                        implicitHeight: Math.round(30 * fontScale)
                                        font.pointSize: 10 * fontScale
                                        text: wizardDialog.targetAppClass
                                        placeholderText: qsTr("e.g. Minecraft, org.kde.dolphin, firefox, code")
                                        onTextChanged: {
                                            wizardDialog.targetAppClass = text
                                            wizardDialog.targetAppName = wizardDialog.resolveAppNameFromWindow(
                                                text,
                                                wizardDialog.targetAppTitle,
                                                wizardDialog.profileName
                                            )
                                        }
                                    }
                                }

                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 2
                                    QQC2.Label {
                                        text: qsTr("Window Title Filter (optional):")
                                        font.bold: true
                                        font.pointSize: 9.5 * fontScale
                                    }
                                    QQC2.TextField {
                                        Layout.fillWidth: true
                                        implicitHeight: Math.round(30 * fontScale)
                                        font.pointSize: 10 * fontScale
                                        text: wizardDialog.targetAppTitle
                                        placeholderText: qsTr("Optional title regex or substring")
                                        onTextChanged: {
                                            wizardDialog.targetAppTitle = text
                                            wizardDialog.targetAppName = wizardDialog.resolveAppNameFromWindow(
                                                wizardDialog.targetAppClass,
                                                text,
                                                wizardDialog.profileName
                                            )
                                        }
                                    }
                                }
                            }

                            QQC2.CheckBox {
                                id: autoSwitchToggle
                                text: qsTr("Enable automatic profile switching when this application is focused")
                                font.pointSize: 10 * fontScale
                                checked: wizardDialog.autoSwitchEnabled
                                onToggled: wizardDialog.autoSwitchEnabled = checked
                            }
                        }
                    }

                    // Online Search Permission & Discovery Section
                    Rectangle {
                        Layout.fillWidth: true
                        radius: 8
                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.08)
                        border.width: 1
                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.35)
                        implicitHeight: googleCol.implicitHeight + 24

                        ColumnLayout {
                            id: googleCol
                            anchors.fill: parent
                            anchors.margins: 14
                            spacing: 10

                            RowLayout {
                                spacing: 8
                                QQC2.Label {
                                    text: "🔍"
                                    font.pixelSize: 18
                                }
                                Kirigami.Heading {
                                    level: 4
                                    text: qsTr("Keyboard Shortcut Discovery")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("DuckyDeck Studio searches built-in game/app catalogs, local KDE shortcuts, and online cheatsheets for %1.").arg(wizardDialog.resolveEffectiveAppName() || qsTr("the selected application"))
                                wrapMode: Text.WordWrap
                                Layout.fillWidth: true
                                font.pointSize: 10 * fontScale
                                color: Kirigami.Theme.textColor
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8

                                QQC2.Label {
                                    text: qsTr("Search Application / Game:")
                                    font.bold: true
                                    font.pointSize: 9.5 * fontScale
                                }

                                QQC2.TextField {
                                    Layout.fillWidth: true
                                    implicitHeight: Math.round(30 * fontScale)
                                    font.pointSize: 10 * fontScale
                                    placeholderText: qsTr("e.g. Minecraft, Blender, VS Code, Firefox, OBS...")
                                    text: wizardDialog.targetAppName
                                    onTextChanged: wizardDialog.targetAppName = text
                                    onAccepted: wizardDialog.triggerShortcutsSearch(wizardDialog.targetAppName)
                                }
                            }

                            QQC2.CheckBox {
                                id: googlePermCheck
                                text: qsTr("Allow online web search for shortcut cheatsheets (privacy permission)")
                                font.pointSize: 10 * fontScale
                                checked: wizardDialog.allowGoogleSearch
                                onToggled: {
                                    wizardDialog.allowGoogleSearch = checked
                                    if (controller) {
                                        controller.setGoogleSearchPermission(checked)
                                    }
                                    if (wizardDialog.resolveEffectiveAppName().length > 0) {
                                        wizardDialog.triggerShortcutsSearch(wizardDialog.resolveEffectiveAppName())
                                    }
                                }
                            }

                            RowLayout {
                                spacing: 10
                                QQC2.Button {
                                    text: controller && controller.searchShortcutsBusy
                                        ? qsTr("Searching Shortcuts...")
                                        : qsTr("🔍 Find Shortcuts for %1").arg(wizardDialog.resolveEffectiveAppName() || "App")
                                    implicitHeight: Math.round(30 * fontScale)
                                    font.pointSize: 10 * fontScale
                                    enabled: !!wizardDialog.resolveEffectiveAppName() && !(controller && controller.searchShortcutsBusy)
                                    highlighted: true
                                    onClicked: {
                                        wizardDialog.triggerShortcutsSearch(wizardDialog.resolveEffectiveAppName())
                                    }
                                }

                                QQC2.BusyIndicator {
                                    running: controller && controller.searchShortcutsBusy
                                    visible: running
                                    Layout.preferredWidth: 22
                                    Layout.preferredHeight: 22
                                }

                                QQC2.Label {
                                    text: wizardDialog.discoveredShortcutsList.length > 0
                                        ? qsTr("✓ Found %1 shortcuts ready to drag and drop!").arg(wizardDialog.discoveredShortcutsList.length)
                                        : qsTr("Shortcuts will be shown in Step 4.")
                                    font.pointSize: 10 * fontScale
                                    color: wizardDialog.discoveredShortcutsList.length > 0 ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.disabledTextColor
                                    font.bold: wizardDialog.discoveredShortcutsList.length > 0
                                }
                            }
                        }
                    }
                }
            }

            // ==============================================================
            // Step 2: Appearance & Orientation
            // ==============================================================
            QQC2.ScrollView {
                clip: true
                ColumnLayout {
                    width: parent.width - 20
                    spacing: 16

                    Kirigami.Heading {
                        level: 2
                        text: qsTr("Appearance & Hardware Orientation")
                    }

                    QQC2.Label {
                        text: qsTr("Customize backlight colors and OLED screen orientation for your duckyPad.")
                        font.pointSize: 10 * fontScale
                        color: Kirigami.Theme.disabledTextColor
                    }

                    // Background color palette
                    ColumnLayout {
                        spacing: 8
                        Layout.fillWidth: true

                        QQC2.Label {
                            text: qsTr("Profile Backlight Color:")
                            font.bold: true
                            font.pointSize: 10 * fontScale
                        }

                        RowLayout {
                            spacing: 8
                            Repeater {
                                model: ["#00aaff", "#a855f7", "#22c55e", "#ef4444", "#f97316", "#eab308", "#ec4899", "#14b8a6", "#ffffff"]
                                delegate: Rectangle {
                                    width: 32
                                    height: 32
                                    radius: 6
                                    color: modelData
                                    border.width: wizardDialog.profileBgColor === modelData ? 3 : 1
                                    border.color: wizardDialog.profileBgColor === modelData ? Kirigami.Theme.highlightColor : Qt.alpha(Kirigami.Theme.textColor, 0.3)
                                    MouseArea {
                                        anchors.fill: parent
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: wizardDialog.profileBgColor = modelData
                                    }
                                }
                            }

                            Rectangle {
                                width: 32
                                height: 32
                                radius: 6
                                color: wizardDialog.profileBgColor
                                border.width: 2
                                border.color: Kirigami.Theme.textColor
                            }
                        }
                    }

                    // Keydown color palette
                    ColumnLayout {
                        spacing: 8
                        Layout.fillWidth: true

                        QQC2.Label {
                            text: qsTr("Key Press Flash Color (KeyDown):")
                            font.bold: true
                            font.pointSize: 10 * fontScale
                        }

                        RowLayout {
                            spacing: 8
                            Repeater {
                                model: ["#ffffff", "#ff0055", "#00f0ff", "#ffdd00", "#00ff66", "#aa00ff"]
                                delegate: Rectangle {
                                    width: 32
                                    height: 32
                                    radius: 6
                                    color: modelData
                                    border.width: wizardDialog.keydownColor === modelData ? 3 : 1
                                    border.color: wizardDialog.keydownColor === modelData ? Kirigami.Theme.highlightColor : Qt.alpha(Kirigami.Theme.textColor, 0.3)
                                    MouseArea {
                                        anchors.fill: parent
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: wizardDialog.keydownColor = modelData
                                    }
                                }
                            }
                        }
                    }

                    QQC2.CheckBox {
                        text: qsTr("Dim unmapped/unused keys to reduce visual clutter")
                        font.pointSize: 10 * fontScale
                        checked: wizardDialog.dimUnusedKeys
                        onToggled: wizardDialog.dimUnusedKeys = checked
                    }

                    // OLED Orientation
                    ColumnLayout {
                        spacing: 6
                        Layout.fillWidth: true

                        QQC2.Label {
                            text: qsTr("duckyPad Hardware Orientation:")
                            font.bold: true
                            font.pointSize: 10 * fontScale
                        }

                        RowLayout {
                            spacing: 12
                            QQC2.RadioButton {
                                text: qsTr("0° (Normal Portrait)")
                                font.pointSize: 10 * fontScale
                                checked: wizardDialog.oledRotation === 0
                                onToggled: if (checked) wizardDialog.oledRotation = 0
                            }
                            QQC2.RadioButton {
                                text: qsTr("90° (Landscape Right)")
                                font.pointSize: 10 * fontScale
                                checked: wizardDialog.oledRotation === 90
                                onToggled: if (checked) wizardDialog.oledRotation = 90
                            }
                            QQC2.RadioButton {
                                text: qsTr("180° (Inverted Portrait)")
                                font.pointSize: 10 * fontScale
                                checked: wizardDialog.oledRotation === 180
                                onToggled: if (checked) wizardDialog.oledRotation = 180
                            }
                            QQC2.RadioButton {
                                text: qsTr("270° (Landscape Left)")
                                font.pointSize: 10 * fontScale
                                checked: wizardDialog.oledRotation === 270
                                onToggled: if (checked) wizardDialog.oledRotation = 270
                            }
                        }
                    }
                }
            }

            // ==============================================================
            // Step 3: Hotkeys & Shortcuts Layout (Main Window Look & Feel + Drag & Drop)
            // ==============================================================
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true

                RowLayout {
                    anchors.fill: parent
                    spacing: 14

                    // Left Column: Authentic 4x5 (20-key) duckyPad Pro Key Matrix matching HardwareCanvas.qml
                    ColumnLayout {
                        Layout.preferredWidth: 340
                        Layout.minimumWidth: 315
                        Layout.fillHeight: true
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("duckyPad Key Matrix (1–20)")
                                font.bold: true
                                font.pointSize: 10.5 * fontScale
                                color: Kirigami.Theme.textColor
                            }

                            Item { Layout.fillWidth: true }

                            QQC2.Button {
                                text: qsTr("Auto-fill")
                                icon.name: "edit-paste"
                                implicitHeight: Math.round(28 * fontScale)
                                font.pointSize: 9.5 * fontScale
                                enabled: wizardDialog.discoveredShortcutsList.length > 0
                                onClicked: wizardDialog.autoFillEmptySlots()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Automatically fill empty keys with top discovered shortcuts")
                            }

                            QQC2.Button {
                                text: qsTr("Clear")
                                icon.name: "edit-clear"
                                implicitHeight: Math.round(28 * fontScale)
                                font.pointSize: 9.5 * fontScale
                                enabled: Object.keys(wizardDialog.keyAssignments).length > 0
                                onClicked: wizardDialog.clearAllAssignments()
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Clear all assigned keys (or right-click any key to clear it)")
                            }
                        }

                        // PCB Chassis Frame matching HardwareCanvas.qml
                        Rectangle {
                            id: wizardPcbChassis
                            Kirigami.Theme.colorSet: Kirigami.Theme.View
                            Kirigami.Theme.inherit: false
                            readonly property color frameBorderColor: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                            readonly property real uiScale: Math.max(
                                0.72,
                                Math.min(
                                    (wizardPcbChassis.width - 24) / 282.0,
                                    (wizardPcbChassis.height - 24) / 354.0
                                )
                            )

                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 12
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.color: wizardPcbChassis.frameBorderColor
                            border.width: 1
                            clip: true

                            Rectangle {
                                anchors.fill: parent
                                anchors.margins: 5
                                radius: 10
                                color: "transparent"
                                border.color: wizardPcbChassis.frameBorderColor
                                opacity: 0.5
                                border.width: 1
                            }

                            Grid {
                                id: wizardSwitchMatrix
                                anchors.centerIn: parent
                                columns: 4
                                rowSpacing: 6
                                columnSpacing: 6
                                scale: wizardPcbChassis.uiScale
                                transformOrigin: Item.Center

                                Repeater {
                                    model: 20
                                    delegate: Item {
                                        id: wizKeycapItem
                                        width: 66
                                        height: 66
                                        readonly property int slot: index + 1
                                        readonly property var assigned: wizardDialog.keyAssignments[String(slot)]
                                        readonly property string l1: assigned ? (assigned.label1 || "") : ""
                                        readonly property string l2: assigned ? (assigned.label2 || "") : ""
                                        readonly property string scText: assigned ? (assigned.shortcut || "") : ""
                                        readonly property bool isSelected: wizardDialog.selectedMatrixSlot === slot
                                        readonly property bool isDraggingThis: wizardDialog.draggingMatrixSlot === slot
                                        readonly property bool isDropTarget: (wizardDialog.dropTargetSlot === slot && (wizardDialog.draggingShortcutItem !== null || (wizardDialog.draggingMatrixSlot >= 1 && wizardDialog.draggingMatrixSlot !== slot)))
                                            || wizKeyDropArea.containsDrag
                                        readonly property color ledColor: assigned
                                            ? wizardDialog.categoryLedColor(assigned.category)
                                            : (wizardDialog.dimUnusedKeys
                                                ? Qt.rgba(0.28, 0.32, 0.40, 0.45)
                                                : wizardDialog.profileBgColor)
                                        property real dragOffsetX: 0
                                        property real dragOffsetY: 0

                                        z: isDraggingThis ? 100 : (isDropTarget ? 10 : 1)

                                        // Socket shown while dragging keycap
                                        Rectangle {
                                            visible: wizKeycapItem.isDraggingThis
                                            anchors.fill: parent
                                            radius: 8
                                            color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.45)
                                            border.width: 1
                                            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                                        }

                                        Rectangle {
                                            id: wizKeyVisual
                                            width: parent.width
                                            height: parent.height
                                            x: wizKeycapItem.isDraggingThis ? wizKeycapItem.dragOffsetX : 0
                                            y: wizKeycapItem.isDraggingThis ? wizKeycapItem.dragOffsetY : 0
                                            scale: wizKeycapItem.isDraggingThis ? 1.06 : 1.0
                                            opacity: wizKeycapItem.isDraggingThis ? 0.92 : 1.0
                                            radius: 8
                                            color: Kirigami.Theme.backgroundColor
                                            border.width: wizKeycapItem.isDropTarget ? 2.5 : (wizKeycapItem.isSelected ? 2 : 1)
                                            border.color: wizKeycapItem.isDropTarget
                                                ? Kirigami.Theme.positiveTextColor
                                                : (wizKeycapItem.isSelected
                                                    ? Kirigami.Theme.highlightColor
                                                    : wizardPcbChassis.frameBorderColor)

                                            QQC2.Label {
                                                x: 6
                                                y: 1
                                                width: 14
                                                height: 10
                                                horizontalAlignment: Text.AlignLeft
                                                verticalAlignment: Text.AlignVCenter
                                                text: wizKeycapItem.slot
                                                font.pixelSize: 8
                                                font.bold: true
                                                font.family: "monospace"
                                                color: wizKeycapItem.isSelected
                                                    ? Kirigami.Theme.textColor
                                                    : Kirigami.Theme.disabledTextColor
                                            }

                                            Rectangle {
                                                x: 22
                                                y: 4
                                                width: 34
                                                height: 4
                                                radius: 2
                                                color: wizKeycapItem.ledColor
                                            }

                                            Rectangle {
                                                id: wizKeyInnerWell
                                                anchors.fill: parent
                                                anchors.margins: 5
                                                anchors.topMargin: 12
                                                radius: 6
                                                clip: true
                                                color: wizKeycapItem.isDropTarget
                                                    ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.positiveTextColor.r, Kirigami.Theme.positiveTextColor.g, Kirigami.Theme.positiveTextColor.b, 0.28))
                                                    : (wizKeyMouse.pressed
                                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(wizKeycapItem.ledColor.r, wizKeycapItem.ledColor.g, wizKeycapItem.ledColor.b, 0.35))
                                                        : (wizKeycapItem.isSelected
                                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.18))
                                                            : Kirigami.Theme.alternateBackgroundColor))
                                                border.width: 1
                                                border.color: wizKeycapItem.isDropTarget
                                                    ? Kirigami.Theme.positiveTextColor
                                                    : (wizKeycapItem.isSelected
                                                        ? Kirigami.Theme.highlightColor
                                                        : Qt.rgba(wizKeycapItem.ledColor.r, wizKeycapItem.ledColor.g, wizKeycapItem.ledColor.b, wizKeycapItem.assigned ? 0.65 : 0.3))

                                                Column {
                                                    anchors.horizontalCenter: parent.horizontalCenter
                                                    anchors.verticalCenter: parent.verticalCenter
                                                    anchors.verticalCenterOffset: (wizKeycapItem.assigned && !wizKeycapItem.isDropTarget) ? -4 : 0
                                                    width: parent.width - 6
                                                    spacing: 0

                                                    QQC2.Label {
                                                        width: parent.width
                                                        horizontalAlignment: Text.AlignHCenter
                                                        verticalAlignment: Text.AlignVCenter
                                                        text: wizKeycapItem.assigned ? wizKeycapItem.l1 : ""
                                                        font.pixelSize: wizKeycapItem.l2.length > 0 ? 10 : 11
                                                        fontSizeMode: Text.HorizontalFit
                                                        minimumPixelSize: 7
                                                        elide: Text.ElideRight
                                                        font.bold: true
                                                        color: Kirigami.Theme.textColor
                                                    }

                                                    QQC2.Label {
                                                        visible: wizKeycapItem.l2.length > 0
                                                        width: parent.width
                                                        horizontalAlignment: Text.AlignHCenter
                                                        verticalAlignment: Text.AlignVCenter
                                                        text: wizKeycapItem.l2
                                                        font.pixelSize: 9
                                                        fontSizeMode: Text.HorizontalFit
                                                        minimumPixelSize: 7
                                                        elide: Text.ElideRight
                                                        font.weight: Font.Medium
                                                        color: Kirigami.Theme.disabledTextColor
                                                    }
                                                }

                                                // Subtle footer shortcut pill when assigned
                                                Rectangle {
                                                    visible: !!wizKeycapItem.assigned && wizKeycapItem.scText.length > 0 && !wizKeycapItem.isDropTarget
                                                    anchors.bottom: parent.bottom
                                                    anchors.bottomMargin: 2
                                                    anchors.horizontalCenter: parent.horizontalCenter
                                                    radius: 2
                                                    width: Math.min(parent.width - 4, scFooterLbl.implicitWidth + 4)
                                                    height: 10
                                                    color: Qt.alpha(wizKeycapItem.ledColor, wizardDialog.isLightTheme ? 0.16 : 0.22)
                                                    border.width: 1
                                                    border.color: Qt.alpha(wizKeycapItem.ledColor, 0.6)

                                                    QQC2.Label {
                                                        id: scFooterLbl
                                                        anchors.centerIn: parent
                                                        width: parent.width - 2
                                                        horizontalAlignment: Text.AlignHCenter
                                                        text: wizKeycapItem.scText
                                                        font.pixelSize: 7
                                                        font.family: "monospace"
                                                        font.bold: true
                                                        elide: Text.ElideRight
                                                        color: Kirigami.Theme.textColor
                                                    }
                                                }

                                                // Drop Target Indicator Pill
                                                Rectangle {
                                                    visible: wizKeycapItem.isDropTarget
                                                    anchors.bottom: parent.bottom
                                                    anchors.bottomMargin: 2
                                                    anchors.horizontalCenter: parent.horizontalCenter
                                                    radius: 3
                                                    width: wizDropLbl.implicitWidth + 6
                                                    height: 13
                                                    color: Kirigami.Theme.positiveTextColor

                                                    QQC2.Label {
                                                        id: wizDropLbl
                                                        anchors.centerIn: parent
                                                        text: wizardDialog.draggingShortcutItem !== null ? qsTr("⬇ Assign") : qsTr("⇄ Swap")
                                                        font.pixelSize: 9
                                                        font.bold: true
                                                        color: Kirigami.Theme.backgroundColor
                                                    }
                                                }
                                            }
                                        }

                                        DropArea {
                                            id: wizKeyDropArea
                                            anchors.fill: parent
                                            keys: ["application/x-ducky-shortcut"]
                                            onEntered: (drag) => drag.acceptProposedAction()
                                            onDropped: (drop) => {
                                                if (drop.hasFormat("application/x-ducky-shortcut")) {
                                                    try {
                                                        const data = JSON.parse(drop.getDataAsString("application/x-ducky-shortcut"))
                                                        wizardDialog.assignShortcutToSlot(wizKeycapItem.slot, data)
                                                        drop.acceptProposedAction()
                                                    } catch (e) {
                                                        console.warn("Drop parse error:", e)
                                                    }
                                                }
                                            }
                                        }

                                        MouseArea {
                                            id: wizKeyMouse
                                            anchors.fill: parent
                                            hoverEnabled: true
                                            acceptedButtons: Qt.LeftButton | Qt.RightButton
                                            preventStealing: true
                                            cursorShape: Qt.PointingHandCursor
                                            property real pressX: 0
                                            property real pressY: 0
                                            property bool didDrag: false

                                            onPressed: (mouse) => {
                                                if (mouse.button === Qt.LeftButton) {
                                                    pressX = mouse.x
                                                    pressY = mouse.y
                                                    didDrag = false
                                                    wizKeycapItem.dragOffsetX = 0
                                                    wizKeycapItem.dragOffsetY = 0
                                                }
                                            }

                                            onPositionChanged: (mouse) => {
                                                if (!(mouse.buttons & Qt.LeftButton) || !wizKeycapItem.assigned) return
                                                const dx = mouse.x - pressX
                                                const dy = mouse.y - pressY
                                                if (!didDrag && Math.hypot(dx, dy) > 6) {
                                                    didDrag = true
                                                    wizardDialog.draggingMatrixSlot = wizKeycapItem.slot
                                                }
                                                if (didDrag) {
                                                    wizKeycapItem.dragOffsetX = dx
                                                    wizKeycapItem.dragOffsetY = dy
                                                    const target = wizardDialog.hitTestMatrixSlot(wizKeyMouse, mouse.x, mouse.y)
                                                    wizardDialog.dropTargetSlot = (target >= 1 && target <= 20 && target !== wizKeycapItem.slot) ? target : -1
                                                }
                                            }

                                            onReleased: (mouse) => {
                                                if (mouse.button === Qt.LeftButton && didDrag) {
                                                    const target = wizardDialog.dropTargetSlot
                                                    didDrag = false
                                                    wizKeycapItem.dragOffsetX = 0
                                                    wizKeycapItem.dragOffsetY = 0
                                                    wizardDialog.draggingMatrixSlot = -1
                                                    wizardDialog.dropTargetSlot = -1
                                                    if (target >= 1 && target <= 20 && target !== wizKeycapItem.slot) {
                                                        wizardDialog.swapMatrixSlots(wizKeycapItem.slot, target)
                                                    }
                                                    return
                                                }
                                                if (mouse.button === Qt.LeftButton) {
                                                    wizardDialog.selectedMatrixSlot = wizKeycapItem.slot
                                                } else if (mouse.button === Qt.RightButton) {
                                                    wizardDialog.clearSlotAssignment(wizKeycapItem.slot)
                                                }
                                            }

                                            onCanceled: {
                                                didDrag = false
                                                wizKeycapItem.dragOffsetX = 0
                                                wizKeycapItem.dragOffsetY = 0
                                                if (wizardDialog.draggingMatrixSlot === wizKeycapItem.slot) {
                                                    wizardDialog.draggingMatrixSlot = -1
                                                    wizardDialog.dropTargetSlot = -1
                                                }
                                            }

                                            QQC2.ToolTip.visible: wizKeyMouse.containsMouse && !didDrag
                                            QQC2.ToolTip.delay: 500
                                            QQC2.ToolTip.text: wizKeycapItem.assigned
                                                ? qsTr("Key %1: %2 (%3) • Drag to swap • Right-click to clear").arg(wizKeycapItem.slot).arg(wizKeycapItem.assigned.action).arg(wizKeycapItem.assigned.shortcut)
                                                : qsTr("Key %1 (Empty) • Drag any shortcut key here or click 'Assign → K%1'").arg(wizKeycapItem.slot)
                                        }
                                    }
                                }
                            }
                        }

                        QQC2.Label {
                            Layout.fillWidth: true
                            text: qsTr("Tip: Drag any shortcut keycap onto the matrix, drag assigned keys to swap, or right-click a key to clear.")
                            font.pointSize: 8.5 * fontScale
                            color: Kirigami.Theme.disabledTextColor
                            elide: Text.ElideRight
                        }
                    }

                    // Right Column: Discovered Shortcuts Menu (Styled as Main Window Keycaps)
                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        spacing: 8

                        // App Search + Online Search Toggle Bar directly in Step 4
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.TextField {
                                id: step4AppSearchField
                                Layout.fillWidth: true
                                implicitHeight: Math.round(28 * fontScale)
                                font.pointSize: 10 * fontScale
                                placeholderText: qsTr("App or game (e.g. Minecraft, Blender, VS Code, Firefox)...")
                                text: wizardDialog.targetAppName || wizardDialog.profileName
                                onTextEdited: wizardDialog.targetAppName = text
                                onAccepted: wizardDialog.triggerShortcutsSearch(step4AppSearchField.text)
                            }

                            QQC2.CheckBox {
                                id: step4WebCheck
                                text: qsTr("🌐 Web Search")
                                font.pointSize: 9.5 * fontScale
                                checked: wizardDialog.allowGoogleSearch
                                onToggled: {
                                    wizardDialog.allowGoogleSearch = checked
                                    if (controller) {
                                        controller.setGoogleSearchPermission(checked)
                                    }
                                    wizardDialog.triggerShortcutsSearch(step4AppSearchField.text)
                                }
                                QQC2.ToolTip.visible: hovered
                                QQC2.ToolTip.text: qsTr("Allow online web search for application shortcut cheatsheets")
                            }

                            QQC2.Button {
                                text: controller && controller.searchShortcutsBusy ? qsTr("Searching...") : qsTr("🔍 Search")
                                implicitHeight: Math.round(28 * fontScale)
                                font.pointSize: 10 * fontScale
                                highlighted: true
                                enabled: step4AppSearchField.text.trim().length > 0 && !(controller && controller.searchShortcutsBusy)
                                onClicked: wizardDialog.triggerShortcutsSearch(step4AppSearchField.text)
                            }
                        }

                        // Filter Bar
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.TextField {
                                id: searchBox
                                Layout.fillWidth: true
                                implicitHeight: Math.round(28 * fontScale)
                                font.pointSize: 10 * fontScale
                                placeholderText: qsTr("Filter %1 shortcuts (e.g. Inventory, Debug, F3, Chunk, Save)...").arg(wizardDialog.discoveredShortcutsList.length)
                                text: wizardDialog.shortcutSearchFilter
                                onTextChanged: wizardDialog.shortcutSearchFilter = text
                            }

                            QQC2.Label {
                                text: qsTr("%1 keys").arg(shortcutsListView.count)
                                font.pointSize: 9.5 * fontScale
                                font.bold: true
                                color: Kirigami.Theme.disabledTextColor
                            }
                        }

                        // Menu of Shortcut Keys (Each rendered with Main Window Keycap Look & Feel)
                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 12
                            color: Kirigami.Theme.alternateBackgroundColor
                            border.width: 1
                            border.color: wizardDialog.frameBorderColor
                            clip: true

                            ListView {
                                id: shortcutsListView
                                anchors.fill: parent
                                anchors.margins: 8
                                spacing: 6
                                boundsBehavior: Flickable.StopAtBounds
                                model: {
                                    const filter = wizardDialog.shortcutSearchFilter.trim().toLowerCase()
                                    if (!filter) return wizardDialog.discoveredShortcutsList
                                    return wizardDialog.discoveredShortcutsList.filter(s =>
                                        (s.action && s.action.toLowerCase().includes(filter)) ||
                                        (s.shortcut && s.shortcut.toLowerCase().includes(filter)) ||
                                        (s.category && s.category.toLowerCase().includes(filter)) ||
                                        (s.label_l1 && s.label_l1.toLowerCase().includes(filter)) ||
                                        (s.label_l2 && s.label_l2.toLowerCase().includes(filter))
                                    )
                                }

                                delegate: Item {
                                    id: shortcutCard
                                    width: shortcutsListView.width
                                    height: 76
                                    readonly property color itemLedColor: wizardDialog.categoryLedColor(modelData.category)

                                    Rectangle {
                                        id: cardVisual
                                        anchors.fill: parent
                                        radius: 8
                                        color: cardDragMouse.containsMouse
                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.10))
                                            : Kirigami.Theme.backgroundColor
                                        border.width: cardDragMouse.containsMouse ? 1.5 : 1
                                        border.color: cardDragMouse.containsMouse
                                            ? Kirigami.Theme.highlightColor
                                            : wizardDialog.frameBorderColor

                                        RowLayout {
                                            anchors.fill: parent
                                            anchors.leftMargin: 6
                                            anchors.rightMargin: 10
                                            spacing: 10

                                            // Authentic Main-Window 66x66 duckyPad Keycap Preview
                                            Item {
                                                Layout.preferredWidth: 66
                                                Layout.preferredHeight: 66

                                                Rectangle {
                                                    anchors.fill: parent
                                                    radius: 8
                                                    color: Kirigami.Theme.backgroundColor
                                                    border.width: cardDragMouse.containsMouse ? 2 : 1
                                                    border.color: cardDragMouse.containsMouse
                                                        ? Kirigami.Theme.highlightColor
                                                        : wizardDialog.frameBorderColor

                                                    QQC2.Label {
                                                        x: 6
                                                        y: 1
                                                        width: 14
                                                        height: 10
                                                        text: "⋮⋮"
                                                        font.pixelSize: 8
                                                        font.bold: true
                                                        font.family: "monospace"
                                                        color: Kirigami.Theme.disabledTextColor
                                                    }

                                                    Rectangle {
                                                        x: 22
                                                        y: 4
                                                        width: 34
                                                        height: 4
                                                        radius: 2
                                                        color: shortcutCard.itemLedColor
                                                    }

                                                    Rectangle {
                                                        anchors.fill: parent
                                                        anchors.margins: 5
                                                        anchors.topMargin: 12
                                                        radius: 6
                                                        clip: true
                                                        color: cardDragMouse.pressed
                                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(shortcutCard.itemLedColor.r, shortcutCard.itemLedColor.g, shortcutCard.itemLedColor.b, 0.30))
                                                            : (cardDragMouse.containsMouse
                                                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.16))
                                                                : Kirigami.Theme.alternateBackgroundColor)
                                                        border.width: 1
                                                        border.color: cardDragMouse.containsMouse
                                                            ? Kirigami.Theme.highlightColor
                                                            : Qt.rgba(shortcutCard.itemLedColor.r, shortcutCard.itemLedColor.g, shortcutCard.itemLedColor.b, 0.55)

                                                        Column {
                                                            anchors.centerIn: parent
                                                            width: parent.width - 6
                                                            spacing: 0

                                                            QQC2.Label {
                                                                width: parent.width
                                                                horizontalAlignment: Text.AlignHCenter
                                                                verticalAlignment: Text.AlignVCenter
                                                                text: modelData.label_l1 || ""
                                                                font.pixelSize: (modelData.label_l2 && modelData.label_l2.length > 0) ? 10 : 11
                                                                fontSizeMode: Text.HorizontalFit
                                                                minimumPixelSize: 7
                                                                elide: Text.ElideRight
                                                                font.bold: true
                                                                color: Kirigami.Theme.textColor
                                                            }

                                                            QQC2.Label {
                                                                visible: !!(modelData.label_l2 && modelData.label_l2.length > 0)
                                                                width: parent.width
                                                                horizontalAlignment: Text.AlignHCenter
                                                                verticalAlignment: Text.AlignVCenter
                                                                text: modelData.label_l2 || ""
                                                                font.pixelSize: 9
                                                                fontSizeMode: Text.HorizontalFit
                                                                minimumPixelSize: 7
                                                                elide: Text.ElideRight
                                                                font.weight: Font.Medium
                                                                color: Kirigami.Theme.disabledTextColor
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            // Action Details & Key Combo Badges
                                            ColumnLayout {
                                                Layout.fillWidth: true
                                                spacing: 3

                                                QQC2.Label {
                                                    Layout.fillWidth: true
                                                    text: modelData.action || ""
                                                    font.bold: true
                                                    font.pointSize: 10 * fontScale
                                                    elide: Text.ElideRight
                                                    color: Kirigami.Theme.textColor
                                                }

                                                RowLayout {
                                                    spacing: 6

                                                    Rectangle {
                                                        radius: 4
                                                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                                                        border.width: 1
                                                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.55)
                                                        implicitWidth: scBadgeText.implicitWidth + 10
                                                        implicitHeight: Math.round(18 * fontScale)

                                                        QQC2.Label {
                                                            id: scBadgeText
                                                            anchors.centerIn: parent
                                                            text: modelData.shortcut || ""
                                                            font.family: "monospace"
                                                            font.bold: true
                                                            font.pointSize: 9 * fontScale
                                                            color: Kirigami.Theme.highlightColor
                                                        }
                                                    }

                                                    Rectangle {
                                                        visible: !!(modelData.category || modelData.source)
                                                        radius: 4
                                                        color: Qt.alpha(shortcutCard.itemLedColor, 0.14)
                                                        border.width: 1
                                                        border.color: Qt.alpha(shortcutCard.itemLedColor, 0.45)
                                                        implicitWidth: catBadgeText.implicitWidth + 8
                                                        implicitHeight: Math.round(18 * fontScale)

                                                        QQC2.Label {
                                                            id: catBadgeText
                                                            anchors.centerIn: parent
                                                            text: modelData.category || modelData.source || "General"
                                                            font.pointSize: 8.5 * fontScale
                                                            color: Kirigami.Theme.textColor
                                                        }
                                                    }
                                                }
                                            }

                                            // Main-Window-Styled Assign Button
                                            QQC2.Button {
                                                id: quickAssignBtn
                                                z: 5
                                                text: qsTr("Assign → K%1").arg(wizardDialog.selectedMatrixSlot)
                                                implicitHeight: Math.round(28 * fontScale)
                                                font.pointSize: 9.5 * fontScale
                                                highlighted: true
                                                onClicked: {
                                                    wizardDialog.assignShortcutToSlot(wizardDialog.selectedMatrixSlot, modelData)
                                                    if (wizardDialog.selectedMatrixSlot < 20) {
                                                        wizardDialog.selectedMatrixSlot++
                                                    }
                                                }
                                            }
                                        }

                                        // Pointer-Tracked Drag & Drop MouseArea (100% reliable on Wayland & X11)
                                        MouseArea {
                                            id: cardDragMouse
                                            anchors.top: parent.top
                                            anchors.bottom: parent.bottom
                                            anchors.left: parent.left
                                            anchors.right: quickAssignBtn.left
                                            anchors.rightMargin: 6
                                            hoverEnabled: true
                                            preventStealing: true
                                            cursorShape: didDrag ? Qt.ClosedHandCursor : Qt.OpenHandCursor
                                            property real pressX: 0
                                            property real pressY: 0
                                            property bool didDrag: false

                                            onPressed: (mouse) => {
                                                if (mouse.button === Qt.LeftButton) {
                                                    pressX = mouse.x
                                                    pressY = mouse.y
                                                    didDrag = false
                                                }
                                            }

                                            onPositionChanged: (mouse) => {
                                                if (!(mouse.buttons & Qt.LeftButton)) return
                                                const dx = mouse.x - pressX
                                                const dy = mouse.y - pressY
                                                if (!didDrag && Math.hypot(dx, dy) > 5) {
                                                    didDrag = true
                                                    wizardDialog.draggingShortcutItem = modelData
                                                }
                                                if (didDrag) {
                                                    const winPt = cardDragMouse.mapToItem(wizardDialog.contentItem, mouse.x, mouse.y)
                                                    wizardDialog.dragOverlayX = winPt.x - 33
                                                    wizardDialog.dragOverlayY = winPt.y - 33
                                                    wizardDialog.dropTargetSlot = wizardDialog.hitTestMatrixSlot(cardDragMouse, mouse.x, mouse.y)
                                                }
                                            }

                                            onReleased: (mouse) => {
                                                if (didDrag) {
                                                    const target = wizardDialog.dropTargetSlot
                                                    const itemData = wizardDialog.draggingShortcutItem
                                                    didDrag = false
                                                    wizardDialog.draggingShortcutItem = null
                                                    wizardDialog.dropTargetSlot = -1
                                                    if (target >= 1 && target <= 20 && itemData) {
                                                        wizardDialog.assignShortcutToSlot(target, itemData)
                                                    }
                                                    return
                                                }
                                            }

                                            onDoubleClicked: {
                                                wizardDialog.assignShortcutToSlot(wizardDialog.selectedMatrixSlot, modelData)
                                                if (wizardDialog.selectedMatrixSlot < 20) {
                                                    wizardDialog.selectedMatrixSlot++
                                                }
                                            }

                                            onCanceled: {
                                                didDrag = false
                                                wizardDialog.draggingShortcutItem = null
                                                wizardDialog.dropTargetSlot = -1
                                            }

                                            QQC2.ToolTip.visible: cardDragMouse.containsMouse && !didDrag
                                            QQC2.ToolTip.delay: 450
                                            QQC2.ToolTip.text: qsTr("Drag this keycap onto any duckyPad key (1–20), or double-click to assign to Key %1").arg(wizardDialog.selectedMatrixSlot)
                                        }
                                    }
                                }

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    visible: shortcutsListView.count === 0 && !(controller && controller.searchShortcutsBusy)
                                    text: qsTr("No shortcuts found. Type an application or game name above (e.g. Minecraft) and click Search.")
                                    font.pointSize: 10 * fontScale
                                    color: Kirigami.Theme.disabledTextColor
                                    wrapMode: Text.WordWrap
                                    width: parent.width - 40
                                    horizontalAlignment: Text.AlignHCenter
                                }

                                QQC2.BusyIndicator {
                                    anchors.centerIn: parent
                                    running: controller && controller.searchShortcutsBusy
                                    visible: running
                                }
                            }
                        }
                    }
                }
            }
        }

        // Bottom Navigation Bar
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            QQC2.Button {
                text: qsTr("Cancel")
                implicitHeight: Math.round(30 * fontScale)
                font.pointSize: 10 * fontScale
                onClicked: wizardDialog.close()
            }

            Item { Layout.fillWidth: true }

            QQC2.Button {
                text: qsTr("‹ Back")
                implicitHeight: Math.round(30 * fontScale)
                font.pointSize: 10 * fontScale
                enabled: wizardDialog.currentStep > 0
                onClicked: wizardDialog.currentStep--
            }

            QQC2.Button {
                text: qsTr("Next ›")
                implicitHeight: Math.round(30 * fontScale)
                font.pointSize: 10 * fontScale
                visible: wizardDialog.currentStep < 3
                highlighted: true
                onClicked: wizardDialog.currentStep++
            }

            QQC2.Button {
                text: qsTr("✓ Create Profile & Finish")
                implicitHeight: Math.round(30 * fontScale)
                font.pointSize: 10 * fontScale
                visible: wizardDialog.currentStep === 3
                highlighted: true
                onClicked: wizardDialog.finishAndCreateProfile()
            }
        }
    }

    // Floating Lifted Keycap Overlay during Shortcut Drag-and-Drop
    Item {
        id: floatingShortcutOverlay
        parent: wizardDialog.contentItem
        visible: wizardDialog.draggingShortcutItem !== null
        x: wizardDialog.dragOverlayX
        y: wizardDialog.dragOverlayY
        width: 66
        height: 66
        z: 9999
        scale: 1.08
        opacity: 0.94

        readonly property color dragLedColor: wizardDialog.draggingShortcutItem
            ? wizardDialog.categoryLedColor(wizardDialog.draggingShortcutItem.category)
            : wizardDialog.profileBgColor

        Rectangle {
            anchors.fill: parent
            radius: 8
            color: Kirigami.Theme.backgroundColor
            border.width: 2
            border.color: wizardDialog.dropTargetSlot >= 1
                ? Kirigami.Theme.positiveTextColor
                : Kirigami.Theme.highlightColor

            Rectangle {
                x: 22
                y: 4
                width: 34
                height: 4
                radius: 2
                color: floatingShortcutOverlay.dragLedColor
            }

            Rectangle {
                anchors.fill: parent
                anchors.margins: 5
                anchors.topMargin: 12
                radius: 6
                clip: true
                color: Qt.tint(
                    Kirigami.Theme.backgroundColor,
                    Qt.rgba(
                        floatingShortcutOverlay.dragLedColor.r,
                        floatingShortcutOverlay.dragLedColor.g,
                        floatingShortcutOverlay.dragLedColor.b,
                        0.28
                    )
                )
                border.width: 1
                border.color: wizardDialog.dropTargetSlot >= 1
                    ? Kirigami.Theme.positiveTextColor
                    : Kirigami.Theme.highlightColor

                Column {
                    anchors.centerIn: parent
                    width: parent.width - 6
                    spacing: 0

                    QQC2.Label {
                        width: parent.width
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        text: wizardDialog.draggingShortcutItem ? (wizardDialog.draggingShortcutItem.label_l1 || "") : ""
                        font.pixelSize: 10
                        fontSizeMode: Text.HorizontalFit
                        minimumPixelSize: 7
                        elide: Text.ElideRight
                        font.bold: true
                        color: Kirigami.Theme.textColor
                    }

                    QQC2.Label {
                        visible: !!(wizardDialog.draggingShortcutItem && wizardDialog.draggingShortcutItem.label_l2)
                        width: parent.width
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        text: wizardDialog.draggingShortcutItem ? (wizardDialog.draggingShortcutItem.label_l2 || "") : ""
                        font.pixelSize: 9
                        fontSizeMode: Text.HorizontalFit
                        minimumPixelSize: 7
                        elide: Text.ElideRight
                        font.weight: Font.Medium
                        color: Kirigami.Theme.disabledTextColor
                    }
                }
            }
        }
    }
}
