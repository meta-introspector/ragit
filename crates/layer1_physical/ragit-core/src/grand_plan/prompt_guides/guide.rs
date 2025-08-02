use crate::grand_plan::poem_concepts::base_space::BaseSpace;
use crate::grand_plan::poem_concepts::quasifiber::Quasifiber;
use crate::grand_plan::prompt_guides::prompt::Prompt;

/// Demonstrates how a Prompt guides the selection of a Quasifiber from the BaseSpace.
/// This is a conceptual function; the actual logic would be far more complex.
pub fn guide_the_quasifiber<T>(
    base_space: &BaseSpace,
    prompt: &Prompt,
    type_name: &str,
    size: usize,
) -> Quasifiber<T> {
    println!(
        "Guiding the selection of a Quasifiber of type '{}' and size '{}' using the {:?} prompt.",
        type_name,
        size,
        prompt
    );

    // In a real system, the prompt would influence the selection process.
    // For now, we just retrieve the corresponding universe directly.
    let type_store = base_space.0.get_store(type_name).unwrap();
    let universe = match type_store {
        crate::grand_plan::unified_store::TypeStore::Char(s) => unsafe {
            &*(s.get_by_size(size).unwrap() as *const _ as *const crate::grand_plan::binary_id_tree::Universe<T>)
        },
        crate::grand_plan::unified_store::TypeStore::I64(s) => unsafe {
            &*(s.get_by_size(size).unwrap() as *const _ as *const crate::grand_plan::binary_id_tree::Universe<T>)
        },
    };

    Quasifiber(universe.clone())
}
