use crate::prelude::*;
use crate::chunk::render::RenderedChunk;
use crate::error::Error;
use crate::prompts::Prompt;
use crate::query::QueryResponse;
use crate::Uid;

use ragit_utils::chunk::{ChunkBuildInfo, ChunkSource, MultiModalContent, AtomicToken};
use ragit_utils::index::index_struct::Index;
use ragit_utils::path_utils;
use ragit_utils::index::tfidf;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use flate2::read::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::io::Read;
use ragit_fs::{read_bytes, write_bytes, WriteMode, parent, exists, try_create_dir, set_extension};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chunk {
    pub uid: Uid,
    pub timestamp: u64,
    pub searchable: bool,
    pub build_info: ChunkBuildInfo,
    pub data: String,
    pub source: ChunkSource,
}

impl Chunk {
    pub fn dummy() -> Self {
        Chunk {
            uid: Uid::dummy(),
            timestamp: 0,
            searchable: true,
            build_info: ChunkBuildInfo::default(),
            data: String::new(),
            source: ChunkSource::Dummy,
        }
    }

    pub fn load_from_file(path: &PathBuf) -> Result<Chunk, Error> {
        let content = read_bytes(&path_utils::pathbuf_to_str(path))?;

        if content[0] == 0x1f && content[1] == 0x8b {
            let mut gz = GzDecoder::new(&content[1..]);
            let mut s = String::new();
            gz.read_to_string(&mut s)?;
            Ok(serde_json::from_str(&s)?)
        }

        else {
            Ok(serde_json::from_slice(&content)?)
        }
    }

    pub fn save_to_file(
        path: &PathBuf,
        chunk: &Chunk,
        compression_threshold: u64,
        compression_level: u32,
        root_dir: &PathBuf,
        create_tfidf: bool,
    ) -> Result<(), Error> {
        let parent_path = parent(&path_utils::pathbuf_to_str(path))?;

        if !exists(&path_utils::pathbuf_to_str(&parent_path)) {
            try_create_dir(&path_utils::pathbuf_to_str(&parent_path))?;
        }

        if create_tfidf {
            let tfidf_path = path_utils::str_to_pathbuf(&set_extension(&path_utils::pathbuf_to_str(path), "tfidf")?);
            tfidf::save_to_file(&tfidf_path, &chunk.data)?;
        }

        let serialized_chunk = serde_json::to_vec(chunk)?;

        if serialized_chunk.len() as u64 > compression_threshold {
            let mut gz = GzEncoder::new(&serialized_chunk[..], Compression::new(compression_level));
            let compressed_bytes = gz.finish()?;
            Ok(write_bytes(&path_utils::pathbuf_to_str(path), &compressed_bytes, WriteMode::Create)?)
        }

        else {
            Ok(write_bytes(&path_utils::pathbuf_to_str(path), &serialized_chunk, WriteMode::Create)?)
        }
    }

    pub fn create_chunk_from(
        index: &Index,
        path: &PathBuf,
        data: &str,
        source: ChunkSource,
        timestamp: u64,
        searchable: bool,
        compression_threshold: u64,
        compression_level: u32,
        create_tfidf: bool,
    ) -> Result<Chunk, Error> {
        let mut result = Chunk {
            uid: Uid::dummy(),
            timestamp,
            searchable,
            build_info: ChunkBuildInfo::default(),
            data: data.to_string(),
            source,
        };

        result.uid = Uid::new_chunk_from_chunk(&result);

        Chunk::save_to_file(
            path,
            &result,
            compression_threshold,
            compression_level,
            &index.root_dir,
            create_tfidf,
        )?;

        Ok(result)
    }

