FROM fedora:42

RUN dnf update -y && \
    dnf install git clang curl make cmake protobuf-compiler -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /workdir
COPY rust-toolchain.toml .
RUN rustup show
