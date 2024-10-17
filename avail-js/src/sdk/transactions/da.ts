import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { EventRecord, H256 } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result, ok } from "neverthrow"
import * as TransactionData from "./../transaction_data"
import { decodeError, fromHexToAscii } from "../../helpers"
import { WaitFor, standardCallback, TransactionFailed, TransactionOptions } from "./common"
import { parseTransactionResult, TxResultDetails } from "../utils"
import { Bytes } from "@polkadot/types-codec"

export type DispatchFeeModifier = {
  weightMaximumFee: BN | null
  weightFeeDivider: number | null
  weightFeeMultiplier: number | null
}

export class SubmitDataTx {
  constructor(
    public txData: TransactionData.DataAvailability.SubmitData,
    public event: Events.DataSubmittedEvent,
    public details: TxResultDetails,
  ) {}
}

export class CreateApplicationKeyTx {
  constructor(
    public event: Events.ApplicationKeyCreatedEvent,
    public details: TxResultDetails,
  ) {}
}

export class SetApplicationKeyTx {
  constructor(
    public event: Events.ApplicationKeySetEvent,
    public details: TxResultDetails,
  ) {}
}

export class SubmitBlockLengthProposalTx {
  constructor(
    public event: Events.BlockLengthProposalSubmittedEvent,
    public details: TxResultDetails,
  ) {}
}

export class SetSubmitDataFeeModifierTx {
  constructor(
    public event: Events.SubmitDataFeeModifierSetEvent,
    public details: TxResultDetails,
  ) {}
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
  ): Promise<Result<SubmitDataTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const event = Events.DataSubmittedEvent.New(txResult.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find DataSubmitted Event", details))
    }

    const maybeTxData = await TransactionData.DataAvailability.SubmitData.New(
      this.api,
      details.txHash,
      details.blockHash,
    )
    if (maybeTxData.isErr()) {
      return err(new TransactionFailed(maybeTxData.error, details))
    }

    return ok(new SubmitDataTx(maybeTxData.value, event, details))
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
  ): Promise<Result<CreateApplicationKeyTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const event = Events.ApplicationKeyCreatedEvent.New(txResult.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find ApplicationKeyCreated Event", details))
    }

    return ok(new CreateApplicationKeyTx(event, details))
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
  ): Promise<Result<SetApplicationKeyTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return err(new TransactionFailed("Failed to find Sudid Event", details))
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return err(new TransactionFailed(decodeError(this.api, sudoResult.asErr), details))
    }

    const event = Events.ApplicationKeySetEvent.New(txResult.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find ApplicationKeySet Event", details))
    }

    return ok(new SetApplicationKeyTx(event, details))
  }

  async submitBlockLengthProposal(
    rows: number,
    cols: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SubmitBlockLengthProposalTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return err(new TransactionFailed("Failed to find Sudid Event", details))
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return err(new TransactionFailed(decodeError(this.api, sudoResult.asErr), details))
    }

    const event = Events.BlockLengthProposalSubmittedEvent.New(txResult.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find BlockLengthProposalSubmitted Event", details))
    }

    return ok(new SubmitBlockLengthProposalTx(event, details))
  }

  async setSubmitDataFeeModifier(
    modifier: DispatchFeeModifier,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SetSubmitDataFeeModifierTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const sudoEvent = txResult.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) {
      return err(new TransactionFailed("Failed to find Sudid Event", details))
    }

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) {
      return err(new TransactionFailed(decodeError(this.api, sudoResult.asErr), details))
    }

    const event = Events.SubmitDataFeeModifierSetEvent.New(txResult.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find SubmitDataFeeModifierSet Event", details))
    }

    return ok(new SetSubmitDataFeeModifierTx(event, details))
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