    pub fn get_approx_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_tokens_len(tokens: &[AtomicToken]) -> usize {
        let mut result = 0;

        for token in tokens {
            match token {
                AtomicToken::String { data, .. } => {
                    result += data.len();
                }
                AtomicToken::Image(ragit_utils::chunk::Image { bytes, image_type, .. }) => {
                    result += bytes.len();
                    result += image_type.to_string().len();
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

    pub async fn render(&self, index: &Index) -> Result<String, Error> {
        match &self.source {
            ChunkSource::File { path, index: _, page: _ } => {
                let mut result = String::new();
                let mut previous_chunk: Option<Chunk> = None;

                if let Some(previous_chunk_uid) = self.uid.checked_sub(1) {
                    if let Ok(chunk) = index.get_chunk_by_uid(previous_chunk_uid) {
                        previous_chunk = Some(chunk);
                    }
                }

                if let Some(previous_chunk) = previous_chunk {
                    let previous_request = previous_chunk.clone().render(index).await?;
                    let prompt = index.get_prompt("summarize")?;
                    let query = QueryResponse::new(
                        index.get_model_by_name(&index.api_config.model)?,
                        &prompt,
                        &previous_request,
                        &self.data,
                    ).await?;

                    result.push_str(&query.answer);
                }

                else {
                    result.push_str(&self.data);
                }

                Ok(result)
            }

            ChunkSource::Dummy => {
                Ok(self.data.clone())
            }

            ChunkSource::Merge { pre, post } => {
                let ChunkSource::File {
                    path: pre_path,
                    index: pre_index,
                    page: pre_page,
                } = pre.source.clone() else { unreachable!() };

                let ChunkSource::File {
                    path: post_path,
                    index: post_index,
                    page: post_page,
                } = post.source.clone() else { unreachable!() };

                let mut result = String::new();

                if pre_path == post_path && pre_index == post_index && pre_page == post_page {
                    result.push_str(&merge_overlapping_strings(
                        pre.data.as_bytes(),
                        post.data.as_bytes(),
                    ));
                }

                else {
                    result.push_str(&pre.render(index).await?);
                    result.push_str(&post.render(index).await?);
                }

                Ok(result)
            }
        }
    }
}

pub fn merge_and_convert_chunks(index: &Index, chunks: Vec<RenderedChunk>) -> Result<Vec<RenderedChunk>, Error> {
    let mut result = Vec::with_capacity(chunks.len());
    let mut current_chunk: Option<RenderedChunk> = None;

    for chunk in chunks {
        if let Some(c) = current_chunk {
            if c.can_merge(&chunk) {
                current_chunk = Some(c.merge(chunk));
            }

            else {
                result.push(c);
                current_chunk = Some(chunk);
            }
        }

        else {
            current_chunk = Some(chunk);
        }
    }

    if let Some(c) = current_chunk {
        result.push(c);
    }

    Ok(result)
}

pub fn merge_overlapping_strings(s1: &[u8], s2: &[u8]) -> String {
    let s1 = String::from_utf8_lossy(s1);
    let s2 = String::from_utf8_lossy(s2);

    let mut best_overlap = 0;
    let mut best_overlap_str = String::new();

    for i in 1..s1.len().min(s2.len()) {
        let overlap = &s1[s1.len() - i..];
        if s2.starts_with(overlap) {
            best_overlap = i;
            best_overlap_str = overlap.to_string();
        }
    }

    if best_overlap > 0 {
        format!("{}{}", &s1[..s1.len() - best_overlap], &s2)
    } else {
        format!("{}{}", s1, s2)
    }
}

pub fn into_multi_modal_contents(data: &str, images: &[Uid]) -> Vec<MultiModalContent> {
    let mut result = Vec::new();
    let mut last_idx = 0;

    for image_uid in images {
        let image_str = format!("{{{{image:{}}}}}", image_uid);
        if let Some(idx) = data[last_idx..].find(&image_str) {
            let idx = last_idx + idx;
            if idx > last_idx {
                result.push(MultiModalContent::Text { content: data[last_idx..idx].to_string() });
            }
            result.push(MultiModalContent::Image { uid: *image_uid });
            last_idx = idx + image_str.len();
        }
    }

    if last_idx < data.len() {
        result.push(MultiModalContent::Text { content: data[last_idx..].to_string() });
    }

    result
}
