import { Keyring, SDK, WaitFor } from "../../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice//stash");

const result = await sdk.tx.staking.chill(WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
