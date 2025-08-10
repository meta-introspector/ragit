use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use rand::prelude::*;

// Model
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Question {
    id: usize,
    text: String,
    embedding: Vec<f32>,
}

#[derive(Clone)]
struct Model {
    questions: Vec<Question>,
    weights: Vec<f32>,
}

impl Model {
    fn new() -> Self {
        let embeddings_path = format!("{}/../../term_embeddings.json", env!("CARGO_MANIFEST_DIR"));
        let embeddings_str = fs::read_to_string(&embeddings_path).expect("Could not read term_embeddings.json");
        let raw_embeddings: HashMap<String, Vec<f32>> = serde_json::from_str(&embeddings_str).expect("Could not parse term_embeddings.json");

        let questions: Vec<Question> = raw_embeddings.into_iter().enumerate().map(|(id, (text, embedding))| {
            Question { id, text, embedding }
        }).collect();

        let weights = vec![1.0; questions.len()];
        Self { questions, weights }
    }

    fn save(&self) {
        let embeddings_path = format!("{}/../../term_embeddings.json", env!("CARGO_MANIFEST_DIR"));
        let mut raw_embeddings = HashMap::new();
        for q in &self.questions {
            raw_embeddings.insert(q.text.clone(), q.embedding.clone());
        }
        let json_str = serde_json::to_string_pretty(&raw_embeddings).expect("Could not serialize embeddings");
        fs::write(&embeddings_path, json_str).expect("Could not write term_embeddings.json");
    }

    fn get_question(&self) -> Option<Question> {
        let mut rng = thread_rng();
        self.questions.choose_weighted(&mut rng, |item| self.weights[item.id]).ok().cloned()
    }

    fn update_weight(&mut self, question_id: usize, correct: bool) {
        if correct {
            self.weights[question_id] *= 1.1;
        } else {
            self.weights[question_id] *= 0.9;
        }
    }

    fn update_embedding(&mut self, question_id: usize, submitted_embedding: Vec<f32>) {
        if let Some(question) = self.questions.get_mut(question_id) {
            for i in 0..question.embedding.len() {
                question.embedding[i] = (question.embedding[i] + submitted_embedding[i]) / 2.0;
            }
        }
    }

    fn calculate_distance(embedding1: &[f32], embedding2: &[f32]) -> f32 {
        embedding1.iter().zip(embedding2.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f32>().sqrt()
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a quiz question
    Quiz,
    /// Submit an answer to a quiz question
    Answer {
        question_id: usize,
        submitted_embedding_str: String,
    },
}

fn parse_embedding(s: &str) -> Result<Vec<f32>, String> {
    s.split(',')
        .map(|s| s.trim().parse::<f32>().map_err(|e| format!("Invalid float: {}", e)))
        .collect()
}

fn main() {
    let cli = Cli::parse();
    let mut model = Model::new();

    match &cli.command {
        Commands::Quiz => {
            if let Some(question) = model.get_question() {
                println!("Question ID: {}", question.id);
                println!("Question Text: {}", question.text);
                println!("Current Embedding: {:?}", question.embedding);
            } else {
                println!("No questions available.");
            }
        },
        Commands::Answer { question_id, submitted_embedding_str } => {
            let submitted_embedding = parse_embedding(submitted_embedding_str)
                .expect("Invalid embedding format");
            if let Some(question) = model.questions.get(*question_id) {
                let distance = Model::calculate_distance(&question.embedding, &submitted_embedding);
                let is_correct = distance < 0.1; // Threshold for correctness

                if !is_correct {
                    model.update_embedding(*question_id, submitted_embedding.clone());
                }
                model.update_weight(*question_id, is_correct);
                model.save();

                println!("Answer submitted for Question ID: {}", question_id);
                println!("Correct: {}", is_correct);
                if !is_correct {
                    println!("Embedding updated.");
                }
            } else {
                println!("Question ID {} not found.", question_id);
            }
        },
    }
}
