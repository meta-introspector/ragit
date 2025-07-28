# Bootstrap Command

The `bootstrap` command is a powerful, self-improving tool for developers working on `ragit` itself. It automates the process of creating a new knowledge base from the `ragit` source code, building an index, and then using that index to improve the `ragit` codebase.

## Usage

```bash
cargo run --package ragit-commands -- bootstrap
```

### Flags

*   `--workers <COUNT>`: Specifies the number of workers to use for building the index. Currently defaults to 1 due to ongoing refactoring.
*   `--verbose`: Enables verbose output, which is useful for debugging.
*   `--timeout <SECONDS>`: Sets a timeout for the bootstrap operation.
*   `--max-iterations <NUMBER>`:  Exits gracefully after a certain number of iterations. This is primarily for debugging.
*   `--max-memory-gb <NUMBER>`: Sets a maximum memory limit in gigabytes for the bootstrap operation.

## Workflow

The `bootstrap` command executes a series of operations, each managed by a dedicated function within the `bootstrap_commands` module. Memory usage is checked before each major step.

### 1. Setup Environment (`setup_environment`)

*   **Creates a temporary directory:** A temporary working directory is created.
*   **Initializes a new repository:** A new `ragit` repository is initialized within the temporary directory.
*   **Initializes a new index:** A new `Index` structure is created and prepared for use.

### 2. Copy Prompts (`copy_prompts`)

*   **Copies prompts:** The `prompts` directory from the actual root is copied to the temporary directory.
*   **Loads prompts into index:** The copied prompts are loaded into the in-memory `Index` structure.

### 3. Add Bootstrap Files (`add_bootstrap_files`)

*   **Identifies and copies files:** Relevant `.rs` files from the `ragit-command-bootstrap` package are identified and copied to the temporary directory.
*   **Adds files to index:** The copied files are added to the `Index`.

### 4. Build Index (`build_index`)

*   **Builds the index:** The `Index` is built from the source code, processing the content of the added `.rs` files.

### 5. Write Chunks to Markdown (`write_chunks_to_markdown`)

*   **Generates markdown:** The processed chunks are written to a markdown file for review.

### 6. Self-Improvement (`perform_self_improvement`)

*   **Analyzes and improves code:** The command reads its own source code, generates a prompt, executes a query, and writes the improved code to a file.

### 7. Final Reflective Query (`perform_final_reflective_query`)

*   **Executes a reflective query:** A hardcoded query is executed against the built index, and the response is printed.

## File Locations

The core `bootstrap_index_self` function is located at:
- `./crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_command.rs`

Supporting functions for adding bootstrap files are located at:
- `./crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/add_bootstrap_files.rs`

Unit tests for the bootstrap command can be found at:
- `./crates/layer7_application/commands/ragit-command-bootstrap/tests/bootstrap_test.rs`

## File Locations

The core `bootstrap_index_self` function is located at:
- `./crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_command.rs`

Supporting functions for adding bootstrap files are located at:
- `./crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/add_bootstrap_files.rs`

Unit tests for the bootstrap command can be found at:
- `./crates/layer7_application/commands/ragit-command-bootstrap/tests/bootstrap_test.rs`

Additional relevant files:
- `./crates/layer7_application/commands/ragit-command-bootstrap/src/file_source.rs`
- `./crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/constants.rs`

## Debugging and Memory Management

If you encounter issues with the `bootstrap` command, especially Out-Of-Memory (OOM) errors, here's what we've learned and how to approach debugging:

### Flags for Debugging

*   `--verbose`: Enables verbose output, which is useful for debugging. The build dashboard now includes more detailed memory usage metrics, such as the delta since the beginning of the process and the average memory usage per loop.
*   `--max-iterations <NUMBER>`: Exits gracefully after a certain number of iterations. This is primarily for debugging and will cause the process to exit gracefully once the specified number of iterations is reached.
*   `--max-memory-gb <NUMBER>`: Sets a maximum memory limit in gigabytes for the bootstrap operation. **Important Note:** This flag is designed for the `ragit` application to *gracefully exit* if its memory usage exceeds the specified limit. It does **not** prevent the operating system from terminating the process due to an Out-Of-Memory (OOM) condition if the system's overall memory is exhausted. If you're experiencing OOM kills (exit code 137), it means the process is consuming memory faster than the application can detect and react, or the system's memory is simply too constrained.

### Understanding OOM Kills (Exit Code 137)

An exit code of 137 typically indicates that the process was terminated by the operating system due to an Out-Of-Memory (OOM) condition. This can happen even if the application is attempting to shut down gracefully, as the system may run out of memory before the application completes its cleanup.

To mitigate persistent OOM issues:
1.  **Increase available RAM:** Ensure your system has sufficient free RAM.
2.  **Run on a more powerful system:** Consider executing the command on a machine with more memory.
3.  **Reduce the scope:** If possible, try to process fewer files or smaller files during the bootstrap process (e.g., by adjusting the `CHUNK_PROCESSING_LIMIT` if applicable, or by targeting specific subsets of files).

### Full Backtrace

To get a detailed backtrace in case of a crash, set the `RUST_BACKTRACE` environment variable to `full`:

```bash
export RUST_BACKTRACE=full
cargo run --package ragit-commands -- bootstrap
```

This will provide a more comprehensive stack trace, which can be invaluable for debugging panics and other runtime errors.

### Memory Profiling with jemalloc

To generate a memory profile using `jemalloc`, you need to set the `MALLOC_CONF` environment variable before running the `bootstrap` command. This will create a heap profile file that you can analyze.

1.  **Set `MALLOC_CONF`**:
    Set the `MALLOC_CONF` environment variable to specify the profiling options. For example, to dump a profile on exit:

    ```bash
    export MALLOC_CONF="prof:true,prof_dump:true,prof_prefix:jeprof"
    ```

    *   `prof:true`: Enables profiling.
    *   `prof_dump:true`: Dumps a profile on exit.
    *   `prof_prefix:jeprof`: Sets the prefix for the profile files (e.g., `jeprof.<pid>.<seq>.heap`).

2.  **Run the `bootstrap` command**:

    ```bash
    cargo run --package ragit-commands -- bootstrap
    ```

    When the process exits (e.g., due to `max_iterations` being reached or normal completion), a `.heap` file will be generated in the current working directory.

3.  **Analyze the profile**:
    You can use `jeprof` (a Perl script that comes with `jemalloc`) to analyze the generated `.heap` file. You'll typically need `graphviz` installed to generate visual graphs.

    ```bash
    jeprof --svg target/debug/ragit-commands jeprof.<pid>.<seq>.heap > profile.svg
    ```

    *   `target/debug/ragit-commands`: Path to your executable.
    *   `jeprof.<pid>.<seq>.heap`: The generated heap profile file.
    *   `profile.svg`: The output SVG file with the call graph.

    This will generate an SVG file showing the memory allocation call graph, helping you identify memory leaks or high memory usage areas.

### Internal Module Structure and Imports

During development, the `memory_utils` module was moved from `ragit-command-bootstrap` to `ragit-utils` to resolve cyclic dependencies and improve modularity. To maintain compatibility with existing `use` statements within `ragit-command-bootstrap`, a shim was introduced in `crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_commands/mod.rs` (specifically, `pub use crate::memory_utils;`). This allows other modules within `ragit-command-bootstrap` to continue using `crate::bootstrap_commands::memory_utils` without needing to update every single import path. This approach prioritizes functionality and avoids extensive refactoring when a simpler solution is available.
