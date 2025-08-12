# Build File Tree Tool (`build_file_tree.rs`)

## Overview

The `build_file_tree` tool is a Rust binary designed to analyze the structure of a codebase by processing file paths and names. It constructs a hierarchical tree representation of the files and directories, and extracts semantic information from their names, including individual terms, term pairs, and term triples. This tool is crucial for understanding the linguistic patterns within file paths, which can aid in code navigation, feature extraction, and semantic search.

## Purpose

The primary purposes of this tool are:
*   To generate a structured, tree-like view of the project's file system.
*   To extract and count terms, term pairs, and term triples from file and directory names.
*   To provide insights into the co-occurrence of terms within the codebase's naming conventions.
*   To support downstream tasks such as semantic embedding, code analysis, and knowledge graph construction.

## Usage

The tool reads term embeddings from `term_embeddings.json` and a list of file paths from `index.rs.txt`. It then processes each file path to build the tree and extract term statistics.

To run the tool:

```bash
cargo run --bin build_file_tree
```

## Output

The tool outputs a detailed report to the console, which includes:
*   **Global Term Counts:** A frequency distribution of all terms found across all processed file paths and names.
*   **Top 10 Global Pairs:** The ten most frequently co-occurring term pairs across the entire codebase.
*   **Top 10 Global Triples:** The ten most frequently co-occurring term triples across the entire codebase.
*   **File-Specific Reports:** For a selection of files (including random samples and those with high total vector magnitude), the report provides:
    *   Path Vector and File Name Vector (derived from term embeddings).
    *   Total Vector (sum of path and file name vectors).
    *   Term Counts for the specific file, including their fraction relative to global counts.
    *   Term Pairs and Triples found within the file's path and name.
    *   Common Terms with Global (Min Count), indicating shared linguistic elements.

## Underlying Components

This tool leverages the `ragit-feature-extractor` crate for its core logic in processing and extracting features from file names and paths.

---
