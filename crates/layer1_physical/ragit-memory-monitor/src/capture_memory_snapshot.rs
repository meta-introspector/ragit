use sysinfo::System;
use std::io::Write;
use super::memory_snapshot::MemorySnapshot;

pub fn capture_memory_snapshot(
    step_name: &str,
    sys: &mut System,
    last_snapshot_data: &mut Option<(u64, u64, u64)>,
    memory_snapshots: &mut Vec<MemorySnapshot>,
    last_snapshot_time: &mut Option<std::time::Instant>,
    units_processed: u64,
) {
    sys.refresh_memory();
    let now = std::time::Instant::now();
    let duration_since_last = last_snapshot_time.map_or(std::time::Duration::from_secs(0), |last| now.duration_since(last));

    let current_process_memory = sys.process(sysinfo::Pid::from_u32(std::process::id()))
        .map_or(0, |p| p.memory());
    let total_system_memory = sys.total_memory();
    let used_system_memory = sys.used_memory();

    let (total_delta, used_delta, rss_delta) = if let Some((last_total, last_used, last_rss)) = *last_snapshot_data {
        (
            total_system_memory as i64 - last_total as i64,
            used_system_memory as i64 - last_used as i64,
            current_process_memory as i64 - last_rss as i64,
        )
    } else {
        (0, 0, 0)
    };

    memory_snapshots.push(MemorySnapshot {
        step: step_name.to_string(),
        total_memory: total_system_memory,
        total_delta,
        used_memory: used_system_memory,
        used_delta,
        process_rss: current_process_memory,
        rss_delta,
        timestamp: now,
        duration_since_last,
        units_processed,
    });
    *last_snapshot_data = Some((total_system_memory, used_system_memory, current_process_memory));
    *last_snapshot_time = Some(now);
    std::io::stdout().flush().unwrap();
}
