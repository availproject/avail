export CXXFLAGS="$CXXFLAGS -include cstdint"

#cargo build --release
tilix --title Alice -w "$PWD" -e "./target/release/avail-node --chain dev.tri --rpc-port 9944 --alice --tmp --listen-addr /ip4/0.0.0.0/tcp/30333 --listen-addr /ip6/::/tcp/30333 --rpc-max-request-size 500 --rpc-max-response-size 500" &
PID1="$!"
sleep 1
tilix --title Bob -w "$PWD" -e "./target/release/avail-node --chain dev.tri --rpc-port 9945 --bob --tmp --listen-addr /ip4/0.0.0.0/tcp/30334 --listen-addr /ip6/::/tcp/30334 --rpc-max-request-size 500 --rpc-max-response-size 500" & 
PID2="$!"
tilix --title Charlie -w "$PWD" -e "./target/release/avail-node --chain dev.tri --rpc-port 9946 --charlie --tmp --listen-addr /ip4/0.0.0.0/tcp/30335 --listen-addr /ip6/::/tcp/30335 --rpc-max-request-size 500 --rpc-max-response-size 500" &
PID3="$!"
