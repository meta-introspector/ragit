use anyhow::Result;
use tempfile::tempdir;
use std::fs;
use std::path::Path;

use ragit_command_bootstrap::{bootstrap_index_self, file_source::StaticFileSource};

pub async fn setup_test_directory(test_dir: &Path) -> Result<()> {
    // Create .ragit dir
    let ragit_dir = test_dir.join(".ragit");
    fs::create_dir_all(&ragit_dir)?;

    // Create prompts dir and dummy prompt
    let prompts_dir = test_dir.join("prompts");
    fs::create_dir_all(&prompts_dir)?;
    let multi_turn_prompt = r#"---
role: user
---
Rephrase the following conversation turns into a single, coherent search query.

{% for turn in turns %}
- {{ turn }}
{% endfor %}
"#;
    fs::write(prompts_dir.join("multi_turn.pdl"), multi_turn_prompt)?;

    // Create dummy source for bootstrap command
    let bootstrap_crate_dir = test_dir.join("crates/ragit-command-bootstrap/src");
    fs::create_dir_all(&bootstrap_crate_dir)?;
    let bootstrap_code = "pub fn bootstrap_index_self() {}";
    fs::write(bootstrap_crate_dir.join("lib.rs"), bootstrap_code)?;

    Ok(())
}

#[tokio::test]
async fn test_bootstrap_index_self() -> Result<()> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    setup_test_directory(temp_path).await?;

    // Change the current directory to the temporary directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(temp_path)?;

    // Call the function
    let files_to_add_source = StaticFileSource { files: vec!["crates/ragit-command-bootstrap".to_string()] };
    let result = bootstrap_index_self(vec![Box::new(files_to_add_source)]).await;

    // Change back to the original directory
    std::env::set_current_dir(original_dir)?;

    // Assert that the function returned Ok
    if let Err(e) = &result {
        eprintln!("bootstrap_index_self failed with: {:?}", e);
    }
    assert!(result.is_ok());

    Ok(())
}