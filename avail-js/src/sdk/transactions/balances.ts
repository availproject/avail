import { ApiPromise } from "@polkadot/api"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import {
  WaitFor,
  TransactionOptions,
  singAndSendAndParseTransaction,
  TxResultDetails,
  TransactionFailed,
} from "./common"
import { err, ok, Result } from "neverthrow"

export class TransferAllTx {
  constructor(
    public event: Events.TransferEvent,
    public event2: Events.KilledAccount | undefined,
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

export class TransferKeepAliveTx {
  constructor(
    public event: Events.TransferEvent,
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
    options?: TransactionOptions,
  ): Promise<Result<TransferAllTx, TransactionFailed>> {
    const tx = this.api.tx.balances.transferAll(dest, keepAlive)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.TransferEvent.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Transfer event", details))
    const event2 = Events.KilledAccount.New(details.events)

    return ok(new TransferAllTx(event, event2, details))
  }

  async transferAllNoWait(
    dest: string,
    keepAlive: boolean,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    return this.api.tx.balances.transferAll(dest, keepAlive).signAndSend(account, options || {})
  }

  async transferAllowDeath(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<TransferAllowDeathTx, TransactionFailed>> {
    const tx = this.api.tx.balances.transferAllowDeath(dest, value)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.TransferEvent.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Transfer event", details))
    const event2 = Events.KilledAccount.New(details.events)

    return ok(new TransferAllowDeathTx(event, event2, details))
  }

  async transferAllowDeathNoWait(
    dest: string,
    value: BN,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    return this.api.tx.balances.transferAllowDeath(dest, value).signAndSend(account, options || {})
  }

  async transferKeepAlive(
    dest: string,
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<TransferKeepAliveTx, TransactionFailed>> {
    const tx = this.api.tx.balances.transferKeepAlive(dest, value)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.TransferEvent.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Transfer event", details))

    return ok(new TransferKeepAliveTx(event, details))
  }

  async transferKeepAliveNoWait(
    dest: string,
    value: BN,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    return this.api.tx.balances.transferKeepAlive(dest, value).signAndSend(account, options || {})
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
      if (ed == undefined) return undefined

      return new TransferEvent(ed["from"].toString(), ed["to"].toString(), ed["amount"])
    }
  }

  export class KilledAccount {
    constructor(public account: string) {}
    static New(events: EventRecord[]): KilledAccount | undefined {
      const ed: any = events.find((e) => e.event.method == "KilledAccount" && e.event.section == "system")?.event.data
      if (ed == undefined) return undefined

      return new KilledAccount(ed["account"].toString())
    }
  }
}
