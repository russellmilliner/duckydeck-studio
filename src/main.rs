pub mod bridge {
    pub mod device_controller;
}
pub mod hardware {
    pub mod permissions;
    pub mod storage;
}
pub mod kwin {
    pub mod auto_switcher;
}
pub mod ai {
    pub mod shortcut_scraper;
}

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::env;

fn main() {
    // Default to KDE Plasma 6 Breeze styling if not already set by the environment
    if env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
        env::set_var("QT_QUICK_CONTROLS_STYLE", "org.kde.desktop");
    }

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from(
            "qrc:/qt/qml/io/github/duckydeck/src/qml/Main.qml",
        ));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
