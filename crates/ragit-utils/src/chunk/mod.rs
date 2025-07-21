pub mod render;
pub mod render_impl;

use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::prelude::*;
use crate::Uid;
use crate::index::file::Image;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ChunkBuildInfo {
    pub model: String,
}

impl Default for ChunkBuildInfo {
    fn default() -> Self {
        ChunkBuildInfo {
            model: String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MultiModalContent {
    Text { content: String },
    Image { uid: Uid },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RenderedChunk {
    pub pdl_data: String,
    pub human_data: String,
    pub raw_data: Vec<MultiModalContent>,
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ChunkSource {
    File { path: String, index: usize, page: Option<usize> },
    Dummy,
    Merge { pre: Box<Chunk>, post: Box<Chunk> },
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Chunk {
    pub data: String,
    pub images: Vec<Uid>,
    pub char_len: usize,
    pub image_count: usize,
    pub title: String,
    pub summary: String,
    pub muse_summaries: HashMap<String, String>,
    pub source: ChunkSource,
    pub uid: Uid,
    pub build_info: ChunkBuildInfo,
    pub timestamp: i64,
    pub searchable: bool,
}

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
            build_info: ChunkBuildInfo::default(),
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
        result.uid = Uid::new_chunk_from_chunk(&result);

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

    pub async fn render(&self, index: &crate::index::index_struct::Index) -> Result<String> {
        match &self.source {
            ChunkSource::File { path, index: _, page: _ } => {
                let mut result = String::new();
                let mut previous_chunk: Option<Chunk> = None;

                if let Some(previous_chunk_uid) = self.uid.checked_sub(Uid::new_from_u128(1)) {
                    if let Ok(chunk) = index.get_chunk_by_uid(previous_chunk_uid) {
                        previous_chunk = Some(chunk);
                    }
                }

                if let Some(previous_chunk) = previous_chunk {
                    let previous_request = previous_chunk.clone().render(index).await?;
                    let prompt = index.get_prompt("summarize")?;
                    let query = crate::query::QueryResponse::new(
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
                let mut result = String::new();

                if pre.source.is_file() && post.source.is_file() && pre.source.get_file_path() == post.source.get_file_path() && pre.source.get_file_index() == post.source.get_file_index() && pre.source.get_file_page() == post.source.get_file_page() {
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

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ChunkSchema {
    pub title: String,
    pub summary: String,
}

impl ChunkSchema {
    pub fn dummy(data: &str, len: usize) -> Self {
        Self {
            title: String::new(),
            summary: String::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            title: String::new(),
            summary: String::new(),
        }
    }

    pub fn render(&self) -> String {
        String::new()
    }
}

impl From<&Chunk> for ChunkSchema {
    fn from(chunk: &Chunk) -> ChunkSchema {
        ChunkSchema {
            title: chunk.title.clone(),
            summary: chunk.summary.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChunkExtraInfo {
    pub page_no: Option<usize>,
}

impl ChunkExtraInfo {
    pub fn merge(&self, other: &ChunkExtraInfo) -> ChunkExtraInfo {
        Self { page_no: None }
    }
}

pub fn load_from_file(path: &PathBuf) -> Result<Chunk> {
    let content = ragit_fs::read_bytes(&crate::path_utils::pathbuf_to_str(path))?;

    if content[0] == 0x1f && content[1] == 0x8b {
        let mut gz = flate2::read::GzDecoder::new(&content[1..]);
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
) -> Result<()> {
    let parent_path = ragit_fs::parent(&crate::path_utils::pathbuf_to_str(path))?;

    if !ragit_fs::exists(&crate::path_utils::pathbuf_to_str(&parent_path)) {
        ragit_fs::try_create_dir(&crate::path_utils::pathbuf_to_str(&parent_path))?;
    }

    if create_tfidf {
        let tfidf_path = crate::path_utils::str_to_pathbuf(&ragit_fs::set_extension(&crate::path_utils::pathbuf_to_str(path), "tfidf")?);
        crate::index::tfidf::save_to_file(&tfidf_path, &chunk.data)?;
    }

    let serialized_chunk = serde_json::to_vec(chunk)?;

    if serialized_chunk.len() as u64 > compression_threshold {
        let mut gz = flate2::read::GzEncoder::new(&serialized_chunk[..], flate2::Compression::new(compression_level));
        let compressed_bytes = gz.finish()?;
        Ok(ragit_fs::write_bytes(&crate::path_utils::pathbuf_to_str(path), &compressed_bytes, ragit_fs::WriteMode::Create)?)
    }

    else {
        Ok(ragit_fs::write_bytes(&crate::path_utils::pathbuf_to_str(path), &serialized_chunk, ragit_fs::WriteMode::Create)?)
    }
}

pub fn merge_and_convert_chunks(index: &crate::index::index_struct::Index, chunks: Vec<RenderedChunk>) -> Result<Vec<RenderedChunk>> {
    Err(anyhow!("Not implemented"))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AtomicToken {
    String { data: String, char_len: usize },
    Image(crate::index::file::Image),
    WebImage { subst: String, url: String },
    PageBreak,
    ChunkExtraInfo(ChunkExtraInfo),
}

pub fn escape_pdl_tokens(s: &str) -> String {
    s.to_string()
}

pub fn merge_overlapping_strings(s1: &[u8], s2: &[u8]) -> String {
    String::new()
}

pub fn into_multi_modal_contents(data: &str, images: &[Uid]) -> Vec<MultiModalContent> {
    // Placeholder implementation
    vec![MultiModalContent::Text { content: data.to_string() }]
}