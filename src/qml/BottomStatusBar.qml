import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Rectangle {
    id: bottomStatusBar
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property var hardwareCanvas: null
    property var deviceSelectorMenu: null
    property var eventLogDialog: null
    property var oledDesktopWidget: null
    property var profileSyncMenu: null
    property var sdSyncDialog: null
    readonly property alias connStatusPill: connStatusPill
    readonly property alias syncStatusBadge: syncStatusBadge
    readonly property alias fwBadgeRect: fwBadgeRect
    readonly property alias sdSpaceBadge: sdSpaceBadge
    readonly property alias oledWidgetStatusBtn: oledWidgetStatusBtn
    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    readonly property bool hasMultiWidgetOrEsp32Row: root.connectedDevicesList.length > 1
        || (deviceController && deviceController.esp32HaDetected)
    width: parent.width
    implicitHeight: Math.round((hasMultiWidgetOrEsp32Row ? 106 : 72) * root.statusBarScale)
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

            // Connection Status Pill (Click to select managed duckyPad or switch model)
            Rectangle {
                id: connStatusPill
                objectName: "connStatusPill"
                radius: 4
                color: deviceController.connected
                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, connStatusMouse.containsMouse ? 0.24 : 0.16)
                    : (deviceController.sdMountPath.length > 0
                        ? Qt.alpha(Kirigami.Theme.highlightColor, connStatusMouse.containsMouse ? 0.24 : 0.16)
                        : Qt.alpha(Kirigami.Theme.neutralTextColor, connStatusMouse.containsMouse ? 0.24 : 0.16))
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
                        text: {
                            const multiSuffix = deviceController.connectedDeviceCount > 1
                                ? qsTr(" (1/%1)").arg(deviceController.connectedDeviceCount)
                                : ""
                            const customName = deviceController && deviceController.customDeviceName
                                ? String(deviceController.customDeviceName).trim()
                                : ""
                            const modelDisplay = customName.length > 0
                                ? qsTr("%1 (%2)").arg(customName).arg(deviceController.modelName)
                                : deviceController.modelName
                            const base = deviceController.connected
                                ? qsTr("USB HID Connected — %1%2").arg(modelDisplay).arg(multiSuffix)
                                : (deviceController.sdMountPath.length > 0
                                    ? qsTr("SD Card Mode — %1").arg(modelDisplay)
                                    : qsTr("Disconnected (%1)").arg(modelDisplay))
                            return base + " ▾"
                        }
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }
                }

                MouseArea {
                    id: connStatusMouse
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        deviceSelectorMenu.popup(connStatusPill, 0, -deviceSelectorMenu.implicitHeight)
                    }
                }

                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: connStatusMouse.containsMouse
                QQC2.ToolTip.text: qsTr("Click to choose which attached duckyPad or mounted SD card to manage, or switch between duckyPad Pro (20-key) and duckyPad 2020 (15-key)")
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

                QQC2.ToolTip.delay: 500
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
                readonly property bool hasAutoSwitchErr: deviceController.autoSwitchErrorMessage.length > 0
                Layout.fillWidth: true
                implicitHeight: Math.round(26 * root.statusBarScale)
                radius: 4
                color: statusLogLineBox.hasAutoSwitchErr
                    ? Qt.alpha(Kirigami.Theme.negativeTextColor, statusLogLineMouse.containsMouse ? 0.24 : 0.16)
                    : (statusLogLineMouse.containsMouse
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.14)
                        : "transparent")
                border.color: statusLogLineBox.hasAutoSwitchErr
                    ? Kirigami.Theme.negativeTextColor
                    : (statusLogLineMouse.containsMouse
                        ? Qt.alpha(Kirigami.Theme.highlightColor, 0.45)
                        : "transparent")
                border.width: 1

                QQC2.Label {
                    anchors.fill: parent
                    anchors.leftMargin: 6
                    anchors.rightMargin: 6
                    verticalAlignment: Text.AlignVCenter
                    text: statusLogLineBox.hasAutoSwitchErr
                        ? deviceController.autoSwitchErrorMessage
                        : deviceController.statusMessage
                    font.pointSize: 11 * root.fontScale
                    font.bold: statusLogLineBox.hasAutoSwitchErr
                    color: statusLogLineBox.hasAutoSwitchErr
                        ? Kirigami.Theme.negativeTextColor
                        : Kirigami.Theme.textColor
                    opacity: (statusLogLineMouse.containsMouse || statusLogLineBox.hasAutoSwitchErr) ? 1.0 : 0.92
                    elide: Text.ElideRight
                }

                MouseArea {
                    id: statusLogLineMouse
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: eventLogDialog.openLogWindow()
                }

                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: statusLogLineMouse.containsMouse
                QQC2.ToolTip.text: (statusLogLineBox.hasAutoSwitchErr
                    ? (deviceController.autoSwitchErrorMessage + "\n" + deviceController.statusMessage + "\n")
                    : (deviceController.statusMessage.length > 0
                        ? (deviceController.statusMessage + "\n")
                        : "")) + qsTr("Click to open the Event Log Viewer")
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

                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: fwBadgeMouse.containsMouse
                QQC2.ToolTip.text: {
                    const originDesc = root.isDuckyDeckStudioFirmware(deviceController.firmwareVersion)
                        ? qsTr("DuckyDeck Studio Custom Firmware v%1 (Monochrome Icons & 0°/90°/180°/270° OLED)").arg(deviceController.firmwareVersion.split(" ")[0])
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

                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: sdSpaceMouse.containsMouse
                QQC2.ToolTip.text: deviceController.sdMountPath.length > 0
                    ? (deviceController.sdSpaceDetail + "\n" + qsTr("Mount Path: %1").arg(deviceController.sdMountPath))
                    : deviceController.sdSpaceDetail
            }

            Item {
                Layout.fillWidth: true
            }

            // Desktop OLED Widget Toggle Badge (hidden when no duckyPad is connected)
            Rectangle {
                id: oledWidgetStatusBtn
                visible: (deviceController && deviceController.connected)
                    || root.connectedDevicesList.length > 0
                    || (deviceController && (deviceController.isUiTestSuiteMode() || deviceController.isDemoShowcaseMode()))
                readonly property int activeWidgetCount: (oledDesktopWidget.visible ? 1 : 0) + root.openDeviceWidgetIdList.length
                radius: 4
                color: activeWidgetCount > 0
                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.16)
                    : Kirigami.Theme.backgroundColor
                border.color: activeWidgetCount > 0
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
                        color: oledWidgetStatusBtn.activeWidgetCount > 0
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
                        text: oledWidgetStatusBtn.activeWidgetCount > 1
                            ? qsTr("Desktop OLED (%1 Active)").arg(oledWidgetStatusBtn.activeWidgetCount)
                            : (oledWidgetStatusBtn.activeWidgetCount === 1
                                ? qsTr("Desktop OLED: Active")
                                : qsTr("Desktop OLED Widget"))
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }
                }

                MouseArea {
                    id: oledWidgetBtnMouse
                    anchors.fill: parent
                    hoverEnabled: true
                    acceptedButtons: Qt.LeftButton | Qt.RightButton
                    cursorShape: Qt.PointingHandCursor
                    onClicked: (mouse) => {
                        if (mouse.button === Qt.RightButton) {
                            deviceSelectorMenu.popup(oledWidgetStatusBtn, 0, -deviceSelectorMenu.implicitHeight)
                        } else {
                            deviceController.toggleOledDesktopWidget()
                        }
                    }
                }

                QQC2.ToolTip.delay: 500
                QQC2.ToolTip.visible: oledWidgetBtnMouse.containsMouse
                QQC2.ToolTip.text: qsTr("Toggle the floating Desktop OLED Widget (Right-click to toggle individual widgets per duckyPad)")
            }
        }

        Rectangle {
            visible: bottomStatusBar.hasMultiWidgetOrEsp32Row
            Layout.fillWidth: true
            height: 1
            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast * 0.65)
        }

        // Status Bar Line 3: Per-Device OLED Widget Items (when >1 connected widget available) & ESP32-HA Web Server Controls
        RowLayout {
            id: multiWidgetStatusBarRow
            visible: bottomStatusBar.hasMultiWidgetOrEsp32Row
            Layout.fillWidth: true
            spacing: 8

            QQC2.Label {
                visible: root.connectedDevicesList.length > 1
                text: qsTr("Widgets:")
                font.bold: true
                font.pointSize: 9.5 * root.fontScale
                color: Kirigami.Theme.disabledTextColor
            }

            Repeater {
                model: root.connectedDevicesList.length > 1 ? root.connectedDevicesList : []
                delegate: Rectangle {
                    id: perDevWidgetStatusBtn
                    required property var modelData
                    required property int index
                    readonly property bool widgetOpen: root.isDeviceOledWidgetOpen(modelData.id, !!modelData.isSelected)
                    readonly property bool isManaged: !!modelData.isSelected
                    readonly property string shortSerial: String(modelData.serialNumber || modelData.id).slice(-4)
                    readonly property string modelShort: modelData.customName && modelData.customName.length > 0
                        ? modelData.customName
                        : (modelData.hwModel === 20 ? "duckyPad 2020" : "duckyPad Pro")
                    objectName: "statusBarWidgetItem_" + String(modelData.id || index)
                    radius: 4
                    color: widgetOpen
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, perDevWidgetMouse.containsMouse ? 0.26 : 0.18)
                        : (isManaged
                            ? Qt.alpha(Kirigami.Theme.highlightColor, perDevWidgetMouse.containsMouse ? 0.22 : 0.12)
                            : (perDevWidgetMouse.containsMouse
                                ? Qt.alpha(Kirigami.Theme.textColor, 0.12)
                                : Kirigami.Theme.backgroundColor))
                    border.color: widgetOpen
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.6)
                        : (isManaged
                            ? Qt.alpha(Kirigami.Theme.highlightColor, 0.5)
                            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast))
                    border.width: 1
                    implicitWidth: perDevWidgetBtnRow.implicitWidth + 14
                    implicitHeight: Math.round(25 * root.statusBarScale)

                    RowLayout {
                        id: perDevWidgetBtnRow
                        anchors.centerIn: parent
                        spacing: 5

                        Rectangle {
                            width: 8
                            height: 8
                            radius: 4
                            color: perDevWidgetStatusBtn.widgetOpen
                                ? Kirigami.Theme.positiveTextColor
                                : (modelData.usbConnected
                                    ? Kirigami.Theme.highlightColor
                                    : Kirigami.Theme.disabledTextColor)
                        }

                        Image {
                            source: "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/oled-widget-icon.svg"
                            Layout.preferredWidth: Math.round(14 * root.statusBarScale)
                            Layout.preferredHeight: Math.round(14 * root.statusBarScale)
                            sourceSize.width: 28
                            sourceSize.height: 28
                            fillMode: Image.PreserveAspectFit
                        }

                        QQC2.Label {
                            text: {
                                const espTag = modelData.isEsp32HaConfigured ? " [ESP32-HA]" : ""
                                const profPart = modelData.usbConnected
                                    ? qsTr(" • P#%1").arg(modelData.activeProfile || 1)
                                    : ""
                                const stateTag = perDevWidgetStatusBtn.widgetOpen ? qsTr(" • Active") : ""
                                return qsTr("%1 (%2)%3%4%5")
                                    .arg(perDevWidgetStatusBtn.modelShort)
                                    .arg(perDevWidgetStatusBtn.shortSerial)
                                    .arg(espTag)
                                    .arg(profPart)
                                    .arg(stateTag)
                            }
                            font.bold: perDevWidgetStatusBtn.widgetOpen || perDevWidgetStatusBtn.isManaged
                            font.pointSize: 9.5 * root.fontScale
                            color: Kirigami.Theme.textColor
                        }
                    }

                    MouseArea {
                        id: perDevWidgetMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        acceptedButtons: Qt.LeftButton | Qt.RightButton
                        cursorShape: Qt.PointingHandCursor
                        onClicked: (mouse) => {
                            if (mouse.button === Qt.RightButton) {
                                if (deviceController && modelData.id) {
                                    if (deviceController.selectManagedDevice(modelData.id)) {
                                        root.syncSidebarProfilesFromController()
                                        hardwareCanvas.syncProfilesFromController()
                                        hardwareCanvas.syncSlotsFromController()
                                        root.refreshSelectedSlotFromController()
                                    }
                                }
                            } else {
                                const shortBadge = modelData.customName && modelData.customName.length > 0
                                    ? modelData.customName
                                    : ((modelData.hwModel === 20 ? "DP20·" : "DP24·")
                                        + perDevWidgetStatusBtn.shortSerial)
                                root.toggleDeviceOledWidget(modelData.id, shortBadge, !!modelData.isSelected)
                            }
                        }
                    }

                    QQC2.ToolTip.delay: 400
                    QQC2.ToolTip.visible: perDevWidgetMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Left-click: Toggle Desktop OLED Widget for %1\nRight-click: Switch Studio to manage this duckyPad")
                        .arg(modelData.displayLabel || modelData.id)
                }
            }

            Item {
                Layout.fillWidth: true
            }

            // ESP32-HA Web Server Status & Toggle Badge (only visible when an ESP32-HA duckyPad is connected on USB)
            Rectangle {
                id: esp32HaStatusBarBadge
                objectName: "esp32HaStatusBarBadge"
                visible: deviceController && deviceController.esp32HaDetected
                radius: 4
                color: deviceController && deviceController.esp32HaServerRunning
                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, esp32HaStatusMouse.containsMouse ? 0.26 : 0.18)
                    : Qt.alpha("#0ea5e9", esp32HaStatusMouse.containsMouse ? 0.26 : 0.16)
                border.color: deviceController && deviceController.esp32HaServerRunning
                    ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.6)
                    : Qt.alpha("#38bdf8", 0.55)
                border.width: 1
                implicitWidth: esp32HaBadgeRow.implicitWidth + 16
                implicitHeight: Math.round(25 * root.statusBarScale)

                RowLayout {
                    id: esp32HaBadgeRow
                    anchors.centerIn: parent
                    spacing: 6

                    Rectangle {
                        width: 8
                        height: 8
                        radius: 4
                        color: deviceController && deviceController.esp32HaServerRunning
                            ? Kirigami.Theme.positiveTextColor
                            : "#38bdf8"
                    }

                    QQC2.Label {
                        text: deviceController && deviceController.esp32HaServerRunning
                            ? qsTr("🌐 ESP32-HA Web Server: Running (:%1)").arg(deviceController.esp32HaServerPort)
                            : qsTr("🌐 ESP32-HA Detected — Start Web Server (:%1)").arg(deviceController ? deviceController.esp32HaServerPort : 8099)
                        font.bold: true
                        font.pointSize: 9.5 * root.fontScale
                        color: Kirigami.Theme.textColor
                    }
                }

                MouseArea {
                    id: esp32HaStatusMouse
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        if (deviceController) {
                            deviceController.toggleEsp32HaWebServer()
                        }
                    }
                }

                QQC2.ToolTip.delay: 400
                QQC2.ToolTip.visible: esp32HaStatusMouse.containsMouse
                QQC2.ToolTip.text: deviceController && deviceController.esp32HaServerRunning
                    ? qsTr("ESP32-HA Bridge Web Server is running at %1\nClick to stop the server").arg(deviceController.esp32HaServerUrl)
                    : qsTr("duckyPad configured for duckypad-esp32-ha detected (SN: %1)\nClick to start the built-in ESP32-HA Web Server").arg(deviceController ? deviceController.esp32HaDeviceSerial : "")
            }

            // Open ESP32-HA Web UI in Browser Button
            Rectangle {
                id: esp32HaOpenWebUiBtn
                objectName: "esp32HaOpenWebUiBtn"
                visible: deviceController && deviceController.esp32HaDetected
                radius: 4
                color: esp32HaOpenMouse.containsMouse
                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.26)
                    : Kirigami.Theme.backgroundColor
                border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.55)
                border.width: 1
                implicitWidth: esp32HaOpenLabel.implicitWidth + 14
                implicitHeight: Math.round(25 * root.statusBarScale)

                QQC2.Label {
                    id: esp32HaOpenLabel
                    anchors.centerIn: parent
                    text: qsTr("↗ Open Web UI")
                    font.bold: true
                    font.pointSize: 9.5 * root.fontScale
                    color: Kirigami.Theme.textColor
                }

                MouseArea {
                    id: esp32HaOpenMouse
                    anchors.fill: parent
                    hoverEnabled: true
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        if (deviceController) {
                            deviceController.openEsp32HaWebUi()
                        }
                    }
                }

                QQC2.ToolTip.delay: 400
                QQC2.ToolTip.visible: esp32HaOpenMouse.containsMouse
                QQC2.ToolTip.text: qsTr("Start (if needed) and open the ESP32-HA Web Configurator (%1) in your browser").arg(deviceController ? deviceController.esp32HaServerUrl : "http://127.0.0.1:8099")
            }
        }
    }
}
