
#[derive(Debug)]
pub enum Response {
    FileComplete { file: PathBuf, chunk_count: usize },
    ChunkComplete { file: PathBuf, index: usize, chunk_uid: Uid },
    Error(ApiError),
}
