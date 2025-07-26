use ragit_index_types::index_struct::Index;
use ragit_error::ApiError;
use std::path::PathBuf;
use ragit_utils::constant::INDEX_FILE_NAME;
use ragit_types::uid::Uid;
use ragit_types::ii_status::IIStatus;
use ragit_types::chunk::RemoveResult;
use ragit_utils::ragit_path_utils;

pub fn index_save_to_file(index: &mut Index, path: PathBuf) -> Result<(), ApiError> {
    index.save_to_file(path)
}

pub fn index_get_uid_path(
    index: &Index,
    root_dir: &str,
    dir_name: &str,
    uid: Uid,
    extension: Option<&str>,
) -> Result<PathBuf, ApiError> {
    index.get_uid_path(root_dir, dir_name, uid, extension)
}

pub fn index_get_data_path(
    index: &Index,
    root_dir: &PathBuf,
    file: &PathBuf,
) -> Result<PathBuf, ApiError> {
    index.get_data_path(root_dir, file)
}

pub async fn index_remove(
    index: &mut Index,
    path: PathBuf,
    dry_run: bool,
    recursive: bool,
    auto: bool,
    staged: bool,
    processed: bool,
) -> Result<RemoveResult, ApiError> {
    index.remove(path, dry_run, recursive, auto, staged, processed).await
}

pub fn uid_new_file(root_dir: &str, file_path: &str) -> Result<Uid, ApiError> {
    Uid::new_file(root_dir, file_path)
}

pub fn index_add_file_index(
    index: &mut Index,
    file_uid: Uid,
    chunk_uids: &[Uid],
) -> Result<(), ApiError> {
    index.add_file_index(file_uid, chunk_uids)
}

pub fn index_processed_files_insert(
    index: &mut Index,
    file: PathBuf,
    file_uid: Uid,
) {
    index.processed_files.insert(file, file_uid);
}

pub fn index_update_ii_buffer(
    index: &mut Index,
    ii_buffer: &mut HashMap<String, Vec<Uid>>,
    chunk_uid: Uid,
) -> Result<(), ApiError> {
    index.update_ii_buffer(ii_buffer, chunk_uid)
}

pub fn index_flush_ii_buffer(
    index: &mut Index,
    ii_buffer: HashMap<String, Vec<Uid>>,
) -> Result<(), ApiError> {
    index.flush_ii_buffer(ii_buffer)
}

pub fn index_reset_uid(index: &mut Index, save_to_file: bool) -> Result<(), ApiError> {
    index.reset_uid(save_to_file)
}

pub fn index_calculate_and_save_uid(index: &mut Index) -> Result<(), ApiError> {
    index.calculate_and_save_uid()
}

pub fn index_get_model_by_name(
    index: &Index,
    model: &ragit_types::api_config::Model,
) -> Result<ragit_types::api_config::Model, ApiError> {
    index.get_model_by_name(model)
}

pub async fn index_add_image_description(
    index: &mut Index,
    uid: Uid,
) -> Result<(), ApiError> {
    index.add_image_description(uid).await
}

pub fn index_get_prompt(
    index: &Index,
    prompt_name: &str,
) -> Result<String, ApiError> {
    match index.prompts.get(prompt_name) {
        Some(prompt) => Ok(prompt.to_string()),
        None => Err(ApiError::PromptMissing(prompt_name.to_string())),
    }
}

pub fn index_api_config_get_api_usage(
    index: &Index,
    root_dir: &PathBuf,
    name: &str,
) -> Result<HashMap<String, AuditRecord>, ApiError> {
    index.api_config.get_api_usage(root_dir, name)
}
