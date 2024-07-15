import { BN, Keyring, SDK, WaitFor } from "../../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const value = new BN(100_000).mul(new BN(10).pow(new BN("18"))); // 100 000 Avail
const payee = "Staked";

const result = await sdk.tx.staking.bond(value, payee, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log("Stash=" + result.event.stash + ", Amount=" + result.event.amount);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
