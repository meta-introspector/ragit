#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Generates a PDL document programmatically with customizable sections.
pub fn generate_pdl_document(
    system_role: &str,
    system_task: &str,
    user_prompt: &str,
    assistant_response: &str,
) -> String {
    format!(
        "[SYSTEM]\nRole: {}\nTask: {}\n\n[USER]\n{}\n\n[ASSISTANT]\n{}",
        system_role,
        system_task,
        user_prompt,
        assistant_response
    )
}

