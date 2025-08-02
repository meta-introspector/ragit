use crate::grand_plan::abi_interface::abi_types::abi_types_enum::AbiValue;
use crate::grand_plan::sized_universe_stores::sized_universe_store_struct::SizedUniverseStore;

pub fn to_abi_value_sized_universe_store<T: std::fmt::Debug>(store: &SizedUniverseStore<T>) -> AbiValue {
    AbiValue::String(format!("{:?}", store))
}
