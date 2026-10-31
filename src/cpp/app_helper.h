#pragma once

#if !defined(_WIN32)
#include <KColorScheme>
#include <KColorSchemeManager>
#include <KIconLoader>
#include <KSharedConfig>
#include <csignal>
#include <unistd.h>
#else
#include "kirigami_compat.h"
#endif
#include <QAbstractItemModel>
#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QCoreApplication>
#include <QCursor>
#include <QDateTime>
#include <QDir>
#include <QEvent>
#include <QFile>
#include <QFileInfo>
#include <QFont>
#include <QGuiApplication>
#include <QHoverEvent>
#include <QIcon>
#include <QImage>
#include <QJsonDocument>
#include <QJsonObject>
#include <QKeyEvent>
#include <QLoggingCategory>
#include <QMenu>
#include <QMessageBox>
#include <QMetaObject>
#include <QModelIndex>
#include <QMouseEvent>
#include <QPainter>
#include <QPainterPath>
#include <QPixmap>
#include <QPointer>
#include <QProcess>
#include <QPushButton>
#include <QQmlApplicationEngine>
#include <QQuickItem>
#include <QQuickWindow>
#include <QScreen>
#include <QStandardPaths>
#include <QString>
#include <QStringList>
#include <QSvgRenderer>
#include <QSystemTrayIcon>
#include <QThread>
#include <QTimer>
#include <QUrl>
#include <QVariant>
#include <QWheelEvent>
#include <QWindow>
#include <algorithm>
#include <cstdint>
#include <cstdlib>

namespace duckydeck {

inline QPointer<QWindow>& main_window_ref() {
    static QPointer<QWindow> win;
    return win;
}

inline QWindow* find_main_window();
inline void ensure_global_key_filter();

inline QIcon build_multisize_svg_icon(const QString& qrcPath) {
    QSvgRenderer renderer(qrcPath);
    QIcon icon;
    const int sizes[] = {16, 22, 24, 32, 48, 64, 128};
    for (int sz : sizes) {
        QPixmap pm(sz, sz);
        pm.fill(Qt::transparent);
        if (renderer.isValid()) {
            QPainter painter(&pm);
            painter.setRenderHint(QPainter::Antialiasing, true);
            painter.setRenderHint(QPainter::SmoothPixmapTransform, true);
            renderer.render(&painter, QRectF(0, 0, sz, sz));
        }
        icon.addPixmap(pm);
    }
    return icon;
}

inline QIcon oled_widget_icon() {
    static QIcon s_icon;
    if (s_icon.isNull()) {
        s_icon = build_multisize_svg_icon(
            QStringLiteral(":/qt/qml/io/github/duckydeck/src/qml/icons/oled-widget-icon.svg")
        );
    }
    return s_icon;
}

inline void update_tray_window_actions();

struct OledWidgetGeometryState {
    bool visible = false;
    bool hasPosition = false;
    int x = 0;
    int y = 0;
    int width = 284;
    int height = 356;
    int transparencyPercent = 0;
    bool alwaysOnTop = true;
    bool onAllDesktops = true;
    bool framelessMode = true;
    bool showRgbDots = true;
    QString screenName;
};

inline qint64& oled_last_shown_ms_ref() {
    static qint64 ts = 0;
    return ts;
}

inline QRect& oled_expected_restore_rect_ref() {
    static QRect r;
    return r;
}

inline bool& oled_restore_settled_ref() {
    static bool settled = false;
    return settled;
}

inline QString oled_widget_geometry_file_path() {
    QString base = QStandardPaths::writableLocation(QStandardPaths::GenericConfigLocation).trimmed();
    if (base.isEmpty()) {
        base = QDir::homePath() + QStringLiteral("/.config");
    }
    return base + QStringLiteral("/duckydeck-studio/oled_widget_geometry.json");
}

inline OledWidgetGeometryState load_oled_widget_geometry() {
    OledWidgetGeometryState st;
    QFile f(oled_widget_geometry_file_path());
    if (!f.exists() || !f.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return st;
    }
    const QJsonDocument doc = QJsonDocument::fromJson(f.readAll());
    f.close();
    if (!doc.isObject()) {
        return st;
    }
    const QJsonObject obj = doc.object();
    st.visible = obj.value(QStringLiteral("visible")).toBool(false);
    st.hasPosition = obj.value(QStringLiteral("has_position")).toBool(false);
    st.x = obj.value(QStringLiteral("x")).toInt(0);
    st.y = obj.value(QStringLiteral("y")).toInt(0);
    st.width = std::clamp(obj.value(QStringLiteral("width")).toInt(284), 184, 1600);
    st.height = std::clamp(obj.value(QStringLiteral("height")).toInt(356), 240, 1600);
    st.transparencyPercent =
        std::clamp(obj.value(QStringLiteral("transparency_percent")).toInt(0), 0, 75);
    st.alwaysOnTop = obj.value(QStringLiteral("always_on_top")).toBool(true);
    st.onAllDesktops = obj.value(QStringLiteral("on_all_desktops")).toBool(true);
    st.framelessMode = obj.value(QStringLiteral("frameless_mode")).toBool(true);
    st.showRgbDots = obj.value(QStringLiteral("show_rgb_dots")).toBool(true);
    st.screenName = obj.value(QStringLiteral("screen_name")).toString().trimmed();
    return st;
}

inline void save_oled_widget_geometry(const OledWidgetGeometryState& st) {
    if (qEnvironmentVariable("DUCKYDECK_DEMO_SHOWCASE") == QStringLiteral("1")) {
        return;
    }
    const QString path = oled_widget_geometry_file_path();
    QFileInfo fi(path);
    if (!fi.dir().exists()) {
        QDir().mkpath(fi.dir().absolutePath());
    }
    QJsonObject obj;
    obj.insert(QStringLiteral("visible"), st.visible);
    obj.insert(QStringLiteral("has_position"), st.hasPosition);
    obj.insert(QStringLiteral("x"), st.x);
    obj.insert(QStringLiteral("y"), st.y);
    obj.insert(QStringLiteral("width"), std::clamp(st.width, 184, 1600));
    obj.insert(QStringLiteral("height"), std::clamp(st.height, 240, 1600));
    obj.insert(
        QStringLiteral("transparency_percent"),
        std::clamp(st.transparencyPercent, 0, 75)
    );
    obj.insert(QStringLiteral("always_on_top"), st.alwaysOnTop);
    obj.insert(QStringLiteral("on_all_desktops"), st.onAllDesktops);
    obj.insert(QStringLiteral("frameless_mode"), st.framelessMode);
    obj.insert(QStringLiteral("show_rgb_dots"), st.showRgbDots);
    obj.insert(QStringLiteral("screen_name"), st.screenName);
    QFile f(path);
    if (f.open(QIODevice::WriteOnly | QIODevice::Truncate | QIODevice::Text)) {
        f.write(QJsonDocument(obj).toJson(QJsonDocument::Indented));
        f.close();
    }
}

inline void sync_oled_widget_props_from_window(QWindow* win, OledWidgetGeometryState& st) {
    if (!win || !win->property("_duckydeck_props_restored").toBool()) {
        return;
    }
    const QVariant tp = win->property("transparencyPercent");
    if (tp.isValid()) {
        st.transparencyPercent = std::clamp(tp.toInt(), 0, 75);
    }
    const QVariant aot = win->property("alwaysOnTop");
    if (aot.isValid()) {
        st.alwaysOnTop = aot.toBool();
    }
    const QVariant oad = win->property("onAllDesktops");
    if (oad.isValid()) {
        st.onAllDesktops = oad.toBool();
    }
    const QVariant fm = win->property("framelessMode");
    if (fm.isValid()) {
        st.framelessMode = fm.toBool();
    }
    const QVariant dots = win->property("showRgbDots");
    if (dots.isValid()) {
        st.showRgbDots = dots.toBool();
    }
}

inline QScreen* find_screen_for_rect(int x, int y, int w, int h, const QString& preferredName) {
    const QPoint centerPt(x + w / 2, y + h / 2);
    const auto screens = QGuiApplication::screens();
    for (QScreen* scr : screens) {
        if (scr && scr->geometry().contains(centerPt)) {
            return scr;
        }
    }
    const QRect targetRect(x, y, std::max(1, w), std::max(1, h));
    for (QScreen* scr : screens) {
        if (scr && scr->geometry().intersects(targetRect)) {
            return scr;
        }
    }
    if (!preferredName.isEmpty()) {
        for (QScreen* scr : screens) {
            if (scr && scr->name() == preferredName) {
                return scr;
            }
        }
    }
    return nullptr;
}

inline QRect resolve_oled_widget_target_rect(
    const OledWidgetGeometryState& st,
    QScreen** outScreen = nullptr
) {
    QScreen* fallbackScreen = nullptr;
    if (QWindow* mainWin = main_window_ref().data()) {
        fallbackScreen = mainWin->screen();
    }
    if (!fallbackScreen) {
        fallbackScreen = QGuiApplication::primaryScreen();
    }
    const QRect fallbackAvail =
        fallbackScreen ? fallbackScreen->availableGeometry() : QRect(0, 0, 1920, 1080);

    const int w = std::clamp(st.width, 184, 1600);
    const int h = std::clamp(st.height, 240, 1600);

    QScreen* matchedScreen = nullptr;
    if (st.hasPosition || !st.screenName.isEmpty()) {
        matchedScreen = find_screen_for_rect(st.x, st.y, w, h, st.screenName);
    }

    int targetX = st.x;
    int targetY = st.y;
    if (matchedScreen) {
        const QRect g = matchedScreen->geometry();
        const int maxX = std::max(g.left(), g.right() + 1 - w);
        const int maxY = std::max(g.top(), g.bottom() + 1 - h);
        if (st.hasPosition && g.intersects(QRect(st.x, st.y, w, h))) {
            targetX = std::clamp(st.x, g.left(), maxX);
            targetY = std::clamp(st.y, g.top(), maxY);
        } else if (st.hasPosition) {
            targetX = std::clamp(st.x, g.left(), maxX);
            targetY = std::clamp(st.y, g.top(), maxY);
        } else {
            targetX = g.x() + (g.width() - w) / 2;
            targetY = g.y() + (g.height() - h) / 2;
        }
        if (outScreen) {
            *outScreen = matchedScreen;
        }
    } else if (st.hasPosition) {
        targetX = st.x;
        targetY = st.y;
        if (outScreen) {
            *outScreen = fallbackScreen;
        }
    } else {
        targetX = fallbackAvail.x() + (fallbackAvail.width() - w) / 2;
        targetY = fallbackAvail.y() + (fallbackAvail.height() - h) / 2;
        if (outScreen) {
            *outScreen = fallbackScreen;
        }
    }
    return QRect(targetX, targetY, w, h);
}

inline void ensure_oled_window_hooks(QWindow* win) {
    if (!win) {
        return;
    }
    if (win->transientParent() != nullptr && !win->isVisible()) {
        win->setTransientParent(nullptr);
    }
    if (win->property("_duckydeck_oled_hooked").toBool()) {
        return;
    }
    win->setProperty("_duckydeck_oled_hooked", true);
    win->setIcon(oled_widget_icon());

    const OledWidgetGeometryState initialLoaded = load_oled_widget_geometry();
    win->setProperty("transparencyPercent", std::clamp(initialLoaded.transparencyPercent, 0, 75));
    win->setProperty("alwaysOnTop", initialLoaded.alwaysOnTop);
    win->setProperty("onAllDesktops", initialLoaded.onAllDesktops);
    win->setProperty("framelessMode", initialLoaded.framelessMode);
    win->setProperty("showRgbDots", initialLoaded.showRgbDots);
    win->setProperty("_duckydeck_props_restored", true);

    QScreen* initialScreen = nullptr;
    const QRect initialTarget = resolve_oled_widget_target_rect(initialLoaded, &initialScreen);
    if (initialScreen && !win->isVisible() && win->handle() == nullptr && win->screen() != initialScreen) {
        win->setScreen(initialScreen);
    }
    win->resize(initialTarget.width(), initialTarget.height());
    if (QGuiApplication::platformName() != QStringLiteral("wayland") || !win->isVisible()) {
        win->setPosition(initialTarget.x(), initialTarget.y());
    }

    QObject::connect(win, &QWindow::windowStateChanged, win, [win](Qt::WindowState state) {
        if ((state & Qt::WindowMinimized) && win->property("alwaysOnTop").toBool()) {
            QTimer::singleShot(0, win, [win]() {
                if (win && win->property("alwaysOnTop").toBool()) {
                    win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
                    win->showNormal();
                    win->raise();
                }
            });
        }
    });

    QObject::connect(win, &QWindow::visibilityChanged, win, [win](QWindow::Visibility visibility) {
        if (visibility == QWindow::Minimized && win->property("alwaysOnTop").toBool()) {
            QTimer::singleShot(0, win, [win]() {
                if (win && win->property("alwaysOnTop").toBool()) {
                    win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
                    win->showNormal();
                    win->raise();
                }
            });
        }
    });

    auto saveSizeIfReady = [win]() {
        if (!win || !win->isVisible()) {
            return;
        }
        if (!oled_restore_settled_ref()
            && (QDateTime::currentMSecsSinceEpoch() - oled_last_shown_ms_ref() < 2500)) {
            return;
        }
        const int w = win->width();
        const int h = win->height();
        if (w < 180 || h < 220) {
            return;
        }
        OledWidgetGeometryState st = load_oled_widget_geometry();
        st.visible = true;
        st.width = std::clamp(w, 184, 1600);
        st.height = std::clamp(h, 240, 1600);
        sync_oled_widget_props_from_window(win, st);
        if (QGuiApplication::platformName() != QStringLiteral("wayland")) {
            st.x = win->x();
            st.y = win->y();
            st.hasPosition = true;
            if (QScreen* scr = find_screen_for_rect(st.x, st.y, st.width, st.height, st.screenName)) {
                st.screenName = scr->name();
            }
        }
        save_oled_widget_geometry(st);
    };

    QObject::connect(win, &QWindow::widthChanged, win, [saveSizeIfReady](int) {
        saveSizeIfReady();
    });
    QObject::connect(win, &QWindow::heightChanged, win, [saveSizeIfReady](int) {
        saveSizeIfReady();
    });
    QObject::connect(win, &QWindow::xChanged, win, [saveSizeIfReady](int) {
        if (QGuiApplication::platformName() != QStringLiteral("wayland")) {
            saveSizeIfReady();
        }
    });
    QObject::connect(win, &QWindow::yChanged, win, [saveSizeIfReady](int) {
        if (QGuiApplication::platformName() != QStringLiteral("wayland")) {
            saveSizeIfReady();
        }
    });
}

inline QWindow* find_oled_widget_window() {
    const auto windows = QGuiApplication::topLevelWindows();
    for (QWindow* w : windows) {
        if (w && w->objectName() == QStringLiteral("duckydeckOledWidgetWindow")) {
            ensure_oled_window_hooks(w);
            return w;
        }
    }
    return nullptr;
}

inline void save_oled_widget_appearance_now() {
    if (QWindow* win = find_oled_widget_window()) {
        OledWidgetGeometryState st = load_oled_widget_geometry();
        sync_oled_widget_props_from_window(win, st);
        save_oled_widget_geometry(st);
    }
}

inline QRect restore_oled_widget_geometry_now() {
    oled_last_shown_ms_ref() = QDateTime::currentMSecsSinceEpoch();
    const OledWidgetGeometryState loaded = load_oled_widget_geometry();
    QScreen* targetScreen = nullptr;
    const QRect target = resolve_oled_widget_target_rect(loaded, &targetScreen);
    oled_expected_restore_rect_ref() = target;
    oled_restore_settled_ref() = false;

    OledWidgetGeometryState resolved = loaded;
    resolved.visible = true;
    if (!loaded.hasPosition) {
        resolved.hasPosition = true;
        resolved.x = target.x();
        resolved.y = target.y();
        if (targetScreen) {
            resolved.screenName = targetScreen->name();
        }
    }
    resolved.width = target.width();
    resolved.height = target.height();
    if (QWindow* win = find_oled_widget_window()) {
        sync_oled_widget_props_from_window(win, resolved);
        if (win->transientParent() != nullptr && !win->isVisible()) {
            win->setTransientParent(nullptr);
        }
        if (targetScreen && !win->isVisible() && win->handle() == nullptr && win->screen() != targetScreen) {
            win->setScreen(targetScreen);
        }
        if (win->width() != target.width() || win->height() != target.height()) {
            win->resize(target.width(), target.height());
        }
        if (QGuiApplication::platformName() != QStringLiteral("wayland") || !win->isVisible()) {
            win->setPosition(target.x(), target.y());
        }
    }
    save_oled_widget_geometry(resolved);
    return target;
}

inline void record_oled_widget_geometry_from_kwin(int x, int y, int width, int height) {
    QWindow* win = find_oled_widget_window();
    if (!win || !win->isVisible() || (win->windowStates() & Qt::WindowMinimized)) {
        return;
    }
    if (width < 180 || height < 220) {
        return;
    }
    const qint64 elapsed = QDateTime::currentMSecsSinceEpoch() - oled_last_shown_ms_ref();
    if (!oled_restore_settled_ref()) {
        const QRect expected = oled_expected_restore_rect_ref();
        if (expected.isValid()
            && std::abs(x - expected.x()) <= 16
            && std::abs(y - expected.y()) <= 16) {
            oled_restore_settled_ref() = true;
        } else if (elapsed < 2500) {
            return;
        } else {
            oled_restore_settled_ref() = true;
        }
    }
    OledWidgetGeometryState st = load_oled_widget_geometry();
    st.visible = true;
    st.hasPosition = true;
    st.x = x;
    st.y = y;
    st.width = std::clamp(width, 184, 1600);
    st.height = std::clamp(height, 240, 1600);
    if (QScreen* scr = find_screen_for_rect(x, y, st.width, st.height, st.screenName)) {
        st.screenName = scr->name();
    }
    sync_oled_widget_props_from_window(win, st);
    save_oled_widget_geometry(st);
}

inline void ensure_window_hooks(QWindow* win) {
    ensure_global_key_filter();
    if (!win || win->property("_duckydeck_minimize_hooked").toBool()) {
        return;
    }
    win->setProperty("_duckydeck_minimize_hooked", true);

    QObject::connect(win, &QWindow::windowStateChanged, win, [win](Qt::WindowState state) {
        if (state & Qt::WindowMinimized) {
            QTimer::singleShot(0, win, [win]() {
                if (win) {
                    QWindow* oledWin = find_oled_widget_window();
                    const bool oledPinnedVisible =
                        oledWin && oledWin->isVisible() && oledWin->property("alwaysOnTop").toBool();
                    win->hide();
                    win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
                    if (oledPinnedVisible && oledWin) {
                        oledWin->setTransientParent(nullptr);
                        oledWin->setWindowStates(oledWin->windowStates() & ~Qt::WindowMinimized);
                        oledWin->showNormal();
                        oledWin->show();
                        oledWin->raise();
                    }
                    update_tray_window_actions();
                }
            });
        } else {
            update_tray_window_actions();
        }
    });

    QObject::connect(win, &QWindow::visibilityChanged, win, [win](QWindow::Visibility visibility) {
        if (visibility == QWindow::Minimized) {
            QTimer::singleShot(0, win, [win]() {
                if (win) {
                    QWindow* oledWin = find_oled_widget_window();
                    const bool oledPinnedVisible =
                        oledWin && oledWin->isVisible() && oledWin->property("alwaysOnTop").toBool();
                    win->hide();
                    win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
                    if (oledPinnedVisible && oledWin) {
                        oledWin->setTransientParent(nullptr);
                        oledWin->setWindowStates(oledWin->windowStates() & ~Qt::WindowMinimized);
                        oledWin->showNormal();
                        oledWin->show();
                        oledWin->raise();
                    }
                    update_tray_window_actions();
                }
            });
        } else {
            update_tray_window_actions();
        }
    });

    QObject::connect(win, &QWindow::visibleChanged, win, [](bool) {
        update_tray_window_actions();
    });
}

inline bool is_candidate_main_window(QWindow* w) {
    if (!w) {
        return false;
    }
    if (w->objectName() == QStringLiteral("duckydeckMainWindow")) {
        return true;
    }
    const QMetaObject* mo = w->metaObject();
    return mo && mo->indexOfMethod("closeTopmostPopupWindow()") != -1;
}

inline QWindow* find_main_window() {
    QPointer<QWindow>& win = main_window_ref();
    if (win && is_candidate_main_window(win.data())) {
        ensure_window_hooks(win.data());
        return win.data();
    }
    win = nullptr;
    const auto windows = QGuiApplication::topLevelWindows();
    for (QWindow* w : windows) {
        if (is_candidate_main_window(w)) {
            win = w;
            ensure_window_hooks(w);
            return w;
        }
    }
    return nullptr;
}

inline bool is_main_window_shown() {
    QWindow* win = find_main_window();
    if (!win) {
        return false;
    }
    return win->isVisible()
        && !(win->windowStates() & Qt::WindowMinimized)
        && win->visibility() != QWindow::Minimized
        && win->visibility() != QWindow::Hidden;
}

inline void show_oled_widget_window() {
    if (QWindow* win = find_oled_widget_window()) {
        if (!win->isVisible() && win->transientParent() != nullptr) {
            win->setTransientParent(nullptr);
        }
        win->setIcon(oled_widget_icon());
        const QRect target = restore_oled_widget_geometry_now();
        win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
        if (win->width() != target.width() || win->height() != target.height()) {
            win->resize(target.width(), target.height());
        }
        if (QGuiApplication::platformName() != QStringLiteral("wayland")) {
            win->setPosition(target.x(), target.y());
        }
        win->showNormal();
        win->show();
        win->raise();
        win->requestActivate();
        win->requestUpdate();
    }
}

inline void hide_oled_widget_window() {
    OledWidgetGeometryState st = load_oled_widget_geometry();
    st.visible = false;
    if (QWindow* win = find_oled_widget_window()) {
        sync_oled_widget_props_from_window(win, st);
        save_oled_widget_geometry(st);
        win->hide();
    } else {
        save_oled_widget_geometry(st);
    }
}

inline void toggle_oled_widget_window() {
    if (QWindow* win = find_oled_widget_window()) {
        if (win->isVisible()) {
            hide_oled_widget_window();
        } else {
            show_oled_widget_window();
        }
    }
}

inline void show_main_window() {
    if (QWindow* win = find_main_window()) {
        if (win->windowStates() & Qt::WindowMinimized) {
            win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
        }
        win->showNormal();
        win->show();
        win->raise();
        win->requestActivate();
    }
    update_tray_window_actions();
}

inline void hide_main_window_to_tray() {
    QWindow* oledWin = find_oled_widget_window();
    const bool oledWasVisible = oledWin && oledWin->isVisible();
    const bool oledPinned = oledWin && oledWin->property("alwaysOnTop").toBool();
    if (oledWin) {
        oledWin->setTransientParent(nullptr);
    }
    if (QWindow* win = find_main_window()) {
        win->hide();
        if (win->windowStates() & Qt::WindowMinimized) {
            win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
        }
    }
    if (oledWasVisible && oledWin) {
        oledWin->setTransientParent(nullptr);
        if (oledPinned) {
            oledWin->setWindowStates(oledWin->windowStates() & ~Qt::WindowMinimized);
            oledWin->showNormal();
            oledWin->show();
            oledWin->raise();
        }
    }
    update_tray_window_actions();
}

inline void show_about_window() {
    if (QWindow* mainWin = find_main_window()) {
        QMetaObject::invokeMethod(mainWin, "openAboutWindow", Qt::QueuedConnection);
        return;
    }
    const auto windows = QGuiApplication::allWindows();
    for (QWindow* w : windows) {
        if (w && w->objectName() == QStringLiteral("duckydeckAboutDialog")) {
            w->showNormal();
            w->show();
            w->raise();
            w->requestActivate();
            return;
        }
    }
}

inline void quit_duckydeck_application() {
    save_oled_widget_appearance_now();
    QCoreApplication::quit();
}

inline void restart_duckydeck_application() {
    save_oled_widget_appearance_now();
    QWindow* win = find_main_window();
    const bool wasHiddenToTray =
        !win || !win->isVisible() || (win->windowStates() & Qt::WindowMinimized);

    QStringList args;
    args << QStringLiteral("--replace");
    if (wasHiddenToTray) {
        args << QStringLiteral("--minimize") << QStringLiteral("--foreground");
    }

    const QString appPath = QCoreApplication::applicationFilePath();
    if (!appPath.isEmpty()) {
        QProcess::startDetached(appPath, args);
    }
    QCoreApplication::quit();
}

class TrayManager {
public:
    static TrayManager& instance() {
        static TrayManager inst;
        return inst;
    }

