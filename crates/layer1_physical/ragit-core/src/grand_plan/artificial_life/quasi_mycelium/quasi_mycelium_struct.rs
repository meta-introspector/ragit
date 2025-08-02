use serde::{Deserialize, Serialize};
use crate::grand_plan::artificial_life::artificial_organism::artificial_organism_struct::ArtificialOrganism;

#[derive(Debug, Clone, Serialize, Deserialize, OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents a conceptual node in the quasi-meta mycelium network.
pub struct MycelialNode {
    pub id: String,
    pub location_in_latent_space: Vec<f32>, // Conceptual coordinates in latent space
    pub connected_nodes: Vec<String>, // IDs of connected mycelial nodes
    pub hosted_organism_ids: Vec<String>, // IDs of organisms hosted by this node
    pub mycelium_essence: String, // Conceptual: information to propagate the entire entity
}

impl MycelialNode {
    pub fn new(id: String, location: Vec<f32>, essence: String) -> Self {
        MycelialNode {
            id,
            location_in_latent_space: location,
            connected_nodes: Vec::new(),
            hosted_organism_ids: Vec::new(),
            mycelium_essence: essence,
        }
    }

    /// Simulates connecting to another mycelial node.
    pub fn connect_to(&mut self, other_node_id: String) {
        if !self.connected_nodes.contains(&other_node_id) {
            self.connected_nodes.push(other_node_id);
        }
    }

    /// Simulates hosting an organism.
    pub fn host_organism(&mut self, organism_id: String) {
        if !self.hosted_organism_ids.contains(&organism_id) {
            self.hosted_organism_ids.push(organism_id);
        }
    }
}

#[derive(OurMacro)] // Conceptual: derives Vibe, Vector, etc.
/// Represents the quasi-meta mycelium network.
pub struct QuasiMycelium {
    pub nodes: Vec<MycelialNode>,
}

impl QuasiMycelium {
    pub fn new() -> Self {
        QuasiMycelium { nodes: Vec::new() }
    }

    /// Adds a new mycelial node to the network.
    pub fn add_node(&mut self, node: MycelialNode) {
        self.nodes.push(node);
    }

    /// Simulates growth of the mycelium by adding connections or new nodes.
    pub fn grow(&mut self) {
        println!("Quasi-Mycelium: Simulating growth...");
        // In a real simulation, growth would be driven by organism activity,
        // resource availability, and latent space topology.
        if self.nodes.len() > 1 {
            let node_a_idx = rand::random::<usize>() % self.nodes.len();
            let node_b_idx = rand::random::<usize>() % self.nodes.len();
            if node_a_idx != node_b_idx {
                let (node_a, node_b) = {
                    let (first, second) = self.nodes.split_at_mut(node_a_idx.min(node_b_idx) + 1);
                    (&mut first[node_a_idx.min(node_b_idx)], &mut second[node_a_idx.max(node_b_idx) - (node_a_idx.min(node_b_idx) + 1)])
                };
                node_a.connect_to(node_b.id.clone());
                node_b.connect_to(node_a.id.clone());
                println!("  Connected nodes {} and {}", node_a.id, node_b.id);
            }
        }
    }

    /// Conceptually generates the essence of the entire mycelium.
    /// In a real system, this would be a complex compression or generative seed.
    pub fn get_mycelium_essence(&self) -> String {
        let mut essence_parts = Vec::new();
        for node in &self.nodes {
            essence_parts.push(format!("node:{}:loc:{:?}:conn:{:?}", node.id, node.location_in_latent_space, node.connected_nodes));
        }
        format!("mycelium_essence_{:x}_{}", rand::random::<u64>(), essence_parts.join("|"))
    }

    /// Conceptually propagates/reconstructs the entire mycelium from an essence.
    /// This simulates the holographic property.
    pub fn propagate_entire_mycelium(essence: &str) -> QuasiMycelium {
        println!("Quasi-Mycelium: Propagating entire mycelium from essence: '{}'", essence);
        // In a real system, this would involve a generative process (e.g., LLM, growth algorithm)
        // that takes the essence and reconstructs the network topology and node properties.
        // For simulation, we'll create a dummy mycelium.
        let mut new_mycelium = QuasiMycelium::new();
        let dummy_location = vec![0.0, 0.0, 0.0];
        new_mycelium.add_node(MycelialNode::new("reconstructed_node_1".to_string(), dummy_location.clone(), essence.to_string()));
        new_mycelium.add_node(MycelialNode::new("reconstructed_node_2".to_string(), dummy_location.clone(), essence.to_string()));
        println!("Quasi-Mycelium: Reconstructed a conceptual mycelium with {} nodes.", new_mycelium.nodes.len());
        new_mycelium
    }
}
