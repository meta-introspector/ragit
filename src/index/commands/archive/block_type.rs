#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockType {
    Index,
    Chunk,
    ImageBytes,
    ImageDesc,
    Tfidf,
    Metadata,
    Config,
    Prompt,
    Meta,
    Splitted,
}

impl BlockType {
    pub(super) fn to_byte(&self) -> u8 {
        match self {
            BlockType::Index => 0,
            BlockType::Chunk => 1,
            BlockType::ImageBytes => 2,
            BlockType::ImageDesc => 3,
            BlockType::Tfidf => 4,
            BlockType::Metadata => 5,
            BlockType::Config => 6,
            BlockType::Prompt => 7,
            BlockType::Meta => 8,
            BlockType::Splitted => 255,
        }
    }
}

impl TryFrom<u8> for BlockType {
    type Error = crate::error::Error; // Changed error type

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BlockType::Index),
            1 => Ok(BlockType::Chunk),
            2 => Ok(BlockType::ImageBytes),
            3 => Ok(BlockType::ImageDesc),
            4 => Ok(BlockType::Tfidf),
            5 => Ok(BlockType::Metadata),
            6 => Ok(BlockType::Config),
            7 => Ok(BlockType::Prompt),
            8 => Ok(BlockType::Meta),
            255 => Ok(BlockType::Splitted),
            _ => Err(crate::error::Error::BrokenArchive(format!("unknown block type: {}", value))), // Return a proper error
        }
    }
}
