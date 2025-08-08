## SOP: Running a Semantic Patch with `cfr`

### 1. Purpose
This SOP outlines the process for applying a semantic patch to Rust source code using the `coccinelleforrust` (cfr) tool to perform automated code transformations.

### 2. Scope
This SOP applies to all developers and environments involved in applying semantic patches to Rust codebases using `cfr`.

### 3. Prerequisites
-   `coccinelleforrust` (cfr) must be installed and accessible in your system's PATH. Refer to [SOP: Building and Installing `coccinelleforrust`](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/build_coccinelleforrust.md) for installation instructions.
-   You must have a semantic patch file (`.cocci` extension) defining the desired code transformation.
-   You must identify the target Rust file or directory to which the patch will be applied.

### 4. Procedure
1.  **Prepare your semantic patch file:** Ensure your `.cocci` file is correctly written and defines the desired transformation.
2.  **Identify your target:** Determine whether you will apply the patch to a single Rust file or an entire directory containing Rust files.
3.  **Execute the `cfr` command:** Use the `cfr` command with the `-c` flag to specify your semantic patch file and provide the target path.

    **a. Applying to a single file (output to stdout):**
    ```bash
    cfr -c your_patch.cocci path/to/your_file.rs
    ```

    **b. Applying to a single file (output to a new file):**
    ```bash
    cfr -c your_patch.cocci path/to/your_file.rs -o path/to/output_file.rs
    ```

    **c. Applying to a directory:**
    ```bash
    cfr -c your_patch.cocci path/to/your_directory/
    ```

    **d. Using the helper script:**
    A convenience script `run_cfr_patch.sh` is available to simplify command execution.
    ```bash
    ./scripts/run_cfr_patch.sh your_patch.cocci path/to/your_file_or_directory [path/to/output_file.rs]
    ```
    -   If `[path/to/output_file.rs]` is provided, the transformed code will be written to that file.
    -   If `[path/to/output_file.rs]` is omitted, the transformed code will be printed to standard output.

### 5. Options
-   `-c, --coccifile <COCCIFILE>`: Specifies the path to the semantic patch file.
-   `-o, --output <OUTPUT>`: Specifies the path to save the transformed file. If omitted, output is printed to stdout.
-   `-r, --rustfmt-config <RUSTFMT_CONFIG>`: Specifies a `rustfmt` configuration file to use for formatting the transformed code.
-   `-i, --ignore <IGNORE>`: Ignores files or folders containing the specified string.
-   `--apply`: Applies the changes directly to the target file(s) in place. Use with caution.
-   `--suppress-diff`: Suppresses the display of the diff output.
-   `--suppress-formatting`: Suppresses `rustfmt` from formatting the output.
-   `--no-parallel`: Disables parallel processing of files.
-   `--worth-trying <STRATEGY>`: Strategy for identifying files that may be matched by the semantic patch. Possible values: `no-scanner`, `grep`, `git-grep`, `cocci-grep` (default).
-   `-d, --debug`: Enables debug logging.
-   `-V, --version`: Prints the version information.
-   `-h, --help`: Prints help information.

### 6. Verification
-   Review the output (either on stdout or in the specified output file) to ensure the transformations were applied as expected.
-   If `--apply` was used, verify the changes directly in the source files.

### 7. Related SOPs
-   [SOP: Building and Installing `coccinelleforrust`](/data/data/com.termux/files/home/storage/github/ragit/docs/sops/build_coccinelleforrust.md)
