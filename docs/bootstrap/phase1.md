# Bootstrap Command - Phase 1: Minimal Single-Threaded Worker

This document outlines the initial phase of creating a minimal, single-threaded build index worker (`ragit-build-index-worker-single-file`) to understand and profile memory usage during the `ragit` bootstrap process.

## Objectives Achieved:

*   **Crate Creation**: A new Rust crate (`ragit-build-index-worker-single-file`) was created under `crates/layer7_application/`.
*   **Synchronous Execution**: The worker is designed to be strictly synchronous, avoiding `async/await` and `tokio` to simplify profiling.
*   **Memory Profiling Integration**:
    *   Initial attempts with `tikv-jemallocator` were made but abandoned due to linking issues and complexity.
    *   A custom memory profiling utility (`ragit_utils::memory_utils`) was integrated to provide real-time memory usage statistics (Total, Used, Process RSS, Delta).
    *   The memory output is now formatted for human readability (KB, MB, GB).
    *   A "poor man's profiler" was implemented to collect memory snapshots at various stages and present them in a summary table at the end of execution.
*   **Bootstrap Function Integration (Synchronous Placeholders)**:
    *   `setup_environment`: Integrated to handle temporary directory creation and initial index setup.
    *   `copy_prompts`: Integrated to copy prompt files to the temporary directory.
    *   `add_bootstrap_files`: Integrated with a synchronous placeholder for adding files to the index. This involved creating a local `add_files_sync` function to replace the original asynchronous `add_files_command`.
    *   `build_index`: Integrated with a synchronous placeholder that iterates through staged files.

## Challenges Encountered:

*   **Asynchronous to Synchronous Conversion**: The primary challenge was adapting asynchronous functions from the `ragit-command-bootstrap` and `ragit-index-core` crates to a synchronous context. This required re-implementing core logic or creating synchronous placeholders.
*   **Macro Usage**: Rust's macro system proved challenging, particularly with `println!` and `format!` expecting string literals. This was resolved by creating a dedicated `print_memory_table` function in `memory_profiler.rs` and removing the problematic macro definitions from `constants.rs`.
*   **String Literal Handling**: Repeated issues with `write_file` due to incorrect handling of multi-line string literals and escaped characters. This highlighted the need for extreme precision when using this tool.
*   **Dependency Management**: Ensuring correct relative paths for inter-crate dependencies within the workspace.

## Next Steps (Phase 2):

*   **Implement Actual Index Building**: Replace the placeholder `build_index` logic with a minimal, synchronous implementation of the actual index building process. This will involve understanding the chunking and indexing logic from `ragit-index-core`.
*   **Further Memory Analysis**: Continue to monitor and analyze memory usage as more complex logic is integrated.

## KitKat Time!

We've made significant progress in setting up our minimal worker and profiling infrastructure. It's time for a well-deserved break.
