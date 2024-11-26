import { ApiPromise } from "@polkadot/api"
import { H256 } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { TransactionOptions } from "./common"
import { Transaction } from "."

export class Balances {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  transferAll(dest: string, keepAlive: boolean): Transaction {
    const tx = this.api.tx.balances.transferAll(dest, keepAlive)
    return new Transaction(this.api, tx)
  }

  async transferAllNoWait(
    dest: string,
    keepAlive: boolean,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    return this.api.tx.balances.transferAll(dest, keepAlive).signAndSend(account, options || {})
  }

  transferAllowDeath(dest: string, value: BN): Transaction {
    const tx = this.api.tx.balances.transferAllowDeath(dest, value)
    return new Transaction(this.api, tx)
  }

  async transferAllowDeathNoWait(
    dest: string,
    value: BN,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<H256> {
    return this.api.tx.balances.transferAllowDeath(dest, value).signAndSend(account, options || {})
  }

  transferKeepAlive(dest: string, value: BN): Transaction {
    const tx = this.api.tx.balances.transferKeepAlive(dest, value)
    return new Transaction(this.api, tx)
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
