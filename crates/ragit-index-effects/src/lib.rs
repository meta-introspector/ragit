use ragit_index_types::load_mode::LoadMode;
use ragit_index_types::index_struct::Index;
use ragit_error::ApiError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Instant, Duration};
use ragit_types::uid::Uid;
use ragit_types::ii_status::IIStatus;
use ragit_fs::{remove_file, try_create_dir, write_bytes, WriteMode, exists, parent};
use ragit_utils::path_utils;
use ragit_tfidf::io::load_from_file;
use tokio::sync::mpsc;
use ragit_utils::constant::{CHUNK_DIR_NAME, IMAGE_DIR_NAME, INDEX_FILE_NAME};
use sha3::{Digest, Sha3_256};
use ragit_types::chunk;
use ragit_api::audit::AuditRecord;
use ragit_types::ChunkBuildInfo;
use ragit_tfidf::save_to_file;

pub struct BuildResult {
    pub success: usize,

    /// Vec<(file, error)>
    pub errors: Vec<(PathBuf, String)>,
}

pub async fn build_worker(
    index: &mut Index,
    workers: &mut Vec<Channel>,
    started_at: Instant,
    quiet: bool,
) -> Result<BuildResult, ApiError> {
    let mut killed_workers = vec![];
    let mut staged_files = index.staged_files.clone();
    let mut curr_completed_files = Vec::<PathBuf>::new();
    let mut success = 0;
    let mut errors = vec![];
    let mut buffered_chunk_count = 0;
    let mut flush_count = 0;

    // HashMap<file, HashMap<index in file, chunk uid>>
    let mut buffer: HashMap<PathBuf, HashMap<usize, Uid>> = HashMap::new();

    // HashMap<worker id, file>
    let mut curr_processing_file: HashMap<usize, PathBuf> = HashMap::new();

    for (worker_index, worker) in workers.iter_mut().enumerate() {
        if let Some(file) = staged_files.pop() {
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
            worker.send(Request::BuildChunks { file }).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up")))?;
        }

        else {
            worker.send(Request::Kill).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
            killed_workers.push(worker_index);
        }
    }

    index.save_to_file(index.root_dir.join(INDEX_FILE_NAME).into())?;
    let mut has_to_erase_lines = false;

    loop {
        if !quiet {
            render_build_dashboard(
                index,
                &buffer,
                &curr_completed_files,
                &errors,
                started_at.clone(),
                flush_count,
                has_to_erase_lines,
            );
            has_to_erase_lines = true;
        }

        for (worker_index, worker) in workers.iter_mut().enumerate() {
            if killed_workers.contains(&worker_index) {
                continue;
            }

            match worker.try_recv() {
                Ok(msg) => match msg {
                    Response::ChunkComplete { file, chunk_uid, index: chunk_index } => {
                        buffered_chunk_count += 1;

                        match buffer.get_mut(&file) {
                            Some(chunks) => {
                                if let Some(prev_uid) = chunks.insert(chunk_index, chunk_uid) {
                                    return Err(ApiError::Internal(format!("{}th chunk of {} is created twice: {prev_uid}, {chunk_uid}", chunk_index + 1, file.display())));
                                }
                            },
                            None => {
                                let mut chunks = HashMap::new();
                                chunks.insert(chunk_index, chunk_uid);
                                buffer.insert(file, chunks);
                            },
                        }
                    },
                    Response::FileComplete { file, chunk_count } => {
                        match buffer.get(&file) {
                            Some(chunks) => {
                                if chunks.len() != chunk_count {
                                    return Err(ApiError::Internal(format!("Some chunks in `{}` are missing: expected {chunk_count} chunks, got only {} chunks.", file.display(), chunks.len())));
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
                                            file.display()
                                        )));
                                    }
                                }
                            },
                            None if chunk_count != 0 => {
                                return Err(ApiError::Internal(format!("Some chunks in `{}` are missing: expected {chunk_count} chunks, got no chunks.", file.display())));
                            },
                            None => {},
                        }

                        if let Some(file) = staged_files.pop() {
                            buffer.insert(file.clone(), HashMap::new());
                            curr_processing_file.insert(worker_index, file.clone());
                            worker.send(Request::BuildChunks { file }).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                        }

                        else {
                            worker.send(Request::Kill).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                            killed_workers.push(worker_index);
                        }

                        curr_completed_files.push(file);
                        success += 1;
                    },
                    Response::Error(e) => {
                        if let Some(file) = curr_processing_file.get(&worker_index) {
                            errors.push((file.display().to_string(), format!("{e:?}")));

                            // clean up garbages of the failed file
                            let chunk_uids = buffer.get(file).unwrap().iter().map(
                                |(_, uid)| *uid
                            ).collect::<Vec<_>>();

                            for chunk_uid in chunk_uids.iter() {
                                let chunk_path = index.get_uid_path(
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
                            buffer.insert(file.clone(), HashMap::new());
                            curr_processing_file.insert(worker_index, file.clone());
                            worker.send(Request::BuildChunks { file }).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                        }

                        else {
                            worker.send(Request::Kill).map_err(|_| ApiError::MPSCError(String::from("Build worker hung up.")))?;
                            killed_workers.push(worker_index);
                        }
                    },
                },
                Err(mpsc::error::TryRecvError::Empty) => {},
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    if !killed_workers.contains(&worker_index) {
                        return Err(ApiError::MPSCError(String::from("Build worker hung up.")));
                    }
                },
            }
        }

        // It flushes and commits 20 or more files at once.
        // TODO: this number has to be configurable
        if curr_completed_files.len() >= 20 || killed_workers.len() == workers.len() {
            index.staged_files = index.staged_files.drain(..).filter(
                |staged_file| !curr_completed_files.contains(staged_file)
            ).collect();
            let mut ii_buffer = HashMap::new();

            for file in curr_completed_files.iter() {
                let real_path = index.get_data_path(
                    &index.root_dir,
                    &file.to_path_buf(),
                )?;

                if index.processed_files.contains_key(file) {
                    index.remove(
                        real_path.clone().into(),
                        false,  // dry run
                        false,  // recursive
                        false,  // auto
                        false,  // staged
                        true,   // processed
                    )?;
                }

                let file_uid = Uid::new_file(&index.root_dir.to_str().unwrap(), real_path.to_str().unwrap())?;
                let mut chunk_uids = buffer.get(file).unwrap().iter().map(
                    |(chunk_index, uid)| (*chunk_index, *uid)
                ).collect::<Vec<_>>();
                chunk_uids.sort_by_key(|(chunk_index, _)| *chunk_index);
                let chunk_uids = chunk_uids.into_iter().map(|(_, chunk_uid)| chunk_uid).collect::<Vec<_>>();
                index.add_file_index(file_uid, &chunk_uids)?;
                index.processed_files.insert(file.clone(), file_uid);

                match index.ii_status {
                    IIStatus::Complete => {
                        for chunk_uid in chunk_uids.iter() {
                            index.update_ii_buffer(&mut ii_buffer, *chunk_uid)?;
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
                index.flush_ii_buffer(ii_buffer)?;
            }

            index.chunk_count += buffered_chunk_count;
            index.reset_uid(false /* save to file */)?;
            index.save_to_file(index.root_dir.join(INDEX_FILE_NAME).into())?;

            buffered_chunk_count = 0;
            curr_completed_files = vec![];
            flush_count += 1;

            if killed_workers.len() == workers.len() {
                if !quiet {
                    render_build_dashboard(
                        index,
                        &buffer,
                        &curr_completed_files,
                        &errors,
                        started_at.clone(),
                        flush_count,
                        has_to_erase_lines,
                    );
                }

                break;
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    index.curr_processing_file = None;
    index.save_to_file(index.root_dir.join(INDEX_FILE_NAME).into())?;
    index.calculate_and_save_uid()?;

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

pub async fn build_chunks(
    index: &Index,
    file: PathBuf,
    prompt_hash: String,
    tx_to_main: mpsc::UnboundedSender<Response>,
) -> Result<(), ApiError> {
    let real_path = index.get_data_path(
        &index.root_dir,
        &file,
    )?;
    let mut fd = FileReader::new(
        file.clone(),
        real_path.to_string_lossy().into_owned(),
        index.root_dir.to_str().unwrap(),
        index.build_config.clone(),
    )?;
    let build_info = ChunkBuildInfo::new(
        fd.file_reader_key(),
        prompt_hash.clone(),

        // it's not a good idea to just use `api_config.model`.
        // different `api_config.model` might point to the same model,
        // but different `get_model_by_name().name` always refer to
        // different models
        index.get_model_by_name(&index.api_config.model)?.name,
    );
    let mut index_in_file = 0;
    let mut previous_summary = None;

    while fd.can_generate_chunk() {
        let new_chunk = fd.generate_chunk(
            &index,
            build_info.clone(),
            previous_summary.clone(),
            index_in_file,
        ).await?;
        previous_summary = Some((new_chunk.clone(), (&new_chunk).into()));
        let new_chunk_uid = new_chunk.uid;
        let new_chunk_path = index.get_uid_path(
            index.root_dir.to_str().unwrap(),
            CHUNK_DIR_NAME,
            new_chunk_uid,
            Some("chunk"),
        )?;

        for (uid, bytes) in fd.images.iter() {
            let image_path = index.get_uid_path(
                index.root_dir.to_str().unwrap(),
                IMAGE_DIR_NAME,
                *uid,
                Some("png"),
            )?;
            let parent_path = parent(image_path.as_path())?;

            if !exists(parent_path.to_str().unwrap()) {
                try_create_dir(parent_path.to_str().unwrap())?;
            }

            write_bytes(
                image_path.to_str().unwrap(),
                &bytes,
                WriteMode::Atomic,
            )?;
            index.add_image_description(*uid).await?;
        }

        save_to_file(
            &new_chunk_path,
            &new_chunk,
            index.build_config.compression_threshold,
            index.build_config.compression_level,
            &index.root_dir,
            true,  // create tfidf
        )?;;
        tx_to_main.send(Response::ChunkComplete {
            file: file.clone(),
            index: index_in_file,
            chunk_uid: new_chunk_uid,
        }).map_err(|_| ApiError::MPSCError(String::from("Failed to send response to main"))).unwrap();
        index_in_file += 1;
    }

    tx_to_main.send(Response::FileComplete {
        file: file.clone(),
        chunk_count: index_in_file,
    }).map_err(|_| ApiError::MPSCError(String::from("Failed to send response to main"))).unwrap();
    Ok(())
}

#[derive(Debug)]
pub enum Request {
    BuildChunks { file: PathBuf },
    Kill,
}

#[derive(Debug)]
pub enum Response {
    FileComplete { file: PathBuf, chunk_count: usize },
    ChunkComplete { file: PathBuf, index: usize, chunk_uid: Uid },
    Error(ApiError),
}

pub struct Channel {
    pub tx_from_main: mpsc::UnboundedSender<Request>,
    pub rx_to_main: mpsc::UnboundedReceiver<Response>,
}

impl Channel {
    pub fn send(&self, msg: Request) -> Result<(), mpsc::error::SendError<Request>> {
        self.tx_from_main.send(msg)
    }

    pub fn try_recv(&mut self) -> Result<Response, mpsc::error::TryRecvError> {
        self.rx_to_main.try_recv()
    }
}

pub fn init_workers(n: usize, root_dir: PathBuf) -> Vec<Channel> {
    (0..n).map(|_| init_worker(root_dir.clone())).collect()
}

pub fn init_worker(root_dir: PathBuf) -> Channel {
    let (tx_to_main, rx_to_main) = mpsc::unbounded_channel();
    let (tx_from_main, mut rx_from_main) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        // Each process requires an instance of `Index`, but I found
        // it too difficult to send the instance via mpsc channels.
        // So I'm just instantiating new ones here.
        // Be careful not to modify the index!
        let index = match Index::load(
            root_dir,
            LoadMode::OnlyJson,
        ) {
            Ok(index) => index,
            Err(e) => {
                let _ = tx_to_main.send(Response::Error(e));
                drop(tx_to_main);
                return;
            },
        };
        let prompt = match index.get_prompt("summarize") {
            Ok(prompt) => prompt,
            Err(e) => {
                let _ = tx_to_main.send(Response::Error(e));
                drop(tx_to_main);
                return;
            },
        };
        let mut hasher = Sha3_256::new();
        hasher.update(prompt.as_bytes());
        let prompt_hash = format!("{prompt_hash:064x}");

        while let Some(msg) = rx_from_main.recv().await {
            match msg {
                Request::BuildChunks { file } => match build_chunks(
                    &index,
                    file,
                    prompt_hash.clone(),
                    tx_to_main.clone(),
                ).await {
                    Ok(_) => {},
                    Err(e) => {
                        if tx_to_main.send(Response::Error(e)).is_err() {
                            // the parent process is dead
                            break;
                        }
                    },
                },
                Request::Kill => { break; },
            }
        }

        drop(tx_to_main);
        return;
    });

    Channel {
        rx_to_main,
        tx_from_main,
    }
}

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

    match index.api_config.get_api_usage(&index.root_dir, "create_chunk_from") {
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

pub async fn build(
    index: &mut Index,
    workers_count: usize,
    quiet: bool,
) -> Result<BuildResult, ApiError> {
    let mut workers = init_workers(workers_count, index.root_dir.clone());
    build_worker(index, &mut workers, Instant::now(), quiet).await
}
