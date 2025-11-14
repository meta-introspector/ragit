use crate::prelude::*;
use crate::index::commands::archive::create::status::Status;
use crate::index::commands::archive::erase_lines;
use std::time::Instant;

impl Index {
    fn render_archive_create_dashboard(
        &self,
        status: &Status,
        workers: usize,
        output_seq: usize,
        has_to_erase_lines: bool,
    ) {
        if has_to_erase_lines {
            erase_lines(7);
        }

        let elapsed_time = Instant::now().duration_since(status.started_at.clone()).as_secs();
        println!("---");
        println!("elapsed time: {:02}:{:02}", elapsed_time / 60, elapsed_time % 60);
        println!("workers: {workers}");
        println!("archives: {}", output_seq + 1);

        println!("chunk blocks: {}", status.block_count.get(&BlockType::Chunk).unwrap_or(&0));
        println!("image blocks (blob): {}", status.block_count.get(&BlockType::ImageBytes).unwrap_or(&0));
        println!("image blocks (desc): {}", status.block_count.get(&BlockType::ImageDesc).unwrap_or(&0));
    }
}
