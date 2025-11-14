use crate::prelude::*;

pub(super) fn check_chunk_files(
    index: &Index,
    chunks_to_files: &mut HashMap<Uid, (String, usize)>,
    processed_files: &mut HashSet<String>,
    all_chunk_uids: &mut HashSet<Uid>,
    chunk_count: &mut usize,
) -> Result<(), Error> {
    for chunk_file in &index.get_all_chunk_files()? {
        let chunk_prefix = basename(path_utils::pathbuf_to_str(&parent(chunk_file.as_path())?))?;
        let chunk_suffix = file_name(path_utils::pathbuf_to_str(chunk_file))?;
        let chunk_uid = Uid::from_prefix_and_suffix(&chunk_prefix, &chunk_suffix)?;
        let chunk = chunk::load_from_file(chunk_file)?;
        *chunk_count += 1;
        all_chunk_uids.insert(chunk_uid);

        // TODO: This condition has to be checked, but it's too tough for old versions of `migration` to pass this condition.
        //       When time passes and almost no user uses old versions, I have to revive this condition.
        // if chunk.uid != Uid::new_chunk(&chunk) {  // Check A-0
        //     return Err(Error::BrokenIndex(format!("Corrupted chunk: `{chunk_file}`'s uid is supposed to be `{}`, but is `{}`.", Uid::new_chunk(&chunk), chunk.uid)));
        // }

        if chunk_uid != chunk.uid {  // Check A-0
            return Err(Error::BrokenIndex(error_reporting::corrupted_chunk_uid_mismatch(path_utils::path_to_display(chunk_file), chunk_uid, chunk.uid)));
        }

        match &chunk.source {
            ChunkSource::File { path, index, page: _ } => {
                chunks_to_files.insert(chunk_uid, (path.to_string(), *index));
                processed_files.insert(path.to_string());
            },
        }

        // images.insert(*image, false /* exists */);
        let tfidf_file = path_utils::str_to_pathbuf(&set_extension(path_utils::pathbuf_to_str(chunk_file), "tfidf")?);
        tfidf::load_from_file(path_utils::pathbuf_to_str(&tfidf_file))?;
    }

    for processed_file in processed_files.iter() {
        if !index.processed_files.contains_key(&path_utils::str_to_pathbuf(processed_file)) {  // Check A-1
            return Err(Error::BrokenIndex(error_reporting::chunk_file_not_in_processed_files(processed_file)));
        }
    }
    Ok(())
}
