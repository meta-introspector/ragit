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

The `bootstrap` command follows a pipeline of operations, each of which is broken down into a separate module.

### 1. Setup Environment

*   **Creates a temporary directory:** The `bootstrap` command starts by creating a temporary directory to work in.
*   **Initializes a new repository:** It initializes a new `ragit` repository in the temporary directory.
*   **Initializes a new index:** It creates a new index and saves it to a file.

### 2. Copy Prompts

*   **Copies prompts:** It copies the `prompts` directory, which is essential for building the index.
*   **Loads prompts into index:** The copied prompts are loaded into the in-memory `Index` structure for use during the build process.

### 3. Add Bootstrap Files

*   **Finds files:** It finds all the `.rs` files in the `ragit-command-bootstrap` package using `glob`.
*   **Copies files:** It copies the content of these files to the temporary directory.
*   **Adds files to index:** It adds the files to the index.

### 4. Build Index

*   **Builds the index:** It builds a new index from the source code. This step now correctly processes the content of the copied `.rs` files.

### 5. Write Chunks to Markdown

*   **Writes chunks to markdown:** It writes the chunks to a markdown file for inspection.

### 6. Self-Improvement

*   **Gets its own source code:** It reads the `lib.rs` file of the `ragit-command-bootstrap` crate.
*   **Formats a prompt:** It creates a prompt with the source code.
*   **Executes a query:** It executes a query with the prompt.
*   **Handles the improved code:** It writes the improved code to a file in the temporary directory.

### 7. Final Reflective Query

*   **Executes a hardcoded query:** It executes a hardcoded query and prints the response.

## Debugging

If you encounter issues with the `bootstrap` command, you can use the `--verbose` flag to get more detailed output. The build dashboard now includes more detailed memory usage metrics, such as the delta since the beginning of the process and the average memory usage per loop.

Additionally, for debugging purposes, the `render_build_dashboard` function in `crates/ragit-index-effects/src/build_dashboard.rs` can be configured to exit gracefully after a certain number of iterations. This `max_iterations` parameter can be passed to the `bootstrap` command.

```sh
cargo run --package ragit-commands -- --verbose bootstrap --max-iterations <NUMBER>
```

Note: The `max_iterations` parameter is primarily for debugging and will cause the process to exit gracefully once the specified number of iterations is reached.

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
