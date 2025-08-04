use crate::grand_plan::llm_sampling_system::llm_model::llm_model_struct::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::tokenizer_struct::Tokenizer;
use crate::grand_plan::llm_sampling_system::embedding_sampler::embedding_sampler_struct::EmbeddingSampler;
use crate::grand_plan::token_indexing_system::token_index::token_index_struct::TokenIndex;
use crate::grand_plan::living_chunks_system::living_chunk_manager::living_chunk_manager_struct::LivingChunkManager;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;

pub struct CoreComponents {
    pub llm_model: LlmModel,
    pub tokenizer: Tokenizer,
    pub embedding_sampler: EmbeddingSampler,
    pub token_index: TokenIndex,
    pub living_chunk_manager: LivingChunkManager,
    pub grand_unified_store: GrandUnifiedStore,
}

pub fn initialize_core_components() -> CoreComponents {
    let llm_model = LlmModel::new(12);
    let tokenizer = Tokenizer::new();
    let embedding_sampler = EmbeddingSampler::new(llm_model.clone(), tokenizer.clone());
    let token_index = TokenIndex::new(tokenizer.clone());
    let living_chunk_manager = LivingChunkManager::new();
    let mut grand_unified_store = GrandUnifiedStore::new();
    grand_unified_store.add_char_store();
    grand_unified_store.add_i64_store();

    CoreComponents {
        llm_model,
        tokenizer,
        embedding_sampler,
        token_index,
        living_chunk_manager,
        grand_unified_store,
    }
}
