#!/bin/bash
export CXXFLAGS="$CXXFLAGS -include cstdint"

cargo build --release

if command -v tilix >/dev/null 2>&1
then
    tilix --title Alice -w "$PWD" -e "./target/release/avail-node --chain dev.tri --rpc-port 9944 --alice --tmp --listen-addr /ip4/0.0.0.0/tcp/30333 --listen-addr /ip6/::/tcp/30333 --rpc-max-request-size 500 --rpc-max-response-size 500"
    sleep 1
    tilix --title Bob -w "$PWD" -e "./target/release/avail-node --chain dev.tri --rpc-port 9945 --bob --tmp --listen-addr /ip4/0.0.0.0/tcp/30334 --listen-addr /ip6/::/tcp/30334 --rpc-max-request-size 500 --rpc-max-response-size 500"
    tilix --title Charlie -w "$PWD" -e "./target/release/avail-node --chain dev.tri --rpc-port 9946 --charlie --tmp --listen-addr /ip4/0.0.0.0/tcp/30335 --listen-addr /ip6/::/tcp/30335 --rpc-max-request-size 500 --rpc-max-response-size 500"
    exit 0
fi

if command -v  gnome-terminal >/dev/null 2>&1
then
    gnome-terminal -e "./target/release/avail-node --chain dev.tri --rpc-port 9944 --alice --tmp --listen-addr /ip4/0.0.0.0/tcp/30333 --listen-addr /ip6/::/tcp/30333 --rpc-max-request-size 500 --rpc-max-response-size 500"
    sleep 1
    gnome-terminal -e "./target/release/avail-node --chain dev.tri --rpc-port 9945 --bob --tmp --listen-addr /ip4/0.0.0.0/tcp/30334 --listen-addr /ip6/::/tcp/30334 --rpc-max-request-size 500 --rpc-max-response-size 500"
    gnome-terminal -e "./target/release/avail-node --chain dev.tri --rpc-port 9946 --charlie --tmp --listen-addr /ip4/0.0.0.0/tcp/30335 --listen-addr /ip6/::/tcp/30335 --rpc-max-request-size 500 --rpc-max-response-size 500"
fi
