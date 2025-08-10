# RAGIT

RAGIT (rag-it) is a git-like software that turns your local files into a knowledge-base. The main goal of this project is to make knowledge-bases easy-to-create and easy-to-share.

```
rag init;
rag add --all;
rag build;
rag query "What makes ragit special?";
```

## Getting Started

To quickly get up to speed with the Gemini CLI and our project workflow, follow these two simple steps:

1.  **Install the Gemini CLI:**
    Open your terminal and run:
    ```bash
    npx https://github.com/google-gemini/gemini-cli
    ```

2.  **Say the Magic Words:**
    Once the CLI is installed, say "GM, WB, LG" to initiate your daily workflow. These acronyms are explained in detail in our [Onboarding SOP](./docs/sops/onboarding.md).

## Why another RAG framework?

RAGIT is very different from the other RAG frameworks.

1. It adds a title and summary to every chunks. The summaries make AIs very easy to rerank chunks.
2. It uses tfidf scores instead of vector searches. It first asks an AI to generate keywords from a query, then runs tfidf search with the keywords.
3. It supports markdown files with images.
4. It supports multi-turn queries (experimental).
5. You can clone/push knowledge-bases, like git.

## Dogfooding RAGIT Search

In line with our "eat your own dogfood" philosophy, the `search_file_content` tool within the Gemini CLI now leverages `ragit`'s internal search capabilities. This provides:

-   **Performance:** Faster searches, especially in large codebases, due to `ragit`'s indexing.
-   **Advanced Search:** Future potential for more sophisticated search functionalities (e.g., semantic search).
-   **Consistency:** Aligns our internal tools with the project's core `ragit` philosophy.
-   **Self-Improvement:** Enables `ragit` to "understand" its own codebase better, crucial for its self-improvement loop.

*(Note: The `solfunmeme` crate has been temporarily excluded from the workspace due to dependency conflicts. This will be revisited once the core search functionality is fully integrated and stable.)*

## Platform support

Ragit is primarily supported on Linux (x64) and Mac (aarch64). It goes through a full test process before each release, on Linux and Mac. It is primarily developed on Linux and Mac.

Ragit works on Windows, but it's [not perfect](https://github.com/baehyunsol/ragit/issues/13).

Other than those 3 platforms, I haven't tested ragit on any platform.

## Comprehensive Documentation

For a structured overview of the project's documentation, including a glossary of terms and an index of all documents, please refer to:

*   [Documentation Index](./docs/index/index.md)
*   [Glossary of Terms](./docs/index/glossary.md)

## Project Insights

For a free-form braindump and synthesis of the project's philosophy, architecture, and implementation details, see:

*   [Project Braindump](./docs/braindump.md)

## More documents

- [Architecture](./docs/architecture.md)
- [Build](./docs/build.md)
- [Bootstrap](./docs/bootstrap.md): A self-improving command for developers working on `ragit` itself.
- [Chunks](./docs/chunks.md)
- [Configuration](./docs/config.md)
- [Contribution](./docs/contribution.md)
- [Evaluation](./docs/eval.md)
- [Multi Turn](./docs/multi_turn.md)
- [Pipeline](./docs/pipeline.md)
- [Prompt Engineering](./docs/prompt_engineering.md)
- [Quick Guide](./docs/quick_guide.md)

## Debugging

Ragit provides a `--verbose` flag for more detailed output, which can be helpful for debugging.

```sh
rag --verbose <command>
```

## Interactive documents

```sh
cargo install ragit;
rag clone https://ragit.baehyunsol.com/sample/ragit;
cd ragit;
export GROQ_API_KEY=YOUR_API_KEY;
rag query "How do I contribute to ragit?";
```