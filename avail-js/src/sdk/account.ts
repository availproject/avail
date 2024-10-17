import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { Result } from "neverthrow"
import { H256, SDK, WaitFor } from "."
import { TransferKeepAliveTx } from "./transactions/balances"
import { TransactionFailed } from "./transactions/common"
import { CreateApplicationKeyTx, SubmitDataTx } from "./transactions/da"
import { Bytes } from "@polkadot/types-codec"

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

  async balanceTransfer(dest: string, value: BN): Promise<Result<TransferKeepAliveTx, TransactionFailed>> {
    const options = this.buildOptions()
    return await this.sdk.tx.balances.transferKeepAlive(dest, value, this.waitFor, this.keyring, options)
  }

  async balanceTransferNoWait(dest: string, value: BN): Promise<H256> {
    const options = this.buildOptions()
    return await this.sdk.tx.balances.transferKeepAliveNoWait(dest, value, this.keyring, options)
  }

  async submitData(data: string | Bytes): Promise<Result<SubmitDataTx, TransactionFailed>> {
    const options = this.buildOptions()
    return await this.sdk.tx.dataAvailability.submitData(data, this.waitFor, this.keyring, options)
  }

  async submitDataNoWait(data: string | Bytes): Promise<H256> {
    const options = this.buildOptions()
    return await this.sdk.tx.dataAvailability.submitDataNoWait(data, this.keyring, options)
  }

  async createApplicationKey(key: string): Promise<Result<CreateApplicationKeyTx, TransactionFailed>> {
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
    const r: any = await this.sdk.api.query.system.account(this.keyring.address)
    return parseInt(r.nonce.toString())
  }

  async getNonceNode(): Promise<number> {
    const r: any = await this.sdk.api.rpc.system.accountNextIndex(this.keyring.address)
    return parseInt(r.toString())
  }

  async appKeys(): Promise<number[]> {
    const appKeys: number[] = []
    const entries = await this.sdk.api.query.dataAvailability.appKeys.entries()
    entries.forEach((entry: any) => {
      if (entry[1].isSome) {
        let { owner, id } = entry[1].unwrap()
        if (owner.toString() == this.keyring.address) {
          appKeys.push(parseInt(id.toString()))
        }
      }
    })

    return appKeys.sort((a, b) => a - b)
  }

  oneAvail(): BN {
    return SDK.oneAvail()
  }

  private buildOptions(): any {
    let options: any = {}
    if (this.nonce != null) {
      options.nonce = this.nonce
    }

    if (this.appId != null) {
      options.app_id = this.appId
    }

    return options
  }
}
