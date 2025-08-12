# KitKat Break: Candle CPU Backend Refactoring - Revised Plan

## Date: 2025-08-07

## Current Status:
The refactoring of `candle-core/src/cpu_backend/ops/` has encountered significant challenges. While the initial goal was to break down `cpu_storage.rs` into smaller, single-declaration files, the approach of extracting individual functions (`fn`) directly led to a cascade of compilation errors.

**Key Issues Encountered:**
*   **Incorrect `self` and `Self` usage**: When individual functions were extracted outside of their `impl` blocks, they lost the implicit `self` and `Self` context, leading to numerous "self parameter is only allowed in associated functions" and "cannot find type `Self` in this scope" errors.
*   **Unresolved module/crate errors**: The `prelude.rs` was created, but the way `pub mod` and `pub use` statements were managed for the extracted functions caused "file not found for module" errors, indicating a misunderstanding of Rust's module system for this granular level of refactoring.
*   **Lingering `ug` crate issue**: An unresolved module `ug` in `candle-core/src/error.rs` persists, suggesting a missing dependency or an unused feature.
*   **Repeated `unexpected closing delimiter`**: This error indicates a fundamental structural issue in one of the `impl` files, likely `impl_backend_storage_for_cpu_storage.rs`, which needs careful manual inspection.

## Lessons Learned:
*   **`impl` blocks are atomic units**: Functions within `impl` blocks rely heavily on the `self` and `Self` context provided by the `impl` block itself. Extracting individual functions directly breaks this context. The correct approach for "one declaration per file" for `impl` methods is to extract the *entire* `impl` block into its own file, and then potentially further refactor *within* that `impl` block's file if methods are still too large.
*   **Module system precision**: When creating new modules and using `pub mod` and `pub use`, extreme precision is required in pathing and visibility. `super::` and `crate::` need to be used correctly.
*   **Iterative verification**: While `cargo build` was run frequently, the nature of the errors (many related to `self`/`Self` context) suggests that the fundamental approach to extraction was flawed from the start. More granular verification steps or a different extraction strategy is needed.

## Revised Strategic Plan:

1.  **Revert `candle-core/src/cpu_backend/ops/` to a clean state**: This is crucial to eliminate the current cascade of errors and start fresh with a correct approach. This will involve:
    *   Deleting all `_fn.rs` and `impl_*.rs` files (except the original `cpu_storage.rs`, `mod.rs`, and `utils.rs`).
    *   Restoring `cpu_storage.rs` to its state before any `impl` block extractions.
    *   Restoring `mod.rs` to its state before any `pub mod` additions for the extracted `impl` blocks.
2.  **Correctly extract `impl` blocks**: 
    *   Extract the entire `impl CpuStorage { ... }` block into `impl_cpu_storage.rs`.
    *   Extract the entire `impl BackendStorage for CpuStorage { ... }` block into `impl_backend_storage_for_cpu_storage.rs`.
    *   Extract the entire `impl BackendDevice for CpuDevice { ... }` block into `impl_backend_device_for_cpu_device.rs`.
    *   Ensure each extracted `impl` file is a valid, self-contained Rust file that compiles independently (if possible, or with minimal external dependencies).
3.  **Refine `prelude.rs`**: 
    *   The `prelude.rs` should primarily re-export common types and traits from `crate::` and `half::`, and potentially `rayon::`.
    *   It should *not* re-export individual functions that are part of `impl` blocks, as those functions are accessed via the `impl` type (e.g., `CpuStorage::as_slice`).
4.  **Update `cpu_backend/ops/mod.rs`**: 
    *   This `mod.rs` should only contain `pub mod` declarations for the extracted `impl` files (e.g., `pub mod impl_cpu_storage;`).
    *   It should also contain `pub use` statements for the `prelude` module.
5.  **Address `ug` crate**: Investigate the `ug` crate dependency. If it's truly needed, add it to the appropriate `Cargo.toml`. If it's a remnant or unused, remove the reference.
6.  **Iterative Verification**: After each major step (extraction, `mod.rs` update, `prelude.rs` update), run `cargo build` and `cargo run --package huggingface-candle-datasets-test` to verify compilation and functionality.

## KitKat Break Initiated.
