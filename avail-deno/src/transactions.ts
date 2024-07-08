import { ApiPromise } from "https://deno.land/x/polkadot@0.2.45/api/mod.ts";
import { KeyringPair } from "https://deno.land/x/polkadot@0.2.45/keyring/types.ts";
import { ISubmittableResult } from "https://deno.land/x/polkadot@0.2.45/types/types/extrinsic.ts";
import { SignerOptions } from "https://deno.land/x/polkadot@0.2.45/api/submittable/types.ts";
import { H256 } from "https://deno.land/x/polkadot@0.2.45/types/interfaces/types.ts";
import { BN } from "https://deno.land/x/polkadot@0.2.45/util/mod.ts";
import { err, ok, Result } from "npm:neverthrow@6.2.2";

import * as TransactionData from "./transaction_data.ts";
import * as Events from "./events.ts";
import { commissionNumberToPerbill, decodeError } from "./common.ts";

export type DispatchFeeModifier = { weightMaximumFee: BN | null; weightFeeDivider: number | null; weightFeeMultiplier: number | null };
export enum WaitFor {
	BlockInclusion,
	BlockFinalization,
}
export type StakingRewardDestination = "Staked" | "Stash" | "None" | { account: string };
type ValidatorPerfs = { commission: string; blocked: boolean };

type GenericFailure = { isErr: true; reason: string };

type SubmitDataTxSuccess = {
	isErr: false;
	txData: TransactionData.DataAvailability.SubmitData;
	event: Events.DataAvailability.DataSubmittedEvent;
	txHash: H256;
	blockHash: H256;
};
type CreateApplicationKeyTxSuccess = {
	isErr: false;
	event: Events.DataAvailability.ApplicationKeyCreatedEvent;
	txHash: H256;
	blockHash: H256;
};
type SetApplicationKeyTxSuccess = {
	isErr: false;
	event: Events.DataAvailability.ApplicationKeySetEvent;
	txHash: H256;
	blockHash: H256;
};
type SubmitBlockLengthProposalTxSuccess = {
	isErr: false;
	event: Events.DataAvailability.BlockLengthProposalSubmittedEvent;
	txHash: H256;
	blockHash: H256;
};
type SetSubmitDataFeeModifierTxSuccess = {
	isErr: false;
	event: Events.DataAvailability.SubmitDataFeeModifierSetEvent;
	txHash: H256;
	blockHash: H256;
};
type TransferKeepAliveTxSuccess = { isErr: false; event: Events.Balances.TransferEvent; txHash: H256; blockHash: H256 };
type TransferAllowDeathTxSuccess = {
	isErr: false;
	event: Events.Balances.TransferEvent;
	event2?: Events.System.KilledAccount;
	txHash: H256;
	blockHash: H256;
};
type TransferAllTxSuccess = {
	isErr: false;
	event: Events.Balances.TransferEvent;
	event2?: Events.System.KilledAccount;
	txHash: H256;
	blockHash: H256;
};
type BondTxSuccess = { isErr: false; event: Events.Staking.Bonded; txHash: H256; blockHash: H256 };
type BondExtraTxSuccess = { isErr: false; event: Events.Staking.Bonded; txHash: H256; blockHash: H256 };
type ChillTxSuccess = { isErr: false; event: Events.Staking.Chilled; txHash: H256; blockHash: H256 };
type ChillOtherTxSuccess = { isErr: false; event: Events.Staking.Chilled; txHash: H256; blockHash: H256 };
type UnbondTxSuccess = { isErr: false; event: Events.Staking.Unbonded; txHash: H256; blockHash: H256 };
type ValidatexSuccess = { isErr: false; event: Events.Staking.ValidatorPrefsSet; txHash: H256; blockHash: H256 };
type NominateTxSuccess = { isErr: false; txData: TransactionData.Staking.Nominate; txHash: H256; blockHash: H256 };

export class Transactions {
	private api: ApiPromise;
	dataAvailability: DataAvailability;
	balances: Balances;
	staking: Staking;

