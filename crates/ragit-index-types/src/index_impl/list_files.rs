use crate::index_struct::Index;
use ragit_types::ApiError;
use ragit_types::FileSchema;

impl Index {
    pub fn list_files<F, M, S>(
        &self,
        _filter: &F,
        _map: &M,
        _sort: &S,
    ) -> Result<Vec<FileSchema>, ApiError>
    where
        F: Fn(&FileSchema) -> bool,
        M: Fn(&FileSchema) -> FileSchema,
        S: Fn(&FileSchema) -> String,
    {
        eprintln!("Placeholder for list_files");
        Ok(vec![])
    }
}