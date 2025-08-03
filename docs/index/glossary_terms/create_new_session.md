- **Emoji:** ❓
- **Vector Locations:**
    - **8D:** `[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]`
    - **23D:** `[0.0, ...]`
    - **41D:** `[0.0, ...]`
    - **800D:** `[0.0, ...]`

---

---
title: create_new_session
---

The `create_new_session` function is responsible for creating a new session record in the database, typically associated with an archive or repository operation.

**Purpose:** It ensures that each significant operation within `ragit` (like pushing or pulling archives) has a unique session identifier, allowing for tracking and management of these operations.

**Key Files/Contexts:**
- `crates/layer7_application/server/src/models/archive.rs`: Contains the definition of the `create_new_session` function.
- `crates/layer7_application/server/src/methods/push.rs`: Shows `create_new_session` being called when handling a push operation.
