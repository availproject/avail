# Avail-js

[![npm stable](https://img.shields.io/npm/v/avail-js-sdk?logo=npm&style=flat-square)](https://www.npmjs.com/package/avail-js-sdk)

Library to connect to Avail

## Introduction

The simplest ways to interact with the avail network.

## Installation

Pre-requisites:
[Node.js](https://nodejs.org/en/download/)

Install the latest stable version of the avail-js library by running this command:

```bash
npm install avail-js-sdk
```

## Structure

This SDK is split into two main parts:

1. **Polkadot JS Wrapper**: This allows you to use all the Polkadot JS functions and types to interact with the chain. For more information and documentation, please refer to the [Polkadot JS Documentation](https://polkadot.js.org/docs/).

2. **Opinionated SDK**: A simpler, more streamlined way to interact with the chain. It offers less customization but provides an easier interface. This SDK will be continuously improved to include everything needed for seamless chain interaction.

### Folder Structure

- **[`src/chain/`](https://github.com/availproject/avail/tree/main/avail-js/src/chain/)**: Contains the basics to initialize an API with the chain, serving as the Polkadot JS wrapper.
- **[`src/helpers/`](https://github.com/availproject/avail/tree/main/avail-js/src/helpers/)**: Includes basic helper functions that facilitate various tasks.
- **[`src/spec/`](https://github.com/availproject/avail/tree/main/avail-js/src/spec/)**: All types, RPC, and signed extensions related to Avail. These are crucial for interacting with the chain, including decoding chain data, transactions, and initiating transactions.
- **[`src/sdk/`](https://github.com/availproject/avail/tree/main/avail-js/src/sdk/)**: Contains all classes related to the SDK, representing the opinionated part of Avail-JS-SDK.

### Additional Information

- **[`./examples/README.md`](https://github.com/availproject/avail/tree/main/avail-js/examples/readme.md)**: Refer to this for examples utilizing the Polkadot JS wrapper.
- **[`./docs/README.md`](https://github.com/availproject/avail/tree/main/avail-js/docs/README.md)**: This contains documentation, interfaces, and examples using the opinionated SDK.

## Documentation and examples

- [Wrapper node usage](https://github.com/availproject/avail/tree/main/avail-js/examples/node-examples)
- [Wrapper web usage](https://github.com/availproject/avail/tree/main/avail-js/examples/next-example)
- [SDK node usage](https://github.com/availproject/avail/tree/main/avail-js/docs)

## Error Reporting

In case you encounter a bug, don't hesitate to [open an issue](https://github.com/availproject/avail/issues/new/choose) with the maximum amount of detail and we will deal with it as soon as possible.
