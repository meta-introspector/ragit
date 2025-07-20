use crate::index::index_struct::Index;

impl Index {
    pub fn dummy_with_version(version: String) -> Self {
        let mut dummy_index = Index::dummy();
        dummy_index.ragit_version = version;
        dummy_index
    }
}
