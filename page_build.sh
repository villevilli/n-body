#!/bin/sh

export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'

# Fail on error
set -e

echo "Building Rust"
cargo build --target wasm32-unknown-unknown --profile wasm-release

echo "Generating Wasm Bindings"
wasm-bindgen --out-name sim_test \
--out-dir web/ \
--target web target/wasm32-unknown-unknown/wasm-release/sim_test.wasm