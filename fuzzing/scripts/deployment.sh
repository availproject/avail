#!/bin/bash

echo "Starting AFL instances..." 

target_binary=$1
num_of_instances=$2

# TODO: automate full Rust, Substrate, Avail and AFL initial setup

# git clone https://github.com/maticnetwork/avail.git
# cargo install afl

# Build the AFL instrumented binary
# cd avail/fuzzing
# cargo afl build 

# Execute fuzzers in parallel 
# Create a single obligatory master instance
cd ..
cargo afl fuzz -i testing/in -o testing/out -M fuzzer01 ../target/debug/$target_binary

# Start slave instances in background
for (( i=2; i<=$num_of_instances ; i++ ))
do
echo cargo afl fuzz -i testing/in -o testing/out -S fuzzer0${i} ../target/debug/$target_binary
cargo afl fuzz -i testing/in -o testing/out -S fuzzer0${i} ../target/debug/$target_binary &
done