use crate::action_result_enum::ActionResult;
use crate::search_type_enum::SearchType;
use ragit_index_types::index_struct::Index;
use ragit_index_io::get_chunk_by_uid;
use ragit_types::{ApiError, Chunk};
use ragit_utils::query::Keywords;
use ragit_utils::string_utils::substr_edit_distance;

pub(crate) async fn run_search(
    argument: &str,
    index: &Index,
    search_type: SearchType,
) -> Result<ActionResult, ApiError> {
    let mut limit = if search_type == SearchType::Exact {
        100
    } else {
        10
    };

    let chunks = 'chunks_loop: loop {
        let candidates = ragit_tfidf::TfidfState::new(&Keywords::from(vec![argument.to_string()]))
            .search(&Keywords::from(vec![argument.to_string()]));
        let mut chunks = Vec::with_capacity(candidates.len());
        let mut chunks_exact_match = vec![];

        for c in candidates.iter() {
            chunks.push(get_chunk_by_uid(index, *c)?);
        }

        if search_type == SearchType::Tfidf {
            break chunks;
        }

        for chunk in chunks.iter() {
            if chunk.content.contains(argument) {
                chunks_exact_match.push(chunk.clone());

                if chunks_exact_match.len() == 10 {
                    break 'chunks_loop chunks_exact_match;
                }
            }
        }

        // We have a complete set of the tfidf result, so there's
        // no point in increasing the limit.
        if candidates.len() < limit || limit == index.chunk_count {
            break chunks_exact_match;
        }

        // Maybe we can get more exact-matches if we increase the
        // limit of the tfidf-match.
        limit = (limit * 5).min(index.chunk_count);
    };

    Ok(ActionResult::Search { r#type: search_type, keyword: argument.to_string(), chunks })
}
