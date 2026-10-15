#pragma once

#include <KColorSchemeManager>
#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QCoreApplication>
#include <QGuiApplication>
#include <QIcon>
#include <QMenu>
#include <QMetaObject>
#include <QModelIndex>
#include <QPainter>
#include <QPixmap>
#include <QPointer>
#include <QQmlApplicationEngine>
#include <QString>
#include <QSvgRenderer>
#include <QSystemTrayIcon>
#include <QTimer>
#include <QUrl>
#include <QVariant>
#include <QWindow>
#include <cstdlib>

namespace duckydeck {

inline QPointer<QWindow>& main_window_ref() {
    static QPointer<QWindow> win;
    return win;
}

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

inline void ensure_window_hooks(QWindow* win) {
    if (!win || win->property("_duckydeck_minimize_hooked").toBool()) {
        return;
    }
    win->setProperty("_duckydeck_minimize_hooked", true);

    QObject::connect(win, &QWindow::windowStateChanged, win, [win](Qt::WindowState state) {
        if (state & Qt::WindowMinimized) {
            QTimer::singleShot(0, win, [win]() {
                if (win) {
                    win->hide();
                    win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
                }
            });
        }
    });

    QObject::connect(win, &QWindow::visibilityChanged, win, [win](QWindow::Visibility visibility) {
        if (visibility == QWindow::Minimized) {
            QTimer::singleShot(0, win, [win]() {
                if (win) {
                    win->hide();
                    win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
                }
            });
        }
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
        if (w && w->inherits("QQuickWindow")) {
            win = w;
            ensure_window_hooks(w);
            return w;
        }
    }
    return nullptr;
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
}

inline void hide_main_window_to_tray() {
    if (QWindow* win = find_main_window()) {
        win->hide();
        if (win->windowStates() & Qt::WindowMinimized) {
            win->setWindowStates(win->windowStates() & ~Qt::WindowMinimized);
        }
    }
}

inline void quit_duckydeck_application() {
    QCoreApplication::quit();
}

class TrayManager {
public:
    static TrayManager& instance() {
        static TrayManager inst;
        return inst;
    }

    void sync(
        QObject* controller,
        bool useLightOnDarkIcon,
        const QString& iconMode,
        const QString& statusHeader,
        const QString& profileHeader,
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
        } else {
            (void)find_main_window();
        }

        m_tray->setToolTip(tooltipText);

        if (m_statusAction) {
            m_statusAction->setText(statusHeader);
        }
        if (m_profileAction) {
            m_profileAction->setText(profileHeader);
        }
        if (m_autoSwitchAction) {
            const bool oldBlock = m_autoSwitchAction->blockSignals(true);
            m_autoSwitchAction->setChecked(autoSwitchEnabled);
            m_autoSwitchAction->setText(
                autoSwitchEnabled
                    ? QStringLiteral("KWin Auto-Switch: Enabled")
                    : QStringLiteral("KWin Auto-Switch: Disabled")
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

        if (!m_tray->isVisible()) {
            m_tray->show();
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

        m_statusAction = m_menu->addAction(QStringLiteral("DuckyDeck Studio"));
        QObject::connect(m_statusAction, &QAction::triggered, []() {
            show_main_window();
        });

        m_profileAction = m_menu->addAction(QStringLiteral("Active Profile: #1"));
        QObject::connect(m_profileAction, &QAction::triggered, []() {
            show_main_window();
        });

        m_menu->addSeparator();

        m_autoSwitchAction = m_menu->addAction(QStringLiteral("KWin Auto-Switch: Enabled"));
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

        QAction* showAction = m_menu->addAction(QStringLiteral("Show DuckyDeck Studio"));
        QObject::connect(showAction, &QAction::triggered, []() {
            show_main_window();
        });

        QAction* hideAction = m_menu->addAction(QStringLiteral("Minimize to Taskbar"));
        QObject::connect(hideAction, &QAction::triggered, []() {
            hide_main_window_to_tray();
        });

        m_menu->addSeparator();

        QAction* quitAction = m_menu->addAction(QStringLiteral("Quit DuckyDeck Studio"));
        QObject::connect(quitAction, &QAction::triggered, []() {
            quit_duckydeck_application();
        });

        m_tray->setContextMenu(m_menu);

        QObject::connect(
            m_tray,
            &QSystemTrayIcon::activated,
            [](QSystemTrayIcon::ActivationReason reason) {
                if (reason == QSystemTrayIcon::Trigger || reason == QSystemTrayIcon::DoubleClick) {
                    QWindow* win = find_main_window();
                    if (!win) {
                        show_main_window();
                        return;
                    }
                    if (win->isVisible() && !(win->windowStates() & Qt::WindowMinimized)) {
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
    QAction* m_autoSwitchAction = nullptr;
    QAction* m_autostartAction = nullptr;
    QAction* m_iconAutoAction = nullptr;
    QAction* m_iconDarkAction = nullptr;
    QAction* m_iconLightAction = nullptr;
    QString m_lastIconPath;
};

} // namespace duckydeck

template <typename ControllerT>
inline void duckydeck_sync_tray(
    ControllerT& controller,
    bool useLightOnDarkIcon,
    const QString& iconMode,
    const QString& statusHeader,
    const QString& profileHeader,
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
        tooltipText,
        autoSwitchEnabled,
        autostartEnabled
    );
}

inline void duckydeck_show_main_window() {
    duckydeck::show_main_window();
}

inline void duckydeck_hide_main_window() {
    duckydeck::hide_main_window_to_tray();
}

inline void duckydeck_quit_app() {
    duckydeck::quit_duckydeck_application();
}

inline void duckydeck_apply_app_theme(const QString& themeMode) {
    if (!QCoreApplication::instance()) {
        return;
    }
    KColorSchemeManager* mgr = KColorSchemeManager::instance();
    if (!mgr) {
        return;
    }
    mgr->setAutosaveChanges(false);
    if (themeMode == QStringLiteral("dark")) {
        QModelIndex idx = mgr->indexForSchemeId(QStringLiteral("BreezeDark"));
        if (!idx.isValid()) {
            idx = mgr->indexForScheme(QStringLiteral("Breeze Dark"));
        }
        if (idx.isValid()) {
            mgr->activateScheme(idx);
        }
    } else if (themeMode == QStringLiteral("light")) {
        QModelIndex idx = mgr->indexForSchemeId(QStringLiteral("BreezeLight"));
        if (!idx.isValid()) {
            idx = mgr->indexForScheme(QStringLiteral("Breeze Light"));
        }
        if (idx.isValid()) {
            mgr->activateScheme(idx);
        }
    } else {
        mgr->activateScheme(QModelIndex());
    }
}

inline int duckydeck_run_app(bool start_minimized) {
    if (!std::getenv("QT_QUICK_CONTROLS_STYLE")) {
        qputenv("QT_QUICK_CONTROLS_STYLE", "org.kde.desktop");
    }

    static int argc = 1;
    static char arg0[] = "duckydeck-studio";
    static char* argv[] = {arg0, nullptr};

    QApplication app(argc, argv);
    QGuiApplication::setApplicationName(QStringLiteral("DuckyDeck Studio"));
    QGuiApplication::setApplicationDisplayName(QStringLiteral("DuckyDeck Studio"));
    QGuiApplication::setDesktopFileName(QStringLiteral("io.github.duckydeck.Studio"));
    QGuiApplication::setQuitOnLastWindowClosed(false);

    QIcon initialIcon = duckydeck::build_multisize_svg_icon(
        QStringLiteral(":/qt/qml/io/github/duckydeck/src/qml/icons/tray-icon-dark.svg")
    );
    QGuiApplication::setWindowIcon(initialIcon);

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/qt/qml/io/github/duckydeck/src/qml/Main.qml")));

    if (!engine.rootObjects().isEmpty()) {
        QWindow* win = qobject_cast<QWindow*>(engine.rootObjects().first());
        if (win) {
            duckydeck::main_window_ref() = win;
            duckydeck::ensure_window_hooks(win);
            win->setIcon(initialIcon);
            if (start_minimized) {
                win->hide();
            }
        }
    }

    return app.exec();
}

