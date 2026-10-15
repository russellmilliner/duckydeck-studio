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
            ],
            qrc_files: &[
                "src/qml/icons/tray-icon-dark.svg",
                "src/qml/icons/tray-icon-light.svg",
            ],
            ..Default::default()
        })
        .build();
    println!("cargo:rustc-link-lib=KF6ColorScheme");
}
