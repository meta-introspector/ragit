use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryConfig {
    pub search_chunk: bool,
    pub search_image: bool,
    pub search_file_uid: bool,
    pub search_file_path: bool,
    pub search_staged_file: bool,
}

impl QueryConfig {
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
