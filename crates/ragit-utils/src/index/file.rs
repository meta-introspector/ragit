pub mod image;
pub mod line;
pub mod markdown;
pub mod plain_text;

pub use image::{Image, ImageDescription, ImageReader};
pub use line::LineReader;
pub use markdown::MarkdownReader;
pub use plain_text::PlainTextReader;

pub trait FileReader {}
