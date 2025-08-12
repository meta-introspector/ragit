# SOP: Code Archaeology and Documentation

## 1. Objective
To understand and document existing code functionality before writing new code, in order to avoid duplication and promote reuse.

## 2. Guiding Principles
- **Search Before You Write**: The code you need probably already exists.
- **Find the Vibe**: Use the project's `ragit` and `vibe` systems to find the semantic and structural location of the code you need.
- **Avoid Duplication**: Do not write new code if existing code can be reused or refactored.

## 3. Procedure

### Step 1: Define Your Target
Clearly define the functionality you are looking for or the crate you want to understand.

### Step 2: Start with the Manifest
- Begin any investigation by examining the `Cargo.toml` file of the relevant crate. This provides immediate context on its dependencies and purpose.
- To find where a crate is used, search for its name as a dependency in all other `**/Cargo.toml` files.

### Step 3: Use Focused Search
- **Avoid broad, keyword-based searches** across the entire codebase (e.g., searching for `vector`). These are slow and produce noisy results.
- **Prioritize structural search**:
    - Search for specific crate names in `Cargo.toml` files.
    - Search for specific module or function names in `*.rs` files.
    - Use `ragit`'s more advanced querying capabilities when available.

### Step 4: Document Your Findings
- Create a dedicated Markdown file for each component (e.g., crate, module) you document.
- Store these files in a structured location, such as `docs/rust_code/`.
- The documentation should cover:
    - Purpose and high-level overview.
    - Key dependencies.
    - Core modules and functions.
    - Usage examples and interaction patterns.

### Step 5: Update the Central Index
- After creating a new documentation file, **immediately** add a link to it in `docs/index.md`. This is critical for discoverability.

### Step 6: The "KitKat" Meta-Program
- If a task becomes frustrating, slow, or you feel stuck, initiate a "KitKat break".
- **Pause**: Stop the current task.
- **Reset**: Clearly state the problem and the goal.
- **Re-plan**: Define a new, clear, and focused plan with smaller, achievable steps.
- **Commit**: Commit any useful work done so far.
- **Resume**: Continue with the new, simplified plan.
