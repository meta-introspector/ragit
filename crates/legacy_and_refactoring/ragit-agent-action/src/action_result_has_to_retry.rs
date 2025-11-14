use super::action_result_enum::ActionResult;

impl ActionResult {
    // If it's ok, the AI can update the context with information from `self.render()`.
    // If it's not ok, `self.render()` will instruct the AI how to generate a valid argument.
    pub fn has_to_retry(&self) -> bool {
        match self {
            ActionResult::ReadFileShort { .. }
            | ActionResult::ReadFileLong(_)
            | ActionResult::ReadDir(_)
            | ActionResult::ReadChunk(_)
            | ActionResult::ReadChunkTooMany { .. }  // There's nothing AI can do
            | ActionResult::Search { .. }
            | ActionResult::GetMeta { .. }
            | ActionResult::GetSummary(_)
            | ActionResult::SimpleRag(_) => false,
            ActionResult::NoSuchFile { .. }
            | ActionResult::NoSuchDir { .. }
            | ActionResult::NoSuchChunk(_)
            | ActionResult::ReadChunkAmbiguous { .. }
            | ActionResult::NoSuchMeta { .. } => true,
        }
    }
}
