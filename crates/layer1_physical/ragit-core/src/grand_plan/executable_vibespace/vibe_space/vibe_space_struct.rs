use crate::grand_plan::abi_interface::abi_types::abi_types_enum::{AbiArgs, AbiResult};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::{FunctionRegistry, AbiFunction};
use crate::grand_plan::llm_sampling_system::embedding_sampler::embedding_sampler_struct::EmbeddingSampler;
use crate::grand_plan::llm_sampling_system::llm_model::llm_model_struct::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::tokenizer_struct::Tokenizer;
use crate::grand_plan::executable_vibespace::vibe_function::vibe_function_struct::VibeFunction;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
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
        let request = crate::grand_plan::llm_embedding_interface::embedding_request::embedding_request_struct::EmbeddingRequest {
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
