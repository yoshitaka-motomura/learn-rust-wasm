#!/bin/bash

# Exit on error
set -e

# Build the project
echo "Building the project..."
wasm-pack build --target web --out-name wasm --out-dir examples/react-example/src/wasm