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

// Model
#[derive(Clone)]
struct Question {
    id: usize,
    text: String,
    answer: String,
}

#[derive(Clone)]
struct Model {
    questions: Vec<Question>,
    weights: Vec<f32>,
}

impl Model {
    fn new() -> Self {
        let questions = vec![
            Question { id: 0, text: "What is the capital of France?".to_string(), answer: "Paris".to_string() },
            Question { id: 1, text: "What is 2 + 2?".to_string(), answer: "4".to_string() },
            Question { id: 2, text: "What is the color of the sky?".to_string(), answer: "Blue".to_string() },
        ];
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
    submitted_answer: String,
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

async fn answer_handler(State(state): State<AppState>, Json(answer): Json<Answer>) -> Json<AnswerResponse> {
    let mut model = state.model.lock().unwrap();
    let correct = if let Some(question) = model.questions.iter().find(|q| q.id == answer.question_id) {
        let correct = question.answer == answer.submitted_answer;
        model.update_weight(answer.question_id, correct);
        correct
    } else {
        false
    };
    Json(AnswerResponse { correct })
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
