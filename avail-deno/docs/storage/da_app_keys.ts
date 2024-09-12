import { SDK } from "https://raw.githubusercontent.com/availproject/avail/main/avail-deno/src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

const key = "Reserved-2";
const value = await sdk.api.query.dataAvailability.appKeys(key);
console.log(value.toHuman());

Deno.exit();
