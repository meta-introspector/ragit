use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
use ragit_index_io::{get_config_by_key, get_all_meta};
use ragit_types::ApiError;
use ragit_utils::string_utils::substr_edit_distance;

pub(crate) async fn run_get_meta(
    argument: &str,
    index: &Index,
) -> Result<ActionResult, ApiError> {
    let mut candidates = vec![argument.to_string()];

    // small QoL: the AI might wrap the key with quotation marks
    if argument.starts_with('"') {
        if let Ok(serde_json::Value::String(s)) = serde_json::from_str(argument) {
            candidates.push(s.to_string());
        }
    }

    let mut result = None;

    for candidate in candidates.iter() {
        if let Some(value) = get_config_by_key(index, candidate.to_string())? {
            result = Some((candidate.to_string(), value.to_string()));
            break;
        }
    }

    if let Some((key, value)) = result {
        Ok(ActionResult::GetMeta { key, value })
    } else {
        let mut similar_keys = vec![];

        for key in get_all_meta(index)?.keys() {
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
        Ok(ActionResult::NoSuchMeta {
            key: argument.to_string(),
            similar_keys,
        })
    }
}
