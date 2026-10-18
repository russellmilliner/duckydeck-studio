#![no_main]

use duckydeck_core::vm::compiler::compile_duckyscript_to_dsb;
use duckydeck_core::{extract_app_launch_config, extract_web_api_config};
use libfuzzer_sys::fuzz_target;

const FLAG_COMBOS: [(bool, bool); 4] = [
    (false, false),
    (true, false),
    (false, true),
    (true, true),
];

fn fuzz_script_str(script: &str, slot_num: usize) {
    for (allow_abort, disable_autorepeat) in FLAG_COMBOS {
        let _ = compile_duckyscript_to_dsb(script, allow_abort, disable_autorepeat);
    }
    let _ = extract_web_api_config(script, slot_num);
    let _ = extract_app_launch_config(script, slot_num);
}

fuzz_target!(|data: &[u8]| {
    let slot_num = data.first().map_or(1, |b| (*b as usize % 36) + 1);
    if let Ok(valid_utf8) = std::str::from_utf8(data) {
        fuzz_script_str(valid_utf8, slot_num);
    } else {
        let lossy = String::from_utf8_lossy(data);
        fuzz_script_str(&lossy, slot_num);
    }
});
