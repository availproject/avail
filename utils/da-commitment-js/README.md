# Generate da-commitment package and publish

- cargo install wasm-pack
- chmod +x ./build_package.sh
- ./build_package.sh <x.x.x> (eg. `./build_package.sh 0.1.0`)
- If there is an issue: https://github.com/wasm-bindgen/wasm-bindgen/issues/4228#issuecomment-2693647441

# Usage

## Node
- `npm install da-commitment-node`

```js
import { build_commitments_js } from 'da-commitment-node';

const text = "aaaaa";
const encoder = new TextEncoder();
const strBytes = encoder.encode(text);
const commitmentsFromString = build_commitments_js(strBytes, 1024, 4096);
const hexFromString = Buffer.from(commitmentsFromString).toString('hex');
```

## web
- `npm install da-commitment-web`

```js
import init, { build_commitments_js } from "da-commitment-web";

await init()
const arrayBuffer = await file.arrayBuffer(); // We use file but we can do the same as above for text
const fileData = new Uint8Array(arrayBuffer);
const commitment = build_commitments_js(fileData, 1024, 4096);
```