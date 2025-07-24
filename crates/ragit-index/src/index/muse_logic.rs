use crate::prelude::*;

impl Index {
    pub fn select_muse(&self, index: usize) -> MuseName {
        let muses: Vec<MuseName> = MuseName::iter().collect();
        muses[index % muses.len()].clone()
    }

    pub fn apply_muse_template(&self, muse_name: &str, text: &str) -> Result<String, Error> {
        let prompt_name = format!("muse_{}", muse_name.to_lowercase());
        let pdl = self
            .prompts
            .get(&prompt_name)
            .ok_or_else(|| Error::PromptMissing(prompt_name.clone()))?;

        let mut context = Context::new();
        context.insert("text", &text);

        let Pdl { messages, .. } = parse_pdl(pdl, &context, ".", true)?;

        let processed = messages
            .into_iter()
            .map(|msg| {
                msg.content
                    .into_iter()
                    .map(|content| {
                        match content {
                            ragit_pdl::MessageContent::String(s) => s,
                            _ => String::new(), // Handle other content types as needed, or error
                        }
                    })
                    .collect::<String>()
            })
            .collect::<String>();
        Ok(processed)
    }
}