	constructor(api: ApiPromise) {
		this.api = api;
		this.dataAvailability = new DataAvailability(api);
		this.balances = new Balances(api);
		this.staking = new Staking(api);
	}
}

function standardCallback(
	result_in: ISubmittableResult,
	result_out: (value: Result<ISubmittableResult, string>) => void,
	waitFor: WaitFor,
) {
	if (result_in.isError) {
		result_out(ok(result_in));
	}

	if (result_in.isInBlock && waitFor == WaitFor.BlockInclusion) {
		result_out(ok(result_in));
	}

	if (result_in.isFinalized) {
		result_out(ok(result_in));
	}
}

function getBlockHashAndTxHash(result: ISubmittableResult, waitFor: WaitFor): [H256, H256] {
	if (waitFor == WaitFor.BlockInclusion) {
		return [result.txHash as H256, result.status.asInBlock as H256];
	} else {
		return [result.txHash as H256, result.status.asFinalized as H256];
	}
}

export class Staking {
	private api: ApiPromise;

	constructor(api: ApiPromise) {
		this.api = api;
	}

	async bond(
		value: BN,
		payee: StakingRewardDestination,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<BondTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.staking.bond(value, payee).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Staking.Bonded.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Bonded event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as BondTxSuccess;
	}

	async bondExtra(
		maxAdditional: BN,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<BondExtraTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.staking.bondExtra(maxAdditional).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Staking.Bonded.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Bonded event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as BondExtraTxSuccess;
	}

	async chill(waitFor: WaitFor, account: KeyringPair, options?: Partial<SignerOptions>): Promise<ChillTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.staking.chill().signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Staking.Chilled.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Chilled event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as ChillTxSuccess;
	}

	async chillOther(
		stash: string,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<ChillOtherTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.staking.chillOther(stash).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Staking.Chilled.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Chilled event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as ChillOtherTxSuccess;
	}

	async nominate(
		targets: string[],
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<NominateTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.staking.nominate(targets).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);
		const maybeTxData = await TransactionData.Staking.Nominate.New(this.api, txHash, blockHash);
		if (maybeTxData.isErr()) {
			return { isErr: true, reason: maybeTxData.error } as GenericFailure;
		}

		return { isErr: false, txData: maybeTxData.value, txHash, blockHash } as NominateTxSuccess;
	}

	async unbond(
		value: BN,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<UnbondTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.staking.unvond(value).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Staking.Unbonded.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Unbonded event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as UnbondTxSuccess;
	}

	async validate(
		commission: number,
		blocked: boolean,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<ValidatexSuccess | GenericFailure> {
		const maybeCommission = commissionNumberToPerbill(commission);
		if (maybeCommission.isErr()) {
			return { isErr: true, reason: maybeCommission.error } as GenericFailure;
		}

		const validatorPerfs = { commission: maybeCommission.value, blocked } as ValidatorPerfs;
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.staking.validate(validatorPerfs).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Staking.ValidatorPrefsSet.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find ValidatorPrefsSet event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as ValidatexSuccess;
	}
}

export class Balances {
	private api: ApiPromise;

	constructor(api: ApiPromise) {
		this.api = api;
	}

	async transferAll(
		dest: string,
		keepAlive: boolean,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<TransferAllTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.balances.transferAll(dest, keepAlive).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Balances.TransferEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Transfer event." } as GenericFailure;
		}
		const event2 = Events.System.KilledAccount.New(txResult.events);

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, event2, txHash, blockHash } as TransferAllTxSuccess;
	}

	async transferAllowDeath(
		dest: string,
		value: BN,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<TransferAllowDeathTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.balances.transferAllowDeath(dest, value).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Balances.TransferEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Transfer event." } as GenericFailure;
		}
		const event2 = Events.System.KilledAccount.New(txResult.events);

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, event2, txHash, blockHash } as TransferAllowDeathTxSuccess;
	}

	async transferKeepAlive(
		dest: string,
		value: BN,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<TransferKeepAliveTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.balances.transferKeepAlive(dest, value).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.Balances.TransferEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find Transfer event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as TransferKeepAliveTxSuccess;
	}
}

export class DataAvailability {
	private api: ApiPromise;

	constructor(api: ApiPromise) {
		this.api = api;
	}

	async submitData(
		data: string,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<SubmitDataTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.dataAvailability.submitData(data).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.DataAvailability.DataSubmittedEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find DataSubmitted event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		const maybeTxData = await TransactionData.DataAvailability.SubmitData.New(this.api, txHash, blockHash);
		if (maybeTxData.isErr()) {
			return { isErr: true, reason: maybeTxData.error } as GenericFailure;
		}

		return { isErr: false, txData: maybeTxData.value, event, txHash, blockHash } as SubmitDataTxSuccess;
	}

	async createApplicationKey(
		key: string,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<CreateApplicationKeyTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			this.api.tx.dataAvailability.createApplicationKey(key).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const event = Events.DataAvailability.ApplicationKeyCreatedEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find ApplicationKeyCreated event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as CreateApplicationKeyTxSuccess;
	}

	async setApplicationKey(
		oldKey: string,
		newKey: string,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<SetApplicationKeyTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			const call = this.api.tx.dataAvailability.setApplicationKey(oldKey, newKey);
			this.api.tx.sudo.sudo(call).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid");
		if (sudoEvent == undefined) {
			return { isErr: true, reason: "Failed to find Sudid event." } as GenericFailure;
		}

		const sudoResult: any = (sudoEvent.event.data as any).sudoResult;
		if (sudoResult.isErr) {
			return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure;
		}

		const event = Events.DataAvailability.ApplicationKeySetEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find ApplicationKeySet event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as SetApplicationKeyTxSuccess;
	}

	async submitBlockLengthProposal(
		rows: number,
		cols: number,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<SubmitBlockLengthProposalTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			const call = this.api.tx.dataAvailability.submitBlockLengthProposal(rows, cols);
			this.api.tx.sudo.sudo(call).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid");
		if (sudoEvent == undefined) {
			return { isErr: true, reason: "Failed to find Sudid event." } as GenericFailure;
		}

		const sudoResult: any = (sudoEvent.event.data as any).sudoResult;
		if (sudoResult.isErr) {
			return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure;
		}

		const event = Events.DataAvailability.BlockLengthProposalSubmittedEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find BlockLengthProposalSubmitted event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as SubmitBlockLengthProposalTxSuccess;
	}

	async setSubmitDataFeeModifier(
		modifier: DispatchFeeModifier,
		waitFor: WaitFor,
		account: KeyringPair,
		options?: Partial<SignerOptions>,
	): Promise<SetSubmitDataFeeModifierTxSuccess | GenericFailure> {
		const optionWrapper = options || {};
		const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
			const call = this.api.tx.dataAvailability.setSubmitDataFeeModifier(modifier);
			this.api.tx.sudo.sudo(call).signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
				standardCallback(result, res, waitFor);
			}).catch((reason) => {
				res(err(reason));
			});
		});

		if (maybeTxResult.isErr()) {
			return { isErr: true, reason: maybeTxResult.error } as GenericFailure;
		}
		const txResult = maybeTxResult.value;

		if (txResult.isError) {
			return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure;
		}

		const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event));
		if (failed != undefined) {
			return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure;
		}

		const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid");
		if (sudoEvent == undefined) {
			return { isErr: true, reason: "Failed to find Sudid event." } as GenericFailure;
		}

		const sudoResult: any = (sudoEvent.event.data as any).sudoResult;
		if (sudoResult.isErr) {
			return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure;
		}

		const event = Events.DataAvailability.SubmitDataFeeModifierSetEvent.New(txResult.events);
		if (event == undefined) {
			return { isErr: true, reason: "Failed to find SubmitDataFeeModifierSet event." } as GenericFailure;
		}

		const [txHash, blockHash] = getBlockHashAndTxHash(txResult, waitFor);

		return { isErr: false, event, txHash, blockHash } as SetSubmitDataFeeModifierTxSuccess;
	}
}
