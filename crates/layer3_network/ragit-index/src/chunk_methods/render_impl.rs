use crate::chunk::{rendered_chunk::MultiModalContent, rendered_chunk::RenderedChunk};
use crate::error::Error;
use crate::index::index_struct::Index;
use crate::prelude::*;
use ragit_pdl::{encode_base64, escape_pdl_tokens};

impl Chunk {
    pub fn render(self, index: &Index) -> Result<RenderedChunk, Error> {
        let human_data = self.data.clone();
        let mut raw_data = vec![MultiModalContent::Text {
            content: self.data.clone(),
        }];
        for image_uid in self.images.iter() {
            raw_data.push(MultiModalContent::Image { uid: *image_uid });
        }

        let mut pdl_data = vec![];

        for c in raw_data.iter() {
            match c {
                MultiModalContent::Text { content } => {
                    pdl_data.push(escape_pdl_tokens(content));
                }
                MultiModalContent::Image { uid } => {
                    let image_bytes = index.get_image_bytes_by_uid(*uid)?;
                    pdl_data.push(format!(
                        "<|raw_media(png:{})|>",
                        encode_base64(&image_bytes)
                    ));
                }
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
