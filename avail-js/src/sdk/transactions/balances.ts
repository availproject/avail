import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result } from "neverthrow"

import { SignerOptions } from "@polkadot/api/types"
import { decodeError } from "../../helpers"
import { WaitFor, GenericFailure, standardCallback, getBlockHashAndTxHash } from "./common"

type TransferKeepAliveTxSuccess = {
  isErr: false
  event: Events.TransferEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type TransferAllowDeathTxSuccess = {
  isErr: false
  event: Events.TransferEvent
  event2?: Events.KilledAccount
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type TransferAllTxSuccess = {
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
    options?: Partial<SignerOptions>,
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

    const event = Events.TransferEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Transfer event." } as GenericFailure
    }
    const event2 = Events.KilledAccount.New(txResult.events)

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, event2, events, txHash, txIndex, blockHash, blockNumber } as TransferAllTxSuccess
  }

  async transferAllowDeath(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
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

    const event = Events.TransferEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Transfer event." } as GenericFailure
    }
    const event2 = Events.KilledAccount.New(txResult.events)

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return {
      isErr: false,
      event,
      event2,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    } as TransferAllowDeathTxSuccess
  }

  async transferKeepAlive(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
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

    const event = Events.TransferEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Transfer event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as TransferKeepAliveTxSuccess
  }
}

export namespace Events {
  export class TransferEvent {
    constructor(
      public from: string,
      public to: string,
      public amount: string,
    ) {}
    static New(events: EventRecord[]): TransferEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "Transfer")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new TransferEvent(ed["from"].toString(), ed["to"].toString(), ed["amount"].toString())
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
