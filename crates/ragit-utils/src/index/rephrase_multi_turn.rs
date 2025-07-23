use crate::error::Error;
use crate::index::index_struct::Index;
use ragit_api::Request;
use ragit_pdl::{parse_pdl, Pdl};
use serde_json::Value;

use crate::query::multi_turn_schema::MultiTurnSchema;

impl Index {
    pub async fn rephrase_multi_turn(&self, turns: Vec<String>) -> Result<MultiTurnSchema, Error> {
        let turns_json = Value::Array(
            turns
                .iter()
                .map(|turn| Value::String(turn.to_string()))
                .collect(),
        );
        let turns_json = serde_json::to_string_pretty(&turns_json)?;
        let mut tera_context = tera::Context::new();
        tera_context.insert("turns", &turns_json);

        let Pdl { messages, schema } = parse_pdl(
            &self.get_prompt("multi_turn")?,
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
                .create_pdl_path(&self.root_dir, "rephrase_multi_turn")
                .map(|p| p.to_str().unwrap().to_string()),
            dump_json_at: self
                .api_config
                .dump_log_at(&self.root_dir)
                .map(|p| p.to_str().unwrap().to_string()),
            model: self.get_model_by_name(&self.api_config.model)?,
            dump_api_usage_at: self
                .api_config
                .dump_api_usage_at(&self.root_dir, "rephrase_multi_turn"),
            schema,
            schema_max_try: 3,
        };

        Ok(request
            .send_and_validate::<MultiTurnSchema>(MultiTurnSchema::default())
            .await?)
    }
}
