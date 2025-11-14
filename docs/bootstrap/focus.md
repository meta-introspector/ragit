# Current Development Focus: Bootstrap Command

Our primary development focus is on ensuring the `ragit bootstrap` command compiles and functions correctly.

## Strategy:

To achieve a clean build for the `bootstrap` command, we are systematically commenting out broken or non-priority code in other modules. This allows us to isolate issues and focus solely on the core `bootstrap` functionality.

## Modules Currently Commented Out (FIX ME LATER):

- `ragit-muse`
- `ragit-command-qa-tune`
- `ragit-command-qa-test`
- `ragit-command-ls`
- `ragit-agent-action`
- `ragit-server`
- `ragit-legacy-log-qa-results`
- `ragit-legacy-uid`
- `ragit-groq-data`
- `ragit-legacy-agent`
- `ragit-legacy-api-config`
- `ragit-legacy-chunk`
- `ragit-command-ii-reset`
- `ragit-command-ii-status`
- `ragit-command-status`
- `ragit-command-summary`
- `ragit-command-help`
- `ragit-command-cat-file`

Each commented-out section is marked with a `panic!("FIX ME LATER: Fix the bootstrap first and this code later.")` to indicate that it needs to be addressed in a later phase.

## Next Steps:

Continue fixing compilation errors, prioritizing those that prevent the `bootstrap` command from building successfully.