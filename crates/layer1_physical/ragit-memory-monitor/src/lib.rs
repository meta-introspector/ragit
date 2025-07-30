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

pub struct MemoryMonitor {
    sys: System,
    last_snapshot_data: Option<(u64, u64, u64)>,
    snapshots: Vec<MemorySnapshot>,
    last_snapshot_time: Option<std::time::Instant>,
    units_processed_since_last_snapshot: u64,
    verbose: bool,
}

impl MemoryMonitor {
    pub fn new(verbose: bool) -> Self {
        MemoryMonitor {
            sys: System::new_all(),
            last_snapshot_data: None,
            snapshots: Vec::new(),
            last_snapshot_time: None,
            units_processed_since_last_snapshot: 0,
            verbose,
        }
    }

    pub fn verbose(&self, message: &str) {
        if self.verbose {
            println!("{}", message);
        }
    }

    pub fn process_unit(&mut self) {
        self.units_processed_since_last_snapshot += 1;
    }

    pub fn process_units(&mut self, n: u64) {
        self.units_processed_since_last_snapshot += n;
    }

    pub fn capture_and_log_snapshot(&mut self, step_name: &str) {
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
}
