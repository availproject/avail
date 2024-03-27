FROM ubuntu:22.04 as builder

# This installs all dependencies that we need (besides Rust).
RUN apt update -y && \
    apt install build-essential git clang curl libssl-dev llvm libudev-dev make cmake protobuf-compiler -y

ADD . ./workdir
WORKDIR "/workdir"

# Create output folder
RUN mkdir -p output

VOLUME ["/output"]
CMD bash
