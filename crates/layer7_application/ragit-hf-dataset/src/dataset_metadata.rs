use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetadata {
    pub dataset_name: String,
    pub description: String,
    pub version: String,
    pub total_examples: usize,
    pub splits: DatasetSplits,
    pub features: DatasetFeatures,
    pub universe_emojis: Vec<String>,
    pub categories: Vec<String>,
    pub compilation_info: CompilationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetSplits {
    pub train: usize,
    pub validation: usize,
    pub test: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetFeatures {
    pub emoji: String,
    pub unicode_codepoint: String,
    pub count: String,
    pub category: String,
    pub is_universe_emoji: String,
    pub multivector_coefficients: String,
    pub multivector_norm: String,
    pub semantic_properties: String,
    pub context_samples: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationInfo {
    pub processed_at: String,
    pub ragit_version: String,
    pub vectorization_method: String,
    pub clifford_algebra_dimension: u8,
    pub universe_system_emojis: usize,
    pub total_unique_emojis: usize,
}

impl DatasetMetadata {
    pub fn new(
        train_size: usize,
        val_size: usize,
        test_size: usize,
        universe_emojis_count: usize,
    ) -> Self {
        let total_examples = train_size + val_size + test_size;
        
        Self {
            dataset_name: "solfunmeme-emoji-vectors".to_string(),
            description: "Comprehensive emoji vectorization dataset using Clifford algebra multivectors from the ragit matrix-to-emoji transformation system. Each emoji is represented as an 8-dimensional multivector with semantic properties and contextual information.".to_string(),
            version: "1.0.0".to_string(),
            total_examples,
            splits: DatasetSplits {
                train: train_size,
                validation: val_size,
                test: test_size,
            },
            features: DatasetFeatures {
                emoji: "string".to_string(),
                unicode_codepoint: "string".to_string(),
                count: "int64".to_string(),
                category: "string".to_string(),
                is_universe_emoji: "bool".to_string(),
                multivector_coefficients: "array[float32, 8]".to_string(),
                multivector_norm: "float32".to_string(),
                semantic_properties: "dict[string, float32]".to_string(),
                context_samples: "array[string, 3]".to_string(),
            },
            universe_emojis: vec![
                "🧮".to_string(), "🔢".to_string(), "✨".to_string(), "💫".to_string(),
                "🔥".to_string(), "🌊".to_string(), "📊".to_string(), "🎯".to_string(),
                "💎".to_string(), "🕳️".to_string(), "📱".to_string(), "🌙".to_string(),
                "⭐".to_string(), "🌌".to_string(), "🚀".to_string(), "🪐".to_string(),
            ],
            categories: vec![
                "computational_core".to_string(),
                "elemental_forces".to_string(),
                "cosmic_operations".to_string(),
                "targeting_precision".to_string(),
                "crystalline_structures".to_string(),
                "void_space".to_string(),
                "structural_elements".to_string(),
                "mathematical_symbols".to_string(),
                "communication".to_string(),
                "development".to_string(),
                "status_indicators".to_string(),
                "directional".to_string(),
                "faces_and_people".to_string(),
                "nature_and_objects".to_string(),
                "transport_and_symbols".to_string(),
                "miscellaneous_symbols".to_string(),
                "dingbats".to_string(),
                "other".to_string(),
            ],
            compilation_info: CompilationInfo {
                processed_at: chrono::Utc::now().to_rfc3339(),
                ragit_version: "0.4.1".to_string(),
                vectorization_method: "solfunmeme_clifford_sha256_multivector".to_string(),
                clifford_algebra_dimension: 8,
                universe_system_emojis: universe_emojis_count,
                total_unique_emojis: total_examples,
            },
        }
    }
    
    /// Write dataset metadata files to the output directory
    pub fn write_to_dir<P: AsRef<Path>>(&self, output_dir: P) -> Result<()> {
        let output_dir = output_dir.as_ref();
        
        // Write dataset_info.json
        let dataset_info_path = output_dir.join("dataset_info.json");
        let dataset_info_json = serde_json::to_string_pretty(self)?;
        std::fs::write(dataset_info_path, dataset_info_json)?;
        
        // Write README.md
        let readme_path = output_dir.join("README.md");
        let readme_content = self.generate_readme();
        std::fs::write(readme_path, readme_content)?;
        
        // Write .gitattributes for LFS
        let gitattributes_path = output_dir.join(".gitattributes");
        let gitattributes_content = "*.parquet filter=lfs diff=lfs merge=lfs -text\n";
        std::fs::write(gitattributes_path, gitattributes_content)?;
        
        Ok(())
    }
    
    /// Generate README.md content
    fn generate_readme(&self) -> String {
        format!(r#"# 🧮 Solfunmeme Emoji Vectors Dataset

## Overview

This dataset contains **{total_examples} unique emojis** vectorized using Clifford algebra multivectors from the ragit matrix-to-emoji transformation system. Each emoji is represented as an 8-dimensional mathematical vector with semantic properties and contextual information.

## 🌌 Universe System Emojis

This dataset includes **{universe_count}/16** emojis from our universe system:
{universe_list}

These emojis represent the core computational philosophy: **"vibe is vector is meme is quasifiber is multivector is manifold is universe of universe"**

## 📊 Dataset Statistics

- **Total Examples**: {total_examples:,}
- **Training Set**: {train_size:,} examples ({train_percent:.1}%)
- **Validation Set**: {val_size:,} examples ({val_percent:.1}%)
- **Test Set**: {test_size:,} examples ({test_percent:.1}%)
- **Categories**: {category_count} emoji categories
- **Vector Dimension**: 8D Clifford algebra multivectors

## 🔬 Features

Each emoji record contains:

### Core Information
- `emoji`: The emoji character
- `unicode_codepoint`: Unicode representation (e.g., "U+1F4A1")
- `count`: Number of occurrences in the source codebase
- `category`: Semantic category classification
- `is_universe_emoji`: Boolean indicating if it's part of the universe system

### Mathematical Representation
- `mv_coeff_0` to `mv_coeff_7`: 8-dimensional multivector coefficients
- `mv_norm`: Euclidean norm of the multivector

### Semantic Properties
- `complexity`: Computed complexity score based on norm and frequency
- `frequency_score`: Normalized frequency score (0-1)
- `context_diversity`: Measure of contextual usage diversity
- `universe_relevance`: Relevance to the universe system (0-1)
- `semantic_distance`: Distance from universe system center

### Context Samples
- `context_sample_1`, `context_sample_2`, `context_sample_3`: Representative usage examples

### Metadata
- `compilation_timestamp`: When the dataset was generated
- `vectorization_method`: Method used for vectorization

## 🚀 Usage

### Loading with Hugging Face Datasets

```python
from datasets import load_dataset

# Load the complete dataset
dataset = load_dataset("path/to/solfunmeme-emoji-vectors")

# Access different splits
train_data = dataset["train"]
val_data = dataset["validation"]
test_data = dataset["test"]

# Example: Get multivector for an emoji
emoji_record = train_data[0]
multivector = [
    emoji_record["mv_coeff_0"],
    emoji_record["mv_coeff_1"],
    emoji_record["mv_coeff_2"],
    emoji_record["mv_coeff_3"],
    emoji_record["mv_coeff_4"],
    emoji_record["mv_coeff_5"],
    emoji_record["mv_coeff_6"],
    emoji_record["mv_coeff_7"],
]
```

### Loading with Pandas

```python
import pandas as pd

# Load training data
train_df = pd.read_parquet("train-00000-of-00001.parquet")

# Filter universe emojis
universe_emojis = train_df[train_df["is_universe_emoji"] == True]

# Get multivector columns
multivector_cols = [f"mv_coeff_{{i}}" for i in range(8)]
vectors = train_df[multivector_cols].values
```

## 🧠 Computational Philosophy

This dataset embodies the revolutionary matrix-to-emoji transformation philosophy where:

- **Emojis are Mathematical Objects**: Each emoji is a precise 8-dimensional multivector
- **Semantic Web Integration**: Formal RDF ontologies map programming concepts to emojis
- **Universe System**: 16 core emojis represent fundamental computational operations
- **Clifford Algebra**: Mathematical foundation using geometric algebra principles
- **Deterministic Generation**: SHA-256 hashing ensures reproducible vectorization

## 📚 Citation

If you use this dataset in your research, please cite:

```bibtex
@dataset{{solfunmeme_emoji_vectors,
  title={{Solfunmeme Emoji Vectors: Matrix-to-Emoji Transformation Dataset}},
  author={{Ragit Contributors}},
  year={{2025}},
  version={{1.0.0}},
  url={{https://github.com/meta-introspector/ragit}}
}}
```

## 🔗 Related Work

- [Ragit Project](https://github.com/meta-introspector/ragit) - The source project implementing matrix-to-emoji transformation
- [Solfunmeme Clifford](https://github.com/meta-introspector/ragit/tree/main/crates/layer4_transport/solfunmeme_clifford) - Clifford algebra implementation
- [Matrix-to-Emoji Transformation SOPs](https://github.com/meta-introspector/ragit/tree/main/docs/sops) - Technical documentation

## 📄 License

This dataset is released under the same license as the Ragit project (MIT OR Apache-2.0).

## 🤝 Contributing

For issues, improvements, or questions about this dataset, please visit the [Ragit repository](https://github.com/meta-introspector/ragit).

---

*Generated by ragit-hf-dataset v{version} on {timestamp}*
"#,
            total_examples = self.total_examples,
            universe_count = self.compilation_info.universe_system_emojis,
            universe_list = self.universe_emojis.iter()
                .map(|e| format!("- {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            train_size = self.splits.train,
            val_size = self.splits.validation,
            test_size = self.splits.test,
            train_percent = (self.splits.train as f64 / self.total_examples as f64) * 100.0,
            val_percent = (self.splits.validation as f64 / self.total_examples as f64) * 100.0,
            test_percent = (self.splits.test as f64 / self.total_examples as f64) * 100.0,
            category_count = self.categories.len(),
            version = self.version,
            timestamp = self.compilation_info.processed_at,
        )
    }
}
