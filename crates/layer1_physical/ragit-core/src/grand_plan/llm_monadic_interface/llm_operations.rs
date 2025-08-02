use crate::grand_plan::llm_sampling_system::llm_model::LlmModel;
use crate::grand_plan::llm_embedding_interface::{EmbeddingRequest, EmbeddingResponse};

/// Represents a conceptual LLM operation.
#[derive(Debug)]
pub enum LlmOperation {
    LoadModel(String), // Model identifier
    TrainModel { data_source: String, epochs: u32 },
    SampleText(String), // Prompt
    GetEmbeddings(EmbeddingRequest),
    ImproveModel { feedback_data: String },
}

/// Represents the conceptual result of an LLM operation.
#[derive(Debug)]
pub enum LlmResult {
    ModelLoaded(LlmModel),
    TrainingComplete(String), // Report or model ID
    SampledText(String),
    Embeddings(EmbeddingResponse),
    ModelImproved(String), // Report or model ID
    Error(String),
}
