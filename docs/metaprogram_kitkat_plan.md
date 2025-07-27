# KitKat Metaprogram - Current State (2025-07-27)

## Overview
This document outlines the current state of the `ragit` project refactoring efforts as of July 27, 2025. The primary goal has been to improve modularity and resolve dependency issues, particularly cyclic dependencies between core crates, and to get the `ragit bootstrap` command working.

## Refactoring Actions Taken:
- **`impl Index` functions refactored:** Functions previously part of `impl Index` blocks in `ragit-index-effects` have been moved into individual, standalone files within `ragit-index-types/src/index_impl/`. Each function now resides in its own `.rs` file and is re-exported through `ragit-index-types/src/index_impl/mod.rs`.
- **`create_chunk_from` function moved:** The `create_chunk_from` function was extracted from `ragit-types` and moved to a newly created `ragit-chunk` crate (`crates/ragit-chunk/src/lib.rs`). This crate now handles the logic for creating chunks.
- **`FileError` and `FileErrorKind` moved:** To break a cyclic dependency between `ragit-error` and `ragit-fs`, `FileError` and `FileErrorKind` definitions were moved from `ragit-fs/src/lib.rs` to `ragit-types/src/file_error.rs`. Imports in `ragit-fs` and `ragit-error` were updated accordingly.
- **`uid_new_file` function moved:** To break a cyclic dependency between `ragit-fs` and `ragit-types`, the `uid_new_file` function was moved from `ragit-fs/src/lib.rs` to `ragit-utils/src/uid_helpers.rs`. Imports in `ragit-fs` and `ragit-index-effects` were updated.
- **Resolved `ragit-types` `api_response` module not found:** Removed `pub mod api_response;` and `pub use api_response::Response;` from `crates/ragit-types/src/lib.rs`.
- **Fixed `ragit-types` `api_error.rs` unresolved import:** Changed `use ragit_types::JsonType;` to `use crate::json_type::JsonType;` in `crates/ragit-types/src/api_error.rs`.
- **Removed `ragit-model` `prelude.rs` unresolved import:** Removed `pub use ragit_types::Response;` from `crates/ragit-model/src/prelude.rs`.
- **Added `ragit-error` dependency to `ragit-tfidf` and `ragit-model-query-response`:** Updated `Cargo.toml` files for these crates.
- **Removed unused imports in `ragit-tfidf` and `ragit-model-query-response`:** Cleaned up `lib.rs` files.
- **Fixed `ragit-session-query` unused variable warning:** Renamed `test_model` to `_test_model` in `crates/ragit-session-query/src/response.rs`.
- **Removed `ragit-api` `response` module not found:** Removed `pub mod response;` from `crates/api/src/lib.rs`.
- **Fixed `ragit-api` `request/mod.rs` unresolved import of `Response`:** Changed `use ragit_types::{ApiError as Error, Response};` to `use ragit_types::ApiError as Error;` and added `use ragit_session_query::response::Response;` in `crates/api/src/request/mod.rs`.
- **Added `InvalidRequest` variant to `ApiError`:** Added `#[error("invalid request")] InvalidRequest,` to `ApiError` enum in `crates/ragit-types/src/api_error.rs`.
- **Created `ragit-error-conversions` crate:** Created `crates/ragit-error-conversions` with `Cargo.toml` and `src/lib.rs` to handle `From<PdlError> for ApiError` using a `WrappedPdlError` struct to resolve orphan rule violation. Added to workspace members.
- **Updated `ragit-chunk` to use `WrappedPdlError`:** Modified `crates/ragit-chunk/src/lib.rs` to use `ragit_error_conversions::WrappedPdlError` for `PdlError` conversion.
- **Fixed `E0382` (use of moved value) errors in `ragit-chunk`:** Cloned `messages`, `model`, `dump_api_usage_at`, `dump_pdl_at`, `dump_json_at`, and `schema` in `crates/ragit-chunk/src/lib.rs`.
- **Removed duplicate `channel_imp.rs`:** Deleted `crates/ragit-index-effects/src/channel_imp.rs` and updated `crates/ragit-index-effects/src/lib.rs` to remove its re-export.
- **Refactored `ragit-index-effects/src/channel.rs`:** Defined `WorkerRequest` and `WorkerResponse` enums and updated `Channel` struct to use them.
- **Updated `ragit-index-effects/src/build_chunks.rs` and `ragit-index-effects/src/build_worker.rs`:** Modified to use the new `WorkerRequest` and `WorkerResponse` enums for inter-worker communication.
- **Fixed `ragit-index-effects/src/init_worker.rs`:** Updated to use `WorkerRequest` and `WorkerResponse` for sending and receiving messages. Removed duplicate `Channel` import.
- **Fixed `PathBuf` and `String` mismatches in `ragit-index-effects/src/build_worker.rs`:** Converted `String` to `PathBuf` where necessary for `HashMap` keys and `push` operations.
- **Fixed `Model::from_name` ambiguity in `ragit-index-effects/src/build_chunks.rs`:** Changed `&ragit_model::Model::from_name(&index.api_config.model))?.name` to `index.api_config.model.clone()` as `index.api_config.model` already holds the model name.
- **Fixed `Index::load` usage in `ragit-command-bootstrap`:** Changed `use ragit_index_core::load_index_from_path;` to `use ragit_index_types::index_struct::Index;` and `use ragit_index_types::load_mode::LoadMode;` and updated the call to `Index::load(index_path, LoadMode::OnlyJson)?;`.
- **Fixed `ragit-index-types/src/index_struct.rs`:** Changed `pub models: Vec<Model>,` to `pub models: Vec<ragit_model::Model>,` and updated the import to `use ragit_model::Model;`.
- **Fixed `ragit-index-types/src/index_impl/get_model_by_name.rs`:** Updated the function signature and return type to use `ragit_model::Model` explicitly.
- **Removed compile-time prompt loading:** From `crates/ragit-utils/src/prompts.rs` and `crates/ragit-utils/src/prelude.rs`.
- **Modified `bootstrap_index_self`:** To create a temporary directory within the project root and use relative paths for files added to the index.
- **Updated `crates/ragit-commands/src/main.rs`:** To reflect changes in `bootstrap_index_self` signature and remove temporary directory creation.
- **Updated `docs/lessons.md`:** With new insights regarding compile-time vs. runtime loading, specific error handling, and `cargo run` usage.
- **Introduced `dry_run_llm` flag:** Added a `dry_run_llm` boolean flag to `send_and_validate_chunk_response` in `ragit-chunk/src/chunk_creation_utils.rs` to log LLM queries to `tmp_bootstrap/llm_queries.md` instead of making actual API calls. This required propagating the flag through `create_chunk_from` in `ragit-chunk/src/create_chunk_from.rs`, `generate_chunk` in `ragit-readers/src/lib.rs`, `build` in `ragit-index-effects/src/build.rs`, `build_worker` in `ragit-index-effects/src/build_worker.rs`, and `WorkerRequest::BuildChunks` in `ragit-index-effects/src/channel.rs` and `ragit-index-effects/src/init_worker.rs`.
- **Fixed type mismatch:** Corrected a type mismatch (`String` vs. `&str`) in `ragit-chunk/src/chunk_creation_utils.rs` related to the `dry_run_llm` implementation.

