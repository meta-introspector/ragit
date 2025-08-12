use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use std::path::PathBuf;

pub fn load_graph_internal() -> Result<RdfGraph> {
    let numberology_ttl_path = PathBuf::from("ontologies/numberology.ttl");

    let graph = RdfGraph::from_file(&numberology_ttl_path)?;

    Ok(graph)
}
