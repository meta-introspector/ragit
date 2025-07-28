use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use std::time::Instant;
use crate::build_worker::build_worker;
use crate::init_workers::init_workers;

pub async fn build(
    index: &mut Index,
    workers_count: usize,
    quiet: bool,
    dry_run_llm: bool,
    max_iterations: Option<usize>,
) -> Result<BuildResult, ApiError> {
    let mut workers = init_workers(workers_count, index.root_dir.clone());
    build_worker(index, &mut workers, Instant::now(), quiet, dry_run_llm, max_iterations).await
}

pub struct BuildResult {
    pub success: usize,

    /// Vec<(file, error)>
    pub errors: Vec<(std::path::PathBuf, String)>,
}