use crate::index::index_struct::Index;
use crate::error::Error;

impl Index {
    pub fn meta(
        &mut self,
        key: String,
        value: Option<String>,
    ) -> Result<(), Error> {
        if let Some(value) = value {
            self.set_config_by_key(key, value)?;
        }

        else {
            let value = self.get_config_by_key(key.clone())?;
            println!("{}", value);
        }

        Ok(())
    }
}
