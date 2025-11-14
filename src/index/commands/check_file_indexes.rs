use crate::prelude::*;

pub(super) fn check_file_indexes(
    index: &Index,
    chunks_to_files: &HashMap<Uid, (String, usize)>,
    uids_to_files: &HashMap<String, String>,
    file_uid_checks: &mut HashMap<String, bool>,
) -> Result<(), Error> {
    for file_index in &index.get_all_file_indexes()? {
        let uid_prefix = basename(path_utils::pathbuf_to_str(&ragit_fs::parent(file_index.as_path())?))?;
        let uid_suffix = file_name(path_utils::pathbuf_to_str(file_index))?;
        let file_uid = format!("{uid_prefix}{uid_suffix}");
        let file_name = match uids_to_files.get(&file_uid) {
            Some(file_name) => file_name.to_string(),
            None => {  // Check B-1
                return Err(Error::BrokenIndex(error_reporting::file_index_not_in_processed_files(&file_uid)));
            },
        };

        match file_uid_checks.get_mut(&file_uid) {
            Some(exists) => { *exists = true; },
            None => unreachable!(),  // Check B-1, already checked
        }

        for (index1, uid) in uid::load_from_file(path_utils::pathbuf_to_str(file_index))?.iter().enumerate() {
            match chunks_to_files.get(uid) {
                Some((file_name_from_chunk, index2)) => {
                    if &file_name != file_name_from_chunk {  // Check B-0
                        return Err(Error::BrokenIndex(error_reporting::chunk_points_to_wrong_file(path_utils::path_to_display(file_index), &file_name, *uid, file_name_from_chunk)));
                    }

                    // Extra check: chunk uids in a file_index must be in a correct order
                    if index1 != *index2 {
                            return Err(Error::BrokenIndex(error_reporting::chunk_index_mismatch(path_utils::path_to_display(file_index), index1, *uid, *index2)));
                        }
                },
                None => {  // Check B-0
                    return Err(Error::BrokenIndex(error_reporting::chunk_not_found(path_utils::path_to_display(file_index), *uid, INDEX_DIR_NAME, CHUNK_DIR_NAME)));
                }
            }
        }
    }

    for (file_uid, exists) in file_uid_checks.iter() {
        if !*exists {  // Check B-2
            let file_name = uids_to_files.get(file_uid).unwrap();
            return Err(Error::BrokenIndex(error_reporting::file_has_no_index(&file_name)));
        }
    }
    Ok(())
}
