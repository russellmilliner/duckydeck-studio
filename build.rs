use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Network")
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .qt_module("Widgets")
        .qt_module("Svg")
        .cc_builder(|cc| {
            cc.include("src/cpp");
            cc.include("/usr/include/KF6/KColorScheme");
            cc.include("/usr/include/KF6/KConfig");
            cc.include("/usr/include/KF6/KConfigCore");
            cc.include("/usr/include/KF6/KIconThemes");
        })
        .qml_module(QmlModule {
            uri: "io.github.duckydeck",
            version_major: 1,
            version_minor: 0,
            rust_files: &["src/bridge/device_controller.rs"],
            qml_files: &[
                "src/qml/Main.qml",
                "src/qml/UdevBanner.qml",
                "src/qml/HardwareCanvas.qml",
                "src/qml/PgvInspectorDrawer.qml",
                "src/qml/AutoSwitchDrawer.qml",
                "src/qml/AiArchitectModal.qml",
                "src/qml/OledDesktopWidget.qml",
            ],
            qrc_files: &[
                "src/qml/icons/tray-icon-dark.svg",
                "src/qml/icons/tray-icon-light.svg",
                "src/qml/icons/oled-widget-icon.svg",
            ],
            ..Default::default()
        })
        .build();
    println!("cargo:rerun-if-changed=src/cpp/app_helper.h");
    println!("cargo:rustc-link-lib=KF6ColorScheme");
    println!("cargo:rustc-link-lib=KF6ConfigCore");
    println!("cargo:rustc-link-lib=KF6IconThemes");
}
