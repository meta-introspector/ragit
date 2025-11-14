use std::collections::HashMap;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::thread;

use ragit_fs::{file_size, read_bytes_offset, remove_file, write_bytes, WriteMode};

use crate::error::Error;
use crate::index::{index_struct::Index, load_mode::LoadMode};

use super::{block_type::BlockType, request::Request, response::Response, status::Status, Channel};

impl Index {
    pub fn extract_archive_worker(
        root_dir: &str,
        mut archives: Vec<String>,
        workers: &[Channel],
        quiet: bool,
    ) -> Result<HashMap<BlockType, usize>, Error> {
        let mut killed_workers = vec![];
        let mut round_robin = 0;
        let mut status = Status {
            started_at: Instant::now(),
            block_count: HashMap::new(),
            block_complete: HashMap::new(),
        };

        Index::new(root_dir.to_string())?;
        let mut splitted_blocks: HashMap<usize, HashMap<usize, Vec<u8>>> = HashMap::new();
        let mut tmp_files_for_splitted_blocks = vec![];
        let mut has_to_erase_lines = false;

        while let Some(archive) = archives.pop() {
            let archive_size = file_size(&archive)?;
            let mut cursor = 0;

            loop {
                let header = read_bytes_offset(&archive, cursor, cursor + 5)?;

                if header[0] == BlockType::Splitted.to_byte() {
                    match status.block_count.get_mut(&BlockType::Splitted) {
                        Some(n) => {
                            *n += 1;
                        }
                        None => {
                            status.block_count.insert(BlockType::Splitted, 1);
                        }
                    }

                    let header = read_bytes_offset(&archive, cursor, cursor + 8)?;
                    let body = read_bytes_offset(&archive, cursor + 8, file_size(&archive)?)?;
                    let outer_index =
                        ((header[1] as usize) << 16) + ((header[2] as usize) << 8) + header[3] as usize;
                    let inner_index = ((header[4] as usize) << 8) + header[5] as usize;
                    let total_count = ((header[6] as usize) << 8) + header[7] as usize;

                    match splitted_blocks.get_mut(&outer_index) {
                        Some(blocks) => {
                            blocks.insert(inner_index, body);

                            if blocks.len() == total_count {
                                let mut blocks = blocks
                                    .iter()
                                    .map(|(inner_index, body)| (*inner_index, body.to_vec()))
                                    .collect::<Vec<_>>();
                                blocks.sort_by_key(|(inner_index, _)| *inner_index);
                                let blocks =
                                    blocks.into_iter().map(|(_, body)| body).collect::<Vec<_>>();
                                let tmp_file_for_splitted_blocks =
                                    format!("{archive}-splitted-{outer_index:06}");
                                write_bytes(
                                    &tmp_file_for_splitted_blocks,
                                    &blocks.concat(),
                                    WriteMode::AlwaysCreate,
                                )?;
                                splitted_blocks.remove(&outer_index);
                                archives.push(tmp_file_for_splitted_blocks.clone());
                                tmp_files_for_splitted_blocks
                                    .push(tmp_file_for_splitted_blocks);
                            }
                        }
                        None => {
                            let mut blocks = HashMap::new();
                            blocks.insert(inner_index, body);
                            splitted_blocks.insert(outer_index, blocks);
                        }
                    }

                    break;
                }

                let block_type = BlockType::try_from(header[0])
                    .map_err(|_| Error::BrokenArchive(format!("unknown block type: {}", header[0])))?;
                let body_size = ((header[1] as u64) << 24)
                    + ((header[2] as u64) << 16)
                    + ((header[3] as u64) << 8)
                    + header[4] as u64;

                match status.block_count.get_mut(&block_type) {
                    Some(n) => {
                        *n += 1;
                    }
                    None => {
                        status.block_count.insert(block_type, 1);
                    }
                }

                workers[round_robin % workers.len()]
                    .tx.send(Request::Extract {
                        block_type,
                        path: archive.to_string(),
                        from: cursor + 5,
                        to: cursor + 5 + body_size,
                    })
                    .map_err(|_| Error::MPSCError(String::from("Extract-archive worker hung up.")))?;
                cursor += 5 + body_size;
                round_robin += 1;

                if cursor == archive_size {
                    break;
                } else if cursor > archive_size {
                    return Err(Error::BrokenArchive(format!(
                        "`{archive}` is broken. cursor: {cursor}, archive_size: {archive_size}"
                    )));
                }
            }

            if !quiet {
                super::render_dashboard::render_archive_extract_dashboard(
                    &status,
                    workers.len() - killed_workers.len(),
                    has_to_erase_lines,
                );
                has_to_erase_lines = true;
            }
        }

        for worker in workers.iter() {
            worker
                .tx.send(Request::TellMeWhenYouAreDone)
                .map_err(|_| Error::MPSCError(String::from("Extract-archive worker hung up.")))?;
        }

        loop {
            if !quiet {
                super::render_dashboard::render_archive_extract_dashboard(
                    &status,
                    workers.len() - killed_workers.len(),
                    has_to_erase_lines,
                );
                has_to_erase_lines = true;
            }

            for (worker_id, worker) in workers.iter().enumerate() {
                if killed_workers.contains(&worker_id) {
                    continue;
                }

                match worker.rx.try_recv() {
                    Ok(msg) => match msg {
                        Response::Complete(block_type) => {
                            match status.block_complete.get_mut(&block_type) {
                                Some(n) => {
                                    *n += 1;
                                }
                                None => {
                                    status.block_complete.insert(block_type, 1);
                                }
                            }
                        }
                        Response::IAmDone => {
                            worker
                                .tx.send(Request::Kill)
                                .map_err(|_| Error::MPSCError(String::from("Extract-archive worker hung up.")))?;
                            killed_workers.push(worker_id);
                        }
                        Response::Error(e) => {
                            return Err(e);
                        }
                    },
                    Err(mpsc::TryRecvError::Empty) => {}
                    Err(mpsc::TryRecvError::Disconnected) => {
                        return Err(Error::MPSCError(String::from(
                            "Extract-archive worker hung up.",
                        )));
                    }
                }
            }

            if killed_workers.len() == workers.len() {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        for tmp_file_for_splitted_blocks in tmp_files_for_splitted_blocks.iter() {
            remove_file(tmp_file_for_splitted_blocks)?;
        }

        if !quiet {
            super::render_dashboard::render_archive_extract_dashboard(
                &status,
                workers.len() - killed_workers.len(),
                has_to_erase_lines,
            );
            has_to_erase_lines = true;
        }

        // it creates file indexes and tfidfs
        let mut index = Index::load(root_dir.to_string().into(), LoadMode::Minimum)?;
        index.recover()?;

        // archives created by older versions do not have a uid
        // so it has to create one
        index.calculate_and_save_uid()?;

        if !quiet {
            super::render_dashboard::render_archive_extract_dashboard(
                &status,
                workers.len() - killed_workers.len(),
                has_to_erase_lines,
            );
        }

        Ok(status.block_count)
    }
}
