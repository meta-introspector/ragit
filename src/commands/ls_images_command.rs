use crate::{Error, Index, LoadMode, UidQueryConfig};
use ragit::schema::Prettify;
use ragit_cli::{ArgCount, ArgParser, ArgType};

pub fn ls_images_command(args: &[String]) -> Result<(), Error> {
    let parsed_args = ArgParser::new()
        .optional_flag(&["--uid-only", "--stat-only"])
        .optional_flag(&["--json"])
        .short_flag(&["--json"])
        .args(ArgType::UidOrPath, ArgCount::Any)
        .parse(args, 2)?;

    if parsed_args.show_help() {
        println!("{}", include_str!("../../docs/commands/ls-images.txt"));
        return Ok(());
    }

    let uid_only = parsed_args.get_flag(0).unwrap_or_default() == "--uid-only";
    let stat_only = parsed_args.get_flag(0).unwrap_or_default() == "--stat-only";
    let json_mode = parsed_args.get_flag(1).is_some();
    let index = Index::load(crate::find_root()?.to_string_lossy().into_owned(), LoadMode::OnlyJson)?;
    let args = parsed_args.get_args();

    let images = if args.is_empty() {
        index.list_images(
            &|_| true,  // no filter
            &|image| image,  // no map
            &|_| 0,  // no sort
        )?
    } else {
        let query = index.uid_query(&args, UidQueryConfig::new())?;
        let mut image_uids = vec![];

        for (_, uid) in query.get_processed_files() {
            for image_uid in index.get_images_of_file(uid)? {
                image_uids.push(image_uid);
            }
        }

        for uid in query.get_chunk_uids() {
            let chunk = index.get_chunk_by_uid(uid)?;

            for image_uid in chunk.images {
                image_uids.push(image_uid);
            }
        }

        for image_uid in query.get_image_uids() {
            image_uids.push(image_uid);
        }

        if image_uids.is_empty() {
            return Err(Error::UidQueryError(format!("There's no chunk/file/image that matches `{}`.", args.join(" "))));
        }

        let mut result = Vec::with_capacity(image_uids.len());

        for image_uid in image_uids.iter() {
            result.push(index.get_image_schema(*image_uid, false)?);
        }

        result
    };

    if uid_only {
        if json_mode {
            println!(
                "{}",
                serde_json::to_string_pretty(&images.iter().map(|image| image.uid.to_string()).collect::<Vec<_>>())?,
            );
        } else {
            for image in images.iter() {
                println!("{}", image.uid);
            }
        }
    } else if stat_only {
        if json_mode {
            println!("{{\"images\":{}}}", images.len());
        } else {
            println!("{} images", images.len());
        }
    } else {
        if json_mode {
            println!(
                "{}",
                serde_json::to_string_pretty(&images.prettify()?)?,
            );
        } else {
            println!("{} images", images.len());

            for image in images.iter() {
                println!("----------");
                println!("uid: {}", image.uid);
                println!("explanation: {}", image.explanation);
                println!("extracted_text: {}", image.extracted_text);
                println!("size: {}", image.size);
            }
        }
    }

    Ok(())
}