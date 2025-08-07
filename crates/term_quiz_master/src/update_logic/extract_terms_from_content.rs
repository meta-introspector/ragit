use std::collections::HashSet;

pub fn extract_terms_from_content(content: &str, stopwords: &HashSet<String>) -> Vec<String> {
    // This is a very basic placeholder. In a real scenario, this would involve
    // more sophisticated parsing, NLP, or regex to extract meaningful terms.
    content.split_whitespace()
           .filter(|s| s.len() > 3 && s.chars().all(char::is_alphanumeric) && !stopwords.contains(&s.to_lowercase()))
           .map(|s| s.to_lowercase())
           .collect()
}
