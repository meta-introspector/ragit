use std::collections::HashMap;
use crate::index_struct::Index;

pub fn count_words_in_chunks(index: &Index) -> HashMap<String, usize> {
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for chunk in &index.chunks {
        let text = &chunk.data;
        for word in text.as_str().split_whitespace() {
            *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }

    word_counts
}
