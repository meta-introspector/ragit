use ragit_commands::{Error, Index, LoadMode, ArgParser, IIStatus, find_root, get_doc_content, ii_build_command_main, ii_reset_command_main, ii_status_command_main};

pub fn ii_command(args: &[String]) -> Result<(), Error> {
    match args.get(1).map(|arg| arg.as_str()) {
        Some("ii-build") | Some("build-ii") => {
            ii_build_command_main(args)?;
        }
        Some("ii-reset") | Some("reset-ii") => {
            ii_reset_command_main(args)?;
        }
        Some("ii-status") => {
            ii_status_command_main(args)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}