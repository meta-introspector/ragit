use ragit::Error;
use ragit_api::ModelQAResult;
use ragit_cli::{ArgParser, ArgCount, ArgType, Span};

pub async fn qa_tune_command(args: &Vec<String>) -> Result<(), Error> {
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
    let user_score = parsed_args.get_args_exact(3)?[0].parse::<f64>().map_err(|e| Error::CliError { message: format!("Invalid float for user_score: {}", e), span: (String::new(), 0, 0) })?;

    let mut results: Vec<ModelQAResult> = serde_json::from_str(&std::fs::read_to_string(&result_file)?)?;
    if let Some(result) = results.iter_mut().find(|r| r.model_name == model_name) {
        if user_score >= 0.0 && user_score <= 1.0 {
            result.user_score = Some(user_score);
            let updated_log = serde_json::to_string_pretty(&results)?;
            std::fs::write(&result_file, updated_log)?;
            println!("Updated user score for {} to {:.2} in {}", model_name, user_score, result_file);
        } else {
            println!("Error: User score must be between 0.0 and 1.0");
        }
    } else {
        println!("Error: Model {} not found in {}", model_name, result_file);
    }
    Ok(())
}
