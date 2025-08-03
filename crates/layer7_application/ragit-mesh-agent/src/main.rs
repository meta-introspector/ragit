//! Ragit Mesh Agent
//!
//! This agent connects to a p2p network of other agents, using a private
//! Solana validator for peer discovery.

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Settings {
    solana_rpc_url: String,
    keypair_path: String,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("crates/layer7_application/ragit-mesh-agent/mesh-config.toml"))
            .build()?;
        s.try_deserialize()
    }
}

fn main() {
    println!("Starting Ragit Mesh Agent...");

    // 1. Load configuration from `mesh-config.toml`.
    let settings = match Settings::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            return;
        }
    };
    println!("Loaded settings: {:?}", settings);

    // 2. Initialize the `libp2p` transport and swarm.

    // 3. Connect to the Solana RPC endpoint.

    // 4. Register this agent's `libp2p` multi-address on the Solana validator.

    // 5. Query the validator for a list of other peers.

    // 6. Tell the `libp2p` swarm to dial the other peers.

    // 7. Start the main event loop to handle incoming connections and messages.

    println!("Ragit Mesh Agent shutting down.");
}
