# Change Request: Gemini Auth Integration

## 1. Objective

To integrate Google Gemini LLM authentication and message sending capabilities into the `amazon-q-developer-cli` framework, specifically by translating the OAuth2 authentication flow and message sending functionality from the `gemini-cli` (Node.js project) into a new Rust crate.

## 2. Scope

This change will primarily affect the `amazon-q-developer-cli` project, specifically by adding a new Rust crate and modifying its `Cargo.toml`.

**Affected components:**

*   `amazon-q-developer-cli` (main repository)
*   New `gemini-auth` Rust crate
*   `vendor/google-oauth` (Rust library for OAuth2 verification)
*   `vendor/google-ai-rs` (Rust library for Google AI platform interaction)

## 3. Proposed Changes

### 3.1. Create `gemini-auth` Crate

*   A new binary Rust crate named `gemini-auth` will be created within `vendor/amazon-q-developer-cli/crates/`.
*   This crate will be added to the `[workspace.members]` section of the root `Cargo.toml` of the `amazon-q-developer-cli` project.

### 3.2. Implement OAuth2 Authentication Flow

*   The `gemini-auth` crate will implement the OAuth2 authentication flow for Google Gemini, based on the logic found in the `gemini-cli` (Node.js) project.
*   This will involve:
    *   Constructing the authorization URL.
    *   Guiding the user to open the URL in a browser for authentication.
    *   (Future enhancement: Implementing a local web server to automatically catch the redirect and extract the authorization code. For now, manual input of the code will be required).
    *   Exchanging the authorization code for access and ID tokens using `reqwest`.
    *   Verifying the ID token using the `google-oauth` crate.
*   The `oauth` subcommand will be defined using `clap` to handle the necessary command-line arguments (client ID, client secret, redirect URI).

### 3.3. Implement Gemini LLM Message Sending

*   After successful OAuth2 authentication and obtaining an access token, the `gemini-auth` crate will provide functionality to send messages to the Google Gemini LLM.
*   The `google-ai-rs` crate will be used for interacting with the Google AI platform in a type-safe manner.
*   A new `send-message` subcommand will be added to the `gemini-auth` CLI, taking the message and the obtained access token as arguments.

### 3.4. Dependency Integration

*   The `gemini-auth/Cargo.toml` will include the following dependencies:
    *   `clap` (for CLI argument parsing)
    *   `reqwest` (for HTTP requests)
    *   `tokio` (for asynchronous operations)
    *   `url` (for URL manipulation)
    *   `serde_json` (for JSON serialization/deserialization)
    *   `google-oauth` (path dependency to `vendor/google-oauth`)
    *   `google-ai-rs` (path dependency to `vendor/google-ai-rs/google-ai-rs`)

## 4. Rationale

This change is necessary to extend the capabilities of the `amazon-q-developer-cli` to support Google Gemini LLM, providing users with more flexibility in choosing their preferred LLM backend. By integrating the OAuth2 flow, we ensure secure and standardized authentication with Google services. The use of existing Rust crates (`google-oauth`, `google-ai-rs`) will accelerate development and leverage well-tested libraries.

## 5. Testing Strategy

*   **Unit Tests:**
    *   Basic unit tests will be written for the `oauth_flow` function to ensure it compiles and handles basic input/output, even if manual intervention is required for the full flow.
    *   Unit tests will be written for the `send_message_to_gemini` function to verify correct interaction with the `google-ai-rs` client.
*   **Manual Testing:**
    *   The `oauth` command will be manually tested by running the CLI tool, opening the authorization URL in a browser, and manually providing the authorization code.
    *   The `send-message` command will be manually tested after a successful OAuth flow, using the obtained access token to send a sample message to the Gemini LLM.

## 6. Rollback Plan

To roll back these changes, the following steps can be taken:

1.  Revert the commit that adds the `CRQ_Gemini_Auth_Integration.md` document.
2.  Revert the commit that adds the `gemini-auth` crate and modifies the root `Cargo.toml`.
3.  Remove the `vendor/amazon-q-developer-cli/crates/gemini-auth` directory.
4.  Revert the commits that added the new vendor submodules (`llmclient`, `llm`, `ask_gemini`, `google-oauth`, `google-ai-rs`, `askrepo`) if they were added specifically for this feature and are not used elsewhere.

## 7. Approval

[ ] Approved
[ ] Not Approved

**Approver Signature:** ________________________
**Date:** ________________________
