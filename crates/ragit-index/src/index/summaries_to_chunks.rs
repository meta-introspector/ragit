use crate::prelude::*;

impl Index {
    pub async fn summaries_to_chunks(
        &self,
        query: &str,
        chunks: Vec<Chunk>,
        max_retrieval: usize,
    ) -> Result<Vec<Chunk>, Error> {
        let mut tera_context = tera::Context::new();
        tera_context.insert(
            "entries",
            &chunks
                .iter()
                .enumerate()
                .map(|(index, chunk)| {
                    format!(
                        "{}. {}\nsource: {}\nsummary: {}",
                        index + 1,
                        chunk.title,
                        chunk.render_source(),
                        chunk.summary,
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
        tera_context.insert("query", &query);
        tera_context.insert("max_retrieval", &max_retrieval);
        tera_context.insert("max_index", &chunks.len());

        let Pdl { messages, schema } = parse_pdl(
            &self.get_prompt("rerank_summary")?,
            &tera_context,
            "/", // TODO: `<|media|>` is not supported for this prompt
            true,
        )?;
        let request = Request {
            messages,
            frequency_penalty: None,
            max_tokens: None,
            temperature: None,
            timeout: self.api_config.timeout,
            max_retry: self.api_config.max_retry,
            sleep_between_retries: self.api_config.sleep_between_retries,
            dump_pdl_at: self
                .api_config
                .create_pdl_path(&self.root_dir, "rerank_summary")
                .map(|p| p.to_str().unwrap().to_string()),
            dump_json_at: self
                .api_config
                .dump_log_at(&self.root_dir)
                .map(|p| p.to_str().unwrap().to_string()),
            model: self.get_model_by_name(&self.api_config.model)?,
            dump_api_usage_at: self
                .api_config
                .dump_api_usage_at(&self.root_dir, "rerank_summary"),
            schema,
            schema_max_try: 3,
        };
        let chunk_indices = request.send_and_validate::<Vec<usize>>(vec![]).await?;

        Ok(chunks
            .into_iter()
            .enumerate()
            .filter(|(index, _)| chunk_indices.contains(&(index + 1)))
            .map(|(_, chunk)| chunk)
            .collect())
    }
}
