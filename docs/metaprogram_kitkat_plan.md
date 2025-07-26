# Meta-Program: Have a KitKat (2025-07-26)

This document outlines the next critical path for the RAGIT project, established after a "kitkat" meta-program cycle.

## New Critical Path: Achieving True Self-Improvement

The `bootstrap_index_self` function is now technically operational but relies on placeholder functions. The next critical path is to make the self-improvement loop *real* by implementing the underlying logic.

### 1. Implement Core Functions

The immediate priority is to replace the placeholder logic in the core indexing and querying pipeline.

- **`add_files_command`:** Replace the placeholder with the actual implementation that reads files from the specified path, breaks them into chunks according to the project's chunking strategy, and adds them to the index's staging area.
- **`index.build`:** Implement the logic to process the staged chunks. This includes generating summaries, calculating TF-IDF scores, creating vector embeddings, and building the final, queryable index structures.
- **`index.query`:** This is the most critical step. Connect this function to a real LLM backend (e.g., via the `ragit-api` crate). The function must be able to take a text prompt, send it to the configured LLM, and return the response.

### 2. Execute and Refine the Self-Improvement Loop

With the core functions implemented, the next step is to test and refine the self-improvement loop.

- **Execute:** Run the `bootstrap_index_self` command.
- **Debug:** Carefully monitor and debug the entire process:
    - Verify that the bootstrap command's own source code is correctly read and chunked.
    - Ensure the index is built successfully.
    - Confirm that the self-improvement prompt is sent to the LLM.
    - Validate that the LLM's response is a meaningful code suggestion.
    - Ensure the `write_string` operation successfully overwrites the source file with the improved code.
- **Goal:** The primary objective is to achieve a state where the `bootstrap_index_self` function can successfully and meaningfully modify its own source code in a single, automated run.

### 3. Documentation and Reflection

Once the self-improvement loop is fully functional, it needs to be properly documented.

- **Command Documentation:** Update `docs/commands/bootstrap.md` to reflect the command's new, powerful self-improvement capability.
- **Project README:** Add a section to the main `README.md` or a core architecture document explaining this unique feature of the RAGIT system. This is a key differentiator and should be highlighted.
