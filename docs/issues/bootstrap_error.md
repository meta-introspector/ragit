# Issue: `ragit bootstrap` Fails with "Is a directory" Error

## Error Description

When attempting to run the `ragit bootstrap` command, the `ragit-build-index-worker-single-file` executable, which is spawned as a subprocess, fails with a "No such file or directory (os error 21)" error. This occurs despite the executable existing and having correct permissions, and even when attempting to execute it directly from the shell. The error message "Is a directory" is misleading, as the target is an executable file.

The root cause appears to be an environment or pathing issue when `ragit-commands` invokes the worker, leading the worker to incorrectly interpret a directory as a file during its internal file operations (specifically related to the `prompts` directory).

## Reproduction Steps

1.  Ensure the `ragit` project is in a clean state (all local changes reverted).
2.  Navigate to the project root directory: `/data/data/com.termux/files/home/storage/github/ragit`
3.  Run the bootstrap command with `max_iterations` set to 1:
    ```bash
    cargo run --package ragit-commands -- bootstrap --max-iterations 1
    ```

## Expected Outcome

The `ragit bootstrap` command should execute successfully, initializing the index and performing the bootstrap process for one iteration.

## Actual Outcome

The command fails with output similar to the following:

```
Command: cargo run --package ragit-commands -- bootstrap --max-iterations 1
Directory: (root)
Stdout: Verbose mode enabled.
configure_memory_settings: Finished


Stderr: warning: unused import: `std::path::PathBuf`
 --> crates/layer7_application/commands/ragit-command-bootstrap/src/bootstrap_command.rs:3:5
  |
3 | use std::path::PathBuf;
  |     ^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `ragit-command-bootstrap` (lib) generated 1 warning (run `cargo fix --lib -p ragit-command-bootstrap` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/ragit-commands bootstrap --max-iterations 1`
Error: No such file or directory (os error 21)

Error: ragit-build-index-worker-single-file failed with status: exit status: 1

Error: (none)
Exit Code: 1
Signal: (none)
Background PIDs: (none)
Process Group PGID: 17796
```
