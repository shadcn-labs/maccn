#!/bin/bash
# Build the native-sdk WASM preview engine
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building native-sdk preview engine (C → WASM)..."
rm -f main.wasm main.o
zig cc -target wasm32-wasi -O2 -nostdlib -fno-builtin \
  -Wl,--no-entry -Wl,--export-dynamic \
  -o main.wasm src/main.c

echo "Copying to docs/public/native-sdk/preview.wasm..."
cp main.wasm ../../docs/public/native-sdk/preview.wasm

echo "Done. WASM size: $(du -h ../../docs/public/native-sdk/preview.wasm | cut -f1)"
