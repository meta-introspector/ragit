use super::{AtomicToken, BuildConfig, Error, FileReaderImpl};
use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Image;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ImageDescription;

pub struct ImageReader {
    _path: String,
    _root_dir: String,
    _config: BuildConfig,
    _tokens: VecDeque<AtomicToken>,
}

impl FileReaderImpl for ImageReader {
    fn new(path: &str, root_dir: &str, config: &BuildConfig) -> Result<Self, Error> {
        Ok(ImageReader {
            _path: path.to_string(),
            _root_dir: root_dir.to_string(),
            _config: config.clone(),
            _tokens: VecDeque::new(),
        })
    }

    fn pop_all_tokens(&mut self) -> Result<Vec<AtomicToken>, Error> {
        Ok(self._tokens.drain(..).collect())
    }

    fn load_tokens(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn has_more_to_read(&self) -> bool {
        false
    }

    fn key(&self) -> String {
        "image".to_string()
    }
}