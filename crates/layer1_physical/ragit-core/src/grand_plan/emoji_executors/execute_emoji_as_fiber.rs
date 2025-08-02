use crate::grand_plan::abi_interface::abi_types::abi_types_enum::{AbiArgs, AbiResult};
use crate::grand_plan::bott_periodic_function_mappers::get_bott_periodic_function_registry::get_bott_periodic_function_registry;
use crate::grand_plan::semantic_lambdas::get_bott_periodic_lambdas::get_bott_periodic_lambdas;

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Executes the Rust function associated with a given emoji.
pub fn execute_emoji_as_fiber(emoji: char, args: AbiArgs) -> AbiResult {
    let registry = get_bott_periodic_function_registry();
    let lambdas = get_bott_periodic_lambdas();

    if let Some(semantic_lambda) = lambdas.iter().find(|&sl| sl.emoji == emoji) {
        registry.call_function(semantic_lambda.name, args)
    } else {
        Err(format!("No function registered for emoji: {}", emoji))
    }
}
