import { KeyringPair } from "https://deno.land/x/polkadot@0.2.45/keyring/types.ts";
import { Keyring, SDK, WaitFor } from "https://raw.githubusercontent.com/availproject/avail/main/avail-deno/src/sdk.ts";

export class SharedState {
	sdk: SDK;
	account: KeyringPair;
	waitFor: WaitFor = WaitFor.BlockInclusion;

	constructor(sdk: SDK) {
		this.sdk = sdk;
		this.account = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
	}
}

export enum ExecutionState {
	Idle,
	Busy,
}
