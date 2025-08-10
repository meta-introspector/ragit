Title: Feature Request: Ontology-Driven Embedding Training Data Generation

Description:
Currently, the `ragit-embedding-quiz` tool relies on manual identification of similar terms for training. To enable more systematic and semantically informed embedding refinement, we need a mechanism to leverage the project's existing RDF ontologies.

This feature requests a new tool or an enhancement to an existing tool that can:

1.  **Parse RDF/Turtle files**: Specifically, it should be able to read and parse `.ttl` files from the `ontologies/` directory.
2.  **Extract Terms and Relationships**: From the parsed RDF graph, it should extract:
    *   All `rdfs:label` values (representing terms).
    *   Relationships defined by `owl:ObjectProperty` (e.g., `num:hasMeaning`, `num:represents`, `agent:hasTask`).
    *   `owl:disjointWith` relationships between classes.
3.  **Map IRIs to String Terms**: It should be able to map RDF IRIs (e.g., `num:Prime2`) to their corresponding string terms (e.g., "0002") using `rdfs:label` or other conventions.
4.  **Generate Structured Output**: The tool should output this extracted information in a structured format (e.g., JSON) that can be consumed by `ragit-embedding-quiz` for training. This output could include:
    *   A list of all extracted terms.
    *   A list of positive pairs (terms that should be similar, derived from `rdfs:label` or `owl:ObjectProperty` relationships).
    *   A list of negative pairs (terms that should be dissimilar, derived from `owl:disjointWith` relationships).

Justification:
*   **Improved Embedding Quality**: By training with semantically informed relationships, the embedding model can learn more meaningful representations of terms.
*   **Automated Training Data Generation**: Reduces the manual effort required to create training data.
*   **Leverages Existing Knowledge**: Utilizes the rich semantic information already present in the project's ontologies.
*   **Foundation for Semantic Querying**: Provides the necessary data for future semantic query capabilities in `ragit-embedding-quiz`.

Proposed Implementation (High-Level):
*   A new Rust binary crate (e.g., `ragit-ontology-extractor`) or an extension to `ragit-embedding-quiz`.
*   Utilize `sophia` for RDF parsing.
*   Output to a JSON file (e.g., `ontology_training_data.json`).
