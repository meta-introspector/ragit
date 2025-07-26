use super::Model;

//use ragit_types::ApiError as Error;
use ragit_error::ApiError as Error;
pub fn get_model_by_name(models: &[Model], name: &str) -> Result<Model, Error> {
    let mut partial_matches = vec![];

    for model in models.iter() {
        if model.name == name {
            return Ok(model.clone());
        }

        if partial_match(&model.name, name) {
            partial_matches.push(model);
        }
    }

    if partial_matches.len() == 1 {
        Ok(partial_matches[0].clone())
    } else if name == "dummy" {
        Ok(Model::dummy())
    } else if name == "stdin" {
        Ok(Model::stdin())
    } else if name == "error" {
        Ok(Model::error())
    } else {
        Err(Error::InvalidModelName {
            name: name.to_string(),
            candidates: partial_matches
                .iter()
                .map(|model| model.name.to_string())
                .collect(),
        })
    }
}

fn partial_match(haystack: &str, needle: &str) -> bool {
    let h_bytes = haystack.bytes().collect::<Vec<_>>();
    let n_bytes = needle.bytes().collect::<Vec<_>>();
    let mut h_cursor = 0;
    let mut n_cursor = 0;

    while h_cursor < h_bytes.len() && n_cursor < n_bytes.len() {
        if h_bytes[h_cursor] == n_bytes[n_cursor] {
            h_cursor += 1;
            n_cursor += 1;
        } else {
            h_cursor += 1;
        }
    }

    n_cursor == n_bytes.len()
}
