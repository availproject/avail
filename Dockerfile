# syntax = docker/dockerfile:1.2

# Phase 0: Builder
# =========================
FROM paritytech/ci-linux:1.69.0-bullseye as builder

# Install needed packages
RUN apt-get update && \
	apt-get install -yqq --no-install-recommends git openssh-client && \
	rm -rf /var/lib/apt/lists

# Install nightly Rust for WASM  & prepare folders
# RUN	rustup toolchain install nightly && \
#	rustup target add wasm32-unknown-unknown --toolchain nightly && \
#	rustup default nightly

# Clone & build node binary.
ARG AVAIL_TAG=v1.6.0
RUN \
	mkdir -p /da/state && \
	mkdir -p /da/keystore && \
	git clone -b $AVAIL_TAG --single-branch https://github.com/availproject/avail.git /da/src/ && \
	cd /da/src && \
	cargo build --release -p data-avail

# Install binaries 
RUN \ 
	mkdir -p /da/bin && \
	mv /da/src/misc/genesis /da && \
	mv /da/src/target/release/data-avail /da/bin && \
	# Clean src \
	rm -rf /da/src

# Phase 1: Binary deploy
# =========================
FROM debian:bullseye-slim

RUN \
	apt-get update && \
	apt-get install -y curl && \
	rm -rf /var/lib/apt/lists && \
	groupadd -r avail && \
	useradd --no-log-init -r -g avail avail

COPY --chown=avail.avail --from=builder /da/ /da
COPY --chown=avail.avail --chmod=755 entrypoint.sh /

ENV \
	DA_CHAIN="/da/genesis/testnet.kate.chain.spec.raw.json" \
	DA_NAME="AvailNode" \
	LANG=C.UTF-8 \
	DA_MAX_IN_PEERS=50 \
	DA_MAX_OUT_PEERS=50 \
	DA_P2P_PORT="30333"

# Opencontainers annotations
LABEL \
	org.opencontainers.image.authors="The Avail Project Team" \
	org.opencontainers.image.url="https://www.availproject.org/" \
	org.opencontainers.image.documentation="https://github.com/availproject/avail-deployment#readme" \
	org.opencontainers.image.source="https://github.com/availproject/avail-deployment" \
	org.opencontainers.image.version="1.0.0" \
	org.opencontainers.image.revision="1" \
	org.opencontainers.image.vendor="The Avail Project" \
	org.opencontainers.image.licenses="MIT" \
	org.opencontainers.image.title="Avail Node" \
	org.opencontainers.image.description="Data Availability Docker Node"

# USER avail:avail
WORKDIR /da
VOLUME ["/tmp", "/da/state", "/da/keystore"]
ENTRYPOINT ["/entrypoint.sh"]

