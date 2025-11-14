use super::model_struct::Model;

impl Model {
    pub fn is_test_model(&self) -> bool {
        self.api_provider_name == "test"
    }
}
