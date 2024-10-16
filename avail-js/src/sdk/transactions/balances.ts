import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result, ok } from "neverthrow"

import { SignerOptions } from "@polkadot/api/types"
import { WaitFor, standardCallback, TransactionFailed } from "./common"
import { parseTransactionResult, TxResultDetails } from "../utils"

export class TransferKeepAliveTx {
  constructor(
    public event: Events.TransferEvent,
    public details: TxResultDetails,
  ) {}
}

export class TransferAllowDeathTx {
  constructor(
    public event: Events.TransferEvent,
    public event2: Events.KilledAccount | undefined,
    public details: TxResultDetails,
  ) {}
}

export class TransferAllTx {
  constructor(
    public event: Events.TransferEvent,
    public event2: Events.KilledAccount | undefined,
    public details: TxResultDetails,
  ) {}
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
  ): Promise<Result<TransferAllTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const event = Events.TransferEvent.New(details.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find Transfer event", details))
    }
    const event2 = Events.KilledAccount.New(txResult.events)

    return ok(new TransferAllTx(event, event2, details))
  }

  async transferAllowDeath(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<Result<TransferAllowDeathTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const event = Events.TransferEvent.New(details.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find Transfer event", details))
    }
    const event2 = Events.KilledAccount.New(txResult.events)

    return ok(new TransferAllowDeathTx(event, event2, details))
  }

  async transferKeepAlive(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: Partial<SignerOptions>,
  ): Promise<Result<TransferKeepAliveTx, TransactionFailed>> {
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
      return err(new TransactionFailed(maybeTxResult.error, null))
    }
    const txResult = maybeTxResult.value
    const maybeParsed = await parseTransactionResult(this.api, txResult, waitFor)
    if (maybeParsed.isErr()) {
      return err(maybeParsed.error)
    }
    const details = maybeParsed.value

    const event = Events.TransferEvent.New(details.events)
    if (event == undefined) {
      return err(new TransactionFailed("Failed to find Transfer event", details))
    }

    return ok(new TransferKeepAliveTx(event, details))
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
