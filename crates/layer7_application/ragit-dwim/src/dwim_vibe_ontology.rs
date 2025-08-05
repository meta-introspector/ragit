use std::collections::HashMap;

pub mod tests;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct VibeEntry {
    pub emoji: String,
    pub semantic_name: String,
    pub numerical_address: u64, // Or a more complex vector type later
    pub layer: u8,
}

#[derive(Debug, Default)]
pub struct DwimVibeOntology {
    // Map from semantic name to VibeEntry
    pub entries: HashMap<String, VibeEntry>,
    // Maybe other mappings, e.g., emoji to VibeEntry
    pub emoji_to_entry: HashMap<String, VibeEntry>,
}

impl DwimVibeOntology {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: VibeEntry) {
        self.entries.insert(entry.semantic_name.clone(), entry.clone());
        self.emoji_to_entry.insert(entry.emoji.clone(), entry);
    }

    pub fn get_by_semantic_name(&self, name: &str) -> Option<&VibeEntry> {
        self.entries.get(name)
    }

    pub fn get_by_emoji(&self, emoji: &str) -> Option<&VibeEntry> {
        self.emoji_to_entry.get(emoji)
    }
}
