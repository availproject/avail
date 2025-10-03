
FROM ubuntu:24.04 AS builder

# This installs all dependencies that we need (besides Rust).
RUN apt update -y && \
    apt install --fix-missing build-essential git clang curl libssl-dev llvm libudev-dev make cmake protobuf-compiler  pkg-config -y

# This installs Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./workdir
WORKDIR "/workdir"

# This installs the right toolchain
RUN rustup show

RUN cargo bench -p avail-blob --bench submit_data --no-run

ENTRYPOINT ["./scripts/run_blob_benchmark.sh"]
