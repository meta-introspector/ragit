use crate::prelude::*;

pub async fn load_chunks_or_tfidf(
    index: &Index,
    query: &str,
    limit: usize,
) -> Result<Vec<Chunk>, ApiError> {
    // Assuming `chunk_count` is now a method or accessed differently
    // For now, let's assume it's a method `index.get_chunk_count()`
    // You'll need to implement `get_chunk_count` in `Index` if it's not there.
    // if index.get_chunk_count() > limit {
    //     let keywords = index.extract_keywords(query).await?;
    //     let tfidf_results = index.run_tfidf(
    //         keywords,
    //         limit,
    //     )?;
    //     let mut chunks = Vec::with_capacity(tfidf_results.len());

    //     for tfidf_result in tfidf_results.into_iter() {
    //         let uid = tfidf_result.doc_id; // Assuming `id` changed to `doc_id`
    //         chunks.push(index.get_chunk_by_uid(uid)?);
    //     }

    //     Ok(chunks)
    // }

    // else {
        let mut chunks = vec![];

        for chunk_path in index.get_all_chunk_files()?.into_iter() { // Added .into_iter()
            chunks.push(helpers::load_chunk_from_path(&chunk_path)?);
        }

        Ok(chunks)
    // }
}
