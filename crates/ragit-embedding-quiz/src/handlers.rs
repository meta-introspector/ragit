use crate::model::{Model, Question, TrainingPair};
use std::fs;
use serde_json;

pub fn handle_quiz_command(model: &mut Model) {
    if let Some(question) = model.get_question() {
        println!("Question ID: {}", question.id);
        println!("Question Text: {}", question.text);
        println!("Current Embedding: {:?}", question.embedding);
        if question.is_missing_embedding {
            println!("This is a new term. Please provide an embedding.");
        }

        let similar_embeddings = model.find_similar_embeddings(&question);
        if !similar_embeddings.is_empty() {
            println!("\nMost Similar Embeddings:");
            for (sim_q, distance) in similar_embeddings {
                println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}", sim_q.id, sim_q.text, distance, sim_q.embedding);
            }
        }
    } else {
        println!("No questions available.");
    }
}

pub fn handle_answer_command(model: &mut Model, question_id: usize, submitted_embedding: Vec<f32>) {
    if let Some(question) = model.questions.get_mut(question_id) {
        if question.is_missing_embedding {
            question.embedding = submitted_embedding.clone(); // Directly set embedding for new terms
            question.is_missing_embedding = false; // Mark as no longer missing
            println!("New embedding added for Question ID: {}", question_id);
        } else {
            let distance = Model::calculate_distance(&question.embedding, &submitted_embedding);
            let is_correct = distance < 0.1; // Threshold for correctness

            if !is_correct {
                model.update_embedding(question_id, submitted_embedding.clone());
            }
            model.update_weight(question_id, is_correct);
            println!("Answer submitted for Question ID: {}", question_id);
            println!("Correct: {}", is_correct);
            if !is_correct {
                println!("Embedding updated.");
            }
        }
        model.save(); // Save after any update
    } else {
        println!("Question ID {} not found.", question_id);
    }
}

pub fn handle_suggest_terms_command(model: &Model) {
    let missing_terms: Vec<&Question> = model.questions.iter()
        .filter(|q| q.is_missing_embedding)
        .collect();

    if !missing_terms.is_empty() {
        println!("Suggested Terms with Missing Embeddings:");
        for term in missing_terms {
            println!("  - ID: {}, Text: {}", term.id, term.text);
        }
    } else {
        println!("No missing embeddings found. All terms have an embedding.");
    }
}

pub fn handle_train_command(model: &mut Model, training_data_path: &str, learning_rate: Option<f32>) {
    let learning_rate = learning_rate.unwrap_or(0.1); // Default learning rate

    let training_data_str = fs::read_to_string(training_data_path)
        .expect(&format!("Could not read training data from {}", training_data_path));
    let training_pairs: Vec<TrainingPair> = serde_json::from_str(&training_data_str)
        .expect("Could not parse training data");

    println!("Training with {} pairs...", training_pairs.len());

    for pair in training_pairs {
        let term1_id = model.questions.iter().find(|q| q.text == pair.term1).map(|q| q.id);
        let term2_id = model.questions.iter().find(|q| q.text == pair.term2).map(|q| q.id);

        if let (Some(id1), Some(id2)) = (term1_id, term2_id) {
            let embedding1 = model.questions[id1].embedding.clone();
            let embedding2 = model.questions[id2].embedding.clone();

            // Move embedding1 closer to embedding2
            for i in 0..embedding1.len() {
                model.questions[id1].embedding[i] += (embedding2[i] - embedding1[i]) * learning_rate;
            }
            // Optionally, move embedding2 closer to embedding1 as well
            for i in 0..embedding2.len() {
                model.questions[id2].embedding[i] += (embedding1[i] - embedding2[i]) * learning_rate;
            }

            println!("Adjusted embeddings for \"{}\" and \"{}\"", pair.term1, pair.term2);
        } else {
            println!("Warning: Could not find one or both terms in training pair: \"{}\", \"{}\"", pair.term1, pair.term2);
        }
    }

    model.save();
    println!("Training complete. Embeddings saved.");
}
