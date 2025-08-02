# Grand Plan: Fundamental Size Structure and Modular Implementation

This document outlines the high-level conceptual "Grand Plan" for the project's fundamental size structures and their generation. The implementation details, while complex, adhere to principles of modularity and consistency.

1.  **Hierarchical Generation:** We generate structures based on a hierarchy of prime numbers:
    *   Initial units are created in pairs, then triples, and further expanded using prime numbers (5, 7, 11, 17, 19).
    *   These units then form the basis for exponential growth, such as 2^2, 2^3, ..., up to 19^19, and further recursive structures like 2^2^2.
2.  **Conceptual Sampling:** At each stage, we conceptually "sample" data (e.g., characters, words, trees, or groups) according to these defined size structures. This forms our fundamental size structure.

**Implementation Notes:**
The underlying Rust codebase has undergone significant refactoring to support this grand plan with improved modularity and maintainability. Key architectural principles applied include:
*   **"One Declaration Per File"**: Each major struct, enum, or function is now typically defined in its own dedicated file, promoting clear separation of concerns and reusability.
*   **Standard Rust Features**: Custom procedural macros like `OurMacro` have been removed in favor of standard Rust language features and derive macros, enhancing code clarity and reducing reliance on custom tooling.
*   **Consistent Data Handling**: Improvements have been made to ensure consistent handling of data types, lifetimes, and error reporting across the codebase.