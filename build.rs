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
                "src/qml/ColorPickerWindow.qml",
                "src/qml/WebApiBuilder.qml",
                "src/qml/AppLaunchBuilder.qml",
                "src/qml/EventLogDialog.qml",
                "src/qml/SettingsPage.qml",
                "src/qml/SdSyncDialog.qml",
                "src/qml/RestApiSettings.qml",
            ],
            qrc_files: &[
                "src/qml/icons/tray-icon-dark.svg",
                "src/qml/icons/tray-icon-light.svg",
                "src/qml/icons/oled-widget-icon.svg",
                "docs/user-guide.html",
            ],
        })
        .build();
    println!("cargo:rerun-if-changed=src/cpp/app_helper.h");
    println!("cargo:rerun-if-changed=src/paths.rs");
    println!("cargo:rerun-if-changed=src/bridge/device_controller.rs");
    println!("cargo:rerun-if-changed=docs/user-guide.html");
    println!("cargo:rerun-if-changed=src/qml/Main.qml");
    println!("cargo:rerun-if-changed=src/qml/UdevBanner.qml");
    println!("cargo:rerun-if-changed=src/qml/HardwareCanvas.qml");
    println!("cargo:rerun-if-changed=src/qml/PgvInspectorDrawer.qml");
    println!("cargo:rerun-if-changed=src/qml/AutoSwitchDrawer.qml");
    println!("cargo:rerun-if-changed=src/qml/AiArchitectModal.qml");
    println!("cargo:rerun-if-changed=src/qml/OledDesktopWidget.qml");
    println!("cargo:rerun-if-changed=src/qml/ColorPickerWindow.qml");
    println!("cargo:rerun-if-changed=src/qml/WebApiBuilder.qml");
    println!("cargo:rerun-if-changed=src/qml/AppLaunchBuilder.qml");
    println!("cargo:rerun-if-changed=src/qml/EventLogDialog.qml");
    println!("cargo:rerun-if-changed=src/qml/SettingsPage.qml");
    println!("cargo:rerun-if-changed=src/qml/SdSyncDialog.qml");
    println!("cargo:rerun-if-changed=src/qml/RestApiSettings.qml");
    println!("cargo:rustc-link-lib=KF6ColorScheme");
    println!("cargo:rustc-link-lib=KF6ConfigCore");
    println!("cargo:rustc-link-lib=KF6IconThemes");
}
