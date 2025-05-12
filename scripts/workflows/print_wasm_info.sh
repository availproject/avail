#!/bin/bash

# This installs Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y
. "$HOME/.cargo/env"

rustup show
cargo build --locked --release

cargo install --locked --git https://github.com/chevdor/subwasm --tag v0.21.3
subwasm info ./target/release/wbuild/da-runtime/da_runtime.compact.compressed.wasm
subwasm version ./target/release/wbuild/da-runtime/da_runtime.compact.compressed.wasm

cp ./target/release/wbuild/da-runtime/da_runtime.compact.compressed.wasm /output/
