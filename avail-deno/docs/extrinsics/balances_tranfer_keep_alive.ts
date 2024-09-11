import { BN, Keyring, SDK, WaitFor } from "https://raw.githubusercontent.com/availproject/avail/main/avail-deno/src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"; // Eve
const amount = new BN(10).pow(new BN(18)); // one Avail

const result = await sdk.tx.balances.transferKeepAlive(dest, amount, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log(JSON.stringify(result, null, 4));

Deno.exit();
