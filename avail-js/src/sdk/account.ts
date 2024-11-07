import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { H256, Keyring, SDK, sdkUtil, WaitFor } from "."
import { CreateApplicationKeyTx, SubmitDataTx } from "./transactions/da"
import { Bytes } from "@polkadot/types-codec"
import { getNonceState, getNonceNode } from "./utils"
import { TransferKeepAliveTx } from "./transactions/balances"
import { TransactionFailed } from "./transactions/common"
import { Result } from "neverthrow"

export interface AccountBalance {
  free: BN
  reserved: BN
  frozen: BN
  flags: BN
}

export class Account {
  waitFor: WaitFor = WaitFor.BlockInclusion
  nonce: number | null = null
  appId: number | null = null

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

  setWaitFor(value: WaitFor) {
    this.waitFor = value
  }

  address(): string {
    return this.keyring.address
  }

  async balanceTransfer(dest: string, value: BN): Promise<Result<TransferKeepAliveTx, TransactionFailed>> {
    return await this.sdk.tx.balances.transferKeepAlive(dest, value, this.waitFor, this.keyring, this.buildOptions())
  }

  async balanceTransferNoWait(dest: string, value: BN): Promise<H256> {
    return await this.sdk.tx.balances.transferKeepAliveNoWait(dest, value, this.keyring, this.buildOptions())
  }

  async submitData(data: string | Bytes): Promise<Result<SubmitDataTx, TransactionFailed>> {
    return await this.sdk.tx.dataAvailability.submitData(data, this.waitFor, this.keyring, this.buildOptions())
  }

  async submitDataNoWait(data: string | Bytes): Promise<H256> {
    return await this.sdk.tx.dataAvailability.submitDataNoWait(data, this.keyring, this.buildOptions())
  }

  async createApplicationKey(key: string): Promise<Result<CreateApplicationKeyTx, TransactionFailed>> {
    return await this.sdk.tx.dataAvailability.createApplicationKey(key, this.waitFor, this.keyring, this.buildOptions())
  }

  async createApplicationKeyNoWait(key: string): Promise<H256> {
    return await this.sdk.tx.dataAvailability.createApplicationKeyNoWait(key, this.keyring, this.buildOptions())
  }

  async getBalance(): Promise<AccountBalance> {
    const r: any = await this.sdk.api.query.system.account(this.keyring.address)
    return { free: r.data.free, reserved: r.data.reserved, frozen: r.data.frozen, flags: r.data.flags }
  }

  async getNonceState(): Promise<number> {
    return await getNonceState(this.sdk.api, this.keyring.address)
  }

  async getNonceNode(): Promise<number> {
    return await getNonceNode(this.sdk.api, this.keyring.address)
  }

  async getAppKeys(): Promise<number[]> {
    return sdkUtil.getAppKeys(this.sdk.api, this.keyring.address)
  }

  oneAvail(): BN {
    return SDK.oneAvail()
  }

  private buildOptions(): any {
    const options: any = {}
    if (this.nonce != null) {
      options.nonce = this.nonce
    }

    if (this.appId != null) {
      options.app_id = this.appId
    }

    return options
  }
}
