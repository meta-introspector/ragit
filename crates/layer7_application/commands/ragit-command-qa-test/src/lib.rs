use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
// use ragit_qa_system::ModelQASystem;
// use ragit_index_types::index_struct::Index;
// use ragit_types::api_config::ApiConfig;
// use ragit_api::{Request, Message, Role, Model};
// use ragit_utils::project_root::find_root;

pub async fn qa_test_command_main(_args: &[String]) -> Result<(), anyhow::Error> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    // let parsed_args = ArgParser::new()
    //     .args(ArgType::Query, ArgCount::Any)
    //     .parse(args, 2)?;

    // if parsed_args.show_help() {
    //     println!("Usage: rag qa-test <prompt>");
    //     return Ok(());
    // }

    // let prompt = parsed_args.get_args().join(" ");
    // let config = ApiConfig::default();
    // let models = Index::get_initial_models(&find_root()?)?;
    // let qa_system = ModelQASystem::new(models, config.throttling_safety_margin.into());
    // let request = Request::default();
    // let request = Request {
    //     messages: vec![Message::simple_message(Role::User, prompt)],
    //     model: Model::dummy(), // Will be overridden per model
    //     ..request
    // };
    // let results = qa_system.test_request(request).await?;
    // println!(
    //     "{}",
    //     ragit_qa_system::compare_results::compare_results(&qa_system.models, &results)
    // );
    // // TODO: log_qa_results::log_qa_results(&results)?;
    // Ok(())
}