use serde::{Deserialize, Serialize};
use ragit_types::{Uid, Chunk};
use ragit_types::chunk::rendered_chunk::RenderedChunk;
use ragit_model_query_response::ModelQueryResponse;
use ragit_agent::file_tree::FileTree;
use crate::search_type_enum::SearchType;

#[derive(Clone, Debug, Serialize)]
pub enum ActionResult {
    // ... (existing variants)
}

#[derive(Clone, Debug, Serialize)]
pub enum ActionResult {
    // If the file is short enough, it'll merge its chunks into one.
    ReadFileShort {
        chunk_uids: Vec<Uid>,
        rendered: RenderedChunk,
    },
    ReadFileLong(Vec<Chunk>),
    NoSuchFile {
        file: String,
        similar_files: Vec<String>,
    },

    ReadDir(FileTree),
    NoSuchDir {
        dir: String,
        similar_dirs: Vec<String>,
    },
    ReadChunk(Chunk),
    NoSuchChunk(String),
    ReadChunkAmbiguous {
        query: String,
        chunks: Vec<Chunk>,
    },
    ReadChunkTooMany {
        query: String,
        chunk_uids: usize,
    },
    Search {
        r#type: SearchType,
        keyword: String,
        chunks: Vec<Chunk>,
    },
    GetMeta {
        key: String,
        value: String,
    },
    NoSuchMeta {
        key: String,
        similar_keys: Vec<String>,
    },
    GetSummary(String),
    SimpleRag(ModelQueryResponse),
}

impl ActionResult {
    pub fn render(&self) -> String {
        match self {
            ActionResult::ReadFileShort { rendered, .. } => {
                crate::action_result_render::render_read_file_short::render_read_file_short(rendered)
            }
            ActionResult::ReadFileLong(chunks) => {
                crate::action_result_render::render_read_file_long::render_read_file_long(chunks)
            }
            ActionResult::NoSuchFile { file, similar_files } => {
                crate::action_result_render::render_no_such_file::render_no_such_file(file, similar_files)
            }
            ActionResult::ReadDir(file_tree) => {
                crate::action_result_render::render_read_dir::render_read_dir(file_tree)
            }
            ActionResult::NoSuchDir { dir, similar_dirs } => {
                crate::action_result_render::render_no_such_dir::render_no_such_dir(dir, similar_dirs)
            }
            ActionResult::ReadChunk(chunk) => {
                crate::action_result_render::render_read_chunk::render_read_chunk(chunk)
            }
            ActionResult::NoSuchChunk(query) => {
                crate::action_result_render::render_no_such_chunk::render_no_such_chunk(query)
            }
            ActionResult::ReadChunkAmbiguous { query, chunks } => {
                crate::action_result_render::render_read_chunk_ambiguous::render_read_chunk_ambiguous(query, chunks)
            }
            ActionResult::ReadChunkTooMany { query, chunk_uids } => {
                crate::action_result_render::render_read_chunk_too_many::render_read_chunk_too_many(query, *chunk_uids)
            }
            ActionResult::Search { r#type, keyword, chunks } => {
                crate::action_result_render::render_search::render_search(r#type, keyword, chunks)
            }
            ActionResult::GetMeta { key, value } => {
                crate::action_result_render::render_get_meta::render_get_meta(key, value)
            }
            ActionResult::NoSuchMeta { key, similar_keys } => {
                crate::action_result_render::render_no_such_meta::render_no_such_meta(key, similar_keys)
            }
            ActionResult::GetSummary(summary) => {
                crate::action_result_render::render_get_summary::render_get_summary(summary)
            }
            ActionResult::SimpleRag(response) => {
                crate::action_result_render::render_simple_rag::render_simple_rag(response)
            }
        }
    }
}


