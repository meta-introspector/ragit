use anyhow::{Result, anyhow};
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use ragit_index_types::Index;
use ragit_index_types::load_mode::LoadMode;
use ragit_types::chunk::chunk_source::ChunkSource;

#[derive(Parser, Debug)]
pub struct DuplicateChunksArgs {
    /// Number of top duplicate chunks to display
    #[arg(short, long, default_value = "10")]
    pub n: usize,
    /// Path to the index directory
    #[arg(long)]
    pub index_path: Option<PathBuf>,
}

pub async fn duplicate_chunks_command_main(args: DuplicateChunksArgs) -> Result<()> {
    let index_path = args.index_path.unwrap_or_else(|| {
        std::env::current_dir().expect("Failed to get current directory")
    });

    println!("Loading index from: {:?}", index_path);
    let index = Index::load(index_path.clone(), LoadMode::Check)
        .map_err(|e| anyhow!("Failed to load index from {:?}: {}", index_path, e))?;

    let mut chunk_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for chunk in index.chunks.iter() {
        let content = chunk.data.as_str().to_string();
        let file_path = match &chunk.source {
            ChunkSource::File { path, .. } => PathBuf::from(path),
            _ => continue, // Skip non-file chunks for this purpose
        };
        chunk_map.entry(content).or_default().push(file_path);
    }

    let mut duplicates: Vec<(&String, &Vec<PathBuf>)> = chunk_map
        .iter()
        .filter(|(_content, paths)| paths.len() > 1)
        .collect();

    duplicates.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    println!("Top {} most duplicate chunks:", args.n);
    for (i, (content, paths)) in duplicates.iter().take(args.n).enumerate() {
        println!("--- Duplicate Chunk {} ({} occurrences) ---", i + 1, paths.len());
        println!("Content: {}", content);
        println!("Found in files:");
        for path in paths.iter() {
            println!("  - {:?}", path);
        }
        println!();
    }

    Ok(())
}
