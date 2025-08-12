use crate::grand_plan::bootstrapper::bootstrap_orchestrator::bootstrap_orchestrator_struct::BootstrapOrchestrator;

pub struct ToolchainBootstrappingComponents {
    pub bootstrap_orchestrator: BootstrapOrchestrator,
}

pub fn initialize_toolchain_bootstrapping_components() -> ToolchainBootstrappingComponents {
    let bootstrap_orchestrator = BootstrapOrchestrator::new();

    ToolchainBootstrappingComponents {
        bootstrap_orchestrator,
    }
}
