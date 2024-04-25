#!/bin/bash

ENGINE="${ENGINE:-docker}"

IMAGE="ubuntu-2204.Dockerfile"
DOCKER_FILE="./scripts/chainspec/mainnet/$IMAGE"
DOCKER_IGNORE_FILE="./scripts/chainspec/mainnet/ignore.dockerignore"

mkdir -p "output/"

# Build the image
"$ENGINE" build -t availnodet --ignorefile=$DOCKER_IGNORE_FILE -f $DOCKER_FILE .

selinuxenabled
if [ $? -ne 0 ]; then
    "$ENGINE" run --rm -v ./output/$ARCH/$DISTRO:/output availnodet
else
    "$ENGINE" run --rm -v ./output/$ARCH/$DISTRO:/output:z availnodet
fi
