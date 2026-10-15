use std::fs::{self, OpenOptions};
use std::path::Path;

const VENDOR_ID: &str = "00000483";
const PID_PRO: &str = "0000D11D";
const PID_ORIG: &str = "0000D11C";

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceAccessState {
    NotConnected,
    ConnectedAndAccessible,
    UdevRuleMissing, // Device is plugged in, but /dev/hidraw* returned PermissionDenied
}

pub fn check_duckypad_udev_state() -> DeviceAccessState {
    let Ok(entries) = fs::read_dir("/sys/class/hidraw") else {
        return DeviceAccessState::NotConnected;
    };

    for entry in entries.flatten() {
        let uevent_path = entry.path().join("device/uevent");
        if let Ok(uevent) = fs::read_to_string(&uevent_path) {
            let is_duckypad = uevent.contains(&format!("HID_ID=0003:{VENDOR_ID}:{PID_PRO}"))
                || uevent.contains(&format!("HID_ID=0003:{VENDOR_ID}:{PID_ORIG}"));

            if is_duckypad {
                let dev_node = Path::new("/dev").join(entry.file_name());
                match OpenOptions::new().read(true).write(true).open(&dev_node) {
                    Ok(_) => return DeviceAccessState::ConnectedAndAccessible,
                    Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                        return DeviceAccessState::UdevRuleMissing;
                    }
                    Err(_) => continue,
                }
            }
        }
    }
    DeviceAccessState::NotConnected
}

/// Launches a native KDE Polkit password dialog on the host to install 70-duckypad.rules
pub fn install_udev_via_pkexec() -> Result<(), String> {
    let script = r#"cat << 'EOF' > /etc/udev/rules.d/70-duckypad.rules
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11d", MODE="0660", TAG+="uaccess"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11d", MODE="0660", TAG+="uaccess"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11c", MODE="0660", TAG+="uaccess"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0483", ATTRS{idProduct}=="d11c", MODE="0660", TAG+="uaccess"
EOF
udevadm control --reload-rules && udevadm trigger --subsystem-match=hidraw"#;

    let is_flatpak = Path::new("/.flatpak-info").exists();
    let status = if is_flatpak {
        std::process::Command::new("flatpak-spawn")
            .args(["--host", "pkexec", "sh", "-c", script])
            .status()
    } else {
        std::process::Command::new("pkexec")
            .args(["sh", "-c", script])
            .status()
    };

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("Polkit exited with status: {s}")),
        Err(e) => Err(format!("Failed to invoke pkexec: {e}")),
    }
}
