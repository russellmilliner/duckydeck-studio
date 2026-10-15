import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Item {
    id: canvasRoot
    implicitWidth: 520
    implicitHeight: 450
    clip: true

    property var controller: null
    property int selectedSlot: 1
    property bool isRotated: false
    property bool showExpansion: false
    property bool oledScriptMode: false

    property string enc1CcwLabel: "Size -"
    property string enc1CwLabel: "Size +"
    property string enc1PushLabel: "Invert"
    property string enc2CcwLabel: "Zoom -"
    property string enc2CwLabel: "Zoom +"
    property string enc2PushLabel: "Frame"

    signal slotSelected(int slotNumber, string slotTitle, color slotColor)

    property var profileLabels: []
    property var profileNumbers: []
    property var expansionLabels: ({})

    property ListModel keyModel: ListModel {
        Component.onCompleted: {
            for (let i = 1; i <= 20; ++i) {
                append({
                    slot: i,
                    l1: "",
                    l2: "",
                    r: 0,
                    g: 170,
                    b: 255,
                    hasRel: false,
                    hasLoop: false
                })
            }
            canvasRoot.syncProfilesFromController()
            canvasRoot.syncSlotsFromController()
        }
    }

    onControllerChanged: {
        syncProfilesFromController()
        syncSlotsFromController()
    }

    Connections {
        target: canvasRoot.controller
        function onActiveProfileSlotsJsonChanged() {
            canvasRoot.syncSlotsFromController()
        }
        function onProfilesJsonChanged() {
            canvasRoot.syncProfilesFromController()
        }
        function onActiveProfileChanged() {
            canvasRoot.syncActiveProfileSelection()
            oledSimulator.requestPaint()
        }
        function onActiveProfileNameChanged() {
            oledSimulator.requestPaint()
        }
    }

    function syncActiveProfileSelection() {
        if (!controller || !profileNumbers || profileNumbers.length === 0) return
        const idx = profileNumbers.indexOf(controller.activeProfile)
        if (idx >= 0 && profileCombo.currentIndex !== idx) {
            profileCombo.currentIndex = idx
        }
    }

    function syncProfilesFromController() {
        if (!controller || !controller.profilesJson) return
        try {
            const arr = JSON.parse(controller.profilesJson)
            if (!Array.isArray(arr)) return
            const labels = []
            const nums = []
            let activeIdx = 0
            for (let i = 0; i < arr.length; ++i) {
                const p = arr[i]
                nums.push(p.number)
                labels.push("P" + p.number + ": " + p.name)
                if (p.number === controller.activeProfile) {
                    activeIdx = i
                }
            }
            canvasRoot.profileNumbers = nums
            canvasRoot.profileLabels = labels
            profileCombo.currentIndex = activeIdx
        } catch (e) {}
    }

    function formatEncoderLabel(item, fallback) {
        const l1 = (item.l1 || "").trim()
        const l2 = (item.l2 || "").trim()
        const combined = (l1 + (l2.length > 0 ? " " + l2 : "")).trim()
        return combined.length > 0 ? combined : fallback
    }

    function syncSlotsFromController() {
        if (!controller || !controller.activeProfileSlotsJson || keyModel.count < 20) return
        try {
            const arr = JSON.parse(controller.activeProfileSlotsJson)
            if (!Array.isArray(arr)) return
            const expMap = {}
            for (let i = 0; i < arr.length; ++i) {
                const item = arr[i]
                const s = item.slot
                if (s >= 1 && s <= 20) {
                    keyModel.setProperty(s - 1, "l1", item.l1 || "")
                    keyModel.setProperty(s - 1, "l2", item.l2 || "")
                    keyModel.setProperty(s - 1, "r", item.r !== undefined ? item.r : 0)
                    keyModel.setProperty(s - 1, "g", item.g !== undefined ? item.g : 170)
                    keyModel.setProperty(s - 1, "b", item.b !== undefined ? item.b : 255)
                    keyModel.setProperty(s - 1, "hasRel", !!item.hasRel)
                    keyModel.setProperty(s - 1, "hasLoop", !!item.hasLoop)
                } else if (s === 21) {
                    canvasRoot.enc1CcwLabel = formatEncoderLabel(item, "CCW")
                } else if (s === 22) {
                    canvasRoot.enc1CwLabel = formatEncoderLabel(item, "CW")
                } else if (s === 23) {
                    canvasRoot.enc1PushLabel = formatEncoderLabel(item, "Push")
                } else if (s === 24) {
                    canvasRoot.enc2CcwLabel = formatEncoderLabel(item, "CCW")
                } else if (s === 25) {
                    canvasRoot.enc2CwLabel = formatEncoderLabel(item, "CW")
                } else if (s === 26) {
                    canvasRoot.enc2PushLabel = formatEncoderLabel(item, "Push")
                } else if (s >= 27 && s <= 58) {
                    const lbl = ((item.l1 || "") + (item.l2 ? " " + item.l2 : "")).trim()
                    if (lbl.length > 0 || item.hasScript) {
                        expMap[s] = lbl.length > 0 ? lbl : "Active"
                    }
                }
            }
            canvasRoot.expansionLabels = expMap
            oledSimulator.requestPaint()
        } catch (e) {}
    }

    function slotColorFor(slotNumber) {
        if (slotNumber >= 1 && slotNumber <= 20 && keyModel.count >= slotNumber) {
            const item = keyModel.get(slotNumber - 1)
            return Qt.rgba(item.r / 255.0, item.g / 255.0, item.b / 255.0, 1.0)
        }
        return Kirigami.Theme.highlightColor
    }

    function slotTitleFor(slotNumber) {
        if (slotNumber >= 1 && slotNumber <= 20 && keyModel.count >= slotNumber) {
            const item = keyModel.get(slotNumber - 1)
            const label = ((item.l1 || "") + " " + (item.l2 || "")).trim()
            return label.length > 0
                ? ("Key " + slotNumber + " (" + label + ")")
                : ("Key " + slotNumber + " (Unassigned)")
        } else if (slotNumber === 21) {
            return "Encoder 1 CCW — " + canvasRoot.enc1CcwLabel + " (Slot 21)"
        } else if (slotNumber === 22) {
            return "Encoder 1 CW — " + canvasRoot.enc1CwLabel + " (Slot 22)"
        } else if (slotNumber === 23) {
            return "Encoder 1 Push — " + canvasRoot.enc1PushLabel + " (Slot 23)"
        } else if (slotNumber === 24) {
            return "Encoder 2 CCW — " + canvasRoot.enc2CcwLabel + " (Slot 24)"
        } else if (slotNumber === 25) {
            return "Encoder 2 CW — " + canvasRoot.enc2CwLabel + " (Slot 25)"
        } else if (slotNumber === 26) {
            return "Encoder 2 Push — " + canvasRoot.enc2PushLabel + " (Slot 26)"
        } else {
            const idx = slotNumber - 27
            const modNum = Math.floor(idx / 8) + 1
            const chNum = (idx % 8) + 1
            const expLbl = canvasRoot.expansionLabels[slotNumber] || ""
            return expLbl.length > 0
                ? ("Expansion M" + modNum + "·C" + chNum + " (" + expLbl + " • Slot " + slotNumber + ")")
                : ("Expansion M" + modNum + "·C" + chNum + " (Slot " + slotNumber + ")")
        }
    }

    function updateSelectedKeyRgb(r, g, b) {
        if (selectedSlot >= 1 && selectedSlot <= 20) {
            keyModel.setProperty(selectedSlot - 1, "r", r)
            keyModel.setProperty(selectedSlot - 1, "g", g)
            keyModel.setProperty(selectedSlot - 1, "b", b)
            if (controller) {
                controller.setKeyRgb(selectedSlot - 1, r, g, b)
            }
            oledSimulator.requestPaint()
        }
    }

    ColorDialog {
        id: rgbPickerDialog
        title: qsTr("Select North-Facing ARGB LED Color for Key %1").arg(canvasRoot.selectedSlot)
        onAccepted: {
            const c = selectedColor
            canvasRoot.updateSelectedKeyRgb(
                Math.round(c.r * 255),
                Math.round(c.g * 255),
                Math.round(c.b * 255)
            )
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: Kirigami.Units.smallSpacing

        RowLayout {
            Layout.fillWidth: true
            spacing: 4

            QQC2.ComboBox {
                id: profileCombo
                Layout.preferredWidth: 185
                Layout.fillWidth: true
                Layout.maximumWidth: 230
                model: canvasRoot.profileLabels
                onActivated: (index) => {
                    if (index >= 0 && index < canvasRoot.profileNumbers.length && canvasRoot.controller) {
                        const profNum = canvasRoot.profileNumbers[index]
                        canvasRoot.controller.selectProfile(profNum)
                        canvasRoot.slotSelected(
                            canvasRoot.selectedSlot,
                            canvasRoot.slotTitleFor(canvasRoot.selectedSlot),
                            canvasRoot.slotColorFor(canvasRoot.selectedSlot)
                        )
                    }
                }
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: canvasRoot.controller && canvasRoot.controller.sdMountPath.length > 0
                    ? qsTr("SD Path: %1 (%2 profiles)").arg(canvasRoot.controller.sdMountPath).arg(canvasRoot.profileNumbers.length)
                    : qsTr("Active Profile (Load SD Card to view all SD profiles)")
            }

            QQC2.ButtonGroup { id: modeGroup }

            QQC2.ToolButton {
                QQC2.ButtonGroup.group: modeGroup
                checkable: true
                checked: !canvasRoot.showExpansion
                icon.name: "input-keyboard"
                text: qsTr("Onboard")
                display: QQC2.AbstractButton.TextBesideIcon
                onClicked: canvasRoot.showExpansion = false
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Onboard (20 Keys + 2 Encoders)")
            }

            QQC2.ToolButton {
                QQC2.ButtonGroup.group: modeGroup
                checkable: true
                checked: canvasRoot.showExpansion
                icon.name: "network-wired"
                text: qsTr("Expansion")
                display: QQC2.AbstractButton.TextBesideIcon
                onClicked: canvasRoot.showExpansion = true
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Daisy-Chained Expansion Channels (27–58)")
            }

            Item { Layout.fillWidth: true }

            QQC2.ToolButton {
                icon.name: "color-picker"
                text: qsTr("RGB")
                display: QQC2.AbstractButton.TextBesideIcon
                enabled: canvasRoot.selectedSlot >= 1 && canvasRoot.selectedSlot <= 20
                onClicked: rgbPickerDialog.open()
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: qsTr("Set custom ARGB LED color for Key %1 (SWCOLOR)").arg(canvasRoot.selectedSlot)
            }

            QQC2.ToolButton {
                checkable: true
                checked: canvasRoot.oledScriptMode
                icon.name: "video-television"
                display: QQC2.AbstractButton.TextBesideIcon
                text: canvasRoot.oledScriptMode ? qsTr("OLED VM") : qsTr("OLED Grid")
                onToggled: {
                    canvasRoot.oledScriptMode = checked
                    oledSimulator.requestPaint()
                }
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: canvasRoot.oledScriptMode ? qsTr("Switch to Profile Grid HUD") : qsTr("Switch to duckStack VM HUD")
            }
        }

        Rectangle {
            id: pcbChassis
            Kirigami.Theme.colorSet: Kirigami.Theme.View
            Kirigami.Theme.inherit: false
            readonly property color frameBorderColor: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
            readonly property real uiScale: Math.max(
                0.85,
                Math.min(
                    (pcbChassis.width - 32) / 430.0,
                    (pcbChassis.height - 32) / 360.0
                )
            )

            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: 12
            color: Kirigami.Theme.alternateBackgroundColor
            border.color: pcbChassis.frameBorderColor
            border.width: 1
            clip: true

            Rectangle {
                anchors.fill: parent
                anchors.margins: 5
                radius: 10
                color: "transparent"
                border.color: pcbChassis.frameBorderColor
                opacity: 0.5
                border.width: 1
            }

            Row {
                anchors.centerIn: parent
                spacing: 12
                scale: pcbChassis.uiScale
                transformOrigin: Item.Center
                visible: !canvasRoot.showExpansion

                Grid {
                    id: switchMatrix
                    columns: 4
                    rowSpacing: 6
                    columnSpacing: 6

                    Repeater {
                        model: canvasRoot.keyModel

                        delegate: Item {
                            id: keycapItem
                            width: 66
                            height: 66
                            required property int index
                            required property int slot
                            required property string l1
                            required property string l2
                            required property int r
                            required property int g
                            required property int b
                            required property bool hasRel
                            required property bool hasLoop

                            readonly property bool isSelected: canvasRoot.selectedSlot === slot
                            readonly property color ledColor: Qt.rgba(r / 255.0, g / 255.0, b / 255.0, 1.0)

                            Rectangle {
                                anchors.fill: parent
                                radius: 8
                                color: Kirigami.Theme.backgroundColor
                                border.width: keycapItem.isSelected ? 2 : 1
                                border.color: keycapItem.isSelected
                                    ? Kirigami.Theme.highlightColor
                                    : pcbChassis.frameBorderColor

                                Rectangle {
                                    anchors.top: parent.top
                                    anchors.topMargin: 3
                                    anchors.horizontalCenter: parent.horizontalCenter
                                    width: 28
                                    height: 4
                                    radius: 2
                                    color: keycapItem.ledColor
                                }

                                Rectangle {
                                    anchors.fill: parent
                                    anchors.margins: 5
                                    anchors.topMargin: 9
                                    radius: 6
                                    color: keyMouse.pressed
                                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(keycapItem.ledColor.r, keycapItem.ledColor.g, keycapItem.ledColor.b, 0.35))
                                        : (keycapItem.isSelected
                                            ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.18))
                                            : Kirigami.Theme.alternateBackgroundColor)
                                    border.width: 1
                                    border.color: keycapItem.isSelected
                                        ? Kirigami.Theme.highlightColor
                                        : Qt.rgba(r / 255.0, g / 255.0, b / 255.0, 0.5)

                                    QQC2.Label {
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.margins: 3
                                        text: keycapItem.slot
                                        font.pixelSize: 9
                                        font.bold: true
                                        color: Kirigami.Theme.disabledTextColor
                                    }

                                    Row {
                                        anchors.top: parent.top
                                        anchors.right: parent.right
                                        anchors.margins: 3
                                        spacing: 2

                                        QQC2.Label {
                                            visible: keycapItem.hasRel
                                            text: "↑"
                                            font.pixelSize: 10
                                            font.bold: true
                                            color: Kirigami.Theme.highlightColor
                                        }
                                        QQC2.Label {
                                            visible: keycapItem.hasLoop
                                            text: "∞"
                                            font.pixelSize: 11
                                            font.bold: true
                                            color: Kirigami.Theme.neutralTextColor
                                        }
                                    }

                                    Column {
                                        anchors.centerIn: parent
                                        anchors.verticalCenterOffset: 3
                                        spacing: 1

                                        QQC2.Label {
                                            anchors.horizontalCenter: parent.horizontalCenter
                                            text: keycapItem.l1
                                            font.pixelSize: 12
                                            font.bold: true
                                            color: Kirigami.Theme.textColor
                                        }
                                        QQC2.Label {
                                            anchors.horizontalCenter: parent.horizontalCenter
                                            text: keycapItem.l2
                                            font.pixelSize: 10
                                            font.weight: Font.Medium
                                            color: Kirigami.Theme.disabledTextColor
                                        }
                                    }
                                }
                            }

                            MouseArea {
                                id: keyMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                acceptedButtons: Qt.LeftButton | Qt.RightButton
                                onClicked: (mouse) => {
                                    canvasRoot.selectedSlot = keycapItem.slot
                                    canvasRoot.slotSelected(
                                        keycapItem.slot,
                                        canvasRoot.slotTitleFor(keycapItem.slot),
                                        keycapItem.ledColor
                                    )
                                    oledSimulator.requestPaint()

                                    if (mouse.button === Qt.RightButton) {
                                        rgbPickerDialog.open()
                                    }
                                }
                            }
                        }
                    }
                }

                Column {
                    width: 136
                    spacing: 6
                    anchors.verticalCenter: parent.verticalCenter

                    RotaryEncoderWidget {
                        encoderTitle: qsTr("Encoder 1 (Top)")
                        ccwSlot: 21
                        cwSlot: 22
                        pushSlot: 23
                        ccwLabel: canvasRoot.enc1CcwLabel
                        cwLabel: canvasRoot.enc1CwLabel
                        pushLabel: canvasRoot.enc1PushLabel
                        selectedSlot: canvasRoot.selectedSlot
                        onActionTriggered: (slot, desc) => {
                            canvasRoot.selectedSlot = slot
                            canvasRoot.slotSelected(slot, canvasRoot.slotTitleFor(slot), Kirigami.Theme.highlightColor)
                            oledSimulator.requestPaint()
                        }
                    }

                    RotaryEncoderWidget {
                        encoderTitle: qsTr("Encoder 2 (Mid)")
                        ccwSlot: 24
                        cwSlot: 25
                        pushSlot: 26
                        ccwLabel: canvasRoot.enc2CcwLabel
                        cwLabel: canvasRoot.enc2CwLabel
                        pushLabel: canvasRoot.enc2PushLabel
                        selectedSlot: canvasRoot.selectedSlot
                        onActionTriggered: (slot, desc) => {
                            canvasRoot.selectedSlot = slot
                            canvasRoot.slotSelected(slot, canvasRoot.slotTitleFor(slot), Kirigami.Theme.highlightColor)
                            oledSimulator.requestPaint()
                        }
                    }

                    Rectangle {
                        anchors.horizontalCenter: parent.horizontalCenter
                        width: 134
                        height: 134
                        radius: 8
                        color: "#000000" // Keep OLED simulator true black
                        border.color: pcbChassis.frameBorderColor
                        border.width: 2

                        Canvas {
                            id: oledSimulator
                            width: 128
                            height: 128
                            anchors.centerIn: parent
                            scale: 1.0
                            smooth: false

                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.fillStyle = "#000000"
                                ctx.fillRect(0, 0, 128, 128)

                                if (!canvasRoot.oledScriptMode) {
                                    ctx.fillStyle = "#38bdf8"
                                    ctx.fillRect(0, 0, 128, 13)
                                    ctx.fillStyle = "#020617"
                                    ctx.font = "bold 9px sans-serif"
                                    const profNum = canvasRoot.controller ? canvasRoot.controller.activeProfile : 1
                                    const profName = (canvasRoot.controller && canvasRoot.controller.activeProfileName)
                                        ? canvasRoot.controller.activeProfileName
                                        : "Default"
                                    ctx.fillText("P" + profNum + ": " + profName.slice(0, 16), 3, 10)

                                    const cellW = 32
                                    const cellH = 23
                                    ctx.font = "bold 8px monospace"

                                    for (let i = 0; i < canvasRoot.keyModel.count; ++i) {
                                        const item = canvasRoot.keyModel.get(i)
                                        const col = i % 4
                                        const row = Math.floor(i / 4)
                                        const x = col * cellW
                                        const y = 13 + row * cellH
                                        const isSel = (canvasRoot.selectedSlot === item.slot)

                                        if (isSel) {
                                            ctx.fillStyle = "#38bdf8"
                                            ctx.fillRect(x + 1, y + 1, cellW - 2, cellH - 2)
                                            ctx.fillStyle = "#020617"
                                        } else {
                                            ctx.strokeStyle = "#334155"
                                            ctx.strokeRect(x, y, cellW, cellH)
                                            ctx.fillStyle = "#f8fafc"
                                        }

                                        ctx.fillText(item.l1.slice(0, 5), x + 2, y + 9)
                                        ctx.fillText(item.l2.slice(0, 5), x + 2, y + 19)
                                    }
                                } else {
                                    ctx.strokeStyle = "#38bdf8"
                                    ctx.lineWidth = 1
                                    ctx.strokeRect(2, 2, 124, 124)

                                    ctx.font = "bold 10px sans-serif"
                                    ctx.fillStyle = "#38bdf8"
                                    ctx.fillText("duckStack VM HUD", 14, 18)

                                    ctx.beginPath()
                                    ctx.moveTo(8, 24)
                                    ctx.lineTo(120, 24)
                                    ctx.stroke()

                                    ctx.beginPath()
                                    ctx.arc(64, 68, 20, 0, Math.PI * 2)
                                    ctx.stroke()

                                    ctx.font = "bold 10px monospace"
                                    ctx.fillStyle = "#ffffff"
                                    ctx.fillText("Slot #" + canvasRoot.selectedSlot, 36, 72)
                                    ctx.fillStyle = "#94a3b8"
                                    ctx.fillText("128x128 OLED", 28, 114)
                                }
                            }
                        }
                    }

                    Row {
                        anchors.horizontalCenter: parent.horizontalCenter
                        spacing: 6

                        QQC2.Button {
                            text: "− Profile"
                            implicitWidth: 64
                            implicitHeight: 26
                            font.pixelSize: 10
                            onClicked: {
                                if (canvasRoot.controller) {
                                    if (canvasRoot.profileNumbers.length > 1 && profileCombo.currentIndex > 0) {
                                        profileCombo.currentIndex -= 1
                                        const profNum = canvasRoot.profileNumbers[profileCombo.currentIndex]
                                        canvasRoot.controller.selectProfile(profNum)
                                    } else {
                                        const next = Math.max(1, canvasRoot.controller.activeProfile - 1)
                                        canvasRoot.controller.selectProfile(next)
                                    }
                                    canvasRoot.slotSelected(
                                        canvasRoot.selectedSlot,
                                        canvasRoot.slotTitleFor(canvasRoot.selectedSlot),
                                        canvasRoot.slotColorFor(canvasRoot.selectedSlot)
                                    )
                                }
                            }
                        }

                        QQC2.Button {
                            text: "+ Profile"
                            implicitWidth: 64
                            implicitHeight: 26
                            font.pixelSize: 10
                            onClicked: {
                                if (canvasRoot.controller) {
                                    if (canvasRoot.profileNumbers.length > 1 && profileCombo.currentIndex + 1 < canvasRoot.profileNumbers.length) {
                                        profileCombo.currentIndex += 1
                                        const profNum = canvasRoot.profileNumbers[profileCombo.currentIndex]
                                        canvasRoot.controller.selectProfile(profNum)
                                    } else {
                                        const next = Math.min(64, canvasRoot.controller.activeProfile + 1)
                                        canvasRoot.controller.selectProfile(next)
                                    }
                                    canvasRoot.slotSelected(
                                        canvasRoot.selectedSlot,
                                        canvasRoot.slotTitleFor(canvasRoot.selectedSlot),
                                        canvasRoot.slotColorFor(canvasRoot.selectedSlot)
                                    )
                                }
                            }
                        }
                    }
                }
            }

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 14
                spacing: 8
                visible: canvasRoot.showExpansion

                QQC2.Label {
                    text: qsTr("Expansion Modules (4 Modules × 8 Channels)")
                    font.bold: true
                    font.pixelSize: 12
                    color: Kirigami.Theme.textColor
                }

                GridLayout {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    columns: 4
                    rowSpacing: 6
                    columnSpacing: 6

                    Repeater {
                        model: 32
                        delegate: Rectangle {
                            required property int index
                            readonly property int slotNum: 27 + index
                            readonly property int moduleNum: Math.floor(index / 8) + 1
                            readonly property int chanNum: (index % 8) + 1
                            readonly property bool isSel: canvasRoot.selectedSlot === slotNum
                            readonly property string expLabel: canvasRoot.expansionLabels[slotNum] || ""

                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 6
                            color: isSel
                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                                : Kirigami.Theme.backgroundColor
                            border.width: isSel || expLabel.length > 0 ? 2 : 1
                            border.color: isSel
                                ? Kirigami.Theme.highlightColor
                                : (expLabel.length > 0 ? Qt.alpha(Kirigami.Theme.highlightColor, 0.55) : pcbChassis.frameBorderColor)

                            Column {
                                anchors.centerIn: parent
                                spacing: 1

                                QQC2.Label {
                                    anchors.horizontalCenter: parent.horizontalCenter
                                    text: expLabel.length > 0
                                        ? ("M" + moduleNum + "·C" + chanNum + ": " + expLabel)
                                        : ("M" + moduleNum + "·C" + chanNum)
                                    font.bold: true
                                    font.pixelSize: 10
                                    color: Kirigami.Theme.textColor
                                }
                                QQC2.Label {
                                    anchors.horizontalCenter: parent.horizontalCenter
                                    text: "Slot " + slotNum
                                    font.pixelSize: 9
                                    color: Kirigami.Theme.disabledTextColor
                                }
                            }

                            MouseArea {
                                anchors.fill: parent
                                onClicked: {
                                    canvasRoot.selectedSlot = slotNum
                                    canvasRoot.slotSelected(
                                        slotNum,
                                        canvasRoot.slotTitleFor(slotNum),
                                        Kirigami.Theme.highlightColor
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    component RotaryEncoderWidget: Item {
        id: encRoot
        implicitWidth: 136
        implicitHeight: 68

        property string encoderTitle: "Encoder"
        property int ccwSlot: 21
        property int cwSlot: 22
        property int pushSlot: 23
        property string ccwLabel: "CCW"
        property string cwLabel: "CW"
        property string pushLabel: "Push"
        property int selectedSlot: 1
        property real visualAngle: 0
        readonly property color frameBorderColor: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

        signal actionTriggered(int slot, string description)

        Column {
            anchors.centerIn: parent
            spacing: 3

            QQC2.Label {
                anchors.horizontalCenter: parent.horizontalCenter
                text: encRoot.encoderTitle
                font.pixelSize: 11
                font.bold: true
                color: Kirigami.Theme.textColor
            }

            Row {
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: 5

                Rectangle {
                    width: 35
                    height: 48
                    radius: 6
                    color: encRoot.selectedSlot === encRoot.ccwSlot
                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                        : Kirigami.Theme.backgroundColor
                    border.width: encRoot.selectedSlot === encRoot.ccwSlot ? 2 : 1
                    border.color: encRoot.selectedSlot === encRoot.ccwSlot
                        ? Kirigami.Theme.highlightColor
                        : encRoot.frameBorderColor

                    Column {
                        anchors.centerIn: parent
                        spacing: 1
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "⟲"
                            font.pixelSize: 16
                            color: Kirigami.Theme.highlightColor
                        }
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: encRoot.ccwLabel
                            font.pixelSize: 9
                            font.bold: true
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            encRoot.visualAngle -= 15
                            encRoot.actionTriggered(
                                encRoot.ccwSlot,
                                encRoot.encoderTitle + " Counter-Clockwise (Slot " + encRoot.ccwSlot + ")"
                            )
                        }
                    }
                }

                Rectangle {
                    id: knobBody
                    width: 48
                    height: 48
                    radius: 24
                    color: encRoot.selectedSlot === encRoot.pushSlot
                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                        : Kirigami.Theme.backgroundColor
                    border.width: encRoot.selectedSlot === encRoot.pushSlot ? 2 : 1.5
                    border.color: encRoot.selectedSlot === encRoot.pushSlot
                        ? Kirigami.Theme.highlightColor
                        : encRoot.frameBorderColor

                    Item {
                        anchors.fill: parent
                        rotation: encRoot.visualAngle
                        Behavior on rotation { NumberAnimation { duration: 90 } }

                        Rectangle {
                            anchors.top: parent.top
                            anchors.topMargin: 4
                            anchors.horizontalCenter: parent.horizontalCenter
                            width: 3
                            height: 7
                            radius: 1.5
                            color: Kirigami.Theme.highlightColor
                        }
                    }

                    Column {
                        anchors.centerIn: parent
                        spacing: 0
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "PUSH"
                            font.pixelSize: 8
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: encRoot.pushLabel
                            font.pixelSize: 9
                            font.bold: true
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            encRoot.actionTriggered(
                                encRoot.pushSlot,
                                encRoot.encoderTitle + " Center Push (Slot " + encRoot.pushSlot + ")"
                            )
                        }
                        onWheel: (wheel) => {
                            if (wheel.angleDelta.y > 0) {
                                encRoot.visualAngle += 15
                                encRoot.actionTriggered(
                                    encRoot.cwSlot,
                                    encRoot.encoderTitle + " Clockwise (Slot " + encRoot.cwSlot + ")"
                                )
                            } else {
                                encRoot.visualAngle -= 15
                                encRoot.actionTriggered(
                                    encRoot.ccwSlot,
                                    encRoot.encoderTitle + " Counter-Clockwise (Slot " + encRoot.ccwSlot + ")"
                                )
                            }
                        }
                    }
                }

                Rectangle {
                    width: 35
                    height: 48
                    radius: 6
                    color: encRoot.selectedSlot === encRoot.cwSlot
                        ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.2))
                        : Kirigami.Theme.backgroundColor
                    border.width: encRoot.selectedSlot === encRoot.cwSlot ? 2 : 1
                    border.color: encRoot.selectedSlot === encRoot.cwSlot
                        ? Kirigami.Theme.highlightColor
                        : encRoot.frameBorderColor

                    Column {
                        anchors.centerIn: parent
                        spacing: 1
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "⟳"
                            font.pixelSize: 16
                            color: Kirigami.Theme.highlightColor
                        }
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: encRoot.cwLabel
                            font.pixelSize: 9
                            font.bold: true
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            encRoot.visualAngle += 15
                            encRoot.actionTriggered(
                                encRoot.cwSlot,
                                encRoot.encoderTitle + " Clockwise (Slot " + encRoot.cwSlot + ")"
                            )
                        }
                    }
                }
            }
        }
    }
}
