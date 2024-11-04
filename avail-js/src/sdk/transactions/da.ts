import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, ok, Result } from "neverthrow"
import { decodeError, fromHexToAscii } from "../../helpers"
import { WaitFor, GenericFailure, standardCallback, TransactionOptions } from "./common"
import { parseTransactionResult } from "../utils"
import { Bytes } from "@polkadot/types-codec"

export type DispatchFeeModifier = {
  weightMaximumFee: BN | null
  weightFeeDivider: number | null
  weightFeeMultiplier: number | null
}

export type SubmitDataTxSuccess = {
  isErr: false
  txData: TransactionData.SubmitData
  event: Events.DataSubmittedEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type CreateApplicationKeyTxSuccess = {
  isErr: false
  event: Events.ApplicationKeyCreatedEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type SetApplicationKeyTxSuccess = {
  isErr: false
  event: Events.ApplicationKeySetEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type SubmitBlockLengthProposalTxSuccess = {
  isErr: false
  event: Events.BlockLengthProposalSubmittedEvent
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type SetSubmitDataFeeModifierTxSuccess = {
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
    data: string | Bytes,
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.DataSubmittedEvent.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find DataSubmitted event." } as GenericFailure
    }

    const maybeTxData = await TransactionData.SubmitData.New(this.api, txHash, blockHash)
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

  async submitDataNoWait(data: string | Bytes, account: KeyringPair, options?: TransactionOptions): Promise<H256> {
    const optionWrapper = options || {}
    return this.api.tx.dataAvailability.submitData(data).signAndSend(account, optionWrapper)
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.ApplicationKeyCreatedEvent.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find ApplicationKeyCreated event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async createApplicationKeyNoWait(key: string, account: KeyringPair, options?: TransactionOptions): Promise<H256> {
    const optionWrapper = options || {}
    return this.api.tx.dataAvailability.createApplicationKey(key).signAndSend(account, optionWrapper)
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const sudoEvent = events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return { isErr: true, reason: "Failed to find Sudid event." }
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure
    }

    const event = Events.ApplicationKeySetEvent.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find ApplicationKeySet event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const sudoEvent = events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return { isErr: true, reason: "Failed to find Sudid event." } as GenericFailure
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure
    }

    const event = Events.BlockLengthProposalSubmittedEvent.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find BlockLengthProposalSubmitted event." }
    }

    return {
      isErr: false,
      event,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    }
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
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const sudoEvent = events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return { isErr: true, reason: "Failed to find Sudid event." }
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return { isErr: true, isFailure: true, reason: decodeError(this.api, sudoResult.asErr) } as GenericFailure
    }

    const event = Events.SubmitDataFeeModifierSetEvent.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find SubmitDataFeeModifierSet event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
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

export namespace TransactionData {
  export class SubmitData {
    constructor(public data: string) {}

    static async New(api: ApiPromise, txHash: H256, blockHash: H256): Promise<Result<SubmitData, string>> {
      const block = await api.rpc.chain.getBlock(blockHash)
      const tx = block.block.extrinsics.find((tx) => tx.hash.toHex() == txHash.toHex())
      if (tx == undefined) {
        return err("Failed to find submit data transaction.")
      }

      // Data retrieved from the extrinsic data
      let dataHex = tx.method.args.map((a) => a.toString()).join(", ")
      if (dataHex.startsWith("0x")) {
        dataHex = dataHex.slice(2)
      }

      return ok(new SubmitData(dataHex))
    }
  }
}
