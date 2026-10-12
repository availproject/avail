# Table of Contents

Document for developer notes that do not belong to README.md

1. [Distro Cross-Compiling](#distro-cross-compiling)
2. [Manual Session Keys Insertion](#manual-session-keys-insertion)
3. [Try Runtime](#try-runtime)
4. [Fix For Libsrockdb Compile Error](#fix-for-libsrockdb-compile-error)

## Distro Cross-Compiling

We provide a simple script that allows us to cross-compile our app to different
linux distros. This is only supported if you are on x64 platform. Right now
cross-compiling from x64 to arm and vice versa is not supported.

Example:

```bash
ENGINE=podman ZIP="yes" DISTRO="ubuntu-2204" ./scripts/binaries/build.sh
# Supported engines: podman docker
# Supported distros: ubuntu-2204 ubuntu-2404 ubuntu-2606 fedora 44 debian 13
# ZIP="yes" additionally zips the final binary
```

## Manual Session Keys Insertion

There is the new way to manually insert session keys. For more information
contact our dev-ops team.

```
srseed="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66"  # SR Secret seed
edseed="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66"  # ED Secret seed
edkey="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66" # ED Public key (hex)
srkey="0x9a97a54175e8fdb81f411e19be9c267b2bdc80ab3af0bf2b3e0762eb55ab9e66" # SR Public key (hex)

curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"gran\",\"$edseed\",\"$edkey\"]}"
curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"babe\",\"$srseed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"imon\",\"$srseed\",\"$srkey\"]}"
curl --trace-ascii here.txt http://localhost:9944/ -H "Content-Type:application/json;charset=utf-8" -d "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"author_insertKey\",\"params\": [\"audi\",\"$srseed\",\"$srkey\"]}"
```

## Try Runtime

### New Way

TODO

```bash
# Install latest version of try-runtime-cli
cargo install --git https://github.com/paritytech/try-runtime-cli --locked

try-runtime --runtime ./target/release/wbuild/da-runtime/da_runtime.wasm on-runtime-upgrade live --uri wss://mainnet-rpc.avail.so/ws --at 0xa1b16c1efd889a9f17375ec4dd5c1b4351a2be17fa069564fced10d23b9b3836
```

### Old Way

```bash
cargo build --release --features try-runtime

# Change the path to the runtime file and the 'at' block hash
./target/release/avail-node try-runtime --runtime ./target/release/wbuild/da-runtime/da_runtime.wasm  execute-block --try-state none live --uri wss://mainnet-rpc.avail.so/ws --at 0x6e3102e893f5418222fe6aa9b72a8c72dfcf0aa0b2bd3c09f10e743d8470dd40
```

## Fix For Libsrockdb Compile Error

```
export CXXFLAGS="$CXXFLAGS -include cstdint"
cargo build
```
