use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum Action {
    ReadFile,
    ReadDir,
    ReadChunk,
    SearchExact,
    SearchTfidf,

    /// This action will be filtered out if there's no metadata.
    GetMeta,

    /// This action will be filtered out if there's no summary.
    /// Please make sure to run `rag summary` if you want to
    /// use this action.
    GetSummary,

    /// I'm using the term "simple" because I want to make sure that it's not an agentic RAG.
    SimpleRag,
}
