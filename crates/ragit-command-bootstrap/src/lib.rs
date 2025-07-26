use anyhow::Result;
use ragit_command_init::init_command_main;
use ragit_command_add::add_command_main;
use ragit_command_build::build_command_main;
use ragit_command_query::query_command_main;


pub async fn bootstrap_command_main(args: &[String]) -> Result<()> {
    // 1. rag init
    println!("Running: rag init");
    init_command_main(&[]).await?;

    // 2. rag add --all
    println!("Running: rag add --all");
    add_command_main(&["add".to_string(), "--all".to_string()]).await?;

    // 3. rag build
    println!("Running: rag build");
    build_command_main(&[]).await?;

    // 4. rag query "What makes ragit special?"
    println!("Running: rag query \"What makes ragit special?\"");
    query_command_main(&["query".to_string(), "What makes ragit special?".to_string()]).await?;

    Ok(())
}
