import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result } from "neverthrow"
import * as TransactionData from "./../transaction_data"
import { decodeError, fromHexToAscii } from "../../helpers"
import { WaitFor, GenericFailure, standardCallback, getBlockHashAndTxHash, TransactionOptions } from "./common"

export type DispatchFeeModifier = {
  weightMaximumFee: BN | null
  weightFeeDivider: number | null
  weightFeeMultiplier: number | null
}

type SubmitDataTxSuccess = {
  isErr: false
  txData: TransactionData.DataAvailability.SubmitData
  event: Events.DataSubmittedEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type CreateApplicationKeyTxSuccess = {
  isErr: false
  event: Events.ApplicationKeyCreatedEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type SetApplicationKeyTxSuccess = {
  isErr: false
  event: Events.ApplicationKeySetEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type SubmitBlockLengthProposalTxSuccess = {
  isErr: false
  event: Events.BlockLengthProposalSubmittedEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}
type SetSubmitDataFeeModifierTxSuccess = {
  isErr: false
  event: Events.SubmitDataFeeModifierSetEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export class DataAvailability {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  async submitData(
    data: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<SubmitDataTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.dataAvailability
        .submitData(data)
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

    const event = Events.DataSubmittedEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find DataSubmitted event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    const maybeTxData = await TransactionData.DataAvailability.SubmitData.New(this.api, txHash, blockHash)
    if (maybeTxData.isErr()) {
      return { isErr: true, reason: maybeTxData.error } as GenericFailure
    }

    return {
      isErr: false,
      txData: maybeTxData.value,
      event,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    } as SubmitDataTxSuccess
  }

  async createApplicationKey(
    key: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<CreateApplicationKeyTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.dataAvailability
        .createApplicationKey(key)
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

    const event = Events.ApplicationKeyCreatedEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find ApplicationKeyCreated event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as CreateApplicationKeyTxSuccess
  }

  async setApplicationKey(
    oldKey: string,
    newKey: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<SetApplicationKeyTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      const call = this.api.tx.dataAvailability.setApplicationKey(oldKey, newKey)
      this.api.tx.sudo
        .sudo(call)
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

    const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return { isErr: true, reason: "Failed to find Sudid event." } as GenericFailure
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure
    }

    const event = Events.ApplicationKeySetEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find ApplicationKeySet event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as SetApplicationKeyTxSuccess
  }

  async submitBlockLengthProposal(
    rows: number,
    cols: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<SubmitBlockLengthProposalTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      const call = this.api.tx.dataAvailability.submitBlockLengthProposal(rows, cols)
      this.api.tx.sudo
        .sudo(call)
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

    const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return { isErr: true, reason: "Failed to find Sudid event." } as GenericFailure
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure
    }

    const event = Events.BlockLengthProposalSubmittedEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find BlockLengthProposalSubmitted event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return {
      isErr: false,
      event,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    } as SubmitBlockLengthProposalTxSuccess
  }

  async setSubmitDataFeeModifier(
    modifier: DispatchFeeModifier,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<SetSubmitDataFeeModifierTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      const call = this.api.tx.dataAvailability.setSubmitDataFeeModifier(modifier)
      this.api.tx.sudo
        .sudo(call)
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

    const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return { isErr: true, reason: "Failed to find Sudid event." } as GenericFailure
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure
    }

    const event = Events.SubmitDataFeeModifierSetEvent.New(txResult.events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find SubmitDataFeeModifierSet event." } as GenericFailure
    }

    const events = txResult.events
    const [txHash, txIndex, blockHash, blockNumber] = await getBlockHashAndTxHash(txResult, waitFor, this.api)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber } as SetSubmitDataFeeModifierTxSuccess
  }
}

export namespace Events {
  export class DataSubmittedEvent {
    constructor(
      public who: string,
      public dataHash: string,
    ) {}
    static New(events: EventRecord[]): DataSubmittedEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "DataSubmitted")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new DataSubmittedEvent(ed["who"].toString(), ed["dataHash"].toString())
    }
  }

  export class ApplicationKeyCreatedEvent {
    constructor(
      public key: string,
      public owner: string,
      public id: string,
    ) {}
    static New(events: EventRecord[]): ApplicationKeyCreatedEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "ApplicationKeyCreated")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new ApplicationKeyCreatedEvent(ed["key"].toString(), ed["owner"].toString(), ed["id"].toString())
    }
  }

  export class ApplicationKeySetEvent {
    constructor(
      public oldKey: string,
      public newKey: string,
    ) {}
    static New(events: EventRecord[]): ApplicationKeySetEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "ApplicationKeySet")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new ApplicationKeySetEvent(
        fromHexToAscii(ed["oldKey"].toString()),
        fromHexToAscii(ed["newKey"].toString()),
      )
    }
  }

  export class BlockLengthProposalSubmittedEvent {
    constructor(
      public rows: string,
      public cols: string,
    ) {}
    static New(events: EventRecord[]): BlockLengthProposalSubmittedEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "BlockLengthProposalSubmitted")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new BlockLengthProposalSubmittedEvent(ed["rows"].toString(), ed["cols"].toString())
    }
  }

  export class SubmitDataFeeModifierSetEvent {
    constructor(
      public weightMaximumFee: BN | null,
      public weightFeeDivider: string | null,
      public weightFeeMultiplier: string | null,
    ) {}
    static New(events: EventRecord[]): SubmitDataFeeModifierSetEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "SubmitDataFeeModifierSet")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new SubmitDataFeeModifierSetEvent(
        ed["weightMaximumFee"]?.toString(),
        ed["weightFeeDivider"]?.toString(),
        ed["weightFeeMultiplier"]?.toString(),
      )
    }
  }
}
