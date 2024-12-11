import { ApiPromise } from "@polkadot/api"
import { BN } from "@polkadot/util"
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

  transferAllowDeath(dest: string, value: BN): Transaction {
    const tx = this.api.tx.balances.transferAllowDeath(dest, value)
    return new Transaction(this.api, tx)
  }

  transferKeepAlive(dest: string, value: BN): Transaction {
    const tx = this.api.tx.balances.transferKeepAlive(dest, value)
    return new Transaction(this.api, tx)
  }
}
