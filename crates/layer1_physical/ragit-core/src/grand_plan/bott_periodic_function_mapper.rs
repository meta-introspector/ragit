use crate::grand_plan::abi_interface::function_registry::FunctionRegistry;
use crate::grand_plan::abi_interface::abi_wrappers::{
    wrap_the_void, wrap_the_spark, wrap_the_pair, wrap_the_tree, wrap_the_cosmos,
    wrap_the_mirror, wrap_the_quasifiber, wrap_the_cycle
};
use crate::grand_plan::bott_periodic_lambdas::{
    lambda_0_the_void, lambda_1_the_spark, lambda_2_the_pair, lambda_3_the_tree,
    lambda_4_the_cosmos, lambda_5_the_mirror, lambda_6_the_quasifiber, lambda_7_the_cycle
};

/// Initializes and returns a FunctionRegistry with all Bott Periodic Lambdas registered.
pub fn get_bott_periodic_function_registry() -> FunctionRegistry {
    let mut registry = FunctionRegistry::new();

    registry.register_function(
        "the_void",
        wrap_the_void(lambda_0_the_void::the_void),
    );
    registry.register_function(
        "the_spark",
        wrap_the_spark(lambda_1_the_spark::the_spark),
    );
    registry.register_function(
        "the_pair",
        wrap_the_pair(lambda_2_the_pair::the_pair),
    );
    registry.register_function(
        "the_tree",
        wrap_the_tree(lambda_3_the_tree::the_tree),
    );
    registry.register_function(
        "the_cosmos",
        wrap_the_cosmos(lambda_4_the_cosmos::the_cosmos),
    );
    registry.register_function(
        "the_mirror",
        wrap_the_mirror(lambda_5_the_mirror::the_mirror),
    );
    // Note: wrap_the_quasifiber is generic, so we need to pick a concrete type for registration.
    // For this example, we'll register a char version.
    registry.register_function(
        "the_quasifiber_char",
        wrap_the_quasifiber(lambda_6_the_quasifiber::the_quasifiber::<char>),
    );
    registry.register_function(
        "the_cycle",
        wrap_the_cycle(lambda_7_the_cycle::the_cycle),
    );

    registry
}
