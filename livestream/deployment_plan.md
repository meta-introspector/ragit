# `ragit-mesh-agent` Deployment Plan

This document outlines the technical plan for creating and deploying a command to enable a p2p mesh network between Termux devices, using a private Solana validator for discovery and coordination.

## Phase 1: The Agent Crate

-   **Action:** Create a new binary crate named `ragit-mesh-agent`.
-   **Purpose:** This will be the core application that runs on each device in the mesh network.
-   **Configuration:** It will be configured via a `mesh-config.toml` file to specify the Solana RPC endpoint, peer identity keys, and other settings.

## Phase 2: P2P Communication Layer (`libp2p`)

-   **Technology:** We will use the `libp2p` library for direct peer-to-peer communication.
-   **Solana's Role (Discovery):** We will NOT use Solana for real-time message passing. Instead, it will function as a decentralized address book.
    1.  An agent starts up and generates its `libp2p` peer ID and multi-address.
    2.  The agent sends a transaction to a custom program on our private Solana validator to register its address.
    3.  The agent queries the Solana program to get a list of other registered, active peers.
    4.  The agent uses `libp2p` to connect directly to the peers from the list.

## Phase 3: P2P Storage Layer

-   **Technology:** A combination of a simple `libp2p` protocol and the Solana chain.
-   **Workflow:**
    1.  To "store" or share a file, a peer announces the file's metadata (name, size, hash) and its own `libp2p` address via a Solana transaction.
    2.  Other peers see this transaction and can initiate a direct `libp2p` connection to the announcing peer to request and receive the file data.
    3.  This avoids storing large, expensive data blobs on-chain while still providing a decentralized way to announce and discover available data.

## Phase 4: Deployment

-   **Compilation:** The agent will be compiled into a static binary using `cargo build --release --target=aarch64-linux-android`.
-   **Transfer:** The initial deployment method will be manual:
    -   Using `scp`, `adb push`, or a simple file transfer app to move the compiled binary and the `mesh-config.toml` file to the target phone.
-   **Execution:** The agent will be run directly from the Termux shell on the target device.
