use super::{AtomicToken, BuildConfig, Error, FileReaderImpl};
use std::collections::VecDeque;

pub struct PlainTextReader {
    _path: String,
    _root_dir: String,
    _config: BuildConfig,
    _tokens: VecDeque<AtomicToken>,
}

impl FileReaderImpl for PlainTextReader {
    fn new(path: &str, root_dir: &str, config: &BuildConfig) -> Result<Self, Error> {
        let mut reader = PlainTextReader {
            _path: path.to_string(),
            _root_dir: root_dir.to_string(),
            _config: config.clone(),
            _tokens: VecDeque::new(),
        };
        reader.load_tokens()?;
        Ok(reader)
    }

    fn pop_all_tokens(&mut self) -> Result<Vec<AtomicToken>, Error> {
        Ok(self._tokens.drain(..).collect())
    }

    fn load_tokens(&mut self) -> Result<(), Error> {
        use std::fs;
        use std::io::Read;
        use anyhow::Context;

        let mut file = fs::File::open(&self._path)
            .context(format!("Failed to open plain text file: {}", self._path))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context(format!("Failed to read plain text file: {}", self._path))?;

        let chunk_size = self._config.chunk_size;
        let mut current_char_index = 0;
        let chars: Vec<char> = contents.chars().collect();

        while current_char_index < chars.len() {
            let end_index = (current_char_index + chunk_size).min(chars.len());
            let chunk_chars = &chars[current_char_index..end_index];
            let chunk_string: String = chunk_chars.iter().collect();

            self._tokens.push_back(AtomicToken::String {
                data: chunk_string.clone(),
                char_len: chunk_string.chars().count(),
            });
            current_char_index = end_index;
        }
        Ok(())
    }

    fn has_more_to_read(&self) -> bool {
        false
    }

    fn key(&self) -> String {
        "plain_text".to_string()
    }
}