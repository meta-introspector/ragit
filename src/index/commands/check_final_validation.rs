use crate::index::index_struct::Index;
use crate::error::Error;
use crate::error_reporting;
use crate::IIStatus;
use crate::prelude::*;

pub(super) fn check_final_validation(
    index: &Index,
    chunk_count: usize,
) -> Result<(), Error> {
    if chunk_count != index.chunk_count {  // Check D
        return Err(Error::BrokenIndex(error_reporting::chunk_count_mismatch(index.chunk_count, chunk_count)));
    }

    if index.ii_status == IIStatus::Complete {
        index.check_ii()?;
    }

    if let Some(uid) = index.uid {
        let c_uid = index.calculate_uid(true  /* --force */)?;

        if uid != c_uid {
            return Err(Error::BrokenIndex(error_reporting::uid_mismatch(uid, c_uid)));
        }
    }

    Ok(())
}
