use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    pub api_provider_name: String,
    pub api_name: String,
}
