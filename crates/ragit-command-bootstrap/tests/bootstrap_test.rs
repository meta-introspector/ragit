use anyhow::Result;
use tempfile::tempdir;
use std::fs;
use std::path::Path;
use std::io;
use ragit_utils::project_root;

use ragit_command_bootstrap::bootstrap_index_self;

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

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

    // Copy the bootstrap crate into the temporary directory
    let project_root = ragit_utils::project_root::find_root()?;
    let bootstrap_crate_src = project_root.join("crates/ragit-command-bootstrap");
    let bootstrap_crate_dst = test_dir.join("crates/ragit-command-bootstrap");
    copy_dir_all(&bootstrap_crate_src, &bootstrap_crate_dst)?;

    Ok(())
}

#[tokio::test]
async fn test_bootstrap_index_self() -> Result<()> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    setup_test_directory(temp_path).await?;

    // Call the function
    let result = bootstrap_index_self(temp_path).await;

    println!("Contents of .ragit/chunks after build:");
    let chunks_dir = temp_path.join(".ragit/chunks");
    if chunks_dir.exists() {
        for entry in fs::read_dir(&chunks_dir)? {
            let entry = entry?;
            println!("  {:?}", entry.path());
        }
    } else {
        println!("  .ragit/chunks directory does not exist.");
    }

    // Assert that the function returned Ok
    if let Err(e) = &result {
        eprintln!("bootstrap_index_self failed with: {:?}", e);
    }
    assert!(result.is_ok());

    Ok(())
}