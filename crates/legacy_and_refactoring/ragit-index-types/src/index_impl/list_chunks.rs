use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::chunk::chunk_schema::ChunkSchema;

impl Index {
    pub fn list_chunks<F, M, S>(
        &self,
        _filter: &F,
        _map: &M,
        _sort: &S,
    ) -> Result<Vec<ChunkSchema>, ApiError>
    where
        F: Fn(&ChunkSchema) -> bool,
        M: Fn(&ChunkSchema) -> ChunkSchema,
        S: Fn(&ChunkSchema) -> String,
    {
        eprintln!("Placeholder for list_chunks");
        Ok(vec![])
    }
}