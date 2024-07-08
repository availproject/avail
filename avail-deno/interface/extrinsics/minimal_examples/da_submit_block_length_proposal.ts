import { Keyring, SDK, WaitFor } from "../../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const rows = 128;
const cols = 128;

const result = await sdk.tx.dataAvailability.submitBlockLengthProposal(rows, cols, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Rows=" + result.event.rows + ", Cols=" + result.event.cols);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
