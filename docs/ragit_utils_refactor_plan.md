# Refactoring Plan for `ragit-utils` Crate

This document outlines the refactoring efforts for the `ragit-utils` crate, focusing on modularity and resolving compilation issues.

## Phase 1: Refactor `chunk/mod.rs` (Completed)

The `crates/ragit-utils/src/chunk/mod.rs` file has been successfully refactored into a new `chunk` directory (`crates/ragit-utils/src/chunk/`). Each struct, enum, and `impl` block has been moved to its own file, improving code organization and readability.

Key changes include:

*   **Directory Structure**: `crates/ragit-utils/src/chunk/` was created, and the original `mod.rs` was replaced with a new one that declares and exports the new submodules.
*   **Declaration Splitting**: `chunk_struct.rs`, `chunk_extra_info.rs`, `rendered_chunk.rs`, `impl_chunk.rs`, `io.rs`, and `utils.rs` were created to house their respective declarations and implementations.
*   **`render` Method Resolution**: The conflicting `render` methods were addressed by renaming the synchronous version to `render_chunk_simple` to resolve the E0592 error.

## Phase 2: Fix Compilation Errors Systematically (Ongoing)

Many compilation errors have been addressed through the modularization and subsequent fixes. The following categories of errors are being systematically resolved:

1.  **`PathBuf` and `Path` Usage**: Inconsistent usage of `PathBuf` and `&Path` has been largely resolved by ensuring correct conversions and borrowing. `Path::new()` calls have been corrected to accept `&str` arguments where appropriate.
2.  **Unresolved Imports**: Missing imports for `std::collections::HashSet` and other modules have been added.
3.  **Duplicate Definitions**: Duplicate function implementations, particularly within `index_struct.rs`, are being removed or consolidated to ensure a single source of truth for each function.
4.  **Type Mismatches**: Mismatches in function arguments (e.g., `&PathBuf` vs. `&str`) are being corrected by ensuring the correct type is passed.
5.  **Missing Fields and Methods**: Definitions of structs like `AddResult` and `RemoveResult` have been updated to include necessary fields (`added_files`, `added_chunks`, `removed_files`, `removed_chunks`).
6.  **Mutability Issues**: Instances where mutable borrowing was required but not declared (e.g., `index` in `pull.rs`) have been fixed by adding `mut`.
7.  **Missing Associated Items**: `PushResult` has been updated to include `AlreadyUpToDate` as an enum variant.

## Phase 3: Address Warnings (Pending)

Once all compilation errors are resolved, the remaining warnings will be addressed. This includes:

*   Removing all `unused_imports`.
*   Prefixing all `unused_variables` with an underscore (e.g., `_index`).
*   Reviewing `unexpected_cfgs` for features like `korean` and either adding the feature to `Cargo.toml` or removing the conditional compilation flags.