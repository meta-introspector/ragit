use crate::{Error, Index, LoadMode, MergeMode};
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub async fn merge_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--ignore", "--force", "--interactive", "--reject"])
        .optional_flag(&["--dry-run"])
        .optional_arg_flag("--prefix", ArgType::Path)
        .optional_flag(&["--quiet"])
        .short_flag(&["--force", "--quiet"])
        .args(ArgType::Path, ArgCount::Geq(1))
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/merge.txt"));
        return Ok(());
    }

    let mut index = Index::load(crate::find_root()?, LoadMode::OnlyJson)?;
    let bases = parsed_args.get_args();
    let merge_mode = MergeMode::from_flag(&parsed_args.get_flag(0).unwrap_or_else(|| "--ignore".to_string())).unwrap();
    let dry_run = parsed_args.get_flag(1).is_some();
    let quiet = parsed_args.get_flag(2).is_some();

    // if it's `--reject` mode, it first runs with `--dry-run` mode.
    // if the dry_run has no problem, then it actually runs
    for base in bases.iter() {
        index.merge(
            base.to_string(),
            parsed_args.arg_flags.get("--prefix").map(|p| p.to_string()),
            merge_mode,
            (merge_mode == MergeMode::Reject && !dry_run) || quiet,
            dry_run || merge_mode == MergeMode::Reject,
        ).await?;
    }

    if merge_mode == MergeMode::Reject && !dry_run {
        for base in bases.iter() {
            index.merge(
                base.to_string(),
                parsed_args.arg_flags.get("--prefix").map(|p| p.to_string()),
                merge_mode,
                quiet,
                dry_run,
            ).await?;
        }
    }

    Ok(())
}
