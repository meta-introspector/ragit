# SOP: Rust Refactoring - Managing Imports and Dependencies

## 1. Purpose
This Standard Operating Procedure (SOP) provides guidelines for managing imports and dependencies when refactoring Rust code, particularly when splitting large files into smaller, single-declaration files. Proper import management is crucial to avoid compilation errors and maintain code clarity.

## 2. Scope
This SOP applies to all Rust refactoring efforts within the `ragit` project that involve moving code between files or creating new modules.

## 3. Procedure

### 3.1. Understand the Module Hierarchy
Before splitting a file, thoroughly understand the existing module hierarchy (`mod.rs` files) and how the original file imports its dependencies and exposes its own items.

### 3.2. Extracting Code Blocks (Functions, Structs, Impls)

#### 3.2.1. Create New File
Create a new `.rs` file for the extracted code block (e.g., `my_function.rs`, `my_struct.rs`, `impl_my_trait_for_my_type.rs`).

#### 3.2.2. Copy Code
Copy the exact code block (function, struct, enum, `impl` block) into the new file.

#### 3.2.3. Identify and Add Necessary Imports
This is the most critical step. The new file will likely need `use` statements for:
*   **Crate-level items**: Types, traits, or functions defined at the root of the current crate (e.g., `use crate::{Error, Result, Layout};`).
*   **Sibling modules**: Items from other modules within the same parent directory (e.g., `use super::utils::{unary_map, binary_map};`).
*   **External crates**: Dependencies from `Cargo.toml` (e.g., `use half::{bf16, f16};`).

**Best Practice**: Start by copying all `use` statements from the original file to the new file. Then, systematically remove unused imports and resolve any "unresolved import" errors.

#### 3.2.4. Add Metadata Header
Add the standard metadata header to the top of the new file, as defined in `docs/sops/documenting_cpu_backend_operations_refactoring.md`.

### 3.3. Updating the Parent Module (`mod.rs`)

#### 3.3.1. Add `pub mod` Declaration
In the `mod.rs` file of the parent directory, add a `pub mod` declaration for the new file.
```rust
pub mod my_new_file;
```

#### 3.3.2. Add `pub use` Statements (if necessary)
If the items in the new file need to be re-exported through the parent module, add `pub use` statements.
```rust
pub use my_new_file::{MyStruct, MyFunction};
```

#### 3.3.3. Remove Original Code
Once the code is successfully extracted and the new module is declared, remove the original code block from the source file.

### 3.4. Iterative Compilation and Verification
*   **Compile Frequently**: After each extraction and `mod.rs` update, run `cargo build` or `cargo check` to catch compilation errors early.
*   **Address Errors Systematically**: Focus on resolving import-related errors first. The compiler messages often provide helpful suggestions for missing `use` statements.
*   **Test**: Run relevant unit tests to ensure the refactored code functions as expected.

## 4. Common Pitfalls and Troubleshooting
*   **"Unresolved import" errors**: The most common issue. Double-check the path in the `use` statement. Remember `super::` for sibling modules and `crate::` for root-level items.
*   **"Cannot find type/trait in this scope"**: Often caused by a missing `use` statement.
*   **Circular Dependencies**: Avoid situations where module A depends on B, and B depends on A. This can be tricky to resolve and may require re-evaluating the module design.
*   **`#[macro_export]` macros**: If a macro is moved, ensure it's still accessible from where it's used. This might require re-exporting it or adjusting its visibility.

## 5. Tools
*   `cargo build` / `cargo check`
*   `grep` (for finding declarations and usage)
*   `sed` (for automated extraction and modification)
*   `git status`, `git diff` (for tracking changes)

## 6. Expected Outcome
*   Clean, modular Rust code adhering to the "one declaration per file" principle.
*   Correctly managed imports and dependencies.
*   Reduced technical debt and improved maintainability.
