use crate::error::Error;
use crate::prelude::*;
use crate::query::Keywords;
use ragit_uid::Uid;

use super::{consume_processed_doc, TfidfResult, TfidfState};
use crate::index::index_struct::Index;

impl Index {
    pub fn run_tfidf(
        &self,
        keywords: Keywords,
        limit: usize,
    ) -> Result<Vec<TfidfResult<Uid>>, Error> {
        let mut tfidf_state = TfidfState::new(&keywords);

        // TODO: I'm still trying to figure out the best value for `ii_coeff`.
        //       I found that 20 is too small. 50 works on most cases, but `tests/ii.py` is still failing.
        // TODO: How about making it configurable?
        let ii_coeff = 50;

        if self.query_config.enable_ii && self.is_ii_built() {
            for chunk_uid in self.get_search_candidates(&tfidf_state.terms, limit * ii_coeff)? {
                let processed_doc = self.get_tfidf_by_chunk_uid(chunk_uid)?;
                consume_processed_doc(processed_doc, &mut tfidf_state)?;
            }
        } else {
            for tfidf_file in &self.get_all_tfidf_files()? {
                let processed_doc = super::tfidf::load_from_file(tfidf_file.to_str().unwrap())?;
                consume_processed_doc(processed_doc, &mut tfidf_state)?;
            }
        }

        Ok(tfidf_state.get_top(limit))
    }

    pub fn run_tfidf_on(
        &self,
        chunks: &[Uid],
        keywords: Keywords,
        limit: usize,
    ) -> Result<Vec<TfidfResult<Uid>>, Error> {
        let mut tfidf_state = TfidfState::new(&keywords);

        for chunk in chunks.iter() {
            let processed_doc = self.get_tfidf_by_chunk_uid(*chunk)?;
            consume_processed_doc(processed_doc, &mut tfidf_state)?;
        }

        Ok(tfidf_state.get_top(limit))
    }
}
