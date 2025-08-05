## 6. Code Analysis Strategy: Leveraging Existing Tools (SDLC: Design & Implementation Phases)

**Objective:** To efficiently traverse the `ragit` codebase, parse Rust source files, and extract essential information about declarations and dependencies. This will form the foundation for automated import management, prelude generation, and code topology analysis. Instead of creating a new code analyzer from scratch, we are leveraging and wrapping existing, proven components within the project.

**Leveraged Crate:** `solfunmeme-core-logic` (now located in `crates/layer3_network/solfunmeme-core-logic/`)

This crate already contains a `CodeAnalyzer` struct capable of:
*   Parsing Rust source files into an Abstract Syntax Tree (AST) using `syn`.
*   Extracting declarations (functions, structs, enums, traits, modules).
*   Identifying duplicates and performing basic code metrics.

**New Wrapper Crate:** `ragit-code-analyzer` (located in `crates/layer3_network/ragit-code-analyzer/`)

This new crate will serve as a wrapper around `solfunmeme-core-logic::CodeAnalyzer`, exposing its functionality to the rest of the `ragit` project in a standardized manner. This adheres to our "write programs that edit files" philosophy by providing a programmatic interface for code analysis.

### Implementation Steps:

#### 1. Relocate `solfunmeme-core-logic`

The `solfunmeme-core-logic` crate has been moved from `vendor/meta-introspector/solfunmeme-dioxus/crates/solfunmeme_core_logic` to `crates/layer3_network/solfunmeme-core-logic`. Its `Cargo.toml` has been updated to remove `dioxus` related dependencies and ensure correct paths.

#### 2. Scaffold `ragit-code-analyzer` Crate (Wrapper)

**`crates/layer3_network/ragit-code-analyzer/Cargo.toml`:**
```toml
[package]
name = "ragit-code-analyzer"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0" # For simplified error handling
solfunmeme-core-logic = { path = "../solfunmeme-core-logic" } # Dependency on the moved crate
```

#### 3. Implement Wrapper Logic in `ragit-code-analyzer`

The `ragit-code-analyzer` crate will provide a simplified interface to the `CodeAnalyzer` from `solfunmeme-core-logic`.

**`crates/layer3_network/ragit-code-analyzer/src/lib.rs`:**
```rust
use anyhow::Result;
use std::path::PathBuf;
use solfunmeme_core_logic::CodeAnalyzer;

/// Represents the extracted information from a single Rust source file.
#[derive(Debug)]
pub struct FileAnalysis {
    pub file_path: PathBuf,
    pub declarations: Vec<String>, // e.g., "struct MyStruct", "fn my_function"
    pub imports: Vec<String>,      // e.g., "use crate::module::Item"
    // Add more fields for semantic information as needed, derived from CodeAnalysis
}

pub fn analyze_codebase(root_dir: &PathBuf) -> Result<Vec<FileAnalysis>> {
    let mut analyzer = CodeAnalyzer::new(100, 0.8); // Default dimensions and threshold
    let mut analyses = Vec::new();

    // This part needs to be adapted to iterate through files and call analyzer.analyze_file
    // For now, a placeholder for the actual file discovery and analysis loop.
    // The `ragit-command-bootstrap` will handle file discovery and pass individual files.

    Ok(analyses)
}
```

**`crates/layer3_network/ragit-code-analyzer/src/parser.rs` (Removed/Replaced)**

The direct `syn` parsing logic will be handled by `solfunmeme-core-logic::CodeAnalyzer`, so a separate `parser.rs` in `ragit-code-analyzer` is no longer needed for direct AST parsing. The `FileAnalysis` struct will be populated by mapping the results from `solfunmeme-core-logic::CodeAnalysis`.

#### 4. Integration with `ragit-command-bootstrap`

The `ragit-command-bootstrap` has been updated to dynamically determine the files to be added to the index based on a `--target` argument. This allows for indexing specific submodules like `rust-analyzer`.

**`crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/add_bootstrap_files.rs` (Updated Logic):**
The `add_bootstrap_files` function now uses `GlobFileSource` and constructs the glob pattern based on the `--target` argument. If `--target` is "all", it uses `**/*.rs`. If it's "rust-analyzer", it uses `vendor/rust-analyzer/**/*.rs`. This ensures that `rust-analyzer`'s code is included in the indexing process.

This revised strategy ensures we avoid code duplication, leverage existing robust components, and maintain a clean, modular architecture.
