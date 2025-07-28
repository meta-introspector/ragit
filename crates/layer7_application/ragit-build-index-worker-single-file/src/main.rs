use anyhow::Result;
use std::env;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];
    println!("Attempting to read file: {}", file_path);

    let content = fs::read_to_string(file_path).await?;
    println!("File content:
{}", content);

    Ok(())
}
