use crate::prelude::*;
use ragit_index_types::Index;
use ragit_tfidf::TfidfResult;
use ragit_tfidf::TfidfState;
use ragit_tfidf::ProcessedDoc;
use ragit_tfidf::consume_processed_doc;
impl Index {
    pub fn run_tfidf(
        &self,
        keywords: Keywords,
        limit: usize,
	//<Uid>
    ) -> Result<Vec<TfidfResult>, ApiError> {
        let mut tfidf_state = TfidfState::new(&keywords);

        // TODO: Implement get_all_processed_docs
        // for processed_doc in self.get_all_processed_docs()? {
        //     consume_processed_doc(processed_doc, &mut tfidf_state)?;
        // }

        Ok(tfidf_state.search(&keywords))
    }

    pub fn run_tfidf_with_docs(
        &self,
        keywords: Keywords,
        docs: Vec<ProcessedDoc>,
    ) -> Result<Vec<TfidfResult<Uid>>, ApiError> {
        let mut tfidf_state = TfidfState::new(&keywords);

        for processed_doc in docs {
            consume_processed_doc(processed_doc, &mut tfidf_state)?;
        }

        Ok(tfidf_state.search(&keywords))
    }
}
