use super::super::memory_utils::print_memory_usage;
use sysinfo::System;
use super::super::constants::{LOG_RUNNING_FINAL_REFLECTIVE_QUERY, LOG_BEFORE_FINAL_REFLECTIVE_QUERY};

pub fn log_start(verbose: bool, sys: &mut System, last_snapshot_data: &mut Option<(u64, u64, u64)>) {
    if verbose {
        println!("{}", LOG_RUNNING_FINAL_REFLECTIVE_QUERY);
        println!("{}", LOG_BEFORE_FINAL_REFLECTIVE_QUERY);
        print_memory_usage(sys, "Before final reflective query", last_snapshot_data);
    }
}
