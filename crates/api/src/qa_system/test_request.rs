use crate::error::Error;
use crate::request::Request;

use crate::qa_system::evaluate_quality::evaluate_quality;
use crate::qa_system::get_selected_models::get_selected_models;
use crate::qa_system::model_qa_result::ModelQAResult;
use crate::qa_system::model_qa_system_struct::ModelQASystem;
use crate::qa_system::process_single_qa_request::process_single_qa_request;

impl ModelQASystem {
    pub async fn test_request(&self, request: Request) -> Result<Vec<ModelQAResult>, Error> {
        let selected_models = get_selected_models(&self.models)?;

        if selected_models.len() < 2 {
            return Err(Error::TypesApiError(ragit_types::ApiError::InsufficientModels));
        }

        let mut results = Vec::new();
        // TODO: Implement token estimation
        let _estimated_tokens = 0; // Placeholder

        for model_raw in selected_models {
            results.push(
                process_single_qa_request(
                    request.clone(),
                    &model_raw,
                    self.throttling_safety_margin,
                    &|response, req, expectations| evaluate_quality(response, req, expectations),
                )
                .await?,
            );
        }

        Ok(results)
    }
}
