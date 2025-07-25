use serde::{Deserialize, Serialize};
use ragit_types::{Uid, Chunk};
use ragit_types::chunk::rendered_chunk::RenderedChunk;
use ragit_model_query_response::QueryResponse;
use ragit_utils::file_tree::FileTree;
use crate::search_type_enum::SearchType;

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
    SimpleRag(QueryResponse),
}
