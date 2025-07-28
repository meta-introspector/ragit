use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_model_query_response::ModelQAResult;
use ragit_fs::{read_string, write_string, WriteMode};
use ragit_utils::cli_types::CliError;

pub async fn qa_tune_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new()
        .args(ArgType::String, ArgCount::Exact(1))
        .args(ArgType::String, ArgCount::Exact(1))
        .args(ArgType::String, ArgCount::Exact(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("Usage: rag qa-tune <result_file> <model_name> <user_score>");
        return Ok(());
    }

    let result_file = parsed_args.get_args_exact(1)?[0].clone();
    let model_name = parsed_args.get_args_exact(2)?[0].clone();
    let user_score = parsed_args.get_args_exact(3)?[0]
        .parse::<f64>()
        .map_err(|e| {
            anyhow::anyhow!(CliError::new_message(format!(
                "Invalid float for user_score: {}",
                e
            )))
        })?;

    let mut results: Vec<ModelQAResult> = serde_json::from_str(&read_string(&result_file)?)?;
    if let Some(result) = results.iter_mut().find(|r| r.model_name == model_name) {
        if user_score >= 0.0 && user_score <= 1.0 {
            result.user_score = Some(user_score);
            let updated_log = serde_json::to_string_pretty(&results)?;
            write_string(&result_file, &updated_log, WriteMode::CreateOrTruncate)?;
            println!(
                "Updated user score for {} to {:.2} in {}",
                model_name, user_score, result_file
            );
        } else {
            println!("Error: User score must be between 0.0 and 1.0");
        }
    } else {
        println!("Error: Model {} not found in {}", model_name, result_file);
    }
    Ok(())
}