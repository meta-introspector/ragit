use std::env;
use anyhow::Result;
use ragit_command_bootstrap::bootstrap_index_self;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ragit <command>");
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "bootstrap" => bootstrap_index_self(&args).await?,
        _ => eprintln!("Unknown command: {}", command),
    }

    Ok(())
}
