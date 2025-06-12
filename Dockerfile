# Phase 1: Builder
# =========================
FROM debian:12.5-slim AS builder

# Install needed packages
RUN apt update -y && \
    apt install build-essential git clang curl libssl-dev llvm libudev-dev make cmake protobuf-compiler -y

WORKDIR "/da/src"

# Clone repo
ARG AVAIL_TAG=v1.8.0.3
RUN git clone -b $AVAIL_TAG --single-branch https://github.com/availproject/avail.git .

# This installs Rust and updates Rust to the right version.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y

# Build Binary at /da/src/avail-node
RUN $HOME/.cargo/bin/rustup show 
RUN $HOME/.cargo/bin/cargo build --locked --release
RUN cp ./target/release/avail-node .

# Phase 2: Binary deployment
# =========================
FROM debian:12.5-slim

RUN apt update -y && apt install curl -y

COPY --from=builder /da/src/avail-node /usr/local/bin/avail-node
RUN chmod +x /usr/local/bin/avail-node

# Opencontainers annotations
LABEL org.opencontainers.image.authors="The Avail Project Team" \
	org.opencontainers.image.url="https://www.availproject.org/" \
	org.opencontainers.image.source="https://github.com/availproject/avail" \
	org.opencontainers.image.version="1.8.0" \
	org.opencontainers.image.revision="3" \
	org.opencontainers.image.vendor="The Avail Project" \
	org.opencontainers.image.licenses="MIT" \
	org.opencontainers.image.title="Avail Node" \
	org.opencontainers.image.description="Data Availability Docker Node"

VOLUME ["/da/node-data"]
ENTRYPOINT ["/usr/local/bin/avail-node"]
CMD ["--chain", "goldberg", "--tmp", "--name", "MyAwesomeAvailNodeInContainer"]

