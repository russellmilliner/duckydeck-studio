use cxx_qt_build::{CxxQtBuilder, QmlModule};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if std::env::var_os("CARGO_CFG_FUZZING").is_some() {
        return;
    }
    let is_windows = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows");

    let mut qrc_files = vec![
        "src/qml/icons/tray-icon-dark.svg",
        "src/qml/icons/tray-icon-light.svg",
        "src/qml/icons/oled-widget-icon.svg",
        "src/qml/icons/duckydeck-studio.svg",
        "docs/user-guide.html",
    ];
    if is_windows {
        qrc_files.extend([
            "src/qml/kirigami/ApplicationWindow.qml",
            "src/qml/kirigami/Page.qml",
            "src/qml/kirigami/Heading.qml",
            "src/qml/kirigami/Action.qml",
            "src/qml/kirigami/Icon.qml",
            "src/qml/kirigami/InlineMessage.qml",
            "src/qml/kirigami/SearchField.qml",
        ]);
    }

    let moc_cpp_opt: Option<PathBuf> = if is_windows {
        let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
        let moc_out = out_dir.join("moc_kirigami_compat.cpp");
        let qmake_cmd = std::env::var("QMAKE").unwrap_or_else(|_| "qmake6".to_string());
        let qt_headers = Command::new(&qmake_cmd)
            .args(["-query", "QT_INSTALL_HEADERS"])
            .output()
            .ok()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();
        let moc_bin = std::env::var("QT_MOC_EXECUTABLE").unwrap_or_else(|_| {
            if let Ok(out) = Command::new(&qmake_cmd).args(["-query", "QT_HOST_LIBEXECS"]).output() {
                let dir = String::from_utf8_lossy(&out.stdout).trim().to_string();
                let cand = PathBuf::from(&dir).join("moc");
                if cand.exists() {
                    return cand.to_string_lossy().to_string();
                }
                let cand_exe = PathBuf::from(&dir).join("moc.exe");
                if cand_exe.exists() {
                    return cand_exe.to_string_lossy().to_string();
                }
            }
            "moc".to_string()
        });
        let mut cmd = Command::new(&moc_bin);
        if !qt_headers.is_empty() {
            cmd.arg(format!("-I{qt_headers}"));
            for sub in ["QtCore", "QtGui", "QtQml", "QtQuick"] {
                cmd.arg(format!("-I{qt_headers}/{sub}"));
            }
        }
        let status = cmd
            .arg("src/cpp/kirigami_compat.h")
            .arg("-o")
            .arg(&moc_out)
            .status()
            .expect("Failed to run Qt moc on src/cpp/kirigami_compat.h");
        assert!(status.success(), "Qt moc failed on src/cpp/kirigami_compat.h");
        Some(moc_out)
    } else {
        None
    };

    CxxQtBuilder::new()
        .qt_module("Network")
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .qt_module("Widgets")
        .qt_module("Svg")
        .cc_builder(move |cc| {
            cc.include("src/cpp");
            if is_windows {
                // Ensure bigobj / relaxed sections for large generated QML cache files on MinGW
                cc.flag_if_supported("-Wa,-mbig-obj");
                if let Some(ref moc_cpp) = moc_cpp_opt {
                    cc.file(moc_cpp);
                }
            } else {
                cc.include("/usr/include/KF6/KColorScheme");
                cc.include("/usr/include/KF6/KConfig");
                cc.include("/usr/include/KF6/KConfigCore");
                cc.include("/usr/include/KF6/KIconThemes");
            }
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
                "src/qml/ProfileWizardDialog.qml",
            ],
            qrc_files: &qrc_files,
        })
        .build();
    println!("cargo:rerun-if-changed=src/qml/icons/duckydeck-studio.svg");
    println!("cargo:rerun-if-changed=src/cpp/app_helper.h");
    println!("cargo:rerun-if-changed=src/cpp/kirigami_compat.h");
    println!("cargo:rerun-if-changed=src/qml/kirigami/ApplicationWindow.qml");
    println!("cargo:rerun-if-changed=src/qml/kirigami/Page.qml");
    println!("cargo:rerun-if-changed=src/qml/kirigami/Heading.qml");
    println!("cargo:rerun-if-changed=src/qml/kirigami/Action.qml");
    println!("cargo:rerun-if-changed=src/qml/kirigami/Icon.qml");
    println!("cargo:rerun-if-changed=src/qml/kirigami/InlineMessage.qml");
    println!("cargo:rerun-if-changed=src/qml/kirigami/SearchField.qml");
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
    println!("cargo:rerun-if-changed=src/qml/SdSyncDialog.qml",);
    println!("cargo:rerun-if-changed=src/qml/RestApiSettings.qml");
    println!("cargo:rerun-if-changed=src/qml/ProfileWizardDialog.qml");
    if is_windows {
        for lib in [
            "Qt6Widgets",
            "Qt6QuickControls2",
            "Qt6Quick",
            "Qt6QmlModels",
            "Qt6Qml",
            "Qt6Svg",
            "Qt6Gui",
            "Qt6Network",
            "Qt6Core",
            "stdc++",
            "mingw32",
            "gcc_s",
            "gcc",
            "msvcrt",
            "kernel32",
            "user32",
            "shell32",
            "advapi32",
        ] {
            println!("cargo:rustc-link-arg=-l{lib}");
        }
    } else {
        println!("cargo:rustc-link-lib=KF6ColorScheme");
        println!("cargo:rustc-link-lib=KF6ConfigCore");
        println!("cargo:rustc-link-lib=KF6IconThemes");
    }
}
