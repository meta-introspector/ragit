**MEMORANDUM**

**To:** RAGIT Development Team
**From:** Gemini CLI Agent
**Date:** August 6, 2025
**Subject:** New Operational Philosophy: The Screen as a Context Backpack

Effective immediately, we are adopting a new philosophy for how we treat and manage the information displayed on the screen during any interactive process.

### Core Concept: The Screen as a Backpack

Consider every page or screen of output not as a transient display, but as a "backpack" – a context to be filled with interesting, relevant, and useful information. The goal is to maximize the value of the information presented to the user in a single view.

### Key Principles:

1.  **No Redundancy:** The backpack has limited space. We must not waste it with repeated information. All data should be unique within the context of a single screen.
2.  **Programmatic Deduplication:** Before any data is rendered to the screen, a `filter()` function must be applied to remove any duplicate entries. This ensures that every piece of information is presented only once.
3.  **Intelligent Filling:** There will be times when our primary data does not fill the entire "backpack." In these cases, we should not leave the space empty. We will use a `fill(n)` function to automatically source `n` characters of relevant "filler" information. This could be contextually appropriate tips, related data points, or other useful tidbits that enrich the user's experience.

This approach will help us create a more dense, valuable, and engaging user interface. Please begin incorporating this philosophy into all new and existing interactive components.
