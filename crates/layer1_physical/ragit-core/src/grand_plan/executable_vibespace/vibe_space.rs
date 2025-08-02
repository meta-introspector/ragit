use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult, AbiValue};
use crate::grand_plan::abi_interface::function_registry::FunctionRegistry;
use crate::grand_plan::llm_sampling_system::embedding_sampler::EmbeddingSampler;
use crate::grand_plan::llm_sampling_system::llm_model::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::Tokenizer;

/// Represents a function derived from a vibe (embedding vector).
/// This is a conceptual mapping.
pub struct VibeFunction {
    pub name: String,
    pub embedding: Vec<f32>,
    pub path: Vec<f32>, // The vibe is the vector is the path
    // In a real system, this would be a reference to an actual executable function
    // or a compiled piece of code.
}

impl VibeFunction {
    pub fn new(name: String, embedding: Vec<f32>, path: Vec<f32>) -> Self {
        VibeFunction { name, embedding, path }
    }

    /// Simulates executing the vibe function.
    pub fn execute(&self, args: AbiArgs) -> AbiResult {
        println!("Executing vibe function: {} with args: {:?}", self.name, args);
        // In a real system, this would dispatch to a registered function
        // based on the embedding or name.
        Ok(AbiValue::String(format!("Result of {} execution along path {:?}", self.name, self.path)))
    }
}

/// A conceptual space where vibes can be executed as functions.
pub struct VibeSpace {
    pub functions: Vec<VibeFunction>,
    pub embedding_sampler: EmbeddingSampler,
    pub function_registry: FunctionRegistry,
}

impl VibeSpace {
    pub fn new() -> Self {
        let llm_model = LlmModel::new(12);
        let tokenizer = Tokenizer::new();
        let embedding_sampler = EmbeddingSampler::new(llm_model, tokenizer);
        let function_registry = FunctionRegistry::new(); // This would be populated with real functions

        VibeSpace {
            functions: Vec::new(),
            embedding_sampler,
            function_registry,
        }
    }

    /// Simulates creating a VibeFunction from a string (e.g., a token or concept).
    pub fn create_vibe_function_from_string(&mut self, name: &str, layer_depths: &[u32]) -> VibeFunction {
        let tokens = vec![name.to_string()];
        let request = crate::grand_plan::llm_embedding_interface::EmbeddingRequest {
            tokens,
            layer_depths: layer_depths.to_vec(),
        };
        let response = self.embedding_sampler.sample_embeddings(&request);

        // For simplicity, we'll just take the first embedding from the first requested layer.
        let embedding = response.embeddings.values().next().unwrap().first().unwrap().clone();
        let path = embedding.clone(); // The vibe is the vector is the path

        VibeFunction::new(name.to_string(), embedding, path)
    }

    /// Registers a VibeFunction with the internal function registry.
    pub fn register_vibe_function(&mut self, name: &str, func: AbiFunction) {
        self.function_registry.register_function(name, func);
    }

    /// Executes a VibeFunction by name.
    pub fn execute_vibe_function(&self, name: &str, args: AbiArgs) -> AbiResult {
        if let Some(vibe_func) = self.functions.iter().find(|f| f.name == name) {
            vibe_func.execute(args)
        } else {
            self.function_registry.call_function(name, args)
        }
    }
}