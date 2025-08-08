use crate::{Result, ProcessingError, EmojiRecord};
use arrow::array::*;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct EmojiParquetWriter {
    max_file_size_mb: usize,
    records_per_file: usize,
}

impl EmojiParquetWriter {
    pub fn new(max_file_size_mb: usize) -> Self {
        // Estimate records per file based on size limit
        // Rough estimate: each record ~500 bytes, so for 10MB = ~20,000 records
        let records_per_file = (max_file_size_mb * 1024 * 1024) / 500;
        
        Self {
            max_file_size_mb,
            records_per_file,
        }
    }
    
    /// Write a dataset split (train/validation/test) to parquet files
    pub async fn write_split<P: AsRef<Path>>(
        &self,
        output_dir: P,
        split_name: &str,
        records: &[EmojiRecord],
    ) -> Result<Vec<PathBuf>> {
        let output_dir = output_dir.as_ref();
        let mut file_paths = Vec::new();
        
        // Calculate number of files needed
        let num_files = (records.len() + self.records_per_file - 1) / self.records_per_file;
        
        println!("📝 Writing {} records to {} files for {} split", 
                records.len(), num_files, split_name);
        
        for (file_idx, chunk) in records.chunks(self.records_per_file).enumerate() {
            let file_name = if num_files == 1 {
                format!("{}-00000-of-00001.parquet", split_name)
            } else {
                format!("{}-{:05}-of-{:05}.parquet", split_name, file_idx, num_files)
            };
            
            let file_path = output_dir.join(&file_name);
            self.write_parquet_file(&file_path, chunk).await?;
            
            // Check file size
            let file_size = std::fs::metadata(&file_path)?.len();
            let file_size_mb = file_size as f64 / (1024.0 * 1024.0);
            
            println!("✅ Written {}: {:.2}MB ({} records)", 
                    file_name, file_size_mb, chunk.len());
            
            if file_size_mb > self.max_file_size_mb as f64 {
                println!("⚠️  Warning: File {} exceeds {}MB limit", file_name, self.max_file_size_mb);
            }
            
            file_paths.push(file_path);
        }
        
        Ok(file_paths)
    }
    
    /// Write a single parquet file
    async fn write_parquet_file<P: AsRef<Path>>(
        &self,
        file_path: P,
        records: &[EmojiRecord],
    ) -> Result<()> {
        let schema = self.create_schema();
        let batch = self.create_record_batch(&schema, records)?;
        
        let file = std::fs::File::create(file_path)?;
        let props = WriterProperties::builder()
            .set_compression(parquet::basic::Compression::SNAPPY)
            .build();
        
        let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;
        writer.write(&batch)?;
        writer.close()?;
        
        Ok(())
    }
    
    /// Create Arrow schema for emoji records
    fn create_schema(&self) -> Arc<Schema> {
        let fields = vec![
            Field::new("emoji", DataType::Utf8, false),
            Field::new("unicode_codepoint", DataType::Utf8, false),
            Field::new("count", DataType::UInt64, false),
            Field::new("category", DataType::Utf8, false),
            Field::new("is_universe_emoji", DataType::Boolean, false),
            
            // Multivector coefficients
            Field::new("mv_coeff_0", DataType::Float32, false),
            Field::new("mv_coeff_1", DataType::Float32, false),
            Field::new("mv_coeff_2", DataType::Float32, false),
            Field::new("mv_coeff_3", DataType::Float32, false),
            Field::new("mv_coeff_4", DataType::Float32, false),
            Field::new("mv_coeff_5", DataType::Float32, false),
            Field::new("mv_coeff_6", DataType::Float32, false),
            Field::new("mv_coeff_7", DataType::Float32, false),
            Field::new("mv_norm", DataType::Float32, false),
            
            // Semantic properties
            Field::new("complexity", DataType::Float32, false),
            Field::new("frequency_score", DataType::Float32, false),
            Field::new("context_diversity", DataType::Float32, false),
            Field::new("universe_relevance", DataType::Float32, false),
            Field::new("semantic_distance", DataType::Float32, false),
            
            // Context samples
            Field::new("context_sample_1", DataType::Utf8, true),
            Field::new("context_sample_2", DataType::Utf8, true),
            Field::new("context_sample_3", DataType::Utf8, true),
            
            // Metadata
            Field::new("compilation_timestamp", DataType::Utf8, false),
            Field::new("vectorization_method", DataType::Utf8, false),
        ];
        
        Arc::new(Schema::new(fields))
    }
    
    /// Create Arrow record batch from emoji records
    fn create_record_batch(
        &self,
        schema: &Arc<Schema>,
        records: &[EmojiRecord],
    ) -> Result<RecordBatch> {
        let len = records.len();
        
        // Create arrays for each field
        let emoji_array = StringArray::from(
            records.iter().map(|r| r.emoji.as_str()).collect::<Vec<_>>()
        );
        
        let unicode_array = StringArray::from(
            records.iter().map(|r| r.unicode_codepoint.as_str()).collect::<Vec<_>>()
        );
        
        let count_array = UInt64Array::from(
            records.iter().map(|r| r.count).collect::<Vec<_>>()
        );
        
        let category_array = StringArray::from(
            records.iter().map(|r| r.category.as_str()).collect::<Vec<_>>()
        );
        
        let is_universe_array = BooleanArray::from(
            records.iter().map(|r| r.is_universe_emoji).collect::<Vec<_>>()
        );
        
        // Multivector coefficients
        let mv_coeff_0_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_0).collect::<Vec<_>>());
        let mv_coeff_1_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_1).collect::<Vec<_>>());
        let mv_coeff_2_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_2).collect::<Vec<_>>());
        let mv_coeff_3_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_3).collect::<Vec<_>>());
        let mv_coeff_4_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_4).collect::<Vec<_>>());
        let mv_coeff_5_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_5).collect::<Vec<_>>());
        let mv_coeff_6_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_6).collect::<Vec<_>>());
        let mv_coeff_7_array = Float32Array::from(records.iter().map(|r| r.mv_coeff_7).collect::<Vec<_>>());
        let mv_norm_array = Float32Array::from(records.iter().map(|r| r.mv_norm).collect::<Vec<_>>());
        
        // Semantic properties
        let complexity_array = Float32Array::from(records.iter().map(|r| r.complexity).collect::<Vec<_>>());
        let frequency_array = Float32Array::from(records.iter().map(|r| r.frequency_score).collect::<Vec<_>>());
        let context_div_array = Float32Array::from(records.iter().map(|r| r.context_diversity).collect::<Vec<_>>());
        let universe_rel_array = Float32Array::from(records.iter().map(|r| r.universe_relevance).collect::<Vec<_>>());
        let semantic_dist_array = Float32Array::from(records.iter().map(|r| r.semantic_distance).collect::<Vec<_>>());
        
        // Context samples (nullable)
        let context_1_array = StringArray::from(
            records.iter().map(|r| r.context_sample_1.as_deref()).collect::<Vec<_>>()
        );
        let context_2_array = StringArray::from(
            records.iter().map(|r| r.context_sample_2.as_deref()).collect::<Vec<_>>()
        );
        let context_3_array = StringArray::from(
            records.iter().map(|r| r.context_sample_3.as_deref()).collect::<Vec<_>>()
        );
        
        // Metadata
        let timestamp_array = StringArray::from(
            records.iter().map(|r| r.compilation_timestamp.as_str()).collect::<Vec<_>>()
        );
        let method_array = StringArray::from(
            records.iter().map(|r| r.vectorization_method.as_str()).collect::<Vec<_>>()
        );
        
        // Create arrays vector
        let arrays: Vec<Arc<dyn Array>> = vec![
            Arc::new(emoji_array),
            Arc::new(unicode_array),
            Arc::new(count_array),
            Arc::new(category_array),
            Arc::new(is_universe_array),
            
            Arc::new(mv_coeff_0_array),
            Arc::new(mv_coeff_1_array),
            Arc::new(mv_coeff_2_array),
            Arc::new(mv_coeff_3_array),
            Arc::new(mv_coeff_4_array),
            Arc::new(mv_coeff_5_array),
            Arc::new(mv_coeff_6_array),
            Arc::new(mv_coeff_7_array),
            Arc::new(mv_norm_array),
            
            Arc::new(complexity_array),
            Arc::new(frequency_array),
            Arc::new(context_div_array),
            Arc::new(universe_rel_array),
            Arc::new(semantic_dist_array),
            
            Arc::new(context_1_array),
            Arc::new(context_2_array),
            Arc::new(context_3_array),
            
            Arc::new(timestamp_array),
            Arc::new(method_array),
        ];
        
        let batch = RecordBatch::try_new(schema.clone(), arrays)?;
        Ok(batch)
    }
}
