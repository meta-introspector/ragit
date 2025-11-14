use crate::uid::Uid;
//use crate::chunk::chunk_source::ChunkSource;

pub trait ChunkLike {
    fn uid(&self) -> &Uid;
    fn char_len(&self) -> usize;
    fn image_count(&self) -> usize;
    fn render_source(&self) -> String;
    // Add other methods here as needed by the Index or other parts of the system
}
