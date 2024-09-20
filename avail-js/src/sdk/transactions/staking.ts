import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result } from "neverthrow"

import * as Events from "./../events"
import * as TransactionData from "./../transaction_data"
import { SignerOptions } from "@polkadot/api/types"
import { decodeError } from "../../helpers"
import { WaitFor, GenericFailure, standardCallback, getBlockHashAndTxHash } from "./common"
import { commissionNumberToPerbill } from "../utils"

type ValidatorPerfs = { commission: string; blocked: boolean }
export type StakingRewardDestination = "Staked" | "Stash" | "None" | { account: string }

type BondTxSuccess = {
  isErr: false
  event: Events.Staking.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type BondExtraTxSuccess = {
  isErr: false
  event: Events.Staking.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type ChillTxSuccess = {
  isErr: false
  event: Events.Staking.Chilled
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type ChillOtherTxSuccess = {
  isErr: false
  event: Events.Staking.Chilled
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type UnbondTxSuccess = {
  isErr: false
  event: Events.Staking.Unbonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type ValidatexSuccess = {
  isErr: false
  event: Events.Staking.ValidatorPrefsSet
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type NominateTxSuccess = {
  isErr: false
  txData: TransactionData.Staking.Nominate
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export class Staking {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  async bond(
    value: BN,
    payee: StakingRewardDestination,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<BondTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.staking
        .bond(value, payee)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error } as GenericFailure
    }
    const txResult = maybeTxResult.value

    if (txResult.isError) {
      return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure
    }

    const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event))
    if (failed != undefined) {
      return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure
    }

    const event = Events.Staking.Bonded.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as BondTxSuccess
  }

  async bondExtra(
    maxAdditional: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<BondExtraTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.staking
        .bondExtra(maxAdditional)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error } as GenericFailure
    }
    const txResult = maybeTxResult.value

    if (txResult.isError) {
      return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure
    }

    const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event))
    if (failed != undefined) {
      return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure
    }

    const event = Events.Staking.Bonded.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as BondExtraTxSuccess
  }

  async chill(
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<ChillTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.staking
        .chill()
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error } as GenericFailure
    }
    const txResult = maybeTxResult.value

    if (txResult.isError) {
      return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure
    }

    const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event))
    if (failed != undefined) {
      return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure
    }

    const event = Events.Staking.Chilled.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Chilled event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as ChillTxSuccess
  }

  async chillOther(
    stash: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<ChillOtherTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.staking
        .chillOther(stash)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error } as GenericFailure
    }
    const txResult = maybeTxResult.value

    if (txResult.isError) {
      return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure
    }

    const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event))
    if (failed != undefined) {
      return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure
    }

    const event = Events.Staking.Chilled.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Chilled event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as ChillOtherTxSuccess
  }

  async nominate(
    targets: string[],
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<NominateTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.staking
        .nominate(targets)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error } as GenericFailure
    }
    const txResult = maybeTxResult.value

    if (txResult.isError) {
      return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure
    }

    const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event))
    if (failed != undefined) {
      return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    const maybeTxData = await TransactionData.Staking.Nominate.New(this.api, txHash, blockHash)
    if (maybeTxData.isErr()) {
      return { isErr: true, reason: maybeTxData.error } as GenericFailure
    }

    return {
      isErr: false,
      txData: maybeTxData.value,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    } as NominateTxSuccess
  }

  async unbond(
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<UnbondTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.staking
        .unbond(value)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error } as GenericFailure
    }
    const txResult = maybeTxResult.value

    if (txResult.isError) {
      return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure
    }

    const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event))
    if (failed != undefined) {
      return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure
    }

    const event = Events.Staking.Unbonded.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Unbonded event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as UnbondTxSuccess
  }

  async validate(
    commission: number,
    blocked: boolean,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<ValidatexSuccess | GenericFailure> {
    const maybeCommission = commissionNumberToPerbill(commission)
    if (maybeCommission.isErr()) {
      return { isErr: true, reason: maybeCommission.error } as GenericFailure
    }

    const validatorPerfs = { commission: maybeCommission.value, blocked } as ValidatorPerfs
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.staking
        .validate(validatorPerfs)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error } as GenericFailure
    }
    const txResult = maybeTxResult.value

    if (txResult.isError) {
      return { isErr: true, reason: "The transaction was dropped or something." } as GenericFailure
    }

    const failed = txResult.events.find((e) => this.api.events.system.ExtrinsicFailed.is(e.event))
    if (failed != undefined) {
      return { isErr: true, reason: decodeError(this.api, failed.event.data[0]) } as GenericFailure
    }

    const event = Events.Staking.ValidatorPrefsSet.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find ValidatorPrefsSet event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as ValidatexSuccess
  }
}
