#![cfg(test)]

use super::{
    DwimVibeOntology,
    VibeEntry,
};

#[test]
fn test_new_dwim_vibe_ontology() {
    let ontology = DwimVibeOntology::new();
    assert!(ontology.entries.is_empty());
    assert!(ontology.emoji_to_entry.is_empty());
}

#[test]
fn test_add_entry() {
    let mut ontology = DwimVibeOntology::new();
    let entry1 = VibeEntry {
        emoji: "🧠".to_string(),
        semantic_name: "Mind".to_string(),
        numerical_address: 1,
        layer: 1,
    };
    let entry2 = VibeEntry {
        emoji: "💡".to_string(),
        semantic_name: "Insight".to_string(),
        numerical_address: 2,
        layer: 1,
    };

    ontology.add_entry(entry1.clone());
    ontology.add_entry(entry2.clone());

    assert_eq!(ontology.entries.len(), 2);
    assert_eq!(ontology.emoji_to_entry.len(), 2);
    assert_eq!(ontology.get_by_semantic_name("Mind"), Some(&entry1));
    assert_eq!(ontology.get_by_emoji("💡"), Some(&entry2));
}

#[test]
fn test_get_non_existent_entry() {
    let ontology = DwimVibeOntology::new();
    assert_eq!(ontology.get_by_semantic_name("NonExistent"), None);
    assert_eq!(ontology.get_by_emoji("❓"), None);
}
