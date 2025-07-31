// use ragit_index_types::Index;
// use ragit_types::ApiError;
// use ragit_muse::MuseName;
// use anyhow::Result;
// use tera::Context;
// use ragit_pdl::{parse_pdl, Pdl, MessageContent};
pub fn select_muse(_index: usize) -> MuseName {    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");}
pub fn apply_muse_template(_index: &Index, _muse_name: &str, _text: &str) -> Result<String, ApiError> {    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    // let prompt_name = format!("muse_{}", muse_name);
    // let pdl = index
    //     .get_prompt(&prompt_name)
    //     .ok_or_else(|| ApiError::PromptNotFound(prompt_name.clone()))?;

    // let mut context = Context::new();
    // context.insert("text", text);

    // let Pdl { messages, .. } = parse_pdl(pdl, &context, ".", true)?;
    // // TODO: this is a hack. We should have a proper way to render messages.
    // Ok(messages
    //     .iter()
    //     .map(|m| match m.content[0] {
    //         MessageContent::String(s) => s,
    //         _ => unreachable!(),
    //     })
    //     .collect::<Vec<_>>()
    //     .join("\n"))
}