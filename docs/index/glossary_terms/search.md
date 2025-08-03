- **Emoji:** ❓
- **Vector Locations:**
    - **8D:** `[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]`
    - **23D:** `[0.0, ...]`
    - **41D:** `[0.0, ...]`
    - **800D:** `[0.0, ...]`

---

---
title: Search
---

**Search** is a core component of the query process in ragit. It refers to the mechanism of finding and retrieving relevant chunks of text from the indexed data based on a user's query or keyword.

Ragit can employ different search strategies, including:

*   **TF-IDF (Term Frequency-Inverse Document Frequency)**: A statistical measure used to evaluate how important a word is to a document in a collection or corpus.
*   **Exact Match**: A simpler search that looks for the exact occurrence of a keyword or phrase within the chunks.

The `run_search` function is responsible for executing these searches and returning a set of relevant chunks that can then be used by the higher-level query process to generate an answer.

**Related files:**
- `crates/legacy_and_refactoring/ragit-agent-action/src/run_search.rs`
