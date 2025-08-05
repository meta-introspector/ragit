use ragit_types::ApiError;
use lazy_static::lazy_static;
use regex::Regex;
use ragit_index_types::index_struct::Index;
use ragit_uid::Uid;

lazy_static! {
    static ref UID_RE: Regex = Regex::new(r"^([0-9a-z]{1,64})$").unwrap();
}

#[derive(Clone, Debug)]
pub struct UidQueryConfig {
    pub search_chunk: bool,
    pub search_image: bool,
    pub search_file_uid: bool,
    pub search_file_path: bool,
    pub search_staged_file: bool,
}

impl UidQueryConfig {
    pub fn new() -> Self {
        Self {
            search_chunk: true,
            search_image: true,
            search_file_uid: true,
            search_file_path: true,
            search_staged_file: true,
        }
    }

    pub fn chunk_only(mut self) -> Self {
        self.search_chunk = true;
        self.search_image = false;
        self.search_file_uid = false;
        self.search_file_path = false;
        self.search_staged_file = false;
        self
    }

    pub fn image_only(mut self) -> Self {
        self.search_chunk = false;
        self.search_image = true;
        self.search_file_uid = false;
        self.search_file_path = false;
        self.search_staged_file = false;
        self
    }

    pub fn file_uid_only(mut self) -> Self {
        self.search_chunk = false;
        self.search_image = false;
        self.search_file_uid = true;
        self.search_file_path = false;
        self.search_staged_file = false;
        self
    }

    pub fn file_path_only(mut self) -> Self {
        self.search_chunk = false;
        self.search_image = false;
        self.search_file_uid = false;
        self.search_file_path = true;
        self.search_staged_file = false;
        self
    }

    pub fn file_or_chunk_only(mut self) -> Self {
        self.search_chunk = true;
        self.search_image = false;
        self.search_file_uid = true;
        self.search_file_path = false;
        self.search_staged_file = false;
        self
    }

    pub fn no_staged_file(mut self) -> Self {
        self.search_staged_file = false;
        self
    }
}

#[derive(Clone, Debug)]
pub struct UidQueryResult {
    pub chunks: Vec<Uid>,
    pub images: Vec<Uid>,
    pub processed_files: Vec<(String, Uid)>,
    pub staged_files: Vec<String>,
}

impl UidQueryResult {
    pub fn empty() -> Self {
        Self {
            chunks: vec![],
            images: vec![],
            processed_files: vec![],
            staged_files: vec![],
        }
    }

    pub fn get_chunk_uids(&self) -> Vec<Uid> {
        self.chunks.clone()
    }

    pub fn get_file_uids(&self) -> Vec<Uid> {
        self.processed_files.iter().map(|(_, uid)| *uid).collect()
    }

    pub fn get_processed_files(&self) -> Vec<(String, Uid)> {
        self.processed_files.clone()
    }

    pub fn get_staged_files(&self) -> Vec<String> {
        self.staged_files.clone()
    }

    pub fn get_image_uids(&self) -> Vec<Uid> {
        self.images.clone()
    }

    pub fn has_multiple_matches(&self) -> bool {
        self.chunks.len() + self.images.len() + self.processed_files.len() + self.staged_files.len() > 1
    }

    pub fn len(&self) -> usize {
        self.chunks.len() + self.images.len() + self.processed_files.len() + self.staged_files.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_processed_file(&self) -> Option<(String, Uid)> {
        self.processed_files.first().cloned()
    }

    pub fn get_chunk_uid(&self) -> Option<Uid> {
        self.chunks.first().cloned()
    }
}

pub fn uid_query(
    _index: &Index,
    _qs: &[String],
    _config: UidQueryConfig,
) -> Result<UidQueryResult, ApiError> {
    Ok(UidQueryResult::empty())
}
