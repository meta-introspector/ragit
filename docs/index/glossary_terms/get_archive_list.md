- **Emoji:** ❓
- **Vector Locations:**
    - **8D:** `[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]`
    - **23D:** `[0.0, ...]`
    - **41D:** `[0.0, ...]`
    - **800D:** `[0.0, ...]`

---

---
title: get_archive_list
---

This function is responsible for retrieving a list of archives.

**Purpose:** To provide an API endpoint or internal utility for listing available archives, which are likely compressed or bundled collections of `ragit` repositories or data.

**Key Files/Contexts:**
- `crates/layer7_application/server/src/methods.rs`: Shows `get_archive_list` being used as a method.
- `crates/layer7_application/server/src/main.rs`: Shows `get_archive_list` being used in the main server setup.
- `crates/layer7_application/server/src/methods/clone.rs`: Contains the definition of the `get_archive_list` function.
