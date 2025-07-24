use super::model_qa_system_struct::ModelQASystem;
use ragit_model::ModelRaw;

impl ModelQASystem {
    pub fn new(models: Vec<ModelRaw>, throttling_safety_margin: f64) -> Self {
        ModelQASystem {
            models,
            throttling_safety_margin,
        }
    }
}
