use anyhow::{anyhow, Result};
use std::io::{self, Write};
use sysinfo::{Pid, System};

pub fn print_memory_usage(sys: &mut System, message: &str, last_process_memory_kb: &mut Option<u64>) {
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
        let total_memory_kb = sys.total_memory() / 1024;
        let used_memory_kb = sys.used_memory() / 1024;

        if let Some(last_mem_val_kb) = last_process_memory_kb {
            let delta_kb = current_process_memory_kb as i64 - *last_mem_val_kb as i64;
            let delta_str = if delta_kb >= 1024 * 1024 { // If 1GB or more
                format!("{:.2} GB", delta_kb as f64 / 1024.0 / 1024.0)
            } else if delta_kb >= 1024 { // If 1MB or more
                format!("{:.2} MB", delta_kb as f64 / 1024.0)
            } else {
                format!("{} KB", delta_kb)
            };

            println!(
                "Memory Usage ({}): Total: {}, Used: {}, Process RSS: {} (Delta: {})",
                message,
                format_memory(total_memory_kb),
                format_memory(used_memory_kb),
                format_memory(current_process_memory_kb),
                delta_str
            );
            *last_process_memory_kb = Some(current_process_memory_kb);
        } else {
            println!(
                "Memory Usage ({}): Total: {}, Used: {}, Process RSS: {}",
                message,
                format_memory(total_memory_kb),
                format_memory(used_memory_kb),
                format_memory(current_process_memory_kb)
            );
            *last_process_memory_kb = Some(current_process_memory_kb);
        }
    } else {
        println!(
            "Memory Usage ({}): Total: {}, Used: {}",
            message,
            format_memory(sys.total_memory() / 1024),
            format_memory(sys.used_memory() / 1024)
        );
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
                return Err(anyhow!(
                    "Memory limit exceeded at {}: Process RSS {} KB > Limit {} KB ({} GB)",
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
    println!("--- Process List ({}) ---", message);
    let mut process_memory_summary: HashMap<String, u64> = HashMap::new();

    for (pid, process) in sys.processes() {
        let mem_kb = process.memory() / 1024;
        println!(
            "PID: {}, Name: {}, CPU: {}%, Memory: {} KB",
            pid,
            process.name(),
            process.cpu_usage(),
            mem_kb
        );
        *process_memory_summary.entry(process.name().to_string()).or_insert(0) += mem_kb;
    }
    println!("------------------------");

    println!("\n--- Process Memory Summary ({}) ---", message);
    println!("{:<20} {:>15}", "Process Name", "Total Memory (MB)");
    println!("{:-<20} {:-<15}", "", "");
    let mut sorted_summary: Vec<(&String, &u64)> = process_memory_summary.iter().collect();
    sorted_summary.sort_by(|a, b| b.1.cmp(a.1));

    for (name, &total_mem_kb) in sorted_summary {
        println!("{:<20} {:>15.2}", name, (total_mem_kb as f64) / 1024.0);
    }
    println!("-------------------------------------");
    io::stdout().flush().unwrap();
}