# Contributing to Ragit

We welcome contributions to the Ragit project! By joining us, you'll be part of building a collaborative neural network for semantic code understanding.

## Getting Started

1.  **Clone the Repository**: `git clone https://github.com/meta-introspector/ragit.git`
2.  **Navigate to the Project Directory**: `cd ragit`
3.  **Install Rust**: If you don't have Rust installed, follow the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).
4.  **Build the Project**: `cargo build`
5.  **Run Tests**: `cargo test`

## Branching Strategy

We use a feature-branch workflow:
*   **Feature Branches**: For new features, create a branch named `feature/<your-feature-name>`.
*   **Bugfix Branches**: For bug fixes, create a branch named `bugfix/<issue-id>-<short-description>`.
*   **Submodule Branches**: If working within a submodule, create a branch within the submodule's directory (e.g., `cd vendor/some-submodule && git checkout -b <submodule-name>-<your-branch-name>`).

## Code Style

We adhere to standard Rust formatting and linting practices:
*   **Formatting**: Run `cargo fmt` before committing.
*   **Linting**: Run `cargo clippy` and address any warnings.

## Commit Message Guidelines

*   Use clear and concise commit messages.
*   Start with a type (e.g., `feat:`, `fix:`, `docs:`, `refactor:`).
*   Follow with a brief, imperative description of the change.
*   (Optional) Provide a more detailed body if necessary.

## How You Can Contribute

### 1. Embedding Quizzing and Refinement

Help us improve the semantic understanding of code terms:
*   **Run the Quiz**: Use `cargo run --package ragit-embedding-quiz quiz` to get a term and its current embedding.
*   **Refine Embeddings**: Based on the displayed similar terms, use `cargo run --package ragit-embedding-quiz answer <question_id> <new_embedding_values>` to adjust the embedding. Your intuition helps train our model!
*   **Suggest Missing Terms**: Run `cargo run --package ragit-embedding-quiz suggest-terms` and provide embeddings for any new terms identified.

### 2. Training Data Generation

Contribute to our training data by identifying more semantic relationships:
*   **Manual Review**: Review terms in `term_embeddings.json` and identify pairs that should be semantically similar or dissimilar.
*   **Create Training Pairs**: Add new entries to `docs/training_data.json` (or `training_data_vX.json` for new versions).

### 3. Ontology Integration

Help us build a richer semantic graph:
*   **Parse TTL Files**: Develop tools or scripts to extract terms and relationships from our `.ttl` ontologies.
*   **Relationship-Based Training**: Implement logic to use ontological relationships (e.g., `owl:disjointWith`, `rdfs:subClassOf`) to automatically generate training data for embedding adjustments.

### 4. Tool Development

Improve our internal tools:
*   **`ragit-embedding-quiz`**: Enhance its UI, add new features, or optimize its performance.
*   **New Tools**: Propose and develop new small tools that assist with any aspect of Ragit's development or analysis.

### 5. Documentation

Clear documentation is key to collaboration:
*   Improve existing SOPs and create new ones.
*   Expand the glossary of terms.
*   Write tutorials and guides for new features.

## Suggesting New Dimensions (Advanced Contribution)

We are building a collaborative neural network where users can suggest entirely new dimensions of semantic understanding. If you have an idea for a new dimension:

1.  **Create a Change Request**: Submit a detailed change request outlining your proposal, its justification, and potential impact.
2.  **Provide Migration Scripts**: Include scripts that can generate or transform data to populate your new dimension.
3.  **Proof of Value**: Demonstrate how your proposed dimension adds value to Ragit's semantic understanding, ideally with some initial experimental results or use cases.

## Getting Help

*   **Discord**: Join our Discord community [Discord Invite Link - Placeholder] for real-time chat and support.
*   **GitHub Issues**: Open an issue on GitHub for bugs, feature requests, or questions.

Thank you for contributing to Ragit! Together, we can build the future of code intelligence.
