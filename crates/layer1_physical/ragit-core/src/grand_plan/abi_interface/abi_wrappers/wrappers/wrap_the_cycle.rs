use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult, AbiValue};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_grand_unified_store::to_abi_value_grand_unified_store;

pub fn wrap_the_cycle(func: fn(&str, usize) -> GrandUnifiedStore) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        if args.len() != 2 {
            return Err("Expected 2 arguments: &str, usize".to_string());
        }
        let type_name = match &args[0] {
            AbiValue::String(s) => s.as_str(),
            _ => return Err("Expected string for type_name".to_string()),
        };
        let size = match &args[1] {
            AbiValue::U32(n) => *n as usize,
            _ => return Err("Expected usize for size".to_string()),
        };
        let result = func(type_name, size);
        Ok(to_abi_value_grand_unified_store(&result))
    })
}
