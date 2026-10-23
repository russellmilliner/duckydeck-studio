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
    height: 412 // 382 base + ~30 for 1-line title footer (expands down for multi-line titles, contracts for short)
    minimumWidth: 184
    minimumHeight: 264
    visible: false
    color: "transparent"

    property var controller: null
    property string targetDeviceId: ""
    property string targetDeviceLabel: ""
    property var targetDeviceState: null
    readonly property bool isTargetDeviceSelected: targetDeviceId.length === 0
        || (targetDeviceState !== null && !!targetDeviceState.isSelected)
    property int selectedSlot: 0
    property bool alwaysOnTop: true
    property bool framelessMode: true
    property bool showRgbDots: true
    property bool showSizeSlider: false
    property bool showTransparencySlider: false
    property int transparencyPercent: 0
    property bool keyEditorOpen: false
    readonly property bool autoSwitchDisabled: targetDeviceState !== null
        ? !targetDeviceState.autoSwitchEnabled
        : (controller && !controller.autoSwitchEnabled)
    readonly property bool autoSwitchPaused: autoSwitchDisabled || (keyEditorOpen && controller && controller.autoSwitchEnabled)
    readonly property string autoSwitchError: (isTargetDeviceSelected && controller)
        ? String(controller.autoSwitchErrorMessage || "")
        : ""
    readonly property bool hasAutoSwitchError: autoSwitchError.length > 0
    onAutoSwitchErrorChanged: Qt.callLater(updateTitleExpansion)
    readonly property real windowOpacity: Math.max(0.20, Math.min(1.0, (100 - transparencyPercent) / 100.0))
    readonly property int transparencyMax: 80
    readonly property int transparencyWheelStep: 5
    signal requestOpenMainWindow()
    property real wheelTransparencyAccum: 0

    // Active window title footer (KWin focus tracking via controller.activeWindowTitle)
    property bool showWindowTitle: true
    property string lastExternalWindowTitle: ""
    property string lastExternalWindowClass: ""
    property int currentExtraTitleHeight: 0
    property real userBaseHeight: 382
    property int lockedTopY: -1
    property bool isAdjustingExpansion: false

    onYChanged: {
        if (!isAdjustingExpansion) {
            lockedTopY = y
        }
    }

    onHeightChanged: {
        if (!isAdjustingExpansion) {
            const titleAllowance = showWindowTitle ? (baseTitleBarHeight + sectionGap + currentExtraTitleHeight) : 0
            userBaseHeight = Math.max(240, height - titleAllowance)
        }
    }

    function updateTitleExpansion() {
        if (!showWindowTitle) {
            if (currentExtraTitleHeight !== 0) {
                currentExtraTitleHeight = 0
                isAdjustingExpansion = true
                const savedY = lockedTopY >= 0 ? lockedTopY : y
                height = Math.max(minimumHeight, Math.round(userBaseHeight))
                if (savedY >= 0 && y !== savedY) {
                    y = savedY
                }
                isAdjustingExpansion = false
            }
            return
        }
        if (!windowTitleText) return
        const textH = windowTitleText.contentHeight
        const needed = Math.max(baseTitleBarHeight, Math.round(textH + 8))
        const newExtra = Math.max(0, needed - baseTitleBarHeight)
        const targetHeight = Math.max(minimumHeight, Math.round(userBaseHeight + baseTitleBarHeight + sectionGap + newExtra))
        if (newExtra !== currentExtraTitleHeight || height !== targetHeight) {
            isAdjustingExpansion = true
            currentExtraTitleHeight = newExtra
            const savedY = lockedTopY >= 0 ? lockedTopY : y
            height = targetHeight
            if (savedY >= 0 && y !== savedY) {
                y = savedY
            }
            isAdjustingExpansion = false
        }
    }

    function isOwnWindowClass(cls) {
        const c = String(cls || "").toLowerCase()
        return c.length === 0 || c === "listening for kwin..." || c.indexOf("duckydeck") !== -1
    }

    function refreshExternalWindowInfo() {
        if (!controller) return
        const cls = String(controller.activeWindowClass || "")
        // Ignore focus moving to DuckyDeck's own windows (including this widget) so
        // clicking the widget keeps showing the last real application title.
        if (isOwnWindowClass(cls)) return
        lastExternalWindowClass = cls
        lastExternalWindowTitle = String(controller.activeWindowTitle || "")
        Qt.callLater(updateTitleExpansion)
    }

    function setTransparency(percent) {
        const clamped = Math.max(0, Math.min(transparencyMax, Math.round(percent)))
        if (clamped !== transparencyPercent) {
            transparencyPercent = clamped
        }
        transparencySlider.value = clamped
        transparencyHud.flash()
    }

    // Scroll up = more opaque, scroll down = more transparent.
    // Accumulates high-resolution touchpad deltas into 120-unit notches.
    function handleTransparencyWheel(wheel) {
        const rawAngle = wheel.angleDelta.y
        const dy = rawAngle !== 0 ? rawAngle : wheel.pixelDelta.y * 4
        if (dy === 0) return
        wheelTransparencyAccum += dy
        const notches = wheelTransparencyAccum > 0
            ? Math.floor(wheelTransparencyAccum / 120)
            : Math.ceil(wheelTransparencyAccum / 120)
        if (notches === 0) return
        wheelTransparencyAccum -= notches * 120
        setTransparency(transparencyPercent - notches * transparencyWheelStep)
        wheel.accepted = true
    }

    onShowWindowTitleChanged: {
        windowTitleIconCanvas.requestPaint()
        isAdjustingExpansion = true
        const savedY = lockedTopY >= 0 ? lockedTopY : y
        if (showWindowTitle) {
            height = Math.max(minimumHeight, Math.round(userBaseHeight + baseTitleBarHeight + sectionGap + currentExtraTitleHeight))
            if (savedY >= 0 && y !== savedY) {
                y = savedY
            }
            isAdjustingExpansion = false
            Qt.callLater(updateTitleExpansion)
        } else {
            currentExtraTitleHeight = 0
            height = Math.max(minimumHeight, Math.round(userBaseHeight))
            if (savedY >= 0 && y !== savedY) {
                y = savedY
            }
            isAdjustingExpansion = false
        }
    }

    onTransparencyPercentChanged: {
        transparencyIconCanvas.requestPaint()
        if (controller) {
            controller.setOledWidgetKeepAbove(alwaysOnTop)
        }
    }

    onShowRgbDotsChanged: {
        if (controller) {
            controller.setOledWidgetKeepAbove(alwaysOnTop)
        }
    }

    transientParent: null

    flags: Qt.Window
        | (alwaysOnTop ? Qt.WindowStaysOnTopHint : Qt.Widget)
        | (framelessMode ? Qt.FramelessWindowHint : Qt.Widget)

    onFramelessModeChanged: {
        titlebarIconCanvas.requestPaint()
        if (controller) {
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
            Qt.callLater(updateTitleExpansion)
        } else {
            showSizeSlider = false
            showTransparencySlider = false
        }
    }

    readonly property bool closableWithEscape: true
    readonly property real fontScale: (controller && controller.uiTextScale > 0
        ? controller.uiTextScale
        : 100) / 100.0
    readonly property int activeProfile: targetDeviceState !== null && targetDeviceState.activeProfile !== undefined
        ? Number(targetDeviceState.activeProfile)
        : (controller ? controller.activeProfile : 1)
    readonly property string activeProfileName: targetDeviceState !== null && targetDeviceState.activeProfileName
        ? String(targetDeviceState.activeProfileName)
        : ((controller && controller.activeProfileName)
            ? controller.activeProfileName
            : "Default")

    Shortcut {
        sequence: "Escape"
        context: Qt.WindowShortcut
        enabled: oledWidgetWindow.visible
        onActivated: oledWidgetWindow.close()
    }

    // Dynamic Window Title Footer expansion metrics
    readonly property int baseTitleBarHeight: Math.max(22, Math.round(windowTitleFontPx * 1.35) + 8)
    readonly property int currentTitleBarHeight: showWindowTitle ? (baseTitleBarHeight + currentExtraTitleHeight) : 0
    readonly property int windowTitleReserved: showWindowTitle ? (baseTitleBarHeight + sectionGap) : 0

    // Pure top-down geometry calculations (based on userBaseHeight so title expansion never loops)
    readonly property real baseWindowHeight: Math.max(240, userBaseHeight)
    readonly property real uiScale: Math.max(0.85, Math.min(width / 260.0, baseWindowHeight / 366.0))
    readonly property int outerPad: Math.max(6, Math.round(8 * uiScale))
    readonly property int sectionGap: Math.max(4, Math.round(6 * uiScale))
    readonly property int headerBarHeight: Math.max(26, Math.round(28 * uiScale))
    readonly property int footerBarHeight: isDuckyPad2020
        ? Math.max(32, Math.round(34 * uiScale))
        : Math.max(70, Math.round(78 * uiScale))
    readonly property int windowTitleFontPx: Math.max(10, Math.round(10 * uiScale))
    readonly property int availOledWidth: Math.max(120, width - outerPad * 2)
    readonly property int availOledHeight: Math.max(120, baseWindowHeight - outerPad * 2 - headerBarHeight - footerBarHeight - sectionGap * 2)
    readonly property int oledSide: Math.max(120, Math.min(availOledWidth, availOledHeight))
    readonly property real currentScaleMultiplier: Math.max(0.8, Math.min(3.0, oledSide / 128.0))

    property real pendingScaleMultiplier: 1.5
    property int profileHardwareOledRotation: 0
    property int profileOledRotation: 0
    readonly property int effectiveRotation: profileOledRotation
    readonly property bool isDuckyPad2020: targetDeviceState !== null && targetDeviceState.isDuckyPad2020 !== undefined
        ? !!targetDeviceState.isDuckyPad2020
        : (controller ? !!controller.isDuckyPad2020 : false)
    readonly property int onboardKeyCount: targetDeviceState !== null && targetDeviceState.onboardKeyCount > 0
        ? Number(targetDeviceState.onboardKeyCount)
        : ((controller && controller.onboardKeyCount > 0)
            ? controller.onboardKeyCount
            : (isDuckyPad2020 ? 15 : 20))
    readonly property int basePortraitCols: isDuckyPad2020 ? 3 : 4
    readonly property int gridCols: (profileOledRotation === 90 || profileOledRotation === 270) ? 5 : basePortraitCols
    readonly property int gridRows: (profileOledRotation === 90 || profileOledRotation === 270) ? basePortraitCols : 5
    property int dataRevision: 0
    property var slotItems: []
    property string enc1Summary: "CCW · CW · [Push]"
    property string enc2Summary: "CCW · CW · [Push]"

    function slotForCell(colIdx, rowIdx) {
        const rot = oledWidgetWindow.profileOledRotation
        const pCols = oledWidgetWindow.basePortraitCols
        if (rot === 90) {
            return colIdx * pCols + ((pCols - 1) - rowIdx) + 1
        } else if (rot === 180) {
            return (4 - rowIdx) * pCols + ((pCols - 1) - colIdx) + 1
        } else if (rot === 270) {
            return (4 - colIdx) * pCols + rowIdx + 1
        }
        return rowIdx * pCols + colIdx + 1
    }

    function cellSlotForIndex(idx) {
        const item = keyCellRepeater.itemAt(idx)
        return item ? item.slotNum : 0
    }

    function cellDisp1ForIndex(idx) {
        const item = keyCellRepeater.itemAt(idx)
        return item ? String(item.disp1 || "") : ""
    }

    function cellTextRotationForIndex(idx) {
        const item = keyCellRepeater.itemAt(idx)
        return item && item.labelRotation !== undefined ? Number(item.labelRotation) : 0
    }

    onProfileOledRotationChanged: {
        oledWidgetWindow.dataRevision += 1
    }

    Timer {
        id: sliderResizeTimer
        interval: 16
        repeat: false
        onTriggered: {
            oledWidgetWindow.applyScalePreset(oledWidgetWindow.pendingScaleMultiplier)
        }
    }

    function visibleOledChars(str) {
        if (!str) return []
        const raw = Array.from(String(str))
        const out = []
        for (let i = 0; i < raw.length; ++i) {
            const ch = raw[i]
            if (ch === "\uFE0E" || ch === "\uFE0F" || ch === "\u200D") {
                if (out.length > 0) out[out.length - 1] += ch
                continue
            }
            out.push(ch)
        }
        return out
    }

    function sliceOledLabel(str, maxChars) {
        return oledWidgetWindow.visibleOledChars(str).slice(0, maxChars || 5).join("")
    }

    function isSingleIconLabel(str) {
        const trimmed = String(str || "").trim()
        if (!trimmed) return false
        if (trimmed === ">_" || trimmed === "{}") return true
        const chars = oledWidgetWindow.visibleOledChars(trimmed)
        if (chars.length !== 1) return false
        return chars[0].codePointAt(0) > 127
    }

    function formatEnc(item, fallback) {
        if (!item) return fallback
        const wl1 = (item.wl1 || "").trim()
        const wl2 = (item.wl2 || "").trim()
        const l1 = wl1.length > 0 ? wl1 : (item.l1 || "").trim()
        const l2 = wl2.length > 0 ? wl2 : (item.l2 || "").trim()
        const c = (l1 + (l2.length > 0 ? " " + l2 : "")).trim()
        return c.length > 0 ? c : fallback
    }

    readonly property var emptySlotObj: ({ slot: 0, l1: "", l2: "", wl1: "", wl2: "", r: 0, g: 170, b: 255, hasScript: false })
    property bool syncQueued: false

    function scheduleSyncFromController() {
        if (syncQueued) return
        syncQueued = true
        Qt.callLater(function() {
            syncQueued = false
            oledWidgetWindow.syncFromController()
        })
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
        return emptySlotObj
    }

    function applySlotsArray(arr) {
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
                    wl1: it.wl1 || "",
                    wl2: it.wl2 || "",
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
        oledWidgetWindow.dataRevision += 1
    }

    function syncFromController() {
        if (!controller) return
        if (targetDeviceId.length > 0 && controller.multiDeviceOledStatesJson) {
            try {
                const mapObj = JSON.parse(controller.multiDeviceOledStatesJson)
                if (mapObj && typeof mapObj === "object") {
                    let st = mapObj[targetDeviceId]
                    if (!st) {
                        const targetUpper = targetDeviceId.toUpperCase()
                        const keys = Object.keys(mapObj)
                        for (let i = 0; i < keys.length; ++i) {
                            if (keys[i].toUpperCase() === targetUpper) {
                                st = mapObj[keys[i]]
                                break
                            }
                        }
                    }
                    if (st) {
                        targetDeviceState = st
                        oledWidgetWindow.profileHardwareOledRotation = Number(st.profileHardwareOledRotation || 0)
                        oledWidgetWindow.profileOledRotation = Number(st.profileOledRotation || 0)
                        if (Array.isArray(st.slots)) {
                            applySlotsArray(st.slots)
                        }
                        return
                    }
                }
            } catch (e) {}
        }
        targetDeviceState = null
        if (controller.profilesJson) {
            try {
                const parr = JSON.parse(controller.profilesJson)
                if (Array.isArray(parr)) {
                    for (let i = 0; i < parr.length; ++i) {
                        const p = parr[i]
                        if (p.number === controller.activeProfile) {
                            const hwRot = p.oledRotation !== undefined
                                ? Number(p.oledRotation)
                                : (p.isLandscape ? 90 : 0)
                            const nextRot =
                                p.effectiveWidgetRotation !== undefined
                                    ? Number(p.effectiveWidgetRotation)
                                    : hwRot
                            oledWidgetWindow.profileHardwareOledRotation = hwRot
                            oledWidgetWindow.profileOledRotation = nextRot
                            break
                        }
                    }
                }
            } catch (e) {}
        }
        if (!controller.activeProfileSlotsJson) return
        try {
            const arr = JSON.parse(controller.activeProfileSlotsJson)
            applySlotsArray(arr)
        } catch (e) {}
    }

    function applyScalePreset(mult) {
        const clamped = Math.max(0.8, Math.min(3.0, mult))
        const targetOled = Math.round(128 * clamped)
        const nextBaseH = Math.max(240, targetOled + Math.round(140 * Math.max(1.0, clamped * 0.75)))
        oledWidgetWindow.isAdjustingExpansion = true
        oledWidgetWindow.userBaseHeight = nextBaseH
        oledWidgetWindow.width = Math.max(minimumWidth, targetOled + 32)
        oledWidgetWindow.height = Math.max(minimumHeight, nextBaseH + (showWindowTitle ? baseTitleBarHeight + sectionGap + currentExtraTitleHeight : 0))
        oledWidgetWindow.isAdjustingExpansion = false
    }

    Connections {
        target: oledWidgetWindow.controller
        function onActiveProfileSlotsJsonChanged() {
            oledWidgetWindow.scheduleSyncFromController()
        }
        function onActiveProfileChanged() {
            oledWidgetWindow.scheduleSyncFromController()
        }
        function onActiveProfileNameChanged() {
            oledWidgetWindow.scheduleSyncFromController()
        }
        function onProfilesJsonChanged() {
            oledWidgetWindow.scheduleSyncFromController()
        }
        function onMultiDeviceOledStatesJsonChanged() {
            oledWidgetWindow.scheduleSyncFromController()
        }
        function onActiveWindowTitleChanged() {
            oledWidgetWindow.refreshExternalWindowInfo()
        }
        function onActiveWindowClassChanged() {
            oledWidgetWindow.refreshExternalWindowInfo()
        }
    }

    Component.onCompleted: {
        transientParent = null
        syncFromController()
        refreshExternalWindowInfo()
        Qt.callLater(updateTitleExpansion)
    }

    Rectangle {
        id: outerShell
        anchors.fill: parent
        radius: oledWidgetWindow.framelessMode ? 10 : 0
        color: "#0b0f19"
        border.width: oledWidgetWindow.framelessMode ? 1.5 : 1
        border.color: oledWidgetWindow.hasAutoSwitchError
            ? "#ef4444"
            : (oledWidgetWindow.autoSwitchPaused
                ? "#f59e0b"
                : (oledWidgetWindow.alwaysOnTop ? "#38bdf8" : "#334155"))
        opacity: oledWidgetWindow.windowOpacity
        layer.enabled: oledWidgetWindow.windowOpacity < 0.999

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
            border.color: oledWidgetWindow.autoSwitchPaused ? "#f59e0b" : "#1e293b"
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
                onWheel: (wheel) => oledWidgetWindow.handleTransparencyWheel(wheel)
            }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 6
                anchors.rightMargin: 4
                spacing: 5

                // DuckyDeck Studio Application Icon
                Image {
                    id: appIcon
                    width: Math.max(16, Math.round(16 * oledWidgetWindow.uiScale))
                    height: width
                    Layout.preferredWidth: width
                    Layout.preferredHeight: height
                    Layout.alignment: Qt.AlignVCenter
                    source: "qrc:/qt/qml/io/github/duckydeck/src/qml/icons/duckydeck-studio.svg"
                    sourceSize.width: width * 2
                    sourceSize.height: height * 2
                    fillMode: Image.PreserveAspectFit
                    smooth: true
                    mipmap: true

                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: iconMouse.containsMouse
                    QQC2.ToolTip.text: qsTr("DuckyDeck Studio — Click to open main window")

                    MouseArea {
                        id: iconMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        acceptedButtons: Qt.LeftButton
                        onClicked: oledWidgetWindow.requestOpenMainWindow()
                        onWheel: (wheel) => oledWidgetWindow.handleTransparencyWheel(wheel)
                    }
                }

                Rectangle {
                    Layout.preferredWidth: Math.max(7, Math.round(8 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Layout.preferredWidth
                    Layout.alignment: Qt.AlignVCenter
                    width: Layout.preferredWidth
                    height: Layout.preferredHeight
                    radius: width / 2
                    color: (oledWidgetWindow.controller && oledWidgetWindow.controller.connected)
                        ? "#22c55e"
                        : "#38bdf8"

                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: dotMouse.containsMouse
                    QQC2.ToolTip.text: (oledWidgetWindow.controller && oledWidgetWindow.controller.connected)
                        ? qsTr("Connected · P%1: %2").arg(oledWidgetWindow.activeProfile).arg(oledWidgetWindow.activeProfileName)
                        : qsTr("Disconnected · P%1: %2").arg(oledWidgetWindow.activeProfile).arg(oledWidgetWindow.activeProfileName)

                    MouseArea {
                        id: dotMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        acceptedButtons: Qt.NoButton
                    }
                }

                // Multi-Device Identification Pill (shown when multiple duckyPads are connected or bound to a specific device)
                Rectangle {
                    visible: oledWidgetWindow.targetDeviceLabel.length > 0
                        || oledWidgetWindow.targetDeviceId.length > 0
                        || (oledWidgetWindow.controller && oledWidgetWindow.controller.connectedDeviceCount > 1)
                    Layout.preferredHeight: Math.max(16, Math.round(18 * oledWidgetWindow.uiScale))
                    Layout.preferredWidth: devBadgeLabel.implicitWidth + 10
                    radius: height / 2
                    color: oledWidgetWindow.isDuckyPad2020
                        ? Qt.rgba(0.66, 0.33, 0.97, 0.22)
                        : Qt.rgba(0.22, 0.74, 0.97, 0.22)
                    border.width: 1
                    border.color: oledWidgetWindow.isDuckyPad2020 ? "#a855f7" : "#38bdf8"
                    Layout.alignment: Qt.AlignVCenter

                    QQC2.Label {
                        id: devBadgeLabel
                        anchors.centerIn: parent
                        text: {
                            if (oledWidgetWindow.targetDeviceLabel.length > 0) {
                                return oledWidgetWindow.targetDeviceLabel
                            }
                            if (oledWidgetWindow.targetDeviceId.length > 0) {
                                const s = oledWidgetWindow.targetDeviceId
                                return (oledWidgetWindow.isDuckyPad2020 ? "DP20·" : "DP24·") + s.slice(-4)
                            }
                            const sn = oledWidgetWindow.controller ? String(oledWidgetWindow.controller.serialNumber || "") : ""
                            const suffix = (sn.length >= 4 && sn !== "-") ? ("·" + sn.slice(-4)) : ""
                            return (oledWidgetWindow.isDuckyPad2020 ? "DP20" : "DP24") + suffix
                        }
                        font.bold: true
                        font.pixelSize: Math.max(8, Math.round(9 * oledWidgetWindow.uiScale))
                        font.family: "monospace"
                        color: oledWidgetWindow.isDuckyPad2020 ? "#c084fc" : "#38bdf8"
                    }
                }

                // Auto-Switcher Paused / Disabled Indicator Pill
                Rectangle {
                    visible: oledWidgetWindow.autoSwitchPaused
                    Layout.preferredHeight: Math.max(16, Math.round(18 * oledWidgetWindow.uiScale))
                    Layout.preferredWidth: pausedLabel.implicitWidth + 10
                    radius: height / 2
                    color: Qt.rgba(0.96, 0.62, 0.04, 0.22)
                    border.width: 1
                    border.color: "#f59e0b"
                    Layout.alignment: Qt.AlignVCenter

                    QQC2.Label {
                        id: pausedLabel
                        anchors.centerIn: parent
                        text: qsTr("PAUSED")
                        font.bold: true
                        font.pixelSize: Math.max(8, Math.round(9 * oledWidgetWindow.uiScale))
                        font.family: "monospace"
                        color: "#f59e0b"
                    }

                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: pausedMouse.containsMouse
                    QQC2.ToolTip.text: oledWidgetWindow.autoSwitchDisabled
                        ? qsTr("Auto-Switcher is disabled in settings")
                        : qsTr("Auto-Switcher paused while Key Editor is open in DuckyDeck Studio")

                    MouseArea {
                        id: pausedMouse
                        anchors.fill: parent
                        hoverEnabled: true
                        acceptedButtons: Qt.NoButton
                    }
                }

                // Flexible spacer / drag region between connection dot and control buttons
                Item {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
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
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: oledWidgetWindow.alwaysOnTop
                        ? qsTr("Always on Top: ON (Click to unpin)")
                        : qsTr("Always on Top: OFF (Click to keep above all windows)")
                }

                // Window Transparency Slider Toggle Button
                QQC2.ToolButton {
                    id: transparencyToggleBtn
                    Layout.preferredWidth: Math.max(24, Math.round(24 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        if (!oledWidgetWindow.showTransparencySlider) {
                            transparencySlider.value = oledWidgetWindow.transparencyPercent
                            oledWidgetWindow.showSizeSlider = false
                            scaleIconCanvas.requestPaint()
                        }
                        oledWidgetWindow.showTransparencySlider = !oledWidgetWindow.showTransparencySlider
                        transparencyIconCanvas.requestPaint()
                    }
                    contentItem: Item {
                        Canvas {
                            id: transparencyIconCanvas
                            width: 16
                            height: 16
                            anchors.centerIn: parent
                            scale: Math.max(0.85, Math.min(1.35, oledWidgetWindow.uiScale * 0.92))
                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.reset()
                                const active = oledWidgetWindow.showTransparencySlider || oledWidgetWindow.transparencyPercent > 0
                                const col = active ? "#38bdf8" : "#cbd5e1"
                                ctx.strokeStyle = col
                                ctx.fillStyle = col
                                ctx.lineWidth = 1.5

                                // Outer circle representing contrast / opacity dial
                                ctx.beginPath()
                                ctx.arc(8, 8, 5.5, 0, Math.PI * 2)
                                ctx.stroke()

                                // Left half filled solid, right half semi-transparent / hatched
                                ctx.beginPath()
                                ctx.arc(8, 8, 5.5, Math.PI * 0.5, Math.PI * 1.5)
                                ctx.closePath()
                                ctx.fill()

                                ctx.fillStyle = Qt.rgba(0.22, 0.74, 0.97, 0.35)
                                ctx.beginPath()
                                ctx.arc(8, 8, 5.5, -Math.PI * 0.5, Math.PI * 0.5)
                                ctx.closePath()
                                ctx.fill()
                            }
                        }
                    }
                    background: Rectangle {
                        radius: 4
                        color: (oledWidgetWindow.showTransparencySlider || oledWidgetWindow.transparencyPercent > 0)
                            ? Qt.rgba(0.22, 0.74, 0.97, 0.22)
                            : (transparencyToggleBtn.hovered ? "#25334a" : "#1a2333")
                        border.width: 1
                        border.color: (oledWidgetWindow.showTransparencySlider || oledWidgetWindow.transparencyPercent > 0)
                            ? "#38bdf8"
                            : "#344663"
                    }
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Window Transparency: %1% (Click to adjust · Scroll to change)").arg(oledWidgetWindow.transparencyPercent)

                    // Scroll directly over the button to change transparency
                    MouseArea {
                        anchors.fill: parent
                        acceptedButtons: Qt.NoButton
                        onWheel: (wheel) => oledWidgetWindow.handleTransparencyWheel(wheel)
                    }
                }

                // Active Window Title Footer Toggle Button
                QQC2.ToolButton {
                    id: windowTitleToggleBtn
                    objectName: "oledWidgetWindowTitleToggle"
                    Layout.preferredWidth: Math.max(24, Math.round(24 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        oledWidgetWindow.showWindowTitle = !oledWidgetWindow.showWindowTitle
                    }
                    contentItem: Item {
                        Canvas {
                            id: windowTitleIconCanvas
                            width: 16
                            height: 16
                            anchors.centerIn: parent
                            scale: Math.max(0.85, Math.min(1.35, oledWidgetWindow.uiScale * 0.92))
                            onPaint: {
                                const ctx = getContext("2d")
                                ctx.reset()
                                const active = oledWidgetWindow.showWindowTitle
                                const col = active ? "#38bdf8" : "#cbd5e1"
                                ctx.strokeStyle = col
                                ctx.fillStyle = col
                                ctx.lineWidth = 1.5
                                ctx.lineCap = "round"

                                // Window frame
                                ctx.beginPath()
                                ctx.rect(2, 2.5, 12, 11)
                                ctx.stroke()

                                // Text lines at the bottom (the title caption)
                                ctx.beginPath()
                                ctx.moveTo(4.5, 9)
                                ctx.lineTo(11.5, 9)
                                ctx.moveTo(4.5, 11.5)
                                ctx.lineTo(9, 11.5)
                                ctx.stroke()

                                // Strike-through when hidden
                                if (!active) {
                                    ctx.beginPath()
                                    ctx.moveTo(1.5, 14.5)
                                    ctx.lineTo(14.5, 1.5)
                                    ctx.stroke()
                                }
                            }
                        }
                    }
                    background: Rectangle {
                        radius: 4
                        color: oledWidgetWindow.showWindowTitle
                            ? Qt.rgba(0.22, 0.74, 0.97, 0.22)
                            : (windowTitleToggleBtn.hovered ? "#25334a" : "#1a2333")
                        border.width: 1
                        border.color: oledWidgetWindow.showWindowTitle ? "#38bdf8" : "#344663"
                    }
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: oledWidgetWindow.showWindowTitle
                        ? qsTr("Active Window Title: shown (Click to hide)")
                        : qsTr("Active Window Title: hidden (Click to show)")
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
                            oledWidgetWindow.showTransparencySlider = false
                            transparencyIconCanvas.requestPaint()
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
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Adjust OLED Widget Size")
                }

                // Desktop Widget Orientation Cycle Button (0° -> 90° -> 180° -> 270° -> 0°)
                QQC2.ToolButton {
                    id: oledRotCycleBtn
                    Layout.preferredWidth: Math.max(30, Math.round(32 * oledWidgetWindow.uiScale))
                    Layout.preferredHeight: Math.max(20, Math.round(20 * oledWidgetWindow.uiScale))
                    implicitWidth: Layout.preferredWidth
                    implicitHeight: Layout.preferredHeight
                    padding: 0
                    onClicked: {
                        const cur = oledWidgetWindow.profileOledRotation
                        const nextRot = cur === 0 ? 90 : (cur === 90 ? 180 : (cur === 180 ? 270 : 0))
                        oledWidgetWindow.profileOledRotation = nextRot
                        if (oledWidgetWindow.controller) {
                            const targetArg = (nextRot === oledWidgetWindow.profileHardwareOledRotation) ? -1 : nextRot
                            if (oledWidgetWindow.targetDeviceId.length > 0) {
                                oledWidgetWindow.controller.setDeviceProfileWidgetRotation(oledWidgetWindow.targetDeviceId, targetArg)
                            } else {
                                oledWidgetWindow.controller.setActiveProfileWidgetRotation(targetArg)
                            }
                        }
                    }
                    contentItem: Text {
                        text: oledWidgetWindow.profileOledRotation + "°"
                        color: oledWidgetWindow.profileOledRotation !== 0 ? "#38bdf8" : "#cbd5e1"
                        font.family: "monospace"
                        font.bold: true
                        font.pointSize: Math.max(10, Math.round(10 * oledWidgetWindow.uiScale))
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 4
                        color: oledWidgetWindow.profileOledRotation !== 0
                            ? Qt.rgba(0.22, 0.74, 0.97, 0.22)
                            : (oledRotCycleBtn.hovered ? "#25334a" : "#1a2333")
                        border.width: 1
                        border.color: oledWidgetWindow.profileOledRotation !== 0 ? "#38bdf8" : "#344663"
                    }
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Desktop Widget Orientation: %1° (Click to cycle 0° / 90° / 180° / 270°)").arg(oledWidgetWindow.profileOledRotation)
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
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: oledWidgetWindow.framelessMode
                        ? qsTr("Restore Window Titlebar")
                        : qsTr("Frameless Desktop Overlay Mode")
                }
            }
        }

        // Fixed-Geometry Interactive Transparency Slider Overlay
        Rectangle {
            id: transparencySliderOverlay
            visible: oledWidgetWindow.showTransparencySlider
            z: 25
            anchors.top: topHeaderBar.bottom
            anchors.topMargin: 4
            anchors.left: parent.left
            anchors.leftMargin: 8
            width: 194
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
                    id: transparencySlider
                    Layout.preferredWidth: 92
                    Layout.fillWidth: false
                    from: 0
                    to: 80
                    stepSize: 5
                    value: oledWidgetWindow.transparencyPercent
                    onMoved: {
                        oledWidgetWindow.transparencyPercent = Math.round(value)
                    }
                }

                Text {
                    text: oledWidgetWindow.transparencyPercent + "%"
                    color: "#38bdf8"
                    font.family: "monospace"
                    font.bold: true
                    font.pointSize: 10
                    Layout.preferredWidth: 32
                }

                QQC2.ToolButton {
                    id: resetTransparencyBtn
                    Layout.preferredWidth: 26
                    Layout.preferredHeight: 22
                    implicitWidth: 26
                    implicitHeight: 22
                    padding: 0
                    onClicked: {
                        transparencySlider.value = 0
                        oledWidgetWindow.transparencyPercent = 0
                    }
                    contentItem: Text {
                        text: "0%"
                        color: "#f8fafc"
                        font.family: "monospace"
                        font.bold: true
                        font.pointSize: 10
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 4
                        color: resetTransparencyBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: "#344663"
                    }
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Reset to opaque (0% transparency)")
                }

                QQC2.ToolButton {
                    id: closeTransparencyBtn
                    Layout.preferredWidth: 22
                    Layout.preferredHeight: 22
                    implicitWidth: 22
                    implicitHeight: 22
                    padding: 0
                    onClicked: {
                        oledWidgetWindow.showTransparencySlider = false
                        transparencyIconCanvas.requestPaint()
                    }
                    contentItem: Text {
                        text: "✓"
                        color: "#38bdf8"
                        font.bold: true
                        font.pointSize: 10
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 4
                        color: closeTransparencyBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: "#38bdf8"
                    }
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Close transparency slider")
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
                    font.pointSize: 10
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
                        font.pointSize: 10
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 4
                        color: resetScaleBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: "#344663"
                    }
                    QQC2.ToolTip.delay: 500
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
                        font.pointSize: 10
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 4
                        color: closeScaleBtn.hovered ? "#25334a" : "#1a2333"
                        border.width: 1
                        border.color: "#38bdf8"
                    }
                    QQC2.ToolTip.delay: 500
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.text: qsTr("Close size slider")
                }
            }
        }

        // Active Window Title Footer (multi-line, word-wrapped; positioned at the very bottom below profile buttons)
        Rectangle {
            id: windowTitleBar
            objectName: "oledWidgetWindowTitleBar"
            visible: oledWidgetWindow.showWindowTitle
            anchors.bottom: parent.bottom
            anchors.bottomMargin: oledWidgetWindow.outerPad
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.leftMargin: oledWidgetWindow.outerPad
            anchors.rightMargin: oledWidgetWindow.outerPad
            height: oledWidgetWindow.currentTitleBarHeight
            radius: 6
            color: oledWidgetWindow.hasAutoSwitchError
                ? Qt.tint("#111827", Qt.rgba(0.94, 0.27, 0.27, 0.22))
                : (oledWidgetWindow.autoSwitchPaused
                    ? Qt.tint("#111827", Qt.rgba(0.96, 0.62, 0.04, 0.16))
                    : "#111827")
            border.color: oledWidgetWindow.hasAutoSwitchError
                ? "#ef4444"
                : (oledWidgetWindow.autoSwitchPaused ? "#f59e0b" : "#1e293b")
            border.width: 1
            clip: true

            readonly property string titleText: {
                if (oledWidgetWindow.hasAutoSwitchError) {
                    return oledWidgetWindow.autoSwitchError
                }
                if (oledWidgetWindow.autoSwitchPaused) {
                    const winName = oledWidgetWindow.lastExternalWindowTitle.length > 0
                        ? oledWidgetWindow.lastExternalWindowTitle
                        : oledWidgetWindow.lastExternalWindowClass
                    if (winName.length > 0) {
                        return qsTr("⏸ Auto-Switcher Paused • %1").arg(winName)
                    }
                    return oledWidgetWindow.autoSwitchDisabled
                        ? qsTr("⏸ Auto-Switcher Paused (Disabled)")
                        : qsTr("⏸ Auto-Switcher Paused (Key Editor)")
                }
                return oledWidgetWindow.lastExternalWindowTitle.length > 0
                    ? oledWidgetWindow.lastExternalWindowTitle
                    : (oledWidgetWindow.lastExternalWindowClass.length > 0
                        ? oledWidgetWindow.lastExternalWindowClass
                        : qsTr("No active window detected"))
            }

            MouseArea {
                id: windowTitleMouse
                anchors.fill: parent
                acceptedButtons: Qt.NoButton
                hoverEnabled: true
                onWheel: (wheel) => oledWidgetWindow.handleTransparencyWheel(wheel)
            }

            Text {
                id: windowTitleText
                objectName: "oledWidgetWindowTitleText"
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter
                anchors.leftMargin: 8
                anchors.rightMargin: 8
                text: windowTitleBar.titleText
                color: oledWidgetWindow.hasAutoSwitchError
                    ? "#fca5a5"
                    : (oledWidgetWindow.autoSwitchPaused
                        ? "#f59e0b"
                        : (oledWidgetWindow.lastExternalWindowTitle.length > 0 ? "#e2e8f0" : "#64748b"))
                font.bold: oledWidgetWindow.hasAutoSwitchError || oledWidgetWindow.autoSwitchPaused || oledWidgetWindow.lastExternalWindowTitle.length > 0
                font.pixelSize: oledWidgetWindow.windowTitleFontPx
                font.italic: !oledWidgetWindow.hasAutoSwitchError && !oledWidgetWindow.autoSwitchPaused && oledWidgetWindow.lastExternalWindowTitle.length === 0
                wrapMode: Text.WrapAtWordBoundaryOrAnywhere
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                lineHeight: 1.15

                onContentHeightChanged: {
                    oledWidgetWindow.updateTitleExpansion()
                }
            }

            QQC2.ToolTip.visible: windowTitleMouse.containsMouse && windowTitleBar.titleText.length > 0
            QQC2.ToolTip.delay: 400
            QQC2.ToolTip.text: (oledWidgetWindow.lastExternalWindowClass.length > 0
                ? "[" + oledWidgetWindow.lastExternalWindowClass + "] "
                : "") + windowTitleBar.titleText
        }

        // Bottom Rotary Knob Summary (with Rotary Knob Icons 1 & 2), Bottom Profile Buttons & Corner Resize Grip
        Rectangle {
            id: bottomFooterBar
            anchors.bottom: oledWidgetWindow.showWindowTitle ? windowTitleBar.top : parent.bottom
            anchors.bottomMargin: oledWidgetWindow.showWindowTitle ? oledWidgetWindow.sectionGap : oledWidgetWindow.outerPad
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.leftMargin: oledWidgetWindow.outerPad
            anchors.rightMargin: oledWidgetWindow.outerPad
            height: oledWidgetWindow.footerBarHeight
            radius: 6
            color: "#111827"
            border.color: oledWidgetWindow.autoSwitchPaused ? "#f59e0b" : "#1e293b"
            border.width: 1

            readonly property int knobDiameter: Math.max(15, Math.round(oledWidgetWindow.oledSide * 0.082))
            readonly property int encFontSize: Math.max(10, Math.round(oledWidgetWindow.oledSide * 0.056))
            readonly property int profileBtnHeight: Math.max(20, Math.round(21 * oledWidgetWindow.uiScale))

            // Scroll anywhere on the footer to change transparency (declared first so buttons stay on top)
            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.NoButton
                onWheel: (wheel) => oledWidgetWindow.handleTransparencyWheel(wheel)
            }

            ColumnLayout {
                anchors.fill: parent
                anchors.leftMargin: Math.max(6, Math.round(8 * oledWidgetWindow.uiScale))
                anchors.rightMargin: 4
                anchors.topMargin: 3
                anchors.bottomMargin: 3
                spacing: Math.max(2, Math.round(3 * oledWidgetWindow.uiScale))

                // Rotary Knob 1 Row
                RowLayout {
                    visible: !oledWidgetWindow.isDuckyPad2020
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
                    visible: !oledWidgetWindow.isDuckyPad2020
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

                // Bottom Profile Switcher Row (Previous Profile / Current Profile / Next Profile + Corner Resize Grip)
                RowLayout {
                    Layout.fillWidth: true
                    spacing: Math.max(4, Math.round(5 * oledWidgetWindow.uiScale))

                    // Previous Profile Button (− Profile)
                    QQC2.ToolButton {
                        id: prevProfileBtn
                        Layout.fillWidth: true
                        Layout.preferredHeight: bottomFooterBar.profileBtnHeight
                        implicitHeight: Layout.preferredHeight
                        padding: 0
                        onClicked: {
                            if (oledWidgetWindow.controller) {
                                const prev = Math.max(1, oledWidgetWindow.activeProfile - 1)
                                if (oledWidgetWindow.targetDeviceId.length > 0) {
                                    oledWidgetWindow.controller.switchProfileForDevice(oledWidgetWindow.targetDeviceId, prev)
                                } else {
                                    oledWidgetWindow.controller.switchProfile(prev)
                                }
                            }
                        }
                        contentItem: Text {
                            text: qsTr("− Profile")
                            color: "#f8fafc"
                            font.bold: true
                            font.pixelSize: Math.max(10, Math.round(bottomFooterBar.profileBtnHeight * 0.52))
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                        background: Rectangle {
                            radius: 4
                            color: prevProfileBtn.hovered ? "#25334a" : "#1a2333"
                            border.width: 1
                            border.color: prevProfileBtn.hovered ? "#38bdf8" : "#344663"
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Previous Profile")
                    }

                    // Next Profile Button (+ Profile)
                    QQC2.ToolButton {
                        id: nextProfileBtn
                        Layout.fillWidth: true
                        Layout.preferredHeight: bottomFooterBar.profileBtnHeight
                        implicitHeight: Layout.preferredHeight
                        padding: 0
                        onClicked: {
                            if (oledWidgetWindow.controller) {
                                const next = Math.min(64, oledWidgetWindow.activeProfile + 1)
                                if (oledWidgetWindow.targetDeviceId.length > 0) {
                                    oledWidgetWindow.controller.switchProfileForDevice(oledWidgetWindow.targetDeviceId, next)
                                } else {
                                    oledWidgetWindow.controller.switchProfile(next)
                                }
                            }
                        }
                        contentItem: Text {
                            text: qsTr("+ Profile")
                            color: "#f8fafc"
                            font.bold: true
                            font.pixelSize: Math.max(10, Math.round(bottomFooterBar.profileBtnHeight * 0.52))
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                        background: Rectangle {
                            radius: 4
                            color: nextProfileBtn.hovered ? "#25334a" : "#1a2333"
                            border.width: 1
                            border.color: nextProfileBtn.hovered ? "#38bdf8" : "#344663"
                        }
                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: hovered
                        QQC2.ToolTip.text: qsTr("Next Profile")
                    }

                    // Bottom-Right Resize Grip
                    Rectangle {
                        width: Math.max(18, Math.round(20 * oledWidgetWindow.uiScale))
                        height: bottomFooterBar.profileBtnHeight
                        radius: 4
                        color: resizeMouse.containsMouse ? "#1e293b" : "transparent"

                        Text {
                            anchors.centerIn: parent
                            text: "◢"
                            font.pixelSize: Math.max(10, Math.round(bottomFooterBar.profileBtnHeight * 0.52))
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

                        QQC2.ToolTip.delay: 500
                        QQC2.ToolTip.visible: resizeMouse.containsMouse
                        QQC2.ToolTip.text: qsTr("Drag to resize OLED Desktop Widget")
                    }
                }
            }
        }

        // Transient transparency percentage HUD (shown while scrolling)
        Rectangle {
            id: transparencyHud
            objectName: "oledWidgetTransparencyHud"
            z: 30
            anchors.centerIn: parent
            width: hudLabel.implicitWidth + 24
            height: hudLabel.implicitHeight + 12
            radius: 8
            color: "#e6162033"
            border.color: "#38bdf8"
            border.width: 1
            opacity: 0
            visible: opacity > 0

            function flash() {
                hudFadeOut.stop()
                opacity = 1
                hudHideTimer.restart()
            }

            Text {
                id: hudLabel
                anchors.centerIn: parent
                text: qsTr("Transparency %1%").arg(oledWidgetWindow.transparencyPercent)
                color: "#38bdf8"
                font.family: "monospace"
                font.bold: true
                font.pixelSize: Math.max(11, Math.round(12 * oledWidgetWindow.uiScale))
            }

            Timer {
                id: hudHideTimer
                interval: 700
                onTriggered: hudFadeOut.start()
            }

            NumberAnimation {
                id: hudFadeOut
                target: transparencyHud
                property: "opacity"
                to: 0
                duration: 250
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
                border.color: oledWidgetWindow.hasAutoSwitchError
                    ? "#ef4444"
                    : (oledWidgetWindow.autoSwitchPaused ? "#f59e0b" : "#38bdf8")
                border.width: 2
                clip: true

                MouseArea {
                    anchors.fill: parent
                    acceptedButtons: Qt.NoButton
                    onWheel: (wheel) => oledWidgetWindow.handleTransparencyWheel(wheel)
                }

                Item {
                    id: oledScreenContent
                    anchors.fill: parent
                    anchors.margins: 4

                    readonly property bool isLandscapeRot:
                        oledWidgetWindow.profileOledRotation === 90 || oledWidgetWindow.profileOledRotation === 270
                    readonly property int gridCols: isLandscapeRot ? 5 : oledWidgetWindow.basePortraitCols
                    readonly property int gridRows: isLandscapeRot ? oledWidgetWindow.basePortraitCols : 5
                    readonly property int bannerHeight: Math.max(14, Math.round(height * 0.105))
                    readonly property int errorStripHeight: oledWidgetWindow.hasAutoSwitchError
                        ? Math.max(14, Math.round(height * 0.095))
                        : 0
                    readonly property int gridAreaHeight: Math.max(60, height - bannerHeight - errorStripHeight - 2)
                    readonly property real cellWidth: width / gridCols
                    readonly property real cellHeight: gridAreaHeight / gridRows

                    // Top Profile Banner
                    Rectangle {
                        id: oledBanner
                        anchors.top: parent.top
                        anchors.left: parent.left
                        anchors.right: parent.right
                        height: oledScreenContent.bannerHeight
                        color: oledWidgetWindow.hasAutoSwitchError ? "#ef4444" : "#38bdf8"

                        Text {
                            anchors.fill: parent
                            anchors.leftMargin: 4
                            anchors.rightMargin: 4
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            text: "P" + oledWidgetWindow.activeProfile + ": " + oledWidgetWindow.activeProfileName
                                + (oledWidgetWindow.profileOledRotation !== 0 ? " [" + oledWidgetWindow.profileOledRotation + "°]" : "")
                            color: oledWidgetWindow.hasAutoSwitchError ? "#ffffff" : "#020617"
                            font.family: "sans-serif"
                            font.bold: true
                            font.pixelSize: Math.max(10, Math.round(parent.height * 0.62))
                            elide: Text.ElideRight
                        }
                    }

                    // Auto-Switch Error Banner Strip (visible directly inside OLED screen even if bottom title bar is hidden)
                    Rectangle {
                        id: oledErrorStrip
                        visible: oledWidgetWindow.hasAutoSwitchError
                        anchors.top: oledBanner.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right
                        height: oledScreenContent.errorStripHeight
                        color: "#7f1d1d"

                        Text {
                            anchors.fill: parent
                            anchors.leftMargin: 4
                            anchors.rightMargin: 4
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            text: oledWidgetWindow.autoSwitchError
                            color: "#fecaca"
                            font.family: "sans-serif"
                            font.bold: true
                            font.pixelSize: Math.max(8, Math.round(parent.height * 0.60))
                            elide: Text.ElideRight
                        }
                    }

                    // Key Grid with Centered Upright Button Text (supports 0°/90°/180°/270° and longer Desktop Widget labels wl1/wl2)
                    Item {
                        anchors.top: oledWidgetWindow.hasAutoSwitchError ? oledErrorStrip.bottom : oledBanner.bottom
                        anchors.topMargin: 2
                        anchors.bottom: parent.bottom
                        anchors.left: parent.left
                        anchors.right: parent.right

                        Repeater {
                            id: keyCellRepeater
                            model: oledWidgetWindow.onboardKeyCount

                            delegate: Rectangle {
                                id: keyCellRect
                                objectName: "oledWidgetCell_" + index
                                required property int index
                                readonly property int currentRot: oledWidgetWindow.profileOledRotation
                                readonly property bool isLandscapeCell: currentRot === 90 || currentRot === 270
                                readonly property int pCols: oledWidgetWindow.basePortraitCols
                                readonly property int cols: isLandscapeCell ? 5 : pCols
                                readonly property int colIdx: index % cols
                                readonly property int rowIdx: Math.floor(index / cols)
                                readonly property int slotNum: {
                                    const rot = currentRot
                                    const c = colIdx
                                    const r = rowIdx
                                    if (rot === 90) {
                                        return c * pCols + ((pCols - 1) - r) + 1
                                    } else if (rot === 180) {
                                        return (4 - r) * pCols + ((pCols - 1) - c) + 1
                                    } else if (rot === 270) {
                                        return (4 - c) * pCols + r + 1
                                    }
                                    return r * pCols + c + 1
                                }
                                readonly property var slotData: {
                                    const _rev = oledWidgetWindow.dataRevision
                                    const _items = oledWidgetWindow.slotItems
                                    const _rot = currentRot
                                    return oledWidgetWindow.slotDataFor(slotNum)
                                }
                                readonly property string l1: slotData.l1 || ""
                                readonly property string l2: slotData.l2 || ""
                                readonly property string wl1: (slotData.wl1 || "").trim()
                                readonly property string wl2: (slotData.wl2 || "").trim()
                                readonly property int slotR: slotData.r !== undefined ? slotData.r : 0
                                readonly property int slotG: slotData.g !== undefined ? slotData.g : 170
                                readonly property int slotB: slotData.b !== undefined ? slotData.b : 255
                                readonly property bool slotHasScript: !!slotData.hasScript
                                readonly property string disp1: wl1.length > 0
                                    ? wl1
                                    : oledWidgetWindow.sliceOledLabel(l1, isLandscapeCell ? 4 : 5)
                                readonly property string disp2: wl2.length > 0
                                    ? wl2
                                    : oledWidgetWindow.sliceOledLabel(l2, isLandscapeCell ? 4 : 5)
                                readonly property bool isSel: oledWidgetWindow.selectedSlot === slotNum

                                readonly property real labelBoxWidth: Math.max(16, (isLandscapeCell ? height : width) - 4)
                                readonly property real labelBoxHeight: Math.max(16, isLandscapeCell ? width : height)
                                readonly property alias labelRotation: cellLabelColumn.rotation

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
                                        && (parent.disp1.length > 0 || parent.disp2.length > 0 || parent.slotHasScript)
                                    anchors.top: parent.top
                                    anchors.right: parent.right
                                    anchors.margins: Math.max(2, Math.round(Math.min(parent.width, parent.height) * 0.08))
                                    width: Math.max(3, Math.round(Math.min(parent.width, parent.height) * 0.14))
                                    height: width
                                    radius: width / 2
                                    color: Qt.rgba(
                                        parent.slotR / 255.0,
                                        parent.slotG / 255.0,
                                        parent.slotB / 255.0,
                                        1.0
                                    )
                                }

                                // Centered Key Button Labels (rotates with key orientation)
                                Column {
                                    id: cellLabelColumn
                                    objectName: "oledCellLabelColumn"
                                    anchors.centerIn: parent
                                    width: keyCellRect.labelBoxWidth
                                    rotation: keyCellRect.currentRot
                                    transformOrigin: Item.Center
                                    spacing: 0

                                    Text {
                                        objectName: "oledCellText1"
                                        readonly property bool isIcon1: oledWidgetWindow.isSingleIconLabel(keyCellRect.disp1)
                                        readonly property bool isLong1: keyCellRect.wl1.length > 5
                                        visible: keyCellRect.disp1.length > 0
                                        width: parent.width
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        text: keyCellRect.disp1
                                        color: keyCellRect.isSel ? "#020617" : "#f8fafc"
                                        font.family: (isIcon1 || isLong1) ? "sans-serif" : "monospace"
                                        font.bold: true
                                        font.pixelSize: isIcon1
                                            ? Math.max(10, Math.round(keyCellRect.labelBoxHeight * (keyCellRect.disp2.length > 0 ? 0.42 : 0.54)))
                                            : Math.max(9, Math.round(keyCellRect.labelBoxHeight * (keyCellRect.disp2.length > 0 ? 0.34 : 0.42)))
                                        fontSizeMode: Text.HorizontalFit
                                        minimumPixelSize: 7
                                        elide: Text.ElideRight
                                    }

                                    Text {
                                        objectName: "oledCellText2"
                                        readonly property bool isIcon2: oledWidgetWindow.isSingleIconLabel(keyCellRect.disp2)
                                        readonly property bool isLong2: keyCellRect.wl2.length > 5
                                        visible: keyCellRect.disp2.length > 0
                                        width: parent.width
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                        text: keyCellRect.disp2
                                        color: keyCellRect.isSel ? "#0f172a" : "#cbd5e1"
                                        font.family: (isIcon2 || isLong2) ? "sans-serif" : "monospace"
                                        font.bold: true
                                        font.pixelSize: isIcon2
                                            ? Math.max(10, Math.round(keyCellRect.labelBoxHeight * (keyCellRect.disp1.length > 0 ? 0.40 : 0.52)))
                                            : Math.max(9, Math.round(keyCellRect.labelBoxHeight * (keyCellRect.disp1.length > 0 ? 0.32 : 0.40)))
                                        fontSizeMode: Text.HorizontalFit
                                        minimumPixelSize: 7
                                        elide: Text.ElideRight
                                    }
                                }

                                MouseArea {
                                    id: cellMouse
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    cursorShape: Qt.PointingHandCursor
                                    onClicked: {
                                        oledWidgetWindow.selectedSlot = (oledWidgetWindow.selectedSlot === parent.slotNum) ? 0 : parent.slotNum
                                    }
                                    onWheel: (wheel) => oledWidgetWindow.handleTransparencyWheel(wheel)
                                }

                                QQC2.ToolTip.visible: cellMouse.containsMouse && (disp1.length > 0 || disp2.length > 0)
                                QQC2.ToolTip.delay: 400
                                QQC2.ToolTip.text: "Key " + slotNum + ": " + (disp1 + (disp2.length > 0 ? " " + disp2 : "")).trim()
                            }
                        }
                    }
                }
            }
        }
    }
}
