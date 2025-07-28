use anyhow::{anyhow, Result};
use std::io::{self, Write};
use sysinfo::{Pid, System};

pub fn print_memory_usage(sys: &mut System, message: &str, last_process_memory_kb: &mut Option<u64>) {
    sys.refresh_memory();
    if let Some(process) = sys.process(Pid::from_u32(std::process::id())) {
        let current_process_memory_kb = process.memory() / 1024;
        let total_memory_kb = sys.total_memory() / 1024;
        let used_memory_kb = sys.used_memory() / 1024;

        if let Some(last_mem) = last_process_memory_kb {
            let delta = current_process_memory_kb as i64 - *last_mem as i64;
            println!(
                "Memory Usage ({}): Total: {} KB, Used: {} KB, Process RSS: {} KB (Delta: {} KB)",
                message, total_memory_kb, used_memory_kb, current_process_memory_kb, delta
            );
            *last_mem = current_process_memory_kb;
        } else {
            println!(
                "Memory Usage ({}): Total: {} KB, Used: {} KB, Process RSS: {} KB",
                message, total_memory_kb, used_memory_kb, current_process_memory_kb
            );
            *last_process_memory_kb = Some(current_process_memory_kb);
        }
    } else {
        println!(
            "Memory Usage ({}): Total: {} KB, Used: {} KB",
            message,
            sys.total_memory() / 1024,
            sys.used_memory() / 1024
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
