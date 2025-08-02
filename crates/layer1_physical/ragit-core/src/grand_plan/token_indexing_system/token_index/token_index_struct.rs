use std::collections::HashMap;
use crate::grand_plan::llm_sampling_system::tokenizer::tokenizer_struct::Tokenizer;



#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// An inverted index for tokens, mapping tokens to the ideas they appear in.
pub struct TokenIndex {
    tokenizer: Tokenizer,
    /// Stores the original ideas, indexed by their IdeaId.
    ideas: HashMap<IdeaId, String>,
    /// The inverted index: maps a token to a list of IdeaIds where it appears.
    inverted_index: HashMap<String, Vec<IdeaId>>,
    next_idea_id: IdeaId,
}

impl TokenIndex {
    pub fn new(tokenizer: Tokenizer) -> Self {
        TokenIndex {
            tokenizer,
            ideas: HashMap::new(),
            inverted_index: HashMap::new(),
            next_idea_id: 0,
        }
    }

    /// Adds a new idea to the index, tokenizing it and updating the inverted index.
    pub fn add_idea(&mut self, idea_content: &str) -> IdeaId {
        let idea_id = self.next_idea_id;
        self.next_idea_id += 1;

        self.ideas.insert(idea_id, idea_content.to_string());

        // Tokenize the idea using prime-based character chunks
        let tokens = self.tokenizer.tokenize_by_char_chunks(idea_content);

        for token in tokens {
            self.inverted_index
                .entry(token)
                .or_insert_with(Vec::new)
                .push(idea_id);
        }
        idea_id
    }

    /// Finds all ideas that contain a specific token.
    pub fn find_ideas_with_token(&self, token: &str) -> Vec<&String> {
        let mut found_ideas = Vec::new();
        if let Some(idea_ids) = self.inverted_index.get(token) {
            for &id in idea_ids {
                if let Some(idea_content) = self.ideas.get(&id) {
                    found_ideas.push(idea_content);
                }
            }
        }
        found_ideas
    }

    /// Returns the count of occurrences for each token.
    pub fn get_token_counts(&self) -> HashMap<&String, usize> {
        let mut counts = HashMap::new();
        for (token, idea_ids) in &self.inverted_index {
            counts.insert(token, idea_ids.len());
        }
        counts
    }
}
