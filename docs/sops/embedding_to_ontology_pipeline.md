# SOP: Embedding-to-Ontology Pipeline

## 1. Objective
To provide a clear, step-by-step process for adding new terms (e.g., emojis, concepts) to the project's ontology with a semantically meaningful vector representation derived from a BERT model.

## 2. Overview
This pipeline leverages several specialized crates to transform a piece of text into a high-dimensional embedding, convert that embedding into a Clifford algebra multivector, and finally integrate it into the main RDF ontology.

## 3. Core Components
- **Embedding Provider (`solfunmeme_embedding`)**: Generates a vector embedding from text using a pre-trained BERT model via the `candle` ML framework.
- **Multivector Algebra (`solfunmeme_clifford`)**: Converts the numerical embedding into a `SolMultivector`, a representation within a 3D Clifford algebra.
- **Ontology Management (`solfunmeme_ontology_vibe`)**: Handles the loading, modification, and serialization of the RDF graph (`.ttl` files).

## 4. Procedure

### Step 1: Generate the Text Embedding
- **Action:** Call the `solfunmeme_embedding::candle_embedding::embed_text()` function.
- **Input:** The text string for the new term you want to add.
- **Output:** A high-dimensional vector (`Vec<f32>`).
- **Note:** This requires the `with-candle` feature to be enabled at compile time, which will download the necessary model files from the Hugging Face Hub on first run.

```rust
// Conceptual code
// Ensure the `with-candle` feature is enabled for `solfunmeme_embedding`

use solfunmeme_embedding::candle_embedding::embed_text;

let new_term_text = "My New Awesome Concept";
let embedding_vector = embed_text(new_term_text)?;
```

### Step 2: Convert Embedding to a Multivector
- **Action:** Pass the embedding vector to a conversion function within `solfunmeme_clifford`.
- **Input:** The `Vec<f32>` from the previous step.
- **Output:** A `SolMultivector` instance.
- **Note:** This step translates the raw numerical embedding into the project's specific algebraic, semantic representation.

```rust
// Conceptual code
use solfunmeme_clifford::embedding_to_multivector; // Assuming this function exists

let multivector = embedding_to_multivector(embedding_vector)?;
```

### Step 3: Add the New Term to the Ontology Graph
- **Action:** Use the functions from `solfunmeme_ontology_vibe` to modify the ontology.
- **Process:**
    1.  Load the existing ontology graph using `solfunmeme_ontology_vibe::load_graph()`.
    2.  Define the new term's IRI (its unique identifier).
    3.  Create a new triple: `(<new_term_iri>, <onto:hasCliffordVector>, <multivector_literal>)`.
    4.  Add this new triple to the in-memory graph.

```rust
// Conceptual code
use solfunmeme_ontology_vibe::load_graph;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph; // For direct manipulation
use sophia_api::term::{SimpleTerm, Term};
use sophia_api::iri::Iri;

let mut graph = load_graph()?;
let new_term_iri = Iri::new("http://example.org/MyNewAwesomeConcept")?;

// This would be a helper function in solfunmeme_ontology_vibe or similar
add_multivector_triple(&mut graph, &new_term_iri, &multivector)?;
```

### Step 4: Serialize the Updated Graph
- **Action:** Call `solfunmeme_ontology_vibe::serialize_graph()`.
- **Input:** The modified `RdfGraph` instance.
- **Output:** The `ontologies/index.ttl` file is overwritten with the updated data.
- **Note:** Always ensure you have a backup of your ontology files before running a process that modifies them.

```rust
// Conceptual code
use solfunmeme_ontology_vibe::serialize_graph;

serialize_graph(&graph)?;
```
