use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use std::path::PathBuf;
use tokio::sync::mpsc;
use ragit_readers::FileReader;
use ragit_types::{ChunkBuildInfo};
use ragit_utils::constant::{CHUNK_DIR_NAME, IMAGE_DIR_NAME};
use ragit_fs::{try_create_dir, write_bytes, WriteMode, exists, parent};
use ragit_tfidf::save_to_file;
use ragit_index_types::index_impl::{index_get_data_path, index_get_uid_path, index_add_image_description};
use crate::channel::WorkerResponse as Response;
use anyhow::Context;

pub async fn build_chunks(
    index: &mut Index,
    file: PathBuf,
    prompt_hash: String,
    tx_to_main: mpsc::UnboundedSender<Response>,
    dry_run_llm: bool,
) -> Result<(), ApiError> {
    println!("build_chunks: file received: {:?}", file);
    let real_path = index_get_data_path(index,
        &index.root_dir,
        &file,
    )?;
    println!("build_chunks: real_path obtained: {:?}", real_path);
    println!("build_chunks: Calling FileReader::new with:");
    println!("  file.to_string_lossy().into_owned(): {:?}", file.to_string_lossy().into_owned());
    println!("  real_path.to_string_lossy().into_owned(): {:?}", real_path.to_string_lossy().into_owned());
    println!("  index.root_dir.to_str().unwrap(): {:?}", index.root_dir.to_str().unwrap());
    let mut fd = FileReader::new(
        file.to_string_lossy().into_owned(),
        real_path.to_string_lossy().into_owned(),
        index.root_dir.to_str().unwrap(),
        index.build_config.clone(),
    ).await?;
    println!("build_chunks: FileReader initialized. Can generate chunk: {}", fd.can_generate_chunk());
    println!("build_chunks: Processing file: {}", file.file_name().unwrap().to_string_lossy());
    let build_info = ChunkBuildInfo::new(
        fd.file_reader_key(),
        prompt_hash.clone(),
        index.api_config.model.clone(),
    );
    let mut index_in_file = 0;
    let mut previous_summary = None;

    while fd.can_generate_chunk() {
        let new_chunk = fd.generate_chunk(
            &index,
            build_info.clone(),
            previous_summary.clone(),
            index_in_file,
            dry_run_llm,
        ).await?;
        previous_summary = Some((new_chunk.clone(), (&new_chunk).into()));
        let new_chunk_uid = new_chunk.uid;
        let new_chunk_path = index_get_uid_path(index,
            index.root_dir.to_str().unwrap(),
            CHUNK_DIR_NAME,
            new_chunk_uid,
            Some("chunk"),
        )?;

        for (uid, bytes) in fd.images.iter() {
            let image_path = index_get_uid_path(index,
                index.root_dir.to_str().unwrap(),
                IMAGE_DIR_NAME,
                *uid,
                Some("png"),
            )?;
            let parent_path = parent(image_path.as_path())
                .context(format!("Failed to get parent path for image: {:?}", image_path))?;

            if !exists(&parent_path) {
                try_create_dir(parent_path.to_str().unwrap())
                    .context(format!("Failed to create directory for image: {:?}", parent_path))?;
            }

            write_bytes(
                image_path.to_str().unwrap(),
                bytes,
                WriteMode::Atomic,
            ).context(format!("Failed to write image bytes to {:?}", image_path))?;
            index_add_image_description(index, *uid).await?;
        }

        println!("build_chunks: Saving chunk to: {:?}", new_chunk_path);
        save_to_file(
            new_chunk_path.to_str().unwrap(),
            &new_chunk,
            index.root_dir.to_str().unwrap(),
        ).context(format!("Failed to save chunk to file: {:?}", new_chunk_path))?;
        tx_to_main.send(Response::ChunkComplete {
            file: file.to_string_lossy().into_owned(),
            index: index_in_file,
            chunk_uid: new_chunk_uid,
        }).map_err(|_| ApiError::MPSCError(String::from("Failed to send response to main"))).unwrap();
        index_in_file += 1;
    }

    tx_to_main.send(Response::FileComplete {
        file: file.to_string_lossy().into_owned(),
        chunk_count: index_in_file,
    }).map_err(|_| ApiError::MPSCError(String::from("Failed to send response to main"))).unwrap();
    Ok(())
}
