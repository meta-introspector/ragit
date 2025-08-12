# Architecture: Decentralized Tweet Consensus and Posting System

This document outlines the architecture for a decentralized system that proposes, approves, and strategically posts social media messages (e.g., tweets) based on community consensus and real-time market analysis.

## Core Concept

This system transforms social media strategy into a decentralized, autonomous operation. The community, not a central manager, proposes content. The network, not an individual, approves it. And an AI, not a human, decides the perfect moment to post it to maximize impact.

Every proposed tweet is a "meme": a message with a vibe, represented by a vector, that can be analyzed and acted upon by the system.

## The Lifecycle of a Tweet

1.  **Proposal:**
    *   Any agent or user on the network can propose a tweet by submitting its text to an on-chain "Tweet Proposal Program."
    *   The program treats the tweet text as a meme, assigning it a unique, content-addressed UID.

2.  **Vectorization & Vibe Analysis:**
    *   The system uses the **Consensus-Driven Vector Embeddings** mechanism to generate a vector for the proposed tweet. This vector represents its "vibe" (e.g., bullish, technical, humorous).

3.  **Approval Process:**

    *   **Phase 1: Manual Approval via GitHub Flow (Initial Implementation)**
        *   To begin, the approval process will be managed directly within the project's GitHub repository to ensure safety, transparency, and control.
        *   **Proposal:** A user proposes a new tweet by creating a new file in a dedicated directory (e.g., `tweet_proposals/pending/`). The content of the file is the tweet text, and the filename can be a descriptive slug.
        *   **Review:** The user submits this new file as a Pull Request (PR). The PR itself serves as the forum for discussion and review.
        *   **Approval:** To approve the tweet, a project maintainer merges the PR into the main branch.
        *   **Execution:** The merge event triggers a **GitHub Action**. This action is responsible for:
            1.  Reading the content of the newly merged tweet file.
            2.  Passing the tweet text to the **Timing and Execution Engine**, which adds it to the pool of approved tweets awaiting the optimal posting time.
            3.  (Optional) Moving the file from the `pending` to an `approved` directory to maintain a clean record.
        *   This provides a simple, secure, and fully auditable starting point using familiar developer tools.

    *   **Phase 2: Decentralized Consensus (Future Goal)**
        *   The system will evolve to a full consensus model.
        *   The tweet proposal is put to a vote on the Solana sidechain.
        *   Agents on the network vote on the proposal. A proposal is considered "approved" when it reaches a consensus threshold.

4.  **Market Analysis & Timing:**
    *   **Market Data Oracle:** A dedicated service (which could be an on-chain program or a trusted off-chain agent) continuously feeds real-time market data onto the Solana chain. This data includes crypto prices, trading volume, social media sentiment, etc. This stream of data is also vectorized to represent the current "market vibe."
    *   **Timing and Execution Engine:** A service (ideally an off-chain agent authorized by the on-chain program) constantly compares the vectors of all *approved* tweets with the current market vector.

5.  **Execution:**
    *   When the vibe of an approved tweet closely matches the vibe of the market (i.e., the distance between their vectors is below a certain epsilon), the Timing and Execution Engine is triggered.
    *   The engine sends a transaction to the on-chain program to mark the tweet as "posted."
    *   Simultaneously, it calls a secure, off-chain **Poster Agent** (which holds the necessary API keys) to post the tweet's text to the target social media platform (e.g., X/Twitter).

## Key Components

*   **Tweet Proposal Program (On-Chain):** Manages the state of all tweet proposals (proposed, approved, posted, rejected).
*   **Vector Consensus Program (On-Chain):** The governance mechanism for determining a tweet's vector.
*   **Market Data Oracle (On/Off-Chain):** Provides the real-time market data feed.
*   **Timing and Execution Engine (Off-Chain):** The AI brain that matches tweets to market conditions.
*   **Secure Poster Agent (Off-Chain):** The trusted component that holds social media credentials and executes the final post.

## Advantages

*   **Decentralized Content Strategy:** The community controls the message.
*   **Market-Adaptive Communication:** Posts are automatically timed for maximum relevance and impact.
*   **Autonomous Operation:** The system can run continuously, identifying and seizing opportunities without human intervention.
