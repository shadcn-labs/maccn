#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
crate_dir="$(cd "$script_dir/.." && pwd)"
workspace_dir="$(cd "$crate_dir/../../../.." && pwd)"
out_dir="$workspace_dir/docs/public/examples"

profile="debug"
cargo_args=(+nightly build --manifest-path "$crate_dir/Cargo.toml" --target wasm32-unknown-unknown)
if [[ "${1:-}" == "--release" ]]; then profile="release"; cargo_args+=(--release); fi

cargo "${cargo_args[@]}"

mkdir -p "$out_dir/wasm"
wasm-bindgen \
  "$workspace_dir/target/wasm32-unknown-unknown/$profile/maccn_wasm.wasm" \
  --out-dir "$out_dir/wasm" \
  --target web \
  --no-typescript

cp "$crate_dir/static/index.html" "$out_dir/index.html"
cp "$crate_dir/static/main.js" "$out_dir/main.js"

echo "WASM demo built to $out_dir"
