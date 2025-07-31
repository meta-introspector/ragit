### Plan

**Part 1: Documentation Review and Update**

1.  **Analyze `docs/bootstrap/`:** I will start by thoroughly reviewing the contents of the `docs/bootstrap/` directory to ensure all information is up-to-date with the latest refactoring efforts.
2.  **Update `docs/bootstrap.md`:** I will then update the main `docs/bootstrap.md` file to:
    *   Reflect the renaming of `write_chunks_to_markdown` to `export_chunks`.
    *   Clarify the current, hardcoded behavior of the `disable_write_markdown` flag.
    *   Integrate the new debugging and memory profiling features, explaining the purpose of the `--verbose` flag and the `MemoryMonitor`.
    *   Reference the new `debug_report.md` and its role in the debugging process.

**Part 2: Code Improvement and Refactoring**

1.  **Investigate `export_chunks` in `bootstrap-new`:** I will investigate the `bootstrap-new` command's workflow to determine if and how it calls the `export_chunks` function. If it's not being called, I will add it to the workflow and the memory tracking system.
2.  **Enhanced Memory and Operation Tracking:**
    *   I will introduce a new `Loggable` trait with a `log_and_track` method. This will allow us to standardize how we log messages and track operation counts (e.g., chunks processed, files added).
    *   The `MemoryMonitor` will be updated to use this trait to calculate and store memory usage per operation, providing more granular insights into the bootstrap process's performance.
3.  **Replace `if verbose` with a Logging Macro:** To improve code clarity and reduce clutter, I will create a `log_verbose!` macro. This macro will wrap the logging and memory tracking logic, ensuring it only executes when the `--verbose` flag is enabled.
4.  **My Suggestions for Further Improvements:**
    *   **Configuration Struct:** I propose creating a `BootstrapConfig` struct to encapsulate the numerous boolean flags passed to the `bootstrap_index_self` function. This will simplify the function signature and make the code more readable and maintainable.
    *   **Monadic Error Handling:** I will review the existing error handling and propose a more robust, monadic approach. This will lead to more descriptive error messages and a more resilient bootstrap process.
    *   **Code Deduplication:** I will identify areas of code duplication between the `bootstrap` and `bootstrap-new` commands and refactor them into shared modules to improve code reuse and reduce maintenance overhead.
