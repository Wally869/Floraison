#!/bin/bash
# Build script for WASM module
# Builds the Rust WASM crate and outputs to the UI directory

set -e

echo "🔨 Building Floraison WASM module..."

# Navigate to WASM crate directory
cd floraison-wasm

# Build with wasm-pack for web target
wasm-pack build \
  --target web \
  --out-dir ../floraison-ui/src/lib/wasm \
  --out-name floraison \
  --no-typescript

# Go back to root
cd ..

echo "✅ WASM build complete! Output: floraison-ui/src/lib/wasm/"
echo ""
echo "Generated files:"
ls -lh floraison-ui/src/lib/wasm/
