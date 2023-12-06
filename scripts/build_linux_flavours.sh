#!/bin/bash
FLAVOUR="${FLAVOUR:-ubuntu-2204}"
ENGINE="${ENGINE:-docker}"

IMAGE="${FLAVOUR}.Dockerfile"
DOCKER_FILE="./scripts/linux_flavours/$IMAGE"

if ! test -f "$DOCKER_FILE"; then
    echo "Unknown option: $FLAVOUR. Use one of these: ubuntu-2004 ubuntu-2204 ubuntu-2304 ubuntu-2310 fedora-38 fedora-39 debian-11 debian-12 arch"
    exit 0
fi

echo "Selected flavour: $FLAVOUR"
echo "Selected engine: $ENGINE"
echo "Selected docker file: $DOCKER_FILE"

# Build the image
"$ENGINE" build -t availnode -f $DOCKER_FILE .

mkdir -p output/$FLAVOUR

selinuxenabled
if [ $? -ne 0 ]
then
    "$ENGINE" run --rm -v ./output/$FLAVOUR:/output availnode
else
    "$ENGINE" run --rm -v ./output/$FLAVOUR:/output:z availnode
fi
