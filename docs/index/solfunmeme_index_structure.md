# Solfunmeme Index Structure

This document details the file and directory structure of the `solfunmeme-index`, which serves as a core component for the `ragit` project's indexing and semantic understanding capabilities.

## Directory: `terms/`
This directory likely contains individual term definitions or processed data related to terms, possibly in a granular format.

## Files:

*   `README.md`: Provides an overview and instructions for the `solfunmeme-index`.
*   `.gitattributes`: Configuration for Git attributes, affecting how files are handled (e.g., line endings, merge strategies).
*   `augmented_terms_hot_take.0.json`: A snapshot or intermediate version of augmented terms, possibly for quick review or testing.
*   `augmented_terms_hot_take.json`: A version of augmented terms, likely for rapid iteration or experimental data.
*   `augmented_terms.jsonl`: Augmented terms in JSON Lines format, suitable for large datasets where each line is a separate JSON object.
*   `directory_vectors.json`: Contains vector representations of directories, used for semantic search and understanding of code structure.
*   `models.json`: Configuration or metadata for models used within the indexing process (e.g., embedding models, classification models).
*   `path_relationship_matrix.json`: A matrix detailing relationships between different file paths, possibly indicating dependencies or semantic connections.
*   `term_path_map_external.json`: Mapping of terms to external file paths.
*   `term_path_map_internal.json`: Mapping of terms to internal file paths within the project.
*   `term_path_map.json`: A general mapping of terms to file paths.
*   `tree_level_3.json`: A representation of the project's file tree structure, possibly at a specific hierarchical level (e.g., level 3).
*   `tree_term_report_external.json`: A report on terms found in external parts of the project's file tree.
*   `tree_term_report_internal.json`: A report on terms found in internal parts of the project's file tree.
*   `tree_term_report.json`: A general report on terms found across the project's file tree.
