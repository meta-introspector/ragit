# Refactoring Lessons Learned with Gemini

This document summarizes key refactoring principles and challenges encountered during a recent project, offering insights for other users working with Gemini.

## Core Refactoring Principles Adopted

Our refactoring efforts were guided by several strict principles aimed at improving modularity, maintainability, and code clarity:

1.  **"One Declaration Per File"**: Every struct, enum, function, or constant is placed in its own dedicated file. This enhances code discoverability, reduces merge conflicts, and promotes reusability.
    *   **Benefits**: Improved modularity, easier navigation, reduced cognitive load when reading files, better reusability of individual components.
    *   **Challenges**: Initial overhead of creating many small files, managing a larger file tree.

2.  **"Each Library Only Re-exports and Composes Submodules"**: The `lib.rs` file in each crate is strictly reserved for module declarations (`pub mod my_module;`) and re-exports (`pub use my_module::MyStruct;`). No direct function or struct definitions are allowed in `lib.rs`.
    *   **Benefits**: Enforces a clean, hierarchical architecture, makes dependencies explicit, simplifies `lib.rs` to act as a manifest for the crate's public API.
    *   **Challenges**: Requires careful planning of module structure and public interfaces.

3.  **"Always Use Prelude"**: Each crate defines a `prelude.rs` module that re-exports commonly used types and functions from within the crate and its dependencies. Other modules within the crate then import from this `prelude`.
    *   **Benefits**: Simplifies `use` statements in individual files, reduces boilerplate, promotes consistency in imports across the codebase.
    *   **Challenges**: Requires careful management of the `prelude` to avoid name clashes or unnecessary exports.

## Iterative Error Fixing and Refactoring Strategy

Our approach to fixing compilation errors was iterative and cautious:

1.  **"Fix Broken Code, Don't Break Working Code"**: Changes were strictly limited to areas identified by compilation errors. Functioning code was left untouched.

2.  **"Minimal Edits, Use Refactors"**: Instead of quick, localized fixes, we prioritized refactoring the problematic code to align with the architectural principles. Shims were used only as a last resort to enable compilation, with the understanding that they would be replaced by proper refactors later.

3.  **Targeted Module Extraction**: When a file accumulated too many errors or became a central point of complexity, it was extracted into its own new crate. This allowed for isolated development and testing of that component.
    *   **Process**: Create new crate directory, define `Cargo.toml` (with dependencies), move the problematic file(s), update the original crate's `lib.rs` (remove old module, add new crate dependency), and adjust imports in the moved files.

## Common Challenges and Solutions

1.  **`PathBuf` vs. `&str` Type Mismatches**: A frequent source of errors in Rust projects. Functions often expect `&Path` or `&str`, while variables might be `PathBuf` or `String`.
    *   **Solution**: Consistent use of `.to_string_lossy().into_owned()` or `.to_str().unwrap()` for conversions, and ensuring function signatures match the expected type.

2.  **Error Handling Interoperability (`From` Trait)**: When multiple `ApiError` enums existed across crates, the `?` operator failed due to missing `From` trait implementations.
    *   **Solution**: Implementing `impl From<OtherCrateError> for MyCrateError` to allow seamless conversion and propagation of errors across crate boundaries.

3.  **Managing `super::Index` in Extracted Modules**: When methods from an `impl Index` block were moved to a new crate, they became standalone functions. These functions then needed to accept `&Index` as their first argument.
    *   **Solution**: Adjusting function signatures and updating calls to pass the `Index` instance explicitly.

By adhering to these principles and learning from these challenges, we were able to systematically refactor the codebase towards a more modular, maintainable, and robust architecture.