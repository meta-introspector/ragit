use crate::grand_plan::poem_concepts::quasifiber::quasifiber_struct::Quasifiber;
use crate::grand_plan::prompt_guides::prompt::prompt_enum::Prompt;
use crate::grand_plan::unified_stores::grand_unified_store_struct::GrandUnifiedStore;


    pub fn guide_the_quasifiber<T: Clone>(
    base_space: &crate::grand_plan::poem_concepts::base_space::base_space_struct::BaseSpace,
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
    // For now, we'll just retrieve the corresponding universe directly.
    let type_store = base_space.0.get_store(type_name).unwrap();
    let universe = match type_store {
        crate::grand_plan::unified_stores::type_store_enum::TypeStore::Char(s) => unsafe {
            &*(s.get_by_size(size).unwrap() as *const _ as *const crate::grand_plan::binary_id_trees::universe_struct::Universe<T>)
        },
        crate::grand_plan::unified_stores::type_store_enum::TypeStore::I64(s) => unsafe {
            &*(s.get_by_size(size).unwrap() as *const _ as *const crate::grand_plan::binary_id_trees::universe_struct::Universe<T>)
        },
    };

        Quasifiber((*universe).clone())

}
