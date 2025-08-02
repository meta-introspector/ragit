use std::collections::HashMap;
use crate::grand_plan::abi_interface::abi_types::abi_types_enum::{AbiArgs, AbiResult, AbiValue};

#[derive(Default)]
pub struct FunctionRegistry {
    pub functions: HashMap<String, AbiFunction>,
}

pub type AbiFunction = Box<dyn Fn(AbiArgs) -> AbiResult + Send + Sync>;

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
