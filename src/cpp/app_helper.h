#pragma once

#include <KColorSchemeManager>
#include <QAction>
#include <QActionGroup>
#include <QApplication>
#include <QCoreApplication>
#include <QCursor>
#include <QDir>
#include <QFileInfo>
#include <QFont>
#include <QGuiApplication>
#include <QIcon>
#include <QImage>
#include <QLoggingCategory>
#include <QMenu>
#include <QMetaObject>
#include <QModelIndex>
#include <QPainter>
#include <QPainterPath>
#include <QPixmap>
#include <QPointer>
#include <QQmlApplicationEngine>
#include <QQuickWindow>
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
    QLoggingCategory::setFilterRules(QStringLiteral("union.*=false\nqt.core.qobject.connect=false"));
    if (!std::getenv("QT_QUICK_CONTROLS_STYLE")) {
        qputenv("QT_QUICK_CONTROLS_STYLE", "org.kde.desktop");
    }
    if (std::getenv("DUCKYDECK_DEMO_SHOWCASE") && std::string(std::getenv("DUCKYDECK_DEMO_SHOWCASE")) == "1") {
        qputenv("QT_QPA_PLATFORM", "xcb");
        qputenv("QT_SCALE_FACTOR", "1");
        qputenv("QT_AUTO_SCREEN_SCALE_FACTOR", "0");
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
            if (qEnvironmentVariable("DUCKYDECK_DEMO_SHOWCASE") == QStringLiteral("1")) {
                win->resize(1920, 1080);
            }
            if (start_minimized) {
                win->hide();
            }
        }
    }

    return app.exec();
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


