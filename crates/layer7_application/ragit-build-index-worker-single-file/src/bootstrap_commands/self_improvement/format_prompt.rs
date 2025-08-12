use super::super::constants::SELF_IMPROVEMENT_PROMPT_PREFIX;

use ragit_memory_monitor::MemoryMonitor;
pub fn format_prompt(self_code: &str, memory_monitor: &mut MemoryMonitor) -> String {
    memory_monitor.verbose("format_prompt: Starting to format prompt.");
    let formatted_prompt = format!(
        "{}",
        SELF_IMPROVEMENT_PROMPT_PREFIX.replace("{}", self_code)
    );
    memory_monitor.verbose("format_prompt: Finished formatting prompt.");
    formatted_prompt
}