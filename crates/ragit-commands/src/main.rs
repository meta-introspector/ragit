use std::env;
use anyhow::Result;
use ragit_command_bootstrap::bootstrap_index_self;
use tempfile::tempdir;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ragit <command>");
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "bootstrap" => {
            let temp_dir = tempdir()?;
            let temp_path = temp_dir.path();
            bootstrap_index_self(temp_path).await?;
        },
        _ => eprintln!("Unknown command: {}", command),
    }

    Ok(())
}
