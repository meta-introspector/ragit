# SOP: Rust Daemonization

## 1. Purpose

To standardize the process of creating a daemonized server in Rust, ensuring consistency and robustness across the project.

## 2. Scope

This SOP applies to all Rust-based servers within the `ragit` project that need to run as background processes.

## 3. Procedure

### 3.1. Add `daemonize` Dependency

In the `Cargo.toml` file of the server crate, add the `daemonize` crate as a dependency:

```toml
[dependencies]
daemonize = "0.5.0"
```

### 3.2. Implement Daemonization in `main.rs`

Modify the `main.rs` file to use the `daemonize` library. This involves forking the process and creating a PID file.

```rust
use daemonize::Daemonize;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let pid_file = format!("{}/../../tmp/server.pid", crate_dir);

    let daemonize = Daemonize::new()
        .pid_file(&pid_file)
        .chown_pid_file(false)
        .working_directory(crate_dir);

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized.");
            run_server(); // Your server logic
        }
        Err(e) => eprintln!("Error, {}", e),
    }
}

#[tokio::main]
async fn run_server() {
    // ... your server implementation ...
}
```

### 3.3. Create a PID File Directory

Ensure the directory for the PID file exists. This can be done with a simple shell command before running the server for the first time:

```bash
mkdir -p tmp
```

### 3.4. Implement Graceful Shutdown

Implement a graceful shutdown mechanism that cleans up resources and terminates the process. This is typically done with a dedicated endpoint that signals the server to exit.

```rust
async fn stop_handler(State(state): State<AppState>) -> &'static str {
    if let Some(tx) = state.shutdown_tx.lock().unwrap().take() {
        let _ = tx.send(());
        "Shutting down..."
    } else {
        "Shutdown signal already sent or not available."
    }
}

// In the graceful_shutdown block:
let _ = shutdown_rx.await;
println!("Shutdown signal received, shutting down.");
std::process::exit(0);
```

## 4. Tools

*   `daemonize` crate: [https://crates.io/crates/daemonize](https://crates.io/crates/daemonize)

## 5. Expected Outcome

A robust, daemonized Rust server that:
*   Runs in the background without blocking the terminal.
*   Can be managed through its PID file.
*   Shuts down gracefully via a dedicated endpoint.
