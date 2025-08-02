use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::sized_universe_stores::sized_universe_store_struct::SizedUniverseStore;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_sized_universe_store::to_abi_value_sized_universe_store;

pub fn wrap_the_pair(func: fn(&GrandUnifiedStore) -> &SizedUniverseStore<char>) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        // This wrapper assumes the GrandUnifiedStore is passed in a specific way,
        // or is globally accessible. For now, we'll use a placeholder.
        // In a real system, you'd deserialize the store from AbiValue.
        let mut temp_store = GrandUnifiedStore::new();
        temp_store.add_char_store(); // Ensure it has the char store for the example
        let result = func(&temp_store);
        Ok(to_abi_value_sized_universe_store(result))
    })
}
