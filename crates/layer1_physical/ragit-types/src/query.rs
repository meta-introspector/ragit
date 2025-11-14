use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QueryConfig {
    pub enable_rag: bool,
    pub enable_ii: bool,
    pub max_summaries: usize,
    pub max_retrieval: usize,
    pub super_rerank: bool,
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            enable_rag: true,
            enable_ii: true,
            max_summaries: 10,
            max_retrieval: 3,
            super_rerank: false,
        }
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartialQueryConfig {
    pub enable_rag: Option<bool>,
    pub enable_ii: Option<bool>,
    pub max_summaries: Option<usize>,
    pub max_retrieval: Option<usize>,
    pub super_rerank: Option<bool>,
}

impl PartialQueryConfig {
    pub fn apply_to(&self, config: &mut QueryConfig) {
        if let Some(enable_rag) = self.enable_rag {
            config.enable_rag = enable_rag;
        }
        if let Some(enable_ii) = self.enable_ii {
            config.enable_ii = enable_ii;
        }
        if let Some(max_summaries) = self.max_summaries {
            config.max_summaries = max_summaries;
        }
        if let Some(max_retrieval) = self.max_retrieval {
            config.max_retrieval = max_retrieval;
        }
        if let Some(super_rerank) = self.super_rerank {
            config.super_rerank = super_rerank;
        }
    }
}
