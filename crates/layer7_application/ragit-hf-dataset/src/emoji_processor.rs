use crate::{Result, ProcessingError, EmojiParquetWriter, DatasetMetadata};
use serde::{Deserialize, Serialize};
use solfunmeme_clifford::generate_multivector_from_string;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{BufReader, BufRead};

/// Universe system emojis from our matrix-to-emoji transformation
const UNIVERSE_EMOJIS: &[&str] = &[
    "🧮", "🔢", "✨", "💫", "🔥", "🌊", "📊", "🎯", 
    "💎", "🕳️", "📱", "🌙", "⭐", "🌌", "🚀", "🪐"
];

/// Emoji categories for classification
const EMOJI_CATEGORIES: &[(&str, &[&str])] = &[
    ("computational_core", &["🧮", "🔢", "📊", "📱"]),
    ("elemental_forces", &["🔥", "🌊", "✨", "💫"]),
    ("cosmic_operations", &["🌌", "🚀", "🪐", "🌙", "⭐"]),
    ("targeting_precision", &["🎯"]),
    ("crystalline_structures", &["💎"]),
    ("void_space", &["🕳️"]),
    ("structural_elements", &["─", "│", "┌", "┐", "└", "┘", "├", "┤", "┬", "┴", "┼"]),
    ("mathematical_symbols", &["∑", "∏", "∫", "∂", "∇", "∞", "π", "α", "β", "γ", "δ", "λ", "μ", "σ", "φ", "ψ", "ω"]),
    ("communication", &["📡", "📢", "📣", "📻", "📺", "📱", "💬", "💭"]),
    ("development", &["⚙️", "🔧", "🔨", "🛠️", "⚡", "🔌", "💡", "🔍"]),
    ("status_indicators", &["✅", "❌", "⚠️", "🔴", "🟡", "🟢", "🔵", "⭐"]),
    ("directional", &["⬆️", "⬇️", "⬅️", "➡️", "↗️", "↘️", "↙️", "↖️", "🔄", "🔃", "🔁", "🔂"]),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiRecord {
    pub emoji: String,
    pub unicode_codepoint: String,
    pub count: u64,
    pub category: String,
    pub is_universe_emoji: bool,
    
    // 8D Multivector coefficients
    pub mv_coeff_0: f32,
    pub mv_coeff_1: f32,
    pub mv_coeff_2: f32,
    pub mv_coeff_3: f32,
    pub mv_coeff_4: f32,
    pub mv_coeff_5: f32,
    pub mv_coeff_6: f32,
    pub mv_coeff_7: f32,
    pub mv_norm: f32,
    
    // Semantic properties
    pub complexity: f32,
    pub frequency_score: f32,
    pub context_diversity: f32,
    pub universe_relevance: f32,
    pub semantic_distance: f32,
    
    // Context samples (max 3)
    pub context_sample_1: Option<String>,
    pub context_sample_2: Option<String>,
    pub context_sample_3: Option<String>,
    
    // Metadata
    pub compilation_timestamp: String,
    pub vectorization_method: String,
}

#[derive(Debug, Clone)]
pub struct EmojiEntry {
    pub emoji: String,
    pub count: u64,
    pub contexts: Vec<String>,
}

pub struct EmojiDatasetProcessor {
    max_file_size_mb: usize,
    output_dir: PathBuf,
    max_contexts_per_emoji: usize,
}

impl EmojiDatasetProcessor {
    pub fn new() -> Self {
        Self {
            max_file_size_mb: 10,
            output_dir: PathBuf::from("emoji-vectors"),
            max_contexts_per_emoji: 3,
        }
    }
    
    pub fn with_max_file_size_mb(mut self, size: usize) -> Self {
        self.max_file_size_mb = size;
        self
    }
    
    pub fn with_output_dir<P: AsRef<Path>>(mut self, dir: P) -> Self {
        self.output_dir = dir.as_ref().to_path_buf();
        self
    }
    
    pub fn with_max_contexts_per_emoji(mut self, max: usize) -> Self {
        self.max_contexts_per_emoji = max;
        self
    }
    
    /// Process the large emoji analysis JSON file into HF dataset format
    pub async fn process_emoji_analysis_data<P: AsRef<Path>>(&self, input_path: P) -> Result<()> {
        println!("🚀 Starting emoji dataset processing...");
        println!("📁 Input: {}", input_path.as_ref().display());
        println!("📁 Output: {}", self.output_dir.display());
        
        // Create output directory
        std::fs::create_dir_all(&self.output_dir)?;
        
        // Step 1: Stream process the large JSON file to extract unique emojis
        let emoji_entries = self.extract_unique_emojis(&input_path).await?;
        println!("✅ Extracted {} unique emojis", emoji_entries.len());
        
        // Step 2: Generate emoji records with multivectors
        let emoji_records = self.generate_emoji_records(emoji_entries).await?;
        println!("✅ Generated {} emoji records with multivectors", emoji_records.len());
        
        // Step 3: Split into train/validation/test sets
        let (train_records, val_records, test_records) = self.split_dataset(emoji_records);
        println!("✅ Split dataset: train={}, val={}, test={}", 
                train_records.len(), val_records.len(), test_records.len());
        
        // Step 4: Write parquet files
        let parquet_writer = EmojiParquetWriter::new(self.max_file_size_mb);
        
        parquet_writer.write_split(&self.output_dir, "train", &train_records).await?;
        parquet_writer.write_split(&self.output_dir, "validation", &val_records).await?;
        parquet_writer.write_split(&self.output_dir, "test", &test_records).await?;
        
        println!("✅ Written parquet files to {}", self.output_dir.display());
        
        // Step 5: Generate dataset metadata
        let metadata = DatasetMetadata::new(
            train_records.len(),
            val_records.len(), 
            test_records.len(),
            UNIVERSE_EMOJIS.len(),
        );
        
        metadata.write_to_dir(&self.output_dir)?;
        println!("✅ Generated dataset metadata");
        
        println!("🎉 Emoji dataset processing completed successfully!");
        Ok(())
    }
    
    /// Stream process large JSON file to extract unique emojis without loading everything into memory
    async fn extract_unique_emojis<P: AsRef<Path>>(&self, input_path: P) -> Result<HashMap<String, EmojiEntry>> {
        println!("📖 Stream processing large JSON file...");
        
        let file = File::open(input_path)?;
        let reader = BufReader::new(file);
        let mut emoji_entries: HashMap<String, EmojiEntry> = HashMap::new();
        
        // This is a simplified approach - in reality we'd need a proper streaming JSON parser
        // For now, let's assume we can process line by line or in chunks
        
        // TODO: Implement proper streaming JSON processing for the 22GB file
        // For now, return a placeholder that we can test with smaller files
        
        println!("⚠️  Note: Full streaming implementation needed for 22GB file");
        println!("📝 Using placeholder implementation for testing");
        
        // Create some test data based on universe emojis
        for &emoji in UNIVERSE_EMOJIS {
            let entry = EmojiEntry {
                emoji: emoji.to_string(),
                count: 100, // Placeholder count
                contexts: vec![
                    format!("Context 1 for {}", emoji),
                    format!("Context 2 for {}", emoji),
                    format!("Context 3 for {}", emoji),
                ],
            };
            emoji_entries.insert(emoji.to_string(), entry);
        }
        
        Ok(emoji_entries)
    }
    
    /// Generate emoji records with multivectors and metadata
    async fn generate_emoji_records(&self, emoji_entries: HashMap<String, EmojiEntry>) -> Result<Vec<EmojiRecord>> {
        println!("🧮 Generating multivectors for {} emojis...", emoji_entries.len());
        
        let mut records = Vec::new();
        let timestamp = chrono::Utc::now().to_rfc3339();
        
        for (emoji, entry) in emoji_entries {
            // Generate multivector using solfunmeme_clifford
            let multivector = generate_multivector_from_string(&emoji);
            let coefficients = multivector.coeff_array_view();
            let norm = multivector.mag2().sqrt();
            
            // Categorize emoji
            let category = self.categorize_emoji(&emoji);
            let is_universe_emoji = UNIVERSE_EMOJIS.contains(&emoji.as_str());
            
            // Calculate semantic properties
            let complexity = norm * (entry.count as f32).ln();
            let frequency_score = (entry.count as f32 / 10000.0).min(1.0);
            let context_diversity = (entry.contexts.len() as f32 / 50.0).min(1.0);
            let universe_relevance = if is_universe_emoji { 1.0 } else { 0.0 };
            let semantic_distance = if is_universe_emoji { 0.1 } else { 0.8 };
            
            // Get unicode codepoint
            let unicode_codepoint = emoji.chars()
                .next()
                .map(|c| format!("U+{:04X}", c as u32))
                .unwrap_or_else(|| "U+0000".to_string());
            
            // Sample contexts (max 3)
            let contexts: Vec<_> = entry.contexts.into_iter().take(self.max_contexts_per_emoji).collect();
            
            let record = EmojiRecord {
                emoji: emoji.clone(),
                unicode_codepoint,
                count: entry.count,
                category,
                is_universe_emoji,
                
                // Multivector coefficients
                mv_coeff_0: coefficients[0],
                mv_coeff_1: coefficients[1],
                mv_coeff_2: coefficients[2],
                mv_coeff_3: coefficients[3],
                mv_coeff_4: coefficients[4],
                mv_coeff_5: coefficients[5],
                mv_coeff_6: coefficients[6],
                mv_coeff_7: coefficients[7],
                mv_norm: norm,
                
                // Semantic properties
                complexity,
                frequency_score,
                context_diversity,
                universe_relevance,
                semantic_distance,
                
                // Context samples
                context_sample_1: contexts.get(0).cloned(),
                context_sample_2: contexts.get(1).cloned(),
                context_sample_3: contexts.get(2).cloned(),
                
                // Metadata
                compilation_timestamp: timestamp.clone(),
                vectorization_method: "solfunmeme_clifford_sha256_multivector".to_string(),
            };
            
            records.push(record);
        }
        
        println!("✅ Generated {} emoji records", records.len());
        Ok(records)
    }
    
    /// Categorize emoji based on predefined categories
    fn categorize_emoji(&self, emoji: &str) -> String {
        for (category, emojis) in EMOJI_CATEGORIES {
            if emojis.contains(&emoji) {
                return category.to_string();
            }
        }
        
        // Default categorization based on Unicode blocks
        let code_point = emoji.chars().next().map(|c| c as u32).unwrap_or(0);
        
        match code_point {
            0x1F600..=0x1F64F => "faces_and_people".to_string(),
            0x1F300..=0x1F5FF => "nature_and_objects".to_string(),
            0x1F680..=0x1F6FF => "transport_and_symbols".to_string(),
            0x2600..=0x26FF => "miscellaneous_symbols".to_string(),
            0x2700..=0x27BF => "dingbats".to_string(),
            _ => "other".to_string(),
        }
    }
    
    /// Split dataset into train/validation/test sets
    fn split_dataset(&self, mut records: Vec<EmojiRecord>) -> (Vec<EmojiRecord>, Vec<EmojiRecord>, Vec<EmojiRecord>) {
        use rand::seq::SliceRandom;
        use rand::thread_rng;
        
        // Shuffle records
        records.shuffle(&mut thread_rng());
        
        let total = records.len();
        let train_size = (total as f32 * 0.8) as usize;
        let val_size = (total as f32 * 0.1) as usize;
        
        let mut records_iter = records.into_iter();
        
        let train_records: Vec<_> = records_iter.by_ref().take(train_size).collect();
        let val_records: Vec<_> = records_iter.by_ref().take(val_size).collect();
        let test_records: Vec<_> = records_iter.collect();
        
        (train_records, val_records, test_records)
    }
}

impl Default for EmojiDatasetProcessor {
    fn default() -> Self {
        Self::new()
    }
}
