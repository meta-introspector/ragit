mod model;
mod cli;
mod handlers;

use clap::Parser;
use model::Model;
use cli::{Cli, Commands, parse_embedding};
use handlers::{handle_quiz_command, handle_answer_command, handle_suggest_terms_command, handle_train_command};

fn main() {
    let cli = Cli::parse();
    let mut model = Model::new();

    match cli.command {
        Commands::Quiz => {
            handle_quiz_command(&mut model);
        },
        Commands::Answer { question_id, submitted_embedding_str } => {
            let submitted_embedding = parse_embedding(&submitted_embedding_str)
                .expect("Invalid embedding format");
            handle_answer_command(&mut model, question_id, submitted_embedding);
        },
        Commands::SuggestTerms => {
            handle_suggest_terms_command(&model);
        },
        Commands::Train { training_data_path, learning_rate } => {
            handle_train_command(&mut model, &training_data_path, learning_rate);
        },
    }
}