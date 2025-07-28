use ragit_fs::FileError;
use crate::uid::UidError;

pub fn from_file_error(err: FileError) -> UidError {
    UidError::Io(err.into())
}
