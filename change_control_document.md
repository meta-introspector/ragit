## 6. New Tool Development: `ragit-code-analyzer` (SDLC: Design & Implementation Phases)

**Objective:** To efficiently traverse the `ragit` codebase, parse Rust source files in parallel, and extract essential information about declarations and dependencies. This tool will form the foundation for automated import management, prelude generation, and code topology analysis.

**Proposed New Crate:** `ragit-code-analyzer` (located in `crates/ragit-code-analyzer/`)

This crate will be responsible for:
*   Discovering all Rust source files.
*   Parsing each file's Abstract Syntax Tree (AST).
*   Extracting public declarations (functions, structs, enums, traits, modules).
*   Identifying all `use` statements to determine dependencies.
*   Collecting semantic information (e.g., comments, docstrings, identifier names) for future use in vector embedding.

### Implementation Steps:

#### 1. Scaffold `ragit-code-analyzer` Crate

**`crates/ragit-code-analyzer/Cargo.toml`:**
```toml
[package]
name = "ragit-code-analyzer"
version = "0.1.0"
edition = "2021"

[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits"] } # For Rust AST parsing
quote = "1.0" # Often used with syn, might be useful later
rayon = "1.10" # For parallel iteration
walkdir = "2.5" # For efficient directory traversal
anyhow = "1.0" # For simplified error handling
```

#### 2. File Discovery (Parallelized)

We'll use `walkdir` to efficiently traverse the file system and `rayon` to parallelize the collection of `.rs` file paths.

**`crates/ragit-code-analyzer/src/lib.rs` (initial structure):**
```rust
use walkdir::WalkDir;
use rayon::prelude::*;
use std::path::PathBuf;
use anyhow::Result;

/// Represents the extracted information from a single Rust source file.
#[derive(Debug)]
pub struct FileAnalysis {
    pub path: PathBuf,
    pub declarations: Vec<String>, // e.g., "struct MyStruct", "fn my_function"
    pub imports: Vec<String>,      // e.g., "use crate::module::Item"
    // Add more fields for semantic information as needed
}

pub fn analyze_codebase(root_dir: &PathBuf) -> Result<Vec<FileAnalysis>> {
    let rust_files: Vec<PathBuf> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs"))
        .map(|entry| entry.into_path())
        .collect();

    // Parallelize the parsing of each file
    let analyses: Vec<FileAnalysis> = rust_files.par_iter().filter_map(|file_path| {
        // Call a function to parse the file and extract info
        parse_rust_file(file_path).ok()
    }).collect();

    Ok(analyses)
}

// Placeholder for the parsing logic
fn parse_rust_file(file_path: &PathBuf) -> Result<FileAnalysis> {
    // This will contain the syn parsing logic
    // For now, return a dummy analysis
    Ok(FileAnalysis {
        path: file_path.clone(),
        declarations: vec![],
        imports: vec![],
    })
}
```

#### 3. AST Parsing & Data Extraction (Parallelized)

The `parse_rust_file` function will be the core of the parallel processing. It will use `syn` to parse the file content and extract the required information.

**`crates/ragit-code-analyzer/src/parser.rs` (new file):**
```rust
use syn::{File, Item, UseTree, UseGlob, UseName, UsePath, UseRename};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

/// Parses a single Rust file and extracts its declarations and imports.
pub fn parse_rust_file(file_path: &PathBuf) -> Result<super::FileAnalysis> {
    let content = fs::read_to_string(file_path)?;
    let syntax = syn::parse_file(&content)?;

    let mut declarations = Vec::new();
    let mut imports = Vec::new();

    for item in syntax.items {
        match item {
            Item::Fn(item_fn) => {
                if item_fn.vis.is_pub() {
                    declarations.push(format!("fn {}", item_fn.sig.ident));
                }
            }
            Item::Struct(item_struct) => {
                if item_struct.vis.is_pub() {
                    declarations.push(format!("struct {}", item_struct.ident));
                }
            }
            Item::Enum(item_enum) => {
                if item_enum.vis.is_pub() {
                    declarations.push(format!("enum {}", item_enum.ident));
                }
            }
            Item::Mod(item_mod) => {
                if item_mod.vis.is_pub() {
                    declarations.push(format!("mod {}", item_mod.ident));
                }
            }
            Item::Use(item_use) => {
                // Extract import paths
                extract_use_tree(&item_use.tree, &mut imports, String::new());
            }
            _ => {
                // Ignore other items for now
            }
        }
    }

    Ok(super::FileAnalysis {
        path: file_path.clone(),
        declarations,
        imports,
    })
}

// Helper function to recursively extract import paths from a UseTree
fn extract_use_tree(use_tree: &UseTree, imports: &mut Vec<String>, current_path: String) {
    match use_tree {
        UseTree::Path(path) => {
            let new_path = if current_path.is_empty() {
                path.ident.to_string()
            } else {
                format!("{}::{}", current_path, path.ident)
            };
            extract_use_tree(&path.tree, imports, new_path);
        }
        UseTree::Name(name) => {
            imports.push(format!("{}::{}", current_path, name.ident));
        }
        UseTree::Rename(rename) => {
            imports.push(format!("{}::{} as {}", current_path, rename.ident, rename.rename));
        }
        UseTree::Glob(glob) => {
            imports.push(format!("{}::*", current_path));
        }
        UseTree::Group(group) => {
            for tree in &group.items {
                extract_use_tree(tree, imports, current_path.clone());
            }
        }
    }
}
```

**Update `crates/ragit-code-analyzer/src/lib.rs` to use `parser.rs`:**
```rust
// ... (existing use statements)
mod parser; // Declare the new parser module

// ... (existing FileAnalysis struct)

pub fn analyze_codebase(root_dir: &PathBuf) -> Result<Vec<FileAnalysis>> {
    let rust_files: Vec<PathBuf> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rs"))
        .map(|entry| entry.into_path())
        .collect();

    // Parallelize the parsing of each file
    let analyses: Vec<FileAnalysis> = rust_files.par_iter().filter_map(|file_path| {
        parser::parse_rust_file(file_path).ok() // Use the new parser module
    }).collect();

    Ok(analyses)
}
```