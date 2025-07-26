use crate::index_struct::Index;
use ragit_types::summary::Summary;

impl Index {
    pub fn get_summary(&self) -> Option<&str> {
        self.summary.as_ref().map(|s| s.content.as_str())
    }
}