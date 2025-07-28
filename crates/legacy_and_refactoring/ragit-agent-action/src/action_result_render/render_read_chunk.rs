use ragit_types::Chunk;

pub fn render_read_chunk(chunk: &Chunk) -> String {
    chunk.data.clone()
}