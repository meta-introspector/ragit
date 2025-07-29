use anyhow::{anyhow, Result};
use std::io::{self, Write};
use sysinfo::{Pid, System};
use memory_stats::memory_stats;

// Helper function for memory usage reporting
fn report_memory_usage(message: &str, total: String, used: String, rss: String, delta: String) {
    println!(
        r"Memory Usage ({}): System Total: {}, System Used: {}, Process RSS: {} (Delta: {})",
        message,
        total,
        used,
        rss,
        delta
    );
}

// Helper function for memory usage reporting (no process RSS delta)
fn report_memory_usage_no_process(message: &str, total: String, used: String) {
    println!(
        r"Memory Usage ({}): System Total: {}, System Used: {}",
        message,
        total,
        used
    );
}

// Helper function for detailed memory usage reporting
fn report_detailed_memory_usage(physical_mb: f64, virtual_mb: f64) {
    println!(
        r"  Process Physical: {:.2} MB, Process Virtual: {:.2} MB",
        physical_mb,
        virtual_mb
    );
}

// Helper function for detailed memory usage not available
fn report_detailed_memory_usage_not_available(message: &str) {
    println!(r"Detailed Memory Usage ({}): Not available", message);
}

// Helper function for memory limit exceeded error
fn report_memory_limit_exceeded(message: &str, current_kb: u64, limit_kb: u64, limit_gb: u64) -> anyhow::Error {
    anyhow!(
        r"Memory limit exceeded at {}: Process RSS {} KB > Limit {} KB ({} GB)",
        message,
        current_kb,
        limit_kb,
        limit_gb
    )
}

// Helper function for process list header
fn report_process_list_header(message: &str) {
    println!("--- Process List ({}) ---", message);
}

// Helper function for process info
fn report_process_info(pid: sysinfo::Pid, name: &str, cpu_usage: f32, mem_kb: u64) {
    println!(
        r"PID: {}, Name: {}, CPU: {}%, Memory: {} KB",
        pid,
        name,
        cpu_usage,
        mem_kb
    );
}

// Helper function for process list separator
fn report_process_list_separator() {
    println!("------------------------");
}

// Helper function for process memory summary header
fn report_process_summary_header(message: &str) {
    println!("\n--- Process Memory Summary ({}) ---", message);
}

// Helper function for process summary table header
fn report_process_summary_table_header() {
    println!("{:<20} {:>15}", "Process Name", "Total Memory (MB)");
}

// Helper function for process summary table separator
fn report_process_summary_table_separator() {
    println!("{:-<20} {:-<15}", "", "");
}

// Helper function for process summary table row
fn report_process_summary_table_row(name: &str, total_mem_mb: f64) {
    println!("{:<20} {:>15.2}", name, total_mem_mb);
}

// Helper function for process summary table footer
fn report_process_summary_table_footer() {
    println!("-------------------------------------");
}

pub fn print_memory_usage(sys: &mut System, message: &str, last_total_system_memory_kb: &mut Option<u64>) {
    sys.refresh_memory();

    let format_memory = |kb: u64| -> String {
        if kb >= 1024 * 1024 { // If 1GB or more
            format!("{:.2} GB", kb as f64 / 1024.0 / 1024.0)
        } else if kb >= 1024 { // If 1MB or more
            format!("{:.2} MB", kb as f64 / 1024.0)
        } else {
            format!("{} KB", kb)
        }
    };

    if let Some(process) = sys.process(Pid::from_u32(std::process::id())) {
        let current_process_memory_kb = process.memory() / 1024;
        let total_system_memory_kb = sys.total_memory() / 1024;
        let used_system_memory_kb = sys.used_memory() / 1024;

        let mut delta_str = String::new();
        if let Some(last_mem_val_kb) = last_total_system_memory_kb {
            let delta_kb = total_system_memory_kb as i64 - *last_mem_val_kb as i64;
            delta_str = if delta_kb >= 1024 * 1024 { // If 1GB or more
                format!("{:.2} GB", delta_kb as f64 / 1024.0 / 1024.0)
            } else if delta_kb >= 1024 { // If 1MB or more
                format!("{:.2} MB", delta_kb as f64 / 1024.0)
            } else {
                format!("{} KB", delta_kb)
            };
        }

        report_memory_usage(
            message,
            format_memory(total_system_memory_kb),
            format_memory(used_system_memory_kb),
            format_memory(current_process_memory_kb),
            delta_str
        );
        *last_total_system_memory_kb = Some(total_system_memory_kb);
    } else {
        report_memory_usage_no_process(
            message,
            format_memory(sys.total_memory() / 1024),
            format_memory(sys.used_memory() / 1024)
        );
    }

    if let Some(usage) = memory_stats() {
        report_detailed_memory_usage(
            usage.physical_mem as f64 / (1024.0 * 1024.0),
            usage.virtual_mem as f64 / (1024.0 * 1024.0)
        );
    } else {
        report_detailed_memory_usage_not_available(message);
    }
    io::stdout().flush().unwrap();
}

pub fn check_memory_limit(
    sys: &mut System,
    max_memory_gb: Option<u64>,
    message: &str,
) -> Result<()> {
    if let Some(limit_gb) = max_memory_gb {
        sys.refresh_memory();
        if let Some(process) = sys.process(Pid::from_u32(std::process::id())) {
            let current_process_memory_kb = process.memory() / 1024;
            let limit_kb = limit_gb * 1024 * 1024; // Convert GB to KB

            if current_process_memory_kb > limit_kb {
                return Err(report_memory_limit_exceeded(
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

use std::collections::HashMap;

pub fn print_process_list(sys: &mut System, message: &str) {
    sys.refresh_processes();
    report_process_list_header(message);
    let mut process_memory_summary: HashMap<String, u64> = HashMap::new();

    for (pid, process) in sys.processes() {
        let mem_kb = process.memory() / 1024;
        report_process_info(
            *pid,
            process.name(),
            process.cpu_usage(),
            mem_kb
        );
        *process_memory_summary.entry(process.name().to_string()).or_insert(0) += mem_kb;
    }
    report_process_list_separator();

    report_process_summary_header(message);
    report_process_summary_table_header();
    report_process_summary_table_separator();
    let mut sorted_summary: Vec<(&String, &u64)> = process_memory_summary.iter().collect();
    sorted_summary.sort_by(|a, b| b.1.cmp(a.1));

    for (name, &total_mem_kb) in sorted_summary {
        report_process_summary_table_row(name, (total_mem_kb as f64) / 1024.0);
    }
    report_process_summary_table_footer();
    io::stdout().flush().unwrap();
}