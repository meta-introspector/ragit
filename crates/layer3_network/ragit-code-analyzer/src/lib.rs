use walkdir::WalkDir;
use rayon::prelude::*;
use std::path::PathBuf;
use anyhow::Result;
use solfunmeme_core_logic::{CodeAnalyzer, CodeAnalysis as SolfunmemeCodeAnalysis};

/// Represents the extracted information from a single Rust source file.
#[derive(Debug)]
pub struct FileAnalysis {
    pub path: PathBuf,
    pub declarations: Vec<String>, // e.g., "struct MyStruct", "fn my_function"
    pub imports: Vec<String>,      // e.g., "use crate::module::Item"
    // Add more fields for semantic information as needed
}

pub fn analyze_codebase(root_dir: &PathBuf) -> Result<Vec<FileAnalysis>> {
    let rust_files: Vec<PathBuf> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs"))
        .map(|entry| entry.into_path())
        .collect();

    let mut analyzer = CodeAnalyzer::new(100, 0.8); // Initialize CodeAnalyzer

    let analyses: Vec<FileAnalysis> = rust_files.par_iter().filter_map(|file_path| {
        let content = std::fs::read_to_string(file_path).ok()?;
        let solfunmeme_analysis = analyzer.analyze_file(&content, file_path.to_string_lossy().into_owned()).ok()?;

        Some(FileAnalysis {
            path: file_path.clone(),
            declarations: solfunmeme_analysis.declarations.into_iter().map(|d| format!("{:?} {}", d.declaration_type, d.name)).collect(),
            imports: vec![], // solfunmeme_core_logic does not directly extract imports, so this remains empty for now
        })
    }).collect();

    Ok(analyses)
}