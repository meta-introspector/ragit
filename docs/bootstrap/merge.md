## Bootstrap Merging Plan

### Summary of Differences:

*   **Main Entry Points:** The old bootstrap uses `bootstrap_index_self` as its main entry point, which is an `async` function and takes many configurable parameters. The new bootstrap uses a synchronous `run` function with hardcoded `false` for `verbose` and `None` for `max_memory_gb`, `max_iterations`, and `max_files_to_process`.
*   **Asynchronous vs. Synchronous:** The old bootstrap is largely asynchronous (`async/await`), while the new one appears to be synchronous. This is a significant architectural difference.
*   **Configurability:** The old bootstrap is highly configurable via command-line flags, passing these parameters down to its sub-functions. The new bootstrap hardcodes many of these parameters to `false` or `None`.
*   **Memory Monitoring:** Both use `MemoryMonitor`, but the old one initializes it with `verbose`, `time_threshold_ms`, and `memory_threshold_bytes`, while the new one uses `MemoryMonitor::new()` without arguments.
*   **Functionality:**
    *   The old bootstrap includes `configure_memory_settings`, `perform_self_improvement`, and `perform_final_reflective_query`, which are absent in the new bootstrap.
    *   The old bootstrap explicitly calls `export_chunks_main::write_chunks_to_markdown`, while the new one imports the module but doesn't call the function.
    *   The new bootstrap includes `ragit_index_types::word_counter::count_words_in_chunks`, which is absent in the old one.
*   **Prompt Loading:** The old bootstrap explicitly loads prompts into the `index` after copying them. The new bootstrap copies them but doesn't explicitly show the loading step.

### Proposed Plan for Merging:

The goal is to consolidate the functionality and configurability of the old bootstrap into the new bootstrap, while maintaining the "one declaration per file" principle and addressing the synchronous/asynchronous differences.

1.  **Standardize `MemoryMonitor` Initialization:**
    *   **COMPLETED:** Modified `ragit-build-index-worker-single-file/src/bootstrap_process.rs` to initialize `MemoryMonitor` with configurable `verbose`, `time_threshold_ms`, and `memory_threshold_bytes` parameters, similar to the old bootstrap. These parameters are now passed down from the `run` function.

2.  **Introduce Configurability to New Bootstrap:**
    *   **COMPLETED:** Modified the `run` function in `ragit-build-index-worker-single-file/src/bootstrap_process.rs` to accept parameters for `verbose`, `max_memory_gb`, `max_iterations`, `max_files_to_process`, `max_chunk_size`, `max_summary_len`, `min_summary_len`, `disable_write_markdown`, `disable_self_improvement`, `disable_final_query`, and `disable_cleanup`. These parameters are now passed down to the respective functions.

3.  **Reconcile `copy_prompts`:**
    *   **COMPLETED:** Ensured that `ragit-build-index-worker-single-file/src/bootstrap_process.rs` explicitly loads the prompts into the `index` after copying, similar to how it's done in the old bootstrap. The `run` function is now `async`.

4.  **Integrate Missing Functionality from Old Bootstrap:**
    *   **`configure_memory_settings`:** **COMPLETED:** Reintroduced this functionality into the new bootstrap's workflow as a separate function call within `run`.
    *   **`export_chunks_main::write_chunks_to_markdown`:** **COMPLETED:** Added a call to this function in the new bootstrap, making its execution conditional on a `disable_write_markdown` flag.
    *   **`perform_self_improvement` and `perform_final_reflective_query`:** **COMPLETED:** Integrated these functions into `bootstrap_process.rs`, controlled by `disable_self_improvement` and `disable_final_query` flags.

5.  **Address `ragit_index_types::word_counter::count_words_in_chunks`:**
    *   **KEPT:** This functionality remains in the new bootstrap.

6.  **Refactor `bootstrap_command.rs`:**
    *   **COMPLETED:** The `bootstrap_command.rs` file has been refactored to execute `ragit-build-index-worker-single-file` as a separate process, passing all relevant arguments. This centralizes the bootstrap logic in `ragit-build-index-worker-single-file`.

### Next Step:

Propose a commit to save these changes.