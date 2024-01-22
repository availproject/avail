# This is the first stage. Here we install all the dependencies that we need in order to build the Ternoa binary.
FROM registry.hub.docker.com/archlinux/archlinux:latest as builder

# This installs all dependencies that we need (besides Rust).
RUN pacman -Syu --noconfirm git clang curl cmake make protobuf -y

# This installs Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y

ADD . ./workdir
WORKDIR "/workdir"

# This install the right toolchain
RUN $HOME/.cargo/bin/rustup show

# This builds the binary.
RUN $HOME/.cargo/bin/cargo build --locked --release

# Create output folder
RUN mkdir -p output

VOLUME ["/output"]
CMD cp ./target/release/data-avail /output
