use ragit_memory_monitor::MemoryMonitor;

pub fn log_start(verbose: bool, memory_monitor: &mut MemoryMonitor) {
    if verbose {
        println!("bootstrap_index_self: Running self-improvement query");
        memory_monitor.capture_and_log_snapshot("Before self-improvement query");
    }
}
