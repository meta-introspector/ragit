# RAGIT Layered Architecture Plan

This document outlines the strategic plan to refactor the `ragit` project into a formal layered architecture. This plan is the result of the "Have a KitKat" meta-program initiated on 2025-07-28.

## 1. Rationale

The project has been successfully refactored into numerous small, focused crates. While this improves modularity, the large number of crates in a flat structure makes it difficult to understand the overall architecture and dependency flow.

Organizing crates into distinct layers will:
- Provide a clear conceptual map of the system.
- Enforce stricter dependency rules (e.g., higher layers depend on lower layers, not vice-versa).
- Simplify maintenance and onboarding for new contributors.
- Align the physical project structure with its logical design.

## 2. Architectural Layers

The architecture will be based on the "Ragit Crate OSI Model" previously conceptualized.

- **Layer 1 (Physical):** Core data structures and fundamental types.
  - `ragit-types`, `ragit-core`
- **Layer 2 (Data Link):** Filesystem, UID generation, and ignore patterns.
  - `ragit-fs`, `ragit-uid`, `ragit-ignore`
- **Layer 3 (Network):** Indexing and core utilities.
  - `ragit-index`, `ragit-utils`
- **Layer 4 (Transport):** Data readers, querying, and transport utilities.
  - `ragit-readers`, `ragit-query`, `ragit-core-utils`
- **Layer 5 (Session):** Session management, configuration, and rate limiting.
  - `ragit-session-query`, `ragit-config`, `ragit-rate-limit`
- **Layer 6 (Presentation):** Schema definitions, prompt definition language (PDL), and model-specific data structures.
  - `ragit-schema`, `ragit-pdl`, `ragit-model-query-response`, `korean`
- **Layer 7 (Application):** User-facing applications and commands.
  - `ragit-cli`, `api`, `server`, `ragit-commands`, `ragit-model`, `ragit-model-provider`, `ragit-groq-data`, `ragit-muse`

## 3. Refactoring Steps

1.  **Create Directory Structure:** Create new directories inside `crates/` for each layer (e.g., `crates/layer1_physical`, `crates/layer2_data_link`).
2.  **Move Crates:** Move each crate from `crates/` into its corresponding new layer directory.
3.  **Update Root `Cargo.toml`:** Modify the `workspace.members` list to reflect the new paths for all crates.
4.  **Verify Build:** Run `cargo check --workspace` to ensure the compiler can resolve all dependencies and the new structure is valid.
5.  **Documentation:** Update `README.md` and other relevant architectural documents to reflect the new structure.
