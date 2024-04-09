<div align="Center">
<h1>Avail Node</h1>
<h3>Official Client for the Avail blockchain</h3>
</div>

<br>

[![Build status](https://github.com/availproject/avail/actions/workflows/default.yml/badge.svg)](https://github.com/availproject/avail/actions/workflows/default.yml)


![demo](./.github/img/terminal.jpg)

## Running Avail Node
### Manually

> To manually run the Avail Node, you'll need to have the following dependencies installed:
> - [Rust](https://www.rust-lang.org/learn/get-started)
> - [Substrate dependencies](https://docs.substrate.io/install/)


After ensuring you have the dependencies installed, you can run the Avail Node using the following command:
```bash
mkdir -p output
cargo run --locked --release -- --chain mainnet -d ./output
```
This command compiles and runs the Avail Node connected to the Mainnet Network.

```
2024-03-05 12:31:25 Avail Node    
2024-03-05 12:31:25 ✌️  version 2.0.0-7d6ed7f7141    
2024-03-05 12:31:25 ❤️  by Avail Team, 2017-2024    
2024-03-05 12:31:25 📋 Chain specification: Avail Development Network    
2024-03-05 12:31:25 🏷  Node name: Alice    
2024-03-05 12:31:25 👤 Role: AUTHORITY    
2024-03-05 12:31:25 💾 Database: ParityDb at /tmp/substrateecX1Gm/chains/avail_development_network/paritydb/full    
2024-03-05 12:31:27 🔨 Initializing Genesis block/state (state: 0x7d28…a6c8, header-hash: 0x1074…234a)    
2024-03-05 12:31:27 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.    
2024-03-05 12:31:28 👶 Creating empty BABE epoch changes on what appears to be first startup.    
2024-03-05 12:31:28 🏷  Local node identity is: 12D3KooWQDpM9w2zcvUaJS89KVAEdXsDFcBrTtc8awHGusRwSCEr    
2024-03-05 12:31:28 Prometheus metrics extended with avail metrics    
2024-03-05 12:31:28 💻 Operating system: linux    
2024-03-05 12:31:28 💻 CPU architecture: x86_64    
2024-03-05 12:31:28 💻 Target environment: gnu    
2024-03-05 12:31:28 💻 CPU: 13th Gen Intel(R) Core(TM) i7-13700K    
2024-03-05 12:31:28 💻 CPU cores: 16    
2024-03-05 12:31:28 💻 Memory: 31873MB    
2024-03-05 12:31:28 💻 Kernel: 6.7.6-200.fc39.x86_64    
2024-03-05 12:31:28 💻 Linux distribution: Fedora Linux 39 (Workstation Edition)    
2024-03-05 12:31:28 💻 Virtual machine: no    
2024-03-05 12:31:28 📦 Highest known block at #0    
2024-03-05 12:31:28 〽️ Prometheus exporter started at 127.0.0.1:9615    
2024-03-05 12:31:28 Running JSON-RPC server: addr=127.0.0.1:9944, allowed origins=["http://localhost:*", "http://127.0.0.1:*", "https://localhost:*", "https://127.0.0.1:*", "https://polkadot.js.org"]    
2024-03-05 12:31:28 🏁 CPU score: 1.65 GiBs    
2024-03-05 12:31:28 🏁 Memory score: 22.52 GiBs    
2024-03-05 12:31:28 🏁 Disk score (seq. writes): 7.00 GiBs    
2024-03-05 12:31:28 🏁 Disk score (rand. writes): 2.77 GiBs    
2024-03-05 12:31:28 👶 Starting BABE Authorship worker    
2024-03-05 12:31:33 💤 Idle (0 peers), best: #0 (0x1074…234), finalized #0 (0x1074…234a), ⬇ 0 ⬆ 0    
2024-03-05 12:31:38 💤 Idle (0 peers), best: #0 (0x1074…234a), finalized #0 (0x1074…234a), ⬇ 0 ⬆ 0   
```

### Supported Chains
#### Development
A development chain is typically used for testing and development purposes.
```bash
cargo run --locked --release -- --dev
```

#### Testnet New
```bash
cargo run --locked --release -- --chain new
```

#### Mainnet
```bash
cargo run --locked --release -- --chain mainnet
```

### Docker
To run the Avail Node using Docker, follow these steps:

```bash
# Build the Docker image for the Avail Node:
docker build -t availnode -f ./dockerfiles/avail-node.Dockerfile .

# Create an output directory. Here the node's data will be stored.
mkdir output

# Run the Avail Node container:
docker run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output availnode
# For SELinux
docker run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output:z availnode
```

#### Running Dev Chain
There are instructions for running a development chain using Docker. A development chain is typically used for testing and development purposes.

```bash
# Build the Docker image for the Avail Node:
docker build -t availnode -f ./dockerfiles/avail-node.Dockerfile .

# Create an output directory. Here the node's data will be stored.
mkdir output

# Run the Avail Node container:
docker run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output availnode --dev --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all
# For SELinux
docker run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output:z availnode --dev --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all
```

### Podman
To run the Avail Node using Docker, follow these steps:

```bash
# Build the Docker image for the Avail Node:
podman build -t availnode -f ./dockerfiles/avail-node.Dockerfile .

# Create an output directory. Here the node's data will be stored.
mkdir output

# Run the Avail Node container:
podman run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output availnode
# For SELinux
podman run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output:z availnode
```

#### Running Dev Chain
There are instructions for running a development chain using Podman. A development chain is typically used for testing and development purposes.

```bash
# Build the Docker image for the Avail Node:
podman build -t availnode -f ./dockerfiles/avail-node.Dockerfile .

# Create an output directory. Here the node's data will be stored.
mkdir output

# Run the Avail Node container:
podman run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output availnode --dev --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all
# For SELinux
podman run --rm -p 30333:30333 -p 9944:9944 -v ./output:/output:z availnode --dev --rpc-methods=unsafe --unsafe-rpc-external --rpc-cors=all
```

## Kate RPC
To enable Kate RPC you need to pass `--enable-kate-rpc` flag when executing the binary.
`--dev` implies `--enable-kate-rpc`.

```bash
./avail-node --enable-kate-rpc
```

## All Custom Flags
```bash
--enable-kate-rpc
    Enable Kate RPC

--enable-kate-rpc-metrics
    Enable Kate RPC Metrics

--kate-max-cells-size <KATE_MAX_CELLS_SIZE>
    The maximum number of cells that can be requested in one go.
    
    Max size cannot exceed 10_000
    
    [default: 64]
```

## Run Benchmarks
### Kate RPC
```bash
./avail-node --dev
deno run -A ./examples/deno/benchmarks/query_proof.ts && deno run -A ./examples/deno/benchmarks/query_rows.ts && deno run -A ./examples/deno/benchmarks/query_block_length.ts && deno run -A ./examples/deno/benchmarks/query_data_proof.ts
```

### Header Builder
```bash
# Option 1: for time measurement 
cargo bench --bench header_kate_commitment_cri
# Option 2: for time measurement 
cargo bench --bench header_kate_commitment_divan
# Option 1: for instructions, cache and main memory hits
cargo bench --bench header_kate_commitment_iai_callgrind
# Option 2: for instructions, cache and main memory hits
cargo bench --bench header_kate_commitment_iai
```

## Additional Documentation
For additional documentation check our [wiki page](https://github.com/availproject/avail/wiki).
There you can learn how to:
- Run Avail Node together with Avail Light Clients
- Build Avail Node for different Linux flavours
- Find out what node synchronization options are available
- Running Avail Benchmarks


## Interact with the chain
You can find on this repository many example on how to interact with any avail chain.
- In the avail-js folder, you will find our wrapper for polkadot js including multiple helpers.
    - The example folder contains some examples using node-js and an example web app to setup the extension.
- In the avail-subxt folder, you will find our fork of subxt with some example on usage.
- In the examples folders you will find examples for:
  - Deno examples
  - Go examples
