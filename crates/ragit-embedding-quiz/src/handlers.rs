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

pub fn handle_query_command(model: &Model, terms: &[String]) {
    if terms.is_empty() {
        println!("Please provide at least one term to query.");
        return;
    }

    // Query for each individual term
    if terms.len() > 1 {
        println!("--- Individual Term Queries ---");
        for term_str in terms {
            if let Some(question) = model.questions.iter().find(|q| q.text == *term_str) {
                println!("\nQuerying for individual term: \"{}\"", term_str);
                println!("Current Embedding: {:?}\n", question.embedding);

                let similar_embeddings = model.find_similar_embeddings(question);
                if !similar_embeddings.is_empty() {
                    println!("Most Similar Embeddings:");
                    for (sim_q, distance) in similar_embeddings {
                        println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}\n", sim_q.id, sim_q.text, distance, sim_q.embedding);
                    }
                } else {
                    println!("No similar embeddings found for \"{}\".\n", term_str);
                }
            } else {
                println!("Term \"{}\" not found in embeddings. Skipping individual query.\n", term_str);
            }
        }
        println!("--- End Individual Term Queries ---\
");
    }


    // Combined query (always perform if at least one term is found)
    let mut combined_embedding: Vec<f32> = vec![0.0; 8]; // Assuming 8-dimensional embeddings
    let mut found_terms_count = 0;

    for term_str in terms {
        if let Some(question) = model.questions.iter().find(|q| q.text == *term_str) {
            for i in 0..8 {
                combined_embedding[i] += question.embedding[i];
            }
            found_terms_count += 1;
        }
    }

    if found_terms_count == 0 {
        println!("No known terms found among the input. Cannot perform combined query.");
        return;
    }

    // Average the combined embedding
    for i in 0..8 {
        combined_embedding[i] /= found_terms_count as f32;
    }

    // Create a dummy question for the combined embedding to use find_similar_embeddings
    let combined_question = Question {
        id: usize::MAX, // Use a dummy ID that won't conflict
        text: format!("Combined({})", terms.join(", ")),
        embedding: combined_embedding,
        is_missing_embedding: false,
    };

    println!("--- Combined Query ---");
    println!("Combined Embedding for input terms: {:?}", combined_question.embedding);

    let similar_embeddings = model.find_similar_embeddings(&combined_question);
    if !similar_embeddings.is_empty() {
        println!("\nMost Similar Embeddings to combined query:");
        for (sim_q, distance) in similar_embeddings {
            println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}\n", sim_q.id, sim_q.text, distance, sim_q.embedding);
        }
    } else {
        println!("No similar embeddings found for combined query.");
    }

    // Placeholder for RDF integration
    println!("\n--- RDF Information (Placeholder) ---");
    println!("(Would query RDF graph for information related to \"{}\")", terms.join(", "));

    // Placeholder for Documentation integration
    println!("\n--- Documentation Information (Placeholder) ---");
    println!("(Would search documentation for information related to \"{}\")", terms.join(", "));
}

pub fn handle_add_vector_command(model: &mut Model, term: &str, embedding_str: &str) {
    let submitted_embedding = crate::cli::parse_embedding(embedding_str)
        .expect("Invalid embedding format");

    // Check if term already exists
    if model.questions.iter().any(|q| q.text == term) {
        println!("Term \"{}\" already exists. Use 'answer' command to update its embedding.", term);
        return;
    }

    let new_id = model.questions.len(); // Simple sequential ID
    let new_question = Question {
        id: new_id,
        text: term.to_string(),
        embedding: submitted_embedding,
        is_missing_embedding: false, // It's explicitly added, so not missing
    };

    model.questions.push(new_question);
    model.weights.push(1.0); // Give it a default weight
    model.save();

    println!("Successfully added new term \"{}\" with ID {}.", term, new_id);
}
