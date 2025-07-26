use anyhow::Result;
use tempfile::tempdir;
use std::fs;
use std::path::Path;

use ragit_command_bootstrap::bootstrap_index_self;

async fn setup_test_directory(test_dir: &Path) -> Result<()> {
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

    

    Ok(())
}

#[tokio::test]
async fn test_bootstrap_index_self() -> Result<()> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    setup_test_directory(temp_path).await?;

    

    // Call the function
    let result = bootstrap_index_self(temp_path).await;

    

    // Assert that the function returned Ok
    if let Err(e) = &result {
        eprintln!("bootstrap_index_self failed with: {:?}", e);
    }
    assert!(result.is_ok());

    Ok(())
}
