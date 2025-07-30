# Ragit Core Functionality Executable: `ragit-build-index-worker-single-file`

The `ragit-build-index-worker-single-file` executable is a self-contained tool designed to perform core `ragit` functionalities, including bootstrapping the index and querying it. This consolidation aims to provide a streamlined experience for testing and development, allowing for easy execution of fundamental operations within a single binary.

## 1. `bootstrap` Subcommand

The `ragit bootstrap` command is the original, more comprehensive self-improving tool. It builds an index and then uses that index to analyze and improve the `ragit` codebase itself.

### Usage

```bash
cargo run --package ragit-build-index-worker-single-file -- bootstrap [FLAGS]
```

### Flags

*   `--verbose`: Enables verbose output, which is useful for debugging.
*   `--timeout-seconds <SECONDS>`: Sets a timeout for the bootstrap operation.
*   `--max-iterations <NUMBER>`: Exits gracefully after a certain number of iterations. This is primarily for debugging.
*   `--max-memory-gb <NUMBER>`: Sets a maximum memory limit in gigabytes for the bootstrap operation.
*   `--max-files-to-process <NUMBER>`: Limits the number of files to process during the bootstrap.
*   `--max-chunk-size <NUMBER>`: Sets the maximum size of a chunk.
*   `--max-summary-len <NUMBER>`: Sets the maximum length of a summary.
*   `--min-summary-len <NUMBER>`: Sets the minimum length of a summary.
*   `--time-threshold-ms <NUMBER>`: Sets a time threshold in milliseconds for memory monitoring.
*   `--memory-threshold-bytes <NUMBER>`: Sets a memory threshold in bytes for memory monitoring.
*   `--disable-write-markdown`: Disables writing chunks to markdown.
*   `--disable-memory-config`: Disables memory configuration.
*   `--disable-prompt-copy`: Disables copying prompts.
*   `--disable-file-add`: Disables adding files.
*   `--disable-index-build`: Disables index building.
*   `--disable-self-improvement`: Disables the self-improvement phase.
*   `--disable-final-query`: Disables the final reflective query.
*   `--disable-cleanup`: Disables the cleanup of the temporary directory after the bootstrap process.

### Workflow

The `bootstrap` subcommand executes a series of operations, each managed by a dedicated function within the `bootstrap_commands` module. Memory usage is monitored and logged at key steps using `MemoryMonitor`.

1.  **Setup Environment (`setup_environment`):**
    *   Creates a temporary directory.
    *   Initializes a new `ragit` repository within the temporary directory.
    *   Initializes a new `Index` structure.
    *   A memory snapshot is captured and logged after index initialization.

2.  **Copy Prompts (`copy_prompts`):**
    *   Copies the `prompts` directory from the actual root to the temporary directory.
    *   Loads copied prompts into the in-memory `Index` structure.

3.  **Add Bootstrap Files (`add_bootstrap_files`):**
    *   Identifies and copies relevant `.rs` files from the `ragit-command-bootstrap` package to the temporary directory.
    *   Adds the copied files to the `Index`.

4.  **Build Index (`build_index`):**
    *   Builds the `Index` from the source code, processing the content of the added `.rs` files.
    *   Memory snapshots are captured and logged before and after the build process.

5.  **Export Chunks to Content-Addressable Store (`export_chunks`):**
    *   Writes each chunk to a content-addressable store (similar to `.git/objects`) based on its SHA-1 hash.
    *   Memory snapshots are captured and logged before and after this step.

6.  **Self-Improvement (`perform_self_improvement`):**
    *   Analyzes and improves code by reading its own source, generating a prompt, executing a query, and writing improved code to a file.
    *   Memory snapshots are captured and logged at various stages of this process.

7.  **Final Reflective Query (`perform_final_reflective_query`):**
    *   Executes a hardcoded query against the built index, and prints the response.
    *   Memory snapshots are captured and logged before and after this step.

---

## 2. `query` Subcommand

The `query` subcommand allows you to interact with a `ragit` index to retrieve information. It supports both single-turn and multi-turn queries, and can output results in JSON format.

### Usage

```bash
cargo run --package ragit-build-index-worker-single-file -- query <QUERY_STRING> [FLAGS]
```

### Arguments

*   `<QUERY_STRING>`: The query string to search for in the index.

### Flags

*   `--no-pdl`: Disables PDL (Prompt Description Language) processing for the query.
*   `--multi-turn`: Enables multi-turn conversational mode. In this mode, you can ask follow-up questions until you type `.exit`.
*   `--json`: Outputs the query result in JSON format.
*   `--kb-path <PATH>`: Specifies the path to the knowledge base (index) to query. If not provided, it defaults to the current project root.

### Workflow

The `query` subcommand performs the following steps:

1.  **Load Index:** Loads the `ragit` index from the specified knowledge base path (or the current project root if not provided).
2.  **Execute Query:** Executes the provided query against the loaded index.
3.  **Process Response:** Processes the response from the query, handling multi-turn conversations and JSON output as requested.

---

## Debugging and Memory Management

If you encounter issues with the `ragit-build-index-worker-single-file` executable, especially Out-Of-Memory (OOM) errors, here's what we've learned and how to approach debugging:

### Flags for Debugging

*   `--verbose`: Enables verbose output, which is useful for debugging. The build dashboard now includes more detailed memory usage metrics, such as the delta since the beginning of the process and the average memory usage per loop.
*   `--max-iterations <NUMBER>`: Exits gracefully after a certain number of iterations. This is primarily for debugging and will cause the process to exit gracefully once the specified number of iterations is reached.
*   `--max-memory-gb <NUMBER>`: Sets a maximum memory limit in gigabytes for the operation. **Important Note:** This flag is designed for the `ragit` application to *gracefully exit* if its memory usage exceeds the specified limit. It does **not** prevent the operating system from terminating the process due to an Out-Of-Memory (OOM) condition if the system's overall memory is exhausted. If you're experiencing OOM kills (exit code 137), it means the process is consuming memory faster than the application can detect and react, or the system's memory is simply too constrained.

### Understanding OOM Kills (Exit Code 137)

An exit code of 137 typically indicates that the process was terminated by the operating system due to an Out-Of-Memory (OOM) condition. This can happen even if the application is attempting to shut down gracefully, as the system may run out of memory before the application completes its cleanup.

To mitigate persistent OOM issues:
1.  **Increase available RAM:** Ensure your system has sufficient free RAM.
2.  **Run on a more powerful system:** Consider executing the command on a machine with more memory.
3.  **Reduce the scope:** If possible, try to process fewer files or smaller files during the operation (e.g., by adjusting the `CHUNK_PROCESSING_LIMIT` if applicable, or by targeting specific subsets of files).

### Full Backtrace

To get a detailed backtrace in case of a crash, set the `RUST_BACKTRACE` environment variable to `full`:

```bash
export RUST_BACKTRACE=full
cargo run --package ragit-build-index-worker-single-file -- bootstrap # or query
```

This will provide a more comprehensive stack trace, which can be invaluable for debugging panics and other runtime errors.

### Memory Profiling with jemalloc

To generate a memory profile using `jemalloc`, you need to set the `MALLOC_CONF` environment variable before running the command. This will create a heap profile file that you can analyze.

1.  **Set `MALLOC_CONF`**:
    Set the `MALLOC_CONF` environment variable to specify the profiling options. For example, to dump a profile on exit:

    ```bash
    export MALLOC_CONF="prof:true,prof_dump:true,prof_prefix:jeprof"
    ```

    *   `prof:true`: Enables profiling.
    *   `prof_dump:true`: Dumps a profile on exit.
    *   `prof_prefix:jeprof`: Sets the prefix for the profile files (e.g., `jeprof.<pid>.<seq>.heap`).

2.  **Run the command**:

    ```bash
    cargo run --package ragit-build-index-worker-single-file -- bootstrap # or query
    ```

    When the process exits (e.g., due to `max_iterations` being reached or normal completion), a `.heap` file will be generated in the current working directory.

3.  **Analyze the profile**:
    You can use `jeprof` (a Perl script that comes with `jemalloc`) to analyze the generated `.heap` file. You'll typically need `graphviz` installed to generate visual graphs.

    ```bash
    jeprof --svg target/debug/ragit-build-index-worker-single-file jeprof.<pid>.<seq>.heap > profile.svg
    ```

    *   `target/debug/ragit-build-index-worker-single-file`: Path to your executable.
    *   `jeprof.<pid>.<seq>.heap`: The generated heap profile file.
    *   `profile.svg`: The output SVG file with the call graph.

    This will generate an SVG file showing the memory allocation call graph, helping you identify memory leaks or high memory usage areas.

### Internal Module Structure and Imports

During development, the `memory_utils` module was moved from `ragit-command-bootstrap` to `ragit-utils` to resolve cyclic dependencies and improve modularity. To maintain compatibility with existing `use` statements within `ragit-command-bootstrap`, a shim was introduced in `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/mod.rs` (specifically, `pub use crate::memory_utils;`). This allows other modules within `ragit-command-bootstrap` to continue using `crate::bootstrap_commands::memory_utils` without needing to update every single import path. This approach prioritizes functionality and avoids extensive refactoring when a simpler solution is available.