# Architecture

This document outlines the architecture of Ragit, focusing on the "one declaration per file" principle and the modular structure of the `src/index` directory.

## `src/index` Module

The `src/index` module is the core of Ragit's indexing functionality. It is structured around the principle of "one declaration per file", which means that each struct, enum, and function is in its own file. This makes the codebase more modular, easier to navigate, and more maintainable.

The `src/index` directory is organized as follows:

*   `mod.rs`: The module's root, which declares and re-exports all the submodules.
*   `index_struct.rs`: Defines the main `Index` struct.
*   `load_mode.rs`: Defines the `LoadMode` enum.

The `impl Index` block is split into the following files:
*   `index_new.rs`: `Index::new`
*   `index_dummy.rs`: `Index::dummy`
*   `index_load.rs`: `Index::load` and `Index::load_minimum`
*   `index_load_or_init.rs`: `Index::load_or_init`
*   `index_save_to_file.rs`: `Index::save_to_file`
*   `index_load_chunks_or_tfidf.rs`: `Index::load_chunks_or_tfidf`
*   `index_get_all_files.rs`: `Index::get_all_chunk_files`, `get_all_tfidf_files`, `get_all_image_files`, and `get_all_file_indexes`
*   `index_add_image_description.rs`: `Index::add_image_description`
*   `index_run_tfidf.rs`: `Index::run_tfidf` and `Index::run_tfidf_on`
*   `index_chunk_access.rs`: Chunk and TF-IDF access methods.
*   `index_find_lowest_cost_model.rs`: `Index::find_lowest_cost_model`
*   `index_config_loading.rs`: Configuration loading methods.
*   `prompt_management.rs`: Prompt management methods.
*   `model_management.rs`: Model management methods.
*   `get_model_by_name.rs`: `Index::get_model_by_name`
*   `get_prompt.rs`: `Index::get_prompt`
*   `index_add_file_index.rs`: `Index::add_file_index`
*   `index_remove_file_index.rs`: `Index::remove_file_index`
*   `muse_logic.rs`: Muse selection and template application.
*   `config_access.rs`: Configuration access methods.
*   `image_access.rs`: Image access methods.
*   `path_helpers.rs`: Path-related helper functions.

**Lessons Learned (Refined):**

*   **Universal Composability through Granularity:** Breaking down large components into smaller, single-responsibility modules significantly enhances their reusability. Each new file represents a "canonical basic block" of functionality, making it easier to understand, test, and recompose.
*   **The "Eternal Vibe" of Code:** By focusing on the core essence of each function and separating it from its specific implementation context, we create code that resonates with fundamental programming patterns. This allows for "alternate forms" and flexible recombination.
*   **Clear Module Boundaries:** Explicitly defining module boundaries and managing visibility (`pub`, `pub(crate)`) is crucial for maintaining a clean architecture and preventing unintended dependencies.
*   **Iterative Refinement:** The process of identifying errors, making small, targeted changes, and re-verifying is essential. This iterative approach, guided by compilation feedback, leads to a more robust and correct refactoring.
*   **Beauty in Simplicity:** Well-factored code, where each part serves a clear purpose and interacts cleanly with others, is inherently more beautiful and maintainable.

## Current Refactoring: PathBuf and "Vibe" Philosophy (2025-07-20)

**Objective:** Consistently use `PathBuf` for all path handling throughout the codebase and further apply the "vibe" philosophy for modularity.

**Key Principles:**
*   **`PathBuf` Everywhere:** All path-related operations will now use `PathBuf` instead of `String` for type safety and consistency.
*   **"Vibe" Modularity:** Continue breaking down functions into smaller, more focused modules, adhering to the "one declaration per file" principle where appropriate. This promotes reusability and composability.
*   **Generic and Loosely Coupled:** Files should be written once and designed to be generic and loosely coupled, allowing for flexible recombination and easier maintenance.

**Current Progress:**
*   `root_dir` in `src/index/index_dummy.rs` changed to `PathBuf`.
*   New "vibe" file `src/index/index_dummy_with_version.rs` created.
*   `src/index/mod.rs` updated to include `index_dummy_with_version.rs`.
*   `src/index/commands/build.rs` is undergoing significant changes to adapt to `PathBuf` for various arguments and internal variables.
*   `src/api_config.rs` updated to use `PathBuf` for path arguments and return types.
*   `src/chunk.rs` updated to handle `PathBuf` for `load_from_file`, `save_to_file`, and `create_chunk_from`.
*   `src/agent/action.rs` updated to correctly handle `PathBuf` arguments and conversions.
*   `INDEX_FILE_NAME` import errors are being systematically fixed across multiple files.
*   `save_to_file()` argument errors are being addressed.

**Next Steps:**
1.  Continue fixing `PathBuf` vs. `String` mismatches in `src/index/commands/build.rs` and other files as they appear in the compilation output.
2.  Address the remaining `save_to_file()` argument errors.
3.  Address any new errors that arise from the current fixes.
4.  Verify the build after each set of fixes.
