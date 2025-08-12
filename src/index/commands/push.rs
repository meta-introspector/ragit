use crate::error::Error;
use crate::index::commands::archive::erase_lines;
use crate::index::index_struct::Index;
use crate::index::commands::get_ragit_api_key::get_ragit_api_key;
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct PushResult {
    pub success: usize,
    pub errors: usize,
}

impl Index {
    pub async fn push(
        &self,
        remote_url: String,
        quiet: bool,
    ) -> Result<PushResult, Error> {
        let status = HashMap::new();
        let started_at = Instant::now();
        let mut has_to_erase_lines = false;

        if !quiet {
            self.render_push_dashboard(
                &status,
                started_at,
                has_to_erase_lines,
            );
            has_to_erase_lines = true;
        }

        let api_key = get_ragit_api_key()?;
        let self_uid = self.calculate_uid(false  /* force */)?;

        // TODO: implement push
        std::thread::sleep(Duration::from_secs(1));

        if !quiet {
            self.render_push_dashboard(
                &status,
                started_at,
                has_to_erase_lines,
            );
        }

        Ok(PushResult {
            success: 0,
            errors: 0,
        })
    }

    fn render_push_dashboard(
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

        println!("pushed files: {}", status.get("pushed_files").unwrap_or(&0));
        println!("errors: {}", status.get("errors").unwrap_or(&0));
    }
}


