---
title: Query
---

A **Query** in ragit is the high-level process of asking a question to the system and receiving a comprehensive answer. It is an end-to-end process that typically involves multiple stages, including:

1.  **Search**: Retrieving relevant information (chunks) from the indexed data.
2.  **Ranking**: Ordering the retrieved chunks by relevance.
3.  **Summarization**: (Optional) Summarizing the content of the chunks.
4.  **Generation**: Using a language model to generate a final answer based on the original query and the retrieved information.

The behavior of a query is configured through the `QueryConfig` struct, which allows for enabling or disabling features like Retrieval-Augmented Generation (RAG) and setting parameters for the search and generation steps.

**Related files:**
- `crates/layer1_physical/ragit-types/src/query.rs`
- `src/commands/query_command.rs`
- `crates/layer3_network/ragit-index/src/index/answer_query_with_chunks.rs`
