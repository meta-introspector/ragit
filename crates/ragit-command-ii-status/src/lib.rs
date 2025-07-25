use ragit_utils::prelude::*;
use ragit_api::prelude::*;
use ragit_types::prelude::*;

pub fn ii_status_command_main(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/ii-status.txt"));
        return Ok(());
    }

    let index = Index::load(find_root()?.into(), LoadMode::QuickCheck)?;

    let status = match index.ii_status {
        IIStatus::None => "not initialized",
        IIStatus::Complete => "complete",
        IIStatus::Outdated => "outdated",
        IIStatus::Ongoing(_) => "interact",
    };
    println!("{status}");

    Ok(())
}
