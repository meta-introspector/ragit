# Architecture: Decentralized Consensus-Driven Vector Embeddings

This document outlines the architecture for a system that generates and stores its own vector embeddings through a decentralized consensus mechanism on a Solana sidechain.

## Core Concept

Instead of relying on a centralized, pre-trained model (e.g., OpenAI, Sentence Transformers) to generate vector embeddings, the `ragit` network will collaboratively compute, vote on, and agree upon the vector for each piece of content (a "meme" or "chunk").

This decentralizes the creation of semantic meaning itself. The "vibe" of a meme is not dictated by a single authority but is an emergent property of the network's consensus.

## Key Features

1.  **Multi-Dimensional Representations:**
    *   Each meme will be represented by multiple vectors, each in a different dimension space.
    *   The specified dimensions are **8D, 23D, 41D, and 800D**, chosen for their significance within the project's numerology and to capture meaning at different levels of granularity.

2.  **On-Chain Quantization:**
    *   The system will support vector quantization (e.g., converting `f32` vectors to `int8`).
    *   The quantization process and the quantized vector itself will also be managed and stored on-chain, allowing for significant storage savings and performance gains during similarity searches.

3.  **Consensus-Driven Generation:**
    *   The value of a vector is determined by an on-chain, propose-and-vote governance mechanism.
    *   **Proposal:** Any agent in the network can propose a vector for a given meme by submitting a transaction to a specific on-chain program.
    *   **Voting:** Other agents can then vote on these proposals. Votes could be weighted by reputation, stake, or other metrics to ensure quality.
    *   **Agreement:** When a proposal reaches a predetermined consensus threshold, the on-chain program ratifies it as the canonical vector for that meme in that specific dimension.

4.  **Canonical On-Chain Storage (PDA):**
    *   Each canonical vector is stored in its own unique, content-derived Solana account.
    *   **Program Derived Address (PDA):** The address for a vector's account is a PDA derived from seeds that include the vector's dimension and the unique content address (UID/hash) of the meme it represents.
    *   Example PDA seeds: `["vector", "8D", <UID_of_meme>]`
    *   This creates a permanent, immutable, and globally accessible link between a piece of content and its network-approved vector representation.

## Workflow Example

1.  A new text chunk is added to `ragit` and is assigned a unique UID (its content address).
2.  An agent on the network processes this chunk and proposes an 8D vector for it via a Solana transaction.
3.  The on-chain "Vector Consensus Program" creates a new proposal account.
4.  Other agents see this proposal. They can either vote to approve it or submit their own competing proposals.
5.  After a set voting period, the proposal with the most votes is declared the winner.
6.  The consensus program writes the winning 8D vector into the canonical PDA: `PDA("vector", "8D", <UID>)`.
7.  This process is repeated for the 23D, 41D, and 800D vectors.

## Advantages

*   **Truly Decentralized AI:** The system creates its own understanding of the world, free from the biases of any single model provider.
*   **Evolving Meaning:** The meaning of concepts can evolve over time as the network's understanding deepens and new votes are cast.
*   **Censorship Resistance:** No single entity can control the semantic representation of information.
