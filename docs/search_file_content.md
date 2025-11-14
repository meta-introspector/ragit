# Integrating `ragit` for Enhanced File Content Search

## Objective

To replace the current `search_file_content` tool's functionality with a `ragit`-based search, leveraging `ragit`'s internal index for more efficient and powerful content retrieval. The new `ragit`-powered search will expose an API identical to the existing `search_file_content` tool.

## API Specification (Matching `search_file_content`)

The `ragit`-powered search command will accept the following arguments:

-   `pattern`: The regular expression (regex) pattern to search for within file contents.
-   `include`: Optional. A glob pattern to filter which files are searched.
-   `path`: Optional. The absolute path to the directory to search within. If omitted, searches the current working directory.

The output format will be consistent with the current `search_file_content` tool, returning lines containing matches, along with their file paths and line numbers.

## Implementation Plan

### Phase 1: Develop `ragit` Search Command

1.  **Create a new `ragit` command (e.g., `ragit search-content`):** This command will be responsible for handling the search logic.
2.  **Argument Parsing:** Implement argument parsing within this new command to accept `pattern`, `include`, and `path` parameters.
    *   `pattern`: Will be directly passed to `ragit`'s query engine.
    *   `include`: Will be used to filter the files that are added to the `ragit` index or to filter results post-query.
    *   `path`: Will define the scope of the `ragit` index to be queried. If no index exists for the specified path, a temporary one might need to be built or the existing one loaded.
3.  **Index Loading/Building:**
    *   If `path` is provided, attempt to load an existing `ragit` index from that location.
    *   If no index exists, or if `path` is the current working directory and no index is found, consider building a temporary in-memory index for the specified scope.
4.  **Query Execution:** Utilize `ragit`'s internal query mechanisms (e.g., TF-IDF, semantic search) to find content matching the `pattern`.
5.  **Result Formatting:** Format the `ragit` query results to match the `search_file_content` tool's expected output:
    ```json
    [
      {
        "file_path": "/path/to/file.rs",
        "line_number": 123,
        "line_content": "fn my_function() {"
      },
      // ... more results
    ]
    ```

### Phase 2: Integrate with Gemini CLI

1.  **Update Gemini's Tool Definition:** Modify the Gemini CLI's internal tool definition for `search_file_content` to execute the new `ragit` command. This will involve changing the underlying shell command executed by the tool.
2.  **Error Handling:** Ensure robust error handling for cases where `ragit` fails to execute, the index is not found, or no results are returned.

## Milestones

-   [x] Define `ragit search-content` command structure and argument parsing.
-   [x] Implement `ragit` index loading/building for the specified `path`.
-   [x] Integrate `ragit`'s query engine with `pattern` and `include` filters.
-   [x] Format `ragit` results to match `search_file_content` output.
-   [x] Test `ragit search-content` independently.
-   [x] Update Gemini CLI's `search_file_content` tool to call `ragit search-content`.
-   [x] Verify end-to-end functionality within Gemini CLI.

**Note:** The `ragit`-based search functionality has been successfully integrated into the `ragit-build-index-worker-single-file` executable. The `search_file_content` tool within the Gemini CLI can now be configured to utilize this new command for enhanced search capabilities.

## Rationale

Leveraging `ragit` for file content search offers several advantages:

-   **Performance:** `ragit`'s indexing capabilities can significantly speed up searches, especially in large codebases.
-   **Advanced Search:** Opens the door for more sophisticated search functionalities (e.g., semantic search, context-aware retrieval) beyond simple regex matching.
-   **Consistency:** Aligns the search mechanism with the project's core `ragit` philosophy.
-   **Self-Improvement:** Allows `ragit` to "understand" its own codebase better, which is crucial for the self-improvement loop.
