use crate::grand_plan::llm_sampling_system::llm_model::llm_model_struct::LlmModel;
use crate::grand_plan::llm_embedding_interface::embedding_request::embedding_request_struct::EmbeddingRequest;
use crate::grand_plan::llm_embedding_interface::embedding_response::embedding_response_struct::EmbeddingResponse;

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a conceptual LLM operation.
pub enum LlmOperation {
    LoadModel(String), // Model identifier
    TrainModel { data_source: String, epochs: u32 },
    SampleText(String), // Prompt
    GetEmbeddings(EmbeddingRequest),
    ImproveModel { feedback_data: String },
}

#[derive(Debug, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents the conceptual result of an LLM operation.
pub enum LlmResult {
    ModelLoaded(LlmModel),
    TrainingComplete(String), // Report or model ID
    SampledText(String),
    Embeddings(EmbeddingResponse),
    ModelImproved(String), // Report or model ID
    Error(String),
}
