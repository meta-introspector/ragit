use crate::index::commands::archive::BlockType;
use crate::uid::Uid;

pub(crate) enum Request {
    Compress(BlockType, Vec<Uid>),
    TellMeWhenYouAreDone,
    Kill,
}
