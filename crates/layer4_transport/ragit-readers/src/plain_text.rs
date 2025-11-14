use super::{AtomicToken, BuildConfig, Error, FileReaderImpl};
use ragit_types::map_anyhow_error;
use std::collections::VecDeque;
use std::io::{BufReader, Read};
use std::fs::File;
use anyhow::Context;

pub struct PlainTextReader {
    _root_dir: String,
    config: BuildConfig,
    tokens: VecDeque<AtomicToken>,
    file: Option<BufReader<File>>,
    file_size: u64,
    bytes_read: u64,
}

impl FileReaderImpl for PlainTextReader {
    fn new(path: &str, root_dir: &str, config: &BuildConfig) -> Result<Self, Error> {
        let file = map_anyhow_error(File::open(path).context(format!("Failed to open plain text file: {}", path)))?;
        let file_size = map_anyhow_error(file.metadata().context("Failed to get file metadata"))?.len();
        let reader = PlainTextReader {
            _root_dir: root_dir.to_string(),
            config: config.clone(),
            tokens: VecDeque::new(),
            file: Some(BufReader::new(file)),
            file_size,
            bytes_read: 0,
        };
        Ok(reader)
    }

    fn pop_all_tokens(&mut self) -> Result<Vec<AtomicToken>, Error> {
        Ok(self.tokens.drain(..).collect())
    }

    fn load_tokens(&mut self) -> Result<(), Error> {
        let mut file = self.file.take().context("File not available")?;
        let chunk_size_bytes = self.config.chunk_size * 4; // Approximate bytes for UTF-8 chars
        let mut buffer = vec![0; chunk_size_bytes];

        let bytes_read_this_call = map_anyhow_error(file.read(&mut buffer).context("Failed to read from file"))?;
        self.bytes_read += bytes_read_this_call as u64;

        if bytes_read_this_call > 0 {
            let content = String::from_utf8_lossy(&buffer[..bytes_read_this_call]).to_string();
            self.tokens.push_back(AtomicToken::String {
                data: content.clone(),
                char_len: content.chars().count(),
            });
        }
        self.file = Some(file);
        Ok(())
    }

    fn has_more_to_read(&self) -> bool {
        !self.tokens.is_empty() || self.bytes_read < self.file_size
    }

    fn key(&self) -> String {
        "plain_text".to_string()
    }
}