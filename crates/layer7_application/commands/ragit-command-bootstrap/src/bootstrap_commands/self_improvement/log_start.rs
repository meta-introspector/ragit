use super::super::memory_utils::print_memory_usage;
use sysinfo::System;

pub fn log_start(verbose: bool, sys: &mut System, last_snapshot_data: &mut Option<(u64, u64, u64)>) {
    if verbose {
        println!("bootstrap_index_self: Running self-improvement query");
        print_memory_usage(sys, "Before self-improvement query", last_snapshot_data);
    }
}
