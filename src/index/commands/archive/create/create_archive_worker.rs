use crate::prelude::*;
use crate::index::commands::archive::create::status::Status;
use crate::index::commands::archive::create::channel::Channel;
use crate::index::commands::archive::create::request::Request;
use crate::index::commands::archive::create::response::Response;
use crate::index::commands::archive::BlockType;
use std::collections::HashMap;
use std::time::Instant;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use ragit_fs::{write_bytes, WriteMode, file_size, remove_file, exists};
use crate::uid::load_from_file;

const BLOCK_SIZE: usize = 1 << 20;

impl Index {
    pub(crate) fn create_archive_worker(
        &self,
        workers: &[Channel],
        size_limit: Option<u64>,
        output: String,
        include_configs: bool,
        include_prompts: bool,
        quiet: bool,
    ) -> Result<(), Error> {
        let mut curr_block = vec![];
        let mut curr_block_size = 0;
        let mut round_robin = 0;
        let mut status = Status {
            started_at: Instant::now(),
            block_count: HashMap::new(),
        };
        let mut has_to_erase_lines = false;

        if let Some(size_limit) = size_limit {
            if size_limit < 4096 {
                return Err(Error::CannotCreateArchive(String::from("If size-limit is too small, it may behave oddly. Size-limit has to be at least 4 KiB.")));
            }
        }

        workers[round_robin % workers.len()].send(Request::Compress(BlockType::Index, vec![])).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
        round_robin += 1;
        workers[round_robin % workers.len()].send(Request::Compress(BlockType::Meta, vec![])).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
        round_robin += 1;

        if include_prompts {
            workers[round_robin % workers.len()].send(Request::Compress(BlockType::Prompt, vec![])).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
            round_robin += 1;
        }

        if include_configs {
            workers[round_robin % workers.len()].send(Request::Compress(BlockType::Config, vec![])).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
            round_robin += 1;
        }

        for file_index in self.get_all_file_indexes()?.iter() {
            for chunk_uid in load_from_file(file_index)? {
                let chunk = self.get_chunk_by_uid(chunk_uid)?;
                let curr_chunk_size = chunk.get_approx_size();

                if curr_chunk_size + curr_block_size > BLOCK_SIZE {
                    workers[round_robin % workers.len()].send(Request::Compress(BlockType::Chunk, curr_block)).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
                    curr_block = vec![chunk.uid];
                    curr_block_size = curr_chunk_size;
                    round_robin += 1;
                } else {
                    curr_block_size += curr_chunk_size;
                    curr_block.push(chunk.uid);
                }
            }
        }

        let mut curr_image_block = vec![];
        let mut curr_image_desc_block = vec![];
        let mut curr_image_block_size = 0;
        let mut curr_image_desc_block_size = 0;

        for image_file in self.get_all_image_files()?.iter() {
            let image_uid = Uid::from_prefix_and_suffix(
                &file_name(path_utils::str_to_path_ref(&parent(path_utils::pathbuf_to_str(image_file))?)?),
                &file_name(path_utils::str_to_path_ref(path_utils::pathbuf_to_str(image_file)))?,
            )?;
            let image_bytes_len = file_size(path_utils::str_to_path_ref(path_utils::pathbuf_to_str(image_file)))?;
            let image_desc_len = file_size(path_utils::str_to_path_ref(&set_extension(path_utils::pathbuf_to_str(image_file), "json")?))?;

            if image_bytes_len + curr_image_block_size > BLOCK_SIZE as u64 {
                workers[round_robin % workers.len()].send(Request::Compress(BlockType::ImageBytes, curr_image_block)).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
                curr_image_block = vec![image_uid];
                curr_image_block_size = image_bytes_len;
                round_robin += 1;
            } else {
                curr_image_block_size += image_bytes_len;
                curr_image_block.push(image_uid);
            }

            if image_desc_len + curr_image_desc_block_size > BLOCK_SIZE as u64 {
                workers[round_robin % workers.len()].send(Request::Compress(BlockType::ImageDesc, curr_image_desc_block)).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
                curr_image_desc_block = vec![image_uid];
                curr_image_desc_block_size = image_desc_len;
                round_robin += 1;
            } else {
                curr_image_desc_block_size += image_desc_len;
                curr_image_desc_block.push(image_uid);
            }
        }

        if !curr_block.is_empty() {
            workers[round_robin % workers.len()].send(Request::Compress(BlockType::Chunk, curr_block)).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
            round_robin += 1;
        }

        if !curr_image_block.is_empty() {
            workers[round_robin % workers.len()].send(Request::Compress(BlockType::ImageBytes, curr_image_block)).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
            round_robin += 1;
        }

        if !curr_image_desc_block.is_empty() {
            workers[round_robin % workers.len()].send(Request::Compress(BlockType::ImageDesc, curr_image_desc_block)).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
        }

        let mut curr_output_size = 0;
        let mut curr_output_seq = 0;
        let mut killed_workers = vec![];

        let size_limit_comp = size_limit.unwrap_or(u64::MAX);
        let mut curr_output_file = if size_limit.is_some() { format!("{output}-{curr_output_seq:06}") } else { output.clone() };
        write_bytes(
            &curr_output_file,
            &[],
            WriteMode::CreateOrTruncate,
        )?;
        let mut splitted_block_index = 0;

        for worker in workers.iter() {
            worker.send(Request::TellMeWhenYouAreDone).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
        }

        loop {
            if !quiet {
                self.render_archive_create_dashboard(
                    &status,
                    workers.len() - killed_workers.len(),
                    curr_output_seq,
                    has_to_erase_lines,
                );
                has_to_erase_lines = true;
            }

            for (worker_id, worker) in workers.iter().enumerate() {
                if killed_workers.contains(&worker_id) {
                    continue;
                }

                match worker.try_recv() {
                    Ok(msg) => match msg {
                        Response::Compressed(block_type, block_path) => {
                            let block_size = file_size(&block_path)?;

                            match status.block_count.get_mut(&block_type) {
                                Some(n) => { *n += 1; },
                                None => { status.block_count.insert(block_type, 1); },
                            }

                            write_bytes(
                                &curr_output_file,
                                &[
                                    block_type.to_byte(),
                                    (block_size >> 24) as u8,
                                    ((block_size >> 16) & 0xff) as u8,
                                    ((block_size >> 8) & 0xff) as u8,
                                    (block_size & 0xff) as u8,
                                ],
                                WriteMode::AlwaysAppend,
                            )?;
                            write_bytes(
                                &curr_output_file,
                                &read_bytes(&block_path)?,
                                WriteMode::AlwaysAppend,
                            )?;
                            curr_output_size += block_size + 5;

                            if curr_output_size > size_limit_comp {
                                if curr_output_size > size_limit_comp {
                                    let approx_split_count = curr_output_size / (size_limit_comp - 8) + 1;
                                    let chunk_size = (curr_output_size / approx_split_count + 1) as usize;
                                    let bytes = read_bytes(&curr_output_file)?;
                                    let split_count = bytes.chunks(chunk_size).count();

                                    for (index, chunk) in bytes.chunks(chunk_size).enumerate() {
                                        write_bytes(
                                            &curr_output_file,
                                            &[
                                                BlockType::Splitted.to_byte(),
                                                (splitted_block_index >> 16) as u8,
                                                ((splitted_block_index >> 8) & 0xff) as u8,
                                                (splitted_block_index & 0xff) as u8,
                                                (index >> 8) as u8,
                                                (index & 0xff) as u8,
                                                (split_count >> 8) as u8,
                                                (split_count & 0xff) as u8,
                                            ],
                                            WriteMode::CreateOrTruncate,
                                        )?;
                                        write_bytes(
                                            &curr_output_file,
                                            chunk,
                                            WriteMode::AlwaysAppend,
                                        )?;
                                        curr_output_seq += 1;
                                        curr_output_file = format!("{output}-{curr_output_seq:06}");
                                    }

                                    curr_output_seq -= 1;
                                    splitted_block_index += 1;
                                }

                                curr_output_size = 0;
                                curr_output_seq += 1;
                                curr_output_file = format!("{output}-{curr_output_seq:06}");
                                write_bytes(
                                    &curr_output_file,
                                    &[],
                                    WriteMode::AlwaysCreate,
                                )?;
                            }

                            remove_file(&block_path)?;
                        },
                        Response::IAmDone => {
                            worker.send(Request::Kill).map_err(|_| Error::MPSCError(String::from("Create-archive worker hung up.")))?;
                            killed_workers.push(worker_id);
                        },
                        Response::Error(e) => {
                            return Err(e);
                        },
                    },
                    Err(mpsc::TryRecvError::Empty) => {},
                    Err(mpsc::TryRecvError::Disconnected) => {
                        return Err(Error::MPSCError(String::from("Create-archive worker hung up.")));
                    },
                }
            }

            if killed_workers.len() == workers.len() {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        if exists(&curr_output_file) && file_size(&curr_output_file)? == 0 {
            remove_file(&curr_output_file)?;
        }

        if !quiet {
            self.render_archive_create_dashboard(
                &status,
                workers.len() - killed_workers.len(),
                curr_output_seq,
                has_to_erase_lines,
            );
        }

        Ok(())
    }
}
