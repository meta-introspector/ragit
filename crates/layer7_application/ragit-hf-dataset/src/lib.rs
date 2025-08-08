pub mod emoji_processor;
pub mod parquet_writer;
pub mod dataset_metadata;

pub use emoji_processor::EmojiDatasetProcessor;
pub use parquet_writer::EmojiParquetWriter;
pub use dataset_metadata::DatasetMetadata;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Parquet error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),
    
    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),
    
    #[error("Processing error: {0}")]
    Processing(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, ProcessingError>;
