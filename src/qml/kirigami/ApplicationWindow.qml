import QtQuick
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

QQC2.ApplicationWindow {
    id: winRoot

    readonly property Kirigami.PageStack pageStack: Kirigami.PageStack {}

    Component.onCompleted: {
        if (pageStack.initialPage && winRoot.contentItem) {
            pageStack.initialPage.parent = winRoot.contentItem
            pageStack.initialPage.width = Qt.binding(function() { return winRoot.contentItem.width })
            pageStack.initialPage.height = Qt.binding(function() { return winRoot.contentItem.height })
        }
    }
}
