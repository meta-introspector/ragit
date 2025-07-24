pub mod commands;
pub mod get_prompt;
pub mod index_add_file_index;
pub mod index_add_file_to_index;
pub mod index_add_staged_file;
pub mod index_chunk_access;
pub mod index_file_schema;
pub mod index_image_schema;
pub mod index_load;
pub mod index_load_chunks_or_tfidf;
pub mod index_load_or_init;
pub mod index_new;
pub mod index_remove_file_index;
pub mod index_run_tfidf;
pub mod index_save_to_file;
pub mod index_uid;
pub mod model_management;
pub mod muse_logic;
pub mod prompt_management;
pub mod raw_request;
pub mod rephrase_multi_turn;
pub mod retrieve_chunks;
pub mod summaries_to_chunks;

pub use ragit_readers::{FileReader, ImageDescription};
pub use tfidf::{consume_processed_doc, ProcessedDoc, TfidfResult, TfidfState};
pub use crate::prelude::*;
pub use ragit_config::BuildConfig;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Index {
    pub root_dir: PathBuf,
    pub processed_files: HashMap<PathBuf, Uid>,
    pub staged_files: HashSet<PathBuf>,
    pub ragit_version: String,
    pub query_config: QueryConfig,
    pub api_config: ApiConfig,
    pub prompts: HashMap<String, String>,
    pub models: Vec<Model>,
    pub curr_processing_file: Option<PathBuf>,
    pub summary: Option<String>,
    pub uid: Uid,
}

impl Index {
    pub fn get_all_processed_docs(&self) -> Result<Vec<ProcessedDoc>, ApiError> {
        let mut result = vec![];

        for file in self.processed_files.keys() {
            let chunk_uids = self.get_chunks_of_file(*self.processed_files.get(file).unwrap())?;

            for chunk_uid in chunk_uids {
                result.push(ProcessedDoc {
                    doc_id: chunk_uid,
                    tokens: tokenize(&self.get_chunk_by_uid(chunk_uid)?.data),
                });
            }
        }

        Ok(result)
    }

    pub fn get_all_chunk_files(&self) -> Result<Vec<PathBuf>, ApiError> {
        let mut result = vec![];

        for chunk_dir in read_dir(
            &join3(
                self.root_dir.to_str().unwrap(),
                INDEX_DIR_NAME,
                CHUNK_DIR_NAME,
            )?,
            false,
        )? {
            for chunk_file in read_dir(&chunk_dir, false)? {
                if extension(&chunk_file)?.unwrap_or(String::new()) != "chunk" {
                    continue;
                }

                result.push(chunk_file);
            }
        }

        Ok(result)
    }

    pub fn get_all_meta(&self) -> Result<HashMap<String, String>, ApiError> {
        let mut result = HashMap::new();

        result.insert(
            String::from("ragit_version"),
            self.ragit_version.clone(),
        );

        Ok(result)
    }

    pub fn get_summary(&self) -> Option<String> {
        self.summary.clone()
    }

    pub async fn query(
        &self,
        query: &str,
        history: Vec<String>,
        output_schema: Option<Schema>,
    ) -> Result<QueryResponse, ApiError> {
        let mut query = query.to_string();

        if !history.is_empty() {
            let multi_turn_schema = self
                .rephrase_multi_turn(history)
                .await?;
            query = multi_turn_schema.rephrased_query;
        }

        let chunks = self.retrieve_chunks(&query, self.query_config.max_retrieval).await?;

        let response = answer_query_with_chunks(
            self,
            &query,
            output_schema,
            chunks,
        ).await?;

        Ok(QueryResponse { response, retrieved_chunks: vec![] })
    }

    pub async fn pull(
        &mut self,
        include_configs: bool,
        include_prompts: bool,
        quiet: bool,
    ) -> Result<FetchResult, ApiError> {
        let mut fetched = 0;
        let mut updated = 0;

        if include_configs {
            let models = ModelRaw::get_initial_models()?;
            let models_at = get_rag_path(&self.root_dir, &PathBuf::from(MODEL_FILE_NAME))?;
            let current_models = if exists(&models_at) {
                let j = read_string(&models_at)?;
                serde_json::from_str::<Vec<ModelRaw>>(&j)?
            } else {
                vec![]
            };

            for model in models {
                if !current_models.contains(&model) {
                    fetched += 1;
                    self.models.push(Model::try_from(&model)?);
                }
            }

            self.save_models()?;
        }

        if include_prompts {
            for prompt_name in PROMPTS.keys() {
                let prompt_path = get_rag_path(
                    &self.root_dir,
                    &join_paths(
                        &PathBuf::from(PROMPT_DIR_NAME),
                        &PathBuf::from(format!("{}.pdl", prompt_name)),
                    )?,
                )?;

                if !exists(&prompt_path) {
                    fetched += 1;
                    write_string(
                        &prompt_path,
                        PROMPTS.get(prompt_name).unwrap().to_string(),
                        WriteMode::Create,
                    )?;
                }
            }
        }

        Ok(FetchResult { fetched, updated })
    }
}
