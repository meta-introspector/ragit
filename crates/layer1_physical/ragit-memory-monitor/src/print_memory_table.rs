use crate::MemorySnapshot;
use crate::table::Table;
use crate::column::Column;
use crate::format_bytes::format_bytes;
use crate::format_signed_bytes::format_signed_bytes;

pub fn print_memory_table(snapshots: Vec<MemorySnapshot>) {
    let mut table = Table::new(Vec::new());

    table.add_column(Column::new("Step".to_string(), 30));
    table.add_column(Column::new("Duration".to_string(), 15));
    table.add_column(Column::new("Total".to_string(), 15));
    table.add_column(Column::new("Process RSS".to_string(), 15));
    table.add_column(Column::new("RSS Delta".to_string(), 15));
    table.add_column(Column::new("Units".to_string(), 10));
    table.add_column(Column::new("Used Delta/Unit".to_string(), 20));
    table.add_column(Column::new("Duration/Unit".to_string(), 20));

    for snapshot in snapshots {
        if snapshot.duration_since_last.as_nanos() == 0 && snapshot.units_processed == 0 {
            continue; // Skip snapshots with zero duration and zero units processed
        }
        let used_delta_per_unit = if snapshot.units_processed > 0 {
            snapshot.used_delta / snapshot.units_processed as i64
        } else {
            0
        };
        let duration_per_unit = if snapshot.units_processed > 0 {
            snapshot.duration_since_last.as_nanos() / snapshot.units_processed as u128
        } else {
            0
        };
        table.add_row(vec![
            snapshot.step,
            format!("{:.2?}", snapshot.duration_since_last),
            format_bytes(snapshot.total_memory),
            format_bytes(snapshot.process_rss),
            format_signed_bytes(snapshot.rss_delta),
            snapshot.units_processed.to_string(),
            format_signed_bytes(used_delta_per_unit),
            format!("{:.2?}ns", duration_per_unit),
        ]);
    }

    println!("
--- Memory Usage Summary ---");
    table.print();
}
