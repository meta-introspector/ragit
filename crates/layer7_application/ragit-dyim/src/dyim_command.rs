use anyhow::Result;

pub async fn dyim_command(input: String) -> Result<()> {
    let input_lower = input.to_lowercase();
    let mut generated_prompt = String::new();

    if input_lower.contains("sdlc") {
        generated_prompt.push_str("Generate a comprehensive overview of the Software Development Life Cycle (SDLC), including its phases and best practices. ");
    }
    if input_lower.contains("iso9k") || input_lower.contains("iso 9000") || input_lower.contains("iso 9001") {
        generated_prompt.push_str("Explain the ISO 9000 series of quality management standards, focusing on ISO 9001 certification requirements and benefits. ");
    }
    if input_lower.contains("gmp") {
        generated_prompt.push_str("Describe Good Manufacturing Practices (GMP) and their importance in regulated industries. ");
    }
    if input_lower.contains("itil") {
        generated_prompt.push_str("Provide an introduction to IT Infrastructure Library (ITIL), outlining its key processes and how it supports IT service management. ");
    }
    if input_lower.contains("bootstrap ragit") {
        generated_prompt.push_str("Provide instructions on how to bootstrap the ragit project, including any necessary setup steps and initial command execution. ");
    }
    if input_lower.contains("reduce duplicate code") || input_lower.contains("deduplicate code") {
        generated_prompt.push_str("Outline a strategy for identifying and reducing duplicate code within the project, suggesting tools or refactoring techniques. ");
    }
    if input_lower.contains("update gemini memory with all sops") {
        generated_prompt.push_str("Update Gemini's memory with all available Standard Operating Procedures (SOPs) from the project documentation. ");
    }
    if input_lower.contains("find top 10 duplicate blocks of ragit with ragit using ragit to improve ragit") {
        generated_prompt.push_str("Utilize ragit's capabilities to identify and list the top 10 most duplicated code blocks within the ragit project, with the goal of improving code quality and reducing redundancy. ");
    }

    if generated_prompt.is_empty() {
        println!("No specific prompt generated for '{}'. Please provide keywords like 'sdlc', 'iso9k', 'gmp', 'itil', 'bootstrap ragit', 'reduce duplicate code', 'update gemini memory with all sops', or 'find top 10 duplicate blocks of ragit with ragit using ragit to improve ragit'.", input);
    } else {
        println!("Generated Prompt: {}", generated_prompt.trim());
    }

    Ok(())
}
