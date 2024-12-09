import { ApiPromise } from "@polkadot/api"
import { Transaction } from "./common"

export interface SessionKeys {
  babe: string
  grandpa: string
  imOnline: string
  authorityDiscover: string
}

export class Session {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  setKeys(keys: SessionKeys): Transaction {
    const tx = this.api.tx.session.setKeys(keys, [])
    return new Transaction(this.api, tx)
  }
}
