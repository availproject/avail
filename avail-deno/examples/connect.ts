/// The example showcases how to programmatically connect to a network
///

import { ApiPromise, WsProvider } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "../src/api_options.ts";

const endpoint = "wss://kate.avail.tools/ws";
const api = await ApiPromise.create({
	provider: new WsProvider(endpoint),
	rpc: API_RPC,
	types: API_TYPES,
	signedExtensions: API_EXTENSIONS,
});

// Retrieve the chain and node information via rpc calls
const [chain, nodeName, nodeVersion, runtimeVersion] = await Promise.all([
	api.rpc.system.chain(),
	api.rpc.system.name(),
	api.rpc.system.version(),
	api.rpc.state.getRuntimeVersion(),
]);

console.log(
	`Connected to chain ${chain} using ${nodeName}, node version ${nodeVersion} and spec version ${runtimeVersion.specVersion}`,
);
Deno.exit(0);
