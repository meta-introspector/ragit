use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::summary::SummaryMode;

impl Index {
    pub async fn summary(&mut self, mode: Option<SummaryMode>) -> Result<Option<String>, ApiError> {
        eprintln!("Placeholder for summary: mode={:?}", mode);
        Ok(None)
    }
}