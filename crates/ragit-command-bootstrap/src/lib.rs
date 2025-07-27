use std::path::Path;
use ragit_command_init::init_command_main;
use ragit_fs::{read_string, write_string, WriteMode};
use ragit_index_core::add_files::add_files_command;
use ragit_index_io::StorageManager;
use ragit_index_types::{index_struct::Index, load_mode::LoadMode};
use std::fmt::Write as FmtWrite;
use crate::file_source::FileSource;
use std::fs;

pub async fn bootstrap_index_self(temp_path: &Path) -> Result<(), anyhow::Error> {
    println!("bootstrap_index_self: Starting");
    // 1. rag init
    println!("bootstrap_index_self: Running rag init");
    init_command_main(&["ragit".to_string(), "init".to_string()], Some(temp_path)).await?;
    println!("bootstrap_index_self: Loading index");
    let mut index = Index::load(temp_path.to_path_buf(), LoadMode::OnlyJson)?;

    // 2. rag add crates/ragit-command-bootstrap
    println!("bootstrap_index_self: Running rag add");
    let actual_root_dir = ragit_utils::project_root::find_root()?;
    println!("bootstrap_index_self: Found project root: {:?}", actual_root_dir);
    let bootstrap_source = file_source::CargoPackageFileSource { 
        package_name: "ragit-command-bootstrap".to_string(),
        project_root: actual_root_dir.to_str().unwrap().to_string(),
    };
    let all_files_to_add = bootstrap_source.get_files()?;
    println!("bootstrap_index_self: Found {} files to add", all_files_to_add.len());

    add_files_command(&mut index, &all_files_to_add, None, false).await?;
    println!("bootstrap_index_self: Added files to index");

    // 3. rag build
    println!("bootstrap_index_self: Running rag build");
    ragit_index_effects::build(&mut index, 8, false).await?;
    println!("bootstrap_index_self: Built index");

    // Write chunks to markdown file
    println!("bootstrap_index_self: Writing chunks to markdown file");
    let storage_manager = StorageManager::new(index.root_dir.clone());
    let all_chunks = storage_manager.load_all_chunks().await?;
    let mut markdown_output = String::new();
    for chunk in all_chunks {
        writeln!(&mut markdown_output, "## Chunk UID: {}", chunk.uid)?;
        writeln!(&mut markdown_output, "### Title: {}", chunk.title)?;
        writeln!(&mut markdown_output, "### Summary: {}", chunk.summary)?;
        writeln!(&mut markdown_output, "### Source: {:?}", chunk.source)?;
        writeln!(&mut markdown_output, "```
{}
```
", chunk.data)?;
    }
    let chunks_file_path = temp_path.join("chunks_output.md");
    write_string(chunks_file_path.to_str().unwrap(), &markdown_output, WriteMode::CreateOrTruncate)?;
    println!("bootstrap_index_self: Chunks written to {:?}", chunks_file_path);

    // 4. Self-improvement
    println!("bootstrap_index_self: Running self-improvement query");
    let self_path = actual_root_dir.join("crates/ragit-command-bootstrap/src/lib.rs");
    let self_code = read_string(self_path.to_str().unwrap())?;
    let prompt = format!(
        "The following is the Rust source code for a function that bootstraps a RAGIT index and then attempts to improve itself. Please review the code and provide an improved version. The improved version should be more robust, efficient, and clear. The function should also contain a query to reflect on its own functionality. Only output the complete, raw rust code for the file, without any explanations or markdown formatting.\n\n```rust\n{}\n```",
        self_code
    );
    let response = ragit_index_query::query(&index, &prompt, vec![], None).await?;
    let improved_code = response.get_message();
    if !improved_code.is_empty() {
        write_string(self_path.to_str().unwrap(), improved_code, WriteMode::CreateOrTruncate)?;
        println!("bootstrap_index_self: Successfully improved self.");
    } else {
        println!("bootstrap_index_self: Self-improvement returned empty response, skipping.");
    }

    // 5. Final reflective query
    println!("bootstrap_index_self: Running final reflective query");
    let response = ragit_index_query::query(&index, "Explain the bootstrap_index_self function", vec![], None).await?;
    println!("{}", response.get_message());

    println!("bootstrap_index_self: Finished");
    Ok(())
}
