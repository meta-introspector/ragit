---
title: BootstrapArgs
---

`BootstrapArgs` is a struct that defines the command-line arguments and configuration options for the `bootstrap` command in `ragit`. The `bootstrap` command is responsible for initializing and setting up a `ragit` repository, including tasks like indexing files and preparing the environment.

**Purpose:** It allows users to customize the behavior of the `bootstrap` process by providing various parameters, such as paths, flags, and other configuration settings.

**Key Files/Contexts:**
- `crates/layer7_application/ragit-build-index-worker-single-file/src/main.rs`: Shows `BootstrapArgs` being used as an argument to `bootstrap_command_main`.
- `crates/layer7_application/ragit-build-index-worker-single-file/src/args/bootstrap_args.rs`: This is where the `BootstrapArgs` struct is defined.
