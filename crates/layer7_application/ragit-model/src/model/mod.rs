pub mod model_struct;
pub mod model_default;
pub mod model_dummy;
pub mod model_stdin;
pub mod model_error;
pub mod model_get_api_url;
pub mod model_get_api_key;
pub mod model_find_api_key_in_external_files;
pub mod model_find_api_key_in_file;
pub mod model_is_test_model;
pub mod model_default_models;
pub mod model_try_from_model_raw;
pub mod model_from_model_raw;
pub mod model_partial_eq;
pub mod test_model_struct;

pub use test_model_struct::TestModel;
