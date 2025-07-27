use ragit_types::ApiError as Error;
use ragit_types::build_config::BuildConfig;
use ragit_types::chunk::atomic_token::AtomicToken;
use ragit_types::chunk::chunk_extra_info::ChunkExtraInfo;
use ragit_index_types::chunk::chunk_struct::Chunk;
use ragit_chunk;
use ragit_index_types::index_get_prompt;

use ragit_types::uid::Uid;
use ragit_types::image::Image;

use ragit_types::image::ImageType;
use ragit_fs::extension;
use std::collections::{HashMap, VecDeque};




pub mod image;
pub mod line;
pub mod markdown;
pub mod plain_text;

#[cfg(feature = "csv")]
pub mod csv;

#[cfg(feature = "pdf")]
pub mod pdf;

pub use image::{ImageReader};
pub use line::LineReader;
pub use markdown::MarkdownReader;
pub use plain_text::PlainTextReader;
#[cfg(feature = "csv")]
pub use csv::CsvReader;
#[cfg(feature = "pdf")]
pub use pdf::PdfReader;


pub trait FileReaderImpl {
    fn new(path: &str, root_dir: &str, config: &BuildConfig) -> Result<Self, Error> where Self: Sized;
    fn pop_all_tokens(&mut self) -> Result<Vec<AtomicToken>, Error>;
    fn load_tokens(&mut self) -> Result<(), Error>;
    fn has_more_to_read(&self) -> bool;
    fn key(&self) -> String;
}

pub struct FileReader {
    rel_path: String,
    inner: Box<dyn FileReaderImpl + Send>,
    buffer: VecDeque<AtomicToken>,
    curr_buffer_size: usize,
    pub images: HashMap<Uid, Vec<u8>>,
    config: BuildConfig,
    fetched_images: HashMap<String, Uid>,
}

impl FileReader {
    pub async fn new(rel_path: String, real_path: String, root_dir: &str, config: BuildConfig) -> Result<Self, Error> {
        let inner = match extension(&rel_path)?.unwrap_or_default().to_ascii_lowercase().as_str() {
            "md" => Box::new(MarkdownReader::new(&real_path, root_dir, &config)?) as Box<dyn FileReaderImpl + Send>,
            "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => Box::new(ImageReader::new(&real_path, root_dir, &config)?),
            "jsonl" => Box::new(LineReader::new(&real_path, root_dir, &config)?),
            "csv" => {
                #[cfg(feature = "csv")]
                { Box::new(CsvReader::new(&real_path, root_dir, &config)?) }
                #[cfg(not(feature = "csv"))]
                { Box::new(PlainTextReader::new(&real_path, root_dir, &config)?) }
            },
            "pdf" => {
                #[cfg(feature = "pdf")]
                { Box::new(PdfReader::new(&real_path, root_dir, &config)?) }
                #[cfg(not(feature = "pdf"))]
                { return Err(Error::FeatureNotEnabled { feature: String::from("pdf"), action: format!("read `{rel_path}`") }); }
            },
            _ => Box::new(PlainTextReader::new(&real_path, root_dir, &config)?),
        };

        Ok(FileReader {
            rel_path,
            inner,
            buffer: VecDeque::new(),
            curr_buffer_size: 0,
            images: HashMap::new(),
            config,
            fetched_images: HashMap::new(),
        })
    }

    pub fn can_generate_chunk(&self) -> bool {
        !self.buffer.is_empty() || self.inner.has_more_to_read()
    }

    pub async fn generate_chunk(
        &mut self,
        index: &ragit_index_types::index_struct::Index, // Assuming Index is here
        build_info: ragit_types::ChunkBuildInfo,
        _previous_turn: Option<(Chunk, ragit_types::chunk::chunk_schema::ChunkSchema)>,
        index_in_file: usize,
    ) -> Result<Chunk, Error> {
        let (tokens, _chunk_extra_info) = self.next_chunk()?;
        let tokens = self.fetch_images_from_web(tokens)?; // Changed to sync for now

        let chunk = ragit_chunk::create_chunk_from(
            &tokens,
            &self.config,
            self.rel_path.clone(),
            index_in_file,
            &index.api_config,
            &index_get_prompt(index, "summarize")?,
            build_info,
        ).await?;

        // if let Some(ms) = index.api_config.sleep_after_llm_call {
        //     tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
        // }

        Ok(chunk) // Wrapped in Ok
    }