    void updateWindowActionsVisibility() {
        const bool shown = is_main_window_shown();
        if (m_showAction) {
            m_showAction->setVisible(!shown);
        }
        if (m_hideAction) {
            m_hideAction->setVisible(shown);
        }
    }

    void showNotification(const QString& title, const QString& message) {
        ensureInitialized();
        if (m_tray && m_tray->isVisible()) {
            m_tray->showMessage(title, message, m_tray->icon(), 8000);
        }
    }

    void sync(
        QObject* controller,
        bool useLightOnDarkIcon,
        const QString& iconMode,
        const QString& statusHeader,
        const QString& profileHeader,
        const QString& sdSpaceHeader,
        const QString& tooltipText,
        bool autoSwitchEnabled,
        bool autostartEnabled
    ) {
        m_controller = controller;
        ensureInitialized();
        if (!m_tray) {
            return;
        }

        const QString qrcPath = useLightOnDarkIcon
            ? QStringLiteral(":/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg")
            : QStringLiteral(":/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-light.svg");

        if (m_lastIconPath != qrcPath) {
            m_lastIconPath = qrcPath;
            QIcon icon = build_multisize_svg_icon(qrcPath);
            m_tray->setIcon(icon);
            QGuiApplication::setWindowIcon(icon);
            if (QWindow* win = find_main_window()) {
                win->setIcon(icon);
            }
            if (QWindow* oledWin = find_oled_widget_window()) {
                oledWin->setIcon(oled_widget_icon());
            }
        } else {
            (void)find_main_window();
            (void)find_oled_widget_window();
        }

        m_tray->setToolTip(tooltipText);

        if (m_statusAction) {
            m_statusAction->setText(statusHeader);
        }
        if (m_profileAction) {
            m_profileAction->setText(profileHeader);
        }
        if (m_sdSpaceAction) {
            m_sdSpaceAction->setText(sdSpaceHeader);
            m_sdSpaceAction->setVisible(!sdSpaceHeader.isEmpty());
        }
        if (m_autoSwitchAction) {
            const bool oldBlock = m_autoSwitchAction->blockSignals(true);
            m_autoSwitchAction->setChecked(autoSwitchEnabled);
            m_autoSwitchAction->setText(
                autoSwitchEnabled
                    ? QStringLiteral("Auto-Switcher: Enabled")
                    : QStringLiteral("Auto-Switcher: Disabled")
            );
            m_autoSwitchAction->blockSignals(oldBlock);
        }
        if (m_autostartAction) {
            const bool oldBlock = m_autostartAction->blockSignals(true);
            m_autostartAction->setChecked(autostartEnabled);
            m_autostartAction->blockSignals(oldBlock);
        }
        if (m_iconAutoAction && m_iconDarkAction && m_iconLightAction) {
            m_iconAutoAction->blockSignals(true);
            m_iconDarkAction->blockSignals(true);
            m_iconLightAction->blockSignals(true);
            m_iconAutoAction->setChecked(iconMode == QStringLiteral("auto"));
            m_iconDarkAction->setChecked(iconMode == QStringLiteral("dark"));
            m_iconLightAction->setChecked(iconMode == QStringLiteral("light"));
            m_iconAutoAction->blockSignals(false);
            m_iconDarkAction->blockSignals(false);
            m_iconLightAction->blockSignals(false);
        }

        updateWindowActionsVisibility();

        if (!m_tray->isVisible()) {
            m_tray->show();
        }
    }

    void dispatchIpcCommand(const QString& rawCmd) {
        const QString cmd = rawCmd.trimmed();
        if (cmd.isEmpty()) {
            return;
        }
        if (cmd == QStringLiteral("show")) {
            show_main_window();
            return;
        }
        if (cmd == QStringLiteral("hide")) {
            hide_main_window_to_tray();
            return;
        }
        if (cmd == QStringLiteral("toggle-window")) {
            if (is_main_window_shown()) {
                hide_main_window_to_tray();
            } else {
                show_main_window();
            }
            return;
        }
        if (cmd == QStringLiteral("show-oled-widget")) {
            show_oled_widget_window();
            return;
        }
        if (cmd == QStringLiteral("hide-oled-widget")) {
            hide_oled_widget_window();
            return;
        }
        if (cmd == QStringLiteral("toggle-oled-widget")) {
            toggle_oled_widget_window();
            return;
        }
        if (cmd == QStringLiteral("show-about")) {
            show_about_window();
            return;
        }
        if (cmd == QStringLiteral("restart")) {
            restart_duckydeck_application();
            return;
        }
        if (m_controller) {
            QMetaObject::invokeMethod(
                m_controller,
                "applyIpcCommand",
                Qt::QueuedConnection,
                Q_ARG(QString, cmd)
            );
        }
    }

private:
    TrayManager() = default;

    void ensureInitialized() {
        if (m_initialized || !QCoreApplication::instance()) {
            return;
        }
        m_initialized = true;

        QGuiApplication::setQuitOnLastWindowClosed(false);

        m_tray = new QSystemTrayIcon(QCoreApplication::instance());
        m_menu = new QMenu();

        QObject::connect(m_menu, &QMenu::aboutToShow, [this]() {
            updateWindowActionsVisibility();
        });

        m_statusAction = m_menu->addAction(QStringLiteral("DuckyDeck Studio"));
        QObject::connect(m_statusAction, &QAction::triggered, []() {
            show_main_window();
        });

        m_profileAction = m_menu->addAction(QStringLiteral("Active Profile: #1"));
        QObject::connect(m_profileAction, &QAction::triggered, []() {
            show_main_window();
        });

        m_sdSpaceAction = m_menu->addAction(QStringLiteral("💾 SD Card: -"));
        QObject::connect(m_sdSpaceAction, &QAction::triggered, []() {
            show_main_window();
        });

        m_menu->addSeparator();

        m_autoSwitchAction = m_menu->addAction(QStringLiteral("Auto-Switcher: Enabled"));
        m_autoSwitchAction->setCheckable(true);
        QObject::connect(m_autoSwitchAction, &QAction::triggered, [this](bool) {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "toggleAutoSwitchFromTray",
                    Qt::QueuedConnection
                );
            }
        });

#if defined(_WIN32)
        m_autostartAction = m_menu->addAction(QStringLiteral("Start on Windows Login (Autostart)"));
#else
        m_autostartAction = m_menu->addAction(QStringLiteral("Start on KDE Login (Autostart)"));
#endif
        m_autostartAction->setCheckable(true);
        QObject::connect(m_autostartAction, &QAction::triggered, [this](bool) {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "toggleAutostartFromTray",
                    Qt::QueuedConnection
                );
            }
        });

#if defined(_WIN32)
        m_menu->addSeparator();
        QActionGroup* group = new QActionGroup(m_menu);
        group->setExclusive(true);

        m_iconAutoAction = m_menu->addAction(QStringLiteral("Tray Icon: Auto Theme"));
        m_iconAutoAction->setCheckable(true);
        group->addAction(m_iconAutoAction);
        QObject::connect(m_iconAutoAction, &QAction::triggered, [this]() {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "applyTrayIconMode",
                    Qt::QueuedConnection,
                    Q_ARG(QString, QStringLiteral("auto"))
                );
            }
        });

        m_iconDarkAction = m_menu->addAction(QStringLiteral("Tray Icon: Dark Mode (Light-on-Dark)"));
        m_iconDarkAction->setCheckable(true);
        group->addAction(m_iconDarkAction);
        QObject::connect(m_iconDarkAction, &QAction::triggered, [this]() {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "applyTrayIconMode",
                    Qt::QueuedConnection,
                    Q_ARG(QString, QStringLiteral("dark"))
                );
            }
        });

        m_iconLightAction = m_menu->addAction(QStringLiteral("Tray Icon: Light Mode (Dark-on-Light)"));
        m_iconLightAction->setCheckable(true);
        group->addAction(m_iconLightAction);
        QObject::connect(m_iconLightAction, &QAction::triggered, [this]() {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "applyTrayIconMode",
                    Qt::QueuedConnection,
                    Q_ARG(QString, QStringLiteral("light"))
                );
            }
        });
#else
        QMenu* iconMenu = m_menu->addMenu(QStringLiteral("Taskbar Icon Theme"));
        QActionGroup* group = new QActionGroup(iconMenu);
        group->setExclusive(true);

        m_iconAutoAction = iconMenu->addAction(QStringLiteral("Auto (Match KDE Theme)"));
        m_iconAutoAction->setCheckable(true);
        group->addAction(m_iconAutoAction);
        QObject::connect(m_iconAutoAction, &QAction::triggered, [this]() {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "applyTrayIconMode",
                    Qt::QueuedConnection,
                    Q_ARG(QString, QStringLiteral("auto"))
                );
            }
        });

        m_iconDarkAction = iconMenu->addAction(QStringLiteral("Dark Mode Icon (Light-on-Dark)"));
        m_iconDarkAction->setCheckable(true);
        group->addAction(m_iconDarkAction);
        QObject::connect(m_iconDarkAction, &QAction::triggered, [this]() {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "applyTrayIconMode",
                    Qt::QueuedConnection,
                    Q_ARG(QString, QStringLiteral("dark"))
                );
            }
        });

        m_iconLightAction = iconMenu->addAction(QStringLiteral("Light Mode Icon (Dark-on-Light)"));
        m_iconLightAction->setCheckable(true);
        group->addAction(m_iconLightAction);
        QObject::connect(m_iconLightAction, &QAction::triggered, [this]() {
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "applyTrayIconMode",
                    Qt::QueuedConnection,
                    Q_ARG(QString, QStringLiteral("light"))
                );
            }
        });
#endif

        m_menu->addSeparator();

        QAction* oledWidgetAction =
            m_menu->addAction(oled_widget_icon(), QStringLiteral("Toggle Desktop OLED Widget"));
        QObject::connect(oledWidgetAction, &QAction::triggered, []() {
            toggle_oled_widget_window();
        });

        m_showAction = m_menu->addAction(QStringLiteral("Show DuckyDeck Studio"));
        QObject::connect(m_showAction, &QAction::triggered, []() {
            show_main_window();
        });

        m_hideAction = m_menu->addAction(QStringLiteral("Minimize to Tray"));
        QObject::connect(m_hideAction, &QAction::triggered, []() {
            hide_main_window_to_tray();
        });

        QAction* checkUpdateAction = m_menu->addAction(
            QIcon::fromTheme(QStringLiteral("system-software-update")),
            QStringLiteral("Check for Updates...")
        );
        QObject::connect(checkUpdateAction, &QAction::triggered, [this]() {
            show_main_window();
            if (m_controller) {
                QMetaObject::invokeMethod(
                    m_controller,
                    "checkAppUpdates",
                    Qt::QueuedConnection,
                    Q_ARG(bool, true)
                );
            }
        });

        QAction* aboutAction =
            m_menu->addAction(QIcon::fromTheme(QStringLiteral("help-about")), QStringLiteral("About DuckyDeck Studio"));
        QObject::connect(aboutAction, &QAction::triggered, []() {
            show_about_window();
        });

        m_menu->addSeparator();

        QAction* restartAction = m_menu->addAction(QStringLiteral("Restart DuckyDeck Studio"));
        QObject::connect(restartAction, &QAction::triggered, [this]() {
            if (m_tray) {
                m_tray->hide();
            }
            restart_duckydeck_application();
        });

        QAction* quitAction = m_menu->addAction(QStringLiteral("Quit DuckyDeck Studio"));
        QObject::connect(quitAction, &QAction::triggered, []() {
            quit_duckydeck_application();
        });

        updateWindowActionsVisibility();
        m_tray->setContextMenu(m_menu);

        QObject::connect(
            m_tray,
            &QSystemTrayIcon::activated,
            [](QSystemTrayIcon::ActivationReason reason) {
                if (reason == QSystemTrayIcon::Trigger || reason == QSystemTrayIcon::DoubleClick) {
                    if (is_main_window_shown()) {
                        hide_main_window_to_tray();
                    } else {
                        show_main_window();
                    }
                }
            }
        );
    }

    bool m_initialized = false;
    QPointer<QObject> m_controller;
    QPointer<QSystemTrayIcon> m_tray;
    QMenu* m_menu = nullptr;
    QAction* m_statusAction = nullptr;
    QAction* m_profileAction = nullptr;
    QAction* m_sdSpaceAction = nullptr;
    QAction* m_autoSwitchAction = nullptr;
    QAction* m_autostartAction = nullptr;
    QAction* m_iconAutoAction = nullptr;
    QAction* m_iconDarkAction = nullptr;
    QAction* m_iconLightAction = nullptr;
    QAction* m_showAction = nullptr;
    QAction* m_hideAction = nullptr;
    QString m_lastIconPath;
};

inline void update_tray_window_actions() {
    TrayManager::instance().updateWindowActionsVisibility();
}

inline QString load_saved_app_theme_mode() {
    QString base = QStandardPaths::writableLocation(QStandardPaths::GenericConfigLocation).trimmed();
    if (base.isEmpty()) {
        base = QDir::homePath() + QStringLiteral("/.config");
    }
    QFile f(base + QStringLiteral("/duckydeck-studio/kwin_rules.json"));
    if (!f.exists() || !f.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return QStringLiteral("system");
    }
    const QJsonDocument doc = QJsonDocument::fromJson(f.readAll());
    f.close();
    if (!doc.isObject()) {
        return QStringLiteral("system");
    }
    const QString mode = doc.object().value(QStringLiteral("app_theme_mode")).toString().trimmed().toLower();
    if (mode == QStringLiteral("dark") || mode == QStringLiteral("light") || mode == QStringLiteral("system")) {
        return mode;
    }
    return QStringLiteral("system");
}

inline int load_saved_ui_text_scale() {
    QString path;
    if (qEnvironmentVariable("DUCKYDECK_UI_TEST_SUITE") == QStringLiteral("1")) {
        path = QDir::tempPath()
            + QStringLiteral("/duckydeck-ui-test-suite-%1/kwin_rules.json")
                  .arg(static_cast<qlonglong>(QCoreApplication::applicationPid()));
    } else {
        QString base = QStandardPaths::writableLocation(QStandardPaths::GenericConfigLocation).trimmed();
        if (base.isEmpty()) {
            base = QDir::homePath() + QStringLiteral("/.config");
        }
        path = base + QStringLiteral("/duckydeck-studio/kwin_rules.json");
    }
    QFile f(path);
    if (!f.exists() || !f.open(QIODevice::ReadOnly | QIODevice::Text)) {
        return 100;
    }
    const QJsonDocument doc = QJsonDocument::fromJson(f.readAll());
    f.close();
    if (!doc.isObject()) {
        return 100;
    }
    const int scale = doc.object().value(QStringLiteral("ui_text_scale")).toInt(100);
    return std::clamp(scale, 75, 160);
}

inline bool invoke_qml_method(QObject* obj, const char* methodName) {
    if (!obj) {
        return false;
    }
    const QMetaObject* mo = obj->metaObject();
    if (!mo) {
        return false;
    }
    const QByteArray sig = QByteArray(methodName) + "()";
    const int idx = mo->indexOfMethod(sig.constData());
    if (idx < 0) {
        return false;
    }
    const QMetaMethod mm = mo->method(idx);
    if (mm.returnMetaType().id() == QMetaType::QVariant) {
        QVariant ret;
        return mm.invoke(obj, Qt::DirectConnection, Q_RETURN_ARG(QVariant, ret));
    }
    return mm.invoke(obj, Qt::DirectConnection);
}

class DuckyDeckGlobalKeyFilter : public QObject {
public:
    explicit DuckyDeckGlobalKeyFilter(QObject* parent = nullptr) : QObject(parent) {
        m_scrollHoverResumeTimer.setSingleShot(true);
        QObject::connect(&m_scrollHoverResumeTimer, &QTimer::timeout, this, [this]() {
            m_suppressingScrollHover = false;
            if (m_lastScrolledWindow && m_lastScrolledWindow->isVisible()) {
                QHoverEvent hoverEv(
                    QEvent::HoverMove,
                    m_lastWheelLocalPos,
                    m_lastWheelGlobalPos,
                    m_lastWheelLocalPos,
                    Qt::NoModifier
                );
                QCoreApplication::sendEvent(m_lastScrolledWindow.data(), &hoverEv);
            }
        });
    }

private:
    bool m_suppressingScrollHover = false;
    QTimer m_scrollHoverResumeTimer;
    QPointer<QQuickWindow> m_lastScrolledWindow;
    QPointer<QQuickItem> m_lockedFlickable;
    qint64 m_lastWheelEventMs = 0;
    qint64 m_lastTimerRestartMs = 0;
    QPointF m_lastWheelLocalPos;
    QPointF m_lastWheelGlobalPos;

    void beginScrollHoverSuppression(
        QQuickWindow* win,
        const QPointF& localPos,
        const QPointF& globalPos,
        qint64 nowMs
    ) {
        if (!m_suppressingScrollHover) {
            if (win) {
                // Sending HoverLeave to the QQuickWindow triggers QQuickDeliveryAgentPrivate::clearHover()
                // (un-hovering all QQuickItems, MouseAreas, and HoverHandlers) and resets
                // QQuickDeliveryAgentPrivate::lastMousePosition to (0, 0), which makes
                // flushFrameSynchronousEvents() skip recursive scene-graph hover traversal on every scroll frame.
                QHoverEvent hoverLeaveEv(
                    QEvent::HoverLeave,
                    QPointF(-1.0, -1.0),
                    globalPos,
                    localPos,
                    Qt::NoModifier
                );
                QCoreApplication::sendEvent(win, &hoverLeaveEv);
            }
            m_suppressingScrollHover = true;
            m_lastTimerRestartMs = nowMs;
            m_scrollHoverResumeTimer.start(400);
        } else if (nowMs - m_lastTimerRestartMs >= 100) {
            m_lastTimerRestartMs = nowMs;
            m_scrollHoverResumeTimer.start(400);
        }
        m_lastScrolledWindow = win;
        m_lastWheelLocalPos = localPos;
        m_lastWheelGlobalPos = globalPos;
    }

    static bool hasDirectWheelHandlerChild(QQuickItem* item) {
        if (!item || item->inherits("QQuickScrollView")) {
            return false;
        }
        const QObjectList& objs = item->children();
        for (QObject* obj : objs) {
            if (obj && obj->inherits("QQuickWheelHandler")) {
                return true;
            }
        }
        return false;
    }

    static bool tryScrollFlickableItem(
        QQuickItem* flick,
        double dyPx,
        double dxPx,
        QQuickItem*& fallbackBoundaryFlickable,
        QPointer<QQuickItem>* outMovedFlickable = nullptr
    ) {
        if (!flick || !flick->inherits("QQuickFlickable")) {
            return false;
        }
        const QVariant interVar = flick->property("interactive");
        if (interVar.isValid() && !interVar.toBool()) {
            return false;
        }
        const double originY = flick->property("originY").toDouble();
        const double topMargin = flick->property("topMargin").toDouble();
        const double bottomMargin = flick->property("bottomMargin").toDouble();
        const double contentH = flick->property("contentHeight").toDouble();
        const double viewH = flick->height();
        const double minY = originY - topMargin;
        const double maxY = std::max(minY, originY + contentH + bottomMargin - viewH);
        const double curY = flick->property("contentY").toDouble();

        const double originX = flick->property("originX").toDouble();
        const double leftMargin = flick->property("leftMargin").toDouble();
        const double rightMargin = flick->property("rightMargin").toDouble();
        const double contentW = flick->property("contentWidth").toDouble();
        const double viewW = flick->width();
        const double minX = originX - leftMargin;
        const double maxX = std::max(minX, originX + contentW + rightMargin - viewW);
        const double curX = flick->property("contentX").toDouble();

        bool moved = false;
        bool isScrollable = false;
        if (maxY > minY + 0.5 && std::abs(dyPx) > 0.001) {
            isScrollable = true;
            const double targetY = std::clamp(curY - dyPx, minY, maxY);
            if (std::abs(targetY - curY) > 0.001) {
                if (flick->property("flicking").toBool()) {
                    QMetaObject::invokeMethod(flick, "cancelFlick");
                }
                flick->setProperty("contentY", targetY);
                moved = true;
            }
        }
        if (maxX > minX + 0.5 && std::abs(dxPx) > 0.001) {
            isScrollable = true;
            const double targetX = std::clamp(curX - dxPx, minX, maxX);
            if (std::abs(targetX - curX) > 0.001) {
                if (flick->property("flicking").toBool()) {
                    QMetaObject::invokeMethod(flick, "cancelFlick");
                }
                flick->setProperty("contentX", targetX);
                moved = true;
            }
        }
        if (moved) {
            if (outMovedFlickable) {
                *outMovedFlickable = flick;
            }
            return true;
        }
        if (isScrollable && !fallbackBoundaryFlickable) {
            fallbackBoundaryFlickable = flick;
        }
        return false;
    }

    static bool findAndScrollFlickableAt(
        QQuickItem* item,
        const QPointF& scenePos,
        double dyPx,
        double dxPx,
        bool& hitBlockingControl,
        QQuickItem*& fallbackBoundaryFlickable,
        QPointer<QQuickItem>* outMovedFlickable = nullptr
    ) {
        if (!item || !item->isVisible() || !item->isEnabled() || item->opacity() <= 0.001) {
            return false;
        }
        const QPointF localPos = item->mapFromScene(scenePos);
        const bool inside =
            QRectF(0.0, 0.0, item->width(), item->height()).contains(localPos);
        if (!inside && item->clip()) {
            return false;
        }

        const QList<QQuickItem*> rawChildren = item->childItems();
        if (!rawChildren.isEmpty()) {
            QList<QQuickItem*> sortedChildren;
            sortedChildren.reserve(rawChildren.size());
            for (QQuickItem* c : rawChildren) {
                if (c && c->isVisible() && c->isEnabled() && c->opacity() > 0.001) {
                    sortedChildren.append(c);
                }
            }
            // Sort in reverse paint order: highest z() first; for equal z(), last child first
            std::stable_sort(
                sortedChildren.begin(),
                sortedChildren.end(),
                [](QQuickItem* a, QQuickItem* b) { return a->z() > b->z(); }
            );
            // Since std::stable_sort keeps original order for equal z(), reverse equal-z runs
            int i = 0;
            while (i < sortedChildren.size()) {
                int j = i + 1;
                const qreal zVal = sortedChildren[i]->z();
                while (j < sortedChildren.size() && sortedChildren[j]->z() == zVal) {
                    ++j;
                }
                std::reverse(sortedChildren.begin() + i, sortedChildren.begin() + j);
                i = j;
            }

            for (QQuickItem* child : sortedChildren) {
                if (findAndScrollFlickableAt(
                        child,
                        scenePos,
                        dyPx,
                        dxPx,
                        hitBlockingControl,
                        fallbackBoundaryFlickable,
                        outMovedFlickable
                    )) {
                    return true;
                }
                if (hitBlockingControl) {
                    return false;
                }
            }
        }

        if (!inside) {
            return false;
        }

        if (item->inherits("QQuickSlider") || item->inherits("QQuickDial")
            || item->inherits("QQuickSpinBox") || item->inherits("QQuickTumbler")
            || (item->inherits("QQuickTextField") && hasDirectWheelHandlerChild(item))) {
            hitBlockingControl = true;
            return false;
        }

        if (item->objectName() == QStringLiteral("highContrastScrollBar")
            || item->inherits("QQuickScrollBar")) {
            if (QQuickItem* p = item->parentItem()) {
                if (tryScrollFlickableItem(
                        p,
                        dyPx,
                        dxPx,
                        fallbackBoundaryFlickable,
                        outMovedFlickable
                    )) {
                    return true;
                }
            }
        }

        if (item->inherits("QQuickFlickable")) {
            if (tryScrollFlickableItem(
                    item,
                    dyPx,
                    dxPx,
                    fallbackBoundaryFlickable,
                    outMovedFlickable
                )) {
                return true;
            }
        }
        return false;
    }

protected:
    bool eventFilter(QObject* watched, QEvent* event) override {
        if (!watched || !event) {
            return false;
        }
        const QEvent::Type evType = event->type();
        if (m_suppressingScrollHover) {
            if (!watched->isWindowType()) {
                // While actively wheel-scrolling, block HoverEnter/HoverMove/ToolTip on scene items
                // and HoverHandlers so passing controls do not trigger hover state or pop up tooltips
                // mid-scroll. HoverLeave is allowed through so the previously hovered item un-hovers.
                if (evType == QEvent::HoverEnter || evType == QEvent::HoverMove
                    || evType == QEvent::ToolTip) {
                    event->accept();
                    return true;
                }
                return false;
            }
            if (evType == QEvent::MouseMove) {
                auto* me = static_cast<QMouseEvent*>(event);
                const QPointF delta = me->globalPosition() - m_lastWheelGlobalPos;
                if (std::abs(delta.x()) > 3.0 || std::abs(delta.y()) > 3.0) {
                    m_suppressingScrollHover = false;
                    m_scrollHoverResumeTimer.stop();
                }
            } else if (evType == QEvent::HoverMove) {
                auto* he = static_cast<QHoverEvent*>(event);
                const QPointF delta = he->globalPosition() - m_lastWheelGlobalPos;
                if (std::abs(delta.x()) > 3.0 || std::abs(delta.y()) > 3.0) {
                    m_suppressingScrollHover = false;
                    m_scrollHoverResumeTimer.stop();
                } else {
                    event->accept();
                    return true;
                }
            }
        }
        if (!watched->isWindowType()) {
            return false;
        }
        if (evType == QEvent::Show || evType == QEvent::PlatformSurface) {
            QWindow* win = qobject_cast<QWindow*>(watched);
            QWindow* mainWin = main_window_ref().data();
            if (win && mainWin && win != mainWin
                && win->objectName() != QStringLiteral("duckydeckOledWidgetWindow")
                && mainWin->screen() && win->screen() != mainWin->screen()) {
                win->setScreen(mainWin->screen());
            }
            return false;
        }
        if (evType == QEvent::Wheel) {
            QQuickWindow* quickWin = qobject_cast<QQuickWindow*>(watched);
            if (!quickWin
                || quickWin->objectName() == QStringLiteral("duckydeckOledWidgetWindow")
                || quickWin->property("isRecording").toBool()) {
                return false;
            }
            QWheelEvent* we = static_cast<QWheelEvent*>(event);
            if (we->modifiers() & Qt::ControlModifier) {
                return false;
            }
            const QPoint angle = we->angleDelta();
            const QPoint pixel = we->pixelDelta();
            if (angle.isNull() && pixel.isNull()) {
                return false;
            }

            // Standard linear v120 conversion (60 px per 120 angleDelta units = 0.5 px per unit),
            // matching standard desktop applications (Firefox, Chrome, KDE Widgets) without any
            // artificial boost while bypassing QQuickFlickable's NoScrollPhase velocity deadzone.
            double dyPx = 0.0;
            double dxPx = 0.0;
            if (!angle.isNull()) {
                dyPx = static_cast<double>(angle.y()) * 0.50;
                dxPx = static_cast<double>(angle.x()) * 0.50;
            } else {
                dyPx = static_cast<double>(pixel.y());
                dxPx = static_cast<double>(pixel.x());
            }

            if (we->inverted()) {
                dyPx = -dyPx;
                dxPx = -dxPx;
            }

            QQuickItem* rootItem = quickWin->contentItem();
            if (rootItem && rootItem->parentItem()) {
                rootItem = rootItem->parentItem();
            }
            if (!rootItem) {
                return false;
            }

            const qint64 nowMs = QDateTime::currentMSecsSinceEpoch();
            const QPointF cursorDelta = we->globalPosition() - m_lastWheelGlobalPos;
            const bool sameGesture = (m_lastScrolledWindow == quickWin)
                && (m_lastWheelEventMs > 0)
                && (nowMs - m_lastWheelEventMs <= 350)
                && (std::abs(cursorDelta.x()) <= 4.0)
                && (std::abs(cursorDelta.y()) <= 4.0);
            m_lastWheelEventMs = nowMs;

            // Suppress item HoverEnter/HoverMove BEFORE updating contentY so scene items
            // moving under the stationary cursor do not trigger tooltips or hover repaints.
            beginScrollHoverSuppression(quickWin, we->position(), we->globalPosition(), nowMs);

            if (sameGesture && m_lockedFlickable && m_lockedFlickable->isVisible()
                && m_lockedFlickable->isEnabled()) {
                QQuickItem* boundaryFlick = nullptr;
                for (QQuickItem* cur = m_lockedFlickable.data(); cur != nullptr;
                     cur = cur->parentItem()) {
                    if (cur->inherits("QQuickFlickable")
                        && tryScrollFlickableItem(
                            cur,
                            dyPx,
                            dxPx,
                            boundaryFlick,
                            &m_lockedFlickable
                        )) {
                        we->accept();
                        return true;
                    }
                }
                if (boundaryFlick) {
                    we->accept();
                    return true;
                }
            }

            m_lockedFlickable = nullptr;
            bool hitBlockingControl = false;
            QQuickItem* fallbackBoundaryFlickable = nullptr;
            if (findAndScrollFlickableAt(
                    rootItem,
                    we->position(),
                    dyPx,
                    dxPx,
                    hitBlockingControl,
                    fallbackBoundaryFlickable,
                    &m_lockedFlickable
                )) {
                we->accept();
                return true;
            }
            if (!hitBlockingControl && fallbackBoundaryFlickable) {
                m_lockedFlickable = fallbackBoundaryFlickable;
                we->accept();
                return true;
            }
            return false;
        }
        if (evType != QEvent::KeyPress) {
            return false;
        }
        QKeyEvent* ke = static_cast<QKeyEvent*>(event);
        const int key = ke->key();
        const Qt::KeyboardModifiers mods = ke->modifiers();
        QWindow* win = qobject_cast<QWindow*>(watched);
        QWindow* mainWin = find_main_window();

        if (key == Qt::Key_Escape && (mods == Qt::NoModifier || mods == Qt::KeypadModifier)) {
            if (win && win->property("isRecording").toBool()) {
                return false;
            }
            if (win && win->objectName() == QStringLiteral("duckydeckOledWidgetWindow")) {
                const bool showSize = win->property("showSizeSlider").toBool();
                const bool showTrans = win->property("showTransparencySlider").toBool();
                if (showSize || showTrans) {
                    win->setProperty("showSizeSlider", false);
                    win->setProperty("showTransparencySlider", false);
                } else {
                    hide_oled_widget_window();
                }
                ke->accept();
                return true;
            }
            if (mainWin) {
                mainWin->setProperty("lastPopupClosedByEsc", false);
                if (invoke_qml_method(mainWin, "closeTopmostPopupWindow")
                    && mainWin->property("lastPopupClosedByEsc").toBool()) {
                    ke->accept();
                    return true;
                }
            }
            if (win && win != mainWin && win->isVisible()) {
                win->close();
                ke->accept();
                return true;
            }
            return false;
        }

        if ((mods & Qt::ControlModifier) && !(mods & Qt::AltModifier) && !(mods & Qt::MetaModifier)) {
            if (key == Qt::Key_Plus || key == Qt::Key_Equal) {
                if (mainWin) {
                    invoke_qml_method(mainWin, "zoomIn");
                    ke->accept();
                    return true;
                }
            } else if (key == Qt::Key_Minus || key == Qt::Key_Underscore) {
                if (mainWin) {
                    invoke_qml_method(mainWin, "zoomOut");
                    ke->accept();
                    return true;
                }
            } else if (key == Qt::Key_0 || key == Qt::Key_ParenRight) {
                if (mainWin) {
                    invoke_qml_method(mainWin, "resetZoom");
                    ke->accept();
                    return true;
                }
            }
        }
        return false;
    }
};

