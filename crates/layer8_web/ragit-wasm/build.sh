#!/bin/bash

# Build RAGIT WebAssembly module
echo "Building RAGIT WASM..."

# Install wasm-pack if not available
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package
wasm-pack build --target web --out-dir js/pkg

# Copy extension files
echo "Preparing extension files..."
cp js/manifest.json js/pkg/
cp js/popup.html js/pkg/
cp js/content-script.js js/pkg/

echo "Build complete! Extension files are in js/pkg/"
echo "To install:"
echo "1. Open Chrome and go to chrome://extensions/"
echo "2. Enable Developer mode"
echo "3. Click 'Load unpacked' and select the js/pkg/ directory"