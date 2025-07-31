use crate::prelude::*;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RenderedChunk {
    // You can feed `pdl_data` directly to a pdl context.
    // Make sure to use a safe filter, like `{{chunk.pdl_data|safe}}`.
    pub pdl_data: String,

    // Human-readable data. It's used in `rag cat-file`, ragit-server's
    // file-content api, agent's file reader, and many more places.
    pub human_data: String,
    pub raw_data: Vec<MultiModalContent>,
    pub source: String,
}

impl RenderedChunk {
    pub fn fake(data: String, source: String) -> Self {
        RenderedChunk {
            pdl_data: data.to_string(),
            human_data: data.to_string(),
            raw_data: vec![MultiModalContent::Text { content: data }],
            source,
        }
    }
}