inline void ensure_global_key_filter() {
    QCoreApplication* app = QCoreApplication::instance();
    if (!app) {
        return;
    }
    static QPointer<DuckyDeckGlobalKeyFilter> s_filter;
    if (!s_filter) {
        s_filter = new DuckyDeckGlobalKeyFilter(app);
        app->installEventFilter(s_filter);
    }
}

} // namespace duckydeck

template <typename ControllerT>
inline void duckydeck_sync_tray(
    ControllerT& controller,
    const QString& iconMode,
    const QString& statusHeader,
    const QString& profileHeader,
    const QString& sdSpaceHeader,
    const QString& tooltipText,
    std::uint8_t flags
) {
    const bool useLightOnDarkIcon = (flags & 0x01u) != 0u;
    const bool autoSwitchEnabled = (flags & 0x02u) != 0u;
    const bool autostartEnabled = (flags & 0x04u) != 0u;
    duckydeck::TrayManager::instance().sync(
        &controller,
        useLightOnDarkIcon,
        iconMode,
        statusHeader,
        profileHeader,
        sdSpaceHeader,
        tooltipText,
        autoSwitchEnabled,
        autostartEnabled
    );
}

inline void duckydeck_show_main_window() {
    if (QCoreApplication::instance()
        && QThread::currentThread() != QCoreApplication::instance()->thread()) {
        QMetaObject::invokeMethod(
            QCoreApplication::instance(),
            []() { duckydeck::show_main_window(); },
            Qt::QueuedConnection
        );
        return;
    }
    duckydeck::show_main_window();
}

inline void duckydeck_hide_main_window() {
    if (QCoreApplication::instance()
        && QThread::currentThread() != QCoreApplication::instance()->thread()) {
        QMetaObject::invokeMethod(
            QCoreApplication::instance(),
            []() { duckydeck::hide_main_window_to_tray(); },
            Qt::QueuedConnection
        );
        return;
    }
    duckydeck::hide_main_window_to_tray();
}

inline void duckydeck_toggle_oled_widget() {
    duckydeck::toggle_oled_widget_window();
}

inline void duckydeck_restore_oled_widget_geometry() {
    if (QCoreApplication::instance()
        && QThread::currentThread() != QCoreApplication::instance()->thread()) {
        QMetaObject::invokeMethod(
            QCoreApplication::instance(),
            []() { (void)duckydeck::restore_oled_widget_geometry_now(); },
            Qt::QueuedConnection
        );
        return;
    }
    (void)duckydeck::restore_oled_widget_geometry_now();
}

inline void duckydeck_save_oled_widget_appearance() {
    if (QCoreApplication::instance()
        && QThread::currentThread() != QCoreApplication::instance()->thread()) {
        QMetaObject::invokeMethod(
            QCoreApplication::instance(),
            []() { duckydeck::save_oled_widget_appearance_now(); },
            Qt::QueuedConnection
        );
        return;
    }
    duckydeck::save_oled_widget_appearance_now();
}

inline void duckydeck_record_oled_geometry(int32_t x, int32_t y, int32_t width, int32_t height) {
    if (QCoreApplication::instance()
        && QThread::currentThread() != QCoreApplication::instance()->thread()) {
        QMetaObject::invokeMethod(
            QCoreApplication::instance(),
            [x, y, width, height]() {
                duckydeck::record_oled_widget_geometry_from_kwin(x, y, width, height);
            },
            Qt::QueuedConnection
        );
        return;
    }
    duckydeck::record_oled_widget_geometry_from_kwin(x, y, width, height);
}

inline void duckydeck_quit_app() {
    duckydeck::quit_duckydeck_application();
}

inline void duckydeck_exit_app(int32_t exit_code) {
    if (QCoreApplication::instance()) {
        QCoreApplication::exit(exit_code);
        QCoreApplication::quit();
    }
    std::fflush(stdout);
    std::fflush(stderr);
    ::_exit(exit_code);
}

inline void duckydeck_restart_app() {
    duckydeck::restart_duckydeck_application();
}

inline void duckydeck_show_tray_notification(const QString& title, const QString& message) {
    const QString t = title;
    const QString m = message;
    if (QCoreApplication::instance()
        && QThread::currentThread() != QCoreApplication::instance()->thread()) {
        QMetaObject::invokeMethod(
            QCoreApplication::instance(),
            [t, m]() { duckydeck::TrayManager::instance().showNotification(t, m); },
            Qt::QueuedConnection
        );
        return;
    }
    duckydeck::TrayManager::instance().showNotification(t, m);
}

inline bool duckydeck_extract_qrc_user_guide(const QString& dest_path) {
    const QStringList candidates = {
        QStringLiteral(":/qt/qml/io/github/duckydeck/docs/user-guide.html"),
        QStringLiteral(":/docs/user-guide.html"),
    };
    for (const QString& res_path : candidates) {
        QFile res_file(res_path);
        if (res_file.exists() && res_file.open(QIODevice::ReadOnly)) {
            const QByteArray bytes = res_file.readAll();
            res_file.close();
            if (!bytes.isEmpty()) {
                QFile out_file(dest_path);
                if (out_file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
                    out_file.write(bytes);
                    out_file.close();
                    return true;
                }
            }
        }
    }
    return false;
}

inline void duckydeck_dispatch_ipc_command(const QString& command) {
    const QString copied = command;
    if (QCoreApplication::instance()
        && QThread::currentThread() != QCoreApplication::instance()->thread()) {
        QMetaObject::invokeMethod(
            QCoreApplication::instance(),
            [copied]() {
                const QStringList lines = copied.split(QLatin1Char('\n'), Qt::SkipEmptyParts);
                for (const QString& line : lines) {
                    duckydeck::TrayManager::instance().dispatchIpcCommand(line);
                }
            },
            Qt::QueuedConnection
        );
        return;
    }
    const QStringList lines = copied.split(QLatin1Char('\n'), Qt::SkipEmptyParts);
    for (const QString& line : lines) {
        duckydeck::TrayManager::instance().dispatchIpcCommand(line);
    }
}

