## Plan for Memory Profiler Refactoring

**Goal:** Improve the memory profiling output by displaying deltas for Total, Used, and Process RSS memory, and refactor the code for better modularity and readability.

**Current State:**
*   `memory_profiler.rs` contains `MemorySnapshot` struct, `capture_memory_snapshot`, `format_bytes`, `format_signed_bytes`, and `print_memory_table`.
*   `MemorySnapshot` and `capture_memory_snapshot` have been updated to store raw byte values and calculate deltas.
*   `format_bytes` and `format_signed_bytes` have been moved to separate files.
*   `column.rs` and `table.rs` have been created for generic table rendering.

**Proposed Plan:**

1.  **Refactor `memory_profiler.rs`:**
    *   Remove `MemorySnapshot` struct (already moved).
    *   Remove `format_bytes` and `format_signed_bytes` functions (already moved).
    *   Move `print_memory_table` function to its own file (`print_memory_table.rs`).

2.  **Implement `print_memory_table.rs`:**
    *   Use the new `Table` and `Column` structs to render the memory usage summary.
    *   Format the `total_memory`, `used_memory`, `process_rss`, and their respective deltas using `format_bytes` and `format_signed_bytes`.

3.  **Update `bootstrap_process.rs`:**
    *   Adjust the argument passed to `capture_memory_snapshot` to match the new `last_snapshot_data` type (which is `Option<(u64, u64, u64)>`).
    *   Update imports to reflect the new file structure.

4.  **Create `mod.rs` in `memory_profiler/`:**
    *   Re-export all public items from the individual files (`memory_snapshot.rs`, `format_bytes.rs`, `format_signed_bytes.rs`, `capture_memory_snapshot.rs`, `print_memory_table.rs`, `column.rs`, `table.rs`).

5.  **Clean up `memory_profiler.rs`:** Delete the original `memory_profiler.rs` file once all its contents are moved.

6.  **Verify:**
    *   Run `cargo check` to ensure no compilation errors.
    *   Run `cargo run --package ragit-commands -- bootstrap-new` and verify the memory report output is correct and includes the deltas as desired.
