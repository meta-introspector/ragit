pub mod prelude;

pub use prelude::*;
pub use ragit_fs::{read_string, write_string, WriteMode};

pub mod api_provider;
pub mod audit;
pub mod error;
pub mod message;
pub mod muse;
pub mod qa_system;
pub mod rate_limit;
pub mod request;
pub mod response;

#[cfg(test)]
mod tests;

pub use crate::audit::AuditRecord;
pub use crate::message::message_contents_to_json_array;
pub use crate::muse::muse_enum::MuseName;
pub use crate::muse::muse_struct::Muse;
pub use crate::qa_system::model_qa_result::ModelQAResult;
pub use crate::qa_system::model_qa_system_struct::ModelQASystem;
pub use crate::qa_system::quality_scores::QualityScores;

pub struct FetchResult {
    pub fetched: usize,
    pub updated: usize,
}

pub use ragit_types::pdl_types::ImageType;
pub use ragit_types::JsonType;

pub fn load_models(json_path: &str) -> Result<Vec<Model>, Error> {
    let models = read_string(json_path)?;
    let models: Vec<ModelRaw> = serde_json::from_str(&models)?;
    let mut result = Vec::with_capacity(models.len());

    for model in models.iter() {
        result.push(Model::try_from(model)?);
    }

    Ok(result)
}

pub fn save_models(models: &[Model], path: &str) -> Result<(), Error> {
    let models: Vec<ModelRaw> = models.iter().map(|model| model.into()).collect();
    Ok(write_string(
        path,
        &serde_json::to_string_pretty(&models)?,
        WriteMode::CreateOrTruncate,
    )?)
}

pub fn list_models<Filter, Map, Sort, Key: Ord>(
    // `.ragit/models.json`
    models_at: &str,
    
    // `filter` is applied before `map`
    filter: &Filter,
    map: &Map,
    sort_key: &Sort,
) -> Result<Vec<Model>, Error> where Filter: Fn(&Model) -> bool, Map: Fn(Model) -> Model, Sort: Fn(&Model) -> Key {
    let mut result = vec![];
    
    for model in load_models(models_at)? {
        if !filter(&model) {
            continue;
        }

            let model = map(model);
            result.push(model);
        }

        result.sort_by_key(sort_key);
        Ok(result)
    }
