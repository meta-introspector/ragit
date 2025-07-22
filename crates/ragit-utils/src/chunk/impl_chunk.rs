use std::collections::HashMap;
use std::path::PathBuf;
use crate::prelude::*;
use ragit_uid::Uid;
use crate::chunk::chunk_struct::Chunk;
use crate::chunk::chunk_source::ChunkSource;
use crate::chunk::io::save_to_file;
use crate::chunk::atomic_token::AtomicToken;

impl Chunk {
    pub fn dummy(data: String, source: ChunkSource) -> Self {
        Self {
            images: vec![],
            char_len: data.chars().count(),
            image_count: 0,
            title: String::new(),
            summary: String::new(),
            muse_summaries: HashMap::new(),
            uid: Uid::dummy(),
            timestamp: 0,
            searchable: true,
            build_info: Default::default(),
            data,
            source,
        }
    }

    pub fn render_source(&self) -> String {
        String::new()
    }

    pub fn sortable_string(&self) -> String {
        String::new()
    }

    pub(crate) fn get_approx_size(&self) -> usize {
        0
    }

    pub fn create_chunk_from(
        index: &crate::index::index_struct::Index,
        path: &PathBuf,
        data: &str,
        source: ChunkSource,
        timestamp: u64,
        searchable: bool,
        compression_threshold: u64,
        compression_level: u32,
        create_tfidf: bool,
    ) -> Result<Chunk> {
        let mut result = Chunk::dummy(data.to_string(), source);
        result.uid = Uid::new_from_slice(result.data.as_bytes());

        save_to_file(
            path,
            &result,
            compression_threshold,
            compression_level,
            &index.root_dir,
            create_tfidf,
        )?;

        Ok(result)
    }

    pub fn get_tokens_len(tokens: &[AtomicToken]) -> usize {
        let mut result = 0;

        for token in tokens {
            match token {
                AtomicToken::String { data, .. } => {
                    result += data.len();
                }
                AtomicToken::Image(i) => {
                    result += i.bytes.len();
                    result += i.image_type.to_string().len();
                }
                AtomicToken::WebImage { subst, url: _ } => {
                    result += subst.len();
                }
                AtomicToken::PageBreak
                | AtomicToken::ChunkExtraInfo(_) => {
                    // do nothing
                }
            }
        }

        result
    }

    pub fn get_tokens_char_len(tokens: &[AtomicToken]) -> usize {
        let mut result = 0;

        for token in tokens {
            match token {
                AtomicToken::String { data: s, char_len: n } => {
                    result += n;
                }
                AtomicToken::Image(i) => {
                    result += i.bytes.len();
                    result += i.image_type.to_string().len();
                }
                AtomicToken::WebImage { subst, .. } => {
                    result += subst.len();
                }
                AtomicToken::PageBreak
                | AtomicToken::ChunkExtraInfo(_) => {},
            }
        }

        result
    }
}
