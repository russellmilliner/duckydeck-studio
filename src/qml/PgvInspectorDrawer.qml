import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Kirigami.OverlayDrawer {
    id: pgvDrawer
    edge: Qt.RightEdge
    modal: true
    width: Math.min(parent.width * 0.85, 620)
    height: parent.height

    property var controller: null
    property string searchFilter: ""
    property int dirtyCount: 0

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
                text: qsTr("Persistent Global Variables (_GV0 – _GV31)")
                Layout.fillWidth: true
            }

            QQC2.Button {
                icon.name: "view-refresh"
                text: qsTr("Read All (0x18)")
                onClicked: pgvDrawer.readAllFromDevice()
            }

            QQC2.Button {
                icon.name: "document-save"
                enabled: pgvDrawer.dirtyCount > 0
                text: qsTr("Write Dirty (%1)").arg(pgvDrawer.dirtyCount)
                onClicked: pgvDrawer.commitDirtyToDevice()
            }

            QQC2.Button {
                text: qsTr("✕ Close")
                onClicked: pgvDrawer.close()
            }
        }

        Kirigami.SearchField {
            id: filterField
            Layout.fillWidth: true
            placeholderText: qsTr("Filter by variable (_GV0.._GV31) or value...")
            onTextChanged: pgvDrawer.searchFilter = text.trim().toLowerCase()
        }

        QQC2.ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

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
                    height: matchesSearch ? 44 : 0
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
                            Layout.preferredWidth: 58
                            text: rowCard.name
                            font.family: "monospace"
                            font.bold: true
                            color: rowCard.dirty ? Kirigami.Theme.neutralTextColor : Kirigami.Theme.highlightColor
                        }

                        QQC2.TextField {
                            Layout.preferredWidth: 120
                            text: rowCard.signedVal.toString()
                            font.family: "monospace"
                            selectByMouse: true
                            onEditingFinished: {
                                const parsed = parseInt(text, 10)
                                if (!isNaN(parsed) && parsed !== rowCard.signedVal) {
                                    pgvDrawer.updateRowValue(rowCard.idx, parsed, true)
                                }
                            }
                        }

                        QQC2.TextField {
                            Layout.preferredWidth: 120
                            text: rowCard.unsignedVal
                            font.family: "monospace"
                            selectByMouse: true
                            onEditingFinished: {
                                const parsed = Number(text)
                                if (!isNaN(parsed) && parsed >= 0 && parsed <= 4294967295) {
                                    pgvDrawer.updateRowValue(rowCard.idx, parsed | 0, true)
                                }
                            }
                        }

                        QQC2.TextField {
                            Layout.preferredWidth: 118
                            text: rowCard.hexVal
                            font.family: "monospace"
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
