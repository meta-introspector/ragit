use crate::grand_plan::binary_id_tree::Uid as UidType;

/// Assigned a Uid, a time, a place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uid(pub UidType);
