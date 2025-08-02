use crate::grand_plan::abi_interface::abi_types::abi_types_enum::AbiValue;

pub fn to_abi_value_u32(n: u32) -> AbiValue {
    AbiValue::U32(n)
}
