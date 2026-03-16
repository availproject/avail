# Generate da-commitment package and publish

- cargo install wasm-pack
- chmod +x ./build_package.sh
- ./build_package.sh <x.x.x> (eg. `./build_package.sh 0.1.0`)
- If there is an issue: https://github.com/wasm-bindgen/wasm-bindgen/issues/4228#issuecomment-2693647441

# Usage

`build_commitments_js(data, params_version, babe_randomness, blob_hash)` returns:

- `commitment`
- `eval_point_seed`
- `eval_claim`

Simple Node usage:

```js
import { build_commitments_js } from "da-commitment-node";
import { keccakAsU8a } from "@polkadot/util-crypto";

const text = "Hello World!";
const encoder = new TextEncoder();
const data = encoder.encode(text);
const paramsVersion = 0;
const babeRandomness = new Uint8Array(32); // replace with real BABE randomness from chain
const blobHash = keccakAsU8a(data);

const metadata = build_commitments_js(data, paramsVersion, babeRandomness, blobHash);

console.log(Buffer.from(metadata.commitment).toString("hex"));
```

Simple Web usage:

```js
import init, { build_commitments_js } from "da-commitment-web";
import { keccakAsU8a } from "@polkadot/util-crypto";

const toHex = (bytes) =>
  Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");

await init();

const text = "Hello World!";
const encoder = new TextEncoder();
const data = encoder.encode(text);
const paramsVersion = 0;
const babeRandomness = new Uint8Array(32); // replace with real BABE randomness from chain
const blobHash = keccakAsU8a(data);

const metadata = build_commitments_js(data, paramsVersion, babeRandomness, blobHash);

console.log(toHex(metadata.commitment));
```

# Standalone build

If you just want to build locally by hand:

```bash
cd utils/da-commitment-js
cargo install wasm-pack
wasm-pack build --target nodejs --features wasm
mv pkg pkg_node
```

After that, the local Node package is in `pkg_node`.

# JS example

There is a small JS example in `examples/submit_blob_metadata.cjs`.

It shows how to:

- compute the FRI metadata with `build_commitments_js`
- build `dataAvailability.submitBlobMetadata`
- sign it
- submit it with `blob_submitBlob`

You can tweak the constants at the top of the file if needed, for example:

- `WS_URL`
- `TEXT`
- `APP_ID`
- `ERA_PERIOD`

Install and run:

```bash
cd utils/da-commitment-js/examples
npm install @polkadot/api @polkadot/keyring @polkadot/util @polkadot/util-crypto
node submit_blob_metadata.cjs
```
