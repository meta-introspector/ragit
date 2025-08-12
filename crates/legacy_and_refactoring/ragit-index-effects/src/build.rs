use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;
use std::time::Instant;
use crate::build_worker::build_worker;
use crate::init_workers::init_workers;

use sysinfo::System;

pub async fn build(
    index: &mut Index,
    workers_count: usize,
    quiet: bool,
    dry_run_llm: bool,
    max_iterations: Option<usize>,
    sys: &mut System,
    max_memory_gb: Option<u64>,
) -> Result<BuildResult, ApiError> {
    let mut workers = init_workers(workers_count, index.root_dir.clone());
    build_worker(index, &mut workers, Instant::now(), quiet, dry_run_llm, max_iterations, sys, max_memory_gb).await
}

pub struct BuildResult {
    pub success: usize,

    /// Vec<(file, error)>
    pub errors: Vec<(std::path::PathBuf, String)>,
}