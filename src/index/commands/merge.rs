use crate::index::index_struct::Index;
use crate::index::commands::archive::erase_lines;
use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum MergeMode {
    FastForward,
    ThreeWay,
}

pub struct MergeResult {
    pub success: usize,
    pub errors: usize,
}

impl Index {
    pub async fn merge(
        &mut self,
        remote_url: String,
        mode: MergeMode,
        quiet: bool,
    ) -> Result<MergeResult, Error> {
        let status = HashMap::new();
        let started_at = Instant::now();
        let mut has_to_erase_lines = false;

        if !quiet {
            self.render_merge_dashboard(
                &status,
                started_at,
                has_to_erase_lines,
            );
            has_to_erase_lines = true;
        }

        // TODO: implement merge
        std::thread::sleep(Duration::from_secs(1));

        if !quiet {
            self.render_merge_dashboard(
                &status,
                started_at,
                has_to_erase_lines,
            );
        }

        Ok(MergeResult {
            success: 0,
            errors: 0,
        })
    }

    fn render_merge_dashboard(
        &self,
        status: &HashMap<String, usize>,
        started_at: Instant,
        has_to_erase_lines: bool,
    ) {
        if has_to_erase_lines {
            erase_lines(6);
        }

        println!("---");
        let elapsed_time = Instant::now().duration_since(started_at).as_secs();
        println!("elapsed time: {:02}:{:02}", elapsed_time / 60, elapsed_time % 60);

        println!("merged files: {}", status.get("merged_files").unwrap_or(&0));
        println!("conflicts: {}", status.get("conflicts").unwrap_or(&0));
    }
}
