#!/bin/bash
export CXXFLAGS="$CXXFLAGS -include cstdint"

cargo build --release

COMMON="./target/release/avail-node --chain dev.tri --tmp --rpc-max-request-size 500 --rpc-max-response-size 500 --telemetry-url 'ws://localhost:8001/submit 0'"

if command -v tilix >/dev/null 2>&1
then
    tilix --title Alice -w "$PWD" -e "$COMMON --rpc-port 9944 --alice --listen-addr /ip4/0.0.0.0/tcp/30333 --listen-addr /ip6/::/tcp/30333"
    sleep 1
    tilix --title Bob -w "$PWD" -e "$COMMON --rpc-port 9945 --bob --listen-addr /ip4/0.0.0.0/tcp/30334 --listen-addr /ip6/::/tcp/30334"
    tilix --title Charlie -w "$PWD" -e "$COMMON --rpc-port 9946 --charlie --listen-addr /ip4/0.0.0.0/tcp/30335 --listen-addr /ip6/::/tcp/30335"
    exit 0
fi

if command -v  gnome-terminal >/dev/null 2>&1
then
    gnome-terminal -e "$COMMON --rpc-port 9944 --alice --listen-addr /ip4/0.0.0.0/tcp/30333 --listen-addr /ip6/::/tcp/30333"
    sleep 1
    gnome-terminal -e "$COMMON --rpc-port 9945 --bob --listen-addr /ip4/0.0.0.0/tcp/30334 --listen-addr /ip6/::/tcp/30334"
    gnome-terminal -e "$COMMON --rpc-port 9946 --charlie --listen-addr /ip4/0.0.0.0/tcp/30335 --listen-addr /ip6/::/tcp/30335"
fi
