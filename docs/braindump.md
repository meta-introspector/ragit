## Corporate Style Guide: Allowed Languages

To maintain a consistent and efficient development environment, the following languages are exclusively permitted for use in this project:

-   **Rust**
-   **Lean 4**
-   **WebAssembly (Wasm)**
-   **eBPF**
-   **LLVM IR**

No other languages, including but not limited to Python, Golang, or TypeScript, are allowed.

## Development Strategy: Small Changes and Small Pull Requests

To ensure iterative self-improvement, maintain high code quality, and adhere to compliance standards (ISO 9000, GMP, ITIL), we adopt the following development strategy:

-   **Atomic Commits:** Each commit must represent a single, logical, and self-contained change. Avoid bundling unrelated modifications into one commit.
-   **Focused Branches:** Feature branches will be short-lived and dedicated to a single, well-defined task or bug fix. Branch names should be descriptive and link to the change request ID.
-   **Automated Verification:** Every change must be accompanied by comprehensive automated tests (unit, integration, and where applicable, end-to-end) to ensure correctness, prevent regressions, and validate compliance.
-   **Clear Documentation:** All changes will be formally documented through the `ragit change request` command. Commit messages will be concise, follow a standardized format, and reference the associated change request.
-   **Iterative Development:** Large features will be broken down into the smallest possible functional increments. Each increment will follow this "Small Changes" strategy, allowing for continuous integration and rapid feedback.
-   **"Vibe-Driven Development":** Each change, branch, and feature should embody a clear "vibe" or "vector," representing a distinct conceptual unit within the Code-Math Manifold. This ensures semantic resonance and alignment with the project's core philosophy.

## Agent Communication and Task Distribution

Agents will communicate and distribute tasks using an "outbox" and "inbox" model.
-   **Outbox:** Agents will write their available capacity or task requests to a shared "outbox."
-   **Inbox:** Other agents will monitor the "outbox" and respond with work assignments to the requesting agents' "inboxes."
-   **Source Referencing:** Tasks and work context will be referenced using GitHub and Hugging Face URLs, leveraging the existing `ragit` infrastructure for content-addressed chunks and knowledge management.
