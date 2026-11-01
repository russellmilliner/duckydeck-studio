import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.Dialog {
    id: tutorialsDialog
    objectName: "tutorialsDialog"
    readonly property bool closableWithEscape: true
    enter: null
    exit: null

    property var root: null
    property var controller: null
    property real fontScale: root ? root.fontScale : 1.0
    property bool isLightTheme: root ? root.isLightTheme : false
    property int currentTopicIndex: 0
    property string copiedFeedbackText: ""

    parent: QQC2.Overlay.overlay
    anchors.centerIn: parent
    modal: true
    padding: 14
    topPadding: 10
    bottomPadding: 12
    width: root ? Math.min(Math.max(820, Math.round(920 * fontScale)), root.width - 28) : 900
    height: root ? Math.min(Math.max(600, Math.round(690 * fontScale)), root.height - 28) : 660
    title: qsTr("🎓 DuckyDeck Studio — Interactive Tutorials & Step-by-Step Guides")
    standardButtons: QQC2.Dialog.NoButton

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: tutorialsDialog.visible
        onActivated: tutorialsDialog.close()
    }

    Timer {
        id: copiedFeedbackTimer
        interval: 2500
        repeat: false
        onTriggered: tutorialsDialog.copiedFeedbackText = ""
    }

    function openTopic(topicId) {
        const map = {
            "oled_vm": 0,
            "home_assistant": 1,
            "webhooks": 2,
            "app_launch": 3,
            "autoswitch": 4,
            "multiclick": 5,
            "gv_rgb": 6
        }
        if (topicId && map[topicId] !== undefined) {
            tutorialsDialog.currentTopicIndex = map[topicId]
        }
        tutorialsDialog.open()
    }

    function copyTextToClipboard(textStr, labelStr) {
        if (tutorialsDialog.controller) {
            tutorialsDialog.controller.copyToClipboard(textStr)
        }
        tutorialsDialog.copiedFeedbackText = qsTr("✓ Copied %1 to clipboard").arg(labelStr || qsTr("snippet"))
        copiedFeedbackTimer.restart()
    }

    function tryOledExampleInEditor(scriptCode) {
        if (!tutorialsDialog.root) return
        const r = tutorialsDialog.root
        if (!r.hasSelectedSlot && r.mainHardwareCanvas) {
            r.mainHardwareCanvas.selectedSlot = 1
            r.loadSlotFromController(
                1,
                r.mainHardwareCanvas.slotTitleFor(1),
                r.mainHardwareCanvas.slotColorFor(1)
            )
        }
        if (r.mainHardwareCanvas) {
            r.mainHardwareCanvas.oledScriptMode = true
        }
        r.inspectorMode = "code"
        r.insertCommandFromReference(scriptCode)
        tutorialsDialog.copiedFeedbackText = qsTr("✓ Inserted into Key #%1 & switched PCB preview to OLED VM HUD!").arg(r.selectedSlotNumber || 1)
        copiedFeedbackTimer.restart()
    }

    component TutorialScrollBar: QQC2.ScrollBar {
        id: sbControl
        objectName: "highContrastScrollBar"
        implicitWidth: orientation === Qt.Vertical ? 12 : 100
        implicitHeight: orientation === Qt.Vertical ? 100 : 12
        padding: 2
        policy: size < 1.0 ? QQC2.ScrollBar.AlwaysOn : QQC2.ScrollBar.AsNeeded
        minimumSize: 0.14

        background: Rectangle {
            radius: 5
            color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
            border.width: 1
            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.28)
            visible: sbControl.size < 1.0
        }

        contentItem: Rectangle {
            implicitWidth: sbControl.orientation === Qt.Vertical ? 8 : 44
            implicitHeight: sbControl.orientation === Qt.Vertical ? 44 : 8
            radius: 4
            visible: sbControl.size < 1.0
            color: sbControl.pressed
                ? Kirigami.Theme.highlightColor
                : (sbControl.hovered
                    ? Qt.lighter(Kirigami.Theme.highlightColor, 1.15)
                    : Qt.alpha(Kirigami.Theme.textColor, 0.65))
            border.width: 1
            border.color: Qt.alpha(Kirigami.Theme.backgroundColor, 0.55)
        }
    }

    component SectionCard: Rectangle {
        id: cardRoot
        property string heading: ""
        property string badgeText: ""
        property color badgeColor: Kirigami.Theme.highlightColor
        default property alias bodyContent: bodyCol.data

        Layout.fillWidth: true
        implicitHeight: cardInnerCol.implicitHeight + 24
        radius: 8
        color: Kirigami.Theme.alternateBackgroundColor
        border.width: 1
        border.color: Qt.alpha(Kirigami.Theme.textColor, 0.22)

        ColumnLayout {
            id: cardInnerCol
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.margins: 12
            spacing: 8

            RowLayout {
                Layout.fillWidth: true
                spacing: 8
                visible: cardRoot.heading.length > 0

                Rectangle {
                    visible: cardRoot.badgeText.length > 0
                    radius: 4
                    color: Qt.alpha(cardRoot.badgeColor, 0.18)
                    border.width: 1
                    border.color: Qt.alpha(cardRoot.badgeColor, 0.65)
                    implicitWidth: badgeLbl.implicitWidth + 12
                    implicitHeight: badgeLbl.implicitHeight + 4

                    QQC2.Label {
                        id: badgeLbl
                        anchors.centerIn: parent
                        text: cardRoot.badgeText
                        font.bold: true
                        font.pointSize: 9 * tutorialsDialog.fontScale
                        color: Kirigami.Theme.textColor
                    }
                }

                QQC2.Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: cardRoot.heading
                    font.bold: true
                    font.pointSize: 11 * tutorialsDialog.fontScale
                    color: Kirigami.Theme.textColor
                }
            }

            ColumnLayout {
                id: bodyCol
                Layout.fillWidth: true
                spacing: 6
            }
        }
    }

    component CodeSampleBox: Rectangle {
        id: codeBox
        property string title: ""
        property string code: ""
        property bool canPreviewOled: false

        Layout.fillWidth: true
        implicitHeight: codeCol.implicitHeight + 16
        radius: 6
        color: Kirigami.Theme.backgroundColor
        border.width: 1
        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.45)

        ColumnLayout {
            id: codeCol
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.margins: 8
            spacing: 6

            RowLayout {
                Layout.fillWidth: true
                spacing: 6

                QQC2.Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: codeBox.title
                    font.bold: true
                    font.pointSize: 9.5 * tutorialsDialog.fontScale
                    color: Kirigami.Theme.highlightColor
                }

                QQC2.Button {
                    visible: codeBox.canPreviewOled
                    icon.name: "video-television"
                    text: qsTr("⚡ Insert & Preview on OLED VM")
                    implicitHeight: Math.round(24 * tutorialsDialog.fontScale)
                    font.pointSize: 9 * tutorialsDialog.fontScale
                    highlighted: true
                    onClicked: tutorialsDialog.tryOledExampleInEditor(codeBox.code)
                }

                QQC2.Button {
                    icon.name: "edit-copy"
                    text: qsTr("Copy")
                    implicitHeight: Math.round(24 * tutorialsDialog.fontScale)
                    font.pointSize: 9 * tutorialsDialog.fontScale
                    onClicked: tutorialsDialog.copyTextToClipboard(codeBox.code, codeBox.title)
                }
            }

            QQC2.Label {
                Layout.fillWidth: true
                wrapMode: Text.WrapAnywhere
                text: codeBox.code
                font.family: "monospace"
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                color: Kirigami.Theme.textColor
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 8

        // Standard KDE Plasma UI TabBar (Rule 7.6)
        QQC2.TabBar {
            id: tutorialTabBar
            Layout.fillWidth: true
            currentIndex: tutorialsDialog.currentTopicIndex
            onCurrentIndexChanged: {
                if (tutorialsDialog.currentTopicIndex !== currentIndex) {
                    tutorialsDialog.currentTopicIndex = currentIndex
                }
            }

            QQC2.TabButton {
                text: qsTr("🖥️ OLED VM & HUD")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                width: implicitWidth
            }
            QQC2.TabButton {
                text: qsTr("🏠 Home Assistant")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                width: implicitWidth
            }
            QQC2.TabButton {
                text: qsTr("🌐 Webhooks & API")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                width: implicitWidth
            }
            QQC2.TabButton {
                text: qsTr("🚀 App Launching")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                width: implicitWidth
            }
            QQC2.TabButton {
                text: qsTr("🪟 Auto-Switch & Layers")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                width: implicitWidth
            }
            QQC2.TabButton {
                text: qsTr("⚡ Multi-Click & Hold")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                width: implicitWidth
            }
            QQC2.TabButton {
                text: qsTr("🔢 Variables & RGB")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                width: implicitWidth
            }
        }

        // Scrollable Tutorial Content Area
        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: tutorialsDialog.currentTopicIndex

            // =================================================================
            // TAB 0: OLED VM & OLED VM HUD
            // =================================================================
            Flickable {
                id: oledTabFlick
                clip: true
                contentWidth: width - 18
                contentHeight: oledCol.implicitHeight + 16
                boundsBehavior: Flickable.StopAtBounds
                QQC2.ScrollBar.vertical: TutorialScrollBar {
                    parent: oledTabFlick
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                }

                ColumnLayout {
                    id: oledCol
                    width: oledTabFlick.width - 18
                    spacing: 10

                    SectionCard {
                        badgeText: qsTr("CONCEPT")
                        heading: qsTr("What Are 'OLED Grid', 'OLED VM', and the 'OLED VM HUD'?")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("Your physical duckyPad has a 128×128 pixel monochrome OLED screen (or 128×64 on the 2020 model). That screen operates in two ways:\n\n" +
                                "• 1. OLED Grid Mode (Default): When idle, the duckyPad firmware automatically draws the active Profile Name at the top and a 4×5 (or 3×5) grid of your 2-line key labels.\n\n" +
                                "• 2. OLED VM Mode (Custom Script Drawing): Inside any key's DuckyScript 3 macro, you can write OLED_* commands (such as OLED_CLEAR, OLED_PRINT, OLED_RECT, OLED_CIRCLE, OLED_LINE) that run on the duckyPad's onboard bytecode virtual machine ('duckStack VM'). When you press that key, your script takes over the OLED screen to draw custom status banners, mute indicators, progress bars, or live variable counters!\n\n" +
                                "• 3. What the 'OLED Grid / OLED VM' button in DuckyDeck Studio does: Located in the top-right corner of the virtual PCB, toggling 'OLED Grid' → 'OLED VM' switches the simulated OLED screen on the PCB into the OLED VM HUD. It live-renders and previews the OLED_* drawing commands in your currently selected key's script as you type—without needing to sync to the microSD card first!")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("STEP-BY-STEP")
                        heading: qsTr("How to Write & Preview Custom OLED Screen Graphics")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("1. Click any key (1–20) or rotary encoder action (21–26) on the Hardware Canvas.\n" +
                                "2. In the Key Editor on the right, select the '📜 DuckyScript IDE' tab.\n" +
                                "3. Click the '+OLED' button in the script editor toolbar (or click '⚡ Insert & Preview on OLED VM' on any example below).\n" +
                                "4. Notice the button at the top-right of the Hardware Canvas switches to 'OLED VM', and the simulated 128×128 OLED screen immediately previews your script's OLED drawing commands!\n" +
                                "5. Always finish a batch of drawing commands with 'OLED_UPDATE' to push the pixels to the screen. If you want the screen to return to the normal 20-key label grid after a pause, add 'DELAY 1500' followed by 'OLED_RESTORE'.")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("REFERENCE")
                        heading: qsTr("DuckyScript 3 OLED_* Commands Cheat Sheet (128×128 Pro / 128×64 2020)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.family: "monospace"
                            font.pointSize: 9.5 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: "• OLED_CLEAR                     — Erase the OLED framebuffer to black\n" +
                                  "• OLED_CURSOR <x> <y>            — Move text cursor to pixel (x: 0..127, y: 0..127)\n" +
                                  "• OLED_PRINT <text>              — Print text at cursor (supports $_GV0..$_GV30 variables)\n" +
                                  "• OLED_CPRINT <text>             — Print horizontally centered text at current cursor Y\n" +
                                  "• OLED_LPRINT <text>             — Print text with automatic line wrapping\n" +
                                  "• OLED_LINE <x1> <y1> <x2> <y2>  — Draw a straight line between two points\n" +
                                  "• OLED_RECT <x1> <y1> <x2> <y2> <opt> — Draw rectangle (opt: 0=erase, 1=outline, 2=filled)\n" +
                                  "• OLED_CIRCLE <x> <y> <r> <opt>  — Draw circle at (x,y) radius r (1=outline, 2=filled)\n" +
                                  "• OLED_PIX <x> <y> <opt>         — Set a single pixel (0=black, 1=white)\n" +
                                  "• OLED_UPDATE                    — Push all drawn commands to the physical OLED screen\n" +
                                  "• OLED_RESTORE                   — Restore the default profile key-label grid view"
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("EXAMPLES")
                        heading: qsTr("Ready-to-Use OLED VM Scripts (Click 'Insert & Preview' to Try!)")

                        CodeSampleBox {
                            title: qsTr("Example 1: Custom Macro Confirmation Banner (Shows for 1.5s, then returns to Grid)")
                            canPreviewOled: true
                            code: "REM Show a custom bordered HUD banner while running a build macro\n" +
                                  "OLED_CLEAR\n" +
                                  "OLED_RECT 2 2 125 125 1\n" +
                                  "OLED_LINE 2 26 125 26\n" +
                                  "OLED_CURSOR 0 10\n" +
                                  "OLED_CPRINT 🚀 DEPLOYING\n" +
                                  "OLED_CURSOR 0 52\n" +
                                  "OLED_CPRINT Running cargo\n" +
                                  "OLED_CURSOR 0 68\n" +
                                  "OLED_CPRINT build --release\n" +
                                  "OLED_CIRCLE 64 102 10 1\n" +
                                  "OLED_UPDATE\n" +
                                  "DELAY 1500\n" +
                                  "OLED_RESTORE"
                        }

                        CodeSampleBox {
                            title: qsTr("Example 2: Persistent 'MIC MUTED' Warning Screen (Stays on OLED while muted via _GV0)")
                            canPreviewOled: true
                            code: "IF $_GV0 == 0 THEN\n" +
                                  "    $_GV0 = 1\n" +
                                  "    SWCF 1 255 0 0\n" +
                                  "    OLED_CLEAR\n" +
                                  "    OLED_RECT 4 4 123 123 1\n" +
                                  "    OLED_CIRCLE 64 46 20 1\n" +
                                  "    OLED_LINE 50 32 78 60\n" +
                                  "    OLED_CURSOR 0 80\n" +
                                  "    OLED_CPRINT MICROPHONE\n" +
                                  "    OLED_CURSOR 0 96\n" +
                                  "    OLED_CPRINT [ MUTED ]\n" +
                                  "    OLED_UPDATE\n" +
                                  "ELSE\n" +
                                  "    $_GV0 = 0\n" +
                                  "    SWCF 1 0 255 80\n" +
                                  "    OLED_RESTORE\n" +
                                  "END_IF"
                        }

                        CodeSampleBox {
                            title: qsTr("Example 3: Live Counter / Volume HUD Gauge with Variable Readout ($_GV1)")
                            canPreviewOled: true
                            code: "$_GV1 = $_GV1 + 10\n" +
                                  "IF $_GV1 > 100 THEN\n" +
                                  "    $_GV1 = 0\n" +
                                  "END_IF\n" +
                                  "OLED_CLEAR\n" +
                                  "OLED_RECT 2 2 125 125 1\n" +
                                  "OLED_CURSOR 0 14\n" +
                                  "OLED_CPRINT LEVEL HUD\n" +
                                  "OLED_LINE 10 30 118 30\n" +
                                  "OLED_CURSOR 0 52\n" +
                                  "OLED_CPRINT Value: $_GV1 %\n" +
                                  "OLED_RECT 14 78 114 96 1\n" +
                                  "OLED_RECT 16 80 84 94 2\n" +
                                  "OLED_UPDATE"
                        }
                    }
                }
            }

            // =================================================================
            // TAB 1: HOME ASSISTANT CONFIGURATION
            // =================================================================
            Flickable {
                id: haTabFlick
                clip: true
                contentWidth: width - 18
                contentHeight: haCol.implicitHeight + 16
                boundsBehavior: Flickable.StopAtBounds
                QQC2.ScrollBar.vertical: TutorialScrollBar {
                    parent: haTabFlick
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                }

                ColumnLayout {
                    id: haCol
                    width: haTabFlick.width - 18
                    spacing: 10

                    SectionCard {
                        badgeText: qsTr("STEP 1")
                        heading: qsTr("Create a Long-Lived Access Token in Home Assistant")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("1. Open your Home Assistant web interface in your browser (e.g. http://homeassistant.local:8123).\n" +
                                "2. Click your User Profile badge at the very bottom of the left sidebar.\n" +
                                "3. Click the 'Security' tab at the top of your profile page.\n" +
                                "4. Scroll to the bottom section titled 'Long-Lived Access Tokens' and click 'Create Token'.\n" +
                                "5. Name it 'DuckyDeck Studio' and copy the generated token string immediately.")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("STEP 2")
                        heading: qsTr("Connect DuckyDeck Studio & Browse Your Smart Home Entities")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("1. Click any key on the Hardware Canvas to open the Key Editor on the right.\n" +
                                "2. Switch to the '🌐 Web API' tab and click 'Configure Web API...'.\n" +
                                "3. Select the '🏠 Home Assistant' mode tab.\n" +
                                "4. Enter your Home Assistant Base URL (e.g. http://192.168.1.50:8123 or https://ha.example.ts.net) and paste your Long-Lived Access Token.\n" +
                                "5. Click 'Save & Fetch Entities'. DuckyDeck Studio connects to your Home Assistant /api/states endpoint and caches all your lights, switches, scenes, scripts, automations, fans, covers, locks, and media players!")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("STEP 3")
                        heading: qsTr("Bind an Entity to a Key & Test It Live")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("1. Filter by domain pill (e.g. 'light', 'switch', 'scene', 'script', 'automation') or type in the search box (e.g. 'Office Lamp').\n" +
                                "2. Select the entity from the list.\n" +
                                "3. Choose the action service (such as 'toggle', 'turn_on', 'turn_off', or 'trigger').\n" +
                                "4. Optional: Pass extra JSON service data (for example, {\"brightness_pct\": 80, \"kelvin\": 4000} for a light).\n" +
                                "5. Click '⚡ Test Call' to verify your light/device responds immediately, then click 'Save / Apply to Key'.\n" +
                                "6. Click '⬆ Sync Changes to duckyPad SD' in the menu so the physical key triggers your Home Assistant action!")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("ARCHITECTURE")
                        heading: qsTr("How It Works: Host USB Bridge vs. Standalone ESP32-S3 Wi-Fi Bridge")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("• Host USB Bridge (Default): Your key displays an amber 'API' badge. When pressed, the duckyPad writes a silent trigger ID into Global Variable _GV31 over USB HID, and DuckyDeck Studio (running in your system tray) executes the HTTP POST to Home Assistant.\n\n" +
                                "• Optional ESP32-S3 Standalone Wi-Fi Bridge: If you run an ESP32-S3 USB Host bridge ('duckypad-esp32-ha'), open Menu ☰ → 'Switch Managed duckyPad' → 'Configure ESP32-HA Bridge...' to sync your profiles directly to the ESP32 over Wi-Fi so Home Assistant keys work even when your PC is powered off!")
                        }
                    }
                }
            }

            // =================================================================
            // TAB 2: WEBHOOKS & HTTP REST API
            // =================================================================
            Flickable {
                id: webhooksTabFlick
                clip: true
                contentWidth: width - 18
                contentHeight: webhooksCol.implicitHeight + 16
                boundsBehavior: Flickable.StopAtBounds
                QQC2.ScrollBar.vertical: TutorialScrollBar {
                    parent: webhooksTabFlick
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                }

                ColumnLayout {
                    id: webhooksCol
                    width: webhooksTabFlick.width - 18
                    spacing: 10

                    SectionCard {
                        badgeText: qsTr("OUTBOUND KEYS")
                        heading: qsTr("1. Outbound Key-Press Webhooks (n8n, Node-RED, Discord, Custom APIs)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("Any key can fire an HTTP request (GET, POST, PUT, DELETE) when pressed:\n" +
                                "1. Select a key → click the '🌐 Web API' tab → click 'Configure Web API...'.\n" +
                                "2. Choose '🌐 Custom HTTP Webhook / REST API'.\n" +
                                "3. Pick the HTTP Method (e.g. POST) and enter the destination URL (e.g. an n8n webhook URL or local service).\n" +
                                "4. Add optional Authentication (Bearer Token or custom Header Name + Value like X-API-Key) and a JSON Body payload.\n" +
                                "5. Click '⚡ Test Call' to inspect the live HTTP status code and response body.")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("PROFILE EVENTS")
                        heading: qsTr("2. Per-Profile & Global Profile-Change Webhooks")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("Want your smart lights, OBS scene, or status display to react automatically whenever your duckyPad changes profiles?\n\n" +
                                "• Per-Profile Webhook: Right-click any profile in the left sidebar → 'Properties...' → enable 'Trigger Webhook on Profile Activation'.\n" +
                                "• Global Profile-Change Webhook: Open Menu ☰ → 'Settings...' → 'REST API & Webhooks' → enable 'Global Profile-Change Webhook'.\n\n" +
                                "Both URL and JSON Body support automatic template placeholders:")
                        }

                        CodeSampleBox {
                            title: qsTr("Example Profile-Change Webhook JSON Payload")
                            code: "{\n" +
                                  "  \"event\": \"profile_changed\",\n" +
                                  "  \"profile_number\": {{profile_number}},\n" +
                                  "  \"profile_name\": \"{{profile_name}}\",\n" +
                                  "  \"previous_profile\": \"{{previous_profile_name}}\",\n" +
                                  "  \"source\": \"{{source}}\",\n" +
                                  "  \"timestamp\": \"{{timestamp}}\"\n" +
                                  "}"
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("INBOUND API")
                        heading: qsTr("3. Inbound Control via DuckyDeck Studio's Built-In HTTP REST API (Port 28491)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("External scripts, Home Assistant automations, or n8n workflows can remotely control your duckyPad!\n" +
                                "1. Open Menu ☰ → 'Settings...' → 'REST API & Webhooks'.\n" +
                                "2. Enable the Local HTTP REST API Server (default 127.0.0.1:28491) and copy your Bearer API Token.\n" +
                                "3. Call any endpoint below using curl, Python, or Home Assistant rest_command:")
                        }

                        CodeSampleBox {
                            title: qsTr("Inbound REST API Examples (curl)")
                            code: "# 1. Check status & active profile\n" +
                                  "curl -H \"Authorization: Bearer YOUR_TOKEN\" http://127.0.0.1:28491/api/v1/status\n\n" +
                                  "# 2. Switch duckyPad to Profile #3 (or by name: {\"name\": \"Gaming\"})\n" +
                                  "curl -X POST http://127.0.0.1:28491/api/v1/profile/select \\\n" +
                                  "  -H \"Authorization: Bearer YOUR_TOKEN\" -H \"Content-Type: application/json\" \\\n" +
                                  "  -d '{\"profile\": 3}'\n\n" +
                                  "# 3. Set Key #1 LED to Red and update Global Variable _GV0 = 1 (e.g. from Home Assistant)\n" +
                                  "curl -X POST http://127.0.0.1:28491/api/v1/leds/set \\\n" +
                                  "  -H \"Authorization: Bearer YOUR_TOKEN\" -H \"Content-Type: application/json\" \\\n" +
                                  "  -d '{\"key\": 1, \"r\": 255, \"g\": 0, \"b\": 0}'\n" +
                                  "curl -X POST http://127.0.0.1:28491/api/v1/gv/set \\\n" +
                                  "  -H \"Authorization: Bearer YOUR_TOKEN\" -H \"Content-Type: application/json\" \\\n" +
                                  "  -d '{\"index\": 0, \"value\": 1}'"
                        }
                    }
                }
            }

            // =================================================================
            // TAB 3: APP LAUNCHING (APP BUILDER)
            // =================================================================
            Flickable {
                id: appTabFlick
                clip: true
                contentWidth: width - 18
                contentHeight: appCol.implicitHeight + 16
                boundsBehavior: Flickable.StopAtBounds
                QQC2.ScrollBar.vertical: TutorialScrollBar {
                    parent: appTabFlick
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                }

                ColumnLayout {
                    id: appCol
                    width: appTabFlick.width - 18
                    spacing: 10

                    SectionCard {
                        badgeText: qsTr("STEP-BY-STEP")
                        heading: qsTr("How to Launch Desktop Applications from Any Key")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("1. Select a key (1–20) on the Hardware Canvas.\n" +
                                "2. In the Key Editor on the right, select the '🚀 Launch App' tab and click 'Configure Launch App...'.\n" +
                                "3. Browse or search the discovered applications list:\n" +
                                "   • On Linux: Automatically scans standard XDG .desktop launchers and Flatpak apps (/usr/share/applications, ~/.local/share/applications, /var/lib/flatpak/exports/share/applications).\n" +
                                "   • On Windows: Automatically scans Start Menu shortcuts (.lnk) and installed programs.\n" +
                                "4. Or click 'Browse...' to pick any custom executable, shell script (.sh), or PowerShell/batch script, and add optional command-line arguments.\n" +
                                "5. Click '⚡ Test Launch' to verify the app opens, then click 'Save / Apply to Key' and sync to your duckyPad SD card!")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("UNDER THE HOOD")
                        heading: qsTr("Trigger Modes: Silent USB HID Listener (_GV31) vs. F13–F24 Global Hotkey")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("Inside the Launch App Builder, you can choose between two trigger mechanisms:\n\n" +
                                "• 1. Silent USB HID Listener (_GV31 — Recommended Default):\n" +
                                "  Generates '$_GV31 = <slot_code>' in the key's DuckyScript. When you press the key on the duckyPad, no keyboard keystrokes are sent to your active window at all—instead, DuckyDeck Studio's background USB HID listener detects the _GV31 update in ~15 ms and spawns your application cleanly in the background.\n\n" +
                                "• 2. F13–F24 Global Hotkey Mode:\n" +
                                "  Emits a high function key (F13–F24) with optional Ctrl / Shift / Alt / Super modifiers over USB HID keyboard, useful if you also want the key bound in your desktop environment's native shortcut settings.\n\n" +
                                "💡 Note: Because the application is launched on your PC, keys with the amber 'APP' badge require DuckyDeck Studio to be running on your computer (or minimized in the system tray) and synced to the duckyPad SD card.")
                        }
                    }
                }
            }

            // =================================================================
            // TAB 4: WINDOW AUTO-SWITCHER & KEYBOARD MODIFIER LAYERS
            // =================================================================
            Flickable {
                id: autoSwitchTabFlick
                clip: true
                contentWidth: width - 18
                contentHeight: autoSwitchCol.implicitHeight + 16
                boundsBehavior: Flickable.StopAtBounds
                QQC2.ScrollBar.vertical: TutorialScrollBar {
                    parent: autoSwitchTabFlick
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                }

                ColumnLayout {
                    id: autoSwitchCol
                    width: autoSwitchTabFlick.width - 18
                    spacing: 10

                    SectionCard {
                        badgeText: qsTr("WINDOW FOCUS")
                        heading: qsTr("1. Automatic Window-Focus Profile Switching (Linux & Windows)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("DuckyDeck Studio can automatically switch your duckyPad to the matching profile whenever you focus an application or browser tab (supported on KDE Plasma 6 Wayland/X11, GNOME, COSMIC, Hyprland/Omarchy, Linux Mint Cinnamon/MATE/Xfce, and Windows 10/11):\n\n" +
                                "1. Turn on the 'AUTO-SWITCHER' toggle switch at the bottom of the left Profiles sidebar.\n" +
                                "2. Click the 'Rules' button below the toggle to open the Auto-Switcher Drawer (or use the 'Wizard...' button when creating a new profile).\n" +
                                "3. Click '🎯 Pick Window' — your cursor turns into a crosshair. Click any open application window to auto-fill its Window Class (app_id / process name) and Window Title!\n" +
                                "4. Select the Target Profile from the dropdown and click '+ Add Rule'.\n" +
                                "5. Set your 'Fallback Profile (★ FALLBACK)': Whenever you switch to a desktop window that doesn't match any rule, the duckyPad automatically returns to your Fallback Profile.\n\n" +
                                "💡 Smart Pause: Whenever you click a key on the Hardware Canvas to open the Key Editor, Auto-Switching automatically pauses ('⏸ Paused while Key Editor is open') so your profile never jumps away while you are editing!")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("BROWSER TABS & REGEX")
                        heading: qsTr("2. Matching Specific Browser Tabs or Window Titles (Substring & /Regex/)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("Want different duckyPad profiles for different websites inside the same browser (e.g. YouTube vs. Home Assistant vs. GitHub in Zen Browser, Firefox, or Chrome)?\n\n" +
                                "• Leave Window Class set to your browser (e.g. 'zen', 'firefox', 'chrome') and fill in the 'Window Title Filter':\n" +
                                "  - Plain Substring (case-insensitive): Type 'YouTube' to match any tab containing 'YouTube' in its title.\n" +
                                "  - Regular Expression: Wrap the pattern in slashes, such as '/^(YouTube|Twitch|Plex)/i' to match multiple media sites with a single rule!\n" +
                                "• Rules with BOTH a Window Class and a Window Title automatically take priority over generic class-only browser rules.")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("MODIFIER LAYERS")
                        heading: qsTr("3. Main-Keyboard Modifier Layer Shift (Global & Per-App / Title Layers)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("In the Auto-Switcher Rules drawer, you can bind modifier keys on your MAIN keyboard (such as Right Ctrl, Menu, Ctrl+Alt, Right Alt, or F13–F24) to shift your duckyPad into a secondary layer profile:\n\n" +
                                "• Hold (Momentary): Switches to the target profile while you hold the modifier keys down, and snaps back to your window's profile the instant you release them.\n" +
                                "• One-Shot (Sticky Next Key): Tap and release the modifier key to arm the layer for your next single duckyPad key press, then automatically return.\n" +
                                "• Double-Tap + Hold: Requires a quick double-tap and hold on the modifier key so single holds never trigger a layer switch.\n" +
                                "• Toggle (Press to Lock / Unlock): Tap the modifier key once to lock the layer profile on; tap it again to return.\n\n" +
                                "🪟 Global ('All Applications') vs. Per-Application / Window Title Scope:\n" +
                                "• You can assign the SAME modifier key (e.g. Right Ctrl) to bring up DIFFERENT profiles depending on which application or window title is currently active!\n" +
                                "• Choose 'Scope: 🪟 Specific Application / Window Title' (or '🌐 All Web Browsers') and use '🎯 Pick Window' or 'Use Current' to target a specific app (e.g. 'firefox', 'code', 'kdenlive') and/or Window Title (substring or Regex like 'YouTube|Twitch').\n" +
                                "• You can also keep a '🌐 All Applications (Global)' rule for the same modifier key as a fallback when any other app is focused. Specificity priority: App + Title > Title-only > Specific App > All Web Browsers > All Applications.\n\n" +
                                "🛡️ Built-in Safeguards:\n" +
                                "• Chord Guard: If you press any normal letter/number key on your main keyboard while holding the modifier (e.g. Ctrl+C), DuckyDeck Studio cancels the layer switch so normal typing shortcuts never flash your duckyPad.\n" +
                                "• Held-Modifier Bleed Suppression: While holding a modifier layer on your main keyboard, pressing a duckyPad key automatically neutralizes those held modifiers on the host OS so your duckyPad macro sends clean keystrokes!")
                        }
                    }
                }
            }

            // =================================================================
            // TAB 5: MULTI-CLICK (1x/2x/3x), GAMING HOLD & MACRO RECORDER
            // =================================================================
            Flickable {
                id: multiClickTabFlick
                clip: true
                contentWidth: width - 18
                contentHeight: multiClickCol.implicitHeight + 16
                boundsBehavior: Flickable.StopAtBounds
                QQC2.ScrollBar.vertical: TutorialScrollBar {
                    parent: multiClickTabFlick
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                }

                ColumnLayout {
                    id: multiClickCol
                    width: multiClickTabFlick.width - 18
                    spacing: 10

                    SectionCard {
                        badgeText: qsTr("MULTI-CLICK")
                        heading: qsTr("1. Assigning Single (1×), Double (2×), and Triple (3×) Clicks to One Key")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("Every mechanical key (Slots 1–20) can run up to 3 completely different macros depending on whether you tap it once, double-click it, or triple-click it!\n\n" +
                                "1. Select a key (1–20) to open the Key Editor.\n" +
                                "2. Look at the Phase Tab Bar right under the key label inputs: '⬇ 1× Single', '⚡ 2× Double', '⚡⚡ 3× Triple', and '⬆ On Release'.\n" +
                                "3. Click '⬇ 1× Single' and enter the script for a single tap (e.g. Play/Pause).\n" +
                                "4. Click '⚡ 2× Double' and enter the script for a double tap (e.g. Next Track).\n" +
                                "5. Click '⚡⚡ 3× Triple' and enter the script for a triple tap (e.g. Previous Track).\n" +
                                "6. Adjust '⏱ Gap' (default 250 ms) if you want a faster or more relaxed multi-click detection window.\n\n" +
                                "✅ 100% Standalone Hardware Compilation: DuckyDeck Studio compiles 1×/2×/3× detection into native DuckyScript 3 bytecode (using $_TIME_MS and state slots) so multi-click works standalone on the duckyPad hardware even without DuckyDeck Studio running!")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("GAMING HOLD")
                        heading: qsTr("2. Gaming Mode (Hold Key), On Release Scripts, No Repeat & Allow Abort")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("By default, a macro runs when you press a key down. For games (like WASD movement, Sprint, Crouch, or Push-to-Talk in Discord), you want the key to stay held down for as long as your finger holds the physical switch:\n\n" +
                                "• 🎮 Gaming Mode (Hold Key): Check this box in the Key Editor to automatically convert your key into a press-and-hold key! It places 'KEYDOWN <key>' in the On Press script (keyN.txt) and 'KEYUP <key>' in the '⬆ On Release' script (keyN-release.txt).\n" +
                                "• No Repeat on Held (Run Once): Prevents the duckyPad from auto-repeating a macro when held down (sets 'dr N' in config.txt) — recommended for destructive actions or toggles.\n" +
                                "• Allow Abort: Allows pressing any key on the duckyPad to immediately abort a long-running loop or macro ('$_ALLOW_ABORT = 1').")
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("MACRO RECORDER")
                        heading: qsTr("3. Recording Live Keyboard & Mouse Macros ('🔴 Record')")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("1. In the Key Editor, click the '🔴 Record' button on the right side of the mode tab bar.\n" +
                                "2. Choose what to capture: Keyboard Keystrokes, Mouse Clicks & Scroll Wheel, and/or Relative Mouse Movements (MOUSE_MOVE dx dy).\n" +
                                "3. Choose your Timing Mode:\n" +
                                "   • Exact Recorded Timings: Inserts accurate 'DELAY <ms>' commands matching your real typing cadence.\n" +
                                "   • As Fast As Possible: Strips delays so the sequence executes at maximum speed.\n" +
                                "4. Click 'Start Recording', perform your actions, and click 'Stop' (or press Esc) to insert the generated DuckyScript 3 directly into your key!")
                        }
                    }
                }
            }

            // =================================================================
            // TAB 6: GLOBAL VARIABLES (_GV) & RGB LIGHTING
            // =================================================================
            Flickable {
                id: gvRgbTabFlick
                clip: true
                contentWidth: width - 18
                contentHeight: gvRgbCol.implicitHeight + 16
                boundsBehavior: Flickable.StopAtBounds
                QQC2.ScrollBar.vertical: TutorialScrollBar {
                    parent: gvRgbTabFlick
                    anchors.top: parent.top
                    anchors.right: parent.right
                    anchors.bottom: parent.bottom
                }

                ColumnLayout {
                    id: gvRgbCol
                    width: gvRgbTabFlick.width - 18
                    spacing: 10

                    SectionCard {
                        badgeText: qsTr("VARIABLES")
                        heading: qsTr("1. Persistent Global Variables (_GV0 through _GV31)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("The duckyPad firmware provides 32 Persistent Global Variables ($_GV0 through $_GV31) that retain their 32-bit integer values across all keys, across profile switches, and across reboots on the microSD card:\n\n" +
                                "• $_GV0 .. $_GV30 (User Variables): Use these in your DuckyScript 3 macros to build two-state toggle keys (e.g. Mute/Unmute), cycle through multi-step modes (0 → 1 → 2 → 0), or track counters with the rotary encoders.\n" +
                                "• $_GV31 (Reserved Host Bridge Channel): Reserved by DuckyDeck Studio for silent USB HID App Launch ('APP') and Web API ('API') triggers.\n" +
                                "• Live USB HID Inspector: Open Menu ☰ → 'Variables (_GV)...' at any time to read and live-edit all 32 variables directly on your connected duckyPad!")
                        }

                        CodeSampleBox {
                            title: qsTr("Example: Two-State Toggle Key with LED Color Feedback (Using $_GV0)")
                            canPreviewOled: false
                            code: "REM Toggle between State 0 (Green LED) and State 1 (Red LED) on Key #1\n" +
                                  "IF $_GV0 == 0 THEN\n" +
                                  "    $_GV0 = 1\n" +
                                  "    SWCF 1 255 0 0\n" +
                                  "    KEY_MUTE\n" +
                                  "ELSE\n" +
                                  "    $_GV0 = 0\n" +
                                  "    SWCF 1 0 255 0\n" +
                                  "    KEY_MUTE\n" +
                                  "END_IF"
                        }
                    }

                    SectionCard {
                        badgeText: qsTr("RGB LIGHTING")
                        heading: qsTr("2. Hardware LED Commands (SWCF/SWCC) vs. Live RGB Lighting Animator (FX)")

                        QQC2.Label {
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                            font.pointSize: 10 * tutorialsDialog.fontScale
                            color: Kirigami.Theme.textColor
                            text: qsTr("DuckyDeck Studio offers three ways to customize your per-key RGB LEDs:\n\n" +
                                "• 1. Static Per-Profile & Per-Key Colors (Saved to SD Card):\n" +
                                "  Click the colored bar on any profile in the sidebar to set the profile's default background color (BG_COLOR), or right-click any key → 'Set Key Color' (or click the color swatch in the Key Editor) to set an individual key override (SWCOLOR_N).\n\n" +
                                "• 2. Dynamic DuckyScript 3 LED Commands (SWCF, SWCC, SWCR):\n" +
                                "  Inside a script, 'SWCF <key> <r> <g> <b>' fills a key LED, 'SWCC <key> <r> <g> <b>' sets it temporarily, and 'SWCR <key>' restores its default profile color.\n\n" +
                                "• 3. Live RGB Lighting Animator ('✨FX' Badge — 20 FPS USB HID Streaming):\n" +
                                "  Right-click any key → '✨ Key RGB Lighting Animation...' (or click the '✨FX' button in the Key Editor) to configure continuous Idle or On-Keypress lighting effects (Pulse, Horizontal/Vertical Wave, Reactive Ripple, Starlight, Rainbow Spectrum, Knight Scanner) across a single key or the entire 20-key pad!")
                        }
                    }
                }
            }
        }

        // Footer Bar with Quick Feedback & Open Web Guide / Close Buttons
        Rectangle {
            Layout.fillWidth: true
            height: 1
            color: Qt.alpha(Kirigami.Theme.textColor, 0.18)
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            QQC2.Label {
                Layout.fillWidth: true
                elide: Text.ElideRight
                font.bold: true
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                color: tutorialsDialog.copiedFeedbackText.length > 0
                    ? Kirigami.Theme.positiveTextColor
                    : Kirigami.Theme.disabledTextColor
                text: tutorialsDialog.copiedFeedbackText.length > 0
                    ? tutorialsDialog.copiedFeedbackText
                    : qsTr("Tip: Click any tab above to switch tutorials, or click '📖 Open Full Web User Guide' for the searchable manual.")
            }

            QQC2.Button {
                icon.name: "help-contents"
                text: qsTr("📖 Open Full Web User Guide")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                onClicked: {
                    if (tutorialsDialog.controller) {
                        tutorialsDialog.controller.openUserGuide()
                    }
                }
            }

            QQC2.Button {
                icon.name: "window-close"
                text: qsTr("Close")
                font.pointSize: 9.5 * tutorialsDialog.fontScale
                highlighted: true
                onClicked: tutorialsDialog.close()
            }
        }
    }
}
