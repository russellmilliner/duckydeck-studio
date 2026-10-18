import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import QtQuick.Dialogs
import org.kde.kirigami as Kirigami

Window {
    id: sdSyncDialog
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property string effectiveThemeMode: root ? root.effectiveThemeMode : "dark"
    property var hardwareCanvas: null
    property var inspectorCard: null
    property alias unsyncedChangesDialog: unsyncedChangesDialog
    property alias firmwareUpdateDialog: firmwareUpdateDialog

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

    ListModel {
        id: unsyncedItemsModel
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
            sdSyncDialog.unsyncedHeadline = parsed.headline || ""
            sdSyncDialog.unsyncedTotalChanges = parsed.totalChanges || 0
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
            sdSyncDialog.unsyncedHeadline = qsTr("Unable to parse unsynced changes summary.")
            sdSyncDialog.unsyncedTotalChanges = 0
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
            sdSyncDialog.refreshUnsyncedSummaryModel()
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
        sdSyncDialog.refreshUnsyncedSummaryModel()
        unsyncedChangesDialog.open()
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
                        text: sdSyncDialog.unsyncedHeadline.length > 0
                            ? sdSyncDialog.unsyncedHeadline
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
                                    onClicked: sdSyncDialog.discardSingleUnsyncedChange(
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
                                                onClicked: sdSyncDialog.discardSingleUnsyncedChange(
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
                    font.pointSize: 10 * root.fontScale
                    icon.name: "view-refresh"
                    text: qsTr("Refresh Summary")
                    onClicked: sdSyncDialog.refreshUnsyncedSummaryModel()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
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
                    font.pointSize: 10 * root.fontScale
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
                        font.pointSize: 10 * root.fontScale
                        text: qsTr("All (%1)").arg(firmwareUpdateDialog.availableFirmwares.length)
                        highlighted: firmwareUpdateDialog.vendorFilter === "all"
                        onClicked: {
                            firmwareUpdateDialog.vendorFilter = "all"
                            firmwareUpdateDialog.selectedFirmwareIndex = 0
                        }
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
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
                        font.pointSize: 10 * root.fontScale
                        text: qsTr("✔ Official dekuNukem")
                        highlighted: firmwareUpdateDialog.vendorFilter === "official"
                        onClicked: {
                            firmwareUpdateDialog.vendorFilter = "official"
                            firmwareUpdateDialog.selectedFirmwareIndex = 0
                        }
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
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
                        font.pointSize: 10 * root.fontScale
                        icon.name: "internet-services"
                        text: qsTr("Open Official Releases Page (GitHub)")
                        onClicked: Qt.openUrlExternally("https://github.com/dekuNukem/duckyPad-Pro/releases")
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Open https://github.com/dekuNukem/duckyPad-Pro/releases in your web browser")
                    }

                    Item { Layout.fillWidth: true }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
                        icon.name: "document-open"
                        text: firmwareUpdateDialog.customFirmwarePath.length > 0
                            ? qsTr("Custom: %1").arg(firmwareUpdateDialog.selectedTargetLabel())
                            : qsTr("Browse Custom .bin...")
                        onClicked: customFirmwareFileDialog.open()
                    }

                    QQC2.Button {
                        font.pointSize: 10 * root.fontScale
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
                            font.pointSize: 10 * root.fontScale
                            icon.name: "download"
                            text: deviceController.firmwareFetchBusy
                                ? qsTr("Fetching...")
                                : qsTr("⬇ Fetch Official Firmware List")
                            enabled: !deviceController.firmwareFetchBusy
                            highlighted: true
                            onClicked: deviceController.fetchOfficialFirmwaresFromSite()
                        }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
                            icon.name: "internet-services"
                            text: qsTr("Open dekuNukem Releases Page")
                            onClicked: Qt.openUrlExternally("https://github.com/dekuNukem/duckyPad-Pro/releases")
                        }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
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
                    font.pointSize: 10 * root.fontScale
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
                    font.pointSize: 10 * root.fontScale
                    icon.name: "help-contents"
                    text: qsTr("Official Guide")
                    onClicked: deviceController.openFirmwareUpdatePage()
                }

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
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
                    font.pointSize: 10 * root.fontScale
                    icon.name: "window-close"
                    text: qsTr("Close")
                    onClicked: firmwareUpdateDialog.close()
                }
            }
        }
    }

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
                        : (sdSyncDialog.formatUnsyncedSummaryPlainText()
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
                font.pointSize: 10 * root.fontScale
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
                font.pointSize: 10 * root.fontScale
                icon.name: "view-list-details"
                text: qsTr("📋 Changes Summary")
                enabled: !deviceController.sdSyncBusy
                onClicked: sdSyncDialog.openUnsyncedChangesDialog()
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "view-refresh"
                text: qsTr("⬇ Reload from SD")
                enabled: !deviceController.sdSyncBusy && (deviceController.connected || deviceController.sdMountPath.length > 0)
                onClicked: deviceController.reloadProfilesFromDeviceSd()
            }

            Item { Layout.fillWidth: true }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
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
