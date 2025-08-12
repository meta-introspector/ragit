use ragit_model::ModelRaw;
use crate::qa_system::model_qa_result::ModelQAResult;
pub fn compare_results(models: &[ModelRaw], results: &[ModelQAResult]) -> String {
    let mut comparison = String::new();
    for result in results {
        comparison.push_str(&format!(            "Model: {}\nInitial Score: {}\nAPI Key Used: {}\nResponse Time: {}ms (Expected: {}ms)\nTokens Used: {}\nQuality Scores: Accuracy={:.2}, Coherence={:.2}, Relevance={:.2}\nResponse: {}\n\n",            result.model_name,            result.initial_score,            result.api_key_used,            result.response_time_ms,            models.iter().find(|m| m.name == result.model_name).unwrap().expected_response_time_ms.unwrap_or(500),            result.tokens_used,            result.quality_scores.accuracy,            result.quality_scores.coherence,            result.quality_scores.relevance,            result.response        ));
    }
    comparison
}
