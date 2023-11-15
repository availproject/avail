#!/bin/bash
FLAVOUR="${FLAVOUR:-ubuntu-2204}"
SYSTEM="${SYSTEM:-docker}"

IMAGE="${FLAVOUR}.Dockerfile"
DOCKER_FILE="./scripts/linux_flavours/$IMAGE"

if ! test -f "$DOCKER_FILE"; then
    echo "Unknown option: $FLAVOUR. Use one of these: ubuntu-2204 ubuntu-2304 fedora-37 fedora-38 debian-11 debian-12 arch"
    exit 0
fi

echo "Selected flavour: $FLAVOUR"
echo "Selected system: $SYSTEM"
echo "Selected docker file: $DOCKER_FILE"

# Build the image
"$SYSTEM" build -t availnode -f $DOCKER_FILE .

mkdir -p output/$FLAVOUR

selinuxenabled
if [ $? -ne 0 ]
then
    "$SYSTEM" run --rm -v ./output/$FLAVOUR:/output availnode
else
    "$SYSTEM" run --rm -v ./output/$FLAVOUR:/output:z availnode
fi