inline void duckydeck_apply_app_theme(const QString& themeMode) {
    if (!QCoreApplication::instance()) {
        return;
    }
#if defined(_WIN32)
    const bool isLight = (themeMode == QStringLiteral("light"));
    duckydeck::KirigamiTheme::setLightMode(isLight);
    QPalette pal;
    const QColor winBg = isLight ? QColor(0xef, 0xf0, 0xf1) : QColor(0x20, 0x23, 0x26);
    const QColor winText = isLight ? QColor(0x23, 0x26, 0x29) : QColor(0xfc, 0xfc, 0xfc);
    const QColor baseBg = isLight ? QColor(0xff, 0xff, 0xff) : QColor(0x1b, 0x1e, 0x20);
    const QColor altBg = isLight ? QColor(0xf7, 0xf7, 0xf7) : QColor(0x23, 0x26, 0x29);
    const QColor btnBg = isLight ? QColor(0xef, 0xf0, 0xf1) : QColor(0x29, 0x2c, 0x30);
    const QColor disText = isLight ? QColor(0x52, 0x57, 0x5c) : QColor(0xa1, 0xa9, 0xb1);
    const QColor hiBg(0x3d, 0xae, 0xe9);
    const QColor hiText(0xff, 0xff, 0xff);
    for (QPalette::ColorGroup cg : {QPalette::Active, QPalette::Inactive, QPalette::Disabled}) {
        const QColor fg = (cg == QPalette::Disabled) ? disText : winText;
        pal.setColor(cg, QPalette::Window, winBg);
        pal.setColor(cg, QPalette::WindowText, fg);
        pal.setColor(cg, QPalette::Base, baseBg);
        pal.setColor(cg, QPalette::AlternateBase, altBg);
        pal.setColor(cg, QPalette::ToolTipBase, altBg);
        pal.setColor(cg, QPalette::ToolTipText, fg);
        pal.setColor(cg, QPalette::Text, fg);
        pal.setColor(cg, QPalette::Button, btnBg);
        pal.setColor(cg, QPalette::ButtonText, fg);
        pal.setColor(cg, QPalette::BrightText, hiText);
        pal.setColor(cg, QPalette::Highlight, hiBg);
        pal.setColor(cg, QPalette::HighlightedText, hiText);
        pal.setColor(cg, QPalette::PlaceholderText, disText);
    }
    QGuiApplication::setPalette(pal);
    QApplication::setPalette(pal);
    auto applyQuickColorGroup = [&](QObject* cg, const QColor& fg) {
        if (!cg) return;
        cg->setProperty("window", winBg);
        cg->setProperty("windowText", fg);
        cg->setProperty("base", baseBg);
        cg->setProperty("alternateBase", altBg);
        cg->setProperty("toolTipBase", altBg);
        cg->setProperty("toolTipText", fg);
        cg->setProperty("text", fg);
        cg->setProperty("button", btnBg);
        cg->setProperty("buttonText", fg);
        cg->setProperty("brightText", hiText);
        cg->setProperty("highlight", hiBg);
        cg->setProperty("highlightedText", hiText);
        cg->setProperty("placeholderText", disText);
    };
    auto applyQuickPalette = [&](QObject* owner) {
        if (!owner) return;
        QObject* qp = owner->property("palette").value<QObject*>();
        if (!qp) return;
        applyQuickColorGroup(qp, winText);
        applyQuickColorGroup(qp->property("active").value<QObject*>(), winText);
        applyQuickColorGroup(qp->property("inactive").value<QObject*>(), winText);
        applyQuickColorGroup(qp->property("disabled").value<QObject*>(), disText);
    };
    const auto allWins = QGuiApplication::allWindows();
    for (QWindow* w : allWins) {
        if (!w) continue;
        applyQuickPalette(w);
        if (QQuickWindow* qw = qobject_cast<QQuickWindow*>(w)) {
            applyQuickPalette(qw->contentItem());
        }
    }
#else
    static QString s_lastAppliedTheme;
    if (s_lastAppliedTheme == themeMode) {
        return;
    }
    s_lastAppliedTheme = themeMode;
    KColorSchemeManager* mgr = KColorSchemeManager::instance();
    if (!mgr) {
        return;
    }
    mgr->setAutosaveChanges(false);
    QModelIndex targetIdx;
    if (themeMode == QStringLiteral("dark")) {
        targetIdx = mgr->indexForSchemeId(QStringLiteral("BreezeDark"));
        if (!targetIdx.isValid()) {
            targetIdx = mgr->indexForScheme(QStringLiteral("Breeze Dark"));
        }
    } else if (themeMode == QStringLiteral("light")) {
        targetIdx = mgr->indexForSchemeId(QStringLiteral("BreezeLight"));
        if (!targetIdx.isValid()) {
            targetIdx = mgr->indexForScheme(QStringLiteral("Breeze Light"));
        }
    }

    if (targetIdx.isValid()) {
        mgr->activateScheme(targetIdx);
        if (const QAbstractItemModel* model = mgr->model()) {
            const QString schemePath = model->data(targetIdx, Qt::UserRole).toString();
            if (!schemePath.isEmpty()) {
                const QPalette pal = KColorScheme::createApplicationPalette(
                    KSharedConfig::openConfig(schemePath)
                );
                QApplication::setPalette(pal);
                if (KIconLoader* loader = KIconLoader::global()) {
                    loader->setCustomPalette(pal);
                    KIconLoader::emitChange(KIconLoader::Desktop);
                }
            }
        }
    } else {
        mgr->activateScheme(QModelIndex());
        const QPalette sysPal = KColorScheme::createApplicationPalette(
            KSharedConfig::openConfig()
        );
        QApplication::setPalette(sysPal);
        if (KIconLoader* loader = KIconLoader::global()) {
            loader->resetPalette();
            KIconLoader::emitChange(KIconLoader::Desktop);
        }
    }
#endif
}

inline void duckydeck_apply_ui_font_scale(int32_t scalePercent) {
    if (!QCoreApplication::instance()) {
        return;
    }
    const int clamped = std::clamp(static_cast<int>(scalePercent), 75, 160);
    static const double s_osBasePt = std::max(
        10.0,
        QGuiApplication::font().pointSizeF() > 0.0 ? QGuiApplication::font().pointSizeF() : 10.5
    );
    QFont f = QGuiApplication::font();
    f.setPointSizeF(std::round(s_osBasePt * (static_cast<double>(clamped) / 100.0) * 10.0) / 10.0);
    QGuiApplication::setFont(f);
}

inline bool duckydeck_send_key_to_active_window(
    int32_t qtKey,
    int32_t qtModifiers,
    const QString& text
) {
    if (!QCoreApplication::instance()) {
        return false;
    }
    duckydeck::ensure_global_key_filter();
    QWindow* mainWin = duckydeck::find_main_window();
    QWindow* target = QGuiApplication::focusWindow();
    if (!target || !target->isVisible()) {
        target = nullptr;
        const auto windows = QGuiApplication::topLevelWindows();
        for (QWindow* w : windows) {
            if (w && w != mainWin && w->isVisible()
                && w->objectName() != QStringLiteral("duckydeckOledWidgetWindow")) {
                target = w;
                break;
            }
        }
        if (!target) {
            QWindow* oledWin = duckydeck::find_oled_widget_window();
            if (oledWin && oledWin->isVisible() && qtKey == Qt::Key_Escape) {
                target = oledWin;
            } else {
                target = mainWin;
            }
        }
    }
    if (!target) {
        return false;
    }
    QKeyEvent pressEv(
        QEvent::KeyPress,
        qtKey,
        static_cast<Qt::KeyboardModifiers>(qtModifiers),
        text
    );
    QCoreApplication::sendEvent(target, &pressEv);
    QKeyEvent releaseEv(
        QEvent::KeyRelease,
        qtKey,
        static_cast<Qt::KeyboardModifiers>(qtModifiers),
        text
    );
    QCoreApplication::sendEvent(target, &releaseEv);
    return pressEv.isAccepted();
}

#if !defined(_WIN32)
inline void terminate_existing_pids(const QList<pid_t>& pids) {
    for (pid_t pid : pids) {
        if (pid > 1 && ::kill(pid, 0) == 0) {
            ::kill(pid, SIGTERM);
        }
    }
    for (int step = 0; step < 30; ++step) {
        bool anyAlive = false;
        for (pid_t pid : pids) {
            if (pid > 1 && ::kill(pid, 0) == 0) {
                anyAlive = true;
                break;
            }
        }
        if (!anyAlive) {
            return;
        }
        QThread::msleep(50);
    }
    for (pid_t pid : pids) {
        if (pid > 1 && ::kill(pid, 0) == 0) {
            ::kill(pid, SIGKILL);
        }
    }
    QThread::msleep(60);
}
#endif

inline QtMessageHandler& prev_qt_msg_handler() {
    static QtMessageHandler h = nullptr;
    return h;
}

inline void duckydeck_qt_msg_handler(QtMsgType type, const QMessageLogContext& ctx, const QString& msg) {
    if (msg.contains(QStringLiteral("org/kde/breeze/TextArea.qml"))
        && msg.contains(QStringLiteral("QQuickTextInput"))) {
        return;
    }
    if (msg.contains(QStringLiteral("cached device pixel ratio value was stale on window update"))) {
        return;
    }
    if (QtMessageHandler prev = prev_qt_msg_handler()) {
        prev(type, ctx, msg);
    }
}

