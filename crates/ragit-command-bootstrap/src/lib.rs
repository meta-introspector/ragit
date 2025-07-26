use ragit_index_core::load_index_from_path;

use ragit_index_core::add_files::add_files_command;
use std::path::Path;
use ragit_fs::{read_string, write_string, WriteMode};
use std::fmt::Write as FmtWrite;
pub mod file_source;

use ragit_command_init::init_command_main;
use crate::file_source::FileSource;
use ragit_index_io::StorageManager;

pub async fn bootstrap_index_self(temp_path: &Path) -> Result<(), anyhow::Error> {
    // 1. rag init
    println!("Running: rag init");
    init_command_main(&["ragit".to_string(), "init".to_string()], Some(temp_path)).await?;
    let root_dir = ragit_utils::project_root::find_root()?;
    let index_path = temp_path.join(".ragit/index");
    let index_content = r#"{"ragit_version": "0.4.2", "api_config": {"model": "test-model"}, "files": {}, "staged_files": [], "processed_files": {}, "chunks": [], "chunk_count": 0, "inverted_index": null, "ii_status": {"enabled": false}}"#;
    write_string(index_path.to_str().unwrap(), index_content, WriteMode::CreateOrTruncate)?;
    let mut index = load_index_from_path(&index_path)?;

    // 2. rag add crates/ragit-command-bootstrap
    println!("Running: rag add");
    let bootstrap_source = file_source::CargoPackageFileSource { 
        package_name: "ragit-command-bootstrap".to_string(),
        project_root: root_dir.to_str().unwrap().to_string(),
    };
    let all_files_to_add = bootstrap_source.get_files()?;
    add_files_command(&mut index, &all_files_to_add, None, false).await?;

    // 3. rag build
    println!("Running: rag build");
    index.build(8, false).await?;

    // Write chunks to markdown file
    println!("Writing chunks to markdown file...");
    let storage_manager = StorageManager::new(index.root_dir.clone());
    let all_chunks = storage_manager.load_all_chunks().await?;
    let mut markdown_output = String::new();
    for chunk in all_chunks {
        writeln!(&mut markdown_output, "## Chunk UID: {}", chunk.uid)?;
        writeln!(&mut markdown_output, "### Title: {}", chunk.title)?;
        writeln!(&mut markdown_output, "### Summary: {}", chunk.summary)?;
        writeln!(&mut markdown_output, "### Source: {:?}", chunk.source)?;
        writeln!(&mut markdown_output, "```\n{}\n```\n", chunk.data)?;
    }
    let chunks_file_path = temp_path.join("chunks_output.md");
    write_string(chunks_file_path.to_str().unwrap(), &markdown_output, WriteMode::CreateOrTruncate)?;
    println!("Chunks written to {:?}", chunks_file_path);

    // 4. Self-improvement
    println!("Running: Self-improvement query");
    let actual_root_dir = ragit_utils::project_root::find_root()?;
    let self_path = actual_root_dir.join("crates/ragit-command-bootstrap/src/lib.rs");
    let self_code = read_string(self_path.to_str().unwrap())?;
    let prompt = format!(
        "The following is the Rust source code for a function that bootstraps a RAGIT index and then attempts to improve itself. Please review the code and provide an improved version. The improved version should be more robust, efficient, and clear. The function should also contain a query to reflect on its own functionality. Only output the complete, raw rust code for the file, without any explanations or markdown formatting.\n\n```rust\n{}\n```",
        self_code
    );
    let response = index.query(&prompt, vec![], None).await?;
    let improved_code = response.get_message();
    if !improved_code.is_empty() {
        write_string(self_path.to_str().unwrap(), improved_code, WriteMode::CreateOrTruncate)?;
        println!("Successfully improved self.");
    } else {
        println!("Self-improvement returned empty response, skipping.");
    }

    // 5. Final reflective query
    println!("Running: Final reflective query");
    let response = index.query("Explain the bootstrap_index_self function", vec![], None).await?;
    println!("{}", response.get_message());

    Ok(())
}
