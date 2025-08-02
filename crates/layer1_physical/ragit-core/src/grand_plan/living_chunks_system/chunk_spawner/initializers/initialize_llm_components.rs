use crate::grand_plan::llm_sampling_system::llm_model::llm_model_struct::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::tokenizer_struct::Tokenizer;
use crate::grand_plan::llm_sampling_system::embedding_sampler::embedding_sampler_struct::EmbeddingSampler;

pub struct LlmComponents {
    pub llm_model: LlmModel,
    pub tokenizer: Tokenizer,
    pub embedding_sampler: EmbeddingSampler,
}

pub fn initialize_llm_components() -> LlmComponents {
    let llm_model = LlmModel::new(12);
    let tokenizer = Tokenizer::new();
    let embedding_sampler = EmbeddingSampler::new(llm_model, tokenizer);
    LlmComponents {
        llm_model,
        tokenizer,
        embedding_sampler,
    }
}
