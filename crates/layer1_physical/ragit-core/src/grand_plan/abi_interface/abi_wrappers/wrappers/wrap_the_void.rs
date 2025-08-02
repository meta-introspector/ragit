use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult};
use crate::grand_plan::abi_interface::function_registry::function_registry_struct::AbiFunction;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;
use crate::grand_plan::abi_interface::abi_wrappers::helpers::to_abi_value_grand_unified_store::to_abi_value_grand_unified_store;

pub fn wrap_the_void(func: fn() -> GrandUnifiedStore) -> AbiFunction {
    Box::new(move |_args: AbiArgs| -> AbiResult {
        let result = func();
        Ok(to_abi_value_grand_unified_store(&result))
    })
}
