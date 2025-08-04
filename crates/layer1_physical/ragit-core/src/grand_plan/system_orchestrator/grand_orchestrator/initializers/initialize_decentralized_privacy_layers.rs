use crate::grand_plan::gossip_system::gossip_network::gossip_network_struct::GossipNetwork;
use crate::grand_plan::introspector_sidechain::introspector_sidechain::introspector_sidechain_struct::IntrospectorSidechain;
use crate::grand_plan::privacy_and_scaling::federated_learning::federated_learning_structs::FederatedLearningServer;
use crate::grand_plan::privacy_and_scaling::homomorphic_encryption::homomorphic_encryption_structs::HomomorphicEncryption;
use crate::grand_plan::privacy_and_scaling::zero_knowledge_proofs::zero_knowledge_proofs_structs::ZeroKnowledgeProofs;
use crate::grand_plan::privacy_and_scaling::lattice_rollups::lattice_rollups_structs::LatticeRollups;

pub struct DecentralizedPrivacyLayerComponents {
    pub gossip_network: GossipNetwork,
    pub introspector_sidechain: IntrospectorSidechain,
    pub fl_server: FederatedLearningServer,
    pub hme_system: HomomorphicEncryption,
    pub zkp_system: ZeroKnowledgeProofs,
    pub lattice_rollup_system: LatticeRollups,
}

pub fn initialize_decentralized_privacy_layers() -> DecentralizedPrivacyLayerComponents {
    let gossip_network = GossipNetwork::new();
    let introspector_sidechain = IntrospectorSidechain::new();
    let fl_server = FederatedLearningServer::new(vec![0.0; 128]); // Initial model weights
    let hme_system = HomomorphicEncryption::new();
    let zkp_system = ZeroKnowledgeProofs::new();
    let lattice_rollup_system = LatticeRollups::new();

    DecentralizedPrivacyLayerComponents {
        gossip_network,
        introspector_sidechain,
        fl_server,
        hme_system,
        zkp_system,
        lattice_rollup_system,
    }
}
