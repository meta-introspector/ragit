use crate::grand_plan::abi_interface::abi_types::abi_types_enum::{AbiArgs, AbiResult, AbiValue};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_universe::to_abi_value_universe;

pub fn wrap_the_quasifiber<T: 'static + Clone + std::fmt::Debug>(func: for<'b> fn(&'b GrandUnifiedStore, &'b str, usize) -> &'b Universe<T>) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        if args.len() != 3 {
            return Err("Expected 3 arguments: &GrandUnifiedStore, &str, usize".to_string());
        }
        let type_name = match &args[1] {
            AbiValue::String(s) => s.as_str(),
            _ => return Err("Expected string for type_name".to_string()),
        };
        let size = match &args[2] {
            AbiValue::U32(n) => *n as usize,
            _ => return Err("Expected usize for size".to_string()),
        };
        let mut temp_store = GrandUnifiedStore::new();
        temp_store.add_char_store();
        temp_store.add_i64_store();
        let result = func(&temp_store, type_name, size);
        Ok(to_abi_value_universe(result))
    })
}
