import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result, ok } from "neverthrow"
import { WaitFor, GenericFailure, standardCallback, TransactionOptions } from "./common"
import { commissionNumberToPerbill, parseTransactionResult } from "../utils"

type ValidatorPerfs = { commission: string; blocked: boolean }
export type StakingRewardDestination = "Staked" | "Stash" | "None" | { account: string }

export type BondTxSuccess = {
  isErr: false
  event: Events.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
export type BondExtraTxSuccess = {
  isErr: false
  event: Events.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
export type ChillTxSuccess = {
  isErr: false
  event: Events.Chilled
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type ChillOtherTxSuccess = {
  isErr: false
  event: Events.Chilled
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type UnbondTxSuccess = {
  isErr: false
  event: Events.Unbonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type ValidatexSuccess = {
  isErr: false
  event: Events.ValidatorPrefsSet
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type NominateTxSuccess = {
  isErr: false
  txData: TransactionData.Nominate
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
    options?: TransactionOptions,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Bonded.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async bondExtra(
    maxAdditional: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Bonded.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async chill(
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Chilled.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Chilled event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async chillOther(
    stash: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Chilled.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Chilled event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async nominate(
    targets: string[],
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const maybeTxData = await TransactionData.Nominate.New(this.api, txHash, blockHash)
    if (maybeTxData.isErr()) {
      return { isErr: true, reason: maybeTxData.error }
    }

    return {
      isErr: false,
      txData: maybeTxData.value,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    }
  }

  async unbond(
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Unbonded.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Unbonded event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async validate(
    commission: number,
    blocked: boolean,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.ValidatorPrefsSet.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find ValidatorPrefsSet event." } as GenericFailure
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as ValidatexSuccess
  }
}

export namespace Events {
  export class Bonded {
    constructor(
      public stash: string,
      public amount: string,
    ) {}
    static New(events: EventRecord[]): Bonded | undefined {
      const ed: any = events.find((e) => e.event.method == "Bonded")?.event.data
      if (ed == undefined) {
        return undefined
      }

      const amountString = ed["amount"].toString()
      const amount = new BN(amountString).div(new BN(10).pow(new BN(18))).toString()

      return new Bonded(ed["stash"].toString(), amount)
    }
  }

  export class Chilled {
    constructor(public stash: string) {}
    static New(events: EventRecord[]): Chilled | undefined {
      const ed: any = events.find((e) => e.event.method == "Chilled")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new Chilled(ed["stash"].toString())
    }
  }

  export class Unbonded {
    constructor(
      public stash: string,
      public amount: string,
    ) {}
    static New(events: EventRecord[]): Unbonded | undefined {
      const ed: any = events.find((e) => e.event.method == "Unbonded")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new Unbonded(ed["stash"].toString(), ed["amount"].toString())
    }
  }

  export class ValidatorPrefsSet {
    constructor(
      public stash: string,
      public commission: string,
      public blocked: string,
    ) {}
    static New(events: EventRecord[]): ValidatorPrefsSet | undefined {
      const ed: any = events.find((e) => e.event.method == "ValidatorPrefsSet")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new ValidatorPrefsSet(
        ed["stash"].toString(),
        ed["prefs"]["commission"].toString(),
        ed["prefs"]["blocked"].toString(),
      )
    }
  }
}

export namespace TransactionData {
  export class Nominate {
    constructor(public targets: string[]) {}

    static async New(api: ApiPromise, txHash: H256, blockHash: H256): Promise<Result<Nominate, string>> {
      const block = await api.rpc.chain.getBlock(blockHash)
      const tx = block.block.extrinsics.find((tx) => tx.hash.toHex() == txHash.toHex())
      if (tx == undefined) {
        return err("Failed to find nominate transaction.")
      }

      const targets = []
      const txTargets = tx.method.args[0] as any
      for (let i = 0; i < txTargets.length; ++i) {
        targets.push(txTargets[i].toString())
      }

      return ok(new Nominate(targets))
    }
  }
}
