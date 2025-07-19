use crate::error::Error;
use crate::request::Request;

use super::model_qa_result::ModelQAResult;
use super::model_qa_system_struct::ModelQASystem;
use super::get_selected_models::get_selected_models;
use super::process_single_qa_request::process_single_qa_request;

impl ModelQASystem {
    pub async fn test_request(&self, request: Request) -> Result<Vec<ModelQAResult>, Error> {
        let selected_models = get_selected_models(&self.models)?;

        if selected_models.len() < 2 {
            return Err(Error::InsufficientModels);
        }

        let mut results = Vec::new();
        // TODO: Implement token estimation
        let _estimated_tokens = 0; // Placeholder

        for model_raw in selected_models {
            results.push(process_single_qa_request(
                request.clone(),
                model_raw,
                self.throttling_safety_margin,
                &|response, req, expectations| self.evaluate_quality(response, req, expectations),
            ).await?);
        }

        Ok(results)
    }
}
