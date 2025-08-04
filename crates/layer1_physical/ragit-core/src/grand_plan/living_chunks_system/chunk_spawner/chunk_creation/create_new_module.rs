use crate::grand_plan::llm_sampling_system::embedding_sampler::embedding_sampler_struct::EmbeddingSampler;
use crate::grand_plan::llm_embedding_interface::embedding_request::embedding_request_struct::EmbeddingRequest;

pub fn create_new_module(
    embedding_sampler: &EmbeddingSampler,
    new_module_name: &str,
) -> (String, String, Vec<f32>) {
    let new_module_path = format!("crates/layer1_physical/ragit-core/src/grand_plan/generated_code/{}.rs", new_module_name);

    // Generate conceptual vibe for the new chunk
    let tokens = embedding_sampler.tokenizer.tokenize_string(new_module_name);
    let request = EmbeddingRequest {
        tokens,
        layer_depths: vec![0],
    };
    let response = embedding_sampler.sample_embeddings(&request);
    let conceptual_vibe = response.embeddings.get(&0).unwrap().first().unwrap().clone();

    (new_module_name.to_string(), new_module_path, conceptual_vibe)
}
