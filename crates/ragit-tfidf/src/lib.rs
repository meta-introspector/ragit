use ragit_types::ApiError;
use ragit_fs::{read_bytes, write_bytes, WriteMode};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//use std::hash::Hash;
use rust_stemmers::{Stemmer, Algorithm};
use ragit_types::uid::Uid;
use ragit_utils::query::Keywords;
use anyhow::Result;

pub mod processed_doc;
pub use processed_doc::ProcessedDoc;
pub mod io;
pub use io::{load_from_file, save_to_file};





#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TfidfResult {
    pub doc_id: Uid,
    pub score: f64,
}

pub struct TfidfState {
    // doc_id -> tokens
    docs: HashMap<Uid, Vec<String>>,

    // token -> doc_id -> count
    term_freq: HashMap<String, HashMap<Uid, usize>>,

    // token -> count
    doc_freq: HashMap<String, usize>,

    // total number of documents
    doc_count: usize,
}

impl TfidfState {
    pub fn new(_keywords: &Keywords) -> Self {
        TfidfState {
            docs: HashMap::new(),
            term_freq: HashMap::new(),
            doc_freq: HashMap::new(),
            doc_count: 0,
        }
    }

    pub fn add_doc(&mut self, doc_id: Uid, tokens: Vec<String>) {
        self.doc_count += 1;
        self.docs.insert(doc_id.clone(), tokens.clone());

        for token in tokens {
            *self.term_freq.entry(token.clone()).or_default().entry(doc_id.clone()).or_default() += 1;
            *self.doc_freq.entry(token).or_default() += 1;
        }
    }

    pub fn tf(&self, token: &str, doc_id: &Uid) -> f64 {
        let count = *self.term_freq.get(token).and_then(|doc_map| doc_map.get(doc_id)).unwrap_or(&0);
        let total = self.docs.get(doc_id).map_or(0, |tokens| tokens.len());
        count as f64 / total as f64
    }

    pub fn idf(&self, token: &str) -> f64 {
        let count = *self.doc_freq.get(token).unwrap_or(&0);
        ((self.doc_count as f64) / (count as f64 + 1.0)).ln()
    }

    pub fn tfidf(&self, token: &str, doc_id: &Uid) -> f64 {
        self.tf(token, doc_id) * self.idf(token)
    }

    pub fn search(&self, keywords: &Keywords) -> Vec<TfidfResult> {
        let mut scores: HashMap<Uid, f64> = HashMap::new();

        for doc_id in self.docs.keys() {
            for token in keywords.0.iter() {
                *scores.entry(doc_id.clone()).or_default() += self.tfidf(token, doc_id);
            }
        }

        let mut results: Vec<TfidfResult> = scores
            .into_iter()
            .map(|(doc_id, score)| TfidfResult { doc_id, score })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
    let stemmer = Stemmer::create(Algorithm::English);
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| stemmer.stem(s).to_string())
        .collect()
}

pub fn consume_processed_doc(
    processed_doc: ProcessedDoc,
    tfidf_state: &mut TfidfState,
) -> Result<(), ApiError> {
    tfidf_state.add_doc(processed_doc.doc_id, processed_doc.tokens);
    Ok(())
}
