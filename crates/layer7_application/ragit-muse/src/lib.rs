use ragit_utils::error::Error;
use ragit_utils::prompts::PROMPTS;
use ragit_pdl::{Process, Template};
use tera::Context;

pub fn apply_muse_template(muse_name: &str, text: &str) -> Result<String, Error> {
    let prompt_name = format!("muse_{}", muse_name.to_lowercase());
    let pdl = PROMPTS
        .get(&prompt_name)
        .ok_or_else(|| Error::PromptNotFound(prompt_name.clone()))?;

    let mut context = Context::new();
    context.insert("text", &text);

    let template = Template::new(pdl)?;
    let processed = template.render(&mut context)?;
    Ok(processed)
}