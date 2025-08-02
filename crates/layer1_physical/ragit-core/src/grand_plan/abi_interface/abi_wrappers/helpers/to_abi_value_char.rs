use crate::grand_plan::abi_interface::abi_types::AbiValue;

pub fn to_abi_value_char(c: char) -> AbiValue {
    AbiValue::String(c.to_string())
}
