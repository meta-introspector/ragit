#!/bin/bash
# SOP 1: Building and Installing `coccinelleforrust`

# Purpose: To ensure `coccinelleforrust` is correctly built and installed for use within the project.

# Procedure:
# 1. Navigate to the `vendor/coccinelleforrust` directory.
# 2. Execute the build command.
# 3. Copy the compiled binary to a location accessible in the system's PATH.

echo "Navigating to vendor/coccinelleforrust..."
cd /data/data/com.termux/files/home/storage/github/ragit/vendor/coccinelleforrust || { echo "Failed to change directory."; exit 1; }

echo "Building coccinelleforrust in release mode..."
cargo build --release || { echo "Cargo build failed."; exit 1; }

echo "Copying cfr binary to ~/.local/bin/..."
mkdir -p ~/.local/bin/
cp target/release/cfr ~/.local/bin/ || { echo "Failed to copy cfr binary."; exit 1; }

echo "coccinelleforrust (cfr) has been built and installed."
