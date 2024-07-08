// deno-lint-ignore-file no-namespace

import { EventRecord } from "https://deno.land/x/polkadot@0.2.45/types/interfaces/types.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.45/util/mod.ts";
import { fromHexToAscii } from "./common.ts";

export namespace DataAvailability {
	export class DataSubmittedEvent {
		constructor(public who: string, public dataHash: string) {}
		static New(events: EventRecord[]): DataSubmittedEvent | undefined {
			const ed: any = events.find((e) => e.event.method == "DataSubmitted")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new DataSubmittedEvent(ed["who"].toString(), ed["dataHash"].toString());
		}
	}

	export class ApplicationKeyCreatedEvent {
		constructor(public key: string, public owner: string, public id: string) {}
		static New(events: EventRecord[]): ApplicationKeyCreatedEvent | undefined {
			const ed: any = events.find((e) => e.event.method == "ApplicationKeyCreated")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new ApplicationKeyCreatedEvent(ed["key"].toString(), ed["owner"].toString(), ed["id"].toString());
		}
	}

	export class ApplicationKeySetEvent {
		constructor(public oldKey: string, public newKey: string) {}
		static New(events: EventRecord[]): ApplicationKeySetEvent | undefined {
			const ed: any = events.find((e) => e.event.method == "ApplicationKeySet")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new ApplicationKeySetEvent(fromHexToAscii(ed["oldKey"].toString()), fromHexToAscii(ed["newKey"].toString()));
		}
	}

	export class BlockLengthProposalSubmittedEvent {
		constructor(public rows: string, public cols: string) {}
		static New(events: EventRecord[]): BlockLengthProposalSubmittedEvent | undefined {
			const ed: any = events.find((e) => e.event.method == "BlockLengthProposalSubmitted")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new BlockLengthProposalSubmittedEvent(ed["rows"].toString(), ed["cols"].toString());
		}
	}

	export class SubmitDataFeeModifierSetEvent {
		constructor(
			public weightMaximumFee: BN | null,
			public weightFeeDivider: string | null,
			public weightFeeMultiplier: string | null,
		) {}
		static New(events: EventRecord[]): SubmitDataFeeModifierSetEvent | undefined {
			const ed: any = events.find((e) => e.event.method == "SubmitDataFeeModifierSet")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new SubmitDataFeeModifierSetEvent(
				ed["weightMaximumFee"]?.toString(),
				ed["weightFeeDivider"]?.toString(),
				ed["weightFeeMultiplier"]?.toString(),
			);
		}
	}
}

export namespace Balances {
	export class TransferEvent {
		constructor(public from: string, public to: string, public amount: string) {}
		static New(events: EventRecord[]): TransferEvent | undefined {
			const ed: any = events.find((e) => e.event.method == "Transfer")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new TransferEvent(ed["from"].toString(), ed["to"].toString(), ed["amount"].toString());
		}
	}
}

export namespace System {
	export class KilledAccount {
		constructor(public account: string) {}
		static New(events: EventRecord[]): KilledAccount | undefined {
			const ed: any = events.find((e) => e.event.method == "KilledAccount")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new KilledAccount(ed["account"].toString());
		}
	}
}

export namespace Staking {
	export class Bonded {
		constructor(public stash: string, public amount: string) {}
		static New(events: EventRecord[]): Bonded | undefined {
			const ed: any = events.find((e) => e.event.method == "Bonded")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			const amountString = ed["amount"].toString();
			const amount = new BN(amountString).div(new BN(10).pow(new BN(18))).toString();

			return new Bonded(ed["stash"].toString(), amount);
		}
	}

	export class Chilled {
		constructor(public stash: string) {}
		static New(events: EventRecord[]): Chilled | undefined {
			const ed: any = events.find((e) => e.event.method == "Chilled")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new Chilled(ed["stash"].toString());
		}
	}

	export class Unbonded {
		constructor(public stash: string, public amount: string) {}
		static New(events: EventRecord[]): Unbonded | undefined {
			const ed: any = events.find((e) => e.event.method == "Unbonded")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new Unbonded(ed["stash"].toString(), ed["amount"].toString());
		}
	}

	export class ValidatorPrefsSet {
		constructor(public stash: string, public commission: string, public blocked: string) {}
		static New(events: EventRecord[]): ValidatorPrefsSet | undefined {
			const ed: any = events.find((e) => e.event.method == "ValidatorPrefsSet")?.event.data;
			if (ed == undefined) {
				return undefined;
			}

			return new ValidatorPrefsSet(
				ed["stash"].toString(),
				ed["prefs"]["commission"].toString(),
				ed["prefs"]["blocked"].toString(),
			);
		}
	}
}
