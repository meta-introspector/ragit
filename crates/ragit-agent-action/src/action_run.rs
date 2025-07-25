use super::action_enum::Action;
use crate::action_result_enum::ActionResult;
use crate::run_read_chunk::run_read_chunk;
use crate::run_read_dir::run_read_dir;
use crate::run_read_file::run_read_file;
use crate::run_search::run_search;
use crate::run_get_meta::run_get_meta;
use crate::run_get_summary::run_get_summary;
use crate::run_simple_rag::run_simple_rag;
use crate::search_type_enum::SearchType;
use ragit_index_types::index_struct::Index;
use ragit_types::ApiError;

impl Action {
    pub(crate) async fn run(&self, argument: &str, index: &Index) -> Result<ActionResult, ApiError> {
        match self {
            Action::ReadFile => run_read_file(argument, index).await,
            Action::ReadDir => run_read_dir(argument, index).await,
            Action::ReadChunk => run_read_chunk(argument, index).await,
            Action::SearchExact => run_search(argument, index, SearchType::Exact).await,
            Action::SearchTfidf => run_search(argument, index, SearchType::Tfidf).await,
            Action::GetMeta => run_get_meta(argument, index).await,
            Action::GetSummary => run_get_summary(argument, index).await,
            Action::SimpleRag => run_simple_rag(argument, index).await,
        }
    }
}