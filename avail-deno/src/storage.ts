/* // deno-lint-ignore-file no-namespace

import { ApiPromise } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";


export class Storage {
    private api: ApiPromise;
	staking: Staking;

	constructor(api: ApiPromise) {
		this.api = api;
		this.staking = new Staking(api);
	}
}

export class Staking {
	private api: ApiPromise;

	constructor(api: ApiPromise) {
		this.api = api;
	}

	async bonded(account?: string): Promise<[string[], string][]> {
		const entires = await this.api.query.staking.bonded.entries();
		console.log("Hello");
		for(const entry of entires) {
			console.log(entry[0].toJSON());
			const address2 = entry[1].toString();
		}

		return [];
	}
} */