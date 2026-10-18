import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

ColumnLayout {
    id: restApiSettingsRoot
    spacing: 12

    property var controller: null
    readonly property var deviceController: controller
    property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property color highContrastPositiveText: "#4ade80"
    property color highContrastWarningText: "#fbbf24"

    property bool showRestToken: false
    property string restApiFeedback: ""
    property string globalWebhookFeedback: ""

    function syncRestApiFieldsFromController() {
        restApiEnableCheck.checked = deviceController.restApiEnabled
        restApiIpField.text = deviceController.restApiBindIp || "127.0.0.1"
        restApiPortField.text = String(deviceController.restApiPort || 28491)
        restApiRequireAuthCheck.checked = deviceController.restApiRequireAuth
        restApiTokenField.text = deviceController.restApiToken || ""
    }

    function syncGlobalWebhookFieldsFromController() {
        try {
            const wh = JSON.parse(deviceController.getGlobalProfileWebhookJson())
            globalWebhookEnableCheck.checked = Boolean(wh.enabled)
            const mIdx = globalWebhookMethodCombo.indexOfValue((wh.method || "POST").toUpperCase())
            globalWebhookMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 1
            globalWebhookUrlField.text = wh.url || ""
            globalWebhookHeadersArea.text = wh.headers !== undefined ? String(wh.headers) : "Content-Type: application/json"
            globalWebhookBodyArea.text = wh.body !== undefined
                ? String(wh.body)
                : "{\"event\":\"profile_changed\",\"profile_number\":{{profile_number}},\"profile_name\":\"{{profile_name}}\",\"previous_profile_number\":{{previous_profile_number}},\"previous_profile_name\":\"{{previous_profile_name}}\",\"source\":\"{{source}}\",\"device_serial\":\"{{device_serial}}\",\"timestamp\":{{timestamp}}}"
        } catch (e) {
            globalWebhookEnableCheck.checked = false
        }
    }

    function saveGlobalWebhookConfig() {
        const payload = {
            enabled: globalWebhookEnableCheck.checked,
            method: globalWebhookMethodCombo.currentText || "POST",
            url: globalWebhookUrlField.text.trim(),
            headers: globalWebhookHeadersArea.text,
            body: globalWebhookBodyArea.text
        }
        if (deviceController.saveGlobalProfileWebhookJson(JSON.stringify(payload))) {
            restApiSettingsRoot.globalWebhookFeedback = qsTr("✓ Saved global profile-change webhook configuration.")
        }
    }

    function resetAndSync() {
        restApiSettingsRoot.syncRestApiFieldsFromController()
        restApiSettingsRoot.syncGlobalWebhookFieldsFromController()
        restApiSettingsRoot.restApiFeedback = ""
        restApiSettingsRoot.globalWebhookFeedback = ""
    }

    // Section 1: HTTP REST API Server (Remote Control & Programming)
    Rectangle {
        Kirigami.Theme.colorSet: Kirigami.Theme.View
        Kirigami.Theme.inherit: false
        Layout.fillWidth: true
        implicitHeight: restApiCol.implicitHeight + 20
        radius: 8
        color: Kirigami.Theme.alternateBackgroundColor
        border.color: deviceController.restApiEnabled
            ? Kirigami.Theme.positiveTextColor
            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        border.width: 1

        ColumnLayout {
            id: restApiCol
            anchors.fill: parent
            anchors.margins: 10
            spacing: 8

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("🌐 HTTP REST API SERVER (REMOTE CONTROL & PROGRAMMING)")
                    font.bold: true
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                }

                Rectangle {
                    radius: 4
                    color: deviceController.restApiEnabled
                        ? Qt.alpha(Kirigami.Theme.positiveTextColor, 0.18)
                        : Qt.alpha(Kirigami.Theme.textColor, 0.1)
                    border.color: deviceController.restApiEnabled
                        ? Kirigami.Theme.positiveTextColor
                        : Qt.alpha(Kirigami.Theme.textColor, 0.3)
                    border.width: 1
                    implicitWidth: restStatusBadgeLbl.implicitWidth + 12
                    implicitHeight: 22

                    QQC2.Label {
                        id: restStatusBadgeLbl
                        anchors.centerIn: parent
                        text: deviceController.restApiStatus
                        font.family: "monospace"
                        font.bold: true
                        font.pointSize: 10 * restApiSettingsRoot.fontScale
                        color: deviceController.restApiEnabled
                            ? restApiSettingsRoot.highContrastPositiveText
                            : Kirigami.Theme.disabledTextColor
                    }
                }
            }

            // Prominent Security Implications Box
            Rectangle {
                Layout.fillWidth: true
                radius: 6
                color: Qt.alpha("#f59e0b", 0.14)
                border.color: Qt.alpha("#f59e0b", 0.65)
                border.width: 1
                implicitHeight: restSecWarnCol.implicitHeight + 14

                ColumnLayout {
                    id: restSecWarnCol
                    anchors.fill: parent
                    anchors.margins: 7
                    spacing: 4

                    QQC2.Label {
                        text: qsTr("⚠ Security Implications — Please Read Before Enabling")
                        font.bold: true
                        font.pointSize: 10 * restApiSettingsRoot.fontScale
                        color: restApiSettingsRoot.highContrastWarningText
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                    }

                    QQC2.Label {
                        text: qsTr("• Full Device & Host Control: Any client with access to this API can switch/create/edit/delete profiles and keys, write to the microSD card, inject duckyScript keystroke macros onto the host PC, control RGB LEDs and the OLED display, and trigger host actions (APP_LAUNCH, WEB_API, SYS_ACTION) bound to keys.\n• Network Exposure (127.0.0.1 vs 0.0.0.0): Binding to 127.0.0.1 (Default) restricts access strictly to local processes on this computer. Binding to 0.0.0.0 or a LAN IP exposes an unencrypted plain-HTTP server to all devices on your local network.\n• Token Authentication: Keep Bearer Token Authentication enabled at all times (send 'Authorization: Bearer <token>' or 'X-API-Token: <token>'). Never expose this plain-HTTP port directly to the public internet; use an HTTPS reverse proxy or SSH/WireGuard tunnel if remote access is required.")
                        font.pointSize: 10 * restApiSettingsRoot.fontScale
                        color: Kirigami.Theme.textColor
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                    }
                }
            }

            QQC2.CheckBox {
                id: restApiEnableCheck
                Layout.fillWidth: true
                text: qsTr("Enable HTTP REST API Server (/api/v1/*)")
                font.bold: true
            }

            Flow {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("Listen IP:")
                    font.bold: true
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    height: 28
                    verticalAlignment: Text.AlignVCenter
                }

                QQC2.TextField {
                    id: restApiIpField
                    width: 140
                    font.family: "monospace"
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    selectByMouse: true
                    placeholderText: "127.0.0.1"
                }

                QQC2.Button {
                    text: qsTr("127.0.0.1 (Localhost Default)")
                    implicitHeight: 28
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    onClicked: restApiIpField.text = "127.0.0.1"
                }

                QQC2.Button {
                    text: qsTr("0.0.0.0 (All Interfaces / LAN)")
                    implicitHeight: 28
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    onClicked: restApiIpField.text = "0.0.0.0"
                }

                QQC2.Label {
                    text: qsTr("Port:")
                    font.bold: true
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    height: 28
                    verticalAlignment: Text.AlignVCenter
                }

                QQC2.TextField {
                    id: restApiPortField
                    width: 80
                    font.family: "monospace"
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    selectByMouse: true
                    placeholderText: "28491"
                    validator: IntValidator { bottom: 1024; top: 65535 }
                }
            }

            QQC2.CheckBox {
                id: restApiRequireAuthCheck
                Layout.fillWidth: true
                text: qsTr("Require Bearer Token Authentication (Recommended — header 'Authorization: Bearer <token>')")
                font.bold: true
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 6

                QQC2.Label {
                    text: qsTr("API Token:")
                    font.bold: true
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                }

                QQC2.TextField {
                    id: restApiTokenField
                    Layout.fillWidth: true
                    Layout.minimumWidth: 140
                    Layout.preferredWidth: 1
                    font.family: "monospace"
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    selectByMouse: true
                    echoMode: restApiSettingsRoot.showRestToken ? TextInput.Normal : TextInput.Password
                    placeholderText: qsTr("Click 'Generate Token' to create a 160-bit API token")
                }

                QQC2.Button {
                    text: restApiSettingsRoot.showRestToken ? qsTr("Hide") : qsTr("Show")
                    implicitHeight: 28
                    onClicked: restApiSettingsRoot.showRestToken = !restApiSettingsRoot.showRestToken
                }

                QQC2.Button {
                    icon.name: "edit-copy"
                    text: qsTr("Copy Token")
                    implicitHeight: 28
                    onClicked: {
                        const prevShow = restApiSettingsRoot.showRestToken
                        restApiSettingsRoot.showRestToken = true
                        restApiTokenField.selectAll()
                        restApiTokenField.copy()
                        restApiTokenField.deselect()
                        restApiSettingsRoot.showRestToken = prevShow
                        restApiSettingsRoot.restApiFeedback = qsTr("✓ Copied API Bearer token to clipboard.")
                    }
                }

                QQC2.Button {
                    icon.name: "view-refresh"
                    text: qsTr("Generate Token")
                    implicitHeight: 28
                    onClicked: {
                        const newTok = deviceController.regenerateRestApiToken()
                        restApiTokenField.text = newTok
                        restApiSettingsRoot.restApiFeedback = qsTr("✓ Generated new 160-bit API token (ddk_...).")
                    }
                }
            }

            Flow {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Button {
                    icon.name: "dialog-ok-apply"
                    text: qsTr("Apply & Restart REST API Server")
                    highlighted: true
                    font.bold: true
                    onClicked: {
                        const portNum = parseInt(restApiPortField.text.trim(), 10) || 28491
                        if (deviceController.applyRestApiConfig(
                            restApiEnableCheck.checked,
                            restApiIpField.text.trim(),
                            portNum,
                            restApiRequireAuthCheck.checked,
                            restApiTokenField.text.trim()
                        )) {
                            restApiSettingsRoot.syncRestApiFieldsFromController()
                            restApiSettingsRoot.restApiFeedback = qsTr("✓ %1").arg(deviceController.restApiStatus)
                        } else {
                            restApiSettingsRoot.restApiFeedback = qsTr("✗ %1").arg(deviceController.statusMessage)
                        }
                    }
                }

                QQC2.Button {
                    icon.name: "edit-reset"
                    text: qsTr("Reset Defaults (127.0.0.1:28491)")
                    onClicked: {
                        if (deviceController.resetRestApiDefaults()) {
                            restApiSettingsRoot.syncRestApiFieldsFromController()
                            restApiSettingsRoot.restApiFeedback = qsTr("✓ Reset REST API settings to defaults (127.0.0.1:28491, Auth Required).")
                        }
                    }
                }
            }

            QQC2.Label {
                visible: restApiSettingsRoot.restApiFeedback.length > 0
                text: restApiSettingsRoot.restApiFeedback
                font.family: "monospace"
                font.pointSize: 10 * restApiSettingsRoot.fontScale
                font.bold: true
                color: restApiSettingsRoot.restApiFeedback.startsWith("✓")
                    ? Kirigami.Theme.positiveTextColor
                    : Kirigami.Theme.neutralTextColor
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
            }
        }
    }

    // Section 2: Global Profile Change Webhook
    Rectangle {
        Kirigami.Theme.colorSet: Kirigami.Theme.View
        Kirigami.Theme.inherit: false
        Layout.fillWidth: true
        implicitHeight: globalWebhookCol.implicitHeight + 20
        radius: 8
        color: Kirigami.Theme.alternateBackgroundColor
        border.color: globalWebhookEnableCheck.checked
            ? Kirigami.Theme.highlightColor
            : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
        border.width: 1

        ColumnLayout {
            id: globalWebhookCol
            anchors.fill: parent
            anchors.margins: 10
            spacing: 8

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.Label {
                    text: qsTr("🔔 GLOBAL PROFILE CHANGE WEBHOOK")
                    font.bold: true
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                }

                QQC2.Button {
                    icon.name: "network-transmit-receive"
                    text: deviceController.webApiTestBusy ? qsTr("Testing...") : qsTr("Test Webhook")
                    enabled: !deviceController.webApiTestBusy && globalWebhookUrlField.text.trim().length > 0
                    onClicked: {
                        deviceController.testProfileWebhookJson(
                            JSON.stringify({
                                enabled: true,
                                method: globalWebhookMethodCombo.currentText || "POST",
                                url: globalWebhookUrlField.text.trim(),
                                headers: globalWebhookHeadersArea.text,
                                body: globalWebhookBodyArea.text
                            }),
                            0
                        )
                        restApiSettingsRoot.globalWebhookFeedback = qsTr("⏳ Testing webhook → %1 (see result below)").arg(globalWebhookUrlField.text.trim())
                    }
                }

                QQC2.Button {
                    icon.name: "document-save"
                    text: qsTr("Save Global Webhook")
                    highlighted: true
                    onClicked: restApiSettingsRoot.saveGlobalWebhookConfig()
                }
            }

            QQC2.CheckBox {
                id: globalWebhookEnableCheck
                Layout.fillWidth: true
                text: qsTr("Call this HTTP Webhook whenever the active profile changes (UI, Hardware buttons, Window Auto-Switcher, or REST API)")
                onToggled: restApiSettingsRoot.saveGlobalWebhookConfig()
                contentItem: QQC2.Label {
                    leftPadding: (globalWebhookEnableCheck.indicator ? globalWebhookEnableCheck.indicator.width : 18) + globalWebhookEnableCheck.spacing
                    text: globalWebhookEnableCheck.text
                    font: globalWebhookEnableCheck.font
                    color: Kirigami.Theme.textColor
                    verticalAlignment: Text.AlignVCenter
                    wrapMode: Text.WordWrap
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                QQC2.ComboBox {
                    id: globalWebhookMethodCombo
                    Layout.preferredWidth: 105
                    model: ["GET", "POST", "PUT", "PATCH", "DELETE"]
                    currentIndex: 1
                }

                QQC2.TextField {
                    id: globalWebhookUrlField
                    Layout.fillWidth: true
                    Layout.minimumWidth: 120
                    Layout.preferredWidth: 1
                    font.family: "monospace"
                    font.pointSize: 10 * restApiSettingsRoot.fontScale
                    selectByMouse: true
                    placeholderText: qsTr("http://homeassistant.local:8123/api/webhook/duckypad_profile_changed")
                }
            }

            QQC2.Label {
                text: qsTr("Headers (Optional, one 'Header-Name: Value' per line):")
                font.pointSize: 10 * restApiSettingsRoot.fontScale
                font.bold: true
                color: Kirigami.Theme.disabledTextColor
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
            }

            QQC2.TextArea {
                id: globalWebhookHeadersArea
                Layout.fillWidth: true
                Layout.minimumWidth: 100
                Layout.preferredWidth: 1
                implicitHeight: 44
                font.family: "monospace"
                font.pointSize: 10 * restApiSettingsRoot.fontScale
                selectByMouse: true
                wrapMode: TextEdit.Wrap
                clip: true
                placeholderText: qsTr("Content-Type: application/json")
                background: Rectangle {
                    radius: 4
                    color: Kirigami.Theme.backgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                    border.width: 1
                }
            }

            QQC2.Label {
                text: qsTr("Payload Body (Placeholders: {{profile_number}}, {{profile_name}}, {{previous_profile_number}}, {{previous_profile_name}}, {{source}}, {{device_serial}}, {{timestamp}}):")
                font.pointSize: 10 * restApiSettingsRoot.fontScale
                font.bold: true
                color: Kirigami.Theme.disabledTextColor
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
            }

            QQC2.TextArea {
                id: globalWebhookBodyArea
                Layout.fillWidth: true
                Layout.minimumWidth: 100
                Layout.preferredWidth: 1
                implicitHeight: 54
                font.family: "monospace"
                font.pointSize: 10 * restApiSettingsRoot.fontScale
                selectByMouse: true
                wrapMode: TextEdit.Wrap
                clip: true
                background: Rectangle {
                    radius: 4
                    color: Kirigami.Theme.backgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                    border.width: 1
                }
            }

            QQC2.Label {
                visible: restApiSettingsRoot.globalWebhookFeedback.length > 0
                text: restApiSettingsRoot.globalWebhookFeedback
                font.family: "monospace"
                font.pointSize: 10 * restApiSettingsRoot.fontScale
                font.bold: true
                color: restApiSettingsRoot.globalWebhookFeedback.startsWith("✓")
                    ? Kirigami.Theme.positiveTextColor
                    : Kirigami.Theme.neutralTextColor
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
            }
        }
    }
}
