import { SDK } from "https://raw.githubusercontent.com/availproject/avail/main/avail-deno/src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

const appKeys = await sdk.api.query.dataAvailability.appKeys.entries();
for (const [key, value] of appKeys) {
	console.log(key.toHuman());
	console.log(value.toHuman());
}

Deno.exit();
