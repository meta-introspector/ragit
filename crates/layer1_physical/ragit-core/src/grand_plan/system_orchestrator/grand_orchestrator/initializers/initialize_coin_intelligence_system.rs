use crate::grand_plan::coin_intelligence_system::external_data_ingestion::external_data_ingestion_enum::ExternalDataIngestor;
use crate::grand_plan::coin_intelligence_system::data_processor::data_processor_structs::DataProcessor;
use crate::grand_plan::coin_intelligence_system::intelligence_aggregator::intelligence_aggregator_struct::IntelligenceAggregator;
use crate::grand_plan::coin_intelligence_system::model_sharing::model_sharing_structs::ModelSharingSystem;

pub struct CoinIntelligenceSystemComponents {
    pub external_data_ingestor: ExternalDataIngestor,
    pub data_processor: DataProcessor,
    pub intelligence_aggregator: IntelligenceAggregator,
    pub model_sharing_system: ModelSharingSystem,
}

pub fn initialize_coin_intelligence_system_components() -> CoinIntelligenceSystemComponents {
    let external_data_ingestor = ExternalDataIngestor::new();
    let data_processor = DataProcessor::new();
    let intelligence_aggregator = IntelligenceAggregator::new();
    let model_sharing_system = ModelSharingSystem::new(vec![0.0; 128]);

    CoinIntelligenceSystemComponents {
        external_data_ingestor,
        data_processor,
        intelligence_aggregator,
        model_sharing_system,
    }
}
