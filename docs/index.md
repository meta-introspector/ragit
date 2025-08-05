# RAGIT

RAGIT (rag-it) is a git-like software that turns your local files into a knowledge-base. The main goal of this project is to make knowledge-bases easy-to-create and easy-to-share.

```
rag init;
rag add --all;
rag build;
rag query "What makes ragit special?";
```

## Why another RAG framework?

RAGIT is very different from the other RAG frameworks, focusing on a highly modular and extensible design.

1. It adds a title and summary to every chunks. The summaries make AIs very easy to rerank chunks.
2. It uses tfidf scores instead of vector searches. It first asks an AI to generate keywords from a query, then runs tfidf search with the keywords.
3. It supports markdown files with images.
4. It supports multi-turn queries (experimental).
5. You can clone/push knowledge-bases, like git.

## VI. Source Code Modules & Components

### Submodules

### Rust Crates

*   [`solfunmeme_ontology_vibe`](./rust_code/solfunmeme_ontology_vibe.md)
*   [`solfunmeme_clifford`](./rust_code/solfunmeme_clifford.md)

*   **`bootstrap`**
    *   **Themes**: Bootstrap Microkernel, Mathematical Lattice Architecture, Prime Vibe Ontology, RDF, System Commitment.
    *   **Key Takeaways**: A self-aware system integrating mathematical frameworks with semantic processing for continuous evolution.
    *   **Documentation**: [bootstrap.md](./submodules/bootstrap.md)

*   **`ai-agent-terraform`**
    *   **Themes**: Terraform, CloudFormation, AWS, Auto Scaling, Solana integration, AI agents.
    *   **Key Takeaways**: Infrastructure as Code for deploying and managing AI agent ecosystems on AWS, with a focus on scalability and Solana blockchain integration.
    *   **Documentation**: [ai_agent_terraform.md](./submodules/ai_agent_terraform.md)

*   **`meta-meme`**
    *   **Themes**: Meta-memes, Gödelian recursion, hyperspheres, AI cognition, formal verification, mathematical philosophy.
    *   **Key Takeaways**: Explores the nature of knowledge, consciousness, and AI through self-referential symbolic systems and mathematical structures.
    *   **Documentation**: [meta_meme.md](./submodules/meta_meme.md)

*   **`agave-solana-validator`**
    *   **Themes**: Solana blockchain validator, transaction management, ZK proofs, SBPF.
    *   **Key Takeaways**: Core component for Solana blockchain operations, including security and runtime aspects.
    *   **Documentation**: [agave_solana_validator.md](./submodules/agave_solana_validator.md)

*   **`quasi-meta-meme`**
    *   **Themes**: Meta-memes, LaTeX, Emojlang.
    *   **Key Takeaways**: Development of a meta-meme framework, potentially involving the use of LaTeX for documentation or generation of meta-memes.
    *   **Documentation**: [quasi_meta_meme.md](./submodules/quasi_meta_meme.md)

*   **`solfunmeme`**
    *   **Themes**: Diagrams, Architectural Documentation.
    *   **Key Takeaways**: Focus on visual representation and architectural documentation.
    *   **Documentation**: [solfunmeme.md](./submodules/solfunmeme.md)

*   **`solfunmeme-dioxus`**
    *   **Themes**: Dioxus, Ontology Development.
    *   **Key Takeaways**: A Dioxus-based project focused on ontology creation and development.
    *   **Documentation**: [solfunmeme_dioxus.md](./submodules/solfunmeme_dioxus.md)

*   **`time`**
    *   **Themes**: Deployment, Release Management, Automation.
    *   **Key Takeaways**: Focus on automating software delivery processes.
    *   **Documentation**: [time.md](./submodules/time.md)


Ragit is built with a multi-crate architecture, where functionalities are separated into distinct, specialized Rust crates. This approach enhances:

*   **Modularity:** Each component is a self-contained unit, making the codebase easier to understand and manage.
*   **Reusability:** Individual crates can be reused across different parts of the project or even in other applications.
*   **Maintainability:** Changes and updates to one part of the system are isolated, reducing the risk of unintended side effects.
*   **Scalability:** The modular structure allows for easier expansion and integration of new features.

Key functionalities like indexing, API interactions, and command handling are now managed by dedicated crates (e.g., `ragit-index`, `ragit-api`, `ragit-command-*`), contributing to a more robust and flexible system.

## Platform support

Ragit is primarily supported on Linux (x64) and Mac (aarch64). It goes through a full test process before each release, on Linux and Mac. It is primarily developed on Linux and Mac.

Ragit works on Windows, but it's [not perfect](https://github.com/baehyunsol/ragit/issues/13).

Other than those 3 platforms, I haven't tested ragit on any platform.

## More documents

- [Build](./build.md)
- [Chunks](./chunks.md)
- [Configuration](./config.md)
- [Contribution](./contribution.md)
- [Embedding to Ontology Pipeline](./sops/embedding_to_ontology_pipeline.md)
- [Evaluation](./eval.md)
- [Glossary](./index/glossary.md)
- [Hierarchical Search SOP](./sops/hierarchical_search_sop.md)
- [ECO-PAPER-9K](./sops/ECO-PAPER-9K.md)
- [Branching Strategy SOP](./sops/branching_strategy_sop.md)
- [DWIM Embedding Pipeline Work Log](./quality_procedures/dwim_embedding_pipeline_work_log.md)
- [Dependency Management SOP](./quality_procedures/dependency_management_sop.md)
- [GM Meta-Program SOP](./quality_procedures/gm_meta_program_sop.md)
- [Gemini Session Log](./quality_procedures/gemini_session_log.md)
- [KitKat Meta-Program SOP](./quality_procedures/kitkat_meta_program_sop.md)
- [Multi Turn](./multi_turn.md)
- [Pipeline](./pipeline.md)
- [Prompt Engineering](./prompt_engineering.md)
- [Quick Guide](./quick_guide.md)
- [Architecture](./architecture.md)

## Interactive documents

```sh
cargo install ragit;
rag clone https://ragit.baehyunsol.com/sample/ragit;
cd ragit;
export GROQ_API_KEY=YOUR_API_KEY;
rag query "How do I contribute to ragit?";
```