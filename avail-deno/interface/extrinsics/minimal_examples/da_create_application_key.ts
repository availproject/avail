import { Keyring, SDK, WaitFor } from "../../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const key = "MyAwesomeKey";

const result = await sdk.tx.dataAvailability.createApplicationKey(key, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Key=" + result.event.key + ", Owner=" + result.event.owner + ", Id=" + result.event.id);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
