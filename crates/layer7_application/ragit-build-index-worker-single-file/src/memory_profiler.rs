use sysinfo::System;
use std::io::Write;

pub struct MemorySnapshot {
    pub step: String,
    pub total_memory: String,
    pub used_memory: String,
    pub process_rss: String,
    pub delta: String,
}

pub fn capture_memory_snapshot(
    step_name: &str,
    sys: &mut System,
    last_mem_kb: &mut Option<u64>,
    memory_snapshots: &mut Vec<MemorySnapshot>,
) {
    sys.refresh_memory();
    let current_process_memory_kb = sys.process(sysinfo::Pid::from_u32(std::process::id()))
        .map_or(0, |p| p.memory() / 1024);
    let total_memory_kb = sys.total_memory() / 1024;
    let used_memory_kb = sys.used_memory() / 1024;

    let format_memory = |kb: u64| -> String {
        if kb >= 1024 * 1024 { // If 1GB or more
            format!("{:.2} GB", kb as f64 / 1024.0 / 1024.0)
        } else if kb >= 1024 { // If 1MB or more
            format!("{:.2} MB", kb as f64 / 1024.0)
        } else {
            format!("{} KB", kb)
        }
    };

    let delta_kb = if let Some(last_val) = *last_mem_kb {
        current_process_memory_kb as i64 - last_val as i64
    } else {
        0
    };

    let delta_str = if delta_kb >= 1024 * 1024 { // If 1GB or more
        format!("{:.2} GB", delta_kb as f64 / 1024.0 / 1024.0)
    } else if delta_kb >= 1024 { // If 1MB or more
        format!("{:.2} MB", delta_kb as f64 / 1024.0)
    } else {
        format!("{} KB", delta_kb)
    };

    memory_snapshots.push(MemorySnapshot {
        step: step_name.to_string(),
        total_memory: format_memory(total_memory_kb),
        used_memory: format_memory(used_memory_kb),
        process_rss: format_memory(current_process_memory_kb),
        delta: delta_str,
    });
    *last_mem_kb = Some(current_process_memory_kb);
    std::io::stdout().flush().unwrap();
}

pub fn print_memory_table(snapshots: Vec<MemorySnapshot>) {
    println!("\n--- Memory Usage Summary ---");
    println!("{:<30} {:<15} {:<15} {:<15} {:<15}", "Step", "Total", "Used", "Process RSS", "Delta");
    println!("{:-<30} {:-<15} {:-<15} {:-<15} {:-<15}", "", "", "", "", "");
    for snapshot in snapshots {
        println!("{:<30} {:<15} {:<15} {:<15} {:<15}",
            snapshot.step,
            snapshot.total_memory,
            snapshot.used_memory,
            snapshot.process_rss,
            snapshot.delta
        );
    }
    println!("--------------------------------------------------------------------------------------------------");
}