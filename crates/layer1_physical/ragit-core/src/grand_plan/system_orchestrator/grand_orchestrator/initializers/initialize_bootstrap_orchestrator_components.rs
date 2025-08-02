use crate::grand_plan::llm_sampling_system::llm_model::LlmModel;
use crate::grand_plan::llm_sampling_system::tokenizer::Tokenizer;
use crate::grand_plan::llm_sampling_system::embedding_sampler::EmbeddingSampler;
use crate::grand_plan::token_indexing_system::token_index::TokenIndex;
use crate::grand_plan::living_chunks_system::living_chunk_manager::LivingChunkManager;
use crate::grand_plan::solfunmeme_zos::solfunmeme_core::Solfunmeme;
use crate::grand_plan::solfunmeme_zos::hyper_pump_mechanism::HyperPumpMechanism;
use crate::grand_plan::meme_economy::meme_lord::MemeLord;
use crate::grand_plan::solana_integration::compute_marketplace::ComputeMarketplace;
use crate::grand_plan::solana_integration::market_maker::MarketMaker;
use crate::grand_plan::unified_store::GrandUnifiedStore;

pub struct BootstrapOrchestratorComponents {
    pub llm_model: LlmModel,
    pub tokenizer: Tokenizer,
    pub embedding_sampler: EmbeddingSampler,
    pub token_index: TokenIndex,
    pub living_chunk_manager: LivingChunkManager,
    pub solfunmeme_system: Solfunmeme,
    pub hyper_pump: HyperPumpMechanism,
    pub meme_lord: MemeLord,
    pub compute_marketplace: ComputeMarketplace,
    pub market_maker: MarketMaker,
    pub grand_unified_store: GrandUnifiedStore,
}

pub fn initialize_bootstrap_orchestrator_components() -> BootstrapOrchestratorComponents {
    let llm_model = LlmModel::new(12);
    let tokenizer = Tokenizer::new();
    let embedding_sampler = EmbeddingSampler::new(llm_model.clone(), tokenizer.clone());
    let token_index = TokenIndex::new(tokenizer.clone());
    let living_chunk_manager = LivingChunkManager::new();
    let solfunmeme_system = Solfunmeme::new();
    let hyper_pump = HyperPumpMechanism::new();
    let meme_lord = MemeLord::new("initial_meme_lord".to_string(), 1000.0);
    let compute_marketplace = ComputeMarketplace::new();
    let market_maker = MarketMaker::new();
    let mut grand_unified_store = GrandUnifiedStore::new();
    grand_unified_store.add_char_store();
    grand_unified_store.add_i64_store();

    BootstrapOrchestratorComponents {
        llm_model,
        tokenizer,
        embedding_sampler,
        token_index,
        living_chunk_manager,
        solfunmeme_system,
        hyper_pump,
        meme_lord,
        compute_marketplace,
        market_maker,
        grand_unified_store,
    }
}
