use crate::error::Error;
use crate::index::index_struct::Index;

pub async fn query(
    index: &Index,
    query: String,
    json: bool,
) -> Result<(), Error> {
    let response = index.query(&query, vec![], None).await?;

    if json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    }

    else {
        println!("{}", response.response);

        if !response.retrieved_chunks.is_empty() {
            for chunk in response.retrieved_chunks.iter() {
                println!("\n---");
                println!("file: {}", chunk.render_source());
                println!("chunk: {}", chunk.data);
            }
        }
    }

    Ok(())
}
