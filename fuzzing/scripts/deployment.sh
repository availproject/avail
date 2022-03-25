#!/bin/bash

# The script requires an alredy setup Rust env, Avail repo and installed AFL.rs
echo "Starting AFL instances..." 

target_binary=$1
num_of_instances=$2


if [[ (-z "$target_binary") || (-z "$num_of_instances") ]]; then
    echo "Must provide target_binary and num_of_instances in environment" 1>&2
    echo "Usage: $0 target_binary num_of_instances" 1>&2
    exit 1
fi

# TODO: automate full Rust, Substrate, Avail and AFL initial setup

# Execute fuzzers in parallel 
# Create a single obligatory master instance
cd ..
cargo afl fuzz -i testing/in -o testing/out -M fuzzer01 ../target/debug/$target_binary > /dev/null 2>&1 & 

# Start slave instances in background
for (( i=2; i<=$num_of_instances ; i++ ))
do
cargo afl fuzz -i testing/in -o testing/out -S fuzzer0${i} ../target/debug/$target_binary > /dev/null 2>&1 &
done

# Wait for the last instance to start
# TODO: replace sleep with more elegant way of detecting fuzzers starting (ie. parse fuzzer_stats file for time started)
echo "Waiting for fuzzers to start..."
sleep 5

# Print out fuzzer summary
cargo afl whatsup -s testing/out/
