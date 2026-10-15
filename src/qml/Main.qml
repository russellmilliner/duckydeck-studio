import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import org.kde.syntaxhighlighting as KSyntax
import io.github.duckydeck 1.0

Kirigami.ApplicationWindow {
    id: root
    title: qsTr("DuckyDeck Studio — %1").arg(deviceController.modelName)
    width: 1280
    height: 780
    minimumWidth: 1024
    minimumHeight: 680

    color: "#13151b"

    DeviceController {
        id: deviceController
        Component.onCompleted: {
            deviceController.refreshDevice()
        }
        onGeneratedScriptPreviewChanged: {
            if (deviceController.generatedScriptPreview.length > 0) {
                scriptEditor.text = deviceController.generatedScriptPreview
            }
        }
    }

    property int selectedSlotNumber: 1
    property string selectedSlotTitle: "Key 1 (Front View)"
    property color selectedSlotColor: "#00aaff"

    function generateSampleScript(slot, title) {
        if (slot >= 1 && slot <= 20) {
            return "// Slot " + slot + ": " + title + "\n" +
                   "DEFAULTDELAY 20\n" +
                   "OLED_CLEAR\n" +
                   "OLED_CPRINT " + title.replace(/Key \d+\s*\(/, "").replace(/\)/, "").trim() + "\n" +
                   "OLED_UPDATE\n" +
                   "STRING // Trigger macro for " + title + "\n" +
                   "ENTER\n"
        } else if (slot >= 21 && slot <= 26) {
            return "// Rotary Encoder Action: " + title + "\n" +
                   "DEFAULTDELAY 15\n" +
                   "STRING // Encoder step\n"
        } else {
            return "// Daisy-Chained Expansion Input: " + title + "\n" +
                   "DEFAULTDELAY 20\n" +
                   "STRING // Expansion triggered\n"
        }
    }

    PgvInspectorDrawer {
        id: pgvDrawer
        controller: deviceController
    }

    AiArchitectModal {
        id: aiModal
        controller: deviceController
    }

    pageStack.initialPage: Kirigami.Page {
        title: deviceController.connected
            ? qsTr("%1 (Connected)").arg(deviceController.modelName)
            : qsTr("%1").arg(deviceController.statusMessage)

        background: Rectangle {
            color: "#13151b"
        }

        actions: [
            Kirigami.Action {
                icon.name: "view-refresh"
                text: qsTr("Scan USB HID")
                tooltip: qsTr("Detect and connect to duckyPad Pro hardware")
                onTriggered: deviceController.refreshDevice()
            },
            Kirigami.Action {
                icon.name: "preferences-system-time"
                text: qsTr("Sync RTC")
                tooltip: qsTr("Synchronize duckyPad RTC hardware clock with system time")
                enabled: deviceController.connected
                onTriggered: deviceController.syncRtcClock()
            },
            Kirigami.Action {
                icon.name: "format-list-ordered"
                text: qsTr("Variables (_GV)")
                tooltip: qsTr("Inspect and modify persistent global variables (_GV0.._GV31)")
                onTriggered: {
                    pgvDrawer.readAllFromDevice()
                    pgvDrawer.open()
                }
            },
            Kirigami.Action {
                icon.name: "system-run"
                text: qsTr("AI Macro Architect")
                tooltip: qsTr("Generate duckyScript using local KDE shortcuts and AI")
                onTriggered: aiModal.open()
            }
        ]

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Kirigami.Units.mediumSpacing
            spacing: Kirigami.Units.smallSpacing

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
                        color: "#2a313d"
                    }
                }

                HardwareCanvas {
                    id: hardwareCanvas
                    QQC2.SplitView.preferredWidth: 476
                    QQC2.SplitView.minimumWidth: 476
                    QQC2.SplitView.maximumWidth: 490
                    QQC2.SplitView.fillHeight: true
                    controller: deviceController

                    onSlotSelected: (slotNumber, slotTitle, slotColor) => {
                        root.selectedSlotNumber = slotNumber
                        root.selectedSlotTitle = slotTitle
                        root.selectedSlotColor = slotColor
                        scriptEditor.text = root.generateSampleScript(slotNumber, slotTitle)
                    }
                }

                // Right Inspector Card with dark theme styling
                Rectangle {
                    QQC2.SplitView.fillWidth: true
                    QQC2.SplitView.fillHeight: true
                    QQC2.SplitView.minimumWidth: 320
                    radius: 12
                    color: "#181b22"
                    border.color: "#2a313d"
                    border.width: 1
                    clip: true

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Kirigami.Units.largeSpacing
                        spacing: Kirigami.Units.smallSpacing

                        // Inspector Header
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 10

                            Rectangle {
                                width: 28
                                height: 28
                                radius: 6
                                color: "#222734"
                                border.color: root.selectedSlotColor
                                border.width: 2

                                QQC2.Label {
                                    anchors.centerIn: parent
                                    text: root.selectedSlotNumber
                                    font.bold: true
                                    font.pixelSize: 11
                                    color: "#f8fafc"
                                }
                            }

                            ColumnLayout {
                                spacing: 1
                                Layout.fillWidth: true

                                QQC2.Label {
                                    id: inspectorTitle
                                    text: qsTr("%1 — duckyScript 3").arg(root.selectedSlotTitle)
                                    font.bold: true
                                    font.pixelSize: 14
                                    color: "#f8fafc"
                                    Layout.fillWidth: true
                                    elide: Text.ElideRight
                                }

                                QQC2.Label {
                                    text: qsTr("RGB LED: %1 • Target Slot: %2").arg(root.selectedSlotColor).arg(root.selectedSlotNumber)
                                    font.pixelSize: 11
                                    color: "#94a3b8"
                                }
                            }

                            QQC2.Button {
                                icon.name: "system-run"
                                text: qsTr("AI Prompt")
                                onClicked: aiModal.open()
                            }
                        }

                        // Snippet Quick Insert Bar
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 6

                            QQC2.Label {
                                text: qsTr("Snippets:")
                                font.pixelSize: 11
                                font.bold: true
                                color: "#64748b"
                            }

                            QQC2.Button {
                                text: "+ DELAY"
                                implicitHeight: 26
                                font.pixelSize: 11
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "DELAY 100\n")
                            }

                            QQC2.Button {
                                text: "+ STRING"
                                implicitHeight: 26
                                font.pixelSize: 11
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "STRING Hello World\n")
                            }

                            QQC2.Button {
                                text: "+ ENTER"
                                implicitHeight: 26
                                font.pixelSize: 11
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "ENTER\n")
                            }

                            QQC2.Button {
                                text: "+ OLED"
                                implicitHeight: 26
                                font.pixelSize: 11
                                onClicked: scriptEditor.insert(scriptEditor.cursorPosition, "OLED_CLEAR\nOLED_CPRINT Active\nOLED_UPDATE\n")
                            }

                            Item { Layout.fillWidth: true }
                        }

                        // Code Editor Area
                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: 8
                            color: "#0d1017"
                            border.color: "#252b38"
                            border.width: 1

                            QQC2.ScrollView {
                                anchors.fill: parent
                                anchors.margins: 8
                                clip: true

                                QQC2.TextArea {
                                    id: scriptEditor
                                    font.family: "monospace"
                                    font.pixelSize: 14
                                    background: Rectangle { color: "transparent" }
                                    color: "#f1f5f9"
                                    selectionColor: "#0284c7"
                                    selectedTextColor: "#ffffff"
                                    selectByMouse: true
                                    wrapMode: TextEdit.NoWrap
                                    text: deviceController.generatedScriptPreview

                                    KSyntax.SyntaxHighlighter {
                                        textEdit: scriptEditor
                                        definition: "duckyScript 3"
                                        repository: KSyntax.Repository
                                        theme: {
                                            const darkThemes = ["Catppuccin Mocha", "Dracula", "Atom One Dark", "Breeze Dark"]
                                            for (let i = 0; i < darkThemes.length; ++i) {
                                                const th = KSyntax.Repository.theme(darkThemes[i])
                                                if (th && th.name) return th
                                            }
                                            return KSyntax.Repository.defaultTheme(KSyntax.Repository.DarkTheme)
                                        }
                                    }
                                }
                            }
                        }

                        // Inspector Footer Bar
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: Kirigami.Units.smallSpacing

                            QQC2.Label {
                                text: qsTr("duckyScript 3 Engine Ready")
                                font.pixelSize: 11
                                color: "#64748b"
                                Layout.fillWidth: true
                            }

                            QQC2.Button {
                                icon.name: "edit-clear"
                                text: qsTr("Reset")
                                onClicked: scriptEditor.text = root.generateSampleScript(root.selectedSlotNumber, root.selectedSlotTitle)
                            }

                            QQC2.Button {
                                icon.name: "document-save"
                                text: qsTr("Assign to Key %1").arg(root.selectedSlotNumber)
                                highlighted: true
                                onClicked: {
                                    deviceController.statusMessage = qsTr("Saved script to Slot %1").arg(root.selectedSlotNumber)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
