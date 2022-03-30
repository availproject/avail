#!/bin/bash

echo "Setting up the Rust environment for fuzzing..."

# TODO: generalize the script for any OS
sudo yum update -y
sudo yum install git -y

# Clone the avail repo
git clone https://github.com/maticnetwork/avail

# Install and configure Rust dependencies
sudo yum install -y clang curl libssl-dev llvm libudev-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# Use rustup show for testing the configuration

cargo install afl
