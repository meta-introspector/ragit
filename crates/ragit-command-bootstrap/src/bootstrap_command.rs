use ragit_command_init::init_command_main;
use ragit_fs::{read_string, write_string, WriteMode};
use ragit_index_core::add_files::add_files_command;
use ragit_index_io::StorageManager;
use ragit_index_types::{index_struct::Index, load_mode::LoadMode};
use std::fmt::Write as FmtWrite;
use crate::file_source::FileSource;
use std::fs;
use anyhow::Result;
use crate::file_copy_utils;

pub async fn bootstrap_index_self(verbose: bool) -> Result<(), anyhow::Error> {
    if verbose {
        println!("bootstrap_index_self: Starting");
    }
    let actual_root_dir = ragit_utils::project_root::find_root()?;
    let temp_dir = actual_root_dir.join("tmp_bootstrap");
    
    fs::create_dir_all(&temp_dir)?;
    if verbose {
        println!("bootstrap_index_self: Created temporary directory: {:?}", temp_dir);
    }
    // 1. rag init
    if verbose {
        println!("bootstrap_index_self: Running rag init");
    }
    init_command_main(&["ragit".to_string(), "init".to_string()], Some(&temp_dir)).await?;
    if verbose {
        println!("bootstrap_index_self: Checking temp_dir permissions: {:?}", temp_dir.metadata()?.permissions());
        println!("bootstrap_index_self: Loading index");
    }
    let mut index = Index::load(temp_dir.to_path_buf(), LoadMode::OnlyJson)?;

    // 2. rag add crates/ragit-command-bootstrap
    if verbose {
        println!("bootstrap_index_self: Running rag add");
    }
    let actual_root_dir = ragit_utils::project_root::find_root()?;
    if verbose {
        println!("bootstrap_index_self: Found project root: {:?}", actual_root_dir);
    }
    let bootstrap_source = crate::file_source::CargoPackageFileSource { 
        package_name: "ragit-command-bootstrap".to_string(),
        project_root: actual_root_dir.to_str().unwrap().to_string(),
    };
    let original_files_to_add = bootstrap_source.get_files()?;
    if verbose {
        println!("bootstrap_index_self: Found {} files to add", original_files_to_add.len());
    }
    let temp_files_to_add = file_copy_utils::copy_files_to_temp_dir(
        &actual_root_dir,
        &temp_dir,
        original_files_to_add,
        verbose,
    ).await?;

    let prompts_dir = actual_root_dir.join("prompts");
    let temp_prompts_dir = temp_dir.join("prompts");
    fs::create_dir_all(&temp_prompts_dir)?;
    for entry in fs::read_dir(prompts_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let dest_path = temp_prompts_dir.join(path.file_name().unwrap());
            fs::copy(&path, &dest_path)?;
            if verbose {
                println!("bootstrap_index_self: Copied prompt {:?} to {:?}", path, dest_path);
            }
        }
    }

    let relative_temp_files_to_add = temp_files_to_add.iter().map(|p| {
        p.strip_prefix(&temp_dir).unwrap().to_string_lossy().into_owned()
    }).collect::<Vec<String>>();
    add_files_command(&mut index, &relative_temp_files_to_add, None, false).await?;
    if verbose {
        println!("bootstrap_index_self: Added files to index");
    }
    // 3. rag build
    if verbose {
        println!("bootstrap_index_self: Running rag build");
    }
    ragit_index_effects::build(&mut index, 8, false).await?;
    if verbose {
        println!("bootstrap_index_self: Built index");
    }
    // Write chunks to markdown file
    if verbose {
        println!("bootstrap_index_self: Writing chunks to markdown file");
    }
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
    let chunks_file_path = temp_dir.join("chunks_output.md");
    write_string(chunks_file_path.to_str().unwrap(), &markdown_output, WriteMode::CreateOrTruncate)?;
    if verbose {
        println!("bootstrap_index_self: Chunks written to {:?}", chunks_file_path);
    }
    // 4. Self-improvement
    if verbose {
        println!("bootstrap_index_self: Running self-improvement query");
    }
    let self_path = actual_root_dir.join("crates/ragit-command-bootstrap/src/lib.rs");
    let self_code = read_string(self_path.to_str().ok_or_else(|| anyhow::anyhow!("Invalid path for self_code: {:?}", self_path))?)?;
    let prompt = format!(
        "The following is the Rust source code for a function that bootstraps a RAGIT index and then attempts to improve itself. Please review the code and provide an improved version. The improved version should be more robust, efficient, and clear. The function should also contain a query to reflect on its own functionality. Only output the complete, raw rust code for the file, without any explanations or markdown formatting.\n\n```rust\n{}\n```",
        self_code
    );
    let response = ragit_index_query::query(&index, &prompt, vec![], None).await?;
    let improved_code = response.get_message();
    if !improved_code.is_empty() {
        let improved_self_path = temp_dir.join("improved_lib.rs");
        write_string(improved_self_path.to_str().unwrap(), improved_code, WriteMode::CreateOrTruncate)?;
        if verbose {
            println!("bootstrap_index_self: Successfully wrote improved self to {:?}", improved_self_path);
        }
    } else {
        if verbose {
            println!("bootstrap_index_self: Self-improvement returned empty response, skipping.");
        }
    }

    // 5. Final reflective query
    if verbose {
        println!("bootstrap_index_self: Running final reflective query");
    }
    let response = ragit_index_query::query(&index, "Explain the bootstrap_index_self function", vec![], None).await?;
    println!("{}", response.get_message());

    if verbose {
        println!("bootstrap_index_self: Finished");
    }
    Ok(())
}