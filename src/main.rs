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
pub mod vm;

use std::env;

fn main() {
    let start_minimized = env::args().any(|arg| arg == "--minimized" || arg == "--tray");
    let exit_code = bridge::device_controller::run_application(start_minimized);
    std::process::exit(exit_code);
}

