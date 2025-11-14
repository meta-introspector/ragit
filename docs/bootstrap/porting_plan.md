# Bootstrap Porting Plan: From `ragit-commands` to Single-File Worker

This document outlines the phased approach for porting core functionalities from the existing `ragit-commands` bootstrap implementation to the new, synchronous, single-threaded worker located in `crates/layer7_application/ragit-build-index-worker-single-file/`. The goal is to incrementally integrate features while maintaining stability and ensuring proper functionality through rigorous testing.

## Phase 1: Core Index Building Logic (Current Focus)

**Objective:** Implement the actual index building process, moving beyond the current placeholder. This involves chunking files, generating embeddings, and adding them to the `Index` struct.

**Tasks:**

1.  **Resolve `Index` Mutability Discrepancy:**
    *   Modify `crates/layer7_application/ragit-build-index-worker-single-file/src/main.rs` to correctly pass a mutable `Index` reference to `add_bootstrap_files` and `build_index`.
    *   Ensure `Index` is properly updated and persisted.

2.  **Implement Chunking:**
    *   Integrate or port the chunking logic from the old bootstrap or relevant `ragit-index` crates.
    *   This will involve reading file content, splitting it into manageable chunks, and associating metadata.
    *   **Relevant crates:** `ragit-readers`, `ragit-index-core`, `ragit-index-types`.

3.  **Integrate Embedding Generation (Placeholder/Mock):**
    *   For initial porting, use a placeholder or mock embedding generation to avoid external dependencies or complex LLM integrations. The focus is on the data flow.
    *   Eventually, this will involve calling an LLM or embedding model.

4.  **Add Chunks to Index:**
    *   Implement the logic to add the generated chunks (with their embeddings and metadata) to the `Index` struct.
    *   This will likely involve methods from `ragit-index-core` or `ragit-index-types`.

**Testing Strategy (Phase 1):**

*   **Unit Tests:**
    *   Create dedicated unit tests for the chunking logic to ensure files are correctly split and metadata is extracted.
    *   Test the `add_bootstrap_files` function to verify files are correctly staged and added to the `Index`.
    *   Test the `build_index` function (with mock embeddings) to ensure chunks are correctly added to the index structure.
*   **Integration Tests:**
    *   Run the `ragit-build-index-worker-single-file` executable with a small, controlled set of input files.
    *   Verify that the temporary directory contains the expected index file and that its content reflects the processed chunks.
    *   Use memory profiling to ensure memory usage remains within expected bounds.

## Phase 2: Self-Improvement and Reflective Queries

**Objective:** Port the self-improvement and reflective query functionalities, allowing the bootstrap process to analyze and improve its own codebase.

**Tasks:**

1.  **Port Self-Improvement Logic:**
    *   Integrate the code analysis, prompt generation, LLM query execution, and code writing logic.
    *   This will involve adapting the existing `perform_self_improvement` function and its dependencies.
    *   **Relevant crates:** `ragit-model`, `ragit-model-provider`, `ragit-pdl`, `ragit-query`.

2.  **Port Final Reflective Query:**
    *   Integrate the logic for executing a final reflective query against the built index.
    *   This will involve adapting the existing `perform_final_reflective_query` function.

**Testing Strategy (Phase 2):**

*   **Unit Tests:**
    *   Test prompt generation with various code inputs.
    *   Mock LLM responses to test the code analysis and improvement logic.
*   **Integration Tests:**
    *   Run the full bootstrap process with self-improvement enabled.
    *   Verify that the improved code is generated correctly.
    *   Verify the output of the final reflective query.

## Phase 3: Refinement and Optimization

**Objective:** Optimize performance, reduce memory footprint, and enhance the robustness of the bootstrap process.

**Tasks:**

1.  **Memory Optimization:**
    *   Analyze memory profiles generated during testing.
    *   Identify and address memory leaks or inefficient data structures.
    *   Explore strategies like streaming processing or more efficient data serialization.

2.  **Error Handling Enhancement:**
    *   Review and improve error handling across all ported components.
    *   Ensure graceful degradation and informative error messages.

3.  **Performance Tuning:**
    *   Benchmark critical sections of the code (e.g., chunking, embedding).
    *   Identify bottlenecks and apply performance optimizations.

**Testing Strategy (Phase 3):**

*   **Performance Benchmarks:**
    *   Establish baseline performance metrics.
    *   Run benchmarks after each optimization to measure impact.
*   **Stress Testing:**
    *   Test with large codebases and high memory limits to ensure stability.
*   **Fault Injection:**
    *   Simulate errors (e.g., file read failures, LLM timeouts) to test error handling.

## General Principles for Porting:

*   **One Declaration Per File:** Continue to adhere strictly to this principle for all new and refactored code.
*   **PathBuf Consistency:** Ensure consistent use of `PathBuf` for all file system operations.
*   **Incremental Changes:** Make small, testable changes and commit frequently.
*   **Leverage Existing Crates:** Reuse logic from existing `ragit` crates (e.g., `ragit-index-types`, `ragit-utils`, `ragit-readers`) where appropriate, rather than re-implementing.
*   **Memory Profiling:** Continuously monitor memory usage throughout the porting process using the integrated memory profiling tools.
