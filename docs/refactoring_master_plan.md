# Ragit Refactoring Master Plan

**Date:** July 23, 2025

This document provides a comprehensive overview of the entire `ragit` refactoring effort. It consolidates all previous logs and outlines the final plan to achieve a stable, modular, and maintainable codebase.

---

## 1. Historical Context & Key Architectural Changes

The primary motivation for this refactoring was to resolve complex circular dependencies and improve the overall modularity of the `ragit` codebase. This led to a major architectural shift towards a micro-crate design.

*   **`ragit-core`:** Introduced to house fundamental traits (`Matcher`) and break dependency cycles.
*   **`ragit-types`:** Created as a "pure" crate containing only data structures (`Uid`, `FileSchema`, `ChunkSchema`, etc.) with no local dependencies.
*   **`ragit-ignore`:** Became the dedicated home for `.gitignore` parsing logic (`Ignore`, `Pattern`).
*   **`ragit-commands`:** All CLI command implementations were moved here from the main `ragit` binary.
*   **`ragit-config`:** A new crate dedicated to handling all configuration-related structures and logic (`BuildConfig`, `ApiConfig`, etc.).
*   **`ragit-index`:** The new central hub for the core `Index` struct and its associated methods, now including `agent` and `chunk` related logic.
*   **`ragit-muse`:** A small crate to house the logic for applying "muse" templates.
*   **`ragit-readers`:** A new crate to consolidate all file-reading logic.
*   **`ragit-query`:** A new crate dedicated to handling query logic, including `uid_query`.
*   **`ragit-utils`:** This crate has been significantly slimmed down, with most of its previous functionality relocated to the new, more specialized crates.

---

## 2. Current Status: Refactoring in Progress, Resolving Dependencies

The major refactoring effort to decompose the `index` module into specialized crates is ongoing. We have moved `agent`, `chunk`, and `query_helpers` modules to their appropriate new homes. The immediate focus is on resolving remaining compilation errors, particularly cyclic dependencies and unresolved imports.

---

## 3. The Plan to Win: A Three-Phase Approach

This plan will first stabilize the project by fixing any remaining errors, then finalize the new `ragit-readers` crate, and finally, clean up the documentation and codebase.

### **Phase 1: Project-Wide Compilation and Test (Immediate Priority)**

The goal is to ensure the entire project compiles and all tests pass after the extensive refactoring.

1.  **Full Workspace Build:** Run `cargo build --workspace` to identify and fix any remaining compilation errors across all crates, specifically addressing cyclic dependencies and unresolved imports in `ragit-types` and other affected crates.
2.  **Full Workspace Test:** Run `cargo test --workspace` to identify and fix any test failures. This is critical to ensure that the refactoring has not introduced any regressions.

### **Phase 2: Finalize the `ragit-readers` Crate**

The `ragit-readers` crate has been created, but it needs to be fully integrated.

1.  **Integrate `ragit-readers`:** Ensure that all file-reading logic from other crates has been moved to `ragit-readers`.
2.  **Update Dependencies:** Update the `Cargo.toml` files of all crates that read files to use `ragit-readers`.
3.  **Add to Workspace:** Make sure `ragit-readers` is properly included in the main `Cargo.toml` workspace.

### **Phase 3: Documentation and Cleanup**

Once the project is stable and all tests pass, the final step is to update all documentation and remove any dead code.

1.  **Update Documentation:** Review and update all READMEs, architecture diagrams, and other documentation to reflect the new crate structure.
2.  **Remove Dead Code:** Identify and remove any obsolete code, modules, or files that are no longer needed after the refactoring.
3.  **Final Review:** Perform a final review of the entire codebase to ensure consistency and clarity.