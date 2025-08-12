use std::path::PathBuf;

#[derive(Debug)]
pub enum Request {
    BuildChunks { file: PathBuf },
    Kill,
}
