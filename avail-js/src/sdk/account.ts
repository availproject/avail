import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { H256, SDK, sdkUtil, WaitFor } from "."
import { TransferKeepAliveTxSuccess } from "./transactions/balances"
import { GenericFailure } from "./transactions/common"
import { CreateApplicationKeyTxSuccess, SubmitDataTxSuccess } from "./transactions/da"
import { Bytes } from "@polkadot/types-codec"
import { getNonceState, getNonceNode } from "./utils"

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

  async balanceTransfer(dest: string, value: BN): Promise<TransferKeepAliveTxSuccess | GenericFailure> {
    const options = this.buildOptions()
    return await this.sdk.tx.balances.transferKeepAlive(dest, value, this.waitFor, this.keyring, options)
  }

  async balanceTransferNoWait(dest: string, value: BN): Promise<H256> {
    const options = this.buildOptions()
    return await this.sdk.tx.balances.transferKeepAliveNoWait(dest, value, this.keyring, options)
  }

  async submitData(data: string | Bytes): Promise<SubmitDataTxSuccess | GenericFailure> {
    const options = this.buildOptions()
    return await this.sdk.tx.dataAvailability.submitData(data, this.waitFor, this.keyring, options)
  }

  async submitDataNoWait(data: string | Bytes): Promise<H256> {
    const options = this.buildOptions()
    return await this.sdk.tx.dataAvailability.submitDataNoWait(data, this.keyring, options)
  }

  async createApplicationKey(key: string): Promise<CreateApplicationKeyTxSuccess | GenericFailure> {
    const options = this.buildOptions()
    return await this.sdk.tx.dataAvailability.createApplicationKey(key, this.waitFor, this.keyring, options)
  }

  async createApplicationKeyNoWait(key: string): Promise<H256> {
    const options = this.buildOptions()
    return await this.sdk.tx.dataAvailability.createApplicationKeyNoWait(key, this.keyring, options)
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