inline int duckydeck_run_app(bool start_minimized) {
    if (!prev_qt_msg_handler()) {
        prev_qt_msg_handler() = qInstallMessageHandler(duckydeck_qt_msg_handler);
    }
    QLoggingCategory::setFilterRules(QStringLiteral("union.*=false\nqt.core.qobject.connect=false"));
    if (!std::getenv("QT_QUICK_CONTROLS_STYLE")) {
#if defined(_WIN32)
        qputenv("QT_QUICK_CONTROLS_STYLE", "Fusion");
#else
        qputenv("QT_QUICK_CONTROLS_STYLE", "org.kde.breeze");
#endif
    }
    if (std::getenv("DUCKYDECK_DEMO_SHOWCASE") && std::string(std::getenv("DUCKYDECK_DEMO_SHOWCASE")) == "1") {
        qputenv("QT_QPA_PLATFORM", "xcb");
        qputenv("QT_SCALE_FACTOR", "1");
        qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", "0");
    }
    const bool isUiTestSuite = qEnvironmentVariable("DUCKYDECK_UI_TEST_SUITE") == QStringLiteral("1");
    if (isUiTestSuite) {
        qputenv("DUCKYDECK_UI_TEST_SUITE", "1");
        const QString reportPath = qEnvironmentVariable("DUCKYDECK_UI_TEST_REPORT");
        if (!reportPath.isEmpty()) {
            qputenv("DUCKYDECK_UI_TEST_REPORT", reportPath.toUtf8());
        }
        if (qEnvironmentVariable("QT_QPA_PLATFORM").isEmpty()) {
            qputenv("QT_QPA_PLATFORM", "offscreen");
        }
#if defined(_WIN32)
        if (qEnvironmentVariable("QT_QPA_FONTDIR").isEmpty()) {
            qputenv("QT_QPA_FONTDIR", "C:\\Windows\\Fonts");
        }
#endif
        qputenv("QT_SCALE_FACTOR", "1");
        qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", "0");
        qputenv("QT_SCREEN_SCALE_FACTORS", "1");
    }
    if (start_minimized) {
        qputenv("DUCKYDECK_START_MINIMIZED", "1");
    }

    static int argc = 1;
    static char arg0[] = "duckydeck-studio";
    static char* argv[] = {arg0, nullptr};

    QApplication app(argc, argv);
    QGuiApplication::setApplicationName(QStringLiteral("DuckyDeck Studio"));
    QGuiApplication::setApplicationDisplayName(QStringLiteral("DuckyDeck Studio"));
    QGuiApplication::setDesktopFileName(QStringLiteral("io.github.duckydeck.Studio"));
    QGuiApplication::setQuitOnLastWindowClosed(false);

#ifndef QT_NO_SESSIONMANAGER
    if (app.isSessionRestored()) {
        start_minimized = true;
        qputenv("DUCKYDECK_START_MINIMIZED", "1");
    }
#endif

    duckydeck_apply_app_theme(duckydeck::load_saved_app_theme_mode());
    duckydeck_apply_ui_font_scale(duckydeck::load_saved_ui_text_scale());
    duckydeck::ensure_global_key_filter();

    QIcon initialIcon = duckydeck::build_multisize_svg_icon(
        QStringLiteral(":/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg")
    );
    QGuiApplication::setWindowIcon(initialIcon);

#if !defined(_WIN32)
    // Single-instance check: offer to close previous running instance and start new one
    const QString existingPidsRaw = qEnvironmentVariable("DUCKYDECK_EXISTING_PIDS").trimmed();
    if (!isUiTestSuite && !existingPidsRaw.isEmpty()) {
        QList<pid_t> alivePids;
        QStringList aliveStrs;
        const QStringList parts = existingPidsRaw.split(QLatin1Char(','), Qt::SkipEmptyParts);
        for (const QString& part : parts) {
            bool ok = false;
            const int pidVal = part.trimmed().toInt(&ok);
            if (ok && pidVal > 1 && pidVal != static_cast<int>(QCoreApplication::applicationPid()) && ::kill(static_cast<pid_t>(pidVal), 0) == 0) {
                alivePids.append(static_cast<pid_t>(pidVal));
                aliveStrs.append(QString::number(pidVal));
            }
        }

        if (!alivePids.isEmpty()) {
            const bool forceReplace = qEnvironmentVariable("DUCKYDECK_REPLACE_EXISTING") == QStringLiteral("1");
            const bool isOffscreen = qEnvironmentVariable("QT_QPA_PLATFORM") == QStringLiteral("offscreen");
            bool shouldReplace = forceReplace || isOffscreen || start_minimized;

            if (!shouldReplace) {
                QMessageBox msgBox;
                msgBox.setWindowTitle(QStringLiteral("DuckyDeck Studio — Already Running"));
                msgBox.setWindowIcon(initialIcon);
                msgBox.setIcon(QMessageBox::Question);
                msgBox.setText(
                    QStringLiteral("Another instance of DuckyDeck Studio is already running (PID %1).")
                        .arg(aliveStrs.join(QStringLiteral(", ")))
                );
                msgBox.setInformativeText(
                    QStringLiteral("Only one copy of DuckyDeck Studio should run at a time so USB HID and KWin Auto-Switcher events do not conflict.\n\nWould you like to close the previous instance and start a new one?")
                );
                QPushButton* replaceBtn = msgBox.addButton(
                    QStringLiteral("Close Previous && Start New"),
                    QMessageBox::AcceptRole
                );
                QPushButton* cancelBtn = msgBox.addButton(
                    QStringLiteral("Cancel"),
                    QMessageBox::RejectRole
                );
                msgBox.setDefaultButton(replaceBtn);
                msgBox.setEscapeButton(cancelBtn);
                msgBox.exec();

                shouldReplace = (msgBox.clickedButton() == replaceBtn);
            }

            if (!shouldReplace) {
                return 0;
            }
            terminate_existing_pids(alivePids);
        }
    }
#endif

    const QString pidFilePath = qEnvironmentVariable("DUCKYDECK_PID_FILE").trimmed();
    if (!isUiTestSuite && !pidFilePath.isEmpty()) {
        QFile pidFile(pidFilePath);
        if (pidFile.open(QIODevice::WriteOnly | QIODevice::Truncate | QIODevice::Text)) {
            pidFile.write(QByteArray::number(static_cast<qlonglong>(QCoreApplication::applicationPid())) + "\n");
            pidFile.close();
        }
    }

#if defined(_WIN32)
    duckydeck::register_windows_kirigami_compat();
#endif
    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/Main.qml")));

    if (engine.rootObjects().isEmpty()) {
        return 1;
    }
#if defined(_WIN32)
    duckydeck_apply_app_theme(duckydeck::load_saved_app_theme_mode());
#endif

    QWindow* win = qobject_cast<QWindow*>(engine.rootObjects().first());
    if (win) {
        duckydeck::main_window_ref() = win;
        duckydeck::ensure_window_hooks(win);
        win->setIcon(initialIcon);
        if (qEnvironmentVariable("DUCKYDECK_DEMO_SHOWCASE") == QStringLiteral("1")) {
            win->resize(1920, 1080);
        }
        if (start_minimized) {
            win->setVisible(false);
            win->hide();
            QTimer::singleShot(0, win, [win]() {
                if (win) {
                    win->setVisible(false);
                    win->hide();
                    duckydeck::update_tray_window_actions();
                }
            });
            QTimer::singleShot(120, win, [win]() {
                if (win) {
                    win->setVisible(false);
                    win->hide();
                    duckydeck::update_tray_window_actions();
                }
            });
        } else {
            win->show();
            win->raise();
            win->requestActivate();
            duckydeck::update_tray_window_actions();
        }
        const bool shouldRestoreOledWidget =
            !isUiTestSuite
            && (qEnvironmentVariable("DUCKYDECK_SHOW_OLED_WIDGET") == QStringLiteral("1")
                || (qEnvironmentVariable("DUCKYDECK_DEMO_SHOWCASE") != QStringLiteral("1")
                    && duckydeck::load_oled_widget_geometry().visible));
        if (shouldRestoreOledWidget) {
            QTimer::singleShot(180, win, []() {
                duckydeck::show_oled_widget_window();
            });
        }
    }

    const int rc = app.exec();

    if (!pidFilePath.isEmpty()) {
        QFile pidFile(pidFilePath);
        if (pidFile.exists()) {
            pidFile.remove();
        }
    }
    return rc;
}

inline bool duckydeck_capture_composite_frame(
    const QString& outputPath,
    const QString& bannerTitle,
    const QString& bannerSubtitle
) {
    QCursor::setPos(2, 2);
    QWindow* rawMain = duckydeck::find_main_window();
    QQuickWindow* mainWin = qobject_cast<QQuickWindow*>(rawMain);
    if (!mainWin) {
        return false;
    }

    QImage mainImg = mainWin->grabWindow();
    if (mainImg.isNull()) {
        return false;
    }

    QImage canvas(1920, 1080, QImage::Format_ARGB32_Premultiplied);
    canvas.fill(QColor(18, 21, 29));

    QPainter p(&canvas);
    p.setRenderHint(QPainter::Antialiasing, true);
    p.setRenderHint(QPainter::SmoothPixmapTransform, true);
    p.setRenderHint(QPainter::TextAntialiasing, true);

    QImage scaledMain = mainImg.scaled(1920, 1080, Qt::KeepAspectRatio, Qt::SmoothTransformation);
    const int mainPosX = (1920 - scaledMain.width()) / 2;
    const int mainPosY = (1080 - scaledMain.height()) / 2;
    p.drawImage(mainPosX, mainPosY, scaledMain);

    const double scaleX = 1920.0 / std::max(1, mainWin->width());
    const double scaleY = 1080.0 / std::max(1, mainWin->height());
    const double uniformScale = std::min(scaleX, scaleY);

    const auto windows = QGuiApplication::topLevelWindows();
    for (QWindow* w : windows) {
        if (!w || w == mainWin || !w->isVisible()) {
            continue;
        }
        QQuickWindow* subWin = qobject_cast<QQuickWindow*>(w);
        if (!subWin) {
            continue;
        }
        if (QQuickItem* ci = subWin->contentItem()) {
            if (ci->width() < subWin->width() || ci->height() < subWin->height()) {
                ci->setWidth(subWin->width());
                ci->setHeight(subWin->height());
                QCoreApplication::processEvents();
            }
        }
        QImage subImg = subWin->grabWindow();
        if (subImg.isNull()) {
            continue;
        }

        // Dim background slightly to emphasize modal/dialog window
        p.fillRect(0, 0, 1920, 1080, QColor(8, 10, 16, 130));

        const int desiredW = std::min(1820, static_cast<int>(std::round(subWin->width() * uniformScale)));
        const int desiredH = std::min(940, static_cast<int>(std::round(subWin->height() * uniformScale)));
        QImage scaledSub = subImg.scaled(desiredW, desiredH, Qt::KeepAspectRatio, Qt::SmoothTransformation);
        const int targetW = scaledSub.width();
        const int targetH = scaledSub.height();
        const int titleBarH = 34;
        const int totalH = targetH + titleBarH;
        const int posX = (1920 - targetW) / 2;
        const int posY = std::max(24, (1080 - totalH) / 2 - 16);

        // Drop shadow
        for (int s = 18; s >= 2; s -= 4) {
            p.setPen(Qt::NoPen);
            p.setBrush(QColor(0, 0, 0, 22));
            p.drawRoundedRect(QRectF(posX - s, posY - s + 6, targetW + s * 2, totalH + s * 2), 12, 12);
        }

        // Window frame & titlebar
        QPainterPath framePath;
        framePath.addRoundedRect(QRectF(posX, posY, targetW, totalH), 9, 9);
        p.save();
        p.setClipPath(framePath);

        p.fillRect(QRect(posX, posY, targetW, titleBarH), QColor(30, 35, 48));
        p.setPen(QColor(56, 189, 248, 120));
        p.drawLine(posX, posY + titleBarH - 1, posX + targetW, posY + titleBarH - 1);

        QFont titleFont = p.font();
        titleFont.setPixelSize(14);
        titleFont.setBold(true);
        p.setFont(titleFont);
        p.setPen(QColor(238, 242, 248));
        p.drawText(
            QRect(posX + 14, posY, targetW - 28, titleBarH),
            Qt::AlignVCenter | Qt::AlignLeft,
            subWin->title()
        );

        p.drawImage(posX, posY + titleBarH, scaledSub);
        p.restore();

        // Outer border
        p.setPen(QPen(QColor(61, 174, 233, 180), 1.5));
        p.setBrush(Qt::NoBrush);
        p.drawRoundedRect(QRectF(posX, posY, targetW, totalH), 9, 9);
    }

    if (!bannerTitle.isEmpty()) {
        const int pillW = 1040;
        const int pillH = bannerSubtitle.isEmpty() ? 52 : 76;
        const int pillX = (1920 - pillW) / 2;
        const int pillY = 1080 - pillH - 36;

        p.setPen(Qt::NoPen);
        p.setBrush(QColor(0, 0, 0, 110));
        p.drawRoundedRect(QRectF(pillX - 4, pillY + 4, pillW + 8, pillH), 14, 14);

        p.setPen(QPen(QColor(56, 189, 248, 210), 2.0));
        p.setBrush(QColor(20, 24, 34, 238));
        p.drawRoundedRect(QRectF(pillX, pillY, pillW, pillH), 12, 12);

        QFont fTitle = p.font();
        fTitle.setPixelSize(20);
        fTitle.setBold(true);
        p.setFont(fTitle);
        p.setPen(QColor(56, 189, 248));
        p.drawText(
            QRect(pillX + 24, pillY + 8, pillW - 48, 28),
            Qt::AlignHCenter | Qt::AlignVCenter,
            bannerTitle
        );

        if (!bannerSubtitle.isEmpty()) {
            QFont fSub = p.font();
            fSub.setPixelSize(15);
            fSub.setBold(false);
            p.setFont(fSub);
            p.setPen(QColor(226, 232, 240));
            p.drawText(
                QRect(pillX + 24, pillY + 38, pillW - 48, 26),
                Qt::AlignHCenter | Qt::AlignVCenter,
                bannerSubtitle
            );
        }
    }

    p.end();

    QFileInfo fi(outputPath);
    if (!fi.dir().exists()) {
        QDir().mkpath(fi.dir().absolutePath());
    }
    return canvas.save(outputPath, "PNG");
}


