use crate::prelude::*;

pub(super) fn check_configs(
    index: &Index,
) -> Result<(), Error> {
    // Check G
    serde_json::from_str::<BuildConfig>(
        &read_string(path_utils::pathbuf_to_str(&index.get_build_config_path()?.into()))?,
    )?;
    serde_json::from_str::<QueryConfig>(
        &read_string(path_utils::pathbuf_to_str(&index.get_query_config_path()?.into()))?,
    )?;
    serde_json::from_str::<ApiConfig>(
        &read_string(path_utils::pathbuf_to_str(&index.get_api_config_path()?.into()))?,
    )?;

    // Extra check: It checks whether the keys in the config files are unique.
    let mut keys = HashSet::new();

    for path in [
        index.get_build_config_path()?.into(),
        index.get_api_config_path()?.into(),
        index.get_query_config_path()?.into(),
    ].iter() {
        let j = read_string(path_utils::pathbuf_to_str(path))?;
        let j = serde_json::from_str::<Value>(&j)?;

        match j {
            Value::Object(obj) => {
                for (key, _) in obj.iter() {
                    if keys.contains(key) {
                        return Err(Error::BrokenIndex(error_reporting::config_key_conflict(path_utils::path_to_display(path.as_path()), key)));
                    }

                    keys.insert(key.to_string());
                }
            },
            _ => {
                return Err(Error::JsonTypeError {
                    expected: JsonType::Object,
                    got: (&j).into(),
                });
            },
        }
    }
    Ok(())
}
