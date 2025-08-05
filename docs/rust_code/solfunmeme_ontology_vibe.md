# `solfunmeme_ontology_vibe` Crate Documentation

This document provides an overview of the `solfunmeme_ontology_vibe` Rust crate, detailing its purpose, structure, and how it interacts with other components of the `ragit` project, particularly `solfunmeme_rdf_utils` and `solfunmeme_clifford`.

## Overview

The `solfunmeme_ontology_vibe` crate serves as a high-level abstraction layer for managing and processing RDF (Resource Description Framework) ontologies within the Solfunmeme-Dioxus project. Its primary goal is to simplify interactions with the underlying `sophia` RDF library by providing a more user-friendly and domain-specific API. This encapsulation helps in maintaining modularity and reduces the impact of changes in the `sophia` library on the rest of the codebase.

## Dependencies

`solfunmeme_ontology_vibe` directly depends on the following crates:

*   **`solfunmeme_rdf_utils`**: This crate acts as a wrapper around the `sophia` RDF library, providing core functionalities for RDF graph manipulation, including loading from files, merging graphs, and serializing to Turtle format. `solfunmeme_ontology_vibe` extensively uses `solfunmeme_rdf_utils::rdf_graph::RdfGraph` for all its RDF operations.
*   **`solfunmeme_clifford`**: This crate is responsible for generating Clifford multivectors from string inputs. These multivectors are used to semantically enrich the ontology data, particularly for representing crate names and emojis.

## Core Modules and Functionality

The `solfunmeme_ontology_vibe` crate is organized into several modules, each responsible for a specific aspect of ontology management:

### `loader` Module

**Purpose**: Responsible for loading RDF graphs from Turtle (`.ttl`) files.

**Key Function**: `load_graph_internal()`

*   Loads `ontologies/index.ttl` and `ontologies/zos/v1.ttl`.
*   Merges the contents of `zos/v1.ttl` into the graph loaded from `index.ttl`.
*   Returns a combined `RdfGraph` instance.

**Usage Example (from `lib.rs`):**

```rust
pub fn load_graph() -> Result<RdfGraph> {
    loader::load_graph_internal()
}
```

### `processor` Module

**Purpose**: Responsible for processing the loaded RDF graph, specifically by adding semantic data (Clifford multivectors) to existing entities within the graph.

**Sub-modules**:

*   **`crate_data`**: Adds multivector representations for crate data.
    *   **Key Function**: `add_crate_data_internal(graph: &mut RdfGraph)`
    *   Identifies subjects in the graph that are of type `rdfs:Class` and have a `rdfs:label` property, particularly those prefixed with `crates_root`.
    *   Generates a Clifford multivector from the crate's label using `solfunmeme_clifford::generate_multivector_from_string`.
    *   Adds a new triple to the graph, linking the crate subject to its multivector representation using the `onto:hasCliffordVector` property.

*   **`emoji_data`**: Adds multivector representations for emoji data.
    *   **Key Function**: `add_emoji_data_internal(graph: &mut RdfGraph)`
    *   Identifies subjects in the graph that are of type `em:Emoji`.
    *   Generates a Clifford multivector from the emoji's name (extracted from its IRI) using `solfunmeme_clifford::generate_multivector_from_string`.
    *   Adds a new triple to the graph, linking the emoji subject to its multivector representation using the `onto:hasCliffordVector` property.

**Usage Examples (from `lib.rs`):**

```rust
pub fn add_crate_data(graph: &mut RdfGraph) -> Result<()> {
    processor::crate_data::add_crate_data_internal(graph)
}

pub fn add_emoji_data(graph: &mut RdfGraph) -> Result<()> {
    processor::emoji_data::add_emoji_data_internal(graph)
}
```

### `serializer` Module

**Purpose**: Responsible for serializing the processed RDF graph back into a Turtle (`.ttl`) file.

**Key Function**: `serialize_graph_internal(graph: &RdfGraph)`

*   Serializes the provided `RdfGraph` instance to `ontologies/index.ttl`.
*   Leverages `solfunmeme_rdf_utils::rdf_graph::RdfGraph::serialize_to_turtle` for the serialization process.

**Usage Example (from `lib.rs`):**

```rust
pub fn serialize_graph(graph: &RdfGraph) -> Result<()> {
    serializer::serialize_graph_internal(graph)
}
```

## Interaction with `solfunmeme_rdf_utils` and `solfunmeme_clifford`

`solfunmeme_ontology_vibe` acts as an orchestrator, utilizing the functionalities provided by its dependencies:

*   It relies on `solfunmeme_rdf_utils` for all low-level RDF operations, such as file I/O for Turtle files, graph merging, and adding triples. This ensures that the complexities of `sophia` are contained within `solfunmeme_rdf_utils`.
*   It uses `solfunmeme_clifford` to generate semantic representations (multivectors) for various entities (crates, emojis) within the ontology. These multivectors are then integrated into the RDF graph as literal triples.

This layered approach allows `solfunmeme_ontology_vibe` to focus on the domain-specific logic of ontology management, while delegating the generic RDF and Clifford algebra operations to its specialized dependencies.