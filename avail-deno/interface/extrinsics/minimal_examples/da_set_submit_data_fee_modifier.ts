import { BN, DispatchFeeModifier, Keyring, SDK, WaitFor } from "../../../src/sdk.ts";

const providerEndpoint = "ws://127.0.0.1:9944";
const sdk = await SDK.New(providerEndpoint);

// Input
const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
const modifier = { weightMaximumFee: new BN("10").pow(new BN("18")), weightFeeDivider: 20 } as DispatchFeeModifier;

const result = await sdk.tx.dataAvailability.setSubmitDataFeeModifier(modifier, WaitFor.BlockInclusion, account);
if (result.isErr) {
	console.log(result.reason);
	Deno.exit(1);
}

console.log(
	"WeightMaximumFee=" + result.event.weightMaximumFee + ", WeightFeeMultiplier=" + result.event.weightFeeMultiplier +
		", WeightFeeDivider=" + result.event.weightFeeDivider,
);
console.log("TxHash=" + result.txHash + ", BlockHash=" + result.blockHash);

Deno.exit();
