# Architecture: Hybrid Storage Model

This document outlines the architecture for the `ragit` ecosystem's hybrid storage model. This model is designed to use the right tool for the right job, balancing on-chain verifiability with the efficiency and specific advantages of off-chain storage solutions like Git and Hugging Face Datasets.

## Core Concept

Every piece of content—a document, a chunk, a meme—is represented by a canonical account on the Solana sidechain. This on-chain account acts as the central, immutable pointer and metadata store. However, the full content payload can reside in one of several locations, depending on its nature.

The on-chain account for a meme will always contain its UID (content address), its vector(s), and a **location pointer**. This pointer indicates which storage backend holds the actual data.

## The Storage Backends

1.  **On-Chain (Solana Account/PDA):**
    *   **Use Case:** For high-value, small-footprint data. This includes metadata, consensus state, trigger flags, and short text snippets.
    *   **Implementation:** The data is stored directly within the PDA's account data field.
    *   **Advantages:** Fully decentralized, atomic, and verifiable on-chain.

2.  **Off-Chain Versioned (Git):**
    *   **Use Case:** For human-readable, version-controlled content like source code, architectural documents, and markdown files.
    *   **Implementation:** The on-chain account's location pointer will contain a Git URL and a specific commit hash.
    *   **Advantages:** Leverages the robust versioning, branching, and collaborative features of Git. Content is human-readable and easily auditable.

3.  **Off-Chain Bulk (Hugging Face Datasets):**
    *   **Use Case:** For large-scale data, such as AI models, large datasets, image collections, and other binary assets.
    *   **Implementation:** The on-chain account's location pointer will contain a Hugging Face repository URL and a file identifier.
    *   **Advantages:** Optimized for handling large files and datasets, with built-in features for data exploration and loading.

## Workflow Example

1.  A user wants to add a new source code file to the `ragit` knowledge base.
2.  The user commits and pushes the file to a designated Git repository.
3.  The user (or an automated agent) submits a transaction to the Solana sidechain.
4.  The transaction creates a new PDA for the file. The PDA's data includes:
    *   The file's UID (its content hash).
    *   Its consensus-derived vector representation.
    *   A location pointer: `{"backend": "git", "url": "<repo_url>", "commit": "<commit_hash>"}`.
5.  When another agent needs to access this file, it first queries the on-chain PDA to get the location pointer, then fetches the content directly from the specified Git commit.

## Advantages of the Hybrid Model

*   **Flexibility:** Choose the optimal storage solution for each data type.
*   **Scalability:** Avoids bloating the Solana chain with large, expensive data blobs.
*   **Verifiability:** The on-chain pointer provides an immutable, auditable link between a meme's identity and its content, regardless of where that content is stored.
