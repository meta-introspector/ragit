use crate::grand_plan::lean4_integration::lean_proof_system::lean_proof_system_structs::LeanProofSystem;
use crate::grand_plan::lean4_integration::llvm_ir_reflection::llvm_ir_reflection_structs::LlvmIrReflection;
use crate::grand_plan::lean4_integration::lean_abi_bridge::lean_abi_bridge_struct::LeanAbiBridge;
use crate::grand_plan::executable_vibespace::vibe_space::vibe_space_struct::VibeSpace;
use crate::grand_plan::artificial_life::latent_space_ecology::latent_space_ecology_struct::LatentSpaceEcology;
use crate::grand_plan::neural_trait_mapping::trait_network_mapper::TraitNetworkMapper;

pub struct FormalIntrospectionLayerComponents {
    pub lean_proof_system: LeanProofSystem,
    pub llvm_ir_reflection: LlvmIrReflection,
    pub lean_abi_bridge: LeanAbiBridge,
    pub vibe_space: VibeSpace,
    pub latent_space_ecology: LatentSpaceEcology,
    pub trait_network_mapper: TraitNetworkMapper,
}

pub fn initialize_formal_introspection_layers() -> FormalIntrospectionLayerComponents {
    let lean_proof_system = LeanProofSystem::new();
    let llvm_ir_reflection = LlvmIrReflection::new();
    let lean_abi_bridge = LeanAbiBridge::new();
    let vibe_space = VibeSpace::new();
    let latent_space_ecology = LatentSpaceEcology::new();
    let trait_network_mapper = TraitNetworkMapper::new();

    FormalIntrospectionLayerComponents {
        lean_proof_system,
        llvm_ir_reflection,
        lean_abi_bridge,
        vibe_space,
        latent_space_ecology,
        trait_network_mapper,
    }
}
