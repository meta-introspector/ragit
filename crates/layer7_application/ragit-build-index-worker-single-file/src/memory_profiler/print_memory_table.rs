use crate::memory_profiler::memory_snapshot::MemorySnapshot;
use crate::memory_profiler::format_bytes::format_bytes;
use crate::memory_profiler::format_signed_bytes::format_signed_bytes;
use crate::memory_profiler::table::Table;
use crate::memory_profiler::column::Column;


pub fn print_memory_table(snapshots: Vec<MemorySnapshot>) {
    let mut table = Table::new(Vec::new());

    table.add_column(Column::new("Step".to_string(), 30));
    table.add_column(Column::new("Total".to_string(), 15));
    table.add_column(Column::new("Used".to_string(), 15));
    table.add_column(Column::new("Process RSS".to_string(), 15));
    table.add_column(Column::new("Total Delta".to_string(), 15));
    table.add_column(Column::new("Used Delta".to_string(), 15));
    table.add_column(Column::new("RSS Delta".to_string(), 15));

    for snapshot in snapshots {
        table.add_row(vec![
            snapshot.step,
            format_bytes(snapshot.total_memory),
            format_bytes(snapshot.used_memory),
            format_bytes(snapshot.process_rss),
            format_signed_bytes(snapshot.total_delta),
            format_signed_bytes(snapshot.used_delta),
            format_signed_bytes(snapshot.rss_delta),
        ]);
    }

    println!("\n--- Memory Usage Summary ---");
    table.print();
}
