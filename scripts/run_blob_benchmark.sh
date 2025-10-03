#!/bin/bash

# echo "--"
# echo "[HELP] You can run this script in two ways"
# echo "NORMAL: (will execute cargo command)"
# echo "      ./scripts/run_blob_benchmark.sh PATH_TO_FILE"
# echo "      FILE=PATH_TO_FILE ./scripts/run_blob_benchmark.sh"
# echo "CONTAINER: (builds or runs existing containers)"
# echo "      1. ./scripts/run_blob_benchmark.sh podman/docker build"
# echo "      2. ./scripts/run_blob_benchmark.sh podman/docker run PATH_TO_FILE"
# echo ""
# echo "! Important: You must call this script from the root directory."
# echo "! Important: In NORMAL mode: Working directory is ./blob so either use absolute paths or paths relative to ./blob directory."
# echo "! Important: In DOCKER mode: Working directory is the root directory."
# echo ""
# echo "Example for a file name 32MiB in root directory:"
# echo "  NORMAL:"
# echo "      ./scripts/run_blob_benchmark.sh ./../32MiB"
# echo ""
# echo "  DOCKER:"
# echo "      ./scripts/run_blob_benchmark.sh podman build # This is not necessary if you already run it once"
# echo "      ./scripts/run_blob_benchmark.sh podman run ./32MiB"
# echo "--"

export CXXFLAGS="$CXXFLAGS -include cstdint"

# CONTAINER
FIRST_ARG="$1"
if [[ "$FIRST_ARG" == "docker" || "$FIRST_ARG" == "podman" ]]; then
    SECOND_ARG="$2"
    if [[ "$SECOND_ARG" == "run" ]]; then
        THIRD_ARG="$3"
        if [[ -v THIRD_ARG ]]; then 
            export FILE="$THIRD_ARG"
        fi

        if [[ ! -v FILE ]]; then 
            echo "You need to set FILE= env variable to point to a file that you want to benchmark"
            exit 1
        fi

        $FIRST_ARG run --rm -it -v ${PWD}:/work:z  myimg /work/"$FILE"
    elif [[ "$SECOND_ARG" == "build" ]]; then
        $FIRST_ARG build -f ./dockerfiles/blob-benchmark.Dockerfile --ignorefile ./dockerfiles/blob-benchmark.dockerignore .
    else
        echo "Supported comamnds: [build] [run]. Example: Docker build"
    fi

    exit 0
fi

# NORMAL
if [[ -v FIRST_ARG ]]; then 
   export FILE="$FIRST_ARG"
fi

if [[ ! -v FILE ]]; then 
   echo "You need to set FILE= env variable to point to a file that you want to benchmark"
   exit 1
fi

cd blob
cargo bench -p avail-blob --bench submit_data
