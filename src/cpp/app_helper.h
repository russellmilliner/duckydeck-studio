#pragma once

#include <KColorScheme>
#include <KColorSchemeManager>
#include <KIconLoader>
#include <KSharedConfig>
#include <QAbstractItemModel>
#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QCoreApplication>
#include <QCursor>
#include <QDateTime>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QFont>
#include <QGuiApplication>
#include <QIcon>
#include <QImage>
#include <QJsonDocument>
#include <QJsonObject>
#include <QLoggingCategory>
#include <QMenu>
#include <QMessageBox>
#include <QMetaObject>
#include <QModelIndex>
#include <QPainter>
#include <QPainterPath>
#include <QPixmap>
#include <QPointer>
#include <QProcess>
#include <QPushButton>
#include <QQmlApplicationEngine>
#include <QQuickWindow>
#include <QScreen>
#include <QString>
#include <QStringList>
#include <QSvgRenderer>
#include <QSystemTrayIcon>
#include <QThread>
#include <QTimer>
#include <QUrl>
#include <QVariant>
#include <QWindow>
#include <algorithm>
#include <cstdint>
#include <csignal>
#include <cstdlib>
#include <unistd.h>

namespace duckydeck {

inline QPointer<QWindow>& main_window_ref() {
    static QPointer<QWindow> win;
    return win;
}

inline QWindow* find_main_window();

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
    QString base = qEnvironmentVariable("XDG_CONFIG_HOME").trimmed();
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
    QFile f(path);
    if (f.open(QIODevice::WriteOnly | QIODevice::Truncate | QIODevice::Text)) {
        f.write(QJsonDocument(obj).toJson(QJsonDocument::Indented));
        f.close();
    }
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

    const int w = std::clamp(st.width, 184, std::max(184, fallbackAvail.width()));
    const int h = std::clamp(st.height, 240, std::max(240, fallbackAvail.height()));

    QScreen* matchedScreen = nullptr;
    if (st.hasPosition) {
        const QPoint centerPt(st.x + w / 2, st.y + h / 2);
        const auto screens = QGuiApplication::screens();
        for (QScreen* scr : screens) {
            if (!scr) {
                continue;
            }
            const QRect g = scr->geometry();
            if (g.contains(centerPt)) {
                matchedScreen = scr;
                break;
            }
        }
    }

