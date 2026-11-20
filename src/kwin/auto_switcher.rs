mod config;
mod icons;
mod kde_integration;
mod kwin_dbus;
mod linux_backends;
mod modifier_engine;
mod rules;
mod secondary_devices;
mod win32_backend;

#[cfg(test)]
#[path = "auto_switcher/tests.rs"]
mod modifier_engine_tests;

pub use config::*;
pub use icons::*;
pub use kde_integration::*;
pub use kwin_dbus::*;
pub use modifier_engine::*;
pub use rules::*;
pub use secondary_devices::*;
#[cfg(target_os = "windows")]
pub use win32_backend::*;
