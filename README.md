<div align="Center">
<h1>Avail Node</h1>
<h3>Official Client for the Avail blockchain</h3>
</div>

<br>

[![Build status](https://github.com/availproject/avail/actions/workflows/default.yml/badge.svg)](https://github.com/availproject/avail/actions/workflows/default.yml)


![demo](./.github/img/terminal.jpg)

## Running Avail Node
> Before running make sure that you have the following dependencies installed:
> - [Rust](https://www.rust-lang.org/learn/get-started)
> - [Substrate dependencies](https://docs.substrate.io/install/)


Command for compiling and running Avail Node:
```bash
cargo run --release -- --dev
```

```bash
2023-10-11 10:11:40 Avail Node    
2023-10-11 10:11:40 ✌️  version 1.7.0-56c3d20e255    
2023-10-11 10:11:40 ❤️  by Anonymous, 2017-2023    
2023-10-11 10:11:40 📋 Chain specification: Avail Development Network    
2023-10-11 10:11:40 🏷  Node name: impartial-size-5902    
2023-10-11 10:11:40 👤 Role: AUTHORITY    
2023-10-11 10:11:40 💾 Database: RocksDb at /tmp/substrateV7xYbu/chains/avail_development_network/db/full    
2023-10-11 10:11:41 [0] 💸 generated 1 npos voters, 1 from validators and 0 nominators    
2023-10-11 10:11:41 [0] 💸 generated 1 npos targets    
2023-10-11 10:11:41 🔨 Initializing Genesis block/state (state: 0xd56d…4fdb, header-hash: 0xf69a…6fae)    
2023-10-11 10:11:41 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.    
2023-10-11 10:11:41 👶 Creating empty BABE epoch changes on what appears to be first startup.    
2023-10-11 10:11:41 🏷  Local node identity is: 12D3KooWBDYA3aovETHf2xRM9qkp6XjTceuXNcj6MF6hyyfsW4Pc    
2023-10-11 10:11:41 Prometheus metrics extended with avail metrics    
2023-10-11 10:11:41 💻 Operating system: linux    
2023-10-11 10:11:41 💻 CPU architecture: x86_64    
2023-10-11 10:11:41 💻 Target environment: gnu    
2023-10-11 10:11:41 💻 CPU: 13th Gen Intel(R) Core(TM) i7-13700K    
2023-10-11 10:11:41 💻 CPU cores: 16    
2023-10-11 10:11:41 💻 Memory: 31863MB    
2023-10-11 10:11:41 💻 Kernel: 6.5.5-100.fc37.x86_64    
2023-10-11 10:11:41 💻 Linux distribution: Fedora Linux 37 (Workstation Edition)    
2023-10-11 10:11:41 💻 Virtual machine: no    
2023-10-11 10:11:41 📦 Highest known block at #0    
2023-10-11 10:11:41 Running JSON-RPC server: addr=127.0.0.1:9944, allowed origins=["*"]    
2023-10-11 10:11:41 🏁 CPU score: 1.64 GiBs    
2023-10-11 10:11:41 〽️ Prometheus exporter started at 127.0.0.1:9615    
2023-10-11 10:11:41 🏁 Memory score: 22.75 GiBs    
2023-10-11 10:11:41 🏁 Disk score (seq. writes): 6.84 GiBs    
2023-10-11 10:11:41 🏁 Disk score (rand. writes): 2.74 GiBs    
2023-10-11 10:11:41 👶 Starting BABE Authorship worker    
2023-10-11 10:11:46 💤 Idle (0 peers), best: #0 (0xf69a…6fae), finalized #0 (0xf69a…6fae), ⬇ 0 ⬆ 0    
2023-10-11 10:11:51 💤 Idle (0 peers), best: #0 (0xf69a…6fae), finalized #0 (0xf69a…6fae), ⬇ 0 ⬆ 0    
2023-10-11 10:11:56 💤 Idle (0 peers), best: #0 (0xf69a…6fae), finalized #0 (0xf69a…6fae), ⬇ 0 ⬆ 0    
2023-10-11 10:12:00 🙌 Starting consensus session on top of parent 0xf69adf40a4fb0b7545a0ef18d9e9de0526025d7df3b92c27c6782e22f8016fae    
2023-10-11 10:12:00 🎁 Prepared block for proposing at 1 (25 ms) [hash: 0x1e21edfa830cfd74a84a71b66499d940acc36b3f0842c54900e00122f4ba746e; parent_hash: 0xf69a…6fae; extrinsics (1): [0x8e09…8593]
```


## Relative documentation
- [Changelog](/CHANGELOG.md)
- [Contributing guide](/CONTRIBUTING.md)
- [Code of conduct](/CODE_OF_CONDUCT.md)


## Transaction Custom IDs

Here is the table of custom IDs for invalid transaction errors:

| Custom ID | Name                 | Description |
| --------- | -------------------- | ----------- |
| 137       | InvalidAppId         | The given `AppId` is not yet registered |
| 138       | ForbiddenAppId       | The extrinsic is not available for the given `AppId` |
| 139       | MaxPaddedLenExceeded | The maximum padded length for a block was exceeded |
| 140       | MaxRecursionExceeded | The maximum recursion was reached for a call with `AppId != 0` |


## Sychronize the chain
### Chainspec
To synchronize your node the chain, you have access to the [chainspec](https://kate.avail.tools/chainspec.json) and most importantly the [raw chainspec](https://kate.avail.tools/chainspec.raw.json).

### Sync mode
You can sync to the chain using:
- Full mode: This is the default if nothing is specified and will download all the blocks data, you can also use `--sync full`.
- Warp mode: This is will download the latest state then all the blocks data. It's the fastest way to have a running node. Use `--sync warp`.
  - This is theoritically handled but is not supported and can fail after drastic updates.
- Fast / Fast Unsafe: This is currently not supported since it does not download data needed for Avail specific computation.

### Unsafe sync
When importing blocks, their content go through an additional check to make sure that the DA commitments are valid.
During initial sync, you can chose to ignore this check to increase the sync speed. This command is compatible with any `sync` mode.
- `--unsafe-da-sync`
Using this flag, Warp / Fast / Fast unsafe become compatible with syncing. 
You can then remove this flag node and restart it.

### Chain Sync Modes and Compatibility

This describes the different sync modes available for the substrate chain and their compatibility with the `--unsafe-da-sync` flag.
#### Compatibility Table

| Sync Mode          | Without `--unsafe-da-sync` | With `--unsafe-da-sync`                                                            |
|--------------------|----------------------------|------------------------------------------------------------------------------------|
| full               | compatible                 | compatible                                                                         |
| warp               | partially compatible       | compatible                                                                         |
| fast / fast unsafe | not compatible             | compatible (with [warnings](https://github.com/paritytech/polkadot-sdk/issues/19)) |



## Generate test code coverage report

We are using [grcov](https://github.com/mozilla/grcov) to aggregate code coverage information and generate reports.

To install grcov run

	$> cargo install grcov

Source code coverage data is generated when running tests with

	$> env RUSTFLAGS="-Zinstrument-coverage" \
		SKIP_WASM_BUILD=true \
		LLVM_PROFILE_FILE="tests-coverage-%p-%m.profraw" \
		cargo test

To generate report, run

	$> grcov . -s . \
		--binary-path ./target/debug/ \
		-t html \
		--branch \
		--ignore-not-existing -o \
		./target/debug/coverage/

To clean up generate coverage information files, run

	$> find . -name \*.profraw -type f -exec rm -f {} +

Open `index.html` from `./target/debug/coverage/` folder to review coverage data. Since WASM build is not possible yet, SKIP_WASM_BUILD is required when running tests.

## Runtime upgrades

Substrate development framework supports forkless upgrades of the runtime. Update is triggered when `spec_version` field of `RuntimeVersion` in `runtimeime/src/lib.rs` is incremented.

### Build and optimize WASM runtime

Use [srtool cli](https://github.com/chevdor/srtool-cli) to compile WASM runtime:

	$> srtool build -r runtime/ --package da-runtime

WASM runtime is already optimized by `srtool` with `wasm-opt` from [Binaryen](https://github.com/WebAssembly/binaryen). If needed, WASM runtime can be further optimized by using:

	$> wasm-opt -Oz -o ./da_runtime.compact.wasm \
		./runtime/target/srtool/release/wbuild/da-runtime/da_runtime.compact.wasm

### Upgrade process

Since we have block size limits, runtime upgrade is a three step process. Preferred way to upgrade runtime is through governance/democracy feature. For each step, submit preimage with changes, and use preimage hash to submit proposal. Steps are:

1. Submit `dataAvailability/submit_block_length_proposal` proposal with increased block size limits (eg. 512 rows x 256 columns)
2. Submit `system/set_code` proposal with uploaded `da_runtime.compact.wasm`
3. Submit `dataAvailability/submit_block_length_proposal` proposal with block limits reverted to initial setting

For development purposes, its possible to use sudo calls with unchecked weight to increase block size limits and upload new runtime. In that case, steps are:

1. Use `sudo/sudoCall` to invoke `dataAvailability/submit_block_length_proposal` with increased block limits (eg. 512 rows x 256 columns)
2. Use `sudo/sudoUncheckedWeight(call, weight)` with 0 weight to invoke `system/set_code` and upload `da_runtime.compact.compressed.wasm`
3. Use `sudo/sudoCall` to invoke `dataAvailability/submit_block_length_proposal` and revert block limits to initial setting

### Verify upgrade

To check if runtime is upgraded, query `system/version:SpVersionRuntimeVersion` constant. This should return latest version values.

## Testing

### Generating blocks of maximum size

Some load testing cases requires blocks of maximum size. To compile node which will always pad block with random data up to maximum block size, compile node with:

	$> cargo build -p data-avail --features "kate/maximum-block-size"

## Docker build

The easiest way to build and deploy your own node is using docker.

### Build the docker image

We recommend the use of `BuildKit`, and specify the branch/tag you want to build. The following
example shows the latest tag for devnet:

	$> export DOCKER_BUILDKIT=1
	$> docker build --build-arg AVAIL_TAG=v1.3.0-rc3 -t avail:v1.3.0-rc3 .

### How to use this image

#### Run an Avail Node

	$> docker run avail:v1.3.0-rc3

### Where to Store Data

There are two main volumes:
  - `/da/state`, where the state of the blockchain is stored.
  - `/da/keystore`, where the keystore is stored.

You can bind to a host folder if you want to persist them even after remove the container:

	$> docker run -v (pwd)/state:/da/state avail:v1.3.0-rc3


### How to customize the node

This image uses several environmental variables to customize the node:

#### DA_CHAIN

It sets the chainspec file used by the node. The default value is `/da/genesis/chainspec.raw.json`,
which allows connection to `devnet06`. You can also customize it by the build argument `CHAIN_SPEC`.

#### DA_NAME

The human-readable name for this node. By default, "AvailNode" is used.

#### DA_MAX_IN_PEERS

The maximum number of incoming connections we're accepting. The default value is `50`.

#### DA_MAX_OUT_PEERS

The number of outgoing connections we're trying to maintain. Default value is `50`.

#### DA_P2P_PORT

Specify p2p protocol TCP port. Default value is `30333`.

#### BOOTNODE_1, BOOTNODE_2, and BOOTNODE_3

Defines 3 bootnodes. By default, `devnet06`'s bootnodes are loaded.

