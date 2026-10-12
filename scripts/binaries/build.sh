#!/bin/bash

ENGINE="${ENGINE:-docker}"
ARCH="${ARCH:-x86_64}"
DISTRO="${DISTRO:-ubuntu-2204}"

IMAGE="${DISTRO}.Dockerfile"
DOCKER_FILE="./scripts/binaries/$ARCH/$IMAGE"
DOCKER_IGNORE_FILE="./scripts/binaries/$ARCH/shared.dockerignore"

if ! test -f "$DOCKER_FILE"; then
    echo "Unknown option"
    echo "Supported DISTRO: ubuntu-2204 ubuntu-2404 ubuntu-2604 fedora-44 debian-13"
    echo "Supported ARCH: x86_64 arm64"
    echo "Supported ENGINE: docker podman"
    exit 0
fi

echo "Selected TRIPLE: $ENGINE - $ARCH - $DISTRO"
echo "Selected docker file: $DOCKER_FILE"

mkdir -p "output/$ARCH/$DISTRO"

# Build the image
"$ENGINE" build -t availnodet --ignorefile=$DOCKER_IGNORE_FILE -f $DOCKER_FILE .

selinuxenabled
if [ $? -ne 0 ]; then
    "$ENGINE" run --rm -v ./output/$ARCH/$DISTRO:/output availnodet
else
    "$ENGINE" run --rm -v ./output/$ARCH/$DISTRO:/output:z availnodet
fi

if  [[ "$ZIP" ]]; then
    cd ./output/$ARCH/$DISTRO
    tar -czf avail-node.tar.gz avail-node
fi
