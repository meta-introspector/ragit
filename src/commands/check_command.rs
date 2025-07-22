use ragit::{Error, Index, LoadMode, Path, get_compatibility_warning};
use ragit_cli::{ArgCount, ArgParser};

pub async fn check_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--recover"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/check.txt"));
        return Ok(());
    }

    let root_dir = crate::find_root()?;
    let recover = parsed_args.get_flag(0).is_some();

    let index = Index::load(root_dir.clone().to_string_lossy().into_owned(), LoadMode::OnlyJson)?;
    if let Ok(index_version) = index.get_ragit_version_info() {
        if recover && get_compatibility_warning(&index_version.to_string()).is_some() {
            if let Ok(Some((v1, v2))) = Index::migrate(&root_dir.to_string_lossy().into_owned(), ragit::VERSION.to_string()) {
                println!("migrated from `{v1}` to `{v2}`");
            }
        }
    }

    match Index::load(root_dir.clone().to_string_lossy().into_owned(), LoadMode::OnlyJson) {
        Ok(mut index) => {
            if index.curr_processing_file.is_some() && recover {
                let recover_result = index.recover()?;
                index.check()?;
                println!("recovered from a corrupted knowledge-base: {recover_result:?}");
            } else {
                match index.check() {
                    Ok(()) => {
                        println!("everything is fine!");
                    }
                    Err(e) => {
                        if recover {
                            let recover_result = index.recover()?;
                            index.check()?;
                            println!("recovered from a corrupted knowledge-base: {recover_result:?}");
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            if recover {
                let mut index = Index::load(root_dir.to_string_lossy().into_owned(), LoadMode::Minimum)?;
                let recover_result = index.recover()?;
                index.check()?;
                println!("recovered from a corrupted knowledge-base: {recover_result:?}");
            } else {
                return Err(e);
            }
        }
    }

    Ok(())
}