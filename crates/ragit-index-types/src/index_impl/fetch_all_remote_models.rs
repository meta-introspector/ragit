use crate::index_struct::Index;
use ragit_types::ApiError;
use super::fetch_remote_models::FetchResult;

impl Index {
    pub async fn fetch_all_remote_models(
        &mut self,
        existing_only: bool,
        remote: &str,
    ) -> Result<FetchResult, ApiError> {
        eprintln!(
            "Placeholder for fetch_all_remote_models: existing_only={}, remote={}",
            existing_only, remote
        );
        Ok(FetchResult { fetched: 0, updated: 0 })
    }
}