# The scripts builds avail-rust in x86 and wasm mode to see if it works for both targets.

cargo build --locked --release
rustup target add wasm32-unknown-unknown
cargo build --locked --release --target wasm32-unknown-unknown --no-default-features
