use std::collections::HashMap;
use crate::grand_plan::lean_interface::lean_types::lean_types_enum::{LeanArgs, LeanResult, LeanValue};

#[derive(Default)]
pub struct FunctionRegistry {
    pub functions: HashMap<String, LeanFunction>,
}

pub type LeanFunction = Box<dyn Fn(LeanArgs) -> LeanResult + Send + Sync>;

impl FunctionRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new function with the registry.
    pub fn register_function(&mut self, name: &str, func: LeanFunction) {
        self.functions.insert(name.to_string(), func);
    }

    /// Calls a registered function by its name.
    pub fn call_function(&self, name: &str, args: LeanArgs) -> LeanResult {
        if let Some(func) = self.functions.get(name) {
            func(args)
        } else {
            Err(format!("Function '{}' not found", name))
        }
    }
}
