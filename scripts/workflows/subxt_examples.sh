#!/bin/bash

# This installs Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y
. "$HOME/.cargo/env"

rustup show
cargo build --locked --release

./target/release/avail-node --dev &
sleep 5
cargo run --release --manifest-path avail-subxt/Cargo.toml --example accounts_from_mnemonics
cargo run --release  --manifest-path avail-subxt/Cargo.toml --example headers
cargo run --release  --manifest-path avail-subxt/Cargo.toml --example max_block_submit
cargo run --release  --manifest-path avail-subxt/Cargo.toml --example submit_data
cargo run --release  --manifest-path avail-subxt/Cargo.toml --example vector_send_msg
cargo run --release  --manifest-path avail-subxt/Cargo.toml --example submit_block_length_proposal
