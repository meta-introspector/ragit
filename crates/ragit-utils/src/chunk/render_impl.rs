use crate::prelude::*;
use crate::chunk::{MultiModalContent, RenderedChunk};
use crate::index::index_struct::Index;
use crate::error::Error;
use crate::Uid;
use ragit_pdl::{escape_pdl_tokens, encode_base64};


impl Chunk {
    pub fn render(self, index: &Index) -> Result<RenderedChunk, Error> {
        let human_data = self.data.clone();
        let mut raw_data = vec![MultiModalContent::Text { content: self.data.clone() }];
        for image_uid in self.images.iter() {
            raw_data.push(MultiModalContent::Image { uid: *image_uid });
        }

        let mut pdl_data = vec![];

        for c in raw_data.iter() {
            match c {
                MultiModalContent::Text { content } => {
                    pdl_data.push(escape_pdl_tokens(content));
                },
                MultiModalContent::Image { uid } => {
                    pdl_data.push(format!("<|raw_media(png:{})|>", encode_base64(&index.get_image_bytes_by_uid(*uid)?)));
                },
            }
        }

        Ok(RenderedChunk {
            human_data,
            pdl_data: pdl_data.join(""),
            raw_data,
            source: self.render_source(),
        })
    }
}
