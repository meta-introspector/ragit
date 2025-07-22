use crate::error::Error;
use crate::index::commands::archive::BlockType;

pub(crate) enum Response {
    Compressed(BlockType, String),
    IAmDone,
    Error(Error),
}
