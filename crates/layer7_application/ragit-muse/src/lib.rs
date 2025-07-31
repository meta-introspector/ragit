use anyhow::Result;
// use ragit_prompt_management::prompts::PROMPTS;
// use ragit_pdl::Template;
// use tera::Context;

pub fn apply_muse_template(_muse_name: &str, _text: &str) -> Result<String, anyhow::Error> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    // let prompt_name = format!("muse_{}", muse_name.to_lowercase());
    // let pdl = PROMPTS
    //     .get(&prompt_name)
    //     .ok_or_else(|| anyhow::anyhow!("Prompt not found: {}", prompt_name.clone()))?;

    // let mut context = Context::new();
    // context.insert("text", &text);

    // let template = Template::new(None, pdl)?;
    // let processed = template.render(&context)?;
    // Ok(processed)
}