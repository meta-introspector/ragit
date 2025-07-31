# Ragit Core Functionality Executable: `ragit-build-index-worker-single-file`

The `ragit-build-index-worker-single-file` executable is a self-contained tool designed to perform core `ragit` functionalities, including bootstrapping the index and querying it. This consolidation aims to provide a streamlined experience for testing and development, allowing for easy execution of fundamental operations within a single binary.

**Important Distinction:** This document describes the internal `bootstrap` subcommand of the `ragit-build-index-worker-single-file` executable. For information on the higher-level `rag bootstrap` command (which orchestrates `rag init`, `rag add`, `rag build`, and `rag query`), please refer to `docs/commands/bootstrap.md`.

## 1. `bootstrap` Subcommand

The `bootstrap` subcommand of `ragit-build-index-worker-single-file` is a comprehensive self-improving tool. It builds an index from the `ragit` codebase and then uses that index to analyze and improve the code itself.

### Usage

```bash
cargo run --package ragit-build-index-worker-single-file -- bootstrap [FLAGS]
```

### Flags

*   **Verbose Output:** Verbose output is always enabled for the `bootstrap` subcommand, providing detailed memory usage metrics and process flow information.
*   `--timeout-seconds <SECONDS>`: Sets a timeout for the bootstrap operation.
*   `--max-iterations <NUMBER>`: Exits gracefully after a certain number of iterations. This is primarily for debugging and will cause the process to exit gracefully once the specified number of iterations is reached.
*   `--max-memory-gb <NUMBER>`: Sets a maximum memory limit in gigabytes for the operation. **Important Note:** This flag is designed for the `ragit` application to *gracefully exit* if its memory usage exceeds the specified limit. It does **not** prevent the operating system from terminating the process due to an Out-Of-Memory (OOM) condition if the system's overall memory is exhausted. If you're experiencing OOM kills (exit code 137), it means the process is consuming memory faster than the application can detect and react, or the system's memory is simply too constrained.
*   `--max-files-to-process <NUMBER>`: Limits the number of files to process during the bootstrap.
*   `--max-chunk-size <NUMBER>`: Sets the maximum size of a chunk.
*   `--max-summary-len <NUMBER>`: Sets the maximum length of a summary.
*   `--min-summary-len <NUMBER>`: Sets the minimum length of a summary.
*   `--time-threshold-ms <NUMBER>`: Sets a time threshold in milliseconds for memory monitoring.
*   `--memory-threshold-bytes <NUMBER>`: Sets a memory threshold in bytes for memory monitoring.
*   `--disable-write-markdown`: Disables writing chunks to markdown.
*   `--disable-memory-config`: Disables memory configuration.
*   `--disable-prompt-copy`: Disables copying prompts.
*   `--disable-file-add`: Disables adding files.
*   `--disable-index-build`: Disables index building.
*   `--disable-self-improvement`: Disables the self-improvement phase.
*   `--disable-final-query`: Disables the final reflective query.
*   `--disable-cleanup`: Disables the cleanup of the temporary directory after the bootstrap process.

### Workflow

The `bootstrap` subcommand executes a series of operations, each managed by a dedicated function within the `bootstrap_commands` module (or its refactored equivalents). Memory usage is monitored and logged at key steps using `MemoryMonitor`.

1.  **Setup Environment (`setup_environment`):**
    *   Creates a temporary directory.
    *   Initializes a new `ragit` repository within the temporary directory.
    *   Initializes a new `Index` structure.
    *   A memory snapshot is captured and logged after index initialization.

2.  **Copy Prompts (`copy_prompts`):**
    *   Copies the `prompts` directory from the actual root to the temporary directory.
    *   Loads copied prompts into the in-memory `Index` structure. This step was crucial in resolving `PromptMissing` errors, as prompts must be loaded *after* being copied to the temporary directory.

3.  **Add Bootstrap Files (`add_bootstrap_files`):**
    *   Identifies and copies relevant `.rs` files (e.g., from the `ragit-command-bootstrap` package) to the temporary directory.
    *   Adds the copied files to the `Index`. The `--max-files-to-process` flag limits the number of files processed here.

4.  **Build Index (`build_index`):**
    *   Builds the `Index` from the source code, processing the content of the added `.rs` files.
    *   Memory snapshots are captured and logged before and after the build process.

5.  **Export Chunks to Content-Addressable Store (`export_chunks`):**
    *   Writes each chunk to a content-addressable store (similar to `.git/objects`) based on its SHA-1 hash.
    *   Memory snapshots are captured and logged before and after this step.

6.  **Self-Improvement (`perform_self_improvement`):**
    *   Analyzes and improves code by reading its own source, generating a prompt, executing a query, and writing improved code to a file. This phase is now iterative, controlled by the `--max-iterations` flag, allowing for repeated cycles of analysis and improvement.
    *   Memory snapshots are captured and logged at various stages of this process.

7.  **Final Reflective Query (`perform_final_reflective_query`):**
    *   Executes a hardcoded query against the built index, and prints the response. This step serves as a final verification of the index's query capabilities.
    *   Memory snapshots are captured and logged before and after this step.

---

## Refactoring History and Key Learnings

The `bootstrap` process has undergone significant refactoring to unify and streamline its implementation. Initially, there were two distinct bootstrap implementations: an "old" one (`ragit-command-bootstrap`) and a "new" one (`ragit-build-index-worker-single-file`). The goal was to centralize the core bootstrap logic in `ragit-build-index-worker-single-file` and refactor `ragit-command-bootstrap` to act as a configurable wrapper.

### Merging Strategy and Implementation Steps:

1.  **Standardize `MemoryMonitor` Initialization:** `ragit-build-index-worker-single-file/src/bootstrap_process.rs` was modified to initialize `MemoryMonitor` with configurable parameters (`verbose`, `time_threshold_ms`, `memory_threshold_bytes`).
2.  **Introduce Configurability to New Bootstrap:** The `run` function in `ragit-build-index-worker-single-file/src/bootstrap_process.rs` was updated to accept all relevant parameters (e.g., `verbose`, `max_memory_gb`, `max_iterations`, `max_chunk_size`, `disable_write_markdown`, `disable_self_improvement`, `disable_final_query`, `disable_cleanup`) and was made `async`.
3.  **Reconcile `copy_prompts`:** Ensured `ragit-build-index-worker-single-file/src/bootstrap_process.rs` explicitly calls `ragit_index_types::index_impl::load_prompts::load_prompts_from_directory` after copying prompts.
4.  **Integrate Missing Functionality:** Functionalities like `configure_memory_settings`, `export_chunks_main::write_chunks_to_markdown`, `perform_self_improvement`, and `perform_final_reflective_query` were integrated into `ragit-build-index-worker-single-file/src/bootstrap_process.rs`, controlled by respective flags.
5.  **Refactor `bootstrap_command.rs`:** `ragit-command-bootstrap/src/bootstrap_command.rs` was modified to execute `ragit-build-index-worker-single-file` as a separate subprocess using `tokio::process::Command`, passing all relevant arguments.

### Key Learnings from Refactoring:

*   **Asynchronous Programming with `tokio`:** Correctly using `tokio::process::Command` for external process execution was crucial. Direct manipulation of `std::env::args()` for inter-crate communication proved incorrect; explicit argument passing to subprocesses is the correct approach.
*   **`Cargo.toml` Dependencies and Features:** Ensuring that `tokio`'s `process` feature was enabled in `ragit-command-bootstrap`'s `Cargo.toml` was a critical fix for compilation errors.
*   **"One Declaration Per File" Principle:** This principle, while increasing the number of files, significantly improved modularity and made it easier to isolate and refactor specific functionalities.
*   **Systematic Debugging:** Addressing errors one by one, starting with the most fundamental ones (like missing dependencies or incorrect `use` statements), proved to be the most effective strategy.
*   **Documentation as a Guide:** Existing documentation (like the previous `docs/bootstrap.md` and `docs/bootstrap/merge.md`) was invaluable in understanding the original design and planning the merging process.

---

## 2. `query` Subcommand

The `query` subcommand allows you to interact with a `ragit` index to retrieve information. It supports both single-turn and multi-turn queries, and can output results in JSON format.

### Usage

```bash
cargo run --package ragit-build-index-worker-single-file -- query <QUERY_STRING> [FLAGS]
```

### Arguments

*   `<QUERY_STRING>`: The query string to search for in the index.

### Flags

*   `--no-pdl`: Disables PDL (Prompt Description Language) processing for the query.
*   `--multi-turn`: Enables multi-turn conversational mode. In this mode, you can ask follow-up questions until you type `.exit`.
*   `--json`: Outputs the query result in JSON format.
*   `--kb-path <PATH>`: Specifies the path to the knowledge base (index) to query. If not provided, it defaults to the current project root.

### Workflow

The `query` subcommand performs the following steps:

1.  **Load Index:** Loads the `ragit` index from the specified knowledge base path (or the current project root if not provided).
2.  **Execute Query:** Executes the provided query against the loaded index.
3.  **Process Response:** Processes the response from the query, handling multi-turn conversations and JSON output as requested.

---

## Debugging and Memory Management

If you encounter issues with the `ragit-build-index-worker-single-file` executable, especially Out-Of-Memory (OOM) errors, here's what we've learned and how to approach debugging:

### Flags for Debugging

*   `--verbose`: Enables verbose output, which is useful for debugging. The build dashboard now includes more detailed memory usage metrics, such as the delta since the beginning of the process and the average memory usage per loop.
*   `--max-iterations <NUMBER>`: Exits gracefully after a certain number of iterations. This is primarily for debugging and will cause the process to exit gracefully once the specified number of iterations is reached.
*   `--max-memory-gb <NUMBER>`: Sets a maximum memory limit in gigabytes for the operation. **Important Note:** This flag is designed for the `ragit` application to *gracefully exit* if its memory usage exceeds the specified limit. It does **not** prevent the operating system from terminating the process due to an Out-Of-Memory (OOM) condition if the system's overall memory is exhausted. If you're experiencing OOM kills (exit code 137), it means the process is consuming memory faster than the application can detect and react, or the system's memory is simply too constrained.

### Understanding OOM Kills (Exit Code 137)

An exit code of 137 typically indicates that the process was terminated by the operating system due to an Out-Of-Memory (OOM) condition. This can happen even if the application is attempting to shut down gracefully, as the system may run out of memory before the application completes its cleanup.

To mitigate persistent OOM issues:
1.  **Increase available RAM:** Ensure your system has sufficient free RAM.
2.  **Run on a more powerful system:** Consider executing the command on a machine with more memory.
3.  **Reduce the scope:** If possible, try to process fewer files or smaller files during the operation (e.g., by adjusting the `CHUNK_PROCESSING_LIMIT` if applicable, or by targeting specific subsets of files).

### Full Backtrace

To get a detailed backtrace in case of a crash, set the `RUST_BACKTRACE` environment variable to `full`:

```bash
export RUST_BACKTRACE=full
cargo run --package ragit-build-index-worker-single-file -- bootstrap # or query
```

This will provide a more comprehensive stack trace, which can be invaluable for debugging panics and other runtime errors.

### General Lessons Learned (Applicable to Bootstrap)

*   **Consistent Path Handling:** Meticulous attention to `PathBuf` conversions and consistent use of helper functions (e.g., `pathbuf_to_str`, `str_to_pathbuf`, `join_paths`) is crucial to avoid errors.
*   **Module Visibility and Re-exports:** Correctly declaring `pub mod` and `pub use` in `lib.rs` or `mod.rs` files is essential for module accessibility.
*   **Error Trait Implementations:** Custom error enums need to explicitly implement `From` traits for all wrapped external error types to enable proper error propagation.
*   **Compile-time vs. Runtime Resource Loading:** Be mindful of how resources (like prompts) are loaded. Prefer runtime loading for dynamic paths, especially in environments like the `bootstrap` command's temporary directory.
*   **`cargo check` vs. `cargo run`:** `cargo check` is for quick syntax and type checking, but `cargo run` (or `cargo test`) is necessary to catch runtime errors and environmental issues.
*   **`cargo run --package <package_name> -- <args>`:** Always use this syntax to specify the correct package and pass arguments to its binary when running within a workspace.

### Internal Module Structure and Imports

During development, the `memory_utils` module was moved from `ragit-command-bootstrap` to `ragit-utils` to resolve cyclic dependencies and improve modularity. To maintain compatibility with existing `use` statements within `ragit-command-bootstrap`, a shim was introduced in `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/mod.rs` (specifically, `pub use crate::memory_utils;`). This allows other modules within `ragit-command-bootstrap` to continue using `crate::bootstrap_commands::memory_utils` without needing to update every single import path. This approach prioritizes functionality and avoids extensive refactoring when a simpler solution is available.

---

## Future Work: Continuing Bootstrap Instrumentation and Testing

The goal is to continue instrumenting and testing the bootstrap process, focusing on completing the self-improvement feedback loop and refining the overall system.

### Phase 1: Verification and Stabilization

1.  **Run Full Workspace Tests:** Execute `cargo test --workspace` to ensure all existing tests pass and there are no compilation errors after the recent refactoring.
2.  **Bootstrap Execution Verification:** Run `cargo run --package ragit-build-index-worker-single-file -- bootstrap --max-iterations 1 --verbose` to:
    *   Confirm the command runs without panics and exits gracefully.
    *   Verify that prompts are loaded correctly and the `PromptMissing` error is resolved.
    *   Observe verbose output for initial memory usage and process flow.

### Phase 2: Completing the Self-Improvement Feedback Loop

1.  **Integrate Code Compilation & Testing:** Modify the `perform_self_improvement` function to include steps for compiling and running tests on the LLM-generated code. If compilation or tests fail, log the failure and potentially trigger a retry or alternative strategy.
2.  **Implement LLM Output Evaluation:** Develop a basic mechanism to evaluate the quality of the LLM's output (e.g., checking for syntax, specific keywords, or adherence to coding standards). This evaluation will inform subsequent self-improvement iterations.

### Phase 3: Refinement and Optimization

1.  **Structured Logging:** Refine the logging output to be more concise and structured, improving readability and debuggability.
2.  **Consolidate Self-Improvement Logic:** Identify and eliminate any remaining code duplication related to the `perform_self_improvement` logic, ensuring a single, authoritative implementation.
3.  **Granular Memory Monitoring:** Further utilize `time_threshold_ms` and `memory_threshold_bytes` flags for more precise memory profiling and control during the bootstrap process.
4.  **Enhanced Error Handling:** Improve error messages to be more informative and ensure robust error handling across all bootstrap stages.

### Phase 4: Continuous Testing and Self-Application

1.  **Automated Bootstrap Tests:** Develop automated tests specifically for the bootstrap process, covering various configurations and scenarios.
2.  **Self-Application:** Regularly run the `bootstrap` command on the `ragit` codebase itself to continuously validate and improve the system's self-improvement capabilities.
