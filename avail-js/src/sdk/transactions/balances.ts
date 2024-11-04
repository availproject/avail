import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result } from "neverthrow"
import { WaitFor, GenericFailure, standardCallback, TransactionOptions } from "./common"
import { parseTransactionResult } from "../utils"

export type TransferKeepAliveTxSuccess = {
  isErr: false
  event: Events.TransferEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type TransferAllowDeathTxSuccess = {
  isErr: false
  event: Events.TransferEvent
  event2?: Events.KilledAccount
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type TransferAllTxSuccess = {
  isErr: false
  event: Events.TransferEvent
  event2?: Events.KilledAccount
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export class Balances {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  async transferAll(
    dest: string,
    keepAlive: boolean,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<TransferAllTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.balances
        .transferAll(dest, keepAlive)
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
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.TransferEvent.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Transfer event" }
    }
    const event2 = Events.KilledAccount.New(events)

    return { isErr: false, event, event2, events, txHash, txIndex, blockHash, blockNumber }
  }

  async transferAllNoWait(
    dest: string,
    keepAlive: boolean,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    const optionWrapper = options || {}
    return this.api.tx.balances.transferAll(dest, keepAlive).signAndSend(account, optionWrapper)
  }

  async transferAllowDeath(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<TransferAllowDeathTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.balances
        .transferAllowDeath(dest, value)
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

    const event = Events.TransferEvent.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Transfer event" }
    }
    const event2 = Events.KilledAccount.New(events)

    return {
      isErr: false,
      event,
      event2,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    }
  }

  async transferAllowDeathNoWait(
    dest: string,
    value: BN,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    const optionWrapper = options || {}
    return this.api.tx.balances.transferAllowDeath(dest, value).signAndSend(account, optionWrapper)
  }

  async transferKeepAlive(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<TransferKeepAliveTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.balances
        .transferKeepAlive(dest, value)
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

    const event = Events.TransferEvent.New(details.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Transfer event" }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as TransferKeepAliveTxSuccess
  }

  async transferKeepAliveNoWait(
    dest: string,
    value: BN,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    const optionWrapper = options || {}
    return this.api.tx.balances.transferKeepAlive(dest, value).signAndSend(account, optionWrapper)
  }
}

export namespace Events {
  export class TransferEvent {
    constructor(
      public from: string,
      public to: string,
      public amount: BN,
    ) {}
    static New(events: EventRecord[]): TransferEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "Transfer")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new TransferEvent(ed["from"].toString(), ed["to"].toString(), ed["amount"])
    }
  }

  export class KilledAccount {
    constructor(public account: string) {}
    static New(events: EventRecord[]): KilledAccount | undefined {
      const ed: any = events.find((e) => e.event.method == "KilledAccount" && e.event.section == "system")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new KilledAccount(ed["account"].toString())
    }
  }
}
