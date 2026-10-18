import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import QtQuick.Templates as T
import org.kde.kirigami as Kirigami

Window {
    id: webApiKeyConfigWindow
    property var root: null
    property var controller: null
    readonly property var deviceController: controller
    property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    property color highContrastHighlightText: root ? root.highContrastHighlightText : "#38bdf8"
    property color highContrastWarningText: root ? root.highContrastWarningText : "#fbbf24"
    property var hardwareCanvas: null
    property var scriptEditor: null
    property var keyPropertiesWindow: null
    property alias webApiDestinationsDialog: webApiDestinationsDialog
    property alias webApiUrlField: webApiUrlField

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

    component HighContrastCheckBox: QQC2.CheckBox {
        id: hcCheck
        spacing: 8
        onCheckedChanged: hcCheckMarkCanvas.requestPaint()
        indicator: Rectangle {
            objectName: "highContrastCheckBoxBox"
            implicitWidth: 18
            implicitHeight: 18
            width: 18
            height: 18
            x: hcCheck.leftPadding
            y: hcCheck.topPadding + Math.round((hcCheck.availableHeight - height) / 2)
            radius: 4
            color: hcCheck.checked
                ? Kirigami.Theme.highlightColor
                : (hcCheck.down
                    ? Qt.alpha(Kirigami.Theme.textColor, 0.22)
                    : Kirigami.Theme.backgroundColor)
            border.width: (hcCheck.checked || hcCheck.hovered || hcCheck.visualFocus) ? 2 : 1.5
            border.color: hcCheck.checked
                ? Kirigami.Theme.highlightColor
                : (hcCheck.hovered || hcCheck.visualFocus
                    ? Kirigami.Theme.highlightColor
                    : Qt.alpha(Kirigami.Theme.textColor, 0.78))

            function pixelMetric(name) {
                return name === "checkboxlabelspacing" ? 8 : 18
            }

            Canvas {
                id: hcCheckMarkCanvas
                anchors.fill: parent
                visible: hcCheck.checked
                onPaint: {
                    const ctx = getContext("2d")
                    ctx.reset()
                    const lum = 0.2126 * Kirigami.Theme.highlightColor.r
                        + 0.7152 * Kirigami.Theme.highlightColor.g
                        + 0.0722 * Kirigami.Theme.highlightColor.b
                    ctx.strokeStyle = lum > 0.52 ? "#020617" : "#ffffff"
                    ctx.lineWidth = 2.2
                    ctx.lineCap = "round"
                    ctx.lineJoin = "round"
                    ctx.beginPath()
                    ctx.moveTo(4.0, 9.5)
                    ctx.lineTo(7.5, 13.0)
                    ctx.lineTo(14.0, 5.0)
                    ctx.stroke()
                }
            }
        }
    }

    function findWebApiDestinationIndex(nameStr, methodStr, urlStr, headersStr) {
        const list = root.webApiDestinationsList || []
        const cleanName = (nameStr || "").trim().toLowerCase()
        if (cleanName.length > 0) {
            for (let i = 0; i < list.length; ++i) {
                if ((list[i].name || "").trim().toLowerCase() === cleanName) {
                    return i
                }
            }
        }
        const cleanUrl = (urlStr || "").trim().toLowerCase()
        const cleanMethod = ((methodStr || "POST").trim().toUpperCase())
        const cleanHeaders = (headersStr || "").trim()
        if (cleanUrl.length > 0) {
            for (let i = 0; i < list.length; ++i) {
                if ((list[i].url || "").trim().toLowerCase() === cleanUrl
                    && ((list[i].method || "POST").trim().toUpperCase()) === cleanMethod
                    && (list[i].headers || "").trim() === cleanHeaders) {
                    return i
                }
            }
        }
        return -1
    }

    function buildWebApiConfigJson() {
        const methodIdx = Math.max(0, webApiMethodCombo.currentIndex)
        const fkeyIdx = Math.max(0, webApiFkeyCombo.currentIndex)
        const cfg = {
            enabled: webApiEnabledCheck.checked,
            destination: (root.webApiSelectedDestination || "").trim(),
            method: root.webApiMethods[methodIdx] || "POST",
            url: webApiUrlField.text.trim(),
            headers: webApiHeadersField.text.trim(),
            body: webApiBodyField.text,
            oledText: webApiOledField.text.trim(),
            ledFeedback: webApiLedCheck.checked,
            triggerMode: webApiTriggerModeCombo.currentIndex === 1 ? "fkey" : "pgv",
            fkey: root.webApiFkeys[fkeyIdx] || "F13"
        }
        return JSON.stringify(cfg)
    }

    function loadWebApiBuilderFromScript(scriptText) {
        if (!deviceController) return
        root.syncingWebApiScript = true
        try {
            const rawJson = deviceController.extractScriptWebApiJson(scriptText, root.selectedSlotNumber)
            const cfg = JSON.parse(rawJson)
            const isEnabled = !!cfg.enabled
            root.webApiEnabled = isEnabled
            webApiEnabledCheck.checked = isEnabled
            const mIdx = root.webApiMethods.indexOf((cfg.method || "POST").toUpperCase())
            webApiMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
            webApiUrlField.text = cfg.url || ""
            webApiHeadersField.text = cfg.headers || ""
            webApiBodyField.text = cfg.body || ""
            webApiOledField.text = cfg.oledText || ""
            webApiLedCheck.checked = cfg.ledFeedback !== undefined ? !!cfg.ledFeedback : true
            webApiTriggerModeCombo.currentIndex = cfg.triggerMode === "fkey" ? 1 : 0
            const fIdx = root.webApiFkeys.indexOf((cfg.fkey || "F13").toUpperCase())
            webApiFkeyCombo.currentIndex = fIdx >= 0 ? fIdx : ((root.selectedSlotNumber - 1) % 12)

            const matchedIdx = webApiKeyConfigWindow.findWebApiDestinationIndex(
                cfg.destination || "",
                cfg.method || "POST",
                cfg.url || "",
                cfg.headers || ""
            )
            if (matchedIdx >= 0 && matchedIdx < root.webApiDestinationsList.length) {
                const d = root.webApiDestinationsList[matchedIdx]
                root.webApiSelectedDestination = d.name
                webApiDestinationCombo.currentIndex = matchedIdx + 1
            } else {
                root.webApiSelectedDestination = (cfg.destination || "").trim()
                webApiDestinationCombo.currentIndex = 0
            }

            if (isEnabled) {
                root.showWebApiBuilder = true
            }
        } catch (e) {
            root.webApiEnabled = false
            root.webApiSelectedDestination = ""
            webApiEnabledCheck.checked = false
            webApiDestinationCombo.currentIndex = 0
        }
        root.syncingWebApiScript = false
    }

    function syncWebApiBuilderToScript() {
        if (root.loadingSlotData || root.syncingWebApiScript || !deviceController) return
        root.syncingWebApiScript = true
        root.webApiEnabled = webApiEnabledCheck.checked
        const cfgJson = webApiKeyConfigWindow.buildWebApiConfigJson()
        const updated = deviceController.applyWebApiToScript(
            scriptEditor.text,
            cfgJson,
            root.selectedSlotNumber,
            root.editingReleaseScript
        )
        scriptEditor.text = updated
        if (root.editingReleaseScript) {
            root.releaseScriptBuffer = updated
        } else {
            root.pressScriptBuffer = updated
        }
        root.syncingWebApiScript = false
        root.updateCompilerStatus()
        if (root.compileOk) {
            root.saveCurrentSlotToController()
            hardwareCanvas.syncSlotsFromController()
        }
    }

    Window {
        id: webApiDestinationsDialog
        readonly property bool closableWithEscape: true
        title: qsTr("Web API Destinations — DuckyDeck Studio")
        width: 740
        height: 560
        minimumWidth: 580
        minimumHeight: 420
        visible: false
        transientParent: root
        modality: Qt.WindowModal
        flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint

        Kirigami.Theme.colorSet: Kirigami.Theme.Window
        Kirigami.Theme.inherit: false
        color: Kirigami.Theme.backgroundColor

        property string editingOriginalName: ""

        function openForDestination(destName, fallbackMethod, fallbackUrl, fallbackHeaders) {
            const clean = (destName || "").trim()
            const list = root.webApiDestinationsList || []
            let found = null
            if (clean.length > 0) {
                for (let i = 0; i < list.length; ++i) {
                    if ((list[i].name || "").trim().toLowerCase() === clean.toLowerCase()) {
                        found = list[i]
                        break
                    }
                }
            }
            if (found) {
                webApiDestinationsDialog.loadEntryIntoForm(found)
            } else {
                webApiDestinationsDialog.editingOriginalName = ""
                destManagerNameField.text = clean
                const mIdx = root.webApiMethods.indexOf(((fallbackMethod || "POST").trim().toUpperCase()))
                destManagerMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
                destManagerUrlField.text = (fallbackUrl || "").trim()
                destManagerHeadersField.text = (fallbackHeaders || "").trim()
            }
            webApiDestinationsDialog.show()
            webApiDestinationsDialog.raise()
            webApiDestinationsDialog.requestActivate()
        }

        function loadEntryIntoForm(entry) {
            if (!entry) return
            webApiDestinationsDialog.editingOriginalName = entry.name || ""
            destManagerNameField.text = entry.name || ""
            const mIdx = root.webApiMethods.indexOf(((entry.method || "POST").trim().toUpperCase()))
            destManagerMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
            destManagerUrlField.text = entry.url || ""
            destManagerHeadersField.text = entry.headers || ""
        }

        function clearForm() {
            webApiDestinationsDialog.editingOriginalName = ""
            destManagerNameField.text = ""
            destManagerMethodCombo.currentIndex = 0
            destManagerUrlField.text = ""
            destManagerHeadersField.text = ""
            destManagerNameField.forceActiveFocus()
        }

        Shortcut {
            sequence: "Escape"
            context: Qt.WindowShortcut
            enabled: webApiDestinationsDialog.visible
            onActivated: webApiDestinationsDialog.close()
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    Layout.preferredWidth: 1
                    spacing: 2

                    Kirigami.Heading {
                        level: 2
                        text: qsTr("Reusable Web API Destinations")
                        color: Kirigami.Theme.textColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }

                    QQC2.Label {
                        text: qsTr("Configure named HTTP endpoints (Method, URL, and Headers) once and reuse them across keys with different payloads.")
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        wrapMode: Text.WordWrap
                        Layout.fillWidth: true
                        Layout.minimumWidth: 0
                    }
                }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "list-add"
                    text: qsTr("New Destination")
                    onClicked: webApiDestinationsDialog.clearForm()
                }
            }

            // Saved Destinations List
            Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: 8
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1

                Flickable {
                    id: destListFlickable
                    anchors.fill: parent
                    anchors.margins: 8
                    clip: true
                    contentWidth: width
                    contentHeight: destListCol.implicitHeight + 4
                    boundsBehavior: Flickable.StopAtBounds
                    flickableDirection: Flickable.VerticalFlick

                    QQC2.ScrollBar.vertical: HighContrastScrollBar {
                        parent: destListFlickable
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        policy: destListFlickable.contentHeight > destListFlickable.height
                            ? QQC2.ScrollBar.AlwaysOn
                            : QQC2.ScrollBar.AsNeeded
                    }

                    ColumnLayout {
                        id: destListCol
                        width: Math.max(200, destListFlickable.width - (destListFlickable.contentHeight > destListFlickable.height ? 16 : 0))
                        spacing: 6

                        QQC2.Label {
                            visible: (root.webApiDestinationsList || []).length === 0
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            Layout.topMargin: 28
                            horizontalAlignment: Text.AlignHCenter
                            wrapMode: Text.WordWrap
                            text: qsTr("No saved Web API destinations yet.\nEnter a Name, HTTP Method, URL, and optional Headers below and click Save Destination.")
                            color: Kirigami.Theme.disabledTextColor
                            font.pointSize: 10 * root.fontScale
                        }

                        Repeater {
                            model: root.webApiDestinationsList || []

                            delegate: Rectangle {
                                required property var modelData
                                required property int index
                                readonly property bool isBeingEdited: (webApiDestinationsDialog.editingOriginalName || "").toLowerCase() === (modelData.name || "").toLowerCase()
                                readonly property bool isSelectedOnCurrentKey: root.hasSelectedSlot && (root.webApiSelectedDestination || "").toLowerCase() === (modelData.name || "").toLowerCase()

                                Layout.fillWidth: true
                                implicitHeight: destRowLayout.implicitHeight + 14
                                radius: 6
                                color: isBeingEdited
                                    ? Qt.alpha(Kirigami.Theme.highlightColor, 0.16)
                                    : Qt.rgba(Kirigami.Theme.backgroundColor.r, Kirigami.Theme.backgroundColor.g, Kirigami.Theme.backgroundColor.b, 0.85)
                                border.color: isBeingEdited
                                    ? Kirigami.Theme.highlightColor
                                    : Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                                border.width: 1

                                RowLayout {
                                    id: destRowLayout
                                    anchors.fill: parent
                                    anchors.margins: 7
                                    spacing: 8

                                    Rectangle {
                                        radius: 4
                                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                                        border.color: Qt.alpha(Kirigami.Theme.highlightColor, 0.55)
                                        border.width: 1
                                        implicitWidth: Math.max(48, methodBadgeLbl.implicitWidth + 12)
                                        implicitHeight: 22

                                        QQC2.Label {
                                            id: methodBadgeLbl
                                            anchors.centerIn: parent
                                            text: (modelData.method || "POST").toUpperCase()
                                            font.family: "monospace"
                                            font.bold: true
                                            font.pointSize: 10 * root.fontScale
                                            color: root.highContrastHighlightText
                                        }
                                    }

                                    ColumnLayout {
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: 0
                                        Layout.preferredWidth: 1
                                        spacing: 2

                                        RowLayout {
                                            Layout.fillWidth: true
                                            spacing: 6

                                            QQC2.Label {
                                                text: modelData.name || ""
                                                font.bold: true
                                                font.pointSize: 10 * root.fontScale
                                                color: Kirigami.Theme.textColor
                                                elide: Text.ElideRight
                                                Layout.fillWidth: true
                                                Layout.minimumWidth: 0
                                            }

                                            Rectangle {
                                                visible: isSelectedOnCurrentKey
                                                radius: 3
                                                color: Qt.alpha(Kirigami.Theme.positiveTextColor, 0.18)
                                                implicitWidth: activeBadgeText.implicitWidth + 10
                                                implicitHeight: 18

                                                QQC2.Label {
                                                    id: activeBadgeText
                                                    anchors.centerIn: parent
                                                    text: qsTr("Active on Key %1").arg(root.selectedSlotNumber)
                                                    font.pointSize: 10 * root.fontScale
                                                    font.bold: true
                                                    color: Kirigami.Theme.positiveTextColor
                                                }
                                            }
                                        }

                                        QQC2.Label {
                                            Layout.fillWidth: true
                                            Layout.minimumWidth: 0
                                            text: modelData.url || ""
                                            font.family: "monospace"
                                            font.pointSize: 10 * root.fontScale
                                            color: Kirigami.Theme.disabledTextColor
                                            elide: Text.ElideMiddle
                                        }
                                    }

                                    QQC2.Button {
                                        visible: root.hasSelectedSlot
                                        text: isSelectedOnCurrentKey ? qsTr("✓ Selected") : qsTr("Use on Key %1").arg(root.selectedSlotNumber)
                                        highlighted: isSelectedOnCurrentKey
                                        implicitHeight: 26
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: {
                                            root.syncingWebApiScript = true
                                            root.webApiSelectedDestination = modelData.name || ""
                                            const mIdx = root.webApiMethods.indexOf((modelData.method || "POST").toUpperCase())
                                            webApiMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
                                            webApiUrlField.text = modelData.url || ""
                                            webApiHeadersField.text = modelData.headers || ""
                                            webApiDestinationCombo.currentIndex = index + 1
                                            if (!webApiEnabledCheck.checked) {
                                                webApiEnabledCheck.checked = true
                                            }
                                            root.syncingWebApiScript = false
                                            webApiKeyConfigWindow.syncWebApiBuilderToScript()
                                        }
                                    }

                                    QQC2.Button {
                                        icon.name: "document-edit"
                                        text: qsTr("Edit")
                                        implicitHeight: 26
                                        font.pointSize: 10 * root.fontScale
                                        onClicked: webApiDestinationsDialog.loadEntryIntoForm(modelData)
                                    }

                                    QQC2.Button {
                                        font.pointSize: 10 * root.fontScale
                                        icon.name: "edit-delete"
                                        implicitHeight: 26
                                        implicitWidth: 30
                                        onClicked: {
                                            const delName = (modelData.name || "").trim()
                                            if (delName.length === 0) return
                                            deviceController.deleteWebApiDestination(delName)
                                            if ((root.webApiSelectedDestination || "").toLowerCase() === delName.toLowerCase()) {
                                                root.webApiSelectedDestination = ""
                                                webApiDestinationCombo.currentIndex = 0
                                                webApiKeyConfigWindow.syncWebApiBuilderToScript()
                                            }
                                            if ((webApiDestinationsDialog.editingOriginalName || "").toLowerCase() === delName.toLowerCase()) {
                                                webApiDestinationsDialog.clearForm()
                                            }
                                        }
                                        QQC2.ToolTip.visible: hovered
                                        QQC2.ToolTip.text: qsTr("Delete destination '%1'").arg(modelData.name || "")
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Destination Editor Card
            Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                Layout.fillWidth: true
                implicitHeight: destEditorFormCol.implicitHeight + 20
                radius: 8
                color: Kirigami.Theme.alternateBackgroundColor
                border.color: Qt.alpha(Kirigami.Theme.textColor, Kirigami.Theme.frameContrast)
                border.width: 1

                ColumnLayout {
                    id: destEditorFormCol
                    anchors.fill: parent
                    anchors.margins: 10
                    spacing: 8

                    QQC2.Label {
                        text: webApiDestinationsDialog.editingOriginalName.length > 0
                            ? qsTr("EDIT DESTINATION: %1").arg(webApiDestinationsDialog.editingOriginalName)
                            : qsTr("ADD / UPDATE DESTINATION")
                        font.bold: true
                        font.pointSize: 10 * root.fontScale
                        color: Kirigami.Theme.disabledTextColor
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Name:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.preferredWidth: 64
                        }

                        QQC2.TextField {
                            font.pointSize: 10 * root.fontScale
                            id: destManagerNameField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 80
                            Layout.preferredWidth: 1
                            selectByMouse: true
                            placeholderText: qsTr("e.g. Home Assistant — Light Toggle, OBS Webhook, Discord Bot")
                        }

                        QQC2.Label {
                            text: qsTr("Method:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.ComboBox {
                            font.pointSize: 10 * root.fontScale
                            id: destManagerMethodCombo
                            model: root.webApiMethods
                            font.bold: true
                            Layout.preferredWidth: 100
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("URL:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.preferredWidth: 64
                        }

                        QQC2.TextField {
                            font.pointSize: 10 * root.fontScale
                            id: destManagerUrlField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 80
                            Layout.preferredWidth: 1
                            font.family: "monospace"
                            selectByMouse: true
                            placeholderText: qsTr("https://api.example.com/webhook  or  http://homeassistant.local:8123/api/services/light/toggle")
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        QQC2.Label {
                            text: qsTr("Headers:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                            Layout.preferredWidth: 64
                            Layout.alignment: Qt.AlignTop
                        }

                        QQC2.TextArea {
                            id: destManagerHeadersField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 80
                            Layout.preferredWidth: 1
                            implicitHeight: 54
                            wrapMode: TextEdit.Wrap
                            clip: true
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            selectByMouse: true
                            placeholderText: qsTr("Optional HTTP headers, one per line (e.g. Content-Type: application/json; charset=utf-8)")
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.backgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        Item { Layout.fillWidth: true }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
                            text: qsTr("Clear")
                            enabled: destManagerNameField.text.length > 0 || destManagerUrlField.text.length > 0 || destManagerHeadersField.text.length > 0
                            onClicked: webApiDestinationsDialog.clearForm()
                        }

                        QQC2.Button {
                            font.pointSize: 10 * root.fontScale
                            icon.name: "document-save"
                            text: webApiDestinationsDialog.editingOriginalName.length > 0
                                ? qsTr("Save Changes to Destination")
                                : qsTr("Save Destination")
                            highlighted: true
                            enabled: destManagerNameField.text.trim().length > 0 && destManagerUrlField.text.trim().length > 0
                            onClicked: {
                                const newName = destManagerNameField.text.trim()
                                const oldName = webApiDestinationsDialog.editingOriginalName.trim()
                                const mIdx = Math.max(0, destManagerMethodCombo.currentIndex)
                                const methodStr = root.webApiMethods[mIdx] || "POST"
                                const urlStr = destManagerUrlField.text.trim()
                                const headersStr = destManagerHeadersField.text.trim()

                                if (oldName.length > 0 && oldName.toLowerCase() !== newName.toLowerCase()) {
                                    deviceController.deleteWebApiDestination(oldName)
                                }
                                deviceController.saveWebApiDestination(newName, methodStr, urlStr, headersStr)
                                webApiDestinationsDialog.editingOriginalName = newName

                                if (root.hasSelectedSlot && (
                                    root.webApiSelectedDestination.trim().length === 0
                                    || root.webApiSelectedDestination.trim().toLowerCase() === oldName.toLowerCase()
                                    || root.webApiSelectedDestination.trim().toLowerCase() === newName.toLowerCase()
                                )) {
                                    root.syncingWebApiScript = true
                                    root.webApiSelectedDestination = newName
                                    webApiMethodCombo.currentIndex = mIdx
                                    webApiUrlField.text = urlStr
                                    webApiHeadersField.text = headersStr
                                    const matchedIdx = webApiKeyConfigWindow.findWebApiDestinationIndex(newName, methodStr, urlStr, headersStr)
                                    if (matchedIdx >= 0) {
                                        webApiDestinationCombo.currentIndex = matchedIdx + 1
                                    }
                                    if (!webApiEnabledCheck.checked && urlStr.length > 0) {
                                        webApiEnabledCheck.checked = true
                                    }
                                    root.syncingWebApiScript = false
                                    webApiKeyConfigWindow.syncWebApiBuilderToScript()
                                }
                            }
                        }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Item { Layout.fillWidth: true }

                QQC2.Button {
                    font.pointSize: 10 * root.fontScale
                    icon.name: "window-close"
                    text: qsTr("Close")
                    highlighted: true
                    onClicked: webApiDestinationsDialog.close()
                }
            }
        }
    }

    readonly property bool closableWithEscape: true
    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: webApiKeyConfigWindow.visible
        onActivated: webApiKeyConfigWindow.close()
    }
    title: qsTr("Web API / HTTP Webhook — Key %1 (%2)").arg(root.selectedSlotNumber).arg(root.selectedSlotTitle)
    width: 820
    height: 760
    minimumWidth: 580
    minimumHeight: 500
    visible: false
    transientParent: root
    modality: Qt.NonModal
    flags: Qt.Dialog | Qt.WindowTitleHint | Qt.WindowSystemMenuHint | Qt.WindowCloseButtonHint | Qt.WindowMaximizeButtonHint

    Kirigami.Theme.colorSet: Kirigami.Theme.Window
    Kirigami.Theme.inherit: false
    color: Kirigami.Theme.backgroundColor

    property var haEntitiesModel: []
    property string haStatusText: ""
    property bool haShowToken: false

    function loadHomeAssistantSavedConfig() {
        try {
            const cfg = JSON.parse(deviceController.getHomeAssistantConfigJson())
            if (cfg) {
                haBaseUrlField.text = cfg.base_url || cfg.baseUrl || "http://homeassistant.local:8123"
                haTokenField.text = cfg.access_token || cfg.accessToken || ""
            }
        } catch (e) {
            if (haBaseUrlField.text.trim().length === 0) {
                haBaseUrlField.text = "http://homeassistant.local:8123"
            }
        }
    }

    function searchHomeAssistantEntities() {
        const baseUrl = haBaseUrlField.text.trim()
        const token = haTokenField.text.trim()
        if (baseUrl.length === 0 || token.length === 0) {
            webApiKeyConfigWindow.haStatusText = qsTr("⚠ Enter your Home Assistant URL and Long-Lived Access Token first.")
            return
        }
        const domainVal = haDomainCombo.currentIndex <= 0 ? "" : haDomainCombo.currentText
        const raw = deviceController.searchHomeAssistantEntitiesJson(
            baseUrl,
            token,
            haSearchQueryField.text.trim(),
            domainVal
        )
        try {
            const parsed = JSON.parse(raw)
            const entitiesList = Array.isArray(parsed)
                ? parsed
                : (parsed && Array.isArray(parsed.entities) ? parsed.entities : null)
            if (parsed && parsed.ok === false) {
                webApiKeyConfigWindow.haEntitiesModel = []
                webApiKeyConfigWindow.haStatusText = qsTr("✗ %1").arg(
                    String(parsed.error || deviceController.statusMessage || "Home Assistant request failed.")
                )
            } else if (Array.isArray(entitiesList)) {
                webApiKeyConfigWindow.haEntitiesModel = entitiesList
                webApiKeyConfigWindow.haStatusText = entitiesList.length > 0
                    ? qsTr("✓ Found %1 matching Home Assistant entities. Click an action below to bind it to Key %2.")
                        .arg(entitiesList.length)
                        .arg(root.selectedSlotNumber)
                    : qsTr("No matching entities found for that filter/query.")
            } else {
                webApiKeyConfigWindow.haEntitiesModel = []
                webApiKeyConfigWindow.haStatusText = String(
                    deviceController.statusMessage || qsTr("Unexpected Home Assistant response.")
                )
            }
        } catch (e) {
            webApiKeyConfigWindow.haEntitiesModel = []
            webApiKeyConfigWindow.haStatusText = qsTr("Failed to parse Home Assistant response: %1").arg(String(e))
        }
    }

    function applyHomeAssistantEntityToKey(entityObj, serviceOverride) {
        const entityId = entityObj ? String(entityObj.entity_id || entityObj.entityId || "") : ""
        if (!entityId) return
        const cleanBase = haBaseUrlField.text.trim().replace(/\/+$/, "")
        const token = haTokenField.text.trim()
        const domain = String(entityObj.domain || entityId.split(".")[0] || "homeassistant")
        let rawService = (serviceOverride && serviceOverride.length > 0)
            ? String(serviceOverride)
            : String(entityObj.suggested_service || entityObj.suggestedService || "toggle")
        if (rawService.indexOf("/") >= 0) {
            rawService = rawService.split("/").pop()
        } else if (rawService.indexOf(".") >= 0) {
            rawService = rawService.split(".").pop()
        }
        const service = rawService || "toggle"

        root.syncingWebApiScript = true
        webApiEnabledCheck.checked = true
        const postIdx = root.webApiMethods.indexOf("POST")
        webApiMethodCombo.currentIndex = postIdx >= 0 ? postIdx : 1
        webApiUrlField.text = cleanBase + "/api/services/" + domain + "/" + service
        webApiHeadersField.text = "Authorization: Bearer " + token + "\nContent-Type: application/json"
        webApiBodyField.text = JSON.stringify({ entity_id: entityId })
        const friendlyRaw = String(entityObj.friendly_name || entityObj.friendlyName || "")
        const friendly = friendlyRaw.length > 0 ? friendlyRaw : entityId
        webApiOledField.text = ("HA:" + friendly).slice(0, 20)
        root.syncingWebApiScript = false
        webApiKeyConfigWindow.syncWebApiBuilderToScript()
        webApiKeyConfigWindow.haStatusText = qsTr("✓ Configured Key %1 to call %2.%3 on '%4' (%5)")
            .arg(root.selectedSlotNumber)
            .arg(domain)
            .arg(service)
            .arg(friendly)
            .arg(entityId)
    }

    function openWindow(slotNumber) {
        const targetSlot = (slotNumber !== undefined && slotNumber > 0) ? slotNumber : root.selectedSlotNumber
        if (targetSlot <= 0) return
        webApiKeyConfigWindow.loadHomeAssistantSavedConfig()
        if (root.hasSelectedSlot && root.selectedSlotNumber === targetSlot) {
            webApiKeyConfigWindow.loadWebApiBuilderFromScript(scriptEditor.text)
        } else if (keyPropertiesWindow.visible && keyPropertiesWindow.slotNum === targetSlot) {
            webApiKeyConfigWindow.loadWebApiBuilderFromScript(keyPropertiesWindow.keyPropPressScriptArea.text)
        }
        webApiKeyConfigWindow.show()
        webApiKeyConfigWindow.raise()
        webApiKeyConfigWindow.requestActivate()
    }

    function openForCurrentSlot() {
        openWindow(root.selectedSlotNumber)
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 14
        spacing: 10

        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Rectangle {
                width: 34
                height: 34
                radius: 8
                color: Qt.alpha(Kirigami.Theme.highlightColor, 0.2)
                border.color: Kirigami.Theme.highlightColor
                border.width: 1
                QQC2.Label {
                    anchors.centerIn: parent
                    text: "🌐"
                    font.pointSize: 13 * root.fontScale
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                Layout.minimumWidth: 0
                Layout.preferredWidth: 1
                spacing: 2
                QQC2.Label {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    elide: Text.ElideRight
                    text: qsTr("Web API & Outbound HTTP Webhook — %1").arg(root.selectedSlotTitle)
                    font.bold: true
                    font.pointSize: 12 * root.fontScale
                }
                QQC2.Label {
                    Layout.fillWidth: true
                    Layout.minimumWidth: 0
                    text: qsTr("Trigger Home Assistant devices/entities or send HTTP REST/Webhook requests when this key is pressed.")
                    font.pointSize: 10 * root.fontScale
                    color: Kirigami.Theme.disabledTextColor
                    wrapMode: Text.WordWrap
                }
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "network-server"
                text: qsTr("Manage Destinations...")
                onClicked: {
                    webApiDestinationsDialog.openForDestination(
                        root.webApiSelectedDestination,
                        webApiMethodCombo.currentText,
                        webApiUrlField.text,
                        webApiHeadersField.text
                    )
                }
            }

            QQC2.Switch {
                font.pointSize: 10 * root.fontScale
                id: webApiEnabledCheck
                text: checked ? qsTr("Enabled") : qsTr("Disabled")
                font.bold: true
                onToggled: webApiKeyConfigWindow.syncWebApiBuilderToScript()
            }
        }

        Rectangle {
            Layout.fillWidth: true
            radius: 5
            color: Qt.alpha("#f59e0b", 0.14)
            border.color: Qt.alpha("#f59e0b", 0.55)
            border.width: 1
            implicitHeight: webApiHostWarnLbl.implicitHeight + 10

            QQC2.Label {
                id: webApiHostWarnLbl
                anchors.fill: parent
                anchors.margins: 5
                wrapMode: Text.WordWrap
                font.pointSize: 10 * root.fontScale
                font.bold: true
                color: root.highContrastWarningText
                text: qsTr("💻 Requires DuckyDeck Studio Running on Host PC (Not Hardware-Portable) — Outbound HTTP REST / Webhook calls are dispatched by DuckyDeck Studio over USB HID. The duckyPad hardware has no network stack and cannot send HTTP requests when DuckyDeck Studio is closed.")
            }
        }

        Flickable {
            id: webApiFlickable
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.minimumHeight: 80
            Layout.preferredHeight: 1
            clip: true
            contentWidth: width
            contentHeight: webApiScrollCol.implicitHeight + 12
            boundsBehavior: Flickable.StopAtBounds
            flickableDirection: Flickable.VerticalFlick

            QQC2.ScrollBar.vertical: HighContrastScrollBar {
                parent: webApiFlickable
                anchors.top: webApiFlickable.top
                anchors.right: webApiFlickable.right
                anchors.bottom: webApiFlickable.bottom
                policy: webApiFlickable.contentHeight > webApiFlickable.height
                    ? QQC2.ScrollBar.AlwaysOn
                    : QQC2.ScrollBar.AsNeeded
            }

            ColumnLayout {
                id: webApiScrollCol
                width: Math.max(200, webApiFlickable.width - (webApiFlickable.contentHeight > webApiFlickable.height ? 18 : 2))
                spacing: 10

                // Home Assistant Web API Device & Entity Search Card
                Rectangle {
                    Layout.fillWidth: true
                    implicitHeight: haCardCol.implicitHeight + 18
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha("#38bdf8", 0.45)
                    border.width: 1

                    ColumnLayout {
                        id: haCardCol
                        anchors.fill: parent
                        anchors.margins: 9
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("🏠 Home Assistant Web API — Search Devices & Entities to Trigger")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.fillWidth: true
                                wrapMode: Text.WordWrap
                            }

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                icon.name: "document-save"
                                text: qsTr("Save HA Connection")
                                onClicked: {
                                    if (deviceController.saveHomeAssistantConfig(haBaseUrlField.text.trim(), haTokenField.text.trim())) {
                                        webApiKeyConfigWindow.haStatusText = qsTr("✓ Saved Home Assistant URL and Long-Lived Access Token.")
                                    }
                                }
                            }
                        }

                        QQC2.Label {
                            text: qsTr("Connect to your Home Assistant instance with a Long-Lived Access Token (Profile → Security → Long-Lived Access Tokens) to search and bind entities directly to this key.")
                            font.pointSize: 10 * root.fontScale
                            color: Kirigami.Theme.disabledTextColor
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("HA URL:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.preferredWidth: 70
                            }

                            QQC2.TextField {
                                id: haBaseUrlField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 140
                                Layout.preferredWidth: 1
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                text: "http://homeassistant.local:8123"
                                placeholderText: qsTr("http://homeassistant.local:8123")
                            }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("HA Token:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                                Layout.preferredWidth: 70
                            }

                            QQC2.TextField {
                                id: haTokenField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 140
                                Layout.preferredWidth: 1
                                font.family: "monospace"
                                font.pointSize: 10 * root.fontScale
                                selectByMouse: true
                                echoMode: webApiKeyConfigWindow.haShowToken ? TextInput.Normal : TextInput.Password
                                placeholderText: qsTr("Paste Home Assistant Long-Lived Access Token...")
                            }

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                text: webApiKeyConfigWindow.haShowToken ? qsTr("Hide") : qsTr("Show")
                                onClicked: webApiKeyConfigWindow.haShowToken = !webApiKeyConfigWindow.haShowToken
                            }
                        }

                        Flow {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.ComboBox {
                                font.pointSize: 10 * root.fontScale
                                id: haDomainCombo
                                width: 150
                                model: [
                                    "All Domains",
                                    "light",
                                    "switch",
                                    "scene",
                                    "script",
                                    "automation",
                                    "button",
                                    "input_boolean",
                                    "media_player",
                                    "cover",
                                    "fan",
                                    "climate",
                                    "lock"
                                ]
                                onActivated: {
                                    if (haTokenField.text.trim().length > 0) {
                                        webApiKeyConfigWindow.searchHomeAssistantEntities()
                                    }
                                }
                            }

                            QQC2.TextField {
                                font.pointSize: 10 * root.fontScale
                                id: haSearchQueryField
                                width: Math.max(200, haCardCol.width - 340)
                                selectByMouse: true
                                placeholderText: qsTr("Search devices/entities (e.g. desk light, studio, scene)...")
                                onAccepted: webApiKeyConfigWindow.searchHomeAssistantEntities()
                            }

                            QQC2.Button {
                                font.pointSize: 10 * root.fontScale
                                icon.name: "edit-find"
                                text: qsTr("🔍 Search Entities")
                                highlighted: true
                                onClicked: webApiKeyConfigWindow.searchHomeAssistantEntities()
                            }
                        }

                        QQC2.Label {
                            visible: webApiKeyConfigWindow.haStatusText.length > 0
                            text: webApiKeyConfigWindow.haStatusText
                            font.pointSize: 10 * root.fontScale
                            font.bold: true
                            color: webApiKeyConfigWindow.haStatusText.startsWith("✓")
                                ? Kirigami.Theme.positiveTextColor
                                : Kirigami.Theme.neutralTextColor
                            Layout.fillWidth: true
                            wrapMode: Text.WordWrap
                        }

                        Rectangle {
                            visible: webApiKeyConfigWindow.haEntitiesModel.length > 0
                            Layout.fillWidth: true
                            implicitHeight: Math.min(220, haEntityRepeaterCol.implicitHeight + 12)
                            radius: 5
                            color: Kirigami.Theme.backgroundColor
                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                            border.width: 1
                            clip: true

                            Flickable {
                                id: haEntityListFlick
                                anchors.fill: parent
                                anchors.margins: 4
                                clip: true
                                contentWidth: width
                                contentHeight: haEntityRepeaterCol.implicitHeight
                                boundsBehavior: Flickable.StopAtBounds
                                flickableDirection: Flickable.VerticalFlick

                                QQC2.ScrollBar.vertical: HighContrastScrollBar {
                                    parent: haEntityListFlick
                                    anchors.top: haEntityListFlick.top
                                    anchors.right: haEntityListFlick.right
                                    anchors.bottom: haEntityListFlick.bottom
                                    policy: haEntityListFlick.contentHeight > haEntityListFlick.height
                                        ? QQC2.ScrollBar.AlwaysOn
                                        : QQC2.ScrollBar.AsNeeded
                                }

                                ColumnLayout {
                                    id: haEntityRepeaterCol
                                    width: Math.max(160, haEntityListFlick.width - 16)
                                    spacing: 4

                                    Repeater {
                                        model: webApiKeyConfigWindow.haEntitiesModel
                                        delegate: Rectangle {
                                            required property var modelData
                                            readonly property string entId: String(modelData.entity_id || modelData.entityId || "")
                                            readonly property string entName: String(modelData.friendly_name || modelData.friendlyName || entId)
                                            readonly property string entDomain: String(modelData.domain || (entId.indexOf(".") >= 0 ? entId.split(".")[0] : "ha"))
                                            readonly property string entService: {
                                                const rawSvc = String(modelData.suggested_service || modelData.suggestedService || "toggle")
                                                if (rawSvc.indexOf("/") >= 0) return rawSvc.split("/").pop()
                                                if (rawSvc.indexOf(".") >= 0) return rawSvc.split(".").pop()
                                                return rawSvc
                                            }
                                            Layout.fillWidth: true
                                            implicitHeight: haRowInnerCol.implicitHeight + 10
                                            radius: 4
                                            color: Kirigami.Theme.alternateBackgroundColor
                                            border.color: Qt.alpha(Kirigami.Theme.textColor, 0.14)
                                            border.width: 1

                                            ColumnLayout {
                                                id: haRowInnerCol
                                                anchors.fill: parent
                                                anchors.margins: 5
                                                spacing: 4

                                                RowLayout {
                                                    Layout.fillWidth: true
                                                    spacing: 6

                                                    QQC2.Label {
                                                        text: entName
                                                        font.bold: true
                                                        font.pointSize: 10 * root.fontScale
                                                        Layout.fillWidth: true
                                                        wrapMode: Text.WordWrap
                                                    }

                                                    Rectangle {
                                                        radius: 3
                                                        color: Qt.alpha(Kirigami.Theme.highlightColor, 0.18)
                                                        implicitWidth: haStateLbl.implicitWidth + 8
                                                        implicitHeight: 18
                                                        QQC2.Label {
                                                            id: haStateLbl
                                                            anchors.centerIn: parent
                                                            text: String(modelData.state || "unknown")
                                                            font.family: "monospace"
                                                            font.pointSize: 10 * root.fontScale
                                                            font.bold: true
                                                        }
                                                    }
                                                }

                                                Flow {
                                                    Layout.fillWidth: true
                                                    spacing: 6

                                                    QQC2.Label {
                                                        text: entId
                                                        font.family: "monospace"
                                                        font.pointSize: 10 * root.fontScale
                                                        color: Kirigami.Theme.disabledTextColor
                                                        height: 24
                                                        verticalAlignment: Text.AlignVCenter
                                                    }

                                                    QQC2.Button {
                                                        text: qsTr("⚡ %1.%2")
                                                            .arg(entDomain)
                                                            .arg(entService)
                                                        highlighted: true
                                                        implicitHeight: 24
                                                        font.pointSize: 10 * root.fontScale
                                                        onClicked: webApiKeyConfigWindow.applyHomeAssistantEntityToKey(
                                                            modelData,
                                                            entService
                                                        )
                                                    }

                                                    QQC2.Button {
                                                        visible: entDomain === "light" || entDomain === "switch" || entDomain === "input_boolean" || entDomain === "fan" || entDomain === "automation"
                                                        text: qsTr("Turn On")
                                                        implicitHeight: 24
                                                        font.pointSize: 10 * root.fontScale
                                                        onClicked: webApiKeyConfigWindow.applyHomeAssistantEntityToKey(modelData, "turn_on")
                                                    }

                                                    QQC2.Button {
                                                        visible: entDomain === "light" || entDomain === "switch" || entDomain === "input_boolean" || entDomain === "fan" || entDomain === "automation"
                                                        text: qsTr("Turn Off")
                                                        implicitHeight: 24
                                                        font.pointSize: 10 * root.fontScale
                                                        onClicked: webApiKeyConfigWindow.applyHomeAssistantEntityToKey(modelData, "turn_off")
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    implicitHeight: apiEndpointCol.implicitHeight + 16
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                    border.width: 1

                    ColumnLayout {
                        id: apiEndpointCol
                        anchors.fill: parent
                        anchors.margins: 8
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("Destination:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            QQC2.ComboBox {
                                font.pointSize: 10 * root.fontScale
                                id: webApiDestinationCombo
                                Layout.fillWidth: true
                                Layout.minimumWidth: 120
                                Layout.preferredWidth: 1
                                model: root.webApiDestinationComboItems
                                onActivated: (index) => {
                                    root.syncingWebApiScript = true
                                    if (index > 0 && (index - 1) < root.webApiDestinationsList.length) {
                                        const d = root.webApiDestinationsList[index - 1]
                                        root.webApiSelectedDestination = d.name || ""
                                        const mIdx = root.webApiMethods.indexOf((d.method || "POST").toUpperCase())
                                        webApiMethodCombo.currentIndex = mIdx >= 0 ? mIdx : 0
                                        webApiUrlField.text = d.url || ""
                                        webApiHeadersField.text = d.headers || ""
                                        if (!webApiEnabledCheck.checked) {
                                            webApiEnabledCheck.checked = true
                                        }
                                    } else {
                                        root.webApiSelectedDestination = ""
                                    }
                                    root.syncingWebApiScript = false
                                    webApiKeyConfigWindow.syncWebApiBuilderToScript()
                                }
                            }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.ComboBox {
                                font.pointSize: 10 * root.fontScale
                                id: webApiMethodCombo
                                Layout.preferredWidth: 105
                                model: root.webApiMethods
                                onActivated: webApiKeyConfigWindow.syncWebApiBuilderToScript()
                            }

                            QQC2.TextField {
                                font.pointSize: 10 * root.fontScale
                                id: webApiUrlField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                font.family: "monospace"
                                placeholderText: qsTr("https://example.com/api/webhook or http://homeassistant.local:8123/api/...")
                                selectByMouse: true
                                onTextEdited: {
                                    if (text.trim().length > 0 && !webApiEnabledCheck.checked) {
                                        webApiEnabledCheck.checked = true
                                    }
                                    webApiKeyConfigWindow.syncWebApiBuilderToScript()
                                }
                            }
                        }

                        QQC2.Label {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            wrapMode: Text.WordWrap
                            text: qsTr("HTTP Headers (Optional — one 'Header-Name: value' per line):")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.TextArea {
                            id: webApiHeadersField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 60
                            wrapMode: TextEdit.Wrap
                            clip: true
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            placeholderText: qsTr("Content-Type: application/json\nAuthorization: Bearer <token>")
                            selectByMouse: true
                            onTextChanged: {
                                if (!root.syncingWebApiScript) webApiKeyConfigWindow.syncWebApiBuilderToScript()
                            }
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.backgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }

                        QQC2.Label {
                            Layout.fillWidth: true
                            Layout.minimumWidth: 0
                            wrapMode: Text.WordWrap
                            text: qsTr("Request Body / JSON Payload:")
                            font.bold: true
                            font.pointSize: 10 * root.fontScale
                        }

                        QQC2.TextArea {
                            id: webApiBodyField
                            Layout.fillWidth: true
                            Layout.minimumWidth: 100
                            Layout.preferredWidth: 1
                            implicitHeight: 76
                            wrapMode: TextEdit.Wrap
                            clip: true
                            font.family: "monospace"
                            font.pointSize: 10 * root.fontScale
                            placeholderText: qsTr("{\"entity_id\": \"light.studio_key_light\"}")
                            selectByMouse: true
                            onTextChanged: {
                                if (!root.syncingWebApiScript) webApiKeyConfigWindow.syncWebApiBuilderToScript()
                            }
                            background: Rectangle {
                                radius: 4
                                color: Kirigami.Theme.backgroundColor
                                border.color: Qt.alpha(Kirigami.Theme.textColor, 0.2)
                                border.width: 1
                            }
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    implicitHeight: webTrigCol.implicitHeight + 16
                    radius: 6
                    color: Kirigami.Theme.alternateBackgroundColor
                    border.color: Qt.alpha(Kirigami.Theme.textColor, 0.16)
                    border.width: 1

                    ColumnLayout {
                        id: webTrigCol
                        anchors.fill: parent
                        anchors.margins: 8
                        spacing: 8

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            QQC2.Label {
                                text: qsTr("Trigger Mode:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            QQC2.ComboBox {
                                font.pointSize: 10 * root.fontScale
                                id: webApiTriggerModeCombo
                                Layout.fillWidth: true
                                Layout.minimumWidth: 120
                                Layout.preferredWidth: 1
                                model: [
                                    qsTr("USB HID Listener (Silent background HTTP call)"),
                                    qsTr("F-Key Shortcut Fallback (Emits F13–F24 + HTTP call)"),
                                    qsTr("Standalone Terminal curl (Types curl command on host)")
                                ]
                                onActivated: webApiKeyConfigWindow.syncWebApiBuilderToScript()
                            }

                            QQC2.ComboBox {
                                font.pointSize: 10 * root.fontScale
                                id: webApiFkeyCombo
                                visible: webApiTriggerModeCombo.currentIndex === 1
                                Layout.preferredWidth: 96
                                model: ["F13","F14","F15","F16","F17","F18","F19","F20","F21","F22","F23","F24"]
                                currentIndex: 7
                                onActivated: webApiKeyConfigWindow.syncWebApiBuilderToScript()
                            }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 12

                            QQC2.Label {
                                text: qsTr("OLED Feedback Text:")
                                font.bold: true
                                font.pointSize: 10 * root.fontScale
                            }

                            QQC2.TextField {
                                font.pointSize: 10 * root.fontScale
                                id: webApiOledField
                                Layout.fillWidth: true
                                Layout.minimumWidth: 100
                                Layout.preferredWidth: 1
                                maximumLength: 20
                                placeholderText: qsTr("Optional OLED status text on trigger")
                                selectByMouse: true
                                onTextEdited: webApiKeyConfigWindow.syncWebApiBuilderToScript()
                            }

                            HighContrastCheckBox {
                                font.pointSize: 10 * root.fontScale
                                id: webApiLedCheck
                                checked: true
                                text: qsTr("Flash Key LED Cyan")
                                onToggled: webApiKeyConfigWindow.syncWebApiBuilderToScript()
                            }
                        }
                    }
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "media-playback-start"
                text: qsTr("Test Webhook Now")
                enabled: webApiUrlField.text.trim().length > 0
                onClicked: {
                    deviceController.testWebApiCall(
                        webApiKeyConfigWindow.buildWebApiConfigJson(),
                        root.selectedSlotNumber
                    )
                }
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "edit-clear"
                text: qsTr("Disable / Clear")
                onClicked: {
                    webApiEnabledCheck.checked = false
                    webApiUrlField.text = ""
                    webApiHeadersField.text = ""
                    webApiBodyField.text = ""
                    webApiKeyConfigWindow.syncWebApiBuilderToScript()
                }
            }

            Item { Layout.fillWidth: true; Layout.minimumWidth: 0 }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "dialog-ok-apply"
                text: qsTr("Save & Close")
                highlighted: true
                font.bold: true
                onClicked: {
                    webApiKeyConfigWindow.syncWebApiBuilderToScript()
                    webApiKeyConfigWindow.close()
                }
            }

            QQC2.Button {
                font.pointSize: 10 * root.fontScale
                icon.name: "window-close"
                text: qsTr("Close")
                onClicked: webApiKeyConfigWindow.close()
            }
        }
    }
}
