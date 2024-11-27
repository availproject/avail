import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { Keyring, SDK, utils } from "."
import { Bytes } from "@polkadot/types-codec"
import { Transaction, TransactionOptions } from "./transactions/common"

export interface AccountBalance {
  free: BN
  reserved: BN
  frozen: BN
  flags: BN
}

export class Account {
  private nonce: number | null = null
  private appId: number | null = null
  private tip: BN | null = null

  constructor(
    public sdk: SDK,
    public keyring: KeyringPair,
  ) {}

  static alice(sdk: SDK): Account {
    return new Account(sdk, new Keyring({ type: "sr25519" }).addFromUri("//Alice"))
  }

  setNonce(value: number | null) {
    this.nonce = value
  }

  setAppId(value: number | null) {
    this.appId = value
  }

  setTip(value: BN | null) {
    this.tip = value
  }

  address(): string {
    return this.keyring.address
  }

  balanceTransfer(dest: string, value: BN): Transaction {
    return this.sdk.tx.balances.transferKeepAlive(dest, value)
  }

  submitData(data: string | Bytes): Transaction {
    return this.sdk.tx.dataAvailability.submitData(data)
  }

  createApplicationKey(key: string): Transaction {
    return this.sdk.tx.dataAvailability.createApplicationKey(key)
  }

  async getBalance(): Promise<AccountBalance> {
    const r: any = await this.sdk.api.query.system.account(this.keyring.address)
    return { free: r.data.free, reserved: r.data.reserved, frozen: r.data.frozen, flags: r.data.flags }
  }

  async getNonceState(): Promise<number> {
    return await utils.getNonceState(this.sdk.api, this.keyring.address)
  }

  async getNonceNode(): Promise<number> {
    return await utils.getNonceNode(this.sdk.api, this.keyring.address)
  }

  async getAppKeys(): Promise<[string, number][]> {
    return await utils.getAppKeys(this.sdk.api, this.keyring.address)
  }

  async getAppIds(): Promise<number[]> {
    return await utils.getAppIds(this.sdk.api, this.keyring.address)
  }

  oneAvail(): BN {
    return SDK.oneAvail()
  }

  private buildOptions(): TransactionOptions {
    const options: TransactionOptions = {}
    if (this.nonce != null) {
      options.nonce = this.nonce
    }

    if (this.appId != null) {
      options.app_id = this.appId
    }

    if (this.tip != null) {
      options.tip = this.tip
    }

    return options
  }
}
