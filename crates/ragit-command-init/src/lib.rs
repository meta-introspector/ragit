use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;
use ragit_utils::doc_utils::get_doc_content;
//use ragit_index_core::Index;

pub fn init_command_main(args: &[String]) -> Result<(), anyhow::Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/init.txt"));
        return Ok(());
    }

    match Index::dummy() {
        _ => {
            println!("initialized");
        }
    }

    Ok(())
}
