use super::prelude::*;
use crate::index_struct::Index;
use ragit_tfidf::{TfidfResult, TfidfState, ProcessedDoc, consume_processed_doc};
use ragit_error::ApiError;
use ragit_utils::query::Keywords;

impl Index {
    pub fn run_tfidf_with_docs(
        &self,
        keywords: &Keywords,
        docs: Vec<ProcessedDoc>,
    ) -> Result<Vec<TfidfResult>, ApiError> {
        let mut tfidf_state = TfidfState::new(keywords);

        for processed_doc in docs {
            consume_processed_doc(processed_doc, &mut tfidf_state)?;
        }

        Ok(tfidf_state.search(keywords))
    }
}
