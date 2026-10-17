import QtQuick
import QtQuick.Window
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

Window {
    id: oledWidgetWindow
    objectName: "duckydeckOledWidgetWindow"
    title: alwaysOnTop
        ? qsTr("duckyPad OLED — P%1: %2").arg(activeProfile).arg(activeProfileName)
        : qsTr("duckyPad OLED — P%1: %2 [Unpinned]").arg(activeProfile).arg(activeProfileName)
    width: 284
    height: 356
    minimumWidth: 184
    minimumHeight: 240
    visible: false
    color: "transparent"

    property var controller: null
    property int selectedSlot: 0
    property bool alwaysOnTop: true
    property bool framelessMode: true
    property bool showRgbDots: true
    property bool showSizeSlider: false

    transientParent: null

    flags: Qt.Window
        | (alwaysOnTop ? Qt.WindowStaysOnTopHint : Qt.Widget)
        | (framelessMode ? Qt.FramelessWindowHint : Qt.Widget)

    onFramelessModeChanged: {
        titlebarIconCanvas.requestPaint()
        if (visible && controller) {
            controller.setOledWidgetKeepAbove(alwaysOnTop)
        }
    }

    onAlwaysOnTopChanged: {
        if (alwaysOnTop && visibility === Window.Minimized) {
            showNormal()
            raise()
        }
        if (controller) {
            controller.setOledWidgetKeepAbove(alwaysOnTop)
        }
        pinIconCanvas.requestPaint()
    }

    onVisibilityChanged: (vis) => {
        if (alwaysOnTop && vis === Window.Minimized) {
            Qt.callLater(function() {
                if (oledWidgetWindow.alwaysOnTop) {
                    oledWidgetWindow.showNormal()
                    oledWidgetWindow.raise()
                }
            })
        }
    }

    onVisibleChanged: {
        if (transientParent !== null) {
            transientParent = null
        }
        if (visible) {
            syncFromController()
            if (controller) {
                controller.restoreOledWidgetPlacement(alwaysOnTop)
            }
        } else {
            showSizeSlider = false
        }
    }

    readonly property int activeProfile: controller ? controller.activeProfile : 1
    readonly property string activeProfileName: (controller && controller.activeProfileName)
        ? controller.activeProfileName
        : "Default"

    // Pure top-down geometry calculations (no child implicitHeight dependencies -> zero binding loops)
    readonly property real uiScale: Math.max(0.85, Math.min(width / 260.0, height / 340.0))
    readonly property int outerPad: Math.max(6, Math.round(8 * uiScale))
    readonly property int sectionGap: Math.max(4, Math.round(6 * uiScale))
    readonly property int headerBarHeight: Math.max(26, Math.round(28 * uiScale))
    readonly property int footerBarHeight: Math.max(44, Math.round(52 * uiScale))
    readonly property int availOledWidth: Math.max(120, width - outerPad * 2)
    readonly property int availOledHeight: Math.max(120, height - outerPad * 2 - headerBarHeight - footerBarHeight - sectionGap * 2)
    readonly property int oledSide: Math.max(120, Math.min(availOledWidth, availOledHeight))
    readonly property real currentScaleMultiplier: Math.max(0.8, Math.min(3.0, oledSide / 128.0))

    property real pendingScaleMultiplier: 1.5
    property var slotItems: []
    property string enc1Summary: "CCW · CW · [Push]"
    property string enc2Summary: "CCW · CW · [Push]"

    Timer {
        id: sliderResizeTimer
        interval: 16
        repeat: false
        onTriggered: {
            oledWidgetWindow.applyScalePreset(oledWidgetWindow.pendingScaleMultiplier)
        }
    }

    function formatEnc(item, fallback) {
        if (!item) return fallback
        const l1 = (item.l1 || "").trim()
        const l2 = (item.l2 || "").trim()
        const c = (l1 + (l2.length > 0 ? " " + l2 : "")).trim()
        return c.length > 0 ? c : fallback
    }

    function slotDataFor(slotNum) {
        if (slotItems && slotNum >= 1 && slotNum <= slotItems.length) {
            const direct = slotItems[slotNum - 1]
            if (direct && direct.slot === slotNum) {
                return direct
            }
            for (let i = 0; i < slotItems.length; ++i) {
                if (slotItems[i].slot === slotNum) {
                    return slotItems[i]
                }
            }
        }
        return { slot: slotNum, l1: "", l2: "", r: 0, g: 170, b: 255, hasScript: false }
    }

    function syncFromController() {
        if (!controller || !controller.activeProfileSlotsJson) return
        try {
            const arr = JSON.parse(controller.activeProfileSlotsJson)
            if (!Array.isArray(arr)) return
            const keys = []
            let e1ccw = "CCW", e1cw = "CW", e1push = "Push"
            let e2ccw = "CCW", e2cw = "CW", e2push = "Push"
            for (let i = 0; i < arr.length; ++i) {
                const it = arr[i]
                if (it.slot >= 1 && it.slot <= 20) {
                    keys.push({
                        slot: it.slot,
                        l1: it.l1 || "",
                        l2: it.l2 || "",
                        r: it.r !== undefined ? it.r : 0,
                        g: it.g !== undefined ? it.g : 170,
                        b: it.b !== undefined ? it.b : 255,
                        hasScript: !!it.hasScript
                    })
                } else if (it.slot === 21) {
                    e1ccw = formatEnc(it, "CCW")
                } else if (it.slot === 22) {
                    e1cw = formatEnc(it, "CW")
                } else if (it.slot === 23) {
                    e1push = formatEnc(it, "Push")
                } else if (it.slot === 24) {
                    e2ccw = formatEnc(it, "CCW")
                } else if (it.slot === 25) {
                    e2cw = formatEnc(it, "CW")
                } else if (it.slot === 26) {
                    e2push = formatEnc(it, "Push")
                }
            }
            slotItems = keys
            enc1Summary = e1ccw + " · " + e1cw + " · [" + e1push + "]"
            enc2Summary = e2ccw + " · " + e2cw + " · [" + e2push + "]"
        } catch (e) {}
    }

    function applyScalePreset(mult) {
        const clamped = Math.max(0.8, Math.min(3.0, mult))
        const targetOled = Math.round(128 * clamped)
        oledWidgetWindow.width = Math.max(minimumWidth, targetOled + 32)
        oledWidgetWindow.height = Math.max(minimumHeight, targetOled + Math.round(114 * Math.max(1.0, clamped * 0.75)))
    }

    Connections {
        target: oledWidgetWindow.controller
        function onActiveProfileSlotsJsonChanged() {
            oledWidgetWindow.syncFromController()
        }
        function onActiveProfileChanged() {
            oledWidgetWindow.syncFromController()
        }
        function onActiveProfileNameChanged() {
            oledWidgetWindow.syncFromController()
        }
        function onProfilesJsonChanged() {
            oledWidgetWindow.syncFromController()
        }
    }

    Component.onCompleted: {
        transientParent = null
        syncFromController()
    }

    Rectangle {
        id: outerShell
        anchors.fill: parent
        radius: oledWidgetWindow.framelessMode ? 10 : 0
        color: "#0b0f19"
        border.width: oledWidgetWindow.framelessMode ? 1.5 : 1
        border.color: oledWidgetWindow.alwaysOnTop ? "#38bdf8" : "#334155"

        // Top Compact Header & Drag Handle
        Rectangle {
            id: topHeaderBar
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.margins: oledWidgetWindow.outerPad
            height: oledWidgetWindow.headerBarHeight
            radius: 6
            color: "#111827"
            border.color: "#1e293b"
            border.width: 1

            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.LeftButton
                onPressed: {
                    oledWidgetWindow.startSystemMove()
                }
                onDoubleClicked: {
                    oledWidgetWindow.framelessMode = !oledWidgetWindow.framelessMode
                }
            }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 6
                anchors.rightMargin: 4
                spacing: 4

                Rectangle {
                    width: Math.max(7, Math.round(8 * oledWidgetWindow.uiScale))
                    height: width
                    radius: width / 2
                    color: (oledWidgetWindow.controller && oledWidgetWindow.controller.connected)
                        ? "#22c55e"
                        : "#38bdf8"
                }

                QQC2.Label {
                    text: "P" + oledWidgetWindow.activeProfile + ": " + oledWidgetWindow.activeProfileName
                    font.bold: true
                    font.pixelSize: Math.max(11, Math.round(12 * oledWidgetWindow.uiScale))
                    color: "#f8fafc"
                    elide: Text.ElideRight
                    Layout.fillWidth: true
                }

                // Previous Profile Button (−)
                QQC2.ToolButton {
                    id: prevProfileBtn
                    Layout.preferredWidth: Math.max(22, Math.round(22 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        if (oledWidgetWindow.controller) {
                            const prev = Math.max(1, oledWidgetWindow.activeProfile - 1)
                            oledWidgetWindow.controller.switchProfile(prev)
                        }
                    }
                    contentItem: Item {
                        Canvas {
                            width: 14
                            height: 14
                            anchors.centerIn: parent
                            scale: Math.max(0.85, Math.min(1.35, oledWidgetWindow.uiScale * 0.92))
                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.reset()
                                ctx.strokeStyle = "#f8fafc"
                                ctx.lineWidth = 2.0
                                ctx.lineCap = "round"
                                ctx.beginPath()
                                ctx.moveTo(3.5, 7)
                                ctx.lineTo(10.5, 7)
                                ctx.stroke()
                            }
                        }
                    }
                    background: Rectangle {
                        radius: 4
                        color: prevProfileBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: prevProfileBtn.hovered ? "#38bdf8" : "#344663"
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Previous Profile")
                }

                // Next Profile Button (+)
                QQC2.ToolButton {
                    id: nextProfileBtn
                    Layout.preferredWidth: Math.max(22, Math.round(22 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        if (oledWidgetWindow.controller) {
                            const next = Math.min(64, oledWidgetWindow.activeProfile + 1)
                            oledWidgetWindow.controller.switchProfile(next)
                        }
                    }
                    contentItem: Item {
                        Canvas {
                            width: 14
                            height: 14
                            anchors.centerIn: parent
                            scale: Math.max(0.85, Math.min(1.35, oledWidgetWindow.uiScale * 0.92))
                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.reset()
                                ctx.strokeStyle = "#f8fafc"
                                ctx.lineWidth = 2.0
                                ctx.lineCap = "round"
                                ctx.beginPath()
                                ctx.moveTo(3.5, 7)
                                ctx.lineTo(10.5, 7)
                                ctx.moveTo(7, 3.5)
                                ctx.lineTo(7, 10.5)
                                ctx.stroke()
                            }
                        }
                    }
                    background: Rectangle {
                        radius: 4
                        color: nextProfileBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: nextProfileBtn.hovered ? "#38bdf8" : "#344663"
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Next Profile")
                }

                // Always-on-Top Pin Icon Button
                QQC2.ToolButton {
                    id: pinTopBtn
                    Layout.preferredWidth: Math.max(24, Math.round(24 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        oledWidgetWindow.alwaysOnTop = !oledWidgetWindow.alwaysOnTop
                    }
                    contentItem: Item {
                        Canvas {
                            id: pinIconCanvas
                            width: 16
                            height: 16
                            anchors.centerIn: parent
                            scale: Math.max(0.85, Math.min(1.35, oledWidgetWindow.uiScale * 0.92))
                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.reset()
                                ctx.save()
                                ctx.translate(8, 8)
                                if (!oledWidgetWindow.alwaysOnTop) {
                                    ctx.rotate(Math.PI / 5)
                                }
                                ctx.translate(-8, -8)

                                const col = oledWidgetWindow.alwaysOnTop ? "#38bdf8" : "#cbd5e1"
                                ctx.strokeStyle = col
                                ctx.fillStyle = col
                                ctx.lineWidth = 1.5
                                ctx.lineCap = "round"
                                ctx.lineJoin = "round"

                                // Top cap of pushpin
                                ctx.beginPath()
                                ctx.moveTo(5, 2.5)
                                ctx.lineTo(11, 2.5)
                                ctx.stroke()

                                // Pin body trapezoid
                                ctx.beginPath()
                                ctx.moveTo(6, 2.5)
                                ctx.lineTo(10, 2.5)
                                ctx.lineTo(11.5, 8.5)
                                ctx.lineTo(4.5, 8.5)
                                ctx.closePath()
                                if (oledWidgetWindow.alwaysOnTop) {
                                    ctx.fill()
                                }
                                ctx.stroke()

                                // Bottom flange
                                ctx.beginPath()
                                ctx.moveTo(3.5, 8.5)
                                ctx.lineTo(12.5, 8.5)
                                ctx.stroke()

                                // Needle point
                                ctx.beginPath()
                                ctx.moveTo(8, 8.5)
                                ctx.lineTo(8, 14)
                                ctx.stroke()

                                ctx.restore()
                            }
                        }
                    }
                    background: Rectangle {
                        radius: 4
                        color: oledWidgetWindow.alwaysOnTop
                            ? Qt.rgba(0.22, 0.74, 0.97, 0.22)
                            : (pinTopBtn.hovered ? "#25334a" : "#1a2333")
                        border.width: 1
                        border.color: oledWidgetWindow.alwaysOnTop ? "#38bdf8" : "#344663"
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: oledWidgetWindow.alwaysOnTop
                        ? qsTr("Always on Top: ON (Click to unpin)")
                        : qsTr("Always on Top: OFF (Click to keep above all windows)")
                }

                // Scale / Size Slider Toggle Button with Resize Icon
                QQC2.ToolButton {
                    id: scaleToggleBtn
                    Layout.preferredWidth: Math.max(24, Math.round(24 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        if (!oledWidgetWindow.showSizeSlider) {
                            oledWidgetWindow.pendingScaleMultiplier = oledWidgetWindow.currentScaleMultiplier
                            sizeSlider.value = oledWidgetWindow.currentScaleMultiplier
                        }
                        oledWidgetWindow.showSizeSlider = !oledWidgetWindow.showSizeSlider
                        scaleIconCanvas.requestPaint()
                    }
                    contentItem: Item {
                        Canvas {
                            id: scaleIconCanvas
                            width: 16
                            height: 16
                            anchors.centerIn: parent
                            scale: Math.max(0.85, Math.min(1.35, oledWidgetWindow.uiScale * 0.92))
                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.reset()
                                const col = oledWidgetWindow.showSizeSlider ? "#38bdf8" : "#cbd5e1"
                                ctx.strokeStyle = col
                                ctx.lineWidth = 1.5
                                ctx.lineCap = "round"
                                ctx.lineJoin = "round"

                                // Outer frame
                                ctx.beginPath()
                                ctx.rect(2, 2, 12, 12)
                                ctx.stroke()

                                // Diagonal double-headed resize arrow (bottom-left to top-right)
                                ctx.beginPath()
                                ctx.moveTo(4.5, 11.5)
                                ctx.lineTo(11.5, 4.5)
                                // Top-right arrowhead
                                ctx.moveTo(8, 4.5)
                                ctx.lineTo(11.5, 4.5)
                                ctx.lineTo(11.5, 8)
                                // Bottom-left arrowhead
                                ctx.moveTo(8, 11.5)
                                ctx.lineTo(4.5, 11.5)
                                ctx.lineTo(4.5, 8)
                                ctx.stroke()
                            }
                        }
                    }
                    background: Rectangle {
                        radius: 4
                        color: oledWidgetWindow.showSizeSlider
                            ? Qt.rgba(0.22, 0.74, 0.97, 0.22)
                            : (scaleToggleBtn.hovered ? "#25334a" : "#1a2333")
                        border.width: 1
                        border.color: oledWidgetWindow.showSizeSlider ? "#38bdf8" : "#344663"
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Adjust OLED Widget Size")
                }

                // Restore Window Titlebar / Frameless Mode Toggle Button with Window Icon
                QQC2.ToolButton {
                    id: titlebarToggleBtn
                    Layout.preferredWidth: Math.max(24, Math.round(24 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        oledWidgetWindow.framelessMode = !oledWidgetWindow.framelessMode
                    }
                    contentItem: Item {
                        Canvas {
                            id: titlebarIconCanvas
                            width: 16
                            height: 16
                            anchors.centerIn: parent
                            scale: Math.max(0.85, Math.min(1.35, oledWidgetWindow.uiScale * 0.92))
                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.reset()
                                const activeTitlebar = !oledWidgetWindow.framelessMode
                                const col = activeTitlebar ? "#38bdf8" : "#cbd5e1"
                                ctx.strokeStyle = col
                                ctx.fillStyle = col
                                ctx.lineWidth = 1.5
                                ctx.lineCap = "round"
                                ctx.lineJoin = "round"

                                // Window outer rectangle
                                ctx.beginPath()
                                ctx.rect(2, 2.5, 12, 11)
                                ctx.stroke()

                                // Window top titlebar header strip
                                if (activeTitlebar) {
                                    ctx.fillRect(2, 2.5, 12, 3.5)
                                } else {
                                    ctx.beginPath()
                                    ctx.moveTo(2, 6)
                                    ctx.lineTo(14, 6)
                                    ctx.stroke()
                                    // Small titlebar dots
                                    ctx.fillRect(3.8, 3.8, 1.3, 1.3)
                                    ctx.fillRect(6.2, 3.8, 1.3, 1.3)
                                }
                            }
                        }
                    }
                    background: Rectangle {
                        radius: 4
                        color: !oledWidgetWindow.framelessMode
                            ? Qt.rgba(0.22, 0.74, 0.97, 0.22)
                            : (titlebarToggleBtn.hovered ? "#25334a" : "#1a2333")
                        border.width: 1
                        border.color: !oledWidgetWindow.framelessMode ? "#38bdf8" : "#344663"
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: oledWidgetWindow.framelessMode
                        ? qsTr("Restore Window Titlebar")
                        : qsTr("Frameless Desktop Overlay Mode")
                }
            }
        }

        // Fixed-Geometry Interactive Size Slider Overlay (fixed track width = zero drag jitter)
        Rectangle {
            id: sizeSliderOverlay
            visible: oledWidgetWindow.showSizeSlider
            z: 25
            anchors.top: topHeaderBar.bottom
            anchors.topMargin: 4
            anchors.left: parent.left
            anchors.leftMargin: 8
            width: 184
            height: 34
            radius: 6
            color: "#162033"
            border.color: "#38bdf8"
            border.width: 1

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 6
                anchors.rightMargin: 6
                spacing: 4

                QQC2.Slider {
                    id: sizeSlider
                    Layout.preferredWidth: 88
                    Layout.fillWidth: false
                    from: 0.8
                    to: 3.0
                    stepSize: 0.05
                    value: 1.5
                    onMoved: {
                        oledWidgetWindow.pendingScaleMultiplier = value
                        sliderResizeTimer.restart()
                    }
                }

                Text {
                    text: sizeSlider.value.toFixed(1) + "×"
                    color: "#38bdf8"
                    font.family: "monospace"
                    font.bold: true
                    font.pixelSize: 11
                    Layout.preferredWidth: 28
                }

                QQC2.ToolButton {
                    id: resetScaleBtn
                    Layout.preferredWidth: 26
                    Layout.preferredHeight: 22
                    implicitWidth: 26
                    implicitHeight: 22
                    padding: 0
                    onClicked: {
                        sizeSlider.value = 1.5
                        oledWidgetWindow.pendingScaleMultiplier = 1.5
                        oledWidgetWindow.applyScalePreset(1.5)
                    }
                    contentItem: Text {
                        text: "1×"
                        color: "#f8fafc"
                        font.family: "monospace"
                        font.bold: true
                        font.pixelSize: 10
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 4
                        color: resetScaleBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: "#344663"
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Reset to default size (1.5×)")
                }

                QQC2.ToolButton {
                    id: closeScaleBtn
                    Layout.preferredWidth: 22
                    Layout.preferredHeight: 22
                    implicitWidth: 22
                    implicitHeight: 22
                    padding: 0
                    onClicked: {
                        oledWidgetWindow.showSizeSlider = false
                        scaleIconCanvas.requestPaint()
                    }
                    contentItem: Text {
                        text: "✓"
                        color: "#38bdf8"
                        font.bold: true
                        font.pixelSize: 11
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 4
                        color: closeScaleBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: "#38bdf8"
                    }
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Close size slider")
                }
            }
        }

        // Bottom Rotary Knob Summary (with Rotary Knob Icons 1 & 2) & Corner Resize Grip
        Rectangle {
            id: bottomFooterBar
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.margins: oledWidgetWindow.outerPad
            height: oledWidgetWindow.footerBarHeight
            radius: 6
            color: "#111827"
            border.color: "#1e293b"
            border.width: 1

            readonly property int knobDiameter: Math.max(16, Math.round(oledWidgetWindow.oledSide * 0.088))
            readonly property int encFontSize: Math.max(11, Math.round(oledWidgetWindow.oledSide * 0.054))

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: Math.max(6, Math.round(8 * oledWidgetWindow.uiScale))
                anchors.rightMargin: 4
                anchors.topMargin: 2
                anchors.bottomMargin: 2
                spacing: 4

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignVCenter
                    spacing: Math.max(2, Math.round(3 * oledWidgetWindow.uiScale))

                    // Rotary Knob 1 Row
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Math.max(5, Math.round(6 * oledWidgetWindow.uiScale))

                        // Rotary Knob 1 Icon
                        Rectangle {
                            width: bottomFooterBar.knobDiameter
                            height: bottomFooterBar.knobDiameter
                            radius: width / 2
                            color: "#0f172a"
                            border.color: "#38bdf8"
                            border.width: Math.max(1.5, width * 0.09)

                            // Top indicator notch on rotary dial
                            Rectangle {
                                width: Math.max(2, Math.round(parent.width * 0.14))
                                height: Math.max(3, Math.round(parent.height * 0.22))
                                radius: 1
                                color: "#38bdf8"
                                anchors.top: parent.top
                                anchors.topMargin: 1
                                anchors.horizontalCenter: parent.horizontalCenter
                            }

                            Text {
                                anchors.centerIn: parent
                                anchors.verticalCenterOffset: 1
                                text: "1"
                                color: "#f8fafc"
                                font.family: "sans-serif"
                                font.bold: true
                                font.pixelSize: Math.max(9, Math.round(parent.height * 0.56))
                            }
                        }

                        Text {
                            text: oledWidgetWindow.enc1Summary
                            font.family: "monospace"
                            font.bold: true
                            font.pixelSize: bottomFooterBar.encFontSize
                            color: "#7dd3fc"
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }
                    }

                    // Rotary Knob 2 Row
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Math.max(5, Math.round(6 * oledWidgetWindow.uiScale))

                        // Rotary Knob 2 Icon
                        Rectangle {
                            width: bottomFooterBar.knobDiameter
                            height: bottomFooterBar.knobDiameter
                            radius: width / 2
                            color: "#0f172a"
                            border.color: "#38bdf8"
                            border.width: Math.max(1.5, width * 0.09)

                            // Top indicator notch on rotary dial
                            Rectangle {
                                width: Math.max(2, Math.round(parent.width * 0.14))
                                height: Math.max(3, Math.round(parent.height * 0.22))
                                radius: 1
                                color: "#38bdf8"
                                anchors.top: parent.top
                                anchors.topMargin: 1
                                anchors.horizontalCenter: parent.horizontalCenter
                            }

                            Text {
                                anchors.centerIn: parent
                                anchors.verticalCenterOffset: 1
                                text: "2"
                                color: "#f8fafc"
                                font.family: "sans-serif"
                                font.bold: true
                                font.pixelSize: Math.max(9, Math.round(parent.height * 0.56))
                            }
                        }

                        Text {
                            text: oledWidgetWindow.enc2Summary
                            font.family: "monospace"
                            font.bold: true
                            font.pixelSize: bottomFooterBar.encFontSize
                            color: "#7dd3fc"
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }
                    }
                }

                // Bottom-Right Resize Grip
                Rectangle {
                    width: Math.max(18, Math.round(20 * oledWidgetWindow.uiScale))
                    height: width
                    radius: 4
                    color: resizeMouse.containsMouse ? "#1e293b" : "transparent"

                    Text {
                        anchors.centerIn: parent
                        text: "◢"
                        font.pixelSize: Math.max(11, Math.round(13 * oledWidgetWindow.uiScale))
                        color: "#64748b"
                    }

                    MouseArea {
                        id: resizeMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.SizeFDiagCursor
                        onPressed: {
                            oledWidgetWindow.startSystemResize(Qt.RightEdge | Qt.BottomEdge)
                        }
                    }

                    QQC2.ToolTip.visible: resizeMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("Drag to resize OLED Desktop Widget")
                }
            }
        }

        // Center 1:1 Aspect-Preserving OLED Bezel (Pure GPU Scene-Graph -> Zero Resize Freeze)
        Item {
            id: oledViewportContainer
            anchors.top: topHeaderBar.bottom
            anchors.bottom: bottomFooterBar.top
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.topMargin: oledWidgetWindow.sectionGap
            anchors.bottomMargin: oledWidgetWindow.sectionGap
            anchors.leftMargin: oledWidgetWindow.outerPad
            anchors.rightMargin: oledWidgetWindow.outerPad

            Rectangle {
                id: oledBezel
                width: oledWidgetWindow.oledSide
                height: oledWidgetWindow.oledSide
                anchors.centerIn: parent
                radius: 8
                color: "#000000"
                border.color: "#38bdf8"
                border.width: 2
                clip: true

                MouseArea {
                    anchors.fill: parent
                    acceptedButtons: Qt.NoButton
                    onWheel: (wheel) => {
                        if (!oledWidgetWindow.controller) return
                        if (wheel.angleDelta.y > 0) {
                            const prev = Math.max(1, oledWidgetWindow.activeProfile - 1)
                            oledWidgetWindow.controller.switchProfile(prev)
                        } else if (wheel.angleDelta.y < 0) {
                            const next = Math.min(64, oledWidgetWindow.activeProfile + 1)
                            oledWidgetWindow.controller.switchProfile(next)
                        }
                    }
                }

                Item {
                    id: oledScreenContent
                    anchors.fill: parent
                    anchors.margins: 4

                    readonly property int bannerHeight: Math.max(14, Math.round(height * 0.105))
                    readonly property int gridAreaHeight: Math.max(80, height - bannerHeight - 2)
                    readonly property real cellWidth: width / 4.0
                    readonly property real cellHeight: gridAreaHeight / 5.0

                    // Top Profile Banner
                    Rectangle {
                        id: oledBanner
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        height: oledScreenContent.bannerHeight
                        color: "#38bdf8"

                        Text {
                            anchors.fill: parent
                            anchors.leftMargin: 4
                            anchors.rightMargin: 4
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            text: "P" + oledWidgetWindow.activeProfile + ": " + oledWidgetWindow.activeProfileName
                            color: "#020617"
                            font.family: "sans-serif"
                            font.bold: true
                            font.pixelSize: Math.max(9, Math.round(parent.height * 0.68))
                            elide: Text.ElideRight
                        }
                    }

                    // 4x5 Key Grid with Centered Button Text
                    Item {
                        anchors.top: oledBanner.bottom
                        anchors.topMargin: 2
                        anchors.bottom: parent.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right

                        Repeater {
                            model: 20

                            delegate: Rectangle {
                                required property int index
                                readonly property int slotNum: index + 1
                                readonly property var itemData: {
                                    // Re-evaluate whenever slotItems changes
                                    const _ = oledWidgetWindow.slotItems
                                    return oledWidgetWindow.slotDataFor(slotNum)
                                }
                                readonly property string l1: itemData.l1 || ""
                                readonly property string l2: itemData.l2 || ""
                                readonly property bool isSel: oledWidgetWindow.selectedSlot === slotNum
                                readonly property int colIdx: index % 4
                                readonly property int rowIdx: Math.floor(index / 4)

                                x: Math.round(colIdx * oledScreenContent.cellWidth)
                                y: Math.round(rowIdx * oledScreenContent.cellHeight)
                                width: Math.round((colIdx + 1) * oledScreenContent.cellWidth) - x
                                height: Math.round((rowIdx + 1) * oledScreenContent.cellHeight) - y

                                color: isSel ? "#38bdf8" : "#000000"
                                border.color: isSel ? "#38bdf8" : "#334155"
                                border.width: 1
                                clip: true

                                // RGB LED indicator dot
                                Rectangle {
                                    visible: oledWidgetWindow.showRgbDots
                                        && !parent.isSel
                                        && (parent.l1.length > 0 || parent.l2.length > 0 || !!parent.itemData.hasScript)
                                    anchors.top: parent.top
                                    anchors.right: parent.right
                                    anchors.margins: Math.max(2, Math.round(parent.height * 0.08))
                                    width: Math.max(3, Math.round(parent.height * 0.14))
                                    height: width
                                    radius: width / 2
                                    color: Qt.rgba(
                                        (parent.itemData.r !== undefined ? parent.itemData.r : 0) / 255.0,
                                        (parent.itemData.g !== undefined ? parent.itemData.g : 170) / 255.0,
                                        (parent.itemData.b !== undefined ? parent.itemData.b : 255) / 255.0,
                                        1.0
                                    )
                                }

                                // Centered Key Button Labels
                                Column {
                                    anchors.centerIn: parent
                                    width: parent.width - 4
                                    spacing: 0

                                    Text {
                                        visible: parent.parent.l1.length > 0
                                        width: parent.width
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        text: parent.parent.l1.slice(0, 6)
                                        color: parent.parent.isSel ? "#020617" : "#f8fafc"
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pixelSize: Math.max(8, Math.round(parent.parent.height * 0.34))
                                        fontSizeMode: Text.HorizontalFit
                                        minimumPixelSize: 6
                                        elide: Text.ElideRight
                                    }

                                    Text {
                                        visible: parent.parent.l2.length > 0
                                        width: parent.width
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        text: parent.parent.l2.slice(0, 6)
                                        color: parent.parent.isSel ? "#0f172a" : "#cbd5e1"
                                        font.family: "monospace"
                                        font.bold: true
                                        font.pixelSize: Math.max(7, Math.round(parent.parent.height * 0.30))
                                        fontSizeMode: Text.HorizontalFit
                                        minimumPixelSize: 6
                                        elide: Text.ElideRight
                                    }
                                }

                                MouseArea {
                                    anchors.fill: parent
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: {
                                        oledWidgetWindow.selectedSlot = (oledWidgetWindow.selectedSlot === parent.slotNum) ? 0 : parent.slotNum
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
