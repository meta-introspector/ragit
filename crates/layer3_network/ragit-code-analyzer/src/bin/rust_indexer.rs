use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the Rust project to index.
    #[clap(short, long)]
    pub project_path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    ragit_code_analyzer::indexer::index_rust_project(&args.project_path)?;

    println!("Indexing complete for project: {:?}", args.project_path);

    Ok(())
}
