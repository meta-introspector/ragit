pub mod helpers;
pub mod prelude;
pub mod get_tfidf_by_chunk_uid;
pub mod get_chunk_by_uid;
pub mod get_chunks_of_file;
pub mod summaries_to_chunks;
pub mod load_all_chunks;
pub mod load_chunks_from_uids;
pub mod extract_keywords;

pub mod file_retrieval_methods;


pub mod utils;
pub mod query_helpers;

use ragit_types::file_schema::FileSchema;
use ragit_index_types::index_struct::Index;
use ragit_types::uid::Uid;
use std::path::PathBuf;
use anyhow::Result;
use ragit_error::ApiError;
use ragit_types::image::ImageSchema;

pub fn get_file_schema(
    _index: &Index,
    _path: Option<PathBuf>,
    _uid: Option<Uid>,
) -> Result<FileSchema, ApiError> {
    eprintln!("Placeholder for get_file_schema");
    Ok(FileSchema::dummy())
}

pub fn get_image_schema(
    _index: &Index,
    _image_uid: Uid,
    _with_bytes: bool,
) -> Result<ImageSchema, ApiError> {
    eprintln!("Placeholder for get_image_schema");
    Ok(ImageSchema::dummy())
}