use ragit_utils::error::Error;
use ragit_utils::index::index_struct::Index;
use ragit_utils::index::load_mode::LoadMode;
use std::path::PathBuf;
use ragit_utils::index::commands::version::get_compatibility_warning;
use ragit_utils::cli_types::{ArgCount, ArgParser};
use ragit_utils::project_root::find_root;

pub async fn check_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--recover"])
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", get_doc_content("commands/check.txt"));
        return Ok(());
    }

    let root_dir = find_root()?;
    let recover = parsed_args.get_flag(0).is_some();

    let index = Index::load(root_dir.clone().to_string_lossy().into_owned(), LoadMode::OnlyJson)?;
    if let Ok(index_version) = index.get_ragit_version_info() {
        if recover && get_compatibility_warning(&index_version).is_some() {
            if let Ok(Some((v1, v2))) = Index::migrate(&root_dir.to_string_lossy().into_owned(), env!("CARGO_PKG_VERSION").to_string()) {
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
                let mut index = Index::load(root_dir.into(), LoadMode::Minimum)?;
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