use ragit::{ApiConfig, Error, Index, LoadMode};
use ragit_api::{Model, Request};
use ragit_api::qa_system::{ModelQASystem, ModelQAResult, QualityScores};
use ragit_pdl::{Message, Role};
use ragit_cli::{ArgParser, ArgCount, ArgType};

pub async fn qa_test_command(args: &Vec<String>) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .args(ArgType::Query, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        // TODO: Add help message for qa-test
        println!("Usage: rag qa-test <prompt>");
        return Ok(());
    }

    let prompt = parsed_args.get_args().join(" ");
    let config = ApiConfig::default();
    let models = Index::get_initial_models()?;
    let qa_system = ModelQASystem::new(models, config.throttling_safety_margin);
    let request = Request {
        messages: vec![Message::simple_message(Role::User, prompt)],
        model: Model::dummy(), // Will be overridden per model
        ..Request::default()
    };
    let results = qa_system.test_request(request).await?;
    println!("{}", ragit_api::qa_system::compare_results::compare_results(&qa_system.models, &results));
    crate::log_qa_results::log_qa_results(&results)?;
    Ok(())
}
