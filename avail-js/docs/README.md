# Avail SDK documentation / examples

This directory contains documentation and examples related to the opinionated SDK and additional functionalities provided by the `avail-js-sdk`.
The following scripts have been tested with `ts-node v10.9.2 node v20.11.1 (npm v10.2.4)`.

## Running Examples in the Docs Directory

To run the examples provided in the `docs/extrinsics` directory, follow these steps:

1. Install `ts-node` globally if you haven't already:

```bash
npm i -g ts-node
```

2. From the avail-js/docs folder, install all necessary dependencies:

```bash
npm install
```

3. Ensure you're running a local Avail node. You can do this with the following command from the root directory:

```bash
cargo build --release
./target/release/avail-node --dev
```

You can also take the latest release from [Github](https://github.com/availproject/avail/releases)

4. To run any example script from the docs/extrinsics folder, use the following command format, replacing NAME_OF_THE_FILE with the actual file name you want to run:

```bash
ts-node ./docs/extrinsics/NAME_OF_THE_FILE.ts
```

For example, to run the staking_nominate.ts script:

```bash
ts-node ./docs/extrinsics/staking_nominate.ts
```

This will execute the chosen example script, showcasing how to interact with the Avail network using the avail-js-sdk.