    fn next_chunk(&mut self) -> Result<(Vec<AtomicToken>, Option<ChunkExtraInfo>), Error> {
        self.fill_buffer_until_n_chunks(2)?;

        let next_chunk_size = if self.config.chunk_size < self.curr_buffer_size && self.curr_buffer_size < self.config.chunk_size * 2 {
            self.curr_buffer_size / 2
        } else {
            self.config.chunk_size
        };

        let mut chunk_deque = VecDeque::new();
        let mut curr_chunk_size = 0;
        let mut has_page_break = false;
        let mut chunk_extra_info: Option<ChunkExtraInfo> = None;

        while curr_chunk_size < next_chunk_size && !self.buffer.is_empty() {
            let token = self.buffer.pop_front().unwrap();

            if let AtomicToken::ChunkExtraInfo(extra_info) = &token {
                match &mut chunk_extra_info {
                    Some(old) => {
                        *old = old.merge(extra_info);
                    },
                    None => {
                        chunk_extra_info = Some(extra_info.clone());
                    },
                }
                continue;
            }

            if let AtomicToken::PageBreak = &token {
                has_page_break = true;
                break;
            }

            self.curr_buffer_size -= token.len(self.config.image_size);
            curr_chunk_size += token.len(self.config.image_size);
            chunk_deque.push_back(token);
        }

        if !has_page_break && (!self.buffer.is_empty() || chunk_deque.len() == 1) {
            let mut sliding_window_deque = VecDeque::new();
            let mut curr_sliding_window_size = 0;

            while curr_sliding_window_size < self.config.slide_len && !chunk_deque.is_empty() {
                let token = chunk_deque.pop_back().unwrap();
                curr_sliding_window_size += token.len(self.config.image_size);
                self.buffer.push_front(token.clone());
                self.curr_buffer_size += token.len(self.config.image_size);
                sliding_window_deque.push_front(token);
            }

            if curr_sliding_window_size == curr_chunk_size {
                let token = self.buffer.pop_front().unwrap();
                self.curr_buffer_size -= token.len(self.config.image_size);
            }

            for token in sliding_window_deque.into_iter() {
                chunk_deque.push_back(token);
            }
        }

        let tokens = merge_tokens(chunk_deque);

        for token in tokens.iter() {
            if let AtomicToken::Image(Image { uid, bytes, .. }) = token {
                self.images.insert(*uid, bytes.clone());
            }
        }

        Ok((tokens, chunk_extra_info))
    }

    fn fill_buffer_until_n_chunks(&mut self, n: usize) -> Result<(), Error> {
        loop {
            if self.curr_buffer_size >= n * self.config.chunk_size {
                return Ok(());
            }

            self.inner.load_tokens()?;

            for token in self.inner.pop_all_tokens()? {
                self.curr_buffer_size += token.len(self.config.image_size);
                self.buffer.push_back(token);
            }

            if !self.inner.has_more_to_read() {
                return Ok(());
            }
        }
    }

    pub fn file_reader_key(&self) -> String {
        self.inner.key()
    }

    fn fetch_images_from_web(&mut self, tokens: Vec<AtomicToken>) -> Result<Vec<AtomicToken>, Error> {
        let mut new_tokens = Vec::with_capacity(tokens.len());

        for token in tokens.into_iter() {
            match &token {
                AtomicToken::WebImage { url, subst } => {
                    if let Some(uid) = self.fetched_images.get(url) {
                        let bytes = self.images.get(uid).unwrap();
                        new_tokens.push(AtomicToken::Image(Image {
                            bytes: bytes.to_vec(),
                            image_type: ImageType::Png,
                            uid: *uid,
                        }));
                    } else {
                        // Placeholder for web fetching logic
                        // This part would typically involve async operations
                        // For now, just use the substitution text
                        new_tokens.push(AtomicToken::String {
                            data: subst.clone(),
                            char_len: subst.chars().count(),
                        });
                    }
                },
                _ => { new_tokens.push(token); },
            }
        }
        Ok(new_tokens)
    }
}

fn merge_tokens(tokens: VecDeque<AtomicToken>) -> Vec<AtomicToken> {
    let mut buffer = vec![];
    let mut result = vec![];

    for token in tokens.into_iter() {
        match token {
            AtomicToken::String { data, .. } => { buffer.push(data); },
            AtomicToken::Image(_) | AtomicToken::WebImage { .. } => {
                if !buffer.is_empty() {
                    let s = buffer.concat();
                    result.push(AtomicToken::String {
                        char_len: s.chars().count(),
                        data: s,
                    });
                    buffer = vec![];
                }
                result.push(token);
            },
            AtomicToken::PageBreak | AtomicToken::ChunkExtraInfo(_) => {},
        }
    }

    if !buffer.is_empty() {
        let s = buffer.concat();
        result.push(AtomicToken::String {
            char_len: s.chars().count(),
            data: s,
        });
    }
    result
}
