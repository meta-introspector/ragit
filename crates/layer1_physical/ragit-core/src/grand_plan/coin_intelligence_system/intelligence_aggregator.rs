use serde::{Deserialize, Serialize};
use crate::grand_plan::coin_intelligence_system::data_processor::ProcessedData;
use crate::grand_plan::llm_monadic_interface::llm_monad::LlmMonad;
use crate::grand_plan::llm_monadic_interface::llm_operations::{LlmOperation, LlmResult};

/// Represents actionable intelligence derived from processed data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinIntelligence {
    pub timestamp: u64,
    pub intelligence_type: String, // e.g., "Sentiment", "Trend", "Anomaly"
    pub value: String, // The actual intelligence (e.g., "positive", "upward", "flash_crash")
    pub supporting_data_refs: Vec<String>, // References to processed data that supports this intelligence
}

/// Aggregates processed data into actionable CoinIntelligence.
pub struct IntelligenceAggregator {
    llm_monad: LlmMonad,
}

impl IntelligenceAggregator {
    pub fn new() -> Self {
        IntelligenceAggregator { llm_monad: LlmMonad::new() }
    }

    /// Aggregates processed data and generates CoinIntelligence.
    pub fn aggregate_intelligence(&mut self, processed_data: Vec<ProcessedData>) -> Vec<CoinIntelligence> {
        println!("Intelligence Aggregator: Aggregating {} processed data points.", processed_data.len());
        let mut intelligence_reports = Vec::new();

        // Simulate LLM-driven intelligence generation
        let mut combined_text = String::new();
        let mut data_refs = Vec::new();

        for data in processed_data {
            match data {
                ProcessedData::CleanedText(text) => {
                    combined_text.push_str(&text);
                    data_refs.push(format!("cleaned_text_hash_{:x}", text.len()));
                },
                ProcessedData::Encrypted(enc) => {
                    data_refs.push(format!("encrypted_data_hash_{:x}", enc.0.len()));
                },
                ProcessedData::ZkProofed(proof) => {
                    data_refs.push(format!("zk_proof_hash_{:x}", proof.0.len()));
                },
                ProcessedData::LatticeRolled(rollup_ref) => {
                    data_refs.push(format!("lattice_rollup_ref_{}", rollup_ref));
                },
            }
        }

        if !combined_text.is_empty() {
            // Use LLM to generate sentiment or trend from combined text
            self.llm_monad = self.llm_monad.bind(LlmOperation::SampleText(format!("Analyze sentiment of: {}", combined_text)));
            if let Some(LlmResult::SampledText(sentiment)) = self.llm_monad.get_result() {
                intelligence_reports.push(CoinIntelligence {
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    intelligence_type: "Sentiment".to_string(),
                    value: sentiment.clone(),
                    supporting_data_refs: data_refs.clone(),
                });
            }
        }

        // Add more conceptual intelligence generation based on other data types
        intelligence_reports.push(CoinIntelligence {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            intelligence_type: "Overall Market Vibe".to_string(),
            value: "Bullish (conceptual)".to_string(),
            supporting_data_refs: data_refs,
        });

        intelligence_reports
    }
}
