use std::collections::HashMap;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use sophia_api::prelude::Term;

pub fn extract_emoji_data(graph: &mut RdfGraph) -> (HashMap<String, (String, String)>, HashMap<String, String>) {
    let mut emoji_data: HashMap<String, (String, String)> = HashMap::new(); // emoji_char -> (concept_id, category)
    let mut concept_descriptions: HashMap<String, String> = HashMap::new(); // concept_id -> description

    graph.namespaces.add_namespace("em", "http://example.org/emoji#").unwrap();

    let emoji_prop = "em:emoji";
    let category_prop = "em:category";
    let description_prop = "em:description";
    let type_prop = "rdf:type";
    let emoji_class = "em:Emoji";

    let subjects = graph.get_subjects_with_property(type_prop, emoji_class).unwrap();

    for subject in subjects {
        let concept_id = subject.clone();
        if let Some(emoji_char) = graph.get_property_value(&subject, emoji_prop).unwrap() {
            let category = graph.get_property_value(&subject, category_prop).unwrap().unwrap_or_else(|| "Unknown".to_string());
            emoji_data.insert(emoji_char, (concept_id.clone(), category));
        }
        if let Some(description) = graph.get_property_value(&subject, description_prop).unwrap() {
            concept_descriptions.insert(concept_id, description);
        }
    }
    (emoji_data, concept_descriptions)
}