use crate::index_struct::Index;

impl Index {
    pub fn load_minimum(root_dir: PathBuf) -> Result<Self, ApiError> {
        let mut result = Index::default();
        result.root_dir = get_normalized_abs_pathbuf(&root_dir)?;

        result.uid = if exists(&get_rag_path(&root_dir, &PathBuf::from(INDEX_FILE_NAME))?) {
            read_string(&get_rag_path(&root_dir, &PathBuf::from(INDEX_FILE_NAME))?.to_str().unwrap())?.parse().unwrap_or_else(|_| Uid::dummy());
        } else {
            Uid::dummy()
        };

        result.query_config = serde_json::from_str::<QueryConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(QUERY_CONFIG_FILE_NAME).to_str().unwrap(),
        )?)?;
        result.api_config = serde_json::from_str::<ApiConfig>(&read_string(
            &result.root_dir.join(CONFIG_DIR_NAME).join(API_CONFIG_FILE_NAME).to_str().unwrap(),
        )?)?;

        Ok(result)
    }
}
