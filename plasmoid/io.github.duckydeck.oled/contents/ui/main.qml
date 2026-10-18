import QtQuick
import QtQuick.Layouts
import org.kde.plasma.plasmoid
import org.kde.plasma.core as PlasmaCore
import org.kde.plasma.components as PlasmaComponents3
import org.kde.plasma.plasma5support as Plasma5Support
import org.kde.kirigami as Kirigami

PlasmoidItem {
    id: root

    Plasmoid.icon: "io.github.duckydeck.Studio"
    Plasmoid.backgroundHints: PlasmaCore.Types.DefaultBackground | PlasmaCore.Types.ConfigurableBackground
    preferredRepresentation: fullRepresentation
    switchWidth: Kirigami.Units.gridUnit * 4
    switchHeight: Kirigami.Units.gridUnit * 4

    width: 284
    height: 356
    Layout.minimumWidth: 160
    Layout.minimumHeight: 190
    Layout.preferredWidth: 284
    Layout.preferredHeight: 356

    property bool connected: false
    property string modelName: "duckyPad Pro"
    property int activeProfile: 1
    property string activeProfileName: "Default"
    property string activeProfileBgColor: "#00aaff"
    property int profileOledRotation: 0
    property int dataRevision: 0
    property int totalProfiles: 1
    property string sdSpaceSummary: ""
    property var slotsList: []
    property int selectedSlot: 0
    property string enc1Summary: "CCW · CW · [Push]"
    property string enc2Summary: "CCW · CW · [Push]"

    function formatEnc(item, fallback) {
        if (!item) return fallback
        const wl1 = (item.wl1 || "").trim()
        const wl2 = (item.wl2 || "").trim()
        const l1 = wl1.length > 0 ? wl1 : (item.l1 || "").trim()
        const l2 = wl2.length > 0 ? wl2 : (item.l2 || "").trim()
        const c = (l1 + (l2.length > 0 ? " " + l2 : "")).trim()
        return c.length > 0 ? c : fallback
    }

    function slotDataFor(slotNum) {
        if (root.slotsList && slotNum >= 1 && slotNum <= root.slotsList.length) {
            const direct = root.slotsList[slotNum - 1]
            if (direct && direct.slot === slotNum) {
                return direct
            }
            for (let i = 0; i < root.slotsList.length; ++i) {
                if (root.slotsList[i].slot === slotNum) {
                    return root.slotsList[i]
                }
            }
        }
        return { slot: slotNum, l1: "", l2: "", wl1: "", wl2: "", r: 0, g: 170, b: 255, hasScript: false }
    }

    function updateEncoderSummaries(arr) {
        let e1ccw = "CCW", e1cw = "CW", e1push = "Push"
        let e2ccw = "CCW", e2cw = "CW", e2push = "Push"
        for (let i = 0; i < arr.length; ++i) {
            const it = arr[i]
            if (it.slot === 21) {
                e1ccw = root.formatEnc(it, "CCW")
            } else if (it.slot === 22) {
                e1cw = root.formatEnc(it, "CW")
            } else if (it.slot === 23) {
                e1push = root.formatEnc(it, "Push")
            } else if (it.slot === 24) {
                e2ccw = root.formatEnc(it, "CCW")
            } else if (it.slot === 25) {
                e2cw = root.formatEnc(it, "CW")
            } else if (it.slot === 26) {
                e2push = root.formatEnc(it, "Push")
            }
        }
        root.enc1Summary = e1ccw + " · " + e1cw + " · [" + e1push + "]"
        root.enc2Summary = e2ccw + " · " + e2cw + " · [" + e2push + "]"
    }

    function runCommand(cmd) {
        cmdRunner.connectSource(cmd)
    }

    Plasma5Support.DataSource {
        id: stateReader
        engine: "executable"
        connectedSources: []
        onNewData: (sourceName, data) => {
            const stdout = (data["stdout"] || "").trim()
            disconnectSource(sourceName)
            if (stdout.length > 0) {
                try {
                    const parsed = JSON.parse(stdout)
                    root.connected = !!parsed.connected
                    root.modelName = parsed.modelName || "duckyPad Pro"
                    root.activeProfile = parsed.activeProfile || 1
                    root.activeProfileName = parsed.activeProfileName || "Default"
                    root.activeProfileBgColor = parsed.activeProfileBgColor || "#00aaff"
                    root.profileOledRotation = parsed.effectiveWidgetRotation !== undefined
                        ? Number(parsed.effectiveWidgetRotation)
                        : (parsed.oledRotation !== undefined ? Number(parsed.oledRotation) : 0)
                    root.totalProfiles = parsed.totalProfiles || 1
                    root.sdSpaceSummary = parsed.sdSpaceSummary || ""
                    const rawSlots = Array.isArray(parsed.slots) ? parsed.slots : []
                    root.slotsList = rawSlots
                    root.updateEncoderSummaries(rawSlots)
                    root.dataRevision += 1
                } catch (e) {
                }
            }
        }
    }

    Plasma5Support.DataSource {
        id: cmdRunner
        engine: "executable"
        connectedSources: []
        onNewData: (sourceName, data) => {
            disconnectSource(sourceName)
            pollTimer.restart()
        }
    }

    Timer {
        id: pollTimer
        interval: 800
        running: true
        repeat: true
        triggeredOnStart: true
        onTriggered: {
            stateReader.connectSource("cat \"$HOME/.local/share/duckydeck-studio/oled_state.json\" 2>/dev/null")
        }
    }

    fullRepresentation: Item {
        id: fullRep
        implicitWidth: 284
        implicitHeight: 356
        Layout.minimumWidth: 160
        Layout.minimumHeight: 190
        Layout.preferredWidth: 284
        Layout.preferredHeight: 356

        // Pure top-down geometry calculations (zero binding loops)
        readonly property real effW: width > 0 ? width : 284
        readonly property real effH: height > 0 ? height : 356
        readonly property real sf: Math.max(0.75, Math.min(effW / 260.0, effH / 330.0))
        readonly property int outerPad: Math.max(6, Math.round(8 * sf))
        readonly property int sectionGap: Math.max(4, Math.round(6 * sf))
        readonly property int headerBarHeight: Math.max(24, Math.round(28 * sf))
        readonly property int footerBarHeight: Math.max(36, Math.round(44 * sf))
        readonly property int availOledWidth: Math.max(100, Math.round(effW) - outerPad * 2)
        readonly property int availOledHeight: Math.max(100, Math.round(effH) - outerPad * 2 - headerBarHeight - footerBarHeight - sectionGap * 2)
        readonly property int oledSide: Math.max(100, Math.min(availOledWidth, availOledHeight))

        Rectangle {
            anchors.fill: parent
            radius: 10
            color: "#0b0f19"
            border.color: root.connected ? "#38bdf8" : "#334155"
            border.width: 1

            // Top Header Bar
            Rectangle {
                id: topHeaderBar
                anchors.top: parent.top
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.margins: fullRep.outerPad
                height: fullRep.headerBarHeight
                radius: 6
                color: "#111827"
                border.color: "#1e293b"
                border.width: 1

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 6
                    anchors.rightMargin: 4
                    spacing: 4

                    Rectangle {
                        width: Math.max(7, Math.round(8 * fullRep.sf))
                        height: width
                        radius: Math.max(3, Math.round(width / 2))
                        color: root.connected ? "#22c55e" : "#38bdf8"
                    }

                    Text {
                        text: "P" + root.activeProfile + ": " + root.activeProfileName
                        color: "#f8fafc"
                        font.bold: true
                        font.pixelSize: Math.max(10, Math.round(12 * fullRep.sf))
                        elide: Text.ElideRight
                        Layout.fillWidth: true
                    }

                    // Previous Profile Button
                    Rectangle {
                        width: Math.max(20, Math.round(22 * fullRep.sf))
                        height: Math.max(20, Math.round(20 * fullRep.sf))
                        radius: 4
                        color: prevMouse.containsMouse ? "#25334a" : "#1a2333"
                        border.color: "#344663"
                        border.width: 1

                        Text {
                            anchors.centerIn: parent
                            text: "−"
                            color: "#e6edf3"
                            font.bold: true
                            font.pixelSize: Math.max(10, Math.round(12 * fullRep.sf))
                        }

                        MouseArea {
                            id: prevMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                const target = Math.max(1, root.activeProfile - 1)
                                root.runCommand("(\"$HOME/.local/bin/duckydeck-studio\" --switch-profile " + target + " || duckydeck-studio --switch-profile " + target + ") >/dev/null 2>&1 &")
                            }
                        }
                    }

                    // Next Profile Button
                    Rectangle {
                        width: Math.max(20, Math.round(22 * fullRep.sf))
                        height: Math.max(20, Math.round(20 * fullRep.sf))
                        radius: 4
                        color: nextMouse.containsMouse ? "#25334a" : "#1a2333"
                        border.color: "#344663"
                        border.width: 1

                        Text {
                            anchors.centerIn: parent
                            text: "+"
                            color: "#e6edf3"
                            font.bold: true
                            font.pixelSize: Math.max(10, Math.round(12 * fullRep.sf))
                        }

                        MouseArea {
                            id: nextMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                const target = Math.min(64, root.activeProfile + 1)
                                root.runCommand("(\"$HOME/.local/bin/duckydeck-studio\" --switch-profile " + target + " || duckydeck-studio --switch-profile " + target + ") >/dev/null 2>&1 &")
                            }
                        }
                    }

                    // Always-on-Top Floating Window Pin Button
                    Rectangle {
                        width: Math.max(22, Math.round(24 * fullRep.sf))
                        height: Math.max(20, Math.round(20 * fullRep.sf))
                        radius: 4
                        color: pinMouse.containsMouse ? "#1b4b66" : "#1a2333"
                        border.color: "#38bdf8"
                        border.width: 1

                        Text {
                            anchors.centerIn: parent
                            text: "📌"
                            font.pixelSize: Math.max(10, Math.round(11 * fullRep.sf))
                        }

                        MouseArea {
                            id: pinMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                root.runCommand("(\"$HOME/.local/bin/duckydeck-studio\" --oled-widget || duckydeck-studio --oled-widget) >/dev/null 2>&1 &")
                            }
                        }

                        PlasmaComponents3.ToolTip {
                            visible: pinMouse.containsMouse
                            text: "Toggle Always-on-Top Resizable OLED Window"
                        }
                    }

                    // Open DuckyDeck Studio Main Window
                    Rectangle {
                        width: Math.max(22, Math.round(24 * fullRep.sf))
                        height: Math.max(20, Math.round(20 * fullRep.sf))
                        radius: 4
                        color: studioMouse.containsMouse ? "#25334a" : "#1a2333"
                        border.color: "#344663"
                        border.width: 1

                        Text {
                            anchors.centerIn: parent
                            text: "⚙"
                            color: "#e6edf3"
                            font.pixelSize: Math.max(10, Math.round(11 * fullRep.sf))
                        }

                        MouseArea {
                            id: studioMouse
                            anchors.fill: parent
                            hoverEnabled: true
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                root.runCommand("(\"$HOME/.local/bin/duckydeck-studio\" --show || duckydeck-studio --show) >/dev/null 2>&1 &")
                            }
                        }

                        PlasmaComponents3.ToolTip {
                            visible: studioMouse.containsMouse
                            text: "Open DuckyDeck Studio"
                        }
                    }
                }
            }

            // Bottom Rotary Knob Summary Strip
            Rectangle {
                id: bottomFooterBar
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.margins: fullRep.outerPad
                height: fullRep.footerBarHeight
                radius: 6
                color: "#111827"
                border.color: "#1e293b"
                border.width: 1

                readonly property int knobIconSize: Math.max(15, Math.round(fullRep.oledSide * 0.072))

                ColumnLayout {
                    anchors.fill: parent
                    anchors.leftMargin: Math.max(6, Math.round(8 * fullRep.sf))
                    anchors.rightMargin: Math.max(6, Math.round(8 * fullRep.sf))
                    anchors.topMargin: 2
                    anchors.bottomMargin: 2
                    spacing: 2

                    // Rotary Knob 1 Row
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Math.max(5, Math.round(6 * fullRep.sf))

                        // Knob 1 Icon
                        Rectangle {
                            Layout.preferredWidth: bottomFooterBar.knobIconSize
                            Layout.preferredHeight: bottomFooterBar.knobIconSize
                            radius: Math.round(bottomFooterBar.knobIconSize / 2)
                            color: "#0f172a"
                            border.color: "#38bdf8"
                            border.width: Math.max(1, Math.round(1.5 * fullRep.sf))

                            // Top indicator notch on the knob dial
                            Rectangle {
                                width: Math.max(2, Math.round(parent.width * 0.14))
                                height: Math.max(3, Math.round(parent.height * 0.24))
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
                            text: root.enc1Summary
                            font.family: "monospace"
                            font.bold: true
                            font.pixelSize: Math.max(11, Math.round(fullRep.oledSide * 0.056))
                            color: "#7dd3fc"
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }
                    }

                    // Rotary Knob 2 Row
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Math.max(5, Math.round(6 * fullRep.sf))

                        // Knob 2 Icon
                        Rectangle {
                            Layout.preferredWidth: bottomFooterBar.knobIconSize
                            Layout.preferredHeight: bottomFooterBar.knobIconSize
                            radius: Math.round(bottomFooterBar.knobIconSize / 2)
                            color: "#0f172a"
                            border.color: "#38bdf8"
                            border.width: Math.max(1, Math.round(1.5 * fullRep.sf))

                            // Top indicator notch on the knob dial
                            Rectangle {
                                width: Math.max(2, Math.round(parent.width * 0.14))
                                height: Math.max(3, Math.round(parent.height * 0.24))
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
                            text: root.enc2Summary
                            font.family: "monospace"
                            font.bold: true
                            font.pixelSize: Math.max(11, Math.round(fullRep.oledSide * 0.056))
                            color: "#7dd3fc"
                            elide: Text.ElideRight
                            Layout.fillWidth: true
                        }
                    }
                }
            }

            // Center 1:1 Aspect-Preserving OLED Bezel
            Item {
                id: oledViewportContainer
                anchors.top: topHeaderBar.bottom
                anchors.bottom: bottomFooterBar.top
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.topMargin: fullRep.sectionGap
                anchors.bottomMargin: fullRep.sectionGap
                anchors.leftMargin: fullRep.outerPad
                anchors.rightMargin: fullRep.outerPad

                Rectangle {
                    id: oledBezel
                    width: fullRep.oledSide
                    height: fullRep.oledSide
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
                            if (wheel.angleDelta.y > 0) {
                                const prev = Math.max(1, root.activeProfile - 1)
                                root.runCommand("(\"$HOME/.local/bin/duckydeck-studio\" --switch-profile " + prev + " || duckydeck-studio --switch-profile " + prev + ") >/dev/null 2>&1 &")
                            } else if (wheel.angleDelta.y < 0) {
                                const next = Math.min(64, root.activeProfile + 1)
                                root.runCommand("(\"$HOME/.local/bin/duckydeck-studio\" --switch-profile " + next + " || duckydeck-studio --switch-profile " + next + ") >/dev/null 2>&1 &")
                            }
                        }
                    }

                    Item {
                        id: oledScreenContent
                        anchors.fill: parent
                        anchors.margins: 4

                        readonly property bool isLandscapeRot:
                            root.profileOledRotation === 90 || root.profileOledRotation === 270
                        readonly property int gridCols: isLandscapeRot ? 5 : 4
                        readonly property int gridRows: isLandscapeRot ? 4 : 5
                        readonly property int bannerHeight: Math.max(14, Math.round(height * 0.105))
                        readonly property int gridAreaHeight: Math.max(80, height - bannerHeight - 2)
                        readonly property real cellWidth: width / gridCols
                        readonly property real cellHeight: gridAreaHeight / gridRows

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
                                text: "P" + root.activeProfile + ": " + root.activeProfileName
                                    + (root.profileOledRotation !== 0 ? " [" + root.profileOledRotation + "°]" : "")
                                color: "#020617"
                                font.family: "sans-serif"
                                font.bold: true
                                font.pixelSize: Math.max(9, Math.round(parent.height * 0.68))
                                elide: Text.ElideRight
                            }
                        }

                        // Key Grid with Centered Button Text (supports 0°/90°/180°/270° and wl1/wl2)
                        Item {
                            anchors.top: oledBanner.bottom
                            anchors.topMargin: 2
                            anchors.bottom: parent.bottom
                            anchors.left: parent.left
                            anchors.right: parent.right

                            Repeater {
                                model: 20

                                delegate: Rectangle {
                                    id: keyCellRect
                                    required property int index
                                    readonly property int currentRot: root.profileOledRotation
                                    readonly property bool isLandscapeCell: currentRot === 90 || currentRot === 270
                                    readonly property int cols: isLandscapeCell ? 5 : 4
                                    readonly property int colIdx: index % cols
                                    readonly property int rowIdx: Math.floor(index / cols)
                                    readonly property int slotNum: {
                                        const rot = currentRot
                                        const c = colIdx
                                        const r = rowIdx
                                        if (rot === 90) {
                                            return c * 4 + (3 - r) + 1
                                        } else if (rot === 180) {
                                            return (4 - r) * 4 + (3 - c) + 1
                                        } else if (rot === 270) {
                                            return (4 - c) * 4 + r + 1
                                        }
                                        return r * 4 + c + 1
                                    }
                                    readonly property string l1: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        return root.slotDataFor(slotNum).l1 || ""
                                    }
                                    readonly property string l2: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        return root.slotDataFor(slotNum).l2 || ""
                                    }
                                    readonly property string wl1: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        return (root.slotDataFor(slotNum).wl1 || "").trim()
                                    }
                                    readonly property string wl2: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        return (root.slotDataFor(slotNum).wl2 || "").trim()
                                    }
                                    readonly property int slotR: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        const d = root.slotDataFor(slotNum)
                                        return d.r !== undefined ? d.r : 0
                                    }
                                    readonly property int slotG: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        const d = root.slotDataFor(slotNum)
                                        return d.g !== undefined ? d.g : 170
                                    }
                                    readonly property int slotB: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        const d = root.slotDataFor(slotNum)
                                        return d.b !== undefined ? d.b : 255
                                    }
                                    readonly property bool slotHasScript: {
                                        const _rev = root.dataRevision
                                        const _items = root.slotsList
                                        const _rot = currentRot
                                        return !!root.slotDataFor(slotNum).hasScript
                                    }
                                    readonly property string disp1: wl1.length > 0
                                        ? wl1
                                        : l1.slice(0, isLandscapeCell ? 4 : 6)
                                    readonly property string disp2: wl2.length > 0
                                        ? wl2
                                        : l2.slice(0, isLandscapeCell ? 4 : 6)
                                    readonly property bool isSel: root.selectedSlot === slotNum
                                    readonly property real labelBoxWidth: Math.max(16, (isLandscapeCell ? height : width) - 4)
                                    readonly property real labelBoxHeight: Math.max(16, isLandscapeCell ? width : height)

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
                                        visible: !parent.isSel
                                            && (parent.disp1.length > 0 || parent.disp2.length > 0 || parent.slotHasScript)
                                        anchors.top: parent.top
                                        anchors.right: parent.right
                                        anchors.margins: Math.max(2, Math.round(Math.min(parent.width, parent.height) * 0.08))
                                        width: Math.max(3, Math.round(Math.min(parent.width, parent.height) * 0.14))
                                        height: width
                                        radius: Math.max(1, Math.round(width / 2))
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
                                        anchors.centerIn: parent
                                        width: keyCellRect.labelBoxWidth
                                        rotation: keyCellRect.currentRot
                                        transformOrigin: Item.Center
                                        spacing: 0

                                        Text {
                                            visible: keyCellRect.disp1.length > 0
                                            width: parent.width
                                            horizontalAlignment: Text.AlignHCenter
                                            verticalAlignment: Text.AlignVCenter
                                            text: keyCellRect.disp1
                                            color: keyCellRect.isSel ? "#020617" : "#f8fafc"
                                            font.family: keyCellRect.wl1.length > 5 ? "sans-serif" : "monospace"
                                            font.bold: true
                                            font.pixelSize: Math.max(8, Math.round(keyCellRect.labelBoxHeight * 0.34))
                                            fontSizeMode: Text.HorizontalFit
                                            minimumPixelSize: 6
                                            elide: Text.ElideRight
                                        }

                                        Text {
                                            visible: keyCellRect.disp2.length > 0
                                            width: parent.width
                                            horizontalAlignment: Text.AlignHCenter
                                            verticalAlignment: Text.AlignVCenter
                                            text: keyCellRect.disp2
                                            color: keyCellRect.isSel ? "#0f172a" : "#cbd5e1"
                                            font.family: keyCellRect.wl2.length > 5 ? "sans-serif" : "monospace"
                                            font.bold: true
                                            font.pixelSize: Math.max(7, Math.round(keyCellRect.labelBoxHeight * 0.30))
                                            fontSizeMode: Text.HorizontalFit
                                            minimumPixelSize: 6
                                            elide: Text.ElideRight
                                        }
                                    }

                                    MouseArea {
                                        anchors.fill: parent
                                        cursorShape: Qt.PointingHandCursor
                                        onClicked: {
                                            root.selectedSlot = (root.selectedSlot === parent.slotNum) ? 0 : parent.slotNum
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
}
