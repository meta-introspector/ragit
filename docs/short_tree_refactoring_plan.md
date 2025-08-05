# Short Tree Refactoring Plan: De-duplication and Reorganization

**Objective:** To streamline the `ragit` project's file system by eliminating redundant files, consolidating logically related components, and establishing a clear, atomic module structure.

**Guiding Principles:**
*   **One Declaration Per File:** Each `.rs` file should ideally contain a single primary public declaration (struct, enum, function, or module).
*   **Atomic Modules:** Modules should be self-contained and have clearly defined responsibilities.
*   **Layered Architecture Adherence:** Files should reside within their appropriate `layerX_` crates.
*   **Vendorization Clarity:** Clearly distinguish between internal code and vendored external dependencies.

**High-Level Steps:**

1.  **Identify Duplicates & Redundancies:**
    *   Leverage the `ragit-code-analyzer` (once implemented) to identify:
        *   Exact duplicate files (based on content hash).
        *   Functionally duplicate code segments (even if in different files).
        *   Files that contain multiple top-level public declarations.
    *   Manually review the `docs/file_system_manifest.md` for obvious structural redundancies or misplacements.

2.  **Consolidate & Relocate:**
    *   **Move `legacy_and_refactoring` content:** Systematically migrate relevant code from `crates/legacy_and_refactoring/` into their appropriate `layerX_` crates, adhering to the "one declaration per file" principle. Deprecate or remove the `legacy_and_refactoring` crate once its contents are fully migrated.
    *   **Consolidate `src/` root:** Move top-level files in `src/` (e.g., `src/constant.rs`, `src/error_reporting.rs`) into more specific modules or crates within the `layerX_` structure.
    *   **Standardize `mod.rs`:** Ensure all `mod.rs` files strictly adhere to the convention of only declaring and re-exporting submodules, without containing direct declarations.

3.  **Enforce Atomic Modules:**
    *   For files identified as containing multiple declarations, split them into separate files, each containing one primary declaration.
    *   Update `mod.rs` files and `use` statements accordingly.

4.  **Update `file_system_manifest.md`:**
    *   Continuously update the `docs/file_system_manifest.md` to reflect the new file organization. This will serve as a living documentation of the refactoring progress.

5.  **Automate with Tools:**
    *   Develop small, targeted Rust programs (using `ragit-code-analyzer` and other libraries) to automate the file movement, renaming, and import path updates. This reinforces our "write programs that edit files" philosophy.
