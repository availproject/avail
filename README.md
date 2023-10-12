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
cargo run --locked --release -- --chain kate -d ./output
```
This command complies and runs the Avail Node connected to the Kate Network.

```
2023-10-11 16:11:31 Avail Node    
2023-10-11 16:11:31 ✌️  version 1.7.0-ad024ff050e    
2023-10-11 16:11:31 ❤️  by Anonymous, 2017-2023    
2023-10-11 16:11:31 📋 Chain specification: Avail Kate Testnet    
2023-10-11 16:11:31 🏷  Node name: decorous-trade-0251    
2023-10-11 16:11:31 👤 Role: FULL    
2023-10-11 16:11:31 💾 Database: RocksDb at /tmp/substrateJwM8xd/chains/Avail Testnet_116d7474-0481-11ee-bc2a-7bfc086be54e/db/full    
2023-10-11 16:11:32 🔨 Initializing Genesis block/state (state: 0x6bc8…8ac6, header-hash: 0xd120…50c6)    
2023-10-11 16:11:32 👴 Loading GRANDPA authority set from genesis on what appears to be first startup.    
2023-10-11 16:11:33 👶 Creating empty BABE epoch changes on what appears to be first startup.    
2023-10-11 16:11:33 🏷  Local node identity is: 12D3KooWMmY2QLodvBGSiP1Cg9ysWrPSMN19qK3w35mRnUhq6pMX    
2023-10-11 16:11:33 Prometheus metrics extended with avail metrics    
2023-10-11 16:11:33 💻 Operating system: linux    
2023-10-11 16:11:33 💻 CPU architecture: x86_64    
2023-10-11 16:11:33 💻 Target environment: gnu    
2023-10-11 16:11:33 💻 CPU: 13th Gen Intel(R) Core(TM) i7-13700K    
2023-10-11 16:11:33 💻 CPU cores: 16    
2023-10-11 16:11:33 💻 Memory: 31863MB    
2023-10-11 16:11:33 💻 Kernel: 6.5.5-100.fc37.x86_64    
2023-10-11 16:11:33 💻 Linux distribution: Fedora Linux 37 (Workstation Edition)    
2023-10-11 16:11:33 💻 Virtual machine: no    
2023-10-11 16:11:33 📦 Highest known block at #0    
2023-10-11 16:11:33 〽️ Prometheus exporter started at 127.0.0.1:9615    
2023-10-11 16:11:33 Running JSON-RPC server: addr=127.0.0.1:9944, allowed origins=["http://localhost:*", "http://127.0.0.1:*", "https://localhost:*", "https://127.0.0.1:*", "https://polkadot.js.org"]    
2023-10-11 16:11:33 🏁 CPU score: 1.65 GiBs    
2023-10-11 16:11:33 🏁 Memory score: 19.49 GiBs    
2023-10-11 16:11:33 🏁 Disk score (seq. writes): 6.74 GiBs    
2023-10-11 16:11:33 🏁 Disk score (rand. writes): 2.65 GiBs    
2023-10-11 16:11:33 🔍 Discovered new external address for our node: /ip4/176.61.156.176/tcp/30333/ws/p2p/12D3KooWMmY2QLodvBGSiP1Cg9ysWrPSMN19qK3w35mRnUhq6pMX    
2023-10-11 16:11:34 [811] 💸 generated 9 npos targets    
2023-10-11 16:11:34 [811] 💸 generated 9 npos voters, 9 from validators and 0 nominators    
2023-10-11 16:11:34 [#811] 🗳  creating a snapshot with metadata SolutionOrSnapshotSize { voters: 9, targets: 9 }    
2023-10-11 16:11:34 [#811] 🗳  Starting phase Signed, round 1.
```

#### Running Dev Chain
A development chain is typically used for testing and development purposes.
```bash
cargo run --locked --release -- --chain dev
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