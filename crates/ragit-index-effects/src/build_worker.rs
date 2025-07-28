use ragit_index_types::index_impl::{index_save_to_file, index_get_uid_path, index_get_data_path, index_remove, index_add_file_index, index_processed_files_insert, index_update_ii_buffer, index_flush_ii_buffer, index_reset_uid, index_calculate_and_save_uid,
				    //index_get_model_by_name,
				    //index_add_image_description,
				    //index_api_config_get_api_usage
};
use ragit_utils::uid_new_file;
//use ragit_index_types::index_get_prompt;
//use ragit_tfidf::save_to_file;
//use ragit_index_types::load_mode::LoadMode;
use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Instant, Duration};
use ragit_types::uid::Uid;
use ragit_types::ii_status::IIStatus;
use ragit_fs::{remove_file,
	       //try_create_dir,
	       //write_bytes,
	       //WriteMode,
	       //exists,
	       //parent
};
use ragit_utils::constant::{CHUNK_DIR_NAME,
			    //IMAGE_DIR_NAME,
			    INDEX_FILE_NAME};
//use ragit_api::audit::AuditRecord;
//use ragit_types::ChunkBuildInfo;
//use ragit_readers::FileReader;
//use ragit_model::Model;
use tokio::sync::mpsc::error::TryRecvError;
use crate::channel::{Channel, WorkerRequest, WorkerResponse};
use crate::build_dashboard::render_build_dashboard;
use crate::build::BuildResult;

