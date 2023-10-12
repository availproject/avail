#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
REPO_DIR=$SCRIPT_DIR/..

FLAVOUR="${FLAVOUR:-ubuntu-2204}"
SYSTEM="${SYSTEM:-docker}"

if [ -z "$FLAVOUR" ]; then
    select FLAVOUR in ubuntu-2204 ubuntu-2304 fedora-36 fedora-37 debian-11 debian-12 arch
    do
        break;
    done
fi

# Get Image name
if [ "$FLAVOUR" = "ubuntu-2204" ]; then
    imageName="ubuntu-2204.Dockerfile"
    elif [ "$FLAVOUR" = "ubuntu-2304" ]; then
    imageName="ubuntu-2304.Dockerfile"
    elif [ "$FLAVOUR" = "fedora-36" ]; then
    imageName="fedora-36.Dockerfile"
    elif [ "$FLAVOUR" = "fedora-37" ]; then
    imageName="fedora-37.Dockerfile"
    elif [ "$FLAVOUR" = "debian-11" ]; then
    imageName="debian-11.Dockerfile"
    elif [ "$FLAVOUR" = "debian-12" ]; then
    imageName="debian-12.Dockerfile"
    elif [ "$FLAVOUR" = "arch" ]; then
    imageName="arch.Dockerfile"
else
    echo "Unknown option: $FLAVOUR. Use one of these: ubuntu-2204 ubuntu-2304 fedora-36 fedora-37 debian-11 debian-12 arc"
    exit 0
fi

echo "Selected flavour: $FLAVOUR"
echo "Selected system: $SYSTEM"

# Move the right directory
cd $REPO_DIR

# Build the image
"$SYSTEM" build -t availnode -f ./scripts/linux_flavours/$imageName .

# Run the image
mkdir -p output/$FLAVOUR

selinuxenabled
if [ $? -ne 0 ]
then
    "$SYSTEM" run --rm -v ./output/$FLAVOUR:/output availnode
else
    "$SYSTEM" run --rm -v ./output/$FLAVOUR:/output:z availnode
fi
