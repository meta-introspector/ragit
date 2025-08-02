use crate::grand_plan::abi_interface::abi_types::abi_types_enum::AbiValue;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;

pub fn to_abi_value_grand_unified_store(store: &GrandUnifiedStore) -> AbiValue {
    AbiValue::String(format!("{:?}", store))
}
