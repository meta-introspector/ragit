use super::BlockType;

pub enum Request {
    Extract {
        block_type: BlockType,
        path: String,
        from: u64,
        to: u64,
    },
    TellMeWhenYouAreDone,
    Kill,
}