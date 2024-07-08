import { ApiPromise, WsProvider } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { API_EXTENSIONS, API_RPC, API_TYPES } from "./api_options.ts";

import { Transactions } from "./transactions.ts";
/* import { Storage } from "./storage.ts"; */

export { BN } from "https://deno.land/x/polkadot@0.2.45/util/mod.ts";
export { Keyring } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
export type { DispatchFeeModifier, StakingRewardDestination } from "./transactions.ts";
export { WaitFor } from "./transactions.ts";

export class SDK {
	api: ApiPromise;
	tx: Transactions;
/* 	storage: Storage; */

	static async New(endpoint: string): Promise<SDK> {
		const api = await ApiPromise.create({
			provider: new WsProvider(endpoint),
			rpc: API_RPC,
			types: API_TYPES,
			signedExtensions: API_EXTENSIONS,
		});
		return new SDK(api);
	}

	private constructor(api: ApiPromise) {
		this.api = api;
		this.tx = new Transactions(api);
/* 		this.storage = new Storage(api); */
	}
}
