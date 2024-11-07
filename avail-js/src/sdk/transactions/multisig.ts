import { ApiPromise } from "@polkadot/api"
import { EventRecord, Weight } from "@polkadot/types/interfaces/types"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result, ok } from "neverthrow"
import {
  WaitFor,
  TransactionFailed,
  TransactionOptions,
  TxResultDetails,
  MultisigTimepoint,
  singAndSendAndParseTransaction,
} from "./common"

export class AsMultiTx {
  constructor(
    public event: Events.MultisigExecuted | undefined,
    public event2: Events.MultisigApproval | undefined,
    public details: TxResultDetails,
  ) {}
}

export class ApproveAsMultiTx {
  constructor(
    public event: Events.NewMultisig | undefined,
    public event2: Events.MultisigApproval | undefined,
    public details: TxResultDetails,
  ) {}
}

export class Multisig {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  async asMulti(
    threshold: number,
    otherSignatures: string[],
    timepoint: MultisigTimepoint | null,
    call: string,
    maxWeight: Weight,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<AsMultiTx, TransactionFailed>> {
    const tx = this.api.tx.multisig.asMulti(threshold, otherSignatures, timepoint, call, maxWeight)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.MultisigExecuted.New(details.events)
    const event2 = Events.MultisigApproval.New(details.events)

    return ok(new AsMultiTx(event, event2, details))
  }

  async approveAsMulti(
    threshold: number,
    otherSignatures: string[],
    timepoint: MultisigTimepoint | null,
    callHash: string,
    maxWeight: Weight,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ApproveAsMultiTx, TransactionFailed>> {
    const tx = this.api.tx.multisig.approveAsMulti(threshold, otherSignatures, timepoint, callHash, maxWeight)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.NewMultisig.New(details.events)
    const event2 = Events.MultisigApproval.New(details.events)

    return ok(new ApproveAsMultiTx(event, event2, details))
  }
}

export namespace Events {
  export class MultisigApproval {
    constructor(
      public approving: string,
      public timepoint: MultisigTimepoint,
      public multisig: string,
      public callHash: string,
    ) {}
    static New(events: EventRecord[]): MultisigApproval | undefined {
      const ed: any = events.find((e) => e.event.method == "MultisigApproval" && e.event.section == "multisig")?.event
        .data
      if (ed == undefined) {
        return undefined
      }

      const timepoint = {
        height: parseInt(ed["timepoint"].height.toString()),
        index: parseInt(ed["timepoint"].index.toString()),
      }

      return new MultisigApproval(
        ed["approving"].toString(),
        timepoint,
        ed["multisig"].toString(),
        ed["callHash"].toString(),
      )
    }
  }

  export class MultisigExecuted {
    constructor(
      public approving: string,
      public timepoint: MultisigTimepoint,
      public multisig: string,
      public callHash: string,
      public result: string,
    ) {}
    static New(events: EventRecord[]): MultisigExecuted | undefined {
      const ed: any = events.find((e) => e.event.method == "MultisigExecuted" && e.event.section == "multisig")?.event
        .data
      if (ed == undefined) {
        return undefined
      }

      const timepoint = {
        height: parseInt(ed["timepoint"].height.toString()),
        index: parseInt(ed["timepoint"].index.toString()),
      }

      return new MultisigExecuted(
        ed["approving"].toString(),
        timepoint,
        ed["multisig"].toString(),
        ed["callHash"].toString(),
        ed["result"].toString(),
      )
    }
  }

  export class NewMultisig {
    constructor(
      public approving: string,
      public multisig: string,
      public callHash: string,
    ) {}
    static New(events: EventRecord[]): NewMultisig | undefined {
      const ed: any = events.find((e) => e.event.method == "NewMultisig" && e.event.section == "multisig")?.event.data
      if (ed == undefined) {
        return undefined
      }

      return new NewMultisig(ed["approving"].toString(), ed["multisig"].toString(), ed["callHash"].toString())
    }
  }
}
