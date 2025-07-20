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

This granular structure allows for universal composability and reusability of components.
