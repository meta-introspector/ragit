use crate::chunk::chunk_trait::ChunkLike;
use serde::{Deserialize, Serialize};
use crate::chunk::chunk_source::ChunkSource;
use crate::processed_doc::ProcessedDoc;
use crate::uid::Uid;

use crate::fixed_types::fixed_string::FixedString;
use crate::fixed_types::fixed_vec::FixedVec;

// Constants for FixedChunkBuildInfo string sizes
const CHUNK_BUILD_INFO_KEY_SIZE: usize = 128;
const CHUNK_BUILD_INFO_PROMPT_HASH_SIZE: usize = 128;
const CHUNK_BUILD_INFO_MODEL_SIZE: usize = 128;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedChunkBuildInfo {
    pub file_reader_key: FixedString<CHUNK_BUILD_INFO_KEY_SIZE>,
    pub prompt_hash: FixedString<CHUNK_BUILD_INFO_PROMPT_HASH_SIZE>,
    pub model: FixedString<CHUNK_BUILD_INFO_MODEL_SIZE>,
}

impl FixedChunkBuildInfo {
    pub fn new(file_reader_key: &str, prompt_hash: &str, model: &str) -> Self {
        FixedChunkBuildInfo {
            file_reader_key: file_reader_key.into(),
            prompt_hash: prompt_hash.into(),
            model: model.into(),
        }
    }
}

impl Default for FixedChunkBuildInfo {
    fn default() -> Self {
        FixedChunkBuildInfo {
            file_reader_key: FixedString::new(),
            prompt_hash: FixedString::new(),
            model: FixedString::new(),
        }
    }
}

impl From<crate::chunk::chunk_struct::ChunkBuildInfo> for FixedChunkBuildInfo {
    fn from(info: crate::chunk::chunk_struct::ChunkBuildInfo) -> Self {
        FixedChunkBuildInfo {
            file_reader_key: info.file_reader_key.into(),
            prompt_hash: info.prompt_hash.into(),
            model: info.model.into(),
        }
    }
}

// Constants for FixedChunk sizes
const CHUNK_DATA_SIZE: usize = 2048; // 25 lines * 80 chars + buffer
const CHUNK_IMAGES_CAPACITY: usize = 16;
const CHUNK_TITLE_SIZE: usize = 256;
const CHUNK_SUMMARY_SIZE: usize = 256;
const CHUNK_MUSE_SUMMARY_KEY_SIZE: usize = 128;
const CHUNK_MUSE_SUMMARY_VALUE_SIZE: usize = 128;
const CHUNK_MUSE_SUMMARY_CAPACITY: usize = 8;
const CHUNK_FILE_SIZE: usize = 512;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedChunk {
    pub data: FixedString<CHUNK_DATA_SIZE>,
    pub images: FixedVec<Uid, CHUNK_IMAGES_CAPACITY>,
    pub char_len: usize,
    pub image_count: usize,
    pub title: FixedString<CHUNK_TITLE_SIZE>,
    pub summary: FixedString<CHUNK_SUMMARY_SIZE>,
    // pub muse_summaries: FixedHashMap<CHUNK_MUSE_SUMMARY_KEY_SIZE, CHUNK_MUSE_SUMMARY_VALUE_SIZE, CHUNK_MUSE_SUMMARY_CAPACITY>,
    pub file: FixedString<CHUNK_FILE_SIZE>,
    pub index: usize,
    pub source: ChunkSource,
    pub uid: Uid,
    pub build_info: FixedChunkBuildInfo,
    pub timestamp: i64,
    pub searchable: bool,
}

impl FixedChunk {
    pub fn render_source(&self) -> String {
        format!("Source: {:?}", self.source)
    }

    pub fn dummy() -> Self {
        Self {
            data: FixedString::new(),
            images: FixedVec::new(),
            char_len: 0,
            image_count: 0,
            title: FixedString::new(),
            summary: FixedString::new(),
//            muse_summaries: FixedHashMap::new(),
            file: FixedString::new(),
            index: 0,
            source: ChunkSource::default(),
            uid: Uid::dummy(),
            build_info: FixedChunkBuildInfo::default(),
            timestamp: 0,
            searchable: false,
        }
    }
}

impl From<crate::chunk::chunk_struct::Chunk> for FixedChunk {
    fn from(chunk: crate::chunk::chunk_struct::Chunk) -> Self {
        FixedChunk {
            data: chunk.data.into(),
            images: chunk.images.into(),
            char_len: chunk.char_len,
            image_count: chunk.image_count,
            title: chunk.title.into(),
            summary: chunk.summary.into(),
            muse_summaries: chunk.muse_summaries.into(),
            file: chunk.file.into(),
            index: chunk.index,
            source: chunk.source,
            uid: chunk.uid,
            build_info: chunk.build_info.into(),
            timestamp: chunk.timestamp,
            searchable: chunk.searchable,
        }
    }
}

impl From<ProcessedDoc> for FixedChunk {
    fn from(doc: ProcessedDoc) -> Self {
        FixedChunk {
            data: doc.tokens.join(" ").into(),
            uid: doc.doc_id,
            images: FixedVec::new(),
            char_len: 0,
            image_count: 0,
            title: FixedString::new(),
            summary: FixedString::new(),
//            muse_summaries: FixedHashMap::new(),
            file: FixedString::new(),
            index: 0,
            source: ChunkSource::default(),
            build_info: FixedChunkBuildInfo::default(),
            timestamp: 0,
            searchable: false,
        }
    }
}

//use super::chunk_trait::ChunkLike;

impl ChunkLike for FixedChunk {
    fn uid(&self) -> &Uid {
        &self.uid
    }

    fn char_len(&self) -> usize {
        self.char_len
    }

    fn image_count(&self) -> usize {
        self.image_count
    }

    fn render_source(&self) -> String {
        self.render_source()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk::Chunk;
    use crate::chunk::chunk_struct::ChunkBuildInfo as OriginalChunkBuildInfo;
    use crate::uid::Uid;
    use std::collections::HashMap;

    #[test]
    fn test_fixed_chunk_build_info_from_original() {
        let original = OriginalChunkBuildInfo::new(
            String::from("reader_key"),
            String::from("prompt_hash"),
            String::from("model_name"),
        );
        let fixed: FixedChunkBuildInfo = original.into();
        assert_eq!(fixed.file_reader_key.as_str(), "reader_key");
        assert_eq!(fixed.prompt_hash.as_str(), "prompt_hash");
        assert_eq!(fixed.model.as_str(), "model_name");
    }

    #[test]
    fn test_fixed_chunk_from_original_chunk() {
        let mut original_muse_summaries = HashMap::new();
        original_muse_summaries.insert(String::from("key1"), String::from("value1"));
        original_muse_summaries.insert(String::from("key2"), String::from("value2"));

        let original_chunk = Chunk {
            data: String::from("some chunk data"),
            images: vec![Uid::dummy(), Uid::dummy()],
            char_len: 17,
            image_count: 2,
            title: String::from("Chunk Title"),
            summary: String::from("Chunk Summary"),
            muse_summaries: original_muse_summaries,
            file: String::from("/path/to/file.txt"),
            index: 1,
            source: ChunkSource::default(),
            uid: Uid::dummy(),
            build_info: OriginalChunkBuildInfo::new(
                String::from("reader"),
                String::from("hash"),
                String::from("model"),
            ),
            timestamp: 12345,
            searchable: true,
        };

        let fixed_chunk: FixedChunk = original_chunk.into();

        assert_eq!(fixed_chunk.data.as_str(), "some chunk data");
        assert_eq!(fixed_chunk.images.len(), 2);
        assert_eq!(fixed_chunk.char_len, 17);
        assert_eq!(fixed_chunk.image_count, 2);
        assert_eq!(fixed_chunk.title.as_str(), "Chunk Title");
        assert_eq!(fixed_chunk.summary.as_str(), "Chunk Summary");
        assert_eq!(fixed_chunk.muse_summaries.len(), 2);
        assert_eq!(fixed_chunk.muse_summaries.get("key1"), Some("value1"));
        assert_eq!(fixed_chunk.muse_summaries.get("key2"), Some("value2"));
        assert_eq!(fixed_chunk.file.as_str(), "/path/to/file.txt");
        assert_eq!(fixed_chunk.index, 1);
        assert_eq!(fixed_chunk.source, ChunkSource::default());
        assert_eq!(fixed_chunk.uid, Uid::dummy());
        assert_eq!(fixed_chunk.build_info.file_reader_key.as_str(), "reader");
        assert_eq!(fixed_chunk.timestamp, 12345);
        assert_eq!(fixed_chunk.searchable, true);
    }

    #[test]
    fn test_fixed_chunk_from_processed_doc() {
        let doc = ProcessedDoc {
            doc_id: Uid::dummy(),
            tokens: vec!["token1".to_string(), "token2".to_string()],
            // other fields are not used in this conversion
        };

        let fixed_chunk: FixedChunk = doc.into();

        assert_eq!(fixed_chunk.data.as_str(), "token1 token2");
        assert_eq!(fixed_chunk.uid, Uid::dummy());
        assert_eq!(fixed_chunk.images.len(), 0);
        assert_eq!(fixed_chunk.title.as_str(), "");
    }
}
