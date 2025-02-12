#!/bin/bash

ENGINE="${ENGINE:-docker}"
if [ -z "$SCRIPT_LOC" ]; then
    echo Make sure that ENV variable SCRIPT_LOC is set to a script path.
    echo Example
    echo "  SCRIPT_LOC=./scripts/workflows/releaser_wasm.sh ./scripts/workflows/run.sh"
    exit
fi

DOCKER_FILE="./scripts/workflows/base.Dockerfile"
DOCKER_IGNORE_FILE="./scripts/workflows/shared.dockerignore"

mkdir -p "output"

"$ENGINE" build -t availnodet --ignorefile=$DOCKER_IGNORE_FILE -f $DOCKER_FILE .


selinuxenabled
if [ $? -ne 0 ]; then
    "$ENGINE" run --rm -v ./output:/output availnodet $SCRIPT_LOC
else
    "$ENGINE" run --rm -v ./output:/output:z availnodet $SCRIPT_LOC
fi
