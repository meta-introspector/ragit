use crate::grand_plan::abi_interface::abi_types::abi_types_enum::{AbiArgs, AbiResult};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::binary_id_trees::universe_struct::Universe;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_universe::to_abi_value_universe;

pub fn wrap_the_tree(func: fn(&GrandUnifiedStore) -> Option<&Universe<char>>) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        let mut temp_store = GrandUnifiedStore::new();
        temp_store.add_char_store();
        let result = func(&temp_store);
        match result {
            Some(universe) => Ok(to_abi_value_universe(universe)),
            None => Err("Failed to get universe".to_string()),
        }
    })
}
