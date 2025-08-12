use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use tokio::signal;
use tokio::sync::oneshot;
use std::sync::{Arc, Mutex};
use daemonize::Daemonize;
use rand::prelude::*;
use std::fs;
use std::collections::HashMap;
use std::io::Write;

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
        let embeddings_str = fs::read_to_string(embeddings_path).expect("Could not read term_embeddings.json");
        let raw_embeddings: HashMap<String, Vec<f32>> = serde_json::from_str(&embeddings_str).expect("Could not parse term_embeddings.json");

        let questions: Vec<Question> = raw_embeddings.into_iter().enumerate().map(|(id, (text, embedding))| {
            Question { id, text, embedding }
        }).collect();

        let weights = vec![1.0; questions.len()];
        Self { questions, weights }
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
            // Simple averaging for now
            for i in 0..question.embedding.len() {
                question.embedding[i] = (question.embedding[i] + submitted_embedding[i]) / 2.0;
            }
        }
    }

    fn calculate_distance(embedding1: &[f32], embedding2: &[f32]) -> f32 {
        embedding1.iter().zip(embedding2.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f32>().sqrt()
    }
}

// Application State
#[derive(Clone)]
struct AppState {
    shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    model: Arc<Mutex<Model>>,
}

// API Data Structures
#[derive(Serialize)]
struct QuizQuestion {
    id: usize,
    text: String,
}

#[derive(Deserialize)]
struct Answer {
    question_id: usize,
    submitted_embedding: Vec<f32>,
}

#[derive(Serialize)]
struct AnswerResponse {
    correct: bool,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
}

// Handlers
async fn status_handler() -> Json<StatusResponse> {
    Json(StatusResponse {
        status: "running".to_string(),
    })
}

async fn stop_handler(State(state): State<AppState>) -> &'static str {
    if let Some(tx) = state.shutdown_tx.lock().unwrap().take() {
        let _ = tx.send(());
        "Shutting down..."
    } else {
        "Shutdown signal already sent or not available."
    }
}

async fn quiz_handler(State(state): State<AppState>) -> Json<Option<QuizQuestion>> {
    let model = state.model.lock().unwrap();
    let question = model.get_question().map(|q| QuizQuestion { id: q.id, text: q.text });
    Json(question)
}

async fn answer_handler(State(state): State<AppState>, payload: Json<serde_json::Value>) -> Json<AnswerResponse> {
    println!("Received raw JSON payload: {:?}", payload);

    let answer: Answer = serde_json::from_value(payload.0).expect("Failed to deserialize Answer");

    let mut model = state.model.lock().unwrap();
    let correct = if let Some(question) = model.questions.get(answer.question_id) {
        let distance = Model::calculate_distance(&question.embedding, &answer.submitted_embedding);
        let is_correct = distance < 0.1; // Threshold for correctness
        if !is_correct {
            model.update_embedding(answer.question_id, answer.submitted_embedding);
        }
        model.update_weight(answer.question_id, is_correct);
        is_correct
    } else {
        false
    };
    Json(AnswerResponse { correct })
}

async fn debug_answer_handler(payload: Json<serde_json::Value>) -> &'static str {
    let log_path = format!("{}/../../tmp/debug_payload.log", env!("CARGO_MANIFEST_DIR"));
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("Could not open debug_payload.log");

    writeln!(file, "Received raw JSON payload on debug endpoint: {:?}", payload)
        .expect("Could not write to debug_payload.log");

    "Debug endpoint reached! Check debug_payload.log"
}

// Main
fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let pid_file = format!("{}/../../tmp/quiz_server.pid", crate_dir);
    let daemonize = Daemonize::new()
        .pid_file(&pid_file)
        .chown_pid_file(false)
        .working_directory(crate_dir);

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized.");
            if let Ok(pid) = fs::read_to_string(&pid_file) {
                println!("PID: {}", pid.trim());
            }
            run_server();
        }
        Err(e) => eprintln!("Error, {}", e),
    }
}

#[tokio::main]
async fn run_server() {
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let model = Model::new();
    let app_state = AppState {
        shutdown_tx: Arc::new(Mutex::new(Some(shutdown_tx))),
        model: Arc::new(Mutex::new(model)),
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, Quiz Server!" }))
        .route("/status", get(status_handler))
        .route("/stop", get(stop_handler))
        .route("/quiz", get(quiz_handler))
        .route("/answer", post(answer_handler))
        .route("/debug_answer", post(debug_answer_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Quiz server listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            signal::ctrl_c().await.unwrap();
            println!("Ctrl-C received, shutting down gracefully.");
            // Also listen for the oneshot signal from the /stop endpoint
            let _ = shutdown_rx.await;
            println!("Shutdown signal received, shutting down.");
            std::process::exit(0);
        })
        .await
        .unwrap();
}
