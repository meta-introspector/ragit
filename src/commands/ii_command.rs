use crate::{Error, IIStatus, Index, LoadMode};
use ragit_cli::ArgParser;

pub fn ii_command(args: &[String]) -> Result<(), Error> {
    let mut index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::QuickCheck)?;

    match args.get(1).map(|arg| arg.as_str()) {
        Some("ii-build") | Some("build-ii") => {
            let parsed_args = ArgParser::new()
                .optional_flag(&["--quiet"])
                .short_flag(&["--quiet"])
                .parse(args, 2)?;

            if parsed_args.show_help() {
                println!("{}", include_str!("../../docs/commands/ii-build.txt"));
                return Ok(());
            }

            let quiet = parsed_args.get_flag(0).is_some();
            index.build_ii(quiet)?;
        }
        Some("ii-reset") | Some("reset-ii") => {
            let parsed_args = ArgParser::new().parse(args, 2)?;

            if parsed_args.show_help() {
                println!("{}", include_str!("../../docs/commands/ii-reset.txt"));
                return Ok(());
            }

            index.reset_ii()?;
        }
        Some("ii-status") => {
            let parsed_args = ArgParser::new().parse(args, 2)?;

            if parsed_args.show_help() {
                println!("{}", include_str!("../../docs/commands/ii-status.txt"));
                return Ok(());
            }

            let status = match index.ii_status {
                IIStatus::None => "not initialized",
                IIStatus::Complete => "complete",
                IIStatus::Outdated => "outdated",
                IIStatus::Ongoing(_) => "interrupted",
            };
            println!("{status}");
        }
        _ => unreachable!(),
    }

    Ok(())
}
