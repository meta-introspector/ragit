use crate::index_struct::Index;
use ragit_error::ApiError;

impl Index {
    pub async fn build(&mut self, jobs: usize, quiet: bool) -> Result<(), ApiError> {
        eprintln!("Placeholder for build: jobs={}, quiet={}", jobs, quiet);
        Ok(())
    }
}