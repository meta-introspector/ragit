use sha2::{Sha256, Digest};

pub fn create_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn estimate_token_count(text: &str) -> usize {
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    (word_count + char_count / 4) / 2
}
