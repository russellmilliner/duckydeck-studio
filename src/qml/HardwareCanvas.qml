import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Item {
    id: canvasRoot
    implicitWidth: 476
    implicitHeight: 450
    width: 476
    clip: true

    property var controller: null
    property int selectedSlot: 1
    property bool isRotated: false
    property bool showExpansion: false
    property bool oledScriptMode: false

    signal slotSelected(int slotNumber, string slotTitle, color slotColor)

    property ListModel keyModel: ListModel {
        Component.onCompleted: {
            const defaults = [
                { slot: 1,  l1: "Front", l2: "View",   r: 0,   g: 170, b: 255, hasRel: false, hasLoop: false },
                { slot: 2,  l1: "Side",  l2: "View",   r: 0,   g: 170, b: 255, hasRel: false, hasLoop: false },
                { slot: 3,  l1: "Top",   l2: "View",   r: 0,   g: 170, b: 255, hasRel: false, hasLoop: false },
                { slot: 4,  l1: "Wire",  l2: "Frame",  r: 0,   g: 220, b: 200, hasRel: true,  hasLoop: false },
                { slot: 5,  l1: "Grab",  l2: "Brush",  r: 40,  g: 220, b: 100, hasRel: false, hasLoop: false },
                { slot: 6,  l1: "Clay",  l2: "Strips", r: 40,  g: 220, b: 100, hasRel: false, hasLoop: false },
                { slot: 7,  l1: "Crease",l2: "Tool",   r: 40,  g: 220, b: 100, hasRel: false, hasLoop: false },
                { slot: 8,  l1: "Smooth",l2: "Hold",   r: 40,  g: 220, b: 100, hasRel: true,  hasLoop: false },
                { slot: 9,  l1: "Draw",  l2: "Sharp",  r: 40,  g: 220, b: 100, hasRel: false, hasLoop: false },
                { slot: 10, l1: "Inflat",l2: "Brush",  r: 40,  g: 220, b: 100, hasRel: false, hasLoop: false },
                { slot: 11, l1: "Pinch", l2: "Edge",   r: 40,  g: 220, b: 100, hasRel: false, hasLoop: false },
                { slot: 12, l1: "Mask",  l2: "Lasso",  r: 255, g: 180, b: 0,   hasRel: false, hasLoop: false },
                { slot: 13, l1: "DynTop",l2: "Toggle", r: 255, g: 140, b: 0,   hasRel: false, hasLoop: false },
                { slot: 14, l1: "Voxel", l2: "Remesh", r: 255, g: 140, b: 0,   hasRel: false, hasLoop: false },
                { slot: 15, l1: "Invert",l2: "Mask",   r: 255, g: 180, b: 0,   hasRel: false, hasLoop: false },
                { slot: 16, l1: "Clear", l2: "Mask",   r: 255, g: 180, b: 0,   hasRel: false, hasLoop: false },
                { slot: 17, l1: "Undo",  l2: "Step",   r: 255, g: 60,  b: 60,  hasRel: false, hasLoop: false },
                { slot: 18, l1: "Redo",  l2: "Step",   r: 255, g: 60,  b: 60,  hasRel: false, hasLoop: false },
                { slot: 19, l1: "Auto",  l2: "Click",  r: 180, g: 80,  b: 255, hasRel: false, hasLoop: true  },
                { slot: 20, l1: "Save",  l2: "Incr+",  r: 180, g: 80,  b: 255, hasRel: false, hasLoop: false }
            ]
            for (let i = 0; i < defaults.length; ++i) {
                append(defaults[i])
            }
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
            Layout.alignment: Qt.AlignHCenter
            Layout.preferredWidth: 456
            spacing: 4

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
                QQC2.ToolTip.text: qsTr("Set custom ARGB LED color for Key %1 (SWC_SET)").arg(canvasRoot.selectedSlot)
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
            width: 456
            height: 396
            Layout.preferredWidth: 456
            Layout.preferredHeight: 396
            Layout.alignment: Qt.AlignHCenter
            Layout.fillWidth: false
            Layout.fillHeight: true
            radius: 14
            color: "#181b22"
            border.color: "#2a313d"
            border.width: 2
            clip: true

            Rectangle {
                anchors.fill: parent
                anchors.margins: 5
                radius: 11
                color: "transparent"
                border.color: "#18ffffff"
                border.width: 1
            }

            Row {
                anchors.centerIn: parent
                spacing: 12
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
                                color: "#0f1115"
                                border.width: keycapItem.isSelected ? 2 : 1
                                border.color: keycapItem.isSelected ? "#38bdf8" : "#2a313d"

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
                                    gradient: Gradient {
                                        GradientStop {
                                            position: 0.0
                                            color: keyMouse.pressed
                                                ? Qt.darker(keycapItem.ledColor, 2.2)
                                                : (keycapItem.isSelected ? "#223045" : "#242832")
                                        }
                                        GradientStop {
                                            position: 1.0
                                            color: keycapItem.isSelected ? "#16202f" : "#191c23"
                                        }
                                    }
                                    border.width: 1
                                    border.color: keycapItem.isSelected
                                        ? "#38bdf8"
                                        : Qt.rgba(r / 255.0, g / 255.0, b / 255.0, 0.45)

                                    QQC2.Label {
                                        anchors.top: parent.top
                                        anchors.left: parent.left
                                        anchors.margins: 3
                                        text: keycapItem.slot
                                        font.pixelSize: 9
                                        font.bold: true
                                        color: "#cbd5e1"
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
                                            color: "#38bdf8"
                                        }
                                        QQC2.Label {
                                            visible: keycapItem.hasLoop
                                            text: "∞"
                                            font.pixelSize: 11
                                            font.bold: true
                                            color: "#c084fc"
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
                                            color: "#ffffff"
                                        }
                                        QQC2.Label {
                                            anchors.horizontalCenter: parent.horizontalCenter
                                            text: keycapItem.l2
                                            font.pixelSize: 10
                                            font.weight: Font.Medium
                                            color: "#93c5fd"
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
                                        "Key " + keycapItem.slot + " (" + keycapItem.l1 + " " + keycapItem.l2 + ")",
                                        keycapItem.ledColor
                                    )
                                    oledSimulator.requestPaint()

                                    if (mouse.button === Qt.RightButton) {
                                        rgbPickerDialog.open()
                                    } else if (canvasRoot.controller) {
                                        canvasRoot.controller.setKeyRgb(
                                            keycapItem.slot - 1,
                                            keycapItem.r,
                                            keycapItem.g,
                                            keycapItem.b
                                        )
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
                        ccwLabel: "Size -"
                        cwLabel: "Size +"
                        pushLabel: "Invert"
                        selectedSlot: canvasRoot.selectedSlot
                        onActionTriggered: (slot, desc) => {
                            canvasRoot.selectedSlot = slot
                            canvasRoot.slotSelected(slot, desc, "#38bdf8")
                            oledSimulator.requestPaint()
                        }
                    }

                    RotaryEncoderWidget {
                        encoderTitle: qsTr("Encoder 2 (Mid)")
                        ccwSlot: 24
                        cwSlot: 25
                        pushSlot: 26
                        ccwLabel: "Zoom -"
                        cwLabel: "Zoom +"
                        pushLabel: "Frame"
                        selectedSlot: canvasRoot.selectedSlot
                        onActionTriggered: (slot, desc) => {
                            canvasRoot.selectedSlot = slot
                            canvasRoot.slotSelected(slot, desc, "#38bdf8")
                            oledSimulator.requestPaint()
                        }
                    }

                    Rectangle {
                        anchors.horizontalCenter: parent.horizontalCenter
                        width: 134
                        height: 134
                        radius: 8
                        color: "#050505"
                        border.color: "#334155"
                        border.width: 1.5

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
                                    ctx.fillText("P" + profNum + ": Blender Sculpt", 3, 10)

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
                                    const next = Math.max(1, canvasRoot.controller.activeProfile - 1)
                                    canvasRoot.controller.switchProfile(next)
                                    oledSimulator.requestPaint()
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
                                    const next = Math.min(64, canvasRoot.controller.activeProfile + 1)
                                    canvasRoot.controller.switchProfile(next)
                                    oledSimulator.requestPaint()
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
                    color: "#e2e8f0"
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

                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 6
                            color: isSel ? "#1e3a5f" : "#1e222b"
                            border.width: isSel ? 2 : 1
                            border.color: isSel ? "#38bdf8" : "#334155"

                            Column {
                                anchors.centerIn: parent
                                spacing: 1

                                QQC2.Label {
                                    anchors.horizontalCenter: parent.horizontalCenter
                                    text: "M" + moduleNum + "·C" + chanNum
                                    font.bold: true
                                    font.pixelSize: 10
                                    color: "#f8fafc"
                                }
                                QQC2.Label {
                                    anchors.horizontalCenter: parent.horizontalCenter
                                    text: "Slot " + slotNum
                                    font.pixelSize: 9
                                    color: "#94a3b8"
                                }
                            }

                            MouseArea {
                                anchors.fill: parent
                                onClicked: {
                                    canvasRoot.selectedSlot = slotNum
                                    canvasRoot.slotSelected(
                                        slotNum,
                                        "Expansion Module " + moduleNum + " Channel " + chanNum + " (Slot " + slotNum + ")",
                                        "#38bdf8"
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

        signal actionTriggered(int slot, string description)

        Column {
            anchors.centerIn: parent
            spacing: 3

            QQC2.Label {
                anchors.horizontalCenter: parent.horizontalCenter
                text: encRoot.encoderTitle
                font.pixelSize: 11
                font.bold: true
                color: "#f1f5f9"
            }

            Row {
                anchors.horizontalCenter: parent.horizontalCenter
                spacing: 5

                Rectangle {
                    width: 35
                    height: 48
                    radius: 6
                    color: encRoot.selectedSlot === encRoot.ccwSlot ? "#1e3a5f" : "#1f242d"
                    border.width: encRoot.selectedSlot === encRoot.ccwSlot ? 2 : 1
                    border.color: encRoot.selectedSlot === encRoot.ccwSlot ? "#38bdf8" : "#334155"

                    Column {
                        anchors.centerIn: parent
                        spacing: 1
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "⟲"
                            font.pixelSize: 16
                            color: "#38bdf8"
                        }
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: encRoot.ccwLabel
                            font.pixelSize: 9
                            font.bold: true
                            color: "#f1f5f9"
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
                    color: encRoot.selectedSlot === encRoot.pushSlot ? "#1e3a5f" : "#262b36"
                    border.width: encRoot.selectedSlot === encRoot.pushSlot ? 2 : 1.5
                    border.color: encRoot.selectedSlot === encRoot.pushSlot ? "#38bdf8" : "#64748b"

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
                            color: "#38bdf8"
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
                            color: "#94a3b8"
                        }
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: encRoot.pushLabel
                            font.pixelSize: 9
                            font.bold: true
                            color: "#ffffff"
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
                    color: encRoot.selectedSlot === encRoot.cwSlot ? "#1e3a5f" : "#1f242d"
                    border.width: encRoot.selectedSlot === encRoot.cwSlot ? 2 : 1
                    border.color: encRoot.selectedSlot === encRoot.cwSlot ? "#38bdf8" : "#334155"

                    Column {
                        anchors.centerIn: parent
                        spacing: 1
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: "⟳"
                            font.pixelSize: 16
                            color: "#38bdf8"
                        }
                        QQC2.Label {
                            anchors.horizontalCenter: parent.horizontalCenter
                            text: encRoot.cwLabel
                            font.pixelSize: 9
                            font.bold: true
                            color: "#f1f5f9"
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
