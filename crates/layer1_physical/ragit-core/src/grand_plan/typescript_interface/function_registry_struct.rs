use std::collections::HashMap;
use crate::grand_plan::typescript_interface::typescript_types::typescript_types_enum::{TypeScriptArgs, TypeScriptResult};

#[derive(Default)]
pub struct FunctionRegistry {
    pub functions: HashMap<String, TypeScriptFunction>,
}

pub type TypeScriptFunction = Box<dyn Fn(TypeScriptArgs) -> TypeScriptResult + Send + Sync>;

impl FunctionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new function with the registry.
    pub fn register_function(&mut self, name: &str, func: TypeScriptFunction) {
        self.functions.insert(name.to_string(), func);
    }

    /// Calls a registered function by its name.
    pub fn call_function(&self, name: &str, args: TypeScriptArgs) -> TypeScriptResult {
        if let Some(func) = self.functions.get(name) {
            func(args)
        } else {
            Err(format!("Function '{}' not found", name))
        }
    }
}
