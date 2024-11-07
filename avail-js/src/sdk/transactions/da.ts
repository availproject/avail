import { ApiPromise } from "@polkadot/api"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, ok, Result } from "neverthrow"
import { decodeError, fromHexToAscii } from "../../helpers"
import {
  WaitFor,
  TransactionOptions,
  singAndSendAndParseTransaction,
  TxResultDetails,
  TransactionFailed,
} from "./common"
import { Bytes } from "@polkadot/types-codec"

export type DispatchFeeModifier = {
  weightMaximumFee: BN | null
  weightFeeDivider: number | null
  weightFeeMultiplier: number | null
}

export class SubmitDataTx {
  constructor(
    public txData: TransactionData.SubmitData,
    public event: Events.DataSubmittedEvent,
    public appId: number,
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
    const appId = optionWrapper.app_id || 0

    const tx = this.api.tx.dataAvailability.submitData(data)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.DataSubmittedEvent.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find DataSubmitted event.", details))

    const maybeTxData = await TransactionData.SubmitData.New(this.api, details.txHash, details.blockHash)
    if (maybeTxData.isErr()) return err(new TransactionFailed(maybeTxData.error, details))

    return ok(new SubmitDataTx(maybeTxData.value, event, appId, details))
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
    const tx = this.api.tx.dataAvailability.createApplicationKey(key)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.ApplicationKeyCreatedEvent.New(details.events)
    if (event == undefined)
      return err(new TransactionFailed("Failed to find ApplicationKeyCreatedEvent event.", details))

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
    const call = this.api.tx.dataAvailability.setApplicationKey(oldKey, newKey)
    const tx = this.api.tx.sudo.sudo(call)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const sudoEvent = details.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) return err(new TransactionFailed("Failed to find Sudid event.", details))

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) return err(new TransactionFailed(decodeError(this.api, sudoResult.asErr), details))

    const event = Events.ApplicationKeySetEvent.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find ApplicationKeySetEvent event.", details))

    return ok(new SetApplicationKeyTx(event, details))
  }

  async submitBlockLengthProposal(
    rows: number,
    cols: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SubmitBlockLengthProposalTx, TransactionFailed>> {
    const call = this.api.tx.dataAvailability.submitBlockLengthProposal(rows, cols)
    const tx = this.api.tx.sudo.sudo(call)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const sudoEvent = details.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) return err(new TransactionFailed("Failed to find Sudid event.", details))

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) return err(new TransactionFailed(decodeError(this.api, sudoResult.asErr), details))

    const event = Events.BlockLengthProposalSubmittedEvent.New(details.events)
    if (event == undefined)
      return err(new TransactionFailed("Failed to find BlockLengthProposalSubmittedEvent event.", details))

    return ok(new SubmitBlockLengthProposalTx(event, details))
  }

  async setSubmitDataFeeModifier(
    modifier: DispatchFeeModifier,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SetSubmitDataFeeModifierTx, TransactionFailed>> {
    const call = this.api.tx.dataAvailability.setSubmitDataFeeModifier(modifier)
    const tx = this.api.tx.sudo.sudo(call)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const sudoEvent = details.events.find((e) => e.event.method == "Sudid")
    if (sudoEvent == undefined) return err(new TransactionFailed("Failed to find Sudid event.", details))

    const sudoResult: any = (sudoEvent.event.data as any).sudoResult
    if (sudoResult.isErr) return err(new TransactionFailed(decodeError(this.api, sudoResult.asErr), details))

    const event = Events.SubmitDataFeeModifierSetEvent.New(details.events)
    if (event == undefined)
      return err(new TransactionFailed("Failed to find SubmitDataFeeModifierSetEvent event.", details))

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
      if (ed == undefined) return undefined

      return new DataSubmittedEvent(ed["who"].toString(), ed["dataHash"].toString())
    }
  }

  export class ApplicationKeyCreatedEvent {
    constructor(
      public key: string,
      public owner: string,
      public id: number,
    ) {}
    static New(events: EventRecord[]): ApplicationKeyCreatedEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "ApplicationKeyCreated")?.event.data
      if (ed == undefined) return undefined

      return new ApplicationKeyCreatedEvent(ed["key"].toString(), ed["owner"].toString(), parseInt(ed["id"].toString()))
    }
  }

  export class ApplicationKeySetEvent {
    constructor(
      public oldKey: string,
      public newKey: string,
    ) {}
    static New(events: EventRecord[]): ApplicationKeySetEvent | undefined {
      const ed: any = events.find((e) => e.event.method == "ApplicationKeySet")?.event.data
      if (ed == undefined) return undefined

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
      if (ed == undefined) return undefined

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
      if (ed == undefined) return undefined

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
      if (tx == undefined) return err("Failed to find submit data transaction.")

      // Data retrieved from the extrinsic data
      let dataHex = tx.method.args.map((a) => a.toString()).join(", ")
      if (dataHex.startsWith("0x")) {
        dataHex = dataHex.slice(2)
      }

      return ok(new SubmitData(dataHex))
    }
  }
}
