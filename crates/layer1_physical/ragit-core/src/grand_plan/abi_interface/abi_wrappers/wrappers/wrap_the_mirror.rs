use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult, AbiValue};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::unified_stores::type_store_enum::TypeStore;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_type_store::to_abi_value_type_store;

pub fn wrap_the_mirror<'a>(func: for<'b> fn(&'b GrandUnifiedStore, &'b str) -> Option<&'b TypeStore>) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        if args.len() != 2 {
            return Err("Expected 2 arguments: &GrandUnifiedStore, &str".to_string());
        }
        let type_name = match &args[1] {
            AbiValue::String(s) => s.as_str(),
            _ => return Err("Expected string for type_name".to_string()),
        };
        let mut temp_store = GrandUnifiedStore::new();
        temp_store.add_char_store();
        temp_store.add_i64_store();
        let result = func(&temp_store, type_name);
        match result {
            Some(store) => Ok(to_abi_value_type_store(store)),
            None => Err(format!("Type store for '{}' not found", type_name)),
        }
    })
}
