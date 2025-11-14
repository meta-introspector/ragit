## Plan for Monadic Memory Reporting Refactoring

**Goal:** Centralize memory profiling state and behavior within a `MemoryMonitor` struct to simplify call sites and improve modularity.

**Current State:**
Memory profiling logic is spread across several functions (`capture_memory_snapshot`, `print_memory_table`) and uses multiple mutable variables (`sys`, `last_snapshot_data`, `memory_snapshots`) passed around.

**Proposed Plan:**

1.  **Create `MemoryMonitor` Struct (`crates/layer7_application/ragit-build-index-worker-single-file/src/memory_profiler/memory_monitor.rs`):**
    *   Encapsulate `sysinfo::System`, `last_snapshot_data` (Option<(u64, u64, u64)>), and `memory_snapshots` (Vec<MemorySnapshot>) within this struct.

2.  **Implement Methods on `MemoryMonitor`:**
    *   `new()`: Initializes `MemoryMonitor` with `System::new_all()`, `last_snapshot_data` set to `None`, and an empty `snapshots` vector.
    *   `capture_and_log_snapshot(&mut self, step_name: &str)`: This method will replace the standalone `capture_memory_snapshot` function. It will:
        *   Call the existing `capture_memory_snapshot` function (which will be moved to `memory_monitor.rs` or kept as a private helper within the module).
        *   Update the internal `self.last_snapshot_data`.
        *   Add a new `MemorySnapshot` to `self.snapshots`.
    *   `print_final_report(&self)`: This method will replace the standalone `print_memory_table` function. It will:
        *   Call the existing `print_memory_table` function, passing `self.snapshots.clone()`.

3.  **Update `crates/layer7_application/ragit-build-index-worker-single-file/src/memory_profiler/mod.rs`:**
    *   Add `pub mod memory_monitor;` to expose the new module.

4.  **Update Call Sites in `crates/layer7_application/ragit-build-index-worker-single-file/src/bootstrap_process.rs` (and potentially other files):**
    *   Remove individual `sys`, `last_snapshot_data`, and `memory_snapshots` variables.
    *   Instantiate `let mut memory_monitor = MemoryMonitor::new();`.
    *   Replace all calls to `capture_memory_snapshot(...)` with `memory_monitor.capture_and_log_snapshot(...)`.
    *   Replace the final call to `print_memory_table(...)` with `memory_monitor.print_final_report()`.
    *   Adjust function signatures of `setup_environment`, `copy_prompts`, `add_bootstrap_files`, and `build_index` to accept `&mut MemoryMonitor` instead of `&mut System`, `&mut Option<(u64, u64, u64)>`, and `&mut Vec<MemorySnapshot>`.

5.  **Refactor `ragit_utils::memory_utils.rs`:**
    *   Remove `print_memory_usage` and `check_memory_limit` from `ragit_utils::memory_utils.rs` as their functionality will be absorbed or managed by `MemoryMonitor`.
    *   Update all call sites of `print_memory_usage` and `check_memory_limit` in `ragit-command-bootstrap` to use the `MemoryMonitor` methods or remove them if no longer needed.

**Benefits:**
*   **Encapsulation:** All memory profiling state and logic are contained within a single struct.
*   **Simplified API:** Call sites become cleaner and easier to read.
*   **Maintainability:** Changes to memory profiling logic are localized to the `MemoryMonitor` struct.
*   **Testability:** The `MemoryMonitor` can be easily instantiated and tested in isolation.

This refactoring will be implemented in a step-by-step manner, with `cargo check` and `cargo run` verification after each significant change.