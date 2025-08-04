use crate::grand_plan::bootstrapper::initializers::initialize_bootstrap_components::BootstrapComponents;
use crate::grand_plan::ragit_chunk_integration::module_ingestor::ingest_grand_plan_modules;

pub fn ingest_grand_plan_modules_as_ragit_chunks(
    bootstrap_components: &mut BootstrapComponents,
) {
    println!("Phase 2: Ingesting Grand Plan Modules as Ragit Chunks.");
    let grand_plan_chunks = ingest_grand_plan_modules();
    for chunk in grand_plan_chunks {
        bootstrap_components.living_chunk_manager.add_active_chunk(chunk);
    }
}
