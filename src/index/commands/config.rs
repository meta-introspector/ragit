use crate::index::index_struct::Index;
use crate::error::Error;

pub fn config(
    index: &mut Index,
    key: String,
    value: Option<String>,
) -> Result<(), Error> {
    if let Some(value) = value {
        index.set_config_by_key(key, value)?;
    }

    else {
        let value = index.get_config_by_key(key.clone())?;
        println!("{}", value);
    }

    Ok(())
}
