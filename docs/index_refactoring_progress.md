# Index Refactoring Progress and Goals

## Current State

The index refactoring aims to establish a clear hierarchy and separation of concerns among related crates. Here's the current status:

*   **`ragit-types`**: This crate is intended to be a pure data-only crate, containing fundamental data structures (structs, enums) and their associated `impl` blocks that do not involve any side effects, I/O, or error handling.
    *   **Status**: `ragit-types/src/api_error.rs` has been cleared, and `test_model.rs` has been removed to enforce purity. Dependencies on `ragit-fs` and `sha3` have been removed from its `Cargo.toml`.
*   **`ragit-error`**: Defines canonical `ApiError` and other error types.
    *   **Status**: `MPSCError` and `Internal` variants have been added to `ApiError`. Dependency on `ragit-utils` has been removed to break a cyclic dependency.
*   **`ragit-fs`**: Provides file system utilities.
*   **`ragit-index-types`**: Contains the core `Index` struct and related data structures. It is designed to be pure and depends only on `ragit-types` and other pure data crates.
    *   **Status**: `build_worker`, `build_chunks`, `Request`, `Response`, `Channel`, `init_workers`, `init_worker`, `render_build_dashboard`, and `build` functions/structs have been moved out of `ragit-index-types` to `ragit-index-effects`.
*   **`ragit-index-io`**: Responsible for serialization and deserialization of index data.
*   **`ragit-index-core`**: Contains the core logic for index state manipulation (pure operations). It will depend on `ragit-index-types` and call `ragit-index-io` and `ragit-index-effects` for monadic operations.
    *   **Status**: `load_index_from_path` has been moved out.
*   **`ragit-index-effects` (New Crate)**: This crate is specifically created to house all monadic operations, including I/O, error handling, and external API calls. It depends on `ragit-index-types`, `ragit-error`, `ragit-fs`, `sha3`, `tokio`, and `ragit-api`.
    *   **Status**: `build_worker`, `build_chunks`, `Request`, `Response`, `Channel`, `init_workers`, `init_worker`, `render_build_dashboard`, and `build` functions/structs have been successfully moved here.
*   **`ragit-utils`**: Provides general utility functions.
    *   **Status**: Updated to import `ApiError` directly from `ragit-error`.
*   **`ragit-command-bootstrap`**: Orchestrates the bootstrap process.
    *   **Status**: Updated to use `StorageManager` and `build` from `ragit-index-effects`.
*   **Excluded Crates**: `crates/server` and `ragit-agent-action` are currently excluded from the workspace to simplify the build process and focus on the core index refactoring.
*   **Dependencies**: `tempfile` version has been updated.

## Lessons Learned

*   **Strict Crate Hierarchy**: Maintaining a strict hierarchy where pure data crates (`ragit-types`, `ragit-index-types`) do not depend on crates with side effects (`ragit-error`, `ragit-fs`, `ragit-index-effects`) is crucial to prevent cyclic dependencies and ensure modularity.
*   **Monadic Separation**: Operations involving `Result`, `Error`, I/O, or external calls (monadic operations) must be isolated in dedicated "effects" crates. This keeps the core logic clean and testable.
*   **Re-exports vs. Direct Imports**: While re-exports can simplify `use` statements, they must be used carefully. In cases where a crate needs to import an item that would create a cyclic dependency if re-exported, direct imports from the canonical source are necessary.
*   **Incremental Refactoring**: Breaking down large refactoring tasks into smaller, manageable steps, and verifying each step, is essential for success.

## Goals

*   **Resolve Remaining Compilation Errors**: Systematically address all current compilation errors, focusing on correct imports and function calls based on the new crate structure.
*   **Ensure Purity of `ragit-index-types`**: Verify that `ragit-index-types` strictly adheres to the pure data-only principle.
*   **Correctly Implement `ragit-index-effects`**: Ensure all moved functions and their dependencies are correctly implemented and integrated within `ragit-index-effects`.
*   **Update Callers**: Modify all code that previously called methods on `Index` (which are now in `ragit-index-effects`) to use the new functions.
*   **Successful Bootstrap**: Achieve a successful run of the `bootstrap_index_self` test, indicating the core index functionality is working as expected.
*   **Refactor `Index` Methods**: Continue refactoring the `Index` struct by moving its methods into appropriate modules within `ragit-index-effects` or other specialized crates, following the "one declaration per file" principle.
*   **Implement `StorageManager`**: Fully implement the `StorageManager` in `ragit-index-io` to centralize file system operations.
*   **Implement `ConfigManager`**: Implement the `ConfigManager` to handle all configuration-related aspects.
*   **Implement `SearchEngine`**: Implement the `SearchEngine` for query operations.
*   **Implement `BuildOrchestrator`**: Implement the `BuildOrchestrator` to manage the complex build process.
*   **Implement `RecoveryManager`**: Implement the `RecoveryManager` for error recovery and validation.
