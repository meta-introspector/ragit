# Livestreaming Setup

This document outlines the hardware and software setup used for live development streams.

## Current Setup

The primary development and streaming environment consists of:

*   **Primary Device:** Android phone.
*   **Screen Mirroring & Recording:** The phone's screen is mirrored to a Windows machine using the **Microsoft Phone Link** application.
*   **Recording:** Screen recording is handled on the Windows machine. This offloads the processing overhead from the Android device, ensuring that the development work is not impacted by the recording process.

## Planned Future Resources

We plan to expand our available computing resources to create a distributed and robust development environment. The following resources are planned to be integrated:

*   **On-Premise:**
    *   Home Linux Server
*   **Cloud Infrastructure:**
    *   Amazon Web Services (AWS)
    *   Oracle Cloud Infrastructure (OCI)
*   **Edge & Serverless:**
    *   Cloudflare Workers
    *   Wasmer (WebAssembly runtime)
    *   Vercel
*   **CI/CD:**
    *   GitHub Actions

This multi-faceted approach will allow us to leverage different environments for various tasks, from backend services and data processing to frontend hosting and automated workflows.
