use super::super::constants::{LOG_RUNNING_FINAL_REFLECTIVE_QUERY, LOG_BEFORE_FINAL_REFLECTIVE_QUERY};
use ragit_memory_monitor::MemoryMonitor;

pub fn log_start(_verbose: bool, memory_monitor: &mut MemoryMonitor) {
    memory_monitor.verbose(LOG_RUNNING_FINAL_REFLECTIVE_QUERY);
    memory_monitor.verbose(LOG_BEFORE_FINAL_REFLECTIVE_QUERY);
    memory_monitor.capture_and_log_snapshot("Before final reflective query");
}
