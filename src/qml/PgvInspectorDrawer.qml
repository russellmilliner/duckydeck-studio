import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.Drawer {
    id: pgvDrawer
    edge: Qt.RightEdge
    modal: true
    interactive: true
    closePolicy: QQC2.Popup.CloseOnEscape | QQC2.Popup.CloseOnPressOutside
    padding: Kirigami.Units.largeSpacing
    width: parent ? Math.min(parent.width * 0.85, Math.round(620 * Math.max(1.0, pgvDrawer.fontScale))) : 560
    height: parent ? parent.height : 600

    Kirigami.Theme.colorSet: Kirigami.Theme.View
    Kirigami.Theme.inherit: false

    background: Rectangle {
        color: Kirigami.Theme.backgroundColor
        Rectangle {
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            width: 1
            color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        }
    }

    property var controller: null
    readonly property bool closableWithEscape: true
    readonly property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property string searchFilter: ""
    property int dirtyCount: 0

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: pgvDrawer.opened || pgvDrawer.visible
        onActivated: pgvDrawer.close()
        onAmbiguouslyActivated: pgvDrawer.close()
    }

    ListModel {
        id: pgvModel
        Component.onCompleted: {
            for (let i = 0; i < 32; ++i) {
                append({
                    idx: i,
                    name: "_GV" + i,
                    signedVal: 0,
                    unsignedVal: "0",
                    hexVal: "0x00000000",
                    dirty: false
                })
            }
        }
    }

    function toUnsignedString(s32) {
        return (s32 >>> 0).toString(10)
    }

    function toHexString(s32) {
        const u32 = (s32 >>> 0)
        return "0x" + u32.toString(16).toUpperCase().padStart(8, "0")
    }

    function clampSigned32(n) {
        return (n | 0)
    }

    function updateRowValue(index, newSigned32, markDirty) {
        const s32 = clampSigned32(newSigned32)
        const wasDirty = pgvModel.get(index).dirty
        pgvModel.setProperty(index, "signedVal", s32)
        pgvModel.setProperty(index, "unsignedVal", toUnsignedString(s32))
        pgvModel.setProperty(index, "hexVal", toHexString(s32))
        pgvModel.setProperty(index, "dirty", markDirty)

        if (!wasDirty && markDirty) {
            pgvDrawer.dirtyCount += 1
        } else if (wasDirty && !markDirty) {
            pgvDrawer.dirtyCount = Math.max(0, pgvDrawer.dirtyCount - 1)
        }
    }

    function readAllFromDevice() {
        if (!controller) return
        if (typeof controller.dumpAllPgvsJson === "function") {
            const rawJson = controller.dumpAllPgvsJson()
            try {
                const arr = JSON.parse(rawJson)
                if (Array.isArray(arr) && arr.length === 32) {
                    pgvDrawer.dirtyCount = 0
                    for (let i = 0; i < 32; ++i) {
                        updateRowValue(i, arr[i], false)
                    }
                    return
                }
            } catch (e) {}
        }

        pgvDrawer.dirtyCount = 0
        for (let i = 0; i < 32; ++i) {
            const val = controller.readPgv(i)
            updateRowValue(i, val, false)
        }
    }

    function commitDirtyToDevice() {
        if (!controller) return
        for (let i = 0; i < 32; ++i) {
            const row = pgvModel.get(i)
            if (row.dirty) {
                if (controller.writePgv(i, row.signedVal)) {
                    pgvModel.setProperty(i, "dirty", false)
                    pgvDrawer.dirtyCount = Math.max(0, pgvDrawer.dirtyCount - 1)
                }
            }
        }
    }

    contentItem: ColumnLayout {
        spacing: Kirigami.Units.mediumSpacing

        RowLayout {
            Layout.fillWidth: true
            spacing: Kirigami.Units.smallSpacing

            Kirigami.Heading {
                level: 2
                font.pointSize: 12 * pgvDrawer.fontScale
                text: qsTr("Persistent Global Variables (_GV0 – _GV31)")
                elide: Text.ElideRight
                Layout.fillWidth: true
            }

            QQC2.Button {
                icon.name: "view-refresh"
                text: qsTr("Read All (0x18)")
                font.pointSize: 10 * pgvDrawer.fontScale
                onClicked: pgvDrawer.readAllFromDevice()
            }

            QQC2.Button {
                icon.name: "document-save"
                enabled: pgvDrawer.dirtyCount > 0
                text: qsTr("Write Dirty (%1)").arg(pgvDrawer.dirtyCount)
                font.pointSize: 10 * pgvDrawer.fontScale
                onClicked: pgvDrawer.commitDirtyToDevice()
            }

            QQC2.Button {
                text: qsTr("✕ Close")
                font.pointSize: 10 * pgvDrawer.fontScale
                onClicked: pgvDrawer.close()
            }
        }

        Kirigami.SearchField {
            id: filterField
            Layout.fillWidth: true
            font.pointSize: 10 * pgvDrawer.fontScale
            placeholderText: qsTr("Filter by variable (_GV0.._GV31) or value...")
            onTextChanged: pgvDrawer.searchFilter = text.trim().toLowerCase()
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

        QQC2.ScrollView {
            id: pgvScrollView
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            QQC2.ScrollBar.vertical: HighContrastScrollBar {
                parent: pgvScrollView
                policy: pgvListView.contentHeight > pgvListView.height
                    ? QQC2.ScrollBar.AlwaysOn
                    : QQC2.ScrollBar.AsNeeded
            }
            QQC2.ScrollBar.horizontal: HighContrastScrollBar {
                parent: pgvScrollView
                orientation: Qt.Horizontal
            }

            ListView {
                id: pgvListView
                model: pgvModel
                spacing: 6

                delegate: Rectangle {
                    id: rowCard
                    required property int index
                    required property int idx
                    required property string name
                    required property int signedVal
                    required property string unsignedVal
                    required property string hexVal
                    required property bool dirty

                    readonly property bool matchesSearch: {
                        if (pgvDrawer.searchFilter === "") return true
                        return name.toLowerCase().indexOf(pgvDrawer.searchFilter) !== -1
                            || signedVal.toString().indexOf(pgvDrawer.searchFilter) !== -1
                            || hexVal.toLowerCase().indexOf(pgvDrawer.searchFilter) !== -1
                    }

                    visible: matchesSearch
                    width: pgvListView.width - 12
                    height: matchesSearch ? Math.round(44 * pgvDrawer.fontScale) : 0
                    radius: 6
                    color: dirty
                        ? Kirigami.Theme.neutralBackgroundColor
                        : (idx % 2 === 0 ? Kirigami.Theme.backgroundColor : Kirigami.Theme.alternateBackgroundColor)
                    border.width: dirty ? 1.5 : 1
                    border.color: dirty
                        ? Kirigami.Theme.neutralTextColor
                        : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 10
                        anchors.rightMargin: 10
                        spacing: 8

                        QQC2.Label {
                            Layout.preferredWidth: Math.round(64 * pgvDrawer.fontScale)
                            text: rowCard.name
                            font.family: "monospace"
                            font.bold: true
                            font.pointSize: 10 * pgvDrawer.fontScale
                            color: rowCard.dirty
                                ? Kirigami.Theme.neutralTextColor
                                : ((0.2126 * Kirigami.Theme.backgroundColor.r + 0.7152 * Kirigami.Theme.backgroundColor.g + 0.0722 * Kirigami.Theme.backgroundColor.b) > 0.45
                                    ? "#0369a1"
                                    : Kirigami.Theme.highlightColor)
                        }

                        QQC2.TextField {
                            Layout.preferredWidth: Math.round(120 * pgvDrawer.fontScale)
                            text: rowCard.signedVal.toString()
                            font.family: "monospace"
                            font.pointSize: 10 * pgvDrawer.fontScale
                            selectByMouse: true
                            onEditingFinished: {
                                const parsed = parseInt(text, 10)
                                if (!isNaN(parsed) && parsed !== rowCard.signedVal) {
                                    pgvDrawer.updateRowValue(rowCard.idx, parsed, true)
                                }
                            }
                        }

                        QQC2.TextField {
                            Layout.preferredWidth: Math.round(120 * pgvDrawer.fontScale)
                            text: rowCard.unsignedVal
                            font.family: "monospace"
                            font.pointSize: 10 * pgvDrawer.fontScale
                            selectByMouse: true
                            onEditingFinished: {
                                const parsed = Number(text)
                                if (!isNaN(parsed) && parsed >= 0 && parsed <= 4294967295) {
                                    pgvDrawer.updateRowValue(rowCard.idx, parsed | 0, true)
                                }
                            }
                        }

                        QQC2.TextField {
                            Layout.preferredWidth: Math.round(118 * pgvDrawer.fontScale)
                            text: rowCard.hexVal
                            font.family: "monospace"
                            font.pointSize: 10 * pgvDrawer.fontScale
                            selectByMouse: true
                            onEditingFinished: {
                                const cleaned = text.trim().replace(/^0x/i, "")
                                const parsed = parseInt(cleaned, 16)
                                if (!isNaN(parsed)) {
                                    pgvDrawer.updateRowValue(rowCard.idx, parsed | 0, true)
                                }
                            }
                        }

                        Item { Layout.fillWidth: true }

                        QQC2.ToolButton {
                            icon.name: "upload"
                            enabled: rowCard.dirty
                            onClicked: {
                                if (pgvDrawer.controller && pgvDrawer.controller.writePgv(rowCard.idx, rowCard.signedVal)) {
                                    pgvModel.setProperty(rowCard.idx, "dirty", false)
                                    pgvDrawer.dirtyCount = Math.max(0, pgvDrawer.dirtyCount - 1)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
