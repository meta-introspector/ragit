use super::prelude::*;
use crate::index_struct::Index;
use ragit_tfidf::{TfidfResult, TfidfState};
use ragit_error::ApiError;
use ragit_utils::query::Keywords;

impl Index {
    pub fn run_tfidf(
        &self,
        keywords: &Keywords,
        _limit: usize,
    ) -> Result<Vec<TfidfResult>, ApiError> {
        let tfidf_state = TfidfState::new(keywords);

        // TODO: Implement get_all_processed_docs
        // for processed_doc in self.get_all_processed_docs()? {
        //     consume_processed_doc(processed_doc, &mut tfidf_state)?;
        // }

        Ok(tfidf_state.search(keywords))
    }
}
