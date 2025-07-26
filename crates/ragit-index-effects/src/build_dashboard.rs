use ragit_index_types::index_struct::Index;
use std::collections::HashMap;
use std::path::PathBuf;
use ragit_types::uid::Uid;
use std::time::Instant;
use ragit_api::audit::AuditRecord;
use ragit_index_types::index_impl::index_api_config_get_api_usage;

pub fn render_build_dashboard(
    index: &Index,
    buffer: &HashMap<PathBuf, HashMap<usize, Uid>>,
    curr_completed_files: &[PathBuf],
    errors: &[(String, String)],
    started_at: Instant,
    flush_count: usize,
    has_to_erase_lines: bool,
) {
    if has_to_erase_lines {
        // erase_lines(9);
    }

    let elapsed_time = Instant::now().duration_since(started_at).as_secs();
    let mut curr_processing_files = vec![];

    for file in buffer.keys() {
        if !curr_completed_files.contains(file) {
            curr_processing_files.push(format!("{}", file.display()));
        }
    }

    println!("---");
    println!("elapsed time: {:02}:{:02}", elapsed_time / 60, elapsed_time % 60);
    println!("staged files: {}, processed files: {}", index.staged_files.len(), index.processed_files.len());
    println!("errors: {}", errors.len());
    println!("committed chunks: {}", index.chunk_count);

    // It messes up with `erase_lines`
    // println!(
    //     "currently processing files: {}",
    //     if curr_processing_files.is_empty() {
    //         String::from("null")
    //     } else {
    //         curr_processing_files.join(", ")
    //     },
    // );

    println!(
        "buffered files: {}, buffered chunks: {}",
        buffer.len(),
        buffer.values().map(|h| h.len()).sum::<usize>(),
    );
    println!("flush count: {flush_count}");
    println!("model: {}", index.api_config.model);

    let mut input_tokens_s = 0;
    let mut output_tokens_s = 0;
    let mut input_cost_s = 0;
    let mut output_cost_s = 0;

    match index_api_config_get_api_usage(index,&index.root_dir, "create_chunk_from") {
        Ok(api_records) => {
            for AuditRecord { input_tokens, output_tokens, input_cost, output_cost } in api_records.values() {
                input_tokens_s += input_tokens;
                output_tokens_s += output_tokens;
                input_cost_s += input_cost;
                output_cost_s += output_cost;
            }

            println!(
                "input tokens: {input_tokens_s} ({:.3}$), output tokens: {output_tokens_s} ({:.3}$)",
                input_cost_s as f64 / 1_000_000.0,
                output_cost_s as f64 / 1_000_000.0,
            );
        },
        Err(_) => {
            println!("input tokens: ??? (????$), output tokens: ??? (????$)");
        },
    }
}