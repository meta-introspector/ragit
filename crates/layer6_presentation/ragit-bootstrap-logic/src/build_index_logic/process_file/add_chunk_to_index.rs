use ragit_index_types::index_struct::Index;
use ragit_memory_monitor::MemoryMonitor;
use ragit_types::fixed_types::fixed_chunk_struct::FixedChunk;
use ragit_types::uid::Uid;
use ragit_types::chunk::chunk_source::ChunkSource;

pub fn add_chunk_to_index(
    index: &mut Index,
    memory_monitor: &mut MemoryMonitor,
    chunk_data: &str,
    file_path_str: &str,
    chunk_index: usize,
) {
    let new_chunk = FixedChunk {
        data: chunk_data.into(),
        file: file_path_str.to_string().into(),
        index: chunk_index,
        uid: Uid::new_from_slice(chunk_data.as_bytes()), // Generate UID from chunk data
        char_len: chunk_data.len(),
        source: ChunkSource::File { path: file_path_str.to_string(), index: chunk_index, page: None },
        ..FixedChunk::dummy()
    };
    // memory_monitor.verbose(&format!("Adding chunk to index: data='{}', uid='{}'", new_chunk.data, new_chunk.uid));
    index.add_chunk(new_chunk);
    memory_monitor.process_unit();
}
