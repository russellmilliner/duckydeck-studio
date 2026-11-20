import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import org.kde.kirigami as Kirigami
import org.kde.syntaxhighlighting as KSyntax

Rectangle {
    id: inspectorCard
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property var hardwareCanvas: null
    property var aiModal: null
    property var appLaunchConfigWindow: null
    property var webApiKeyConfigWindow: null
    property var autocompleteModel: null
    property var commandReferenceModel: null
    readonly property alias scriptEditor: scriptEditor
    readonly property alias scriptScrollView: scriptScrollView
    readonly property alias liveSlotAutoSaveTimer: liveSlotAutoSaveTimer
    readonly property alias label1Input: label1Input
    readonly property alias label2Input: label2Input
    readonly property alias widgetLabel1Input: widgetLabel1Input
    readonly property alias widgetLabel2Input: widgetLabel2Input
    readonly property alias allowAbortCheck: allowAbortCheck
    readonly property alias disableRepeatCheck: disableRepeatCheck
    readonly property alias gamingHoldModeCheck: gamingHoldModeCheck
    readonly property alias mainScriptEditorVerticalScrollBar: mainScriptEditorVerticalScrollBar
    readonly property alias shortcutsLibraryCard: shortcutsLibraryCard
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

                        QQC2.ToolTip.delay: 500
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

                        QQC2.ToolTip.delay: 500
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

                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: keyColorPillMouse.containsMouse
                        QQC2.ToolTip.text: qsTr("Set Key %1 Color (SWCOLOR) or pick from Saved & Used Palettes").arg(root.selectedSlotNumber)
                    }

                    QQC2.ToolButton {
                        text: "✕"
                        implicitWidth: Math.round(26 * root.inspectorScale)
                        implicitHeight: Math.round(26 * root.inspectorScale)
                        font.pointSize: 10 * root.fontScale
                        onClicked: root.clearSlotSelection()
                        QQC2.ToolTip.delay: 500
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
                FontMetrics {
                    id: oledLabelMetrics
                    font.family: "monospace"
                    font.pointSize: 10 * root.fontScale
                }

                RowLayout {
                    id: inspectorLabelsRow
                    Layout.fillWidth: true
                    spacing: 6

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
                            Layout.preferredWidth: Math.max(Math.round(96 * root.inspectorScale), Math.ceil(oledLabelMetrics.advanceWidth("WWWWW") + 32))
                            Layout.minimumWidth: Math.max(Math.round(86 * root.inspectorScale), Math.ceil(oledLabelMetrics.advanceWidth("WWWWW") + 22))
                            implicitHeight: Math.max(Math.round(30 * root.inspectorScale), Math.ceil(oledLabelMetrics.height + 8))
                            verticalAlignment: TextInput.AlignVCenter
                            topPadding: 4
                            bottomPadding: 4
                            leftPadding: 6
                            rightPadding: 6
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
                            Layout.preferredWidth: Math.max(Math.round(96 * root.inspectorScale), Math.ceil(oledLabelMetrics.advanceWidth("WWWWW") + 32))
                            Layout.minimumWidth: Math.max(Math.round(86 * root.inspectorScale), Math.ceil(oledLabelMetrics.advanceWidth("WWWWW") + 22))
                            implicitHeight: Math.max(Math.round(30 * root.inspectorScale), Math.ceil(oledLabelMetrics.height + 8))
                            verticalAlignment: TextInput.AlignVCenter
                            topPadding: 4
                            bottomPadding: 4
                            leftPadding: 6
                            rightPadding: 6
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
                        implicitHeight: Math.max(Math.round(28 * root.inspectorScale), Math.ceil(oledLabelMetrics.height + 6))
                        font.pointSize: 10 * root.fontScale
                        font.bold: true
                        onClicked: oledIconPickerDialog.openForInspector(1)
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Pick an OLED Button Icon or Symbol for Slot %1").arg(root.selectedSlotNumber)
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        icon.name: "edit-clear"
                        text: qsTr("Clear")
                        implicitHeight: Math.max(Math.round(28 * root.inspectorScale), Math.ceil(oledLabelMetrics.height + 6))
                        font.pointSize: 10 * root.fontScale
                        onClicked: root.clearSelectedSlotNow()
                        QQC2.ToolTip.delay: 500
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
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: widgetLblHover.hovered
                        QQC2.ToolTip.text: qsTr("💻 Host App Only (Not Hardware-Portable): Longer labels (up to 24 chars) displayed only on the Desktop OLED Widget while DuckyDeck Studio is running. Physical OLED always uses the 5-char L1/L2 labels above.")
                        HoverHandler { id: widgetLblHover }
                    }

                    QQC2.TextField {
                        id: widgetLabel1Input
                        Layout.fillWidth: true
                        Layout.minimumWidth: 70
                        implicitHeight: Math.max(Math.round(28 * root.inspectorScale), Math.ceil(oledLabelMetrics.height + 6))
                        verticalAlignment: TextInput.AlignVCenter
                        leftPadding: 6
                        rightPadding: 6
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
                        implicitHeight: Math.max(Math.round(28 * root.inspectorScale), Math.ceil(oledLabelMetrics.height + 6))
                        verticalAlignment: TextInput.AlignVCenter
                        leftPadding: 6
                        rightPadding: 6
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
                    spacing: 4

                    QQC2.TabBar {
                        id: inspectorPhaseTabBar
                        Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                        padding: 0
                        spacing: 2
                        background: Item {}
                        currentIndex: root.editingReleaseScript
                            ? (inspectorCard.isMechanicalKey ? 3 : 1)
                            : root.editingClickPhase

                        // 1x Single Click (On Press) Tab
                        QQC2.TabButton {
                            readonly property bool isActiveTab: root.editingClickPhase === 0 && !root.editingReleaseScript
                            checked: isActiveTab
                            width: implicitWidth
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            leftPadding: 10
                            rightPadding: 10
                            text: root.singleClickScriptBuffer.trim().length > 0
                                ? qsTr("⬇ 1× Press ●")
                                : qsTr("⬇ 1× Press")
                            font.pointSize: 10 * root.fontScale
                            font.bold: isActiveTab
                            onClicked: root.switchEditingClickPhase(0)
                            QQC2.ToolTip.delay: 500
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Edit script executed on 1× Single Click (On Press)")
                        }

                        // 2x Double Click Tab (Mechanical Keys 1..20)
                        QQC2.TabButton {
                            visible: inspectorCard.isMechanicalKey
                            readonly property bool isActiveTab: root.editingClickPhase === 1
                            checked: isActiveTab
                            width: visible ? implicitWidth : 0
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            leftPadding: 10
                            rightPadding: 10
                            text: root.doubleClickScriptBuffer.trim().length > 0
                                ? qsTr("⚡ 2× Double ●")
                                : qsTr("⚡ 2× Double")
                            font.pointSize: 10 * root.fontScale
                            font.bold: isActiveTab || root.doubleClickScriptBuffer.trim().length > 0
                            onClicked: root.switchEditingClickPhase(1)
                            QQC2.ToolTip.delay: 500
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Assign an action or DuckyScript 3 executed when this key is double-clicked within %1 ms").arg(deviceController ? deviceController.multiClickIntervalMs : 250)
                        }

                        // 3x Triple Click Tab (Mechanical Keys 1..20)
                        QQC2.TabButton {
                            visible: inspectorCard.isMechanicalKey
                            readonly property bool isActiveTab: root.editingClickPhase === 2
                            checked: isActiveTab
                            width: visible ? implicitWidth : 0
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            leftPadding: 10
                            rightPadding: 10
                            text: root.tripleClickScriptBuffer.trim().length > 0
                                ? qsTr("⚡⚡ 3× Triple ●")
                                : qsTr("⚡⚡ 3× Triple")
                            font.pointSize: 10 * root.fontScale
                            font.bold: isActiveTab || root.tripleClickScriptBuffer.trim().length > 0
                            onClicked: root.switchEditingClickPhase(2)
                            QQC2.ToolTip.delay: 500
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Assign an action or DuckyScript 3 executed when this key is triple-clicked (%1 ms between clicks)").arg(deviceController ? deviceController.multiClickIntervalMs : 250)
                        }

                        // On Release Tab
                        QQC2.TabButton {
                            readonly property bool isActiveTab: root.editingClickPhase === 3 || root.editingReleaseScript
                            checked: isActiveTab
                            width: implicitWidth
                            implicitHeight: Math.round(30 * root.inspectorScale)
                            leftPadding: 10
                            rightPadding: 10
                            text: root.releaseScriptBuffer.trim().length > 0
                                ? qsTr("⬆ On Release ●")
                                : qsTr("⬆ On Release")
                            font.pointSize: 10 * root.fontScale
                            font.bold: isActiveTab
                            onClicked: root.switchEditingClickPhase(3)
                            QQC2.ToolTip.delay: 500
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Edit script executed when released")
                        }
                    }

                    Item { Layout.fillWidth: true }

                    // Quick Double/Triple Click Timing control when on mechanical keys
                    RowLayout {
                        visible: inspectorCard.isMechanicalKey
                            && (root.editingClickPhase === 1
                                || root.editingClickPhase === 2
                                || root.doubleClickScriptBuffer.trim().length > 0
                                || root.tripleClickScriptBuffer.trim().length > 0)
                        spacing: 4

                        QQC2.Label {
                            text: qsTr("⏱ Gap:")
                            font.pointSize: 9.5 * root.fontScale
                            font.bold: true
                            color: Kirigami.Theme.disabledTextColor
                        }

                        QQC2.SpinBox {
                            id: inlineMultiClickIntervalSpin
                            from: 80
                            to: 1000
                            stepSize: 10
                            value: deviceController ? deviceController.multiClickIntervalMs : 250
                            editable: true
                            implicitHeight: Math.round(24 * root.inspectorScale)
                            implicitWidth: Math.round(92 * root.inspectorScale)
                            font.pointSize: 9.5 * root.fontScale
                            textFromValue: function(value) { return value + " ms" }
                            valueFromText: function(text) { return parseInt(text, 10) || 250 }
                            onValueModified: {
                                if (deviceController) {
                                    deviceController.applyMultiClickIntervalMs(value)
                                    root.syncActiveEditorToPhaseBuffers()
                                    liveSlotAutoSaveTimer.restart()
                                }
                            }
                            QQC2.ToolTip.delay: 500
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Global timing window (ms) between clicks for Double & Triple Click detection (also configurable in Settings)")
                        }
                    }
                }
            }

            // Key Hold Behavior Bar: Gaming Mode (Hold Key) + No Repeat on Key Held Down (Run Once) + Allow Abort
            Rectangle {
                Layout.fillWidth: true
                implicitHeight: holdFlagsRow.implicitHeight + 6
                radius: 5
                color: gamingHoldModeCheck.checked
                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                    : (disableRepeatCheck.checked
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.12)
                        : Kirigami.Theme.alternateBackgroundColor)
                border.color: gamingHoldModeCheck.checked
                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.65)
                    : (disableRepeatCheck.checked
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.55)
                        : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast))
                border.width: 1

                RowLayout {
                    id: holdFlagsRow
                    anchors.fill: parent
                    anchors.leftMargin: 6
                    anchors.rightMargin: 6
                    anchors.topMargin: 3
                    anchors.bottomMargin: 3
                    spacing: 8

                    HighContrastCheckBox {
                        id: gamingHoldModeCheck
                        visible: inspectorCard.isMechanicalKey
                        text: qsTr("🎮 Gaming Mode (Hold Key)")
                        font.bold: checked
                        font.pointSize: 10 * root.fontScale
                        onToggled: {
                            if (!deviceController) return
                            root.syncActiveEditorToPhaseBuffers()
                            const curPress = root.singleClickScriptBuffer
                            const curRel = root.releaseScriptBuffer
                            const hint = label1Input.text.trim() || label2Input.text.trim() || ("Key " + root.selectedSlotNumber)
                            try {
                                const raw = deviceController.convertScriptsForGamingHold(
                                    curPress || "",
                                    curRel || "",
                                    checked,
                                    hint
                                )
                                const res = JSON.parse(raw)
                                if (res && res.converted) {
                                    root.loadingSlotData = true
                                    root.singleClickScriptBuffer = res.press || ""
                                    root.releaseScriptBuffer = res.release || ""
                                    root.pressScriptBuffer = deviceController.composeMultiClickScript(
                                        root.singleClickScriptBuffer,
                                        root.doubleClickScriptBuffer,
                                        root.tripleClickScriptBuffer
                                    )
                                    scriptEditor.text = root.scriptForCurrentPhase()
                                    if (checked) {
                                        disableRepeatCheck.checked = false
                                    }
                                    root.loadingSlotData = false
                                    root.updateCompilerStatus()
                                    liveSlotAutoSaveTimer.restart()
                                }
                            } catch (e) {
                                console.warn("Failed converting script for Gaming Hold Mode:", e)
                            }
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("🎮 Gaming Hold Mode: Sends KEYDOWN when pressed and KEYUP when released so movement/action keys (like W/A/S/D/Space/Shift in Minecraft) stay continuously held down while you hold the physical duckyPad key.")
                    }

                    HighContrastCheckBox {
                        id: disableRepeatCheck
                        text: qsTr("No Repeat on Held (Run Once)")
                        font.bold: checked
                        font.pointSize: 10 * root.fontScale
                        onToggled: {
                            root.updateCompilerStatus()
                            liveSlotAutoSaveTimer.restart()
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Execute this key action only once per press even if the key is held down — essential for Delete or destructive actions (dr %1 / $_DONT_REPEAT = 1)").arg(root.selectedSlotNumber)
                    }

                    Item { Layout.fillWidth: true }

                    HighContrastCheckBox {
                        id: allowAbortCheck
                        text: qsTr("Allow Abort")
                        font.pointSize: 10 * root.fontScale
                        onToggled: {
                            root.updateCompilerStatus()
                            liveSlotAutoSaveTimer.restart()
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Allow any key press to abort this script early (ab %1 / $_ALLOW_ABORT = 1)").arg(root.selectedSlotNumber)
                    }

                    QQC2.ToolButton {
                        text: "?"
                        font.bold: true
                        font.pointSize: 9.5 * root.fontScale
                        implicitWidth: 22
                        implicitHeight: 22
                        padding: 0
                        onClicked: root.openTutorial("multiclick")
                        QQC2.ToolTip.delay: 400
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("🎓 Open Tutorial: Multi-Click (1×/2×/3×), Gaming Hold & Macro Recorder")
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
                spacing: 4

                QQC2.TabBar {
                    id: inspectorModeTabBar
                    Layout.preferredHeight: Math.round(30 * root.inspectorScale)
                    padding: 0
                    spacing: 2
                    background: Item {}

                    // Tab 1: DuckyScript IDE
                    QQC2.TabButton {
                        id: codeModeBtn
                        readonly property bool isActiveTab: root.inspectorMode === "code"
                        checked: isActiveTab
                        width: implicitWidth
                        implicitHeight: Math.round(30 * root.inspectorScale)
                        leftPadding: 10
                        rightPadding: 10
                        text: qsTr("📜 DuckyScript IDE")
                        font.pointSize: 10 * root.fontScale
                        font.bold: isActiveTab
                        onClicked: {
                            root.showAppLaunchBuilder = false
                            root.showWebApiBuilder = false
                            root.inspectorMode = "code"
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Full-height DuckyScript 3 editor with syntax highlighting, autocomplete & command reference")
                    }

                    // Tab 2: Launch App
                    QQC2.TabButton {
                        id: appLaunchToggleBtn
                        readonly property bool isActiveTab: root.inspectorMode === "app_launch" || appLaunchConfigWindow.visible
                        checked: isActiveTab
                        width: implicitWidth
                        implicitHeight: Math.round(30 * root.inspectorScale)
                        leftPadding: 10
                        rightPadding: 10
                        text: root.appLaunchEnabled
                            ? qsTr("🚀 Launch App ●")
                            : qsTr("🚀 Launch App")
                        font.pointSize: 10 * root.fontScale
                        font.bold: isActiveTab || root.appLaunchEnabled
                        onClicked: {
                            root.showWebApiBuilder = false
                            root.showAppLaunchBuilder = false
                            root.inspectorMode = "app_launch"
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Switch to Launch App tab")
                    }

                    // Tab 3: Web API
                    QQC2.TabButton {
                        id: webApiToggleBtn
                        readonly property bool isActiveTab: root.inspectorMode === "web_api" || webApiKeyConfigWindow.visible
                        checked: isActiveTab
                        width: implicitWidth
                        implicitHeight: Math.round(30 * root.inspectorScale)
                        leftPadding: 10
                        rightPadding: 10
                        text: root.webApiEnabled
                            ? qsTr("🌐 Web API ●")
                            : qsTr("🌐 Web API")
                        font.pointSize: 10 * root.fontScale
                        font.bold: isActiveTab || root.webApiEnabled
                        onClicked: {
                            root.showAppLaunchBuilder = false
                            root.showWebApiBuilder = false
                            root.inspectorMode = "web_api"
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Switch to Web API tab")
                    }

                    // Tab 4: Shortcuts Library
                    QQC2.TabButton {
                        id: shortcutsToggleBtn
                        readonly property bool isActiveTab: root.inspectorMode === "shortcuts"
                        checked: isActiveTab
                        width: implicitWidth
                        implicitHeight: Math.round(30 * root.inspectorScale)
                        leftPadding: 10
                        rightPadding: 10
                        text: qsTr("⌨️ Shortcuts")
                        font.pointSize: 10 * root.fontScale
                        font.bold: isActiveTab
                        onClicked: {
                            root.showAppLaunchBuilder = false
                            root.showWebApiBuilder = false
                            root.inspectorMode = "shortcuts"
                            if (keyEditorShortcutsListView.count === 0 && shortcutAppInputField.text.trim().length > 0 && deviceController) {
                                deviceController.searchAppShortcuts(
                                    shortcutAppInputField.text.trim(),
                                    allowGoogleShortcutsCheck.checked
                                )
                            }
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Search application keyboard shortcuts (Google & local cheatsheets) and drag or assign them to keys")
                    }
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
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Record live keyboard keys, mouse clicks, and mouse movements into DuckyScript 3 (with Exact Timings or As Fast As Possible)")
                }

                QQC2.Button {
                    text: qsTr("✨ Template")
                    implicitHeight: Math.round(24 * root.inspectorScale)
                    font.pointSize: 10 * root.fontScale
                    leftPadding: 6
                    rightPadding: 6
                    onClicked: aiModal.open()
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Generate DuckyScript 3 macro templates from KDE shortcuts using Macro Template Generator (experimental)")
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
            QQC2.ToolTip.delay: 500
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
                    text: qsTr("🎓 Guide")
                    implicitHeight: Math.round(22 * root.inspectorScale)
                    font.pointSize: 9.5 * root.fontScale
                    onClicked: root.openTutorial("app_launch")
                    QQC2.ToolTip.delay: 400
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Open step-by-step App Launching tutorial")
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
                    text: qsTr("🎓 HA Guide")
                    implicitHeight: Math.round(22 * root.inspectorScale)
                    font.pointSize: 9.5 * root.fontScale
                    onClicked: root.openTutorial("home_assistant")
                    QQC2.ToolTip.delay: 400
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Open step-by-step Home Assistant configuration tutorial")
                }

                QQC2.Button {
                    text: qsTr("🎓 Webhooks")
                    implicitHeight: Math.round(22 * root.inspectorScale)
                    font.pointSize: 9.5 * root.fontScale
                    onClicked: root.openTutorial("webhooks")
                    QQC2.ToolTip.delay: 400
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Open step-by-step Webhooks & REST API tutorial")
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
                        rightPadding: clearCmdRefBtn.visible ? 28 : 8
                        onTextEdited: {
                            root.commandRefSearchText = text
                            root.refreshCommandReferenceModel()
                        }

                        QQC2.ToolButton {
                            id: clearCmdRefBtn
                            visible: commandRefSearchField.text.length > 0
                            anchors.right: parent.right
                            anchors.rightMargin: 3
                            anchors.verticalCenter: parent.verticalCenter
                            width: 20
                            height: 20
                            implicitWidth: 20
                            implicitHeight: 20
                            padding: 0
                            flat: true
                            text: "✕"
                            font.pointSize: 9 * root.fontScale
                            onClicked: {
                                commandRefSearchField.text = ""
                                root.commandRefSearchText = ""
                                root.refreshCommandReferenceModel()
                                commandRefSearchField.forceActiveFocus()
                            }
                            QQC2.ToolTip.delay: 500
                            QQC2.ToolTip.visible: hovered
                            QQC2.ToolTip.text: qsTr("Clear search filter")
                        }
                    }

                    HighContrastCheckBox {
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
                        QQC2.ToolTip.delay: 500
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

        // Collapsible Shortcuts Discovery & Library Panel
        Rectangle {
            id: shortcutsLibraryCard
            visible: root.inspectorMode === "shortcuts"
            Layout.fillWidth: true
            Layout.fillHeight: root.inspectorMode === "shortcuts"
            implicitHeight: visible ? Math.round(340 * root.inspectorScale) : 0
            radius: Kirigami.Units.smallSpacing
            color: Kirigami.Theme.alternateBackgroundColor
            border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
            border.width: 1
            clip: true

            property string keyEditorShortcutFilter: ""
            property bool forceGamingHoldMode: false

            function shortcutCategoryColor(category) {
                const cat = String(category || "").toLowerCase()
                if (cat.includes("debug")) return Qt.rgba(0.96, 0.62, 0.04, 1.0)
                if (cat.includes("combat") || cat.includes("action")) return Qt.rgba(0.94, 0.27, 0.27, 1.0)
                if (cat.includes("move")) return Qt.rgba(0.13, 0.77, 0.37, 1.0)
                if (cat.includes("hotbar") || cat.includes("creative")) return Qt.rgba(0.66, 0.33, 0.97, 1.0)
                if (cat.includes("camera") || cat.includes("view") || cat.includes("hud")) return Qt.rgba(0.02, 0.71, 0.83, 1.0)
                if (cat.includes("chat") || cat.includes("multi")) return Qt.rgba(0.93, 0.28, 0.60, 1.0)
                return root.selectedSlotColor || Kirigami.Theme.highlightColor
            }

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 10
                spacing: 6

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 6

                    QQC2.TextField {
                        id: shortcutAppInputField
                        Layout.fillWidth: true
                        implicitHeight: Math.round(28 * root.inspectorScale)
                        font.pointSize: 10 * root.fontScale
                        placeholderText: qsTr("App or game (e.g. Minecraft, Dolphin, Chrome, VS Code)...")
                        text: (deviceController && deviceController.lastPickedWindowClass)
                            ? deviceController.lastPickedWindowClass
                            : ((deviceController && deviceController.activeProfileName) ? deviceController.activeProfileName : "")
                        onAccepted: searchShortcutsBtn.clicked()
                    }

                    QQC2.Button {
                        text: deviceController && deviceController.windowPickerArmed ? qsTr("🎯 Picking...") : qsTr("🎯 Pick Window")
                        implicitHeight: Math.round(28 * root.inspectorScale)
                        font.pointSize: 10 * root.fontScale
                        highlighted: deviceController && deviceController.windowPickerArmed
                        onClicked: {
                            if (deviceController) deviceController.armWindowPicker(false)
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Pick any open desktop window to search its shortcuts")
                    }

                    QQC2.Button {
                        id: searchShortcutsBtn
                        text: deviceController && deviceController.searchShortcutsBusy ? qsTr("Searching...") : qsTr("🔍 Search")
                        implicitHeight: Math.round(28 * root.inspectorScale)
                        font.pointSize: 10 * root.fontScale
                        highlighted: true
                        enabled: shortcutAppInputField.text.trim().length > 0 && !(deviceController && deviceController.searchShortcutsBusy)
                        onClicked: {
                            if (deviceController) {
                                deviceController.searchAppShortcuts(
                                    shortcutAppInputField.text.trim(),
                                    allowGoogleShortcutsCheck.checked
                                )
                            }
                        }
                    }
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    QQC2.TextField {
                        id: keyEditorShortcutFilterField
                        Layout.fillWidth: true
                        implicitHeight: Math.round(26 * root.inspectorScale)
                        font.pointSize: 9.5 * root.fontScale
                        placeholderText: qsTr("Filter shortcuts (e.g. Inventory, F3, Chunk, Save)...")
                        text: shortcutsLibraryCard.keyEditorShortcutFilter
                        onTextChanged: shortcutsLibraryCard.keyEditorShortcutFilter = text
                    }

                    QQC2.CheckBox {
                        id: keGamingHoldCheck
                        text: qsTr("🎮 Gaming Hold")
                        font.pointSize: 9.5 * root.fontScale
                        font.bold: checked
                        checked: shortcutsLibraryCard.forceGamingHoldMode
                        onToggled: shortcutsLibraryCard.forceGamingHoldMode = checked
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Assign shortcuts in Gaming Hold Mode (KEYDOWN on press + KEYUP on release) so keys stay held while pressed")
                    }

                    QQC2.CheckBox {
                        id: allowGoogleShortcutsCheck
                        text: qsTr("🌐 Web Search")
                        font.pointSize: 9.5 * root.fontScale
                        checked: deviceController ? deviceController.googleSearchShortcutsPermitted : false
                        onToggled: {
                            if (deviceController) {
                                deviceController.setGoogleSearchPermission(checked)
                                if (shortcutAppInputField.text.trim().length > 0) {
                                    deviceController.searchAppShortcuts(
                                        shortcutAppInputField.text.trim(),
                                        checked
                                    )
                                }
                            }
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Allow online web search for shortcut cheatsheets (privacy permission)")
                    }

                    QQC2.Button {
                        text: qsTr("🧙 Profile Wizard...")
                        implicitHeight: Math.round(26 * root.inspectorScale)
                        font.pointSize: 9.5 * root.fontScale
                        onClicked: root.openProfileWizard()
                    }
                }

                // Menu of Shortcut Keys (Main-Window Keycap Look & Feel + Drag-and-Drop onto HardwareCanvas)
                ListView {
                    id: keyEditorShortcutsListView
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    spacing: 6
                    boundsBehavior: Flickable.StopAtBounds
                    model: {
                        if (!deviceController || !deviceController.discoveredShortcutsJson) return []
                        try {
                            const arr = JSON.parse(deviceController.discoveredShortcutsJson)
                            if (!Array.isArray(arr)) return []
                            const filter = shortcutsLibraryCard.keyEditorShortcutFilter.trim().toLowerCase()
                            if (!filter) return arr
                            return arr.filter(s =>
                                (s.action && s.action.toLowerCase().includes(filter)) ||
                                (s.shortcut && s.shortcut.toLowerCase().includes(filter)) ||
                                (s.category && s.category.toLowerCase().includes(filter)) ||
                                (s.label_l1 && s.label_l1.toLowerCase().includes(filter)) ||
                                (s.label_l2 && s.label_l2.toLowerCase().includes(filter))
                            )
                        } catch (e) {
                            return []
                        }
                    }

                    delegate: Item {
                        id: keScDelegate
                        width: keyEditorShortcutsListView.width
                        height: 74
                        readonly property color itemLedColor: shortcutsLibraryCard.shortcutCategoryColor(modelData.category)

                        Rectangle {
                            anchors.fill: parent
                            radius: 8
                            color: scCardMouse.containsMouse
                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.12))
                                : Kirigami.Theme.backgroundColor
                            border.width: scCardMouse.containsMouse ? 1.5 : 1
                            border.color: scCardMouse.containsMouse
                                ? Kirigami.Theme.highlightColor
                                : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

                            RowLayout {
                                anchors.fill: parent
                                anchors.leftMargin: 6
                                anchors.rightMargin: 8
                                spacing: 10

                                // Authentic Main-Window 66x66 duckyPad Keycap Preview
                                Item {
                                    Layout.preferredWidth: 64
                                    Layout.preferredHeight: 64

                                    Rectangle {
                                        anchors.fill: parent
                                        radius: 8
                                        color: Kirigami.Theme.backgroundColor
                                        border.width: scCardMouse.containsMouse ? 2 : 1
                                        border.color: scCardMouse.containsMouse
                                            ? Kirigami.Theme.highlightColor
                                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

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
                                            x: 21
                                            y: 4
                                            width: 34
                                            height: 4
                                            radius: 2
                                            color: keScDelegate.itemLedColor
                                        }

                                        Rectangle {
                                            anchors.fill: parent
                                            anchors.margins: 5
                                            anchors.topMargin: 12
                                            radius: 6
                                            clip: true
                                            color: scCardMouse.pressed
                                                ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(keScDelegate.itemLedColor.r, keScDelegate.itemLedColor.g, keScDelegate.itemLedColor.b, 0.30))
                                                : (scCardMouse.containsMouse
                                                    ? Qt.tint(Kirigami.Theme.backgroundColor, Qt.rgba(Kirigami.Theme.highlightColor.r, Kirigami.Theme.highlightColor.g, Kirigami.Theme.highlightColor.b, 0.16))
                                                    : Kirigami.Theme.alternateBackgroundColor)
                                            border.width: 1
                                            border.color: scCardMouse.containsMouse
                                                ? Kirigami.Theme.highlightColor
                                                : Qt.rgba(keScDelegate.itemLedColor.r, keScDelegate.itemLedColor.g, keScDelegate.itemLedColor.b, 0.55)

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

                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 3

                                    QQC2.Label {
                                        Layout.fillWidth: true
                                        text: modelData.action || ""
                                        font.bold: true
                                        font.pointSize: 9.5 * root.fontScale
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
                                            implicitWidth: keScBadgeLbl.implicitWidth + 8
                                            implicitHeight: Math.round(17 * root.inspectorScale)

                                            QQC2.Label {
                                                id: keScBadgeLbl
                                                anchors.centerIn: parent
                                                text: modelData.shortcut || ""
                                                font.family: "monospace"
                                                font.bold: true
                                                font.pointSize: 8.5 * root.fontScale
                                                color: Kirigami.Theme.highlightColor
                                            }
                                        }

                                        QQC2.Label {
                                            text: (modelData.gaming_hold || shortcutsLibraryCard.forceGamingHoldMode ? "🎮 Hold • " : "") + (modelData.category || modelData.source || "")
                                            font.pointSize: 8.5 * root.fontScale
                                            font.bold: !!(modelData.gaming_hold || shortcutsLibraryCard.forceGamingHoldMode)
                                            color: (modelData.gaming_hold || shortcutsLibraryCard.forceGamingHoldMode)
                                                ? Kirigami.Theme.positiveTextColor
                                                : Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideRight
                                        }
                                    }
                                }

                                QQC2.Button {
                                    id: keApplyBtn
                                    z: 5
                                    text: root.editingClickPhase === 1
                                        ? qsTr("Apply 2× → K%1").arg(root.selectedSlotNumber)
                                        : (root.editingClickPhase === 2
                                            ? qsTr("Apply 3× → K%1").arg(root.selectedSlotNumber)
                                            : qsTr("Apply → K%1").arg(root.selectedSlotNumber))
                                    implicitHeight: Math.round(26 * root.inspectorScale)
                                    font.pointSize: 9 * root.fontScale
                                    highlighted: true
                                    onClicked: {
                                        if (deviceController && root.hasSelectedSlot) {
                                            const useHold = !!(modelData.gaming_hold || shortcutsLibraryCard.forceGamingHoldMode)
                                            const clickSuffix = root.editingClickPhase === 1
                                                ? ":double_click"
                                                : (root.editingClickPhase === 2 ? ":triple_click" : "")
                                            const catArg = (modelData.category || "") + (useHold ? ":gaming_hold" : "") + clickSuffix
                                            deviceController.assignShortcutToKeySlot(
                                                0,
                                                root.selectedSlotNumber,
                                                modelData.action || "",
                                                modelData.shortcut || "",
                                                modelData.duckyscript || "",
                                                catArg
                                            )
                                            root.refreshSelectedSlotFromController()
                                            hardwareCanvas.syncSlotsFromController()
                                            root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
                                        }
                                    }
                                }
                            }
                        }

                        MouseArea {
                            id: scCardMouse
                            anchors.top: parent.top
                            anchors.bottom: parent.bottom
                            anchors.left: parent.left
                            anchors.right: parent.right
                            anchors.rightMargin: keApplyBtn.width + 22
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
                                    root.draggingShortcutItem = modelData
                                    hardwareCanvas.externalShortcutDragActive = true
                                }
                                if (didDrag) {
                                    const winPt = scCardMouse.mapToItem(root.contentItem, mouse.x, mouse.y)
                                    root.shortcutDragOverlayX = winPt.x - 33
                                    root.shortcutDragOverlayY = winPt.y - 33
                                    hardwareCanvas.dropTargetSlot = hardwareCanvas.hitTestSlotAtItemPos(scCardMouse, mouse.x, mouse.y)
                                }
                            }

                            onReleased: (mouse) => {
                                if (didDrag) {
                                    const targetSlot = hardwareCanvas.dropTargetSlot
                                    const sc = root.draggingShortcutItem
                                    didDrag = false
                                    root.draggingShortcutItem = null
                                    hardwareCanvas.externalShortcutDragActive = false
                                    hardwareCanvas.dropTargetSlot = -1
                                    if (targetSlot >= 1 && sc && deviceController) {
                                        const useHold = !!(sc.gaming_hold || shortcutsLibraryCard.forceGamingHoldMode)
                                        const clickSuffix = root.editingClickPhase === 1
                                            ? ":double_click"
                                            : (root.editingClickPhase === 2 ? ":triple_click" : "")
                                        const catArg = (sc.category || "") + (useHold ? ":gaming_hold" : "") + clickSuffix
                                        deviceController.assignShortcutToKeySlot(
                                            0,
                                            targetSlot,
                                            sc.action || "",
                                            sc.shortcut || "",
                                            sc.duckyscript || "",
                                            catArg
                                        )
                                        hardwareCanvas.syncSlotsFromController()
                                        if (targetSlot === root.selectedSlotNumber) {
                                            root.refreshSelectedSlotFromController()
                                            root.selectedSlotTitle = hardwareCanvas.slotTitleFor(root.selectedSlotNumber)
                                        }
                                    }
                                    return
                                }
                            }

                            onCanceled: {
                                didDrag = false
                                root.draggingShortcutItem = null
                                hardwareCanvas.externalShortcutDragActive = false
                                hardwareCanvas.dropTargetSlot = -1
                            }

                            QQC2.ToolTip.visible: scCardMouse.containsMouse && !didDrag
                            QQC2.ToolTip.delay: 450
                            QQC2.ToolTip.text: qsTr("Drag this keycap onto any duckyPad key on the left, or click 'Apply → K%1'").arg(root.selectedSlotNumber)
                        }
                    }

                    QQC2.Label {
                        anchors.centerIn: parent
                        visible: keyEditorShortcutsListView.count === 0 && !(deviceController && deviceController.searchShortcutsBusy)
                        text: qsTr("Enter an application or game name (e.g. Minecraft) or pick a window above, then click Search.")
                        color: Kirigami.Theme.disabledTextColor
                        font.pointSize: 9.5 * root.fontScale
                    }

                    QQC2.BusyIndicator {
                        anchors.centerIn: parent
                        running: deviceController && deviceController.searchShortcutsBusy
                        visible: running
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
                        QQC2.ToolTip.delay: 500
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
                                    : (root.editingClickPhase === 1
                                        ? ("key" + root.selectedSlotNumber + ".txt [2× Double]")
                                        : (root.editingClickPhase === 2
                                            ? ("key" + root.selectedSlotNumber + ".txt [3× Triple]")
                                            : ("key" + root.selectedSlotNumber + ".txt")))
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
                        QQC2.ToolTip.delay: 500
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
                        QQC2.ToolTip.delay: 500
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
                        onClicked: {
                            hardwareCanvas.oledScriptMode = true
                            root.insertCommandFromReference("OLED_CLEAR\nOLED_RECT 2 2 125 125 1\nOLED_CURSOR 0 24\nOLED_CPRINT Hello OLED!\nOLED_UPDATE")
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Insert OLED screen drawing block & switch PCB preview to OLED VM HUD")
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
                        QQC2.ToolTip.delay: 500
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
                        QQC2.ToolTip.delay: 500
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
                    id: mainScriptEditorVerticalScrollBar
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
                    implicitWidth: Math.max(
                        contentWidth + leftPadding + rightPadding,
                        implicitBackgroundWidth + leftInset + rightInset
                    )
                    implicitHeight: Math.max(
                        contentHeight + topPadding + bottomPadding,
                        implicitBackgroundHeight + topInset + bottomInset
                    )
                    background: Rectangle { color: "transparent" }
                    color: Kirigami.Theme.textColor
                    selectionColor: Kirigami.Theme.highlightColor
                    selectedTextColor: Kirigami.Theme.highlightedTextColor
                    selectByMouse: true
                    wrapMode: TextEdit.NoWrap
                    onTextChanged: {
                        if (!root.loadingSlotData) {
                            root.syncActiveEditorToPhaseBuffers()
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
                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.text: inspectorCard.hasRevertibleChanges
                    ? qsTr("Restore Key %1 labels, color, flags, and scripts to their state when this key was selected").arg(root.selectedSlotNumber)
                    : qsTr("No changes made to Key %1 since it was selected").arg(root.selectedSlotNumber)
            }
        }
    }
}
