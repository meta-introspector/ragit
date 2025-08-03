---
title: select_turns_for_context
---

This function selects relevant turns from a chat history to provide context for a new query.

**Purpose:** To improve the quality of multi-turn conversations with the language model by ensuring that only the most pertinent parts of the conversation history are included in the context, preventing the model from being overwhelmed with irrelevant information.

**Key Files/Contexts:**
- `crates/layer5_session/ragit-session-query/src/lib.rs`: Contains the definition of the `select_turns_for_context` function.
- `crates/layer3_network/ragit-index/src/index/query_method.rs`: Shows `select_turns_for_context` being used to rephrase multi-turn queries.
