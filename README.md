# Data Availability Node

[![Build status](https://github.com/maticnetwork/avail/actions/workflows/default.yml/badge.svg)](https://github.com/maticnetwork/avail/actions/workflows/default.yml) [![Code coverage](https://codecov.io/gh/maticnetwork/avail/branch/main/graph/badge.svg?token=OBX2NEE31T)](https://codecov.io/gh/maticnetwork/avail)

## Compile

    $> cargo build --release

## Run Node for Development

In **development mode** the node will run as a collator on a network which requires just one
collator to finalize blocks.
We also add `--tmp`, therefore the state will be deleted at the end of the process.

    $> cargo run --release -- --dev --tmp
    Finished release [optimized] target(s) in 0.41s
     Running `target/release/data-avail --dev --tmp`
    2022-02-14 11:13:35 Running in --dev mode, RPC CORS has been disabled.    
    2022-02-14 11:13:35 Polygon Avail Node    
    2022-02-14 11:13:35 ✌️  version 3.0.0-8983b6b-x86_64-linux-gnu    
    2022-02-14 11:13:35 ❤️  by Anonymous, 2017-2022    
    2022-02-14 11:13:35 📋 Chain specification: Avail-Dev    
    2022-02-14 11:13:35 🏷 Node name: mature-cub-8175    
    2022-02-14 11:13:35 👤 Role: AUTHORITY    
    2022-02-14 11:13:35 💾 Database: RocksDb at /tmp/substrateMHOPAE/chains/Dev/db/full    
    2022-02-14 11:13:35 ⛓  Native runtime: data-avail-1 (data-avail-1.tx1.au10)    
    2022-02-14 11:13:36 [#0] 🗳  Entering emergency mode: ElectionError::Fallback("NoFallback.")    
    2022-02-14 11:13:36 [0] 💸 genesis election provider failed due to ElectionError::Fallback("NoFallback.")    
    2022-02-14 11:13:36 [#0] 🗳  Entering emergency mode: ElectionError::Fallback("NoFallback.")    
    2022-02-14 11:13:36 [0] 💸 genesis election provider failed due to ElectionError::Fallback("NoFallback.")    
    2022-02-14 11:13:36 🔨 Initializing Genesis block/state (state: 0x1037…66da, header-hash: 0xc8fb…adad)    
    2022-02-14 11:13:36 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.    
    2022-02-14 11:13:36 ⏱  Loaded block-time = 20s from block 0xc8fb86cbd158e7f70c64bacfcff6436fa998e7270120db0436ee5d5cf560adad    
    2022-02-14 11:13:36 👶 Creating empty BABE epoch changes on what appears to be first startup.    
    2022-02-14 11:13:36 Using default protocol ID "sup" because none is configured in the chain specs    
    2022-02-14 11:13:36 🏷 Local node identity is: 12D3KooWHwN7qigNMETeDKkpEasGp2zDKmzHzVJdpJEQWjaoGDiW    
    2022-02-14 11:13:36 📦 Highest known block at #0    
    2022-02-14 11:13:36 〽️ Prometheus exporter started at 127.0.0.1:9615    
    2022-02-14 11:13:36 Listening for new connections on 127.0.0.1:9944.    
    2022-02-14 11:13:36 👶 Starting BABE Authorship worker    
    2022-02-14 11:13:40 🙌 Starting consensus session on top of parent 0xc8fb86cbd158e7f70c64bacfcff6436fa998e7270120db0436ee5d5cf560adad    
    2022-02-14 11:13:40 Rows: 1 Cols: 4 Size: 256    
    2022-02-14 11:13:40 Time to extend block 146.101µs    
    2022-02-14 11:13:40 Time to prepare 178.772µs    
    2022-02-14 11:13:40 Number of CPU cores: 16    
    2022-02-14 11:13:40 Time to build a commitment 1.290286ms    
    2022-02-14 11:13:40 🎁 Prepared block for proposing at 1 (9 ms) [hash: 0x285b0ebcac3f335957dd85cc1e61a0b59334c8b98348d3f714cf76d58e517463; parent_hash: 0xc8fb…adad; extrinsics (1): [0xa017…6bae]]    
    2022-02-14 11:13:40 🔖 Pre-sealed block for proposal at 1. Hash now 0x6edbe749ee150f18ee1ce4e89334d2ea45e2baed9b7cb5ae93104175e9408dbc, previously 0x285b0ebcac3f335957dd85cc1e61a0b59334c8b98348d3f714cf76d58e517463.    
    2022-02-14 11:13:40 👶 New epoch 0 launching at block 0x6edb…8dbc (block slot 82241681 >= start slot 82241681).    
    2022-02-14 11:13:40 👶 Next epoch starts at slot 82241711    
    2022-02-14 11:13:40 ✨ Imported #1 (0x6edb…8dbc)    
    2022-02-14 11:13:41 💤 Idle (0 peers), best: #1 (0x6edb…8dbc), finalized #0 (0xc8fb…adad), ⬇ 0 ⬆ 0    
    2022-02-14 11:13:46 💤 Idle (0 peers), best: #1 (0x6edb…8dbc), finalized #0 (0xc8fb…adad), ⬇ 0 ⬆ 0    
    2022-02-14 11:13:51 💤 Idle (0 peers), best: #1 (0x6edb…8dbc), finalized #0 (0xc8fb…adad), ⬇ 0 ⬆ 0    
    2022-02-14 11:13:56 💤 Idle (0 peers), best: #1 (0x6edb…8dbc), finalized #0 (0xc8fb…adad), ⬇ 0 ⬆ 0    
    2022-02-14 11:14:00 🙌 Starting consensus session on top of parent 0x6edbe749ee150f18ee1ce4e89334d2ea45e2baed9b7cb5ae93104175e9408dbc    
    2022-02-14 11:14:00 Rows: 1 Cols: 4 Size: 256    
    2022-02-14 11:14:00 Time to extend block 182.71µs    
    2022-02-14 11:14:00 Time to prepare 222.653µs    
    2022-02-14 11:14:00 Number of CPU cores: 16    
    2022-02-14 11:14:00 Time to build a commitment 2.064504ms    
    2022-02-14 11:14:00 🎁 Prepared block for proposing at 2 (2 ms) [hash: 0x6c4cfdf28ceeb07599f6abc2358e81afc770c3edd6b90cced5f1a370972bab42; parent_hash: 0x6edb…8dbc; extrinsics (1): [0x4c7c…b8ef]]    
    2022-02-14 11:14:00 🔖 Pre-sealed block for proposal at 2. Hash now 0x66c23089eeee13e71a4a970a318b1f921f9ca6501a36a31d20840adc5979848e, previously 0x6c4cfdf28ceeb07599f6abc2358e81afc770c3edd6b90cced5f1a370972bab42.    
    2022-02-14 11:14:00 ✨ Imported #2 (0x66c2…848e)   
    ...

## Run benchmarks

You can run any benchmark and generate the proper `weight.rs` file. In the following command, we are
running the benchmarks from `da-control` pallet, and the generated file is 

    $> cargo run --release --features runtime-benchmarks -- \
        benchmark \
        --chain=dev \
        --steps=20 \
        --repeat=30 \
        --log=warn \
        --execution=wasm \
        --wasm-execution=compiled \
        --template=./.maintain/frame-weight-template.hbs \
        --pallet=da-control \
        --extrinsic=* \
        --output=./pallets/dactr/src/weights.rs



## Transaction Custom IDs

Here is the table of custom IDs for invalid transaction errors:

| Custom ID | Name                | Description |
| --------- | ------------------- | ----------- |
| 137       | InvalidAppId        | The given `AppId` is not yet registered |
| 138       | ForbiddenAppId      | The extrinsic is not available for the given `AppId` |

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
