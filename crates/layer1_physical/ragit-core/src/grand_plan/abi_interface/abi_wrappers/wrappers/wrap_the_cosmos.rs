use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::unified_stores::type_store_enum::TypeStore;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_type_store::to_abi_value_type_store;

pub fn wrap_the_cosmos(func: fn(&GrandUnifiedStore) -> &TypeStore) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        let mut temp_store = GrandUnifiedStore::new();
        temp_store.add_i64_store();
        let result = func(&temp_store);
        Ok(to_abi_value_type_store(result))
    })
}
