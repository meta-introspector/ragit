use super::super::memory_utils::print_memory_usage;
use sysinfo::System;

pub fn log_start(verbose: bool, sys: &mut System) {
    if verbose {
        println!("bootstrap_index_self: Running self-improvement query");
        print_memory_usage(sys, "Before self-improvement query");
    }
}
