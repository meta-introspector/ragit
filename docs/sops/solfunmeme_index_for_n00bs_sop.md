# SOP: Using the Solfunmeme Index with AI for New Contributors ("N00bs")

## 1. Purpose
This Standard Operating Procedure (SOP) provides new contributors to the `ragit` project with a clear guide on how to leverage the Solfunmeme Index and its integrated AI capabilities to quickly understand the codebase, find information, and accelerate their onboarding process.

## 2. Scope
This SOP applies to all new contributors and anyone seeking to efficiently navigate and learn the `ragit` codebase using the AI assistant (Gemini).

## 3. Key Concepts
*   **Solfunmeme Index:** The project's internal knowledge base, which semantically maps all code, documentation, and concepts. Think of it as the `ragit` project's "brain."
*   **AI Assistant (Gemini):** Your primary interface for interacting with the Solfunmeme Index. Gemini uses the index to provide intelligent, context-aware answers to your questions about the codebase.
*   **"Vibe" Emojis:** Emojis assigned to code and concepts to represent their semantic meaning or purpose, making them more intuitive.

## 4. Procedure: How to Bootstrap Your Learning with AI

### Step 1: Access the AI Assistant
*   Your primary interaction point is the Gemini CLI agent. Ensure you have access to the environment where Gemini is running.

### Step 2: Ask Questions About the Codebase
*   Formulate your questions clearly and directly. Gemini will use the Solfunmeme Index to find the most relevant information.
*   **Examples of questions you can ask:**
    *   "Explain the `Index` struct."
    *   "Where is the `add_chunk` method implemented?"
    *   "What are the main components of the `ragit` project?"
    *   "How does `ragit` handle file processing?"
    *   "Show me the code for `analyze_project`."
    *   "What is the purpose of the `ragit-types` crate?"

### Step 3: Interpret AI Responses
*   Gemini's responses will often include:
    *   **Explanations:** High-level summaries or detailed breakdowns of concepts.
    *   **Code Snippets:** Relevant lines or blocks of code directly from the codebase.
    *   **File Paths:** Exact locations of files where the information is found.
    *   **Cross-references:** Pointers to related documentation or code sections.
    *   **Emoji Context:** Emojis that provide a quick visual cue about the topic's "vibe."

### Step 4: Explore and Iterate
*   Use Gemini's responses as a starting point for deeper exploration.
*   If a response is too broad, ask more specific follow-up questions.
*   If a response is unclear, ask Gemini to rephrase or provide more examples.
*   **Example Iteration:**
    *   **You:** "Explain the `Index` struct."
    *   **Gemini:** (Provides overview of `Index` struct and its fields)
    *   **You:** "What is `ii_status` in the `Index` struct?"
    *   **Gemini:** (Explains `ii_status` and its role in the inverted index)

### Step 5: Leverage AI for Task Assistance
*   Once you have a basic understanding, you can ask Gemini for assistance with specific tasks.
*   **Examples:**
    *   "Help me refactor the `Index::new` method."
    *   "Generate a test case for the `add_chunk` function."
    *   "Where should I add new code for a file parsing utility?"

## 5. Best Practices for New Contributors
*   **Start Broad, Then Narrow:** Begin with high-level questions and progressively refine them based on Gemini's responses.
*   **Be Specific:** The more precise your question, the more accurate and helpful Gemini's answer will be.
*   **Read the `README.md`:** Always start with the `README.md` files in relevant directories for a quick overview.
*   **Consult SOPs:** Refer to the `docs/sops/` directory for detailed procedures on various development tasks.
*   **Provide Context:** If your question relates to a specific piece of code you're looking at, mention the file path or code snippet.

## 6. Expected Outcome
*   Accelerated learning and understanding of the `ragit` codebase.
*   Increased efficiency in finding information and performing development tasks.
*   Faster integration into the `ragit` development team.
*   Empowerment to contribute effectively from day one.
