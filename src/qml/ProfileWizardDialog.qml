import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import org.kde.kirigami as Kirigami

Window {
    id: wizardDialog
    title: qsTr("New Profile Wizard — DuckyDeck Studio")
    width: 900
    height: 660
    minimumWidth: 800
    minimumHeight: 580
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

    property int currentStep: 0

    // Step 0: Basics
    property string profileName: ""
    property int targetSlotNumber: 1
    property string selectedTemplateId: "builtin_blank"

    // Step 1: Target App & Auto-Switch
    property string targetAppClass: ""
    property string targetAppTitle: ""
    property string targetAppName: ""
    property bool autoSwitchEnabled: true
    property bool allowGoogleSearch: controller ? controller.googleSearchShortcutsPermitted : false

    // Step 2: Appearance & Orientation
    property color profileBgColor: "#00aaff"
    property color keydownColor: "#ffffff"
    property bool dimUnusedKeys: true
    property int oledRotation: 0

    // Step 3: Assigned Shortcuts Map: slotNumber (1..20) -> { label1, label2, script, action, shortcut, category }
    property var keyAssignments: ({})
    property int selectedMatrixSlot: 1
    property string shortcutSearchFilter: ""

    // Discovered shortcuts parsed from controller
    property var discoveredShortcutsList: []

    function resetWizard() {
        currentStep = 0
        profileName = ""
        targetAppClass = ""
        targetAppTitle = ""
        targetAppName = ""
        autoSwitchEnabled = true
        allowGoogleSearch = controller ? controller.googleSearchShortcutsPermitted : false
        profileBgColor = "#00aaff"
        keydownColor = "#ffffff"
        dimUnusedKeys = true
        oledRotation = 0
        keyAssignments = {}
        selectedMatrixSlot = 1
        shortcutSearchFilter = ""
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
                wizardDialog.targetAppTitle = controller.lastPickedWindowTitle
                if (!wizardDialog.profileName || wizardDialog.profileName === "New Profile") {
                    wizardDialog.profileName = cleanAppNameFromClass(controller.lastPickedWindowClass)
                }
                wizardDialog.targetAppName = cleanAppNameFromClass(controller.lastPickedWindowClass)
                triggerShortcutsSearch(wizardDialog.targetAppName)
            }
        }
    }

    function cleanAppNameFromClass(wmClass) {
        if (!wmClass) return "App"
        const parts = wmClass.split(".")
        const raw = parts[parts.length - 1] || wmClass
        return raw.charAt(0).toUpperCase() + raw.slice(1)
    }

    function triggerShortcutsSearch(appName) {
        if (!controller || !appName) return
        controller.searchAppShortcuts(appName, allowGoogleSearch)
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
    }

    function clearSlotAssignment(slotNum) {
        const newMap = Object.assign({}, keyAssignments)
        delete newMap[String(slotNum)]
        keyAssignments = newMap
    }

    function autoFillEmptySlots() {
        if (!discoveredShortcutsList || discoveredShortcutsList.length === 0) return
        const newMap = Object.assign({}, keyAssignments)
        let scIdx = 0
        for (let slot = 1; slot <= 15; slot++) {
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

        const finalName = profileName.trim() || ("Profile " + targetSlotNumber)
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

        // Color extraction helper
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
            height: 52
            radius: 8
            color: Qt.alpha(Kirigami.Theme.alternateBackgroundColor, 0.6)
            border.width: 1
            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.12)

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
                                    font.pixelSize: Math.round(11 * fontScale)
                                    color: (wizardDialog.currentStep >= modelData.step) ? "#ffffff" : Kirigami.Theme.textColor
                                }
                            }

                            QQC2.Label {
                                text: modelData.title
                                font.bold: wizardDialog.currentStep === modelData.step
                                font.pixelSize: Math.round(12 * fontScale)
                                color: wizardDialog.currentStep === modelData.step
                                    ? Kirigami.Theme.highlightColor
                                    : Kirigami.Theme.textColor
                                opacity: wizardDialog.currentStep >= modelData.step ? 1.0 : 0.6
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
                        color: Kirigami.Theme.disabledTextColor
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        height: 1
                        color: Qt.alpha(Kirigami.Theme.textColor, 0.1)
                    }

                    // Profile Name
                    ColumnLayout {
                        spacing: 4
                        Layout.fillWidth: true

                        QQC2.Label {
                            text: qsTr("Profile Name:")
                            font.bold: true
                        }

                        QQC2.TextField {
                            id: nameField
                            Layout.fillWidth: true
                            placeholderText: qsTr("e.g., Firefox, VS Code, Blender, Streaming")
                            text: wizardDialog.profileName
                            onTextChanged: wizardDialog.profileName = text
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
                            }

                            QQC2.SpinBox {
                                id: slotSpin
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
                            }

                            QQC2.ComboBox {
                                id: templateCombo
                                Layout.fillWidth: true
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
                        color: Kirigami.Theme.disabledTextColor
                    }

                    // Window Picker Card
                    Rectangle {
                        Layout.fillWidth: true
                        radius: 8
                        color: Qt.alpha(Kirigami.Theme.alternateBackgroundColor, 0.4)
                        border.width: 1
                        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
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
                                    font.pixelSize: Math.round(11 * fontScale)
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
                                        font.pixelSize: Math.round(11 * fontScale)
                                    }
                                    QQC2.TextField {
                                        Layout.fillWidth: true
                                        text: wizardDialog.targetAppClass
                                        placeholderText: qsTr("e.g. org.kde.dolphin, firefox, code")
                                        onTextChanged: {
                                            wizardDialog.targetAppClass = text
                                            wizardDialog.targetAppName = wizardDialog.cleanAppNameFromClass(text)
                                        }
                                    }
                                }

                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 2
                                    QQC2.Label {
                                        text: qsTr("Window Title Filter (optional):")
                                        font.bold: true
                                        font.pixelSize: Math.round(11 * fontScale)
                                    }
                                    QQC2.TextField {
                                        Layout.fillWidth: true
                                        text: wizardDialog.targetAppTitle
                                        placeholderText: qsTr("Optional title regex or substring")
                                        onTextChanged: wizardDialog.targetAppTitle = text
                                    }
                                }
                            }

                            QQC2.CheckBox {
                                id: autoSwitchToggle
                                text: qsTr("Enable automatic profile switching when this application is focused")
                                checked: wizardDialog.autoSwitchEnabled
                                onToggled: wizardDialog.autoSwitchEnabled = checked
                            }
                        }
                    }

                    // Google Search Permission & Discovery Section
                    Rectangle {
                        Layout.fillWidth: true
                        radius: 8
                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.08)
                        border.width: 1
                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.3)
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
                                    text: qsTr("Online Keyboard Shortcut Discovery")
                                }
                            }

                            QQC2.Label {
                                text: qsTr("DuckyDeck Studio can query Google to fetch official shortcuts cheatsheets for %1. Local KDE shortcuts and offline catalogs are always available without internet.").arg(wizardDialog.targetAppName || qsTr("the selected application"))
                                wrapMode: Text.WordWrap
                                Layout.fillWidth: true
                                color: Kirigami.Theme.textColor
                            }

                            QQC2.CheckBox {
                                id: googlePermCheck
                                text: qsTr("Allow querying Google Search for shortcut cheatsheets (privacy permission)")
                                checked: wizardDialog.allowGoogleSearch
                                onToggled: {
                                    wizardDialog.allowGoogleSearch = checked
                                    if (controller) {
                                        controller.setGoogleSearchPermission(checked)
                                    }
                                }
                            }

                            RowLayout {
                                spacing: 10
                                QQC2.Button {
                                    text: controller && controller.searchShortcutsBusy ? qsTr("Searching Shortcuts...") : qsTr("Find Shortcuts for %1").arg(wizardDialog.targetAppName || "App")
                                    enabled: !!wizardDialog.targetAppName && !(controller && controller.searchShortcutsBusy)
                                    highlighted: true
                                    onClicked: {
                                        wizardDialog.triggerShortcutsSearch(wizardDialog.targetAppName)
                                    }
                                }

                                QQC2.BusyIndicator {
                                    running: controller && controller.searchShortcutsBusy
                                    Layout.preferredWidth: 22
                                    Layout.preferredHeight: 22
                                }

                                QQC2.Label {
                                    text: wizardDialog.discoveredShortcutsList.length > 0
                                        ? qsTr("✓ Found %1 shortcuts ready to drag and drop!").arg(wizardDialog.discoveredShortcutsList.length)
                                        : qsTr("Shortcuts will be shown in Step 4.")
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
                        color: Kirigami.Theme.disabledTextColor
                    }

                    // Background color palette
                    ColumnLayout {
                        spacing: 8
                        Layout.fillWidth: true

                        QQC2.Label {
                            text: qsTr("Profile Backlight Color:")
                            font.bold: true
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
                        }

                        RowLayout {
                            spacing: 12
                            QQC2.RadioButton {
                                text: qsTr("0° (Normal Portrait)")
                                checked: wizardDialog.oledRotation === 0
                                onToggled: if (checked) wizardDialog.oledRotation = 0
                            }
                            QQC2.RadioButton {
                                text: qsTr("90° (Landscape Right)")
                                checked: wizardDialog.oledRotation === 90
                                onToggled: if (checked) wizardDialog.oledRotation = 90
                            }
                            QQC2.RadioButton {
                                text: qsTr("180° (Inverted Portrait)")
                                checked: wizardDialog.oledRotation === 180
                                onToggled: if (checked) wizardDialog.oledRotation = 180
                            }
                            QQC2.RadioButton {
                                text: qsTr("270° (Landscape Left)")
                                checked: wizardDialog.oledRotation === 270
                                onToggled: if (checked) wizardDialog.oledRotation = 270
                            }
                        }
                    }
                }
            }

            // ==============================================================
            // Step 3: Hotkeys & Shortcuts Layout (Drag & Drop)
            // ==============================================================
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true

                RowLayout {
                    anchors.fill: parent
                    spacing: 14

                    // Left Column: 4x5 Key Matrix (Drop Targets)
                    ColumnLayout {
                        Layout.preferredWidth: 380
                        Layout.fillHeight: true
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            QQC2.Label {
                                text: qsTr("duckyPad Key Matrix (Drop here)")
                                font.bold: true
                            }
                            Item { Layout.fillWidth: true }
                            QQC2.Button {
                                text: qsTr("Auto-fill Empty")
                                icon.name: "edit-paste"
                                enabled: wizardDialog.discoveredShortcutsList.length > 0
                                onClicked: wizardDialog.autoFillEmptySlots()
                            }
                        }

                        // Grid of 15 standard keys + 5 expansion/encoder slots
                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 8
                            color: Qt.alpha(Kirigami.Theme.alternateBackgroundColor, 0.4)
                            border.width: 1
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                            clip: true

                            GridLayout {
                                anchors.centerIn: parent
                                columns: 3
                                rowSpacing: 8
                                columnSpacing: 8

                                Repeater {
                                    model: 15
                                    delegate: Rectangle {
                                        id: keyDropSlot
                                        readonly property int slotNum: index + 1
                                        readonly property var assigned: wizardDialog.keyAssignments[String(slotNum)]

                                        width: 80
                                        height: 64
                                        radius: 6
                                        color: dropTargetArea.containsDrag
                                            ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.25)
                                            : (assigned
                                                ? Qt.alpha(wizardDialog.profileBgColor, 0.35)
                                                : Qt.alpha(Kirigami.Theme.backgroundColor, 0.7))
                                        border.width: dropTargetArea.containsDrag ? 2 : (wizardDialog.selectedMatrixSlot === slotNum ? 2 : 1)
                                        border.color: dropTargetArea.containsDrag
                                            ? Kirigami.Theme.positiveTextColor
                                            : (wizardDialog.selectedMatrixSlot === slotNum
                                                ? Kirigami.Theme.highlightColor
                                                : Qt.alpha(Kirigami.Theme.textColor, 0.2))

                                        ColumnLayout {
                                            anchors.centerIn: parent
                                            spacing: 1

                                            QQC2.Label {
                                                Layout.alignment: Qt.AlignHCenter
                                                text: "Key " + keyDropSlot.slotNum
                                                font.pixelSize: Math.round(9 * fontScale)
                                                color: Kirigami.Theme.disabledTextColor
                                            }

                                            QQC2.Label {
                                                Layout.alignment: Qt.AlignHCenter
                                                text: keyDropSlot.assigned ? keyDropSlot.assigned.label1 : "—"
                                                font.bold: true
                                                font.pixelSize: Math.round(11 * fontScale)
                                                elide: Text.ElideRight
                                                color: Kirigami.Theme.textColor
                                            }

                                            QQC2.Label {
                                                Layout.alignment: Qt.AlignHCenter
                                                text: keyDropSlot.assigned ? (keyDropSlot.assigned.label2 || keyDropSlot.assigned.shortcut) : "+"
                                                font.pixelSize: Math.round(10 * fontScale)
                                                opacity: keyDropSlot.assigned ? 0.9 : 0.4
                                                color: keyDropSlot.assigned ? Kirigami.Theme.highlightColor : Kirigami.Theme.disabledTextColor
                                            }
                                        }

                                        DropArea {
                                            id: dropTargetArea
                                            anchors.fill: parent
                                            keys: ["application/x-ducky-shortcut"]
                                            onEntered: (drag) => drag.acceptProposedAction()
                                            onDropped: (drop) => {
                                                if (drop.hasFormat("application/x-ducky-shortcut")) {
                                                    try {
                                                        const data = JSON.parse(drop.getDataAsString("application/x-ducky-shortcut"))
                                                        wizardDialog.assignShortcutToSlot(keyDropSlot.slotNum, data)
                                                        drop.acceptProposedAction()
                                                    } catch (e) {
                                                        console.warn("Drop parse error:", e)
                                                    }
                                                }
                                            }
                                        }

                                        MouseArea {
                                            anchors.fill: parent
                                            cursorShape: Qt.PointingHandCursor
                                            onClicked: {
                                                wizardDialog.selectedMatrixSlot = keyDropSlot.slotNum
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Right Column: Discovered Shortcuts (Draggable Cards)
                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.TextField {
                                id: searchBox
                                Layout.fillWidth: true
                                placeholderText: qsTr("Filter shortcuts (e.g. Save, Tab, Find)...")
                                text: wizardDialog.shortcutSearchFilter
                                onTextChanged: wizardDialog.shortcutSearchFilter = text
                            }

                            QQC2.Button {
                                text: qsTr("Refresh")
                                icon.name: "view-refresh"
                                onClicked: wizardDialog.triggerShortcutsSearch(wizardDialog.targetAppName)
                            }
                        }

                        // Shortcuts List
                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 8
                            color: Qt.alpha(Kirigami.Theme.alternateBackgroundColor, 0.4)
                            border.width: 1
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.15)
                            clip: true

                            ListView {
                                id: shortcutsListView
                                anchors.fill: parent
                                anchors.margins: 6
                                spacing: 4
                                model: {
                                    const filter = wizardDialog.shortcutSearchFilter.trim().toLowerCase()
                                    if (!filter) return wizardDialog.discoveredShortcutsList
                                    return wizardDialog.discoveredShortcutsList.filter(s =>
                                        (s.action && s.action.toLowerCase().includes(filter)) ||
                                        (s.shortcut && s.shortcut.toLowerCase().includes(filter)) ||
                                        (s.category && s.category.toLowerCase().includes(filter))
                                    )
                                }

                                delegate: Item {
                                    id: shortcutCard
                                    width: shortcutsListView.width
                                    height: 48

                                    Rectangle {
                                        id: cardVisual
                                        anchors.fill: parent
                                        radius: 6
                                        color: cardDragMouse.drag.active
                                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.25)
                                            : (cardHoverArea.containsMouse ? Qt.alpha(Kirigami.Theme.highlightColor, 0.12) : Qt.alpha(Kirigami.Theme.backgroundColor, 0.6))
                                        border.width: 1
                                        border.color: cardHoverArea.containsMouse ? Kirigami.Theme.highlightColor : Qt.alpha(Kirigami.Theme.textColor, 0.15)

                                        RowLayout {
                                            anchors.fill: parent
                                            anchors.leftMargin: 8
                                            anchors.rightMargin: 8
                                            spacing: 8

                                            QQC2.Label {
                                                text: "⋮⋮"
                                                color: Kirigami.Theme.disabledTextColor
                                                font.pixelSize: 14
                                            }

                                            ColumnLayout {
                                                Layout.fillWidth: true
                                                spacing: 1

                                                QQC2.Label {
                                                    text: modelData.action
                                                    font.bold: true
                                                    font.pixelSize: Math.round(11 * fontScale)
                                                    elide: Text.ElideRight
                                                }

                                                RowLayout {
                                                    spacing: 6
                                                    QQC2.Label {
                                                        text: modelData.shortcut
                                                        font.family: "monospace"
                                                        font.pixelSize: Math.round(10 * fontScale)
                                                        color: Kirigami.Theme.highlightColor
                                                    }
                                                    QQC2.Label {
                                                        text: "• " + (modelData.source || modelData.category || "General")
                                                        font.pixelSize: Math.round(9 * fontScale)
                                                        color: Kirigami.Theme.disabledTextColor
                                                    }
                                                }
                                            }

                                            QQC2.Button {
                                                text: qsTr("Assign → K%1").arg(wizardDialog.selectedMatrixSlot)
                                                font.pixelSize: Math.round(9 * fontScale)
                                                onClicked: {
                                                    wizardDialog.assignShortcutToSlot(wizardDialog.selectedMatrixSlot, modelData)
                                                    if (wizardDialog.selectedMatrixSlot < 15) {
                                                        wizardDialog.selectedMatrixSlot++
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Draggable Item setup
                                    Item {
                                        id: dragItem
                                        Drag.active: cardDragMouse.drag.active
                                        Drag.source: dragItem
                                        Drag.hotSpot.x: width / 2
                                        Drag.hotSpot.y: height / 2
                                        Drag.mimeData: ({
                                            "application/x-ducky-shortcut": JSON.stringify(modelData)
                                        })
                                        Drag.dragType: Drag.Automatic
                                    }

                                    MouseArea {
                                        id: cardHoverArea
                                        anchors.fill: parent
                                        hoverEnabled: true
                                        acceptedButtons: Qt.NoButton
                                    }

                                    MouseArea {
                                        id: cardDragMouse
                                        anchors.fill: parent
                                        cursorShape: Qt.OpenHandCursor
                                        drag.target: dragItem
                                    }
                                }

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    visible: shortcutsListView.count === 0 && !(controller && controller.searchShortcutsBusy)
                                    text: qsTr("No shortcuts found. Go to Step 2 to search shortcuts for your app.")
                                    color: Kirigami.Theme.disabledTextColor
                                    wrapMode: Text.WordWrap
                                    width: parent.width - 40
                                    horizontalAlignment: Text.AlignHCenter
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
                onClicked: wizardDialog.close()
            }

            Item { Layout.fillWidth: true }

            QQC2.Button {
                text: qsTr("‹ Back")
                enabled: wizardDialog.currentStep > 0
                onClicked: wizardDialog.currentStep--
            }

            QQC2.Button {
                text: qsTr("Next ›")
                visible: wizardDialog.currentStep < 3
                highlighted: true
                onClicked: wizardDialog.currentStep++
            }

            QQC2.Button {
                text: qsTr("✓ Create Profile & Finish")
                visible: wizardDialog.currentStep === 3
                highlighted: true
                onClicked: wizardDialog.finishAndCreateProfile()
            }
        }
    }
}
