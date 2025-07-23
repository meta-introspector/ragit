# Ragit Refactoring Log

This log documents the refactoring steps taken to resolve dependency cycles and establish a cleaner architecture for the `ragit` project.

## Key Architectural Changes:

1.  **Introduction of `ragit-core` crate:** A new crate (`crates/ragit-core`) was introduced to house fundamental traits, starting with the `Matcher` trait. This ensures a clean separation of concerns and prevents dependency cycles.
2.  **`ragit-types` as a pure types crate:** The `ragit-types` crate was refactored to contain only data structures and their basic implementations, with no dependencies on other local crates. This was achieved by:
    *   Moving `Ignore`, `Pattern`, and `PatternUnit` (and their related logic) out of `ragit-types` and into `ragit-ignore`.
    *   Removing `ragit-utils` as a dependency from `ragit-types`.
3.  **Refactoring `ragit-ignore`:** The `ragit-ignore` crate now contains the `Ignore` and `Pattern` structs and their implementations. It depends on `ragit-core` for the `Matcher` trait and `ragit-fs` for file system utilities.
4.  **Refactoring `ragit-core-utils`:** This crate now focuses on core utility functions, particularly path manipulation. Functions like `is_dir`, `is_symlink`, and `read_dir` were moved back to `ragit-fs` to prevent circular dependencies between `ragit-fs` and `ragit-core-utils`.
5.  **Dependency Resolution:**
    *   The circular dependency between `ragit-api`, `ragit-utils`, `ragit-types`, `ragit-uid`, and `ragit-pdl` was broken by moving `AuditRecordAt` from `ragit-api` to `ragit-types` and updating imports accordingly.
    *   The circular dependency between `ragit-types`, `ragit-utils`, and `ragit-ignore` was resolved by moving `Ignore`-related code to `ragit-ignore` and making `ragit-types` a pure types crate.
    *   The circular dependency between `ragit-fs` and `ragit-core-utils` was resolved by ensuring `ragit-fs` is the foundational file system utility and `ragit-core-utils` imports from it.

## Specific Fixes Implemented:

*   Added `regex` dependency to `ragit-types`.
*   Corrected `regex` usage in `PatternUnit::from_str` in `ragit-ignore`.
*   Ensured `AuditRecordAt` is public in `ragit-types`.
*   Updated `ragit-schema` dependencies and imports to reflect the new crate structure.
*   Corrected `serde_json::from_slice` usage in `ragit-uid` to expect `&[u8]`.
*   Fixed `ok_or_else` to `or_else` in `ragit-core-utils/src/path.rs`.
*   Cleaned up unused imports across various crates.

This refactoring aims to improve modularity, reduce coupling, and make the codebase easier to maintain and extend.
