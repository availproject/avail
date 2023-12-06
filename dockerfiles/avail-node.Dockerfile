FROM ubuntu:22.04

ADD . ./workdir
WORKDIR "/workdir"

# This installs all dependencies that we need (besides Rust).
RUN apt update -y && \
    apt install build-essential git clang curl libssl-dev llvm libudev-dev make cmake protobuf-compiler -y

# This installs Rust and updates Rust to the right version.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y && \
    . $HOME/.cargo/env && rustup show

# This builds the binary.
RUN $HOME/.cargo/bin/cargo build --locked --release
RUN cp target/release/data-avail /bin/

VOLUME ["/output"]
ENTRYPOINT ["/bin/data-avail"]
CMD ["--chain=goldberg", "--name=MyAvailNode", "-d=/output/data", "--rpc-methods=unsafe", "--unsafe-rpc-external", "--rpc-cors=all"]
