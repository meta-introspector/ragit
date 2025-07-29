use sysinfo::System;
use super::memory_snapshot::MemorySnapshot;
use super::print_memory_table::print_memory_table;
use super::capture_memory_snapshot::capture_memory_snapshot;

pub struct MemoryMonitor {
    sys: System,
    last_snapshot_data: Option<(u64, u64, u64)>,
    snapshots: Vec<MemorySnapshot>,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        MemoryMonitor {
            sys: System::new_all(),
            last_snapshot_data: None,
            snapshots: Vec::new(),
        }
    }

    pub fn capture_and_log_snapshot(&mut self, step_name: &str) {
        capture_memory_snapshot(
            step_name,
            &mut self.sys,
            &mut self.last_snapshot_data,
            &mut self.snapshots,
        );
    }

    pub fn print_final_report(&self) {
        print_memory_table(self.snapshots.clone());
    }
}