    int targetX = st.x;
    int targetY = st.y;
    if (matchedScreen) {
        const QRect g = matchedScreen->geometry();
        const int maxX = std::max(g.left(), g.right() + 1 - w);
        const int maxY = std::max(g.top(), g.bottom() + 1 - h);
        targetX = std::clamp(st.x, g.left(), maxX);
        targetY = std::clamp(st.y, g.top(), maxY);
        if (outScreen) {
            *outScreen = matchedScreen;
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
    if (win->transientParent() != nullptr) {
        win->setTransientParent(nullptr);
    }
    if (win->property("_duckydeck_oled_hooked").toBool()) {
        return;
    }
    win->setProperty("_duckydeck_oled_hooked", true);
    win->setIcon(oled_widget_icon());

    const QRect initialTarget = resolve_oled_widget_target_rect(load_oled_widget_geometry(), nullptr);
    win->resize(initialTarget.width(), initialTarget.height());
    win->setPosition(initialTarget.x(), initialTarget.y());

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
            && (QDateTime::currentMSecsSinceEpoch() - oled_last_shown_ms_ref() < 1500)) {
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
        if (QGuiApplication::platformName() != QStringLiteral("wayland")) {
            st.x = win->x();
            st.y = win->y();
            st.hasPosition = true;
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

inline QRect restore_oled_widget_geometry_now() {
    oled_last_shown_ms_ref() = QDateTime::currentMSecsSinceEpoch();
    const OledWidgetGeometryState loaded = load_oled_widget_geometry();
    const QRect target = resolve_oled_widget_target_rect(loaded, nullptr);
    oled_expected_restore_rect_ref() = target;
    oled_restore_settled_ref() = false;

    OledWidgetGeometryState resolved = loaded;
    resolved.visible = true;
    resolved.hasPosition = true;
    resolved.x = target.x();
    resolved.y = target.y();
    resolved.width = target.width();
    resolved.height = target.height();
    save_oled_widget_geometry(resolved);

    if (QWindow* win = find_oled_widget_window()) {
        win->setTransientParent(nullptr);
        win->resize(target.width(), target.height());
        win->setPosition(target.x(), target.y());
    }
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
            && std::abs(x - expected.x()) <= 8
            && std::abs(y - expected.y()) <= 8) {
            oled_restore_settled_ref() = true;
        } else if (elapsed < 1500) {
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
    save_oled_widget_geometry(st);
}

inline void ensure_window_hooks(QWindow* win) {
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

inline QWindow* find_main_window() {
    QPointer<QWindow>& win = main_window_ref();
    if (win) {
        ensure_window_hooks(win.data());
        return win.data();
    }
    const auto windows = QGuiApplication::topLevelWindows();
    for (QWindow* w : windows) {
        if (w && w->objectName() == QStringLiteral("duckydeckMainWindow")) {
            win = w;
            ensure_window_hooks(w);
            return w;
        }
    }
    for (QWindow* w : windows) {
        if (w
            && w->inherits("QQuickWindow")
            && w->objectName() != QStringLiteral("duckydeckOledWidgetWindow")) {
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
        win->setTransientParent(nullptr);
        win->setIcon(oled_widget_icon());
        const QRect target = restore_oled_widget_geometry_now();
        win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
        win->resize(target.width(), target.height());
        win->setPosition(target.x(), target.y());
        win->showNormal();
        win->show();
        win->resize(target.width(), target.height());
        win->setPosition(target.x(), target.y());
        win->raise();
        win->requestActivate();
    }
}

inline void hide_oled_widget_window() {
    OledWidgetGeometryState st = load_oled_widget_geometry();
    st.visible = false;
    save_oled_widget_geometry(st);
    if (QWindow* win = find_oled_widget_window()) {
        win->hide();
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

inline void quit_duckydeck_application() {
    QCoreApplication::quit();
}

inline void restart_duckydeck_application() {
    QWindow* win = find_main_window();
    const bool wasHiddenToTray =
        !win || !win->isVisible() || (win->windowStates() & Qt::WindowMinimized);

    QStringList args;
    args << QStringLiteral("--replace");
    if (wasHiddenToTray) {
        args << QStringLiteral("--minimize") << QStringLiteral("--foreground");
    }

    const QString ipcPath = qEnvironmentVariable("DUCKYDECK_IPC_FILE");
    if (!ipcPath.isEmpty()) {
        QFile::remove(ipcPath);
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

        m_autostartAction = m_menu->addAction(QStringLiteral("Start on KDE Login (Autostart)"));
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
    QString base = qEnvironmentVariable("XDG_CONFIG_HOME").trimmed();
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

} // namespace duckydeck

template <typename ControllerT>
inline void duckydeck_sync_tray(
    ControllerT& controller,
    bool useLightOnDarkIcon,
    const QString& iconMode,
    const QString& statusHeader,
    const QString& profileHeader,
    const QString& sdSpaceHeader,
    const QString& tooltipText,
    bool autoSwitchEnabled,
    bool autostartEnabled
) {
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

inline void duckydeck_restart_app() {
    duckydeck::restart_duckydeck_application();
}

inline void duckydeck_apply_app_theme(const QString& themeMode) {
    if (!QCoreApplication::instance()) {
        return;
    }
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
}

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

inline QtMessageHandler& prev_qt_msg_handler() {
    static QtMessageHandler h = nullptr;
    return h;
}

inline void duckydeck_qt_msg_handler(QtMsgType type, const QMessageLogContext& ctx, const QString& msg) {
    if (msg.contains(QStringLiteral("org/kde/breeze/TextArea.qml"))
        && msg.contains(QStringLiteral("QQuickTextInput"))) {
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
        qputenv("QT_QUICK_CONTROLS_STYLE", "org.kde.breeze");
    }
    if (std::getenv("DUCKYDECK_DEMO_SHOWCASE") && std::string(std::getenv("DUCKYDECK_DEMO_SHOWCASE")) == "1") {
        qputenv("QT_QPA_PLATFORM", "xcb");
        qputenv("QT_SCALE_FACTOR", "1");
        qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", "0");
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

    QIcon initialIcon = duckydeck::build_multisize_svg_icon(
        QStringLiteral(":/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg")
    );
    QGuiApplication::setWindowIcon(initialIcon);

    // Single-instance check: offer to close previous running instance and start new one
    const QString existingPidsRaw = qEnvironmentVariable("DUCKYDECK_EXISTING_PIDS").trimmed();
    if (!existingPidsRaw.isEmpty()) {
        QList<pid_t> alivePids;
        QStringList aliveStrs;
        const QStringList parts = existingPidsRaw.split(QLatin1Char(','), Qt::SkipEmptyParts);
        for (const QString& part : parts) {
            bool ok = false;
            const int pidVal = part.trimmed().toInt(&ok);
            if (ok && pidVal > 1 && pidVal != static_cast<int>(::getpid()) && ::kill(static_cast<pid_t>(pidVal), 0) == 0) {
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

    const QString pidFilePath = qEnvironmentVariable("DUCKYDECK_PID_FILE").trimmed();
    if (!pidFilePath.isEmpty()) {
        QFile pidFile(pidFilePath);
        if (pidFile.open(QIODevice::WriteOnly | QIODevice::Truncate | QIODevice::Text)) {
            pidFile.write(QByteArray::number(static_cast<qlonglong>(::getpid())) + "\n");
            pidFile.close();
        }
    }

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/Main.qml")));

    if (!engine.rootObjects().isEmpty()) {
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
                qEnvironmentVariable("DUCKYDECK_SHOW_OLED_WIDGET") == QStringLiteral("1")
                || (qEnvironmentVariable("DUCKYDECK_DEMO_SHOWCASE") != QStringLiteral("1")
                    && duckydeck::load_oled_widget_geometry().visible);
            if (shouldRestoreOledWidget) {
                QTimer::singleShot(180, win, []() {
                    duckydeck::show_oled_widget_window();
                });
            }
        }
    }

    const QString ipcFilePath = qEnvironmentVariable("DUCKYDECK_IPC_FILE").trimmed();
    QTimer ipcTimer;
    if (!ipcFilePath.isEmpty()) {
        QFile staleIpc(ipcFilePath);
        if (staleIpc.exists()) {
            staleIpc.remove();
        }
        ipcTimer.setInterval(180);
        QObject::connect(&ipcTimer, &QTimer::timeout, [ipcFilePath]() {
            QFile f(ipcFilePath);
            if (!f.exists()) {
                return;
            }
            if (f.open(QIODevice::ReadOnly | QIODevice::Text)) {
                const QString payload = QString::fromUtf8(f.readAll());
                f.close();
                f.remove();
                const QStringList lines = payload.split(QLatin1Char('\n'), Qt::SkipEmptyParts);
                for (const QString& line : lines) {
                    duckydeck::TrayManager::instance().dispatchIpcCommand(line);
                }
            }
        });
        ipcTimer.start();
    }

    const int rc = app.exec();

    if (!ipcFilePath.isEmpty()) {
        QFile ipcFile(ipcFilePath);
        if (ipcFile.exists()) {
            ipcFile.remove();
        }
    }
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

    QImage scaledMain = mainImg.scaled(1920, 1080, Qt::IgnoreAspectRatio, Qt::SmoothTransformation);
    p.drawImage(0, 0, scaledMain);

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
        QImage subImg = subWin->grabWindow();
        if (subImg.isNull()) {
            continue;
        }

        // Dim background slightly to emphasize modal/dialog window
        p.fillRect(0, 0, 1920, 1080, QColor(8, 10, 16, 130));

        const int targetW = std::min(1820, static_cast<int>(subWin->width() * uniformScale));
        const int targetH = std::min(980, static_cast<int>(subWin->height() * uniformScale));
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

        QImage scaledSub = subImg.scaled(targetW, targetH, Qt::IgnoreAspectRatio, Qt::SmoothTransformation);
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


