pub mod capture_memory_snapshot;
pub mod column;
pub mod format_bytes;
pub mod format_signed_bytes;
pub mod memory_snapshot;
pub mod print_memory_table;
pub mod table;

use sysinfo::{Pid, System};
use anyhow::{anyhow, Result};
use crate::memory_snapshot::MemorySnapshot;
use crate::print_memory_table::print_memory_table;
use crate::capture_memory_snapshot::capture_memory_snapshot;

#[derive(Debug)]
pub struct MemoryMonitor {
    sys: System,
    last_snapshot_data: Option<(u64, u64, u64)>,
    snapshots: Vec<MemorySnapshot>,
    last_snapshot_time: Option<std::time::Instant>,
    units_processed_since_last_snapshot: u64,
    verbose: bool,
    time_threshold_ms: Option<u128>,
    memory_threshold_bytes: Option<u64>,
}

impl Clone for MemoryMonitor {
    fn clone(&self) -> Self {
        MemoryMonitor {
            sys: System::new_all(),
            last_snapshot_data: self.last_snapshot_data.clone(),
            snapshots: self.snapshots.clone(),
            last_snapshot_time: self.last_snapshot_time.clone(),
            units_processed_since_last_snapshot: self.units_processed_since_last_snapshot,
            verbose: self.verbose,
            time_threshold_ms: self.time_threshold_ms,
            memory_threshold_bytes: self.memory_threshold_bytes,
        }
    }
}

impl MemoryMonitor {
    pub fn new(verbose: bool, time_threshold_ms: Option<u128>, memory_threshold_bytes: Option<u64>) -> Self {
        MemoryMonitor {
            sys: System::new_all(),
            last_snapshot_data: None,
            snapshots: Vec::new(),
            last_snapshot_time: None,
            units_processed_since_last_snapshot: 0,
            verbose,
            time_threshold_ms,
            memory_threshold_bytes,
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    pub fn verbose(&self, message: &str) {
        if self.verbose {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let truncated_message = if message.len() > 300 {
                format!("{}... (truncated)", &message[..300])
            } else {
                message.to_string()
            };
            println!("[{timestamp}] [INFO] {truncated_message}");
        }
    }

    pub fn process_unit(&mut self) {
        self.units_processed_since_last_snapshot += 1;
    }

    pub fn process_units(&mut self, n: u64) {
        self.units_processed_since_last_snapshot += n;
    }

    pub fn capture_and_log_snapshot(&mut self, step_name: &str) {
        let current_time = std::time::Instant::now();
        self.sys.refresh_processes();
        let mut current_process_memory_kb = 0;
        if let Some(process) = self.sys.process(sysinfo::Pid::from_u32(std::process::id())) {
            current_process_memory_kb = process.memory() / 1024;
        }

        if let Some(last_time) = self.last_snapshot_time {
            let elapsed_time = current_time.duration_since(last_time).as_millis();
            if let Some(threshold) = self.time_threshold_ms {
                if elapsed_time > threshold {
                    self.verbose(&format!("PERFORMANCE ALERT: Step '{step_name}' took {elapsed_time}ms, exceeding threshold of {threshold}ms."));
                }
            }
        }

        if let Some((_, _, last_rss)) = self.last_snapshot_data {
            let memory_diff = current_process_memory_kb as i64 - last_rss as i64;
            if let Some(threshold) = self.memory_threshold_bytes {
                if memory_diff > threshold as i64 / 1024 { // Convert threshold to KB
                    self.verbose(&format!("MEMORY ALERT: Step '{}' changed RSS by {}KB, exceeding threshold of {}KB.", step_name, memory_diff, threshold / 1024));
                }
            }
        }

        capture_memory_snapshot(
            step_name,
            &mut self.sys,
            &mut self.last_snapshot_data,
            &mut self.snapshots,
            &mut self.last_snapshot_time,
            self.units_processed_since_last_snapshot,
        );
        self.units_processed_since_last_snapshot = 0;
    }

    pub fn print_final_report(&self) {
        print_memory_table(self.snapshots.clone());
    }

    pub fn check_memory_limit(&mut self, max_memory_gb: Option<u64>, message: &str) -> Result<()> {
        if let Some(limit_gb) = max_memory_gb {
            self.sys.refresh_memory();
            if let Some(process) = self.sys.process(Pid::from_u32(std::process::id())) {
                let current_process_memory_kb = process.memory() / 1024;
                let limit_kb = limit_gb * 1024 * 1024; // Convert GB to KB

                if current_process_memory_kb > limit_kb {
                    return Err(anyhow!(
                        r"Memory limit exceeded at {}: Process RSS {} KB > Limit {} KB ({} GB)",
                        message,
                        current_process_memory_kb,
                        limit_kb,
                        limit_gb
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn start_tracking(&mut self, step_name: &str) {
        self.capture_and_log_snapshot(&format!("START: {}", step_name));
    }

    pub fn stop_tracking(&mut self, step_name: &str) {
        self.capture_and_log_snapshot(&format!("END: {}", step_name));
    }

    pub fn print_summary(&self) {
        self.print_final_report();
    }
}
