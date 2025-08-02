use crate::grand_plan::abi_interface::abi_types::{AbiArgs, AbiResult, AbiValue};
use crate::grand_plan::abi_interface::function_registry::AbiFunction;
use crate::grand_plan::unified_store::GrandUnifiedStore;
use crate::grand_plan::sized_universe_store::SizedUniverseStore;
use crate::grand_plan::unified_store::TypeStore;
use crate::grand_plan::binary_id_tree::Universe;

// Helper to convert a GrandUnifiedStore to AbiValue
// NOTE: This requires GrandUnifiedStore to be Serialize/Deserialize.
// For simplicity, we'll represent it as a string for now.
pub fn to_abi_value_grand_unified_store(store: &GrandUnifiedStore) -> AbiValue {
    AbiValue::String(format!("{:?}", store))
}

// Helper to convert a SizedUniverseStore to AbiValue
pub fn to_abi_value_sized_universe_store<T>(store: &SizedUniverseStore<T>) -> AbiValue {
    AbiValue::String(format!("{:?}", store))
}

// Helper to convert a TypeStore to AbiValue
pub fn to_abi_value_type_store(store: &TypeStore) -> AbiValue {
    AbiValue::String(format!("{:?}", store))
}

// Helper to convert a Universe to AbiValue
pub fn to_abi_value_universe<T>(universe: &Universe<T>) -> AbiValue {
    AbiValue::String(format!("{:?}", universe))
}

// Helper to convert char to AbiValue
pub fn to_abi_value_char(c: char) -> AbiValue {
    AbiValue::String(c.to_string())
}

// Helper to convert u32 to AbiValue
pub fn to_abi_value_u32(n: u32) -> AbiValue {
    AbiValue::U32(n)
}

// Wrapper for lambda_0_the_void
pub fn wrap_the_void(func: fn() -> GrandUnifiedStore) -> AbiFunction {
    Box::new(move |_args: AbiArgs| -> AbiResult {
        let result = func();
        Ok(to_abi_value_grand_unified_store(&result))
    })
}

// Wrapper for lambda_1_the_spark
pub fn wrap_the_spark(func: fn(usize) -> char) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        if args.len() != 1 {
            return Err("Expected 1 argument: usize".to_string());
        }
        let i = match &args[0] {
            AbiValue::U32(n) => *n as usize,
            _ => return Err("Expected usize argument".to_string()),
        };
        let result = func(i);
        Ok(to_abi_value_char(result))
    })
}

// Wrapper for lambda_2_the_pair
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

// Wrapper for lambda_3_the_tree
pub fn wrap_the_tree(func: fn(&GrandUnifiedStore) -> &SizedUniverseStore<char>) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        let mut temp_store = GrandUnifiedStore::new();
        temp_store.add_char_store();
        let result = func(&temp_store);
        Ok(to_abi_value_sized_universe_store(result))
    })
}

// Wrapper for lambda_4_the_cosmos
pub fn wrap_the_cosmos(func: fn(&GrandUnifiedStore) -> &TypeStore) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        let mut temp_store = GrandUnifiedStore::new();
        temp_store.add_i64_store();
        let result = func(&temp_store);
        Ok(to_abi_value_type_store(result))
    })
}

// Wrapper for lambda_5_the_mirror
pub fn wrap_the_mirror(func: fn(&GrandUnifiedStore, &str) -> Option<&TypeStore>) -> AbiFunction {
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

// Wrapper for lambda_6_the_quasifiber
pub fn wrap_the_quasifiber<T: 'static + Clone>(func: fn(&GrandUnifiedStore, &str, usize) -> &Universe<T>) -> AbiFunction {
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

// Wrapper for lambda_7_the_cycle
pub fn wrap_the_cycle(func: fn(&str, usize) -> GrandUnifiedStore) -> AbiFunction {
    Box::new(move |args: AbiArgs| -> AbiResult {
        if args.len() != 2 {
            return Err("Expected 2 arguments: &str, usize".to_string());
        }
        let type_name = match &args[0] {
            AbiValue::String(s) => s.as_str(),
            _ => return Err("Expected string for type_name".to_string()),
        };
        let size = match &args[1] {
            AbiValue::U32(n) => *n as usize,
            _ => return Err("Expected usize for size".to_string()),
        };
        let result = func(type_name, size);
        Ok(to_abi_value_grand_unified_store(&result))
    })
}
