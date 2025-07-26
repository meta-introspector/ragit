use ragit_index_types::index_struct::Index;
use ragit_error::ApiError;
use std::path::PathBuf;
use tokio::sync::mpsc;
use ragit_readers::FileReader;
use ragit_types::ChunkBuildInfo;
use ragit_utils::constant::{CHUNK_DIR_NAME, IMAGE_DIR_NAME};
use ragit_fs::{remove_file, try_create_dir, write_bytes, WriteMode, exists, parent};
use ragit_tfidf::save_to_file;
use ragit_index_types::index_impl::{index_get_data_path, index_get_uid_path, index_get_model_by_name, index_add_image_description};
use ragit_utils::uid_new_file;
use crate::response::Response;

pub async fn build_chunks(
    index: &Index,
    file: PathBuf,
    prompt_hash: String,
    tx_to_main: mpsc::UnboundedSender<Response>,
) -> Result<(), ApiError> {
    let real_path = index_get_data_path(index,
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
        index_get_model_by_name(index, &index.api_config.model)?.name,
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
            let parent_path = parent(image_path.as_path())?;

            if !exists(&parent_path) {
                try_create_dir(parent_path.to_str().unwrap())?;
            }

            write_bytes(
                image_path.to_str().unwrap(),
                bytes,
                WriteMode::Atomic,
            )?;
            index_add_image_description(index,*uid).await?;
        }

        save_to_file(
            new_chunk_path.to_str().unwrap(),
            &new_chunk,
            index.root_dir.to_str().unwrap(),
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