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

## Debugging

If you encounter issues with the `bootstrap` command, you can use the `--verbose` flag to get more detailed output. The build dashboard now includes more detailed memory usage metrics, such as the delta since the beginning of the process and the average memory usage per loop.

Additionally, for debugging purposes, the `render_build_dashboard` function in `crates/ragit-index-effects/src/build_dashboard.rs` can be configured to exit gracefully after a certain number of iterations. This `max_iterations` parameter can be passed to the `bootstrap` command.

```sh
cargo run --package ragit-commands -- --verbose bootstrap --max-iterations <NUMBER>
```

Note: The `max_iterations` parameter is primarily for debugging and will cause the process to exit gracefully once the specified number of iterations is reached.

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
