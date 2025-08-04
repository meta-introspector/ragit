use crate::grand_plan::abi_interface::abi_types::abi_types_enum::{AbiArgs, AbiResult, AbiValue};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_char::to_abi_value_char;

pub fn wrap_the_spark(func: fn(usize) -> char) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        if args.len() != 1 {
            return Err("Expected 1 argument: usize".to_string());
        }
        let i = match &args[0] {
            AbiValue::U32(n) => *n as usize,
            _ => return Err("Expected usize argument".to_string()),
        };
        let result = func(i);
        Ok(to_abi_value_char(result))
    })
}
