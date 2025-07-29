# Metaprogram: "Have a KitKat" (2025-07-27)

The "Have a KitKat" meta-program is a user-defined workflow for pausing the current line of work, defining a new strategic plan, documenting it, committing the current state, and conceptually "rebooting" the development cycle to focus on the new plan.

## Current Status:
- Implemented a `--verbose` flag for debugging purposes.
- Traced a build failure to a `PromptMissing("summarize")` error.
- The `bootstrap_index_self` command now copies the `prompts` directory to the temporary directory.
- The root cause of the `PromptMissing` error was identified: the `Index` struct's `prompts` field was not being populated correctly because the prompts were copied *after* the index was initialized.
- Refactored `bootstrap_command.rs` into smaller, single-purpose functions, each in its own file, adhering to the "One Declaration Per File" principle.
- Addressed issues with `format!` and `writeln!` macros when using constants by switching to `format_args!`.
- Ensured `use std::io::Write;` is present in all files using `flush()`.
- Made `copy_prompts` an `async` function.
- **Refactored `init_worker`:** The `init_worker` function in `ragit-index-effects` has been split into `init_worker` (for channel setup and spawning the worker task) and `run_worker_task` (containing the main worker logic).
- **Resolved `ApiError` cloning issues:** `ApiError` now correctly derives `Clone` by wrapping non-`Clone`able inner error types in `Arc`. Error handling in `run_worker_task.rs` and `ragit-model-provider/src/lib.rs` has been adjusted to use `ApiError::from(e)` and `map_err` with `Arc::new(e)` where necessary, and to clone `ApiError` instances when sending them through MPSC channels.
- **Removed conflicting `From` implementation:** The custom `impl From<ApiError> for anyhow::Error` was removed from `ragit-types/src/api_error.rs` to resolve conflicts with `anyhow`'s built-in implementation.
- **Fixed `thiserror` prefix errors:** Added whitespace to error messages in `ApiError` to resolve `thiserror` prefix warnings.

## New Critical Path:
The next phase is to successfully run the `bootstrap` command without compilation errors or runtime panics. This involves:
1. Verifying all compilation errors are resolved.
2. Confirming the graceful exit behavior with `max_iterations`.
3. Ensuring the `PromptMissing` error is resolved by correctly populating the `Index`'s `prompts` field.

## Generalization of Learning:
- **"One Declaration Per File" Principle:** This principle, while increasing the number of files, significantly improves modularity, testability, and reusability. It forces a clear separation of concerns and makes code easier to navigate and understand.
- **Macro Peculiarities in Rust:** Rust's macros, especially `format!` and `writeln!`, have strict requirements for their first argument (must be a string literal). When using constants for formatting, `format_args!` is the correct approach to ensure compile-time string literal behavior.
- **Asynchronous Operations and Ownership:** When refactoring, pay close attention to `async` functions and how data is passed between them (e.g., `&mut System` vs. `&System`). Incorrect handling can lead to lifetime or ownership errors.
- **Systematic Debugging:** When encountering multiple errors, address them systematically, starting with the most fundamental ones (e.g., module imports, basic syntax) and progressively moving to more complex issues.
- **Importance of `std::io::Write`:** The `flush()` method requires the `std::io::Write` trait to be in scope. This is a common oversight when refactoring I/O operations.
- **Error Handling with `anyhow` and `thiserror`:** When using `anyhow` and `thiserror` together, ensure that custom error types correctly implement `Clone` (if needed) and that `From` implementations are not conflicting with `anyhow`'s blanket implementations. Explicitly converting errors to `Arc` before wrapping them in `ApiError` can resolve ownership issues.
