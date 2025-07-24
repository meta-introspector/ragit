use crate::prelude::*;

impl Index {
    pub fn select_muse(&self, index: usize) -> MuseName {
        let muses: Vec<MuseName> = MuseName::iter().collect();
        muses[index]
    }

    pub fn apply_muse_template(&self, muse_name: &str, text: &str) -> Result<String, ApiError> {
        let prompt_name = format!("muse_{}", muse_name);
        let pdl = self
            .get_prompt(&prompt_name)
            .ok_or_else(|| ApiError::PromptMissing(prompt_name.clone()))?;

        let mut context = Context::new();
        context.insert("text", text);

        let Pdl { messages, .. } = parse_pdl(pdl, &context, ".", true)?;

        // TODO: this is a hack. We should have a proper way to render messages.
        Ok(messages
            .iter()
            .map(|m| match m.content[0] {
                MessageContent::String(s) => s,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("\n"))
    }
}