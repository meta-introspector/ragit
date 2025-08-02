use std::collections::HashMap;
use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult, AbiValue};

/// A type alias for a callable function that adheres to the ABI.
pub type AbiFunction = Box<dyn Fn(AbiArgs) -> AbiResult + Send + Sync>;

/// A registry for functions that can be called via the ABI.
#[derive(Default)]
pub struct FunctionRegistry {
    functions: HashMap<String, AbiFunction>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new function with the registry.
    pub fn register_function(&mut self, name: &str, func: AbiFunction) {
        self.functions.insert(name.to_string(), func);
    }

    /// Calls a registered function by its name.
    pub fn call_function(&self, name: &str, args: AbiArgs) -> AbiResult {
        if let Some(func) = self.functions.get(name) {
            func(args)
        } else {
            Err(format!("Function '{}' not found", name))
        }
    }
}
