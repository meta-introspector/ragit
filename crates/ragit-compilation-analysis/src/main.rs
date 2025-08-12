use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use arrow::array::{BooleanArray, Int64Array, StringArray, UInt32Array};
use arrow::datatypes::{DataType, SchemaRef};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use serde::{Deserialize, Serialize};

// --- DatasetConfig Structs ---
#[derive(Debug, Deserialize)]
pub struct DatasetConfig {
    pub semantic: SemanticConfig,
    pub project: ProjectConfig,
    pub llvm_ir: LlvmIrConfig,
}

#[derive(Debug, Deserialize)]
pub struct SemanticConfig {
    pub parsing_phase: String,
    pub name_resolution_phase: String,
    pub type_inference_phase: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub metadata: String,
    pub dependency_analysis: String,
    pub build_configuration: String,
}

#[derive(Debug, Deserialize)]
pub struct LlvmIrConfig {
    pub ir_generation: String,
    pub optimization_passes: String,
    pub code_generation: String,
    pub performance_analysis: String,
    pub type_system_mapping: String,
    pub memory_analysis: String,
}

// --- AugmentedTerm Struct (from idx/augmented_terms.jsonl) ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugmentedTerm {
    pub term: String,
    pub count: u32,
    pub category: Option<String>,
    pub significance: Option<String>,
    pub vibe: Option<String>,
    pub action_suggestion: Option<String>,
    pub emoji_representation: Option<String>,
    pub semantic_names: Option<Vec<String>>,
    pub osi_layer: Option<String>,
    pub prime_factor: Option<u32>,
    pub is_power_of_two: Option<bool>,
    pub numerical_address: Option<u64>,
    pub embedding_vectors: Option<Vec<f32>>, // Assuming f32 for embeddings
    pub versions: Option<Vec<String>>,
    pub first_seen_timestamp: Option<i64>,
    pub last_seen_timestamp: Option<i64>,
}

// --- Placeholder ParsingPhaseData Struct ---
// This struct will be refined after actual schema inspection of the Parquet files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsingPhaseData {
    pub file_path: String,
    pub line_number: u32, // Placeholder, adjust type based on actual schema
    pub code_snippet: String,
    pub ast_node_type: String, // Placeholder
    // Add more fields as discovered from Parquet schema
}

// --- Parquet Reading and Schema Inspection Logic ---

/// Reads a Parquet file and prints its schema and a few sample records.
pub fn inspect_parquet_file(file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let metadata = builder.metadata();
    let schema = builder.schema();

    println!("--- Parquet File Schema ---");
    println!("File: {}", file_path.display());
    println!("Number of rows: {}", metadata.file_metadata().num_rows());
    println!("Schema: {:#?}", schema);

    let mut reader = builder.build()?;

    println!("
--- Sample Records (up to 5) ---");
    if let Some(batch_result) = reader.next() {
        let batch = batch_result?;
        print_sample_records(&batch)?;
    } else {
        println!("No records found in the file.");
    }

    Ok(())
}

/// Prints sample records from a RecordBatch.
fn print_sample_records(batch: &RecordBatch) -> Result<(), Box<dyn std::error::Error>> {
    let schema = batch.schema();
    let num_rows_to_print = batch.num_rows().min(5);

    for row_idx in 0..num_rows_to_print {
        println!("Record {}:", row_idx + 1);
        for (col_idx, field) in schema.fields().iter().enumerate() {
            let column = batch.column(col_idx);
            let value = extract_value_at_index(column.as_ref(), row_idx)?;
            println!("  {}: {}", field.name(), value);
        }
    }
    Ok(())
}

/// Extracts value from Arrow array at specific index.
fn extract_value_at_index(array: &dyn arrow::array::Array, index: usize) -> Result<String, Box<dyn std::error::Error>> {
    if array.is_null(index) {
        return Ok("null".to_string());
    }

    match array.data_type() {
        DataType::Utf8 => {
            let string_array = array.as_any().downcast_ref::<StringArray>()
                .ok_or("Failed to downcast to StringArray")?;
            Ok(string_array.value(index).to_string())
        }
        DataType::UInt32 => {
            let uint32_array = array.as_any().downcast_ref::<UInt32Array>()
                .ok_or("Failed to downcast to UInt32Array")?;
            Ok(uint32_array.value(index).to_string())
        }
        DataType::Int64 => {
            let int64_array = array.as_any().downcast_ref::<Int64Array>()
                .ok_or("Failed to downcast to Int64Array")?;
            Ok(int64_array.value(index).to_string())
        }
        DataType::Boolean => {
            let bool_array = array.as_any().downcast_ref::<BooleanArray>()
                .ok_or("Failed to downcast to BooleanArray")?;
            Ok(bool_array.value(index).to_string())
        }
        DataType::List(_) => {
            Ok("[list]".to_string()) // Simplified for lists
        }
        _ => {
            Ok(format!("[{}]", array.data_type()))
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config_path = PathBuf::from("/data/data/com.termux/files/home/storage/github/ragit/idx/DatasetConfig.toml");

    // Try to load from ./idx/DatasetConfig.toml first
    if !config_path.exists() {
        config_path = PathBuf::from("./DatasetConfig.toml");
        // If not found in ./idx, try current directory
        if !config_path.exists() {
            return Err("DatasetConfig.toml not found in ./idx/ or current directory.".into());
        }
    }

    let config_content = fs::read_to_string(&config_path)?;
    let config: DatasetConfig = toml::from_str(&config_content)?;

    println!("Loaded DatasetConfig: {:#?}", config);

    // Use the path from the config file
    let parsing_phase_glob = &config.semantic.parsing_phase;
    // For now, we'll just take the first matching file. In a real scenario, you'd glob and iterate.
    let base_path = config_path.parent().unwrap_or(Path::new("."));
    let file_path = base_path.join(parsing_phase_glob.replace("data-*.parquet", "data-00000-of-00023.parquet")); // Replace glob with specific file for now

    if let Err(e) = inspect_parquet_file(&file_path) {
        eprintln!("Error inspecting Parquet file: {}", e);
    }

    Ok(())
}