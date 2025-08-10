use clap::{Parser, Subcommand};

pub fn parse_embedding(s: &str) -> Result<Vec<f32>, String> {
    s.split(',')
        .map(|s| s.trim().parse::<f32>().map_err(|e| format!("Invalid float: {}", e)))
        .collect()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get a quiz question
    Quiz,
    /// Submit an answer to a quiz question
    Answer {
        question_id: usize,
        submitted_embedding_str: String,
    },
    /// Suggest new terms that might be missing embeddings
    SuggestTerms,
    /// Train embeddings based on a training data file
    Train {
        training_data_path: String,
        learning_rate: Option<f32>,
    },
    /// Query for terms based on a list of input terms
    Query {
        #[arg(num_args = 1..)]
        terms: Vec<String>,
    },
    /// Add a new term and its embedding
    AddVector {
        term: String,
        embedding_str: String,
    },
}