use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::ImageSchema;

impl Index {
    pub fn list_images<F, M, S>(
        &self,
        _filter: &F,
        _map: &M,
        _sort: &S,
    ) -> Result<Vec<ImageSchema>, ApiError>
    where
        F: Fn(&ImageSchema) -> bool,
        M: Fn(&ImageSchema) -> ImageSchema,
        S: Fn(&ImageSchema) -> usize,
    {
        eprintln!("Placeholder for list_images");
        Ok(vec![])
    }
}