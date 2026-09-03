FROM ubuntu:22.04

RUN apt update -y && \
    apt install --fix-missing build-essential git clang curl libssl-dev llvm libudev-dev make cmake protobuf-compiler -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /workdir
COPY rust-toolchain.toml .
RUN rustup show
