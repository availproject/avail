#!/bin/bash

DISTRO="${DISTRO:-ubuntu-2204}"
ENGINE="${ENGINE:-docker}"
ARCH="${ARCH:-x86_64}"

IMAGE="${DISTRO}.Dockerfile"
DOCKER_FILE="./scripts/binaries/$ARCH/$IMAGE"
DOCKER_IGNORE_FILE="./scripts/binaries/$ARCH/shared.dockerignore"

if ! test -f "$DOCKER_FILE"; then
    echo "Unknown option"
    echo "Supported DISTRO: ubuntu-2004 ubuntu-2204 ubuntu-2304 ubuntu-2310 fedora-38 fedora-39 debian-11 debian-12 arch"
    echo "Supported ARCH: x86_64 arm64"
    echo "Supported ENGINE: docker podman"
    exit 0
fi

echo "Selected distro: $DISTRO"
echo "Selected engine: $ENGINE"
echo "Selected arch: $ARCH"
echo "Selected docker file: $DOCKER_FILE"

# Build the image
"$ENGINE" build -t availnodet --ignorefile=$DOCKER_IGNORE_FILE -f $DOCKER_FILE .

mkdir -p "output/$ARCH/$DISTRO"

selinuxenabled
if [ $? -ne 0 ]; then
    "$ENGINE" run --rm -v ./output/$ARCH/$DISTRO:/output availnodet
else
    "$ENGINE" run --rm -v ./output/$ARCH/$DISTRO:/output:z availnodet
fi


if  [[ "$ZIP" ]]; then
    mkdir -p ./output/zips/

    cd ./output/$ARCH/$DISTRO
    tar -czf ./../../../output/zips/${ARCH}-${DISTRO}-avail-node.tar.gz avail-node
fi
