use crate::{
    constant::INDEX_FILE_NAME,
    error::Error,
    Index,
    LoadMode,
};
use crate::index::commands::archive::{erase_lines, BlockType, request_binary_file, request_json_file};
use crate::Path;
use ragit_fs::{remove_dir_all, write_bytes, WriteMode, remove_file};
use std::collections::HashMap;
use std::time::Instant;

pub struct CloneResult {
    pub success: usize,
    pub errors: usize,
}

impl Index {
    pub async fn clone(
        root_dir: Path,
        url: String,
        force: bool,
        quiet: bool,
    ) -> Result<CloneResult, Error> {
        if Index::load(root_dir.clone(), LoadMode::Minimum).is_ok() {
            if force {
                remove_dir_all(path_utils::pathbuf_to_str(&root_dir))?;
            }

            else {
                return Err(Error::CannotClone(format!("`{}` already exists", root_dir.display())));
            }
        }

        let mut result = Index::new(root_dir.clone())?;
        result.repo_url = Some(url.clone());
        result.save_to_file(result.root_dir.join(INDEX_FILE_NAME))?;

        let mut status = HashMap::new();
        let started_at = Instant::now();
        let mut has_to_erase_lines = false;

        loop {
            if !quiet {
                Index::render_clone_dashboard(
                    &status,
                    started_at,
                    has_to_erase_lines,
                );
                has_to_erase_lines = true;
            }

            let block_type = request_json_file(
                &url,
                &format!("block_type/{}", result.uid.unwrap()),
            ).await?;
            let block_type = BlockType::try_from(block_type)?;

            let block_data = request_binary_file(
                &url,
                &format!("block_data/{}", result.uid.unwrap()),
            ).await?;

            match status.get_mut(&block_type) {
                Some(n) => { *n += 1; },
                None => { status.insert(block_type, 1); },
            }

            let block_file_name = format!("__archive_block_{}", result.uid.unwrap());
            write_bytes(
                &block_file_name,
                &block_data,
                WriteMode::AlwaysCreate,
            )?;

            // TODO: process block

            remove_file(&block_file_name)?;

            if !quiet {
                Index::render_clone_dashboard(
                    &status,
                    started_at,
                    has_to_erase_lines,
                );
            }

            break;
        }

        Ok(CloneResult {
            success: 0,
            errors: 0,
        })
    }

    fn render_clone_dashboard(
        status: &HashMap<BlockType, usize>,
        started_at: Instant,
        has_to_erase_lines: bool,
    ) {
        if has_to_erase_lines {
            erase_lines(6);
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