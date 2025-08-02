use crate::grand_plan::abi_interface::abi_types::AbiValue;
use crate::grand_plan::unified_stores::type_store_enum::TypeStore;

pub fn to_abi_value_type_store(store: &TypeStore) -> AbiValue {
    AbiValue::String(format!("{:?}", store))
}
