# Bootstrap Command

The `bootstrap` command is a powerful tool for developers working on `ragit` itself. It automates the process of creating a new knowledge base from the `ragit` source code, building an index, and then using that index to improve the `ragit` codebase.

## Usage

```bash
cargo run --package ragit-commands -- bootstrap
```

## Workflow

1. **Creates a temporary directory:** The `bootstrap` command starts by creating a temporary directory to work in.
2. **Initializes a new repository:** It initializes a new `ragit` repository in the temporary directory.
3. **Copies source code:** It copies the `ragit` source code into the new repository.
4. **Copies prompts:** It copies the `prompts` directory, which is essential for building the index.
5. **Builds the index:** It builds a new index from the source code.
6. **Self-improvement (Prompt Generation for Evaluation):** This step now uses the newly created index to query an LLM and generate prompts for evaluation. These prompts are stored in `tmp_bootstrap/chunks_output.md` and `tmp_bootstrap/improved_lib.rs`.

## Debugging

If you encounter issues with the `bootstrap` command, you can use the `--verbose` flag to get more detailed output:

```sh
cargo run --package ragit-commands -- --verbose bootstrap
```
