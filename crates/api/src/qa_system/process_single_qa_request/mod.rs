use crate::error::Error;
use crate::model::{Model, ModelRaw, QualityExpectations};
use crate::request::Request;
use crate::response::Response;
use std::time::Instant;

use super::model_qa_result::ModelQAResult;
use super::quality_scores::QualityScores;

pub async fn process_single_qa_request(
    request: Request,
    model_raw: &ModelRaw,
    throttling_safety_margin: f64,
    evaluate_quality: &dyn Fn(&Response, &Request, Option<&QualityExpectations>) -> QualityScores,
) -> Result<ModelQAResult, Error> {
    let mut qa_request = request.clone();
    qa_request.model = Model::try_from(model_raw)?;

    let mut rate_limiter = crate::rate_limit::RateLimiter::new(&qa_request.model, throttling_safety_margin);
    let delay = rate_limiter.check_and_throttle().unwrap();
    tokio::time::sleep(delay).await;

    let start_time = Instant::now();
    let response = qa_request.send().await?;
    let response_time_ms = start_time.elapsed().as_millis() as u64;

    let quality_scores = evaluate_quality(&response, &request, model_raw.quality_expectations.as_ref());

    Ok(ModelQAResult {
        model_name: model_raw.name.clone(),
        initial_score: model_raw.initial_score.clone().unwrap_or("unknown".to_string()),
        response: response.get_message(0).map(|m| m.to_string()).unwrap_or_default(),
        response_time_ms,
        tokens_used: response.get_output_token_count() as u32, // TODO: get actual tokens used
        quality_scores,
        user_score: None,
    })
}
