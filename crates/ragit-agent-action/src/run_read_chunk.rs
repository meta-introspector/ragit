use crate::action_result_enum::ActionResult;
use ragit_index_types::index_struct::Index;
use ragit_types::{ApiError, Uid, Chunk};
use ragit_index_io::query_helpers::{uid_query, UidQueryConfig};

pub(crate) async fn run_read_chunk(argument: &str, index: &Index) -> Result<ActionResult, ApiError> {
    if !Uid::is_valid_prefix(argument) {
        return Ok(ActionResult::NoSuchChunk(argument.to_string()));
    }

    let query = uid_query(index, &[argument.to_string()], UidQueryConfig::new().chunk_only())?;
    let chunk_uids = query.get_chunk_uids();

    let result = match chunk_uids.len() {
        0 => ActionResult::NoSuchChunk(argument.to_string()),
        1 => ActionResult::ReadChunk(ragit_types::chunk::chunk_struct::Chunk::dummy()), // Placeholder
        2..=10 => {
            let mut chunks = Vec::with_capacity(chunk_uids.len());

            for _chunk_uid in chunk_uids.iter() {
                // TODO: Replace with StorageManager call
                chunks.push(ragit_types::chunk::chunk_struct::Chunk::dummy()); // Placeholder
            }

            ActionResult::ReadChunkAmbiguous {
                query: argument.to_string(),
                chunks,
            }
        }
        _ => ActionResult::ReadChunkTooMany {
            query: argument.to_string(),
            chunk_uids: chunk_uids.len(),
        },
    };
    Ok(result)
}
