#!/bin/bash

# This installs Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y
. "$HOME/.cargo/env"

rustup show
cargo build --locked --release --features fast-runtime

cd e2e
cargo test --no-run

# Run Node
../target/release/avail-node --dev &
sleep 10

# Run Tests
cargo test -- --nocapture
