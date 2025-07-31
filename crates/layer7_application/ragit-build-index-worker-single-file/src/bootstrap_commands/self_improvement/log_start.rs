use ragit_memory_monitor::MemoryMonitor;

pub fn log_start(_verbose: bool, memory_monitor: &mut MemoryMonitor) {
    memory_monitor.verbose("log_start (self_improvement): Running self-improvement query.");
    memory_monitor.capture_and_log_snapshot("Before self-improvement query");
}
