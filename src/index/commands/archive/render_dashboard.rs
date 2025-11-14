use std::time::Instant;


use super::{erase_lines::erase_lines, status::Status, BlockType};

pub fn render_archive_extract_dashboard(
        status: &Status,
        workers: usize,
        has_to_erase_lines: bool,
    ) {
        if has_to_erase_lines {
            erase_lines(6);
        }

        println!("---");
        let elapsed_time = Instant::now().duration_since(status.started_at.clone()).as_secs();
        println!("elapsed time: {:02}:{:02}", elapsed_time / 60, elapsed_time % 60);
        println!("workers: {workers}");

        println!(
            "chunk blocks: {}/{}",
            status.block_complete.get(&BlockType::Chunk).unwrap_or(&0),
            status.block_count.get(&BlockType::Chunk).unwrap_or(&0),
        );
        println!(
            "image blocks (blob): {}/{}",
            status.block_complete.get(&BlockType::ImageBytes).unwrap_or(&0),
            status.block_count.get(&BlockType::ImageBytes).unwrap_or(&0),
        );
        println!(
            "image blocks (desc): {}/{}",
            status.block_complete.get(&BlockType::ImageDesc).unwrap_or(&0),
            status.block_count.get(&BlockType::ImageDesc).unwrap_or(&0),
        );
    }

