use crate::grand_plan::abi_interface::abi_types::abi_types_enum::AbiValue;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;

pub fn to_abi_value_universe<T: std::fmt::Debug>(universe: &Universe<T>) -> AbiValue {
    AbiValue::String(format!("{:?}", universe))
}
