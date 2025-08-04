# Plan: Code Deduplication and Topological Analysis

This document outlines the plan to use the `ragit` system to analyze its own codebase, identify and eliminate duplicate content, and build a structural dependency graph of the code.

## The Problem

The project contains a significant amount of duplicate and near-duplicate documentation and code, created organically during its rapid evolution. This redundancy makes maintenance difficult and obscures the true, underlying structure of the system.

## The Solution

We will implement a three-phase plan to address this. The core idea is to move beyond simple text-based analysis and to understand the code's **topology**—the network of dependencies that defines how different chunks of code rely on each other to compile and run.

### Phase 1: Universal Ingestion

*   **Action:** Use the `ragit add` and `ragit build` commands to ingest the entire project repository into a new, dedicated knowledge base. This includes all `*.rs` files, `*.md` files, and `*.toml` files.
*   **Goal:** To have every piece of code and documentation broken down into content-addressed chunks, ready for analysis.

### Phase 2: Topological Analysis with `syn`

*   **Action:** Create a new analysis tool or command within the `ragit` ecosystem.
*   **Functionality:**
    1.  This tool will iterate through every chunk identified as Rust code.
    2.  It will use the `syn` crate to parse the Rust code into an Abstract Syntax Tree (AST).
    3.  By walking the AST, the tool will identify every symbol the chunk uses (e.g., function calls, struct names, traits).
    4.  It will then query the `ragit` index to find the chunks where those symbols are defined.
    5.  The result will be a **dependency graph**. For each chunk, we will have a list of the other chunks it depends on.
*   **Goal:** To create a rich, machine-readable understanding of the codebase's structure, stored as metadata alongside the chunks themselves.

### Phase 3: Deduplication and Refactoring Insights

*   **Action:** Leverage the dependency graph and the existing vector embeddings to find refactoring opportunities.
*   **Methods:**
    1.  **Exact Duplicates:** Identify chunks with identical UIDs (content hashes). This is the simplest form of deduplication.
    2.  **Topological Duplicates:** Find chunks that have identical or near-identical dependency lists. These are functions or structs that may be named differently but perform the same role.
    3.  **Semantic + Topological Duplicates:** Combine vector search with graph analysis. We can ask powerful questions like: *"Show me all functions that have a similar 'vibe' (vector) to this one AND call the same helper functions."*
*   **Goal:** To generate a prioritized list of refactoring candidates, allowing us to systematically remove redundancy and improve the overall code quality.
