# Refactoring Plan for `ragit-utils` Crate

Based on the compilation errors from `cargo build` on 2025-07-21, the `ragit-utils` crate is the primary source of issues. All 29 errors originate from `crates/ragit-utils/src/chunk/mod.rs`.

The core strategy is to refactor this oversized and complex file into smaller, single-responsibility modules following the "one declaration per file" principle. This will improve clarity, resolve scope conflicts, and make the code easier to debug.

## Phase 1: Refactor `chunk/mod.rs`

The file `crates/ragit-utils/src/chunk/mod.rs` will be split into a new `chunk` directory (`crates/ragit-utils/src/chunk/`).

1.  **Create Directory Structure**:
    *   Create `crates/ragit-utils/src/chunk/`.
    *   The existing `mod.rs` will be replaced with a new one that declares and exports the new submodules.

2.  **Split Declarations into Files**: Each struct, enum, and `impl` block will be moved to its own file.
    *   `chunk_struct.rs`: `pub struct Chunk`
    *   `chunk_extra_info.rs`: `pub struct ChunkExtraInfo`
    *   `rendered_chunk.rs`: `pub struct RenderedChunk`
    *   `impl_chunk.rs`: The main `impl Chunk` block containing methods like `new`, `dummy`, `get_content_hash`, etc.
    *   `render.rs`: The conflicting `render` methods. This is the most critical part. The duplicate `render` definitions (one sync, one async) will be separated and one will be renamed to resolve the E0592 error.
    *   `io.rs`: `load_from_file` and `save_to_file` functions.
    *   `utils.rs`: Helper functions like `merge_and_convert_chunks`, `merge_overlapping_strings`.

3.  **Create New `mod.rs`**: The new `crates/ragit-utils/src/chunk/mod.rs` will contain `pub mod` declarations for all the new files and `pub use` statements to expose the necessary structs and functions.

## Phase 2: Fix Compilation Errors Systematically

During and after the refactoring, the following specific errors will be addressed:

1.  **Duplicate `render` function (E0592)**:
    *   **Problem**: Two methods named `render` exist for `Chunk`. One is `async` and returns `Result<String>`, the other is synchronous and returns `Result<RenderedChunk, Error>`.
    *   **Solution**: Rename the synchronous `render` to `render_chunk_simple` or a similar descriptive name to resolve the conflict.

2.  **`.await` on non-Future `Result` (E0277)**:
    *   **Problem**: `.await` is used on calls to the synchronous `render` method.
    *   **Solution**: Remove the `.await` from these calls. The refactoring will make it clear which `render` function is being called.

3.  **Type Mismatches (E0308)**:
    *   **`&String` vs `&Prompt`**: In `QueryResponse::new`, ensure a proper `Prompt` object is created instead of passing a raw `String`.
    *   **`&String` vs `&Path`**: Review calls to `ragit_fs` functions. Use `Path::new(&string)` or `PathBuf::from(string)` to convert before passing, or use the `pathbuf_to_str` utility correctly.
    *   **`&Model` vs `Model`**: Apply the compiler's suggestion to borrow the result: `&index.get_model_by_name(...)`.

4.  **Missing Fields and Methods (E0609, E0599)**:
    *   **`query.answer`**: Change to `query.response` as indicated by the compiler.
    *   **`Uid::new_chunk_from_chunk`**: This method is missing. A new UID will be generated for the chunk, likely based on a hash of its content, using existing `Uid` constructors.
    *   **`image.bytes` / `image.image_type`**: The fields on the `Image` struct are likely private or named differently. The definition of `index::file::Image` will be checked and the access corrected.
    *   **`gz.read_to_string`**: Add `use std::io::Read;` to the file where this is called.
    *   **`gz.finish()`**: Replace with the correct method from the `flate2` crate for finalizing a `GzEncoder`, which is likely `try_finish()` or `into_inner()`.
    *   **`WriteMode::Create`**: Find the correct variant in the `ragit_fs::WriteMode` enum (e.g., `CreateNew`, `Truncate`).

5.  **Incorrect Function Arguments (E0061)**:
    *   **`tfidf::save_to_file`**: Correct the function call to provide the three required arguments: `path`, `chunk`, and `root_dir`.

## Phase 3: Address Warnings

Once all errors are fixed, address the 17 warnings:

*   Remove all `unused_imports`.
*   Prefix all `unused_variables` with an underscore (e.g., `_index`).
*   Review the `unexpected_cfgs` for the `korean` feature and either add the feature to `Cargo.toml` or remove the conditional compilation flags.
