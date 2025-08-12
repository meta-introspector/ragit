# Strategic Plan: Numerical Model of Directory Structure

## Objective

To develop an iterative process for generating, evaluating, and refining a numerical (vector) representation of the project's directory structure, leveraging LLM-generated initial weights (from existing term embeddings) and human feedback.

## Phase 1: Initial Model Generation (Term-Embedding Driven)

*   **Define "Directory Structure Unit":** Individual files, represented by `FileReport`s from `ragit-feature-extractor`, will serve as the primary units.
*   **Initial Vector Representation:** The `total_vector` within each `FileReport` (which aggregates existing term embeddings from `term_embeddings.json`) will be used as the initial numerical representation for each file.
*   **Data Preparation:**
    *   `ragit-feature-extractor::get_all_file_reports()` was modified to return all `FileReport`s, not just a sample.
    *   `build_file_tree.rs` now calls `get_all_file_reports()` and performs sampling internally for evaluation purposes.

## Phase 2: Evaluation and Review (Human-in-the-Loop)

*   **Visualization/Projection:**
    *   Implemented PCA in `build_file_tree.rs` to reduce the dimensionality of `total_vector` embeddings to 2D for visualization.
    *   The PCA results (reduced coordinates) are printed alongside file names.
*   **Interactive Review Tool (Quiz Server):**
    *   **Server Component:** Created a new `quiz_server` crate (`crates/quiz_server`) with basic `axum` server.
        *   Implemented `/status` endpoint to report server status.
        *   Implemented `/stop` endpoint for graceful server shutdown.
    *   **CLI Client:** Created a new `ragit-command-quiz` crate (`crates/ragit-command-quiz`) integrated into the main `ragit` CLI.
        *   `ragit quiz start`: Spawns the `quiz_server` in the background.
        *   `ragit quiz stop`: Sends a request to the `/stop` endpoint to shut down the server.
        *   `ragit quiz take`: Placeholder for initiating quiz interaction (will check server status).

## Phase 3: Iterative Adjustment (Tool-driven Refinement)

*   **Feedback Integration:** Develop mechanisms to adjust embeddings based on user feedback (e.g., direct manipulation of vectors, constraint-based optimization).
*   **Loop:** Iteratively refine embeddings based on human judgment and re-evaluation.

## Phase 4: Integration and Application

*   **Integrate with `ragit`:** Incorporate refined embeddings into the `ragit` index for improved semantic search, code navigation, and other features.
*   **Documentation:** Maintain comprehensive documentation throughout the process.

---
