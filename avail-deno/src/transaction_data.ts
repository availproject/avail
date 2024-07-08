// deno-lint-ignore-file no-namespace

import { ApiPromise } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.45/types/interfaces/types.ts";
import { err, ok, Result } from "npm:neverthrow@6.2.2";

export namespace DataAvailability {
	export class SubmitData {
		constructor(public data: string) {}

		static async New(api: ApiPromise, txHash: H256, blockHash: H256): Promise<Result<SubmitData, string>> {
			const block = await api.rpc.chain.getBlock(blockHash);
			const tx = block.block.extrinsics.find((tx) => tx.hash.toHex() == txHash.toHex());
			if (tx == undefined) {
				return err("Failed to find submit data transaction.");
			}

			// Data retrieved from the extrinsic data
			let dataHex = tx.method.args.map((a) => a.toString()).join(", ");
			if (dataHex.startsWith("0x")) {
				dataHex = dataHex.slice(2);
			}

			return ok(new SubmitData(dataHex));
		}
	}
}

export namespace Staking {
	export class Nominate {
		constructor(public targets: string[]) {}

		static async New(api: ApiPromise, txHash: H256, blockHash: H256): Promise<Result<Nominate, string>> {
			const block = await api.rpc.chain.getBlock(blockHash);
			const tx = block.block.extrinsics.find((tx) => tx.hash.toHex() == txHash.toHex());
			if (tx == undefined) {
				return err("Failed to find nominate transaction.");
			}

			const targets = [];
			const txTargets = tx.method.args[0] as any;
			for (let i = 0; i < txTargets.length; ++i) {
				targets.push(txTargets[i].toString());
			}

			return ok(new Nominate(targets));
		}
	}
}
