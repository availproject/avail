#!/bin/bash

echo "--"
echo "[HELP] You can run this script in two ways"
echo "NORMAL: "
echo "      ./scripts/run_blob_benchmark.sh PATH_TO_FILE"
echo "      FILE=PATH_TO_FILE ./scripts/run_blob_benchmark.sh"
echo "DOCKER:"
echo "      ENGINE=podman ./scripts/run_blob_benchmark.sh PATH_TO_FILE"
echo "      ENGINE=podman FILE=PATH_TO_FILE ./scripts/run_blob_benchmark.sh"
echo ""
echo "! Important: You must call this script from the root directory."
echo "! Important: Working directory is ./blob so either use absolute paths or paths relative to ./blob directory."
echo "--"

FIRST_ARG="$1"
if [[ -v FIRST_ARG ]]; then 
   export FILE="$FIRST_ARG"
fi

if [[ ! -v FILE ]]; then 
   echo "You need to set FILE= env variable to point to a file that you want to benchmark"
   exit 1
fi

if [[ ! -v ENGINE ]]; then 
    cd blob
    cargo bench -p avail-blob --bench submit_data
fi

if [[ "$ENGINE" != "docker" && "$ENGINE" != "podman" ]]; then
    echo "ENGINE must be 'docker' or 'podman' (got: '$ENGINE')"
    exit 1
fi

$ENGINE build -f ./dockerfiles/blob-benchmark.Dockerfile --ignorefile ./dockerfiles/blob-benchmark.dockerignore .
$ENGINE run --rm -it myimg "$FILE"
