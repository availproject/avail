# syntax = docker/dockerfile:1.2

# Phase 0: Builder
# =========================
FROM paritytech/ci-linux:production as builder

RUN --mount=type=cache,id=build_apt,target=/var/cache/apt \
	--mount=type=cache,target=/root/.cargo \
	apt-get update && \
	apt-get install -yqq --no-install-recommends git openssh-client && \
	rm -rf /var/lib/apt/lists && \
	# Reinstall nightly \
    rustup toolchain uninstall nightly && \
	rustup toolchain install nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
	rustup default nightly && \
	mkdir -p /avail

ARG AVAIL_TAG=v1.5.0

RUN \
	# Build DA \
	git clone -b $AVAIL_TAG --single-branch https://github.com/availproject/avail.git /avail/ && \
	cd /avail && \
	cargo build --release -p data-avail && \
	# Install binaries \
	mkdir -p /da/bin && \
	mkdir -p /da/genesis && \
	cp -r /avail/misc/genesis /da && \
	mv target/release/data-avail /da/bin && \
	# Clean src \
	rm -rf /avail


# Phase 1: Binary deploy
# =========================
FROM debian:buster-slim

RUN \
	apt-get update && \
	apt-get install -y curl && \
	rm -rf /var/lib/apt/lists

ARG CHAIN_SPEC=https://devnet06.dataavailability.link/chainspec.raw.json

COPY --from=builder /da/ /da
ADD $CHAIN_SPEC /da/genesis/chainspec.raw.json
COPY entrypoint.sh /

# Get devnet06 chain spec
RUN \ 
	# curl https://devnet06.dataavailability.link/chainspec.raw.json -o /da/genesis/devnet06.spec.raw.json && \
	chmod +x /entrypoint.sh

ENV \
	DA_CHAIN="/da/genesis/chainspec.raw.json" \
	DA_NAME="AvailNode" \
	DA_MAX_IN_PEERS=50 \
	DA_MAX_OUT_PEERS=50 \
	DA_P2P_PORT="30333" \
	BOOTNODE_1="/dns/gateway-fullnode-001.devnet06.dataavailability.link/tcp/30333/p2p/12D3KooWBDBcRhvQdhNY6vJyupZXSRFrG8qUCJtok1xv3oG2uucg" \
	BOOTNODE_2="/dns/gateway-fullnode-002.devnet06.dataavailability.link/tcp/30333/p2p/12D3KooWFNQByvLgK4NtyutLZGmoZeyrShojiKCEkzj8m6YE8teu" \
	BOOTNODE_3="/dns/gateway-fullnode-003.devnet06.dataavailability.link/tcp/30333/p2p/12D3KooWS2UhfueAWeHc71pxHWZFX5g6enuDyJGPWPApkuWe1Lfi"

WORKDIR /da
VOLUME ["/tmp", "/da/state", "/da/keystore"]
ENTRYPOINT ["/entrypoint.sh"]

