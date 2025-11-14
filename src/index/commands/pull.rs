use crate::index::index_struct::Index;
use crate::error::Error;
use crate::index::commands::archive::BlockType;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct PullResult {
    pub success: usize,
    pub errors: usize,
}

impl Index {
    pub async fn pull(
        &mut self,
        remote_url: String,
        quiet: bool,
    ) -> Result<PullResult, Error> {
        let status = HashMap::new();
        let started_at = Instant::now();
        let mut has_to_erase_lines = false;

        if !quiet {
            self.render_pull_dashboard(
                &status,
                started_at,
                has_to_erase_lines,
            );
            has_to_erase_lines = true;
        }

        // TODO: implement pull
        std::thread::sleep(Duration::from_secs(1));

        if !quiet {
            self.render_pull_dashboard(
                &status,
                started_at,
                has_to_erase_lines,
            );
        }

        Ok(PullResult {
            success: 0,
            errors: 0,
        })
    }

    fn render_pull_dashboard(
        &self,
        status: &HashMap<BlockType, usize>,
        started_at: Instant,
        has_to_erase_lines: bool,
    ) {
        if has_to_erase_lines {
            // erase_lines(6);
        }

        println!("---");
        let elapsed_time = Instant::now().duration_since(started_at).as_secs();
        println!("elapsed time: {:02}:{:02}", elapsed_time / 60, elapsed_time % 60);

        println!(
            "index blocks: {}",
            status.get(&BlockType::Index).unwrap_or(&0),
        );
        println!(
            "chunk blocks: {}",
            status.get(&BlockType::Chunk).unwrap_or(&0),
        );
        println!(
            "image blocks (blob): {}",
            status.get(&BlockType::ImageBytes).unwrap_or(&0),
        );
        println!(
            "image blocks (desc): {}",
            status.get(&BlockType::ImageDesc).unwrap_or(&0),
        );
    }
}
