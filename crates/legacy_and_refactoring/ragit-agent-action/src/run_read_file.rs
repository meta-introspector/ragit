use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
// use ragit_index_types::index_impl::{get_chunks_of_file, get_chunk_by_uid};
use ragit_types::ApiError;
// use ragit_utils::ragit_path_utils::normalize;
// use ragit_utils::string_utils::substr_edit_distance;

pub(crate) async fn run_read_file(_argument: &str, _index: &Index) -> Result<ActionResult, ApiError> {
    panic!("FIX ME LATER: Fix the bootstrap first and this code later.");
    // let argument_path = PathBuf::from(normalize(argument)?);

    // let result = match index.processed_files.get(&argument_path) {
    //     Some(uid) => {
    //         let chunk_uids = index.get_chunks_of_file(*uid)?;

    //         // If the file is too long, it shows the summaries of its chunks
    //         // instead of `cat-file`ing the file.
    //         // TODO: what if it's sooooo long that even the chunk list is too long?
    //         let max_chunks = index.query_config.max_retrieval;

    //         // NOTE: Even an empty file has a chunk. So `.len()` must be greater than 0.
    //         match chunk_uids.len() {
    //             1 => {
    //                 // TODO: Replace with StorageManager call
    //                 let chunk =
    //                     ragit_types::chunk::chunk_struct::Chunk::dummy().render_source();
    //                 ActionResult::ReadFileShort {
    //                     rendered: chunk,
    //                     file_path: argument_path.display().to_string(),
    //                 }
    //             }
    //             n if n <= max_chunks => {
    //                 // TODO: Replace with StorageManager call
    //                 let chunk =
    //                     ragit_types::chunk::chunk_struct::Chunk::dummy().render_source();
    //                 ActionResult::ReadFileShort {
    //                     rendered: chunk,
    //                     file_path: argument_path.display().to_string(),
    //                 }
    //             }
    //             _ => {
    //                 let mut chunks = Vec::with_capacity(chunk_uids.len());

    //                 for chunk_uid in chunk_uids.iter() {
    //                     // TODO: Replace with StorageManager call
    //                     chunks.push(index.get_chunk_by_uid(*chunk_uid)?);
    //                 }

    //                 ActionResult::ReadFileLong(chunks)
    //             }
    //         }
    //     }
    //     None => {
    //         let mut similar_files = vec![];

    //         // TODO: it might take very very long time if the knowledge-base is large...
    //         for file in index.processed_files.keys() {
    //             let dist = substr_edit_distance(
    //                 argument.as_bytes(),
    //                 file.to_str().unwrap().as_bytes(),
    //             );

    //             if dist < 3 {
    //                 similar_files.push((file.display().to_string(), dist));
    //             }
    //         }

    //         similar_files.sort_by_key(|(_, d)| *d);

    //         if similar_files.len() > 10 {
    //             similar_files = similar_files[..10].to_vec();
    //         }

    //         let similar_files = similar_files.into_iter().map(|(f, _)| f).collect::<Vec<_>>();
    //         ActionResult::NoSuchFile {
    //             file: argument_path.display().to_string(),
    //             similar_files,
    //         }
    //     }
    // };
    // Ok(result)
}