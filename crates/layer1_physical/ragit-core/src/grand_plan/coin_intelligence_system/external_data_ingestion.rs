use serde::{Deserialize, Serialize};

/// Represents a piece of raw external data ingested into the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalData {
    Image(String), // Base64 encoded image or URL
    TwitterDump(String), // Raw text of a Twitter dump
    SolanaTransaction(String), // Raw Solana transaction data
    TelegramLog(String), // Raw Telegram chat log
    DiscordLog(String), // Raw Discord chat log
    // Add more data types as needed
}

/// Simulates ingesting data from various external sources.
pub struct ExternalDataIngestor;

impl ExternalDataIngestor {
    pub fn new() -> Self { ExternalDataIngestor {} }

    /// Simulates ingesting data from a source.
    pub fn ingest_data(&self, source_type: &str, data_payload: String) -> ExternalData {
        println!("Ingesting data from {}: {}...", source_type, &data_payload[0..data_payload.len().min(50)]);
        match source_type {
            "image" => ExternalData::Image(data_payload),
            "twitter" => ExternalData::TwitterDump(data_payload),
            "solana_tx" => ExternalData::SolanaTransaction(data_payload),
            "telegram" => ExternalData::TelegramLog(data_payload),
            "discord" => ExternalData::DiscordLog(data_payload),
            _ => panic!("Unknown data source type"),
        }
    }
}