pub async fn build_worker(
    index: &mut Index,
    workers: &mut Vec<Channel>,
    started_at: Instant,
    quiet: bool,
    dry_run_llm: bool,
    max_iterations: Option<usize>,
) -> Result<BuildResult, ApiError> {
    println!("build_worker: Starting");
    let mut killed_workers = vec![];
    let mut staged_files = index.staged_files.clone();
    let mut curr_completed_files = Vec::<PathBuf>::new();
    let mut success = 0;
    let mut errors: Vec<(PathBuf, String)> = vec![];
    let mut buffered_chunk_count = 0;
    let mut flush_count = 0;

    // HashMap<file, HashMap<index in file, chunk uid>>
    let mut buffer: HashMap<PathBuf, HashMap<usize, Uid>> = HashMap::new();

    // HashMap<worker id, file>
    let mut curr_processing_file: HashMap<usize, PathBuf> = HashMap::new();

    println!("build_worker: Initializing workers");
    for (worker_index, worker) in workers.iter_mut().enumerate() {
        if let Some(file) = staged_files.pop() {
            println!("build_worker: Sending file {:?} to worker {}", file, worker_index);
            // Previously, all the builds were in serial and this field tells
            // which file the index is building. When something goes wrong, ragit
            // reads this field and clean up garbages. Now, all the builds are in
            // parallel and there's no such thing like `curr_processing_file`. But
            // we still need to tell whether something went wrong while building
            // and this field does that. If it's `Some(_)`, something's wrong and
            // clean-up has to be done.
            index.curr_processing_file = Some("".into());

            buffer.insert(file.clone(), HashMap::new());
            curr_processing_file.insert(worker_index, file.clone());
            worker.send(WorkerRequest::BuildChunks { file: file.clone(), dry_run_llm }).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up")))?;
        }

        else {
            println!("build_worker: No more files to process, killing worker {}", worker_index);
            worker.send(WorkerRequest::Kill).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
            killed_workers.push(worker_index);
        }
    }

    println!("build_worker: Saving index state");
    index_save_to_file(index, index.root_dir.join(INDEX_FILE_NAME).into())?;
    let mut has_to_erase_lines = false;

    println!("build_worker: Entering main loop");
    loop {
        if !quiet {
            if !render_build_dashboard(
                index,
                &buffer,
                &curr_completed_files,
                &errors.iter().map(|(path, msg)| (path.to_string_lossy().into_owned(), msg.clone())).collect::<Vec<(String, String)>>(),
                started_at.clone(),
                flush_count,
                has_to_erase_lines,
                max_iterations,
            ) {
                break;
            }
            has_to_erase_lines = true;
        }

        for (worker_index, worker) in workers.iter_mut().enumerate() {
            if killed_workers.contains(&worker_index) {
                continue;
            }

            match worker.try_recv() {
                Ok(msg) => match msg {
                    WorkerResponse::ChunkComplete { file, chunk_uid, index: chunk_index } => {
                        println!("build_worker: Received ChunkComplete from worker {}", worker_index);
                        buffered_chunk_count += 1;

                        let file_path = PathBuf::from(file);
                        match buffer.get_mut(&file_path) {
                            Some(chunks) => {
                                if let Some(prev_uid) = chunks.insert(chunk_index, chunk_uid) {
                                    return Err(ApiError::Internal(format!("{}th chunk of {} is created twice: {prev_uid}, {chunk_uid}", chunk_index + 1, file_path.display())));
                                }
                            },
                            None => {
                                let mut chunks = HashMap::new();
                                chunks.insert(chunk_index, chunk_uid);
                                buffer.insert(file_path, chunks);
                            },
                        }
                    },
                    WorkerResponse::FileComplete { file, chunk_count } => {
                        println!("build_worker: Received FileComplete from worker {}", worker_index);
                        let file_path = PathBuf::from(file);
                        match buffer.get(&file_path) {
                            Some(chunks) => {
                                if chunks.len() != chunk_count {
                                    return Err(ApiError::Internal(format!("Some chunks in `{}` are missing: expected {chunk_count} chunks, got only {} chunks.", file_path.display(), chunks.len())));
                                }

                                for i in 0..chunk_count {
                                    if !chunks.contains_key(&i) {
                                        return Err(ApiError::Internal(format!(
                                            "{} chunk of `{}` is missing.",
                                            match i {
                                                0 => String::from("1st"),
                                                1 => String::from("2nd"),
                                                2 => String::from("3rd"),
                                                n => format!("{}th", n + 1),
                                            },
                                            file_path.display()
                                        )));
                                    }
                                }
                            },
                            None if chunk_count != 0 => {
                                return Err(ApiError::Internal(format!("Some chunks in `{}` are missing: expected {chunk_count} chunks, got no chunks.", file_path.display())));
                            },
                            None => {},
                        }

                        if let Some(file) = staged_files.pop() {
                            println!("build_worker: Sending new file {:?} to worker {}", file, worker_index);
                            let file_path_new = file.clone();
                            buffer.insert(file_path_new.clone(), HashMap::new());
                            curr_processing_file.insert(worker_index, file_path_new.clone());
                            worker.send(WorkerRequest::BuildChunks { file: file_path_new, dry_run_llm }).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                        }

                        else {
                            println!("build_worker: No more files to process, killing worker {}", worker_index);
                            worker.send(WorkerRequest::Kill).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                            killed_workers.push(worker_index);
                        }

                        curr_completed_files.push(file_path);
                        success += 1;
                    },
                    WorkerResponse::Error(e) => {
                        println!("build_worker: Received error from worker {}: {:?}", worker_index, e);
                        if let Some(file) = curr_processing_file.get(&worker_index) {
                            errors.push((file.clone(), format!("{e:?}")));

                            // clean up garbages of the failed file
                            let chunk_uids = buffer.get(file).unwrap().iter().map(
                                |(_, uid)| *uid
                            ).collect::<Vec<_>>();

                            for chunk_uid in chunk_uids.iter() {
                                let chunk_path = index_get_uid_path(index,
                                    index.root_dir.to_str().unwrap(),
                                    CHUNK_DIR_NAME,
                                    *chunk_uid,
                                    Some("chunk"),
                                )?;
                                remove_file(chunk_path.to_str().unwrap())?;
                            }

                            buffered_chunk_count -= chunk_uids.len();
                            buffer.remove(file);
                        }

                        // very small QoL hack: if there's no api key, every file will
                        // fail with the same error. We escape before that happens
                        if matches!(e, ApiError::ApiKeyNotFound { .. }) && success == 0 {
                            return Err(e);
                        }

                        if let Some(file) = staged_files.pop() {
                            println!("build_worker: Sending new file {:?} to worker {}", file, worker_index);
                            buffer.insert(file.clone(), HashMap::new());
                            curr_processing_file.insert(worker_index, file.clone());
                            worker.send(WorkerRequest::BuildChunks { file: file.clone(), dry_run_llm }).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                        }

                        else {
                            println!("build_worker: No more files to process, killing worker {}", worker_index);
                            worker.send(WorkerRequest::Kill).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                            killed_workers.push(worker_index);
                        }
                    },
                },
                Err(TryRecvError::Empty) => {},
                Err(TryRecvError::Disconnected) => {
                    if !killed_workers.contains(&worker_index) {
                        println!("build_worker: Worker {} disconnected unexpectedly", worker_index);
                        return Err(ApiError::MPSCError(String::from("Build worker hung up.")));
                    }
                },
            }
        }

        // It flushes and commits 20 or more files at once.
        // TODO: this number has to be configurable
        if curr_completed_files.len() >= 20 || killed_workers.len() == workers.len() {
            println!("build_worker: Flushing buffer");
            index.staged_files = index.staged_files.drain(..).filter(
                |staged_file| !curr_completed_files.contains(staged_file)
            ).collect();
            let mut ii_buffer = HashMap::new();

            for file in curr_completed_files.iter() {
                let real_path = index_get_data_path(index,
                    &index.root_dir,
                    &file.to_path_buf(),
                )?;

                if index.processed_files.contains_key(file) {
                    index_remove(index).await?;
                }

                let file_uid = uid_new_file(&index.root_dir.to_str().unwrap(), real_path.to_str().unwrap())?;
                let mut chunk_uids = buffer.get(file).unwrap().iter().map(
                    |(chunk_index, uid)| (*chunk_index, *uid)
                ).collect::<Vec<_>>();
                chunk_uids.sort_by_key(|(chunk_index, _)| *chunk_index);
                let chunk_uids = chunk_uids.into_iter().map(|(_, chunk_uid)| chunk_uid).collect::<Vec<_>>();
                index_add_file_index(index,file_uid, &chunk_uids)?;
                index_processed_files_insert(index,file.clone(), file_uid);

                match index.ii_status {
                    IIStatus::Complete => {
                        for chunk_uid in chunk_uids.iter() {
                            index_update_ii_buffer(index,&mut ii_buffer, *chunk_uid)?;
                        }
                    },
                    IIStatus::Ongoing(_)
                    | IIStatus::Outdated => {
                        index.ii_status = IIStatus::Outdated;
                    },
                    IIStatus::None => {},
                }

                buffer.remove(file);
            }

            if let IIStatus::Complete = index.ii_status {
                index_flush_ii_buffer(index,ii_buffer)?;
            }

            index.chunk_count += buffered_chunk_count;
            index_reset_uid(index,false /* save to file */)?;
            index_save_to_file(index, index.root_dir.join(INDEX_FILE_NAME).into())?;

            buffered_chunk_count = 0;
            curr_completed_files = vec![];
            flush_count += 1;

            if killed_workers.len() == workers.len() {
                if !quiet {
                    render_build_dashboard(
                        index,
                        &buffer,
                        &curr_completed_files,
                    &errors.iter().map(|(path, msg)| (path.to_string_lossy().into_owned(), msg.clone())).collect::<Vec<(String, String)>>(),
                        started_at.clone(),
                        flush_count,
                        has_to_erase_lines,
                        max_iterations,
                    );
                }

                break;
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    println!("build_worker: Finishing");
    index.curr_processing_file = None;
    index_save_to_file(index, index.root_dir.join(INDEX_FILE_NAME).into())?;
    index_calculate_and_save_uid(index)?;

    // 1. If there's an error, the knowledge-base is incomplete. We should not create a summary.
    // 2. If there's no success and no error and we already have a summary, then
    //    `index.get_summary().is_none()` would be false, and we'll not create a summary.
    // 3. If there's no success and no error but we don't have a summary yet, we have to create one
    //    because a successful `rag build` must create a summary.
    if index.build_config.summary_after_build && index.get_summary().is_none() && errors.is_empty() {
        if !quiet {
            println!("Creating a summary of the knowledge-base...");
        }

        index.get_summary();
    }

    Ok(BuildResult {
        success,
        errors,
    })
}

