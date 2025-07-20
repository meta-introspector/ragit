use crate::{Error, Index, LoadMode};
use ragit_cli::{ArgParser};

pub fn gc_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .flag(&["--logs", "--images", "--audit", "--all"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/gc.txt"));
        return Ok(());
    }

    let mut index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::OnlyJson)?;

    match parsed_args.get_flag(0).unwrap().as_str() {
        "--logs" => {
            let removed = index.gc_logs()?;
            println!("removed {removed} log files");
        }
        "--images" => {
            let removed = index.gc_images()?;
            println!("removed {removed} images");
        }
        "--audit" => {
            index.gc_audit()?;
            println!("removed audit logs");
        }
        "--all" => {
            let removed_logs = index.gc_logs()?;
            let removed_images = index.gc_images()?;
            index.gc_audit()?;
            println!("removed {removed_logs} log files, {removed_images} images and audit logs");
        }
        _ => unreachable!(),
    }

    Ok(())
}
