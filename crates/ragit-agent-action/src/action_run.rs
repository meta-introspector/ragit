use super::action_enum::Action;
use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
use ragit_types::{ApiError, Uid, Chunk};
use ragit_types::chunk::rendered_chunk::RenderedChunk;
use ragit_model_query_response::ModelQueryResponse;
use ragit_utils::ragit_path_utils::normalize;
use ragit_utils::string_utils::substr_edit_distance;
use ragit_index_io::query_helpers::{UidQueryConfig, UidQueryResult};
use ragit_utils::query::Keywords;
use ragit_agent::file_tree::FileTree;
use crate::search_type_enum::SearchType;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use anyhow::Result;

impl Action {
    pub(crate) async fn run(&self, argument: &str, index: &Index) -> Result<ActionResult, ApiError> {
        let argument_path = PathBuf::from(normalize(&argument)?);

        let r = match self {
            Action::ReadFile => match index.processed_files.get(&argument_path) {
                Some(uid) => {
                    // TODO: Replace with StorageManager call
                    let chunk_uids = vec![]; // Placeholder

                    // If the file is too long, it shows the summaries of its chunks
                    // instead of `cat-file`ing the file.
                    // TODO: what if it's sooooo long that even the chunk list is too long?
                    let max_chunks = index.query_config.max_retrieval;

                    // NOTE: Even an empty file has a chunk. So `.len()` must be greater than 0.
                    match chunk_uids.len() {
                        1 => {
                            // TODO: Replace with StorageManager call
                            let chunk = ragit_types::chunk::chunk_struct::Chunk::dummy().render(index)?;
                            ActionResult::ReadFileShort {
                                chunk_uids,
                                rendered: chunk,
                            }
                        }
                        n if n <= max_chunks => {
                            // TODO: Replace with StorageManager call
                            let chunk_uids = vec![]; // Placeholder
                            let chunk = ragit_types::chunk::chunk_struct::Chunk::dummy(); // Placeholder
                            ActionResult::ReadFileShort {
                                chunk_uids,
                                rendered: chunk,
                            }
                        }
                        _ => {
                            let mut chunks = Vec::with_capacity(chunk_uids.len());

                            for chunk_uid in chunk_uids.iter() {
                                // TODO: Replace with StorageManager call
                                chunks.push(ragit_types::chunk::chunk_struct::Chunk::dummy()); // Placeholder
                            }

                            ActionResult::ReadFileLong(chunks)
                        }
                    }
                }
                None => {
                    let mut similar_files = vec![];

                    // TODO: it might take very very long time if the knowledge-base is large...
                    for file in index.processed_files.keys() {
                        let dist = substr_edit_distance(
                            argument.as_bytes(),
                            file.to_str().unwrap().as_bytes(),
                        );

                        if dist < 3 {
                            similar_files.push((file.display().to_string(), dist));
                        }
                    }

                    similar_files.sort_by_key(|(_, d)| *d);

                    if similar_files.len() > 10 {
                        similar_files = similar_files[..10].to_vec();
                    }

                    let similar_files =
                        similar_files.into_iter().map(|(f, _)| f).collect::<Vec<_>>()
                    ;
                    ActionResult::NoSuchFile {
                        file: argument_path.display().to_string(),
                        similar_files,
                    }
                }
            },
            Action::ReadDir => {
                let mut argument_path_str = argument.to_string();
                if !argument_path_str.ends_with("/") && argument_path_str != "" {
                    argument_path_str = format!("{}/", argument_path_str);
                }
                let argument_path = PathBuf::from(argument_path_str);

                let mut file_tree = FileTree::root();

                for file in index.processed_files.keys() {
                    if file.starts_with(&argument_path) {
                        file_tree
                            .insert(file.strip_prefix(&argument_path).unwrap().to_str().unwrap());
                    }
                }

                if file_tree.is_empty() {
                    ActionResult::NoSuchDir {
                        dir: argument_path.display().to_string(),

                        // TODO: I want to suggest directories with a similar name,
                        //       but it's too tricky to find ones.
                        similar_dirs: vec![],
                    }
                } else {
                    ActionResult::ReadDir(file_tree)
                }
            }
            Action::ReadChunk => {
                if !Uid::is_valid_prefix(&argument) {
                    ActionResult::NoSuchChunk(argument.to_string())
                } else {
                    let query = UidQueryConfig::new().chunk_only().query_unit(
                        index,
                        &[argument.to_string()],
                    )?;
                    let chunk_uids = query.get_chunk_uids();

                    match chunk_uids.len() {
                        0 => ActionResult::NoSuchChunk(argument.to_string()),
                        1 => ActionResult::ReadChunk(ragit_types::chunk::chunk_struct::Chunk::dummy()), // Placeholder
                        2..=10 => {
                            let mut chunks = Vec::with_capacity(chunk_uids.len());

                            for chunk_uid in chunk_uids.iter() {
                                // TODO: Replace with StorageManager call
                                chunks.push(ragit_types::chunk::chunk_struct::Chunk::dummy()); // Placeholder
                            }

                            ActionResult::ReadChunkAmbiguous {
                                query: argument.to_string(),
                                chunks,
                            }
                        }
                        _ => ActionResult::ReadChunkTooMany {
                            query: argument.to_string(),
                            chunk_uids: chunk_uids.len(),
                        },
                    }
                }
            }
            Action::SearchExact | Action::SearchTfidf => {
                // The result of exact search is a subset of the result of tfidf search.
                let mut limit = if *self == Action::SearchExact {
                    100
                } else {
                    10
                };

                let chunks = 'chunks_loop: loop {
                    let candidates =
                        ragit_tfidf::TfidfState::new(&Keywords::from(vec![argument.to_string()])).search(&Keywords::from(vec![argument.to_string()]));
                    let mut chunks = Vec::with_capacity(candidates.len());
                    let mut chunks_exact_match = vec![];

                    for c in candidates.iter() {
                        // TODO: Replace with StorageManager call
                        chunks.push(ragit_types::chunk::chunk_struct::Chunk::dummy()); // Placeholder
                    }

                    if *self == Action::SearchTfidf {
                        break chunks;
                    }

                    for chunk in chunks.iter() {
                        if chunk.data.contains(&argument) {
                            chunks_exact_match.push(chunk.clone());

                            if chunks_exact_match.len() == 10 {
                                break 'chunks_loop chunks_exact_match;
                            }
                        }
                    }

                    // We have a complete set of the tfidf result, so there's
                    // no point in increasing the limit.
                    if candidates.len() < limit || limit == index.chunk_count {
                        break chunks_exact_match;
                    }

                    // Maybe we can get more exact-matches if we increase the
                    // limit of the tfidf-match.
                    limit = (limit * 5).min(index.chunk_count());
                };

                use crate::search_type_enum::SearchType;
                let mut candidates = vec![argument.to_string()];

                // small QoL: the AI might wrap the key with quotation marks
                if argument.starts_with("\"") {
                    if let Ok(serde_json::Value::String(s)) = serde_json::from_str(&argument) {
                        candidates.push(s.to_string());
                    }
                }

                let mut result = None;

                for candidate in candidates.iter() {
                    if let Some(value) = Some("dummy_value".to_string()) {
                        result = Some((candidate.to_string(), value.to_string()));
                        break;
                    }
                }

                if let Some((key, value)) = result {
                    ActionResult::GetMeta { key, value }
                } else {
                    let mut similar_keys = vec![];

                    for key in vec!["dummy_key".to_string()].iter() {
                        let dist = substr_edit_distance(argument.as_bytes(), key.as_bytes());

                        if dist < 3 {
                            similar_keys.push((key.to_string(), dist));
                        }
                    }

                    similar_keys.sort_by_key(|(_, d)| *d);

                    if similar_keys.len() > 10 {
                        similar_keys = similar_keys[..10].to_vec();
                    }

                    let similar_keys = similar_keys.into_iter().map(|(f, _)| f).collect::<Vec<_>>();
                    ActionResult::NoSuchMeta {
                        key: argument.to_string(),
                        similar_keys,
                    }
                }
            }
            Action::GetSummary => {
                // The summary must exist. Otherwise, this action should have been filtered out.
                let summary = "dummy summary".to_string();
                ActionResult::GetSummary(summary.to_string())
            }
            Action::SimpleRag => {
                let response = ragit_model_query_response::ModelQueryResponse::new(
                    &ragit_api::Model::default(), // Placeholder
                    &ragit_pdl::Prompt::default(), // Placeholder
                    "", // Placeholder
                    "", // Placeholder
                ).await?;

                ActionResult::SimpleRag(response)
            }
        };

        Ok(r)
    }
}
