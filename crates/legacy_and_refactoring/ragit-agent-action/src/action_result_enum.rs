use serde::{Deserialize, Serialize};
use ragit_types::{Uid, Chunk};
use ragit_model_query_response::ModelQueryResponse;
use ragit_agent::file_tree::FileTree;
use crate::search_type_enum::SearchType;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionResult {
    ReadFileShort {
        rendered: String,
        file_path: String,
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
        chunk_uids: Vec<Uid>,
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
        panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
        // match self {
        //     ActionResult::ReadFileShort { rendered, .. } => {
        //         render_read_file_short(rendered)
        //     }
        //     ActionResult::ReadFileLong(chunks) => {
        //         render_read_file_long(chunks)
        //     }
        //     ActionResult::NoSuchFile { file, similar_files } => {
        //         render_no_such_file(file, similar_files)
        //     }
        //     ActionResult::ReadDir(file_tree) => {
        //         render_read_dir(file_tree)
        //     }
        //     ActionResult::NoSuchDir { dir, similar_dirs } => {
        //         render_no_such_dir(dir, similar_dirs)
        //     }
        //     ActionResult::ReadChunk(chunk) => {
        //         render_read_chunk(chunk)
        //     }
        //     ActionResult::NoSuchChunk(query) => {
        //         render_no_such_chunk(query)
        //     }
        //     ActionResult::ReadChunkAmbiguous { query, chunks } => {
        //         render_read_chunk_ambiguous(query, chunks)
        //     }
        //     ActionResult::ReadChunkTooMany { query, chunk_uids } => {
        //         render_read_chunk_too_many(query, chunk_uids)
        //     }
        //     ActionResult::Search { r#type, keyword, chunks } => {
        //         render_search(r#type, keyword, chunks)
        //     }
        //     ActionResult::GetMeta { key, value } => {
        //         render_get_meta(key, value)
        //     }
        //     ActionResult::NoSuchMeta { key, similar_keys } => {
        //         render_no_such_meta(key, similar_keys)
        //     }
        //     ActionResult::GetSummary(summary) => {
        //         render_get_summary(summary)
        //     }
        //     ActionResult::SimpleRag(response) => {
        //         render_simple_rag(response)
        //     }
        // }
    }
}