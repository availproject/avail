#!/bin/bash
set -euo pipefail

DISTRO="${DISTRO:-ubuntu-2204}"
ENGINE="${ENGINE:-docker}"
ARCH="${ARCH:-x86_64}"
export CARGO_ARGS="${CARGO_ARGS:-}"

DOCKER_FILE="./scripts/binaries/$ARCH/$DISTRO.Dockerfile"
DOCKER_IGNORE_FILE="./scripts/binaries/$ARCH/shared.dockerignore"

if ! test -f "$DOCKER_FILE"; then
    echo "Unknown option"
    echo "Supported DISTRO: ubuntu-2204 ubuntu-2404 debian-12 debian-13 fedora-42 arch"
    echo "Supported ARCH: x86_64"
    echo "Supported ENGINE: docker podman"
    exit 1
fi

OUT="output/$ARCH/$DISTRO"
mkdir -p "$OUT"
echo "distro=$DISTRO engine=$ENGINE arch=$ARCH cargo_args='$CARGO_ARGS'"

"$ENGINE" build -t "avail-builder-$DISTRO" --ignorefile="$DOCKER_IGNORE_FILE" -f "$DOCKER_FILE" .

Z=""
selinuxenabled 2>/dev/null && Z=":z"

"$ENGINE" run --rm -e CARGO_ARGS -e OUT="$OUT" \
    -v "$PWD:/workdir$Z" \
    -v "avail-cargo-registry:/root/.cargo/registry" \
    -v "avail-target-$DISTRO:/workdir/target" \
    -w /workdir "avail-builder-$DISTRO" bash -ec '
        git config --global --add safe.directory /workdir
        cargo build --locked --release $CARGO_ARGS
        cp target/release/wbuild/da-runtime/da_runtime.compact.compressed.wasm output/
        [ -n "$CARGO_ARGS" ] || cp target/release/avail-node "$OUT/"
    '

if [[ "${ZIP:-}" && -f "$OUT/avail-node" ]]; then
    mkdir -p output/zips
    tar -C "$OUT" -czf "output/zips/${ARCH}-${DISTRO}-avail-node.tar.gz" avail-node
fi
