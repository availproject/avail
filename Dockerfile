# Phase 0: Builder
# =========================
FROM paritytech/ci-linux:production as builder

RUN apt-get update && \
	apt-get install -y git openssh-client && \
	rm -rf /var/lib/apt/lists && \
  mkdir -p /avail

COPY . /avail/

RUN \
	mkdir -p /da/bin && \
	mkdir -p /da/genesis && \
	# Build DA \
	cp -r /avail/misc/genesis /da && \
	cd /avail && \
	cargo build --release -p data-avail && \
	# Install binaries \
	mv target/release/data-avail /da/bin


# Phase 1: Binary deploy
# =========================
FROM debian:buster-slim

RUN apt-get update && \
	apt-get install -y curl && \
	rm -rf /var/lib/apt/lists

COPY --from=builder /da/ /da
COPY entrypoint.sh /
WORKDIR /da

ENV \
	DA_CHAIN="/da/genesis/testnet.chain.spec.raw.json" \
	DA_NAME="AvailNode" \
	DA_MAX_IN_PEERS=50 \
	DA_MAX_OUT_PEERS=50 \
	DA_P2P_PORT="30333"

VOLUME ["/tmp", "/da/state", "/da/keystore"]
ENTRYPOINT ["/entrypoint.sh"]
