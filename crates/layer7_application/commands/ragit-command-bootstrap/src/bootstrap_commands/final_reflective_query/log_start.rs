use super::super::constants::{LOG_RUNNING_FINAL_REFLECTIVE_QUERY, LOG_BEFORE_FINAL_REFLECTIVE_QUERY};
use ragit_memory_monitor::MemoryMonitor;

pub fn log_start(verbose: bool, memory_monitor: &mut MemoryMonitor) {
    if verbose {
        println!("{}", LOG_RUNNING_FINAL_REFLECTIVE_QUERY);
        println!("{}", LOG_BEFORE_FINAL_REFLECTIVE_QUERY);
        memory_monitor.capture_and_log_snapshot("Before final reflective query");
    }
}
