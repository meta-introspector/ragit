// Emojis: 🔄🧠✅
// Hyperspace: [0.10, 0.20, 0.30, 0.40, 0.50, 0.60, 0.70, 0.80]
// Summary Number: 20250805

# SOP: GM Meta-Program - Reboot Recovery

## 1. Objective
To provide a standardized procedure for recovering from a system reboot or unexpected interruption, ensuring rapid re-orientation and continuation of development on the critical path.

## 2. Scope
This SOP applies to the Gemini CLI agent's operational workflow following any system reboot or restart.

## 3. Procedure

### Step 1: Stay on the Critical Path
Immediately re-focus on the primary, most impactful task or objective that was being pursued prior to the reboot. Avoid distractions and secondary tasks until the critical path is re-established.

### Step 2: Review Gemini Memories
Access and review the internal memory (context) to recall user preferences, project-specific facts, and any ongoing discussions or plans. This helps in quickly re-establishing the operational context.

### Step 3: Check Recent Git Commits
Execute the following command to review the most recent changes across all branches. This provides a concise overview of the project's current state and recent modifications.

```bash
git log --patch -3 --all
```

## 4. Expected Outcome
- Rapid re-establishment of operational context.
- Efficient continuation of development on the critical path.
- Minimized downtime and increased productivity after a reboot.

## 5. Documentation
This SOP will be linked from `docs/index.md` and `docs/quality_procedures/index.md` (if it exists).

## 6. Review and Audit
This SOP will be reviewed annually and audited as part of the project's overall quality management system to ensure its effectiveness and compliance.
