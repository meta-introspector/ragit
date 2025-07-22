use crate::chunk::{Chunk, merge_and_convert_chunks, rendered_chunk::RenderedChunk};
use crate::error::Error;
use crate::index::index_struct::Index;
use ragit_api::Request;
use ragit_pdl::{Pdl, Schema, parse_pdl, render_pdl_schema};
use serde_json::Value;

impl Index {
    pub async fn answer_query_with_chunks(
        &self,
        query: &str,
        chunks: Vec<Chunk>,
        schema: Option<Schema>,
    ) -> Result<String, Error> {
        let rendered_chunks: Vec<RenderedChunk> = chunks.into_iter().map(|c| c.render(self)).collect::<Result<Vec<RenderedChunk>, Error>>()?;
        let chunks = merge_and_convert_chunks(self, rendered_chunks)?;
        let mut tera_context = tera::Context::new();
        tera_context.insert(
            "chunks",
            &chunks,
        );
        tera_context.insert(
            "query",
            &query,
        );

        let Pdl { messages, .. } = parse_pdl(
            &self.get_prompt("answer_query")?,
            &tera_context,
            "/",  // TODO: `<|media|>` is not supported for this prompt
            true,
        )?;

        let request = Request {
            messages,
            schema: schema.clone(),
            timeout: self.api_config.timeout,
            max_retry: self.api_config.max_retry,
            sleep_between_retries: self.api_config.sleep_between_retries,
            dump_pdl_at: self.api_config.create_pdl_path(&self.root_dir, "answer_query_with_chunks").map(|p| p.to_str().unwrap().to_string()),
            dump_json_at: self.api_config.dump_log_at(&self.root_dir).map(|p| p.to_str().unwrap().to_string()),
            model: self.get_model_by_name(&self.api_config.model)?,
            dump_api_usage_at: self.api_config.dump_api_usage_at(&self.root_dir, "answer_query_with_chunks"),
            schema_max_try: 3,
            ..Request::default()
        };

        let response = match schema {
            Some(schema) => {
                let result = request.send_and_validate::<Value>(Value::Null).await?;
                render_pdl_schema(&schema, &result)?
            },
            None => request.send().await?.get_message(0).unwrap().to_string(),
        };

        Ok(response)
    }
}
