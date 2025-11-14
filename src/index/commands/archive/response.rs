use crate::error::Error;

use super::BlockType;

pub enum Response {
    Complete(BlockType),
    IAmDone,
    Error(Error),
}