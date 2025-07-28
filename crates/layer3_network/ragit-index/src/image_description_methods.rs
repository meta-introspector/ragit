use ragit_utils::constant::{CHUNK_DIR_NAME, IMAGE_DIR_NAME};
use ragit_utils::prelude::*;
use ragit_fs::{read_bytes, read_string, remove_file, try_create_dir, write_bytes, WriteMode};
use ragit_pdl::{encode_base64, parse_pdl, Pdl};
use ragit_uid::Uid;
use serde_json::Value;
use std::path::Path;

use ragit_readers::ImageDescription;
use ragit_utils::ragit_path_utils::get_uid_path;
use ragit_api::Request;
use ragit_fs::parent;

impl super::Index {
    pub async fn add_image_description(&self, uid: Uid) -> Result<(), Error> {
        let description_path =
            get_uid_path(&self.root_dir, Path::new(CHUNK_DIR_NAME), uid, Some("json"))?;
        let image_path = get_uid_path(&self.root_dir, Path::new(IMAGE_DIR_NAME), uid, Some("png"))?;
        let parent_path = parent(image_path.as_path())?;

        if !parent_path.exists() {
            try_create_dir(parent_path.to_str().unwrap())?;
        }

        let image_bytes = read_bytes(image_path.to_str().unwrap())?;
        let image_bytes = encode_base64(&image_bytes);

        if let Ok(j) = read_string(description_path.to_str().unwrap()) {
            if serde_json::from_str::<Value>(&j).is_ok() {
                return Ok(());
            }
            else {
                remove_file(description_path.to_str().unwrap())?;
            }
        }

        let mut context = tera::Context::new();
        context.insert("image_type", "png");
        context.insert("image_bytes", &image_bytes);
        let pdl = self.get_prompt("describe_image")?;
        let Pdl { messages, schema } = parse_pdl(
            &pdl, &context, "/", // TODO: `<|media|>` is not supported for this prompt
            true,
        )?;

        let request = Request {
            messages,
            model: self.get_model_by_name(&self.api_config.model)?,
            frequency_penalty: None,
            max_tokens: None,
            max_retry: self.api_config.max_retry,
            sleep_between_retries: self.api_config.sleep_between_retries,
            timeout: self.api_config.timeout,
            temperature: None,
            dump_api_usage_at: self
                .api_config
                .dump_api_usage_at(&self.root_dir, "describe_image"),
            dump_pdl_at: self
                .api_config
                .create_pdl_path(&self.root_dir, "describe_image")
                .map(|p| p.to_str().unwrap().to_string()),
            dump_json_at: self
                .api_config
                .dump_log_at(&self.root_dir)
                .map(|p| p.to_str().unwrap().to_string()),
            schema,
            schema_max_try: 3,
        };
        let result = request
            .send_and_validate::<ImageDescription>(ImageDescription::default())
            .await?;

        write_bytes(
            description_path.to_str().unwrap(),
            &serde_json::to_vec_pretty(&result)?,
            WriteMode::CreateOrTruncate,
        )?;

        Ok(())
    }
}
