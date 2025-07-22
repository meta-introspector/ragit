use crate::{Error, Index, LoadMode};
use ragit_cli::ArgParser;

pub fn migrate_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new().parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/migrate.txt"));
        return Ok(());
    }

    let root_dir = crate::find_root()?;

    if let Some((v1, v2)) = Index::migrate(&root_dir, env!("CARGO_PKG_VERSION").to_string())? {
        println!("migrated from `{v1}` to `{v2}`");
    }

    let mut index = Index::load(root_dir.to_string_lossy().into_owned(), LoadMode::Minimum)?;
    let recover_result = index.recover()?;

    if !recover_result.is_empty() {
        println!("recovered from a corrupted knowledge-base: {recover_result:?}");
    }

    Ok(())
}
