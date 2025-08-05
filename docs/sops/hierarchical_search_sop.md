# SOP: Hierarchical Search Strategy

## 1. Objective
To provide a structured, efficient, and hierarchical methodology for locating code, documentation, and concepts within the `ragit` project. This SOP prioritizes targeted searches to avoid slow, resource-intensive queries and to ensure the most relevant information is found first.

## 2. Guiding Principle
Always start with the highest-level, most structured information and progressively move towards more granular, content-based searches. This is the "Vibe Search" philosophy: understand the structure and connections before diving into the raw code.

## 3. Procedure

Follow these steps in order:

### 1. Search the Tree (`tree_level_3.json`)
- **Action:** Examine the `tree_level_3.json` file.
- **Purpose:** To get a high-level overview of the project's directory and file structure. This is the fastest way to find the general location of a crate, module, or document.

### 2. Search the Cargo Manifests (`Cargo.toml`)
- **Action:** Search within all `**/Cargo.toml` files.
- **Purpose:** To understand the dependency graph. This reveals how crates are connected and which components rely on the functionality you are investigating.

### 3. Narrow the Search (Targeted Globs)
- **Action:** Use focused `glob` or `find` commands based on the information gathered in the previous steps.
- **Purpose:** To pinpoint specific files (e.g., `**/module_name/src/lib.rs`) without reading their content. This verifies the existence and exact path of a component.

### 4. Search the Documentation (`**/*.md`)
- **Action:** Search within all Markdown (`.md`) files.
- **Purpose:** To find high-level explanations, SOPs, architectural decisions, and usage examples. The documentation often provides context that is not present in the code itself.

### 5. Search the Ontologies (`**/*.ttl`)
- **Action:** Search within all Turtle (`.ttl`) files.
- **Purpose:** To understand the semantic relationships, data models, and core concepts defined in the project's ontologies. This is crucial for understanding the "vibe" and semantic structure.

### 6. Search the Vibes (`directory_vectors.json`)
- **Action:** Query the `directory_vectors.json` and other semantic vector stores.
- **Purpose:** To find code or concepts that are semantically related to your query, even if they don't share the same keywords. This is a powerful tool for discovering non-obvious connections.

### 7. Search the Rust Files (`**/*.rs`)
- **Action:** As a final step, perform a content search within Rust (`.rs`) source files.
- **Purpose:** To find the specific implementation details, function definitions, and low-level logic. This search should be as targeted as possible, using information from all previous steps to narrow the scope.
