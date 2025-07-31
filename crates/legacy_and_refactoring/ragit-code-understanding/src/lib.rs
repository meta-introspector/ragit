use anyhow::Result;
use ragit_memory_monitor::MemoryMonitor;
use tokei::{Config, LanguageType, Languages};

pub fn log_code_complexity(
    code: &str,
    label: &str,
    memory_monitor: &mut MemoryMonitor,
) -> Result<(), anyhow::Error> {
    panic!("FIX ME LATER: Code understanding is parked. Fix the bootstrap first and this code later.");
    // let mut languages = Languages::new();
    // let config = Config::default();
    // languages.add_code(code.as_bytes(), LanguageType::Rust, &config);

    // if let Some(rust_lang) = languages.get(&LanguageType::Rust) {
    //     memory_monitor.verbose(&format!(
    //         "Code Complexity ({}): Lines = {}, Code = {}, Comments = {}, Blanks = {}, Functions = {}, Complexity = {}",
    //         label,
    //         rust_lang.lines(),
    //         rust_lang.code(),
    //         rust_lang.comments(),
    //         rust_lang.blanks(),
    //         rust_lang.functions(),
    //         rust_lang.complexity()
    //     ));
    // } else {
    //     memory_monitor.verbose(&format!("Could not analyze Rust code complexity for {}.", label));
    // }
    // Ok(())
}
