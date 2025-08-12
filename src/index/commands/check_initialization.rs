use crate::prelude::*;

pub(super) fn initialize_check_data(
    index: &Index,
    images: &mut HashMap<Uid, bool>,
    chunks_to_files: &mut HashMap<Uid, (String, usize)>,
    processed_files: &mut HashSet<String>,
    all_chunk_uids: &mut HashSet<Uid>,
    uids_to_files: &mut HashMap<String, String>,
    file_uid_checks: &mut HashMap<String, bool>,
) {
    *images = HashMap::<Uid, bool>::new();
    *chunks_to_files = HashMap::with_capacity(index.chunk_count);
    *processed_files = HashSet::with_capacity(index.processed_files.len());
    *all_chunk_uids = HashSet::with_capacity(index.chunk_count);
    *uids_to_files = index.processed_files.iter().map(|(file, uid)| (uid.to_string(), file.display().to_string())).collect::<HashMap<_, _>>();
    *file_uid_checks = uids_to_files.keys().map(|uid| (uid.to_string(), false /* exists */)).collect::<HashMap<_, _>>();
}
