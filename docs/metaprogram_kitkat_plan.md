# Meta-Program: "Have a KitKat" (2025-07-29)

The "Have a KitKat" meta-program is a user-defined workflow for pausing the current line of work, defining a new strategic plan, documenting it, committing the current state, and conceptually "rebooting" the development cycle to focus on the new plan.

**Current Status:**
- Resolved compilation errors related to `memory_profiler` import, private `MemoryMonitor` fields, and incorrect function arguments.
- Removed `print_process_list` calls from `bootstrap_command.rs` as `MemoryMonitor` handles logging.
- Silenced unused variable warnings by prefixing with `_` where appropriate, as per user's request to keep the variables for future use.
- The project now builds successfully.

**New Critical Path:**
1.  **Document "KitKat"**: Update `docs/metaprogram_kitkat_plan.md` to reflect the current state and the next steps. (Currently executing this step)
2.  **Commit "KitKat"**: Commit the updated `metaprogram_kitkat_plan.md`.
3.  **Update Docs**: Review and update `docs/bootstrap.md` if necessary, based on the recent changes to the `bootstrap` command.
4.  **Run Bootstrap**: Execute the `ragit bootstrap` command with verbose output.
5.  **Examine Memory Report**: Analyze the memory usage reported by the `bootstrap` command.

**Generalization of Learning:**
- **"One Declaration Per File" Principle:** This principle, while increasing the number of files, significantly improves modularity, testability, and reusability. It forces a clear separation of concerns and makes code easier to navigate and understand.
- **Macro Peculiarities in Rust:** Rust's macros, especially `format!` and `writeln!`, have strict requirements for their first argument (must be a string literal). When using constants for formatting, `format_args!` is the correct approach to ensure compile-time string literal behavior.
- **Asynchronous Operations and Ownership:** When refactoring, pay close attention to `async` functions and how data is passed between them (e.g., `&mut System` vs. `&System`). Incorrect handling can lead to lifetime or ownership errors.
- **Systematic Debugging:** When encountering multiple errors, address them systematically, starting with the most fundamental ones (e.g., module imports, basic syntax) and progressively moving to more complex issues.
- **Importance of `std::io::Write`:** The `flush()` method requires the `std::io::Write` trait to be in scope. This is a common oversight when refactoring I/O operations.
- **Error Handling with `anyhow` and `thiserror`:** When using `anyhow` and `thiserror` together, ensure that custom error types correctly implement `Clone` (if needed) and that `From` implementations are not conflicting with `anyhow`'s blanket implementations. Explicitly converting errors to `Arc` before wrapping them in `ApiError` can resolve ownership issues.