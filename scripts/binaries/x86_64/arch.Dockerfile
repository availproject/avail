FROM registry.hub.docker.com/archlinux/archlinux:latest

RUN pacman -Syu --noconfirm git clang curl cmake make protobuf

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /workdir
COPY rust-toolchain.toml .
RUN rustup show
