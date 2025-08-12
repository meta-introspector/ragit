# Sophia RDF Library Integration SOP

This document outlines the standard operating procedures for integrating and utilizing the `sophia` RDF library within the `ragit` project.

## 1. High-Level Strategy

The core strategy for using `sophia` is to abstract its complexities behind a dedicated wrapper crate: `solfunmeme_rdf_utils`. This approach provides a simplified, high-level API for common RDF operations, reducing boilerplate code and isolating the rest of the codebase from direct dependencies on `sophia`. This is crucial for managing the complexity of the `sophia` API and for mitigating the impact of breaking changes in future versions.

## 2. Core Functionality

`sophia` is primarily used for the following RDF-related tasks:

*   **Parsing:** Parsing Turtle (`.ttl`) and other RDF formats into in-memory graph structures.
*   **Querying:** Programmatically querying the RDF graphs to retrieve semantic data.
*   **Serialization:** Serializing in-memory graphs back into Turtle (`.ttl`) files.
*   **Graph Manipulation:** Creating, updating, and managing RDF graphs.

## 3. Key Crates and Modules

The following crates and modules are central to the `sophia` integration:

*   **`solfunmeme_rdf_utils`:** The primary wrapper crate for `sophia`. All new code that needs to interact with RDF data should use the utilities provided by this crate.
*   **`solfunmeme_ontology_vibe`:** A new crate designed to encapsulate and manage interactions with the `sophia` RDF library, providing a simplified interface and improving modularity.
*   **`solfunmeme_core_logic`:** Directly uses `sophia` for RDF graph manipulation.
*   **`prepare_sources`:** The `ontology_generator` module within this crate uses `sophia` to create RDF ontologies from analyzed function data.
*   **`task_manager`:** Uses `sophia` for parsing, querying, and updating the task graph.
*   **`jsonld_plugin`:** Uses `sophia_jsonld` for JSON-LD processing.

## 4. Sophia 0.8 Migration

The migration from `sophia` version 0.7 to 0.8 introduced significant breaking changes due to the adoption of Generic Associated Types (GATs). Key points to remember when working with `sophia` 0.8.0+ are:

*   **Term and IRI Types:** The `TTerm` trait has been replaced by the new `Term` trait. IRI-related types are now primarily located in the `sophia_iri` crate.
*   **Graph Methods:** Methods like `triples_with_s` have been replaced by the more general `triples_matching` method.
*   **Serializer Traits:** When using serializers like `TurtleSerializer`, ensure that the `sophia_api::prefix::PrefixSink` and `sophia_api::serializer::StreamSerializer` traits are in scope.
*   **Owned vs. Borrowed Types:** Be mindful of Rust's ownership and borrowing rules, especially when converting between `&str` and `String` for `Term` types.

## 5. Code Examples

### Reading a Turtle file

```rust
use sophia::api::prelude::*;
use sophia::inmem::graph::LightGraph;
use sophia::turtle::parser::turtle;
use std::fs::File;
use std::io::BufReader;

fn read_turtle_file(file_path: &str) -> Result<LightGraph, Box<dyn std::error::Error>> {
    let f = BufReader::new(File::open(file_path)?);
    let mut g = LightGraph::new();
    turtle::parse_bufread(f).to_graph(&mut g)?;
    Ok(g)
}
```

### Writing to a Turtle file

```rust
use sophia::api::prelude::*;
use sophia::inmem::graph::LightGraph;
use sophia_turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};
use std::fs::File;

fn write_turtle_file(graph: &LightGraph, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path)?;
    let mut serializer = TurtleSerializer::new_with_config(
        file,
        TurtleConfig::new()
            .with_pretty(true)
    );
    serializer.serialize_graph(graph)?;
    Ok(())
}
```

## 6. Conclusion

By adhering to these procedures, we can ensure a consistent and maintainable integration of the `sophia` RDF library, enabling powerful semantic data capabilities within the `ragit` project.
