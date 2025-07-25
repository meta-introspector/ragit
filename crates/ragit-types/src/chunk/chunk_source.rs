use crate::chunk::chunk_struct::Chunk;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
pub enum ChunkSource {
    File {
        path: String,
        index: usize,
        page: Option<usize>,
    },
    #[default]
    Dummy,
    Merge {
        pre: Box<Chunk>,
        post: Box<Chunk>,
    },
}

impl ChunkSource {
    pub fn is_file(&self) -> bool {
        matches!(self, ChunkSource::File { .. })
    }

    pub fn get_file_path(&self) -> &str {
        if let ChunkSource::File { path, .. } = self {
            path
        } else {
            ""
        }
    }

    pub fn get_file_index(&self) -> usize {
        if let ChunkSource::File { index, .. } = self {
            *index
        } else {
            0
        }
    }

    pub fn get_file_page(&self) -> Option<usize> {
        if let ChunkSource::File { page, .. } = self {
            *page
        } else {
            None
        }
    }
}
