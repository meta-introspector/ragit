use super::super::constants::SELF_IMPROVEMENT_PROMPT_PREFIX;

pub fn format_prompt(self_code: &str) -> String {
    format!(
        "{}",
        SELF_IMPROVEMENT_PROMPT_PREFIX.replace("{}", self_code)
    )
}