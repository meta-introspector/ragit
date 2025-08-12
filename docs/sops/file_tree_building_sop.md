# SOP: File Tree Building with LCA and Term Weights

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the process for building a hierarchical file tree, calculating term weights and fractions for each node, and identifying the Lowest Common Ancestor (LCA) for pairs of files within the `ragit` project. This provides a structured view of the codebase and helps understand the semantic connections between files.

## 2. Scope
This SOP applies to the `build_file_tree.rs` program and its interaction with the `ragit-feature-extractor` crate.

## 3. Problem Identification
The initial approach to finding similar code chunks involved simulated annealing, which proved to be computationally intensive and lacked a clear hierarchical representation of file relationships. There was a need for a more intuitive and structured way to visualize file organization, term distribution, and commonalities between files.

## 4. Analysis
To address the limitations of the simulated annealing approach, a tree-based representation was chosen. This allows for a natural hierarchical view of the file system. The `ragit-feature-extractor` crate was created to centralize the logic for extracting file features (term counts, pairs, triples) and sampling reports, ensuring modularity and reusability. The LCA algorithm was chosen to identify common denominators between files within the hierarchical structure.

## 5. Procedure/Modifications

### 5.1. Creation of `ragit-feature-extractor` Crate

A new binary/library hybrid crate, `ragit-feature-extractor`, was created to encapsulate the logic for extracting file features and generating sampled reports. This promotes modularity and allows `build_file_tree.rs` to consume its output.

*   **Command:** `cargo new crates/ragit-feature-extractor --bin`
*   **Code Moved:** The content of the former `report_path_embeddings.rs` was moved to `crates/ragit-feature_extractor/src/lib.rs` and exposed via a public function `get_sampled_reports()`.
*   **Dependencies Added:** `itertools`, `rand`, `serde`, `serde_json`, and `regex` were added to `crates/ragit-feature-extractor/Cargo.toml`.
*   **Main Function:** A new `main.rs` was created in `ragit-feature-extractor` to call `get_sampled_reports()` for standalone execution.

### 5.2. Modifications to Main `Cargo.toml`

*   The `report_path_embeddings` binary entry was removed.
*   `ragit-feature-extractor` was added as a dependency to the main `Cargo.toml`.

### 5.3. Modifications to `build_file_tree.rs`

*   **Dependency:** `ragit-feature-extractor` was added as a dependency.
*   **`TreeNode` Struct Enhancement:**
    *   `total_weight: f64`: Stores the sum of term counts for the node and its children.
    *   `fraction: f64`: Stores the `total_weight` as a fraction of its parent's `total_weight`.
*   **`add_path` Function:**
    *   **Purpose:** Recursively builds the directory hierarchy.
    *   **Logic:** Iterates through path parts, creating child nodes as needed. Term counts from `FileReport` are added to the *final* node (the file itself) in the path.
    *   **Code Snippet (Simplified):**
        ```rust
        fn add_path(&mut self, path: &[&str], file_report: &FileReport) {
            if path.is_empty() {
                for (term, count) in &file_report.term_counts {
                    *self.term_counts.entry(term.clone()).or_insert(0) += count;
                }
                return;
            }
            let (current, rest) = path.split_at(1);
            let current_name = current[0];
            let child = self.children.entry(current_name.to_string()).or_insert_with(|| TreeNode::new(current_name));
            child.add_path(rest, file_report);
        }
        ```
*   **`aggregate_term_counts` Function:**
    *   **Purpose:** Aggregates term counts from children nodes up to their parents.
    *   **Logic:** Recursively calls itself on children and sums up their term counts into the current node's `term_counts`.
*   **`calculate_weights_and_fractions` Function:**
    *   **Purpose:** Calculates `total_weight` and `fraction` for each node.
    *   **Logic:** `total_weight` is the sum of `term_counts`. `fraction` is `total_weight` divided by parent's `total_weight`.
*   **LCA Logic (`find_path_to_node`, `find_lca`):**
    *   **`find_path_to_node`:** Traverses the tree to find the sequence of nodes from the root to a target file.
    *   **`find_lca`:** Compares the paths to two files to find their Lowest Common Ancestor.
    *   **Code Snippet (Simplified `find_lca`):**
        ```rust
        fn find_lca<'a>(root: &'a TreeNode, path1_parts: &[&'a str], path2_parts: &[&'a str]) -> Option<&'a TreeNode> {
            let path1_nodes = find_path_to_node(root, path1_parts)?;
            let path2_nodes = find_path_to_node(root, path2_parts)?;
            let mut lca: Option<&TreeNode> = None;
            for (node1, node2) in path1_nodes.iter().zip(path2_nodes.iter()) {
                if node1.name == node2.name {
                    lca = Some(node1);
                } else {
                    break;
                }
            }
            lca
        }
        ```
*   **`main` Function:**
    *   Calls `ragit_feature_extractor::get_sampled_reports()` to get the sampled files.
    *   Builds the `TreeNode` hierarchy using `add_path`.
    *   Calls `aggregate_term_counts()` and `calculate_weights_and_fractions()`.
    *   Iterates through pairs of sampled files, finds their LCA using `find_lca`, and prints the LCA along with the paths from the LCA to each file.

## 6. Tools Used
*   `read_file`
*   `write_file`
*   `replace`
*   `run_shell_command`
*   `git`
*   `cargo new`
*   `cargo add`

## 7. Expected Outcome
*   A hierarchical representation of files with accurate term counts, total weights, and fractions for each node.
*   Clear identification of the Lowest Common Ancestor (LCA) for pairs of sampled files, demonstrating their commonalities within the project structure.
*   Improved understanding of term distribution and file relationships.

## 8. Verification
The output of `build_file_tree.rs` was reviewed to confirm:
*   The tree structure accurately reflects the file hierarchy.
*   Term counts, total weights, and fractions are correctly calculated and aggregated.
*   LCA identification is accurate, and paths from the LCA to the files are correctly displayed.