## Current Issues / Remaining Challenges:
The `ragit bootstrap` command is still failing with an exit code 137 (out-of-memory), despite attempts to disable LLM calls via the `dry_run_llm` flag. This suggests that the memory issue is likely not solely due to LLM calls, or the `dry_run_llm` flag isn't fully mitigating the memory usage. The previous "Read-only file system (os error 30)" error has not reappeared, indicating the temporary directory permissions are likely resolved.

## Next Steps (Post-KitKat):
The immediate priority is to resolve the out-of-memory error and get the `ragit bootstrap` command to run successfully. This will involve:
1.  **Investigate memory usage:** Use memory profiling tools (if available and compatible with Android/Termux) or more detailed logging to pinpoint the exact memory-intensive operations during the `rag build` step, especially within `ragit-index-effects` and `ragit-readers`.
2.  **Optimize data processing:** Review the token and chunk processing logic in `ragit-readers` and `ragit-chunk` for potential memory optimizations.
3.  **Review `tokio` features:** Revisit the `tokio` feature set in `crates/ragit-index-core/Cargo.toml` to ensure only necessary features are enabled, as `full` can be memory-intensive.
4.  **Consider smaller test cases:** If the current `bootstrap` process is too large to debug effectively, create a minimal test case that reproduces the memory issue.
5.  **Run Tests:** Execute `cargo check` and `cargo test` to ensure all changes are valid and no regressions were introduced.
6.  **Code Cleanup:** Clean up any remaining warnings and remove the temporary `println!` statements and `dry_run_llm` flags once the memory issue is resolved.
7.  **Functional Verification:** Manually test key `ragit` commands (`add`, `build`, `query`) to confirm they work as expected.
8.  **Documentation Update:** Continue to update relevant documentation files to reflect the new module structure and API changes.
