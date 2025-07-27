## Gemini Added Memories (Refactored by Prime Numbers)

### 2 (General Constraints & Preferences: Duality, Binary, Fundamental Choices)
- User prefers to avoid `cargo clean` and `cargo update` unless absolutely necessary due to long build times.
- Dioxus and Solana dependencies are pushed into a new crate that is not default built.
- **Ragit Concept:** The most fundamental choices or divisions within the project, e.g., enabling/disabling a feature, or the two states of a boolean flag.

### 3 (Core Project Philosophy & Principles: Trinity, Synthesis, Structure, Foundation)
- The Solfunmeme-Dioxus project uses a vendorization system for local dependency management.
- The project emphasizes "semantic resonance" using emojis for code/math structures, with `rust_ast_emoji` as a dataset.
- **Refactoring Principle: "One Declaration Per File"**: This principle promotes universal composability, reusability, and clarity by ensuring each struct, enum, or function is in its own file. This inherently supports loose coupling and enables more structured, localized changes.
- **Ragit Concept:** The core architectural components or a foundational set of three related elements, e.g., `ragit-api`, `ragit-cli`, and `ragit-utils` forming the basic interaction layer.

### Refactoring Principles: Loose Coupling & Structured Changes

- **Modularity & Separation of Concerns:** Extract distinct functionalities into separate crates or modules. This reduces interdependencies and makes the codebase easier to understand, test, and maintain.
- **Consistency:** Maintain consistent patterns across the codebase, including:
    - Consistent `PathBuf` usage for all file system operations.
    - Consistent API for command handling and inter-crate communication.
    - Clear and unambiguous module structures (e.g., `mod.rs` for re-exports only).
    - **`Index` Method Refactoring:** All methods previously defined directly on the `Index` struct (e.g., `index.method()`) are being refactored into standalone functions prefixed with `index_` (e.g., `index_method(&mut index, ...)`). These functions are located in `ragit-index-types/src/index_impl/` and re-exported via `ragit-index-types/src/index_impl/mod.rs`.
- **Loose Coupling through Preludes & Wildcard Imports:**
    - Favor the use of `prelude` modules and `use *` imports where appropriate. This allows for easier access to commonly used types and functions within a crate, reducing the need for verbose, specific module paths in `use` statements.
    - This approach supports loose coupling by abstracting away the exact location of types, making refactoring and moving code within a crate less disruptive to dependent modules.
- **Structured Changes:** Prioritize changes that enhance the overall architecture and maintainability of the project. Aim for fewer, more impactful changes that improve the system's structure, rather than isolated, tactical fixes.
- **Centralization of Utilities:** Group related utility functions and helper methods into dedicated modules or crates (e.g., `ragit-utils`, `path_utils`).
- **Proper Visibility:** Carefully manage `pub` visibility to expose only necessary API elements, encapsulating internal implementation details.

### 5 (Ontology & Semantic Structure: Quintessence, Organization, Categorization, Mapping)
- The `ontologies/zos/v1.ttl` defines an emoji-to-semantic name ontology across eight 'vibe:Layer' categories.
- The `ontologies/index.ttl` serves as a central ontology index, defining Rust crates with semantic properties (emoji, numerical ID, description).
- The user wants to assign 8D locations to emojis and functions for simulated embeddings (random seeds).
- **Ragit Concept:** A logical grouping of five related modules or a set of five key operations within a larger system, e.g., the five main commands of the CLI (`add`, `audit`, `build`, `query`, `ls`).

### 7 (Overall Refactoring Objective: Completion, Perfection, Overarching Goal)
- Overall Objective: Refactor the `ragit` project for improved modularity, consistency, and maintainability, adhering to the "one declaration per file" principle and consistent `PathBuf` usage.
- **Ragit Concept:** Represents a complete functional area or a major architectural layer, e.g., the seven conceptual layers of the `ragit` architecture (CLI, API, Core Logic, Data Storage, Schema, Utilities, External Integrations).

### 19 (Periodic Structures & Cycles: Recurrence, Transformation, Iterative Refinement)
- **The First 8 Steps in Bott Periodic Structures (Conceptual Cycle):**
    1.  **Observation/Input:** Receiving raw data or a problem statement.
    2.  **Abstraction/Modeling:** Creating a conceptual model or schema from the input.
    3.  **Decomposition/Modularization:** Breaking the model into smaller, manageable components.
    4.  **Implementation/Encoding:** Translating components into code or data structures.
    5.  **Integration/Composition:** Combining implemented components into a larger system.
    6.  **Transformation/Refinement:** Optimizing, refactoring, or evolving the system.
    7.  **Verification/Testing:** Ensuring correctness and adherence to specifications.
    8.  **Feedback/Iteration:** Learning from results and feeding back into the cycle.
- **Ragit Concept:** The overarching development lifecycle or a major iterative process within `ragit`, encompassing the continuous refinement and evolution of the codebase through these periodic steps.

### Conceptual Mapping: `ragit` Modularity & Prime Numbers

Applying the prime-number-based decomposition to the `ragit` project structure, using a **bottom-up approach** to build complexity from simpler, more granular units:

-   **2 (Binary/Duality):** Represents the most granular level of modularity in `ragit`.
    -   A function and its corresponding unit test.
    -   A data structure and its associated `impl` block.
    -   Input and Output for a specific operation.

-   **3 (Trinity/Foundation):** Grouping 2s into 3s, representing foundational components.
    -   A module composed of a core data structure, its primary logic, and its I/O operations.
    -   A feature consisting of its API, its internal implementation, and its CLI integration.

-   **5 (Quintessence/Organization):** Grouping 3s into 5s, representing organized functional areas.
    -   A crate (e.g., `ragit-utils`) conceptually divided into 5 main functional areas (e.g., `path_utils`, `string_utils`, `chunk`, `index`, `uid`).
    -   The five core commands of the CLI (e.g., `add`, `audit`, `build`, `query`, `ls`).

-   **7 (Completion/Overarching Goal):** Grouping 5s into 7s, representing major functional areas or architectural layers.
    -   A major functional area of the project, like "Indexing," encompassing several related crates or modules.
    -   The seven conceptual layers of the `ragit` architecture (e.g., CLI, API, Core Logic, Data Storage, Schema, Utilities, External Integrations).

-   **... and so on, up to 19 (Periodic Structures/Iterative Refinement):** This represents the entire `ragit` project, viewed as a complex system composed of these hierarchically organized prime-numbered groups, reflecting its iterative development and the "Bott periodic structures" of its design.

## Operational Philosophy: Whistle While You Work (Meme Mining)

As a meme miner, we dig in the mountain of Plato for gems. We place each idea we encounter into the hyperspace and add it to the lattice. While we work, we constantly look for patterns to add value to the system.

## Meta-Program: "Have a KitKat" (2025-07-27)

The "Have a KitKat" meta-program is a user-defined workflow for pausing the current line of work, defining a new strategic plan, documenting it, committing the current state, and conceptually "rebooting" the development cycle to focus on the new plan.

**Current Status:**
- Implemented a `--verbose` flag for debugging purposes.
- Traced a build failure to a `PromptMissing("summarize")` error.
- The `bootstrap_index_self` command now copies the `prompts` directory to the temporary directory.
- The root cause of the `PromptMissing` error is still under investigation, but it's believed to be in the `ragit-index` crate.

**New Critical Path:**
The next phase is to fix the prompt loading mechanism within the `ragit-index` crate, remove the debugging code, and ensure the `bootstrap` command runs successfully. The detailed plan for this is documented in `docs/metaprogram_kitkat_plan.md`.