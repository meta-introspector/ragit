use anyhow::Result;
use solfunmeme_embedding::candle_embedding::embed_text;
use solfunmeme_clifford::embedding_to_multivector;
// use solfunmeme_ontology_vibe::load_graph;

pub async fn dwim_command(input: String) -> Result<()> {
    // 1. Generate BERT embedding
    let embedding_vector = embed_text(&input)?;
    //    println!("Generated embedding vector: {:?}", embedding_vector);

    // 2. Convert embedding to a multivector
    let multivector = embedding_to_multivector(embedding_vector)?;
    println!("Generated multivector: {:?}", multivector);

    // 4. (Placeholder) Print the multivector
    // In the future, we will use this multivector to find similar items in the graph.
    println!("
Next step: Find items in the ontology with similar multivectors.");

    Ok(())
}