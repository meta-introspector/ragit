## SOP: Building and Installing `coccinelleforrust`

### 1. Purpose
This SOP outlines the process for correctly building and installing the `coccinelleforrust` (cfr) tool, ensuring it is ready for use in the project.

### 2. Scope
This SOP applies to all developers and environments involved in using `coccinelleforrust` for Rust code refactoring.

### 3. Procedure
1.  **Navigate to the `coccinelleforrust` directory:**
    ```bash
    cd /data/data/com.termux/files/home/storage/github/ragit/vendor/coccinelleforrust
    ```
2.  **Build the `coccinelleforrust` tool:**
    Execute the cargo build command in release mode. This will compile the `cfr` binary.
    ```bash
    cargo build --release
    ```
3.  **Install the `cfr` binary:**
    Copy the compiled `cfr` binary to a directory that is included in your system's `PATH` environment variable. A common location is `~/.local/bin/`.
    ```bash
    mkdir -p ~/.local/bin/
    cp target/release/cfr ~/.local/bin/
    ```

### 4. Verification
To verify the installation, open a new terminal or refresh your shell environment and run:
```bash
cfr --version
```
This should display the version information for `coccinelleforrust`.

### 5. Related SOPs
-   SOP 2: Running a Semantic Patch with `cfr`
