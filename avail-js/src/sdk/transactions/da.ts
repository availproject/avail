import { ApiPromise } from "@polkadot/api"
import { H256 } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { TransactionOptions, Transaction } from "./common"
import { Bytes } from "@polkadot/types-codec"

export type DispatchFeeModifier = {
  weightMaximumFee: BN | null
  weightFeeDivider: number | null
  weightFeeMultiplier: number | null
}

export class DataAvailability {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  submitData(data: string | Bytes): Transaction {
    const tx = this.api.tx.dataAvailability.submitData(data)
    return new Transaction(this.api, tx)
  }

  async submitDataNoWait(data: string | Bytes, account: KeyringPair, options?: TransactionOptions): Promise<H256> {
    const optionWrapper = options || {}
    return this.api.tx.dataAvailability.submitData(data).signAndSend(account, optionWrapper)
  }

  createApplicationKey(key: string): Transaction {
    const tx = this.api.tx.dataAvailability.createApplicationKey(key)
    return new Transaction(this.api, tx)
  }

  async createApplicationKeyNoWait(key: string, account: KeyringPair, options?: TransactionOptions): Promise<H256> {
    const optionWrapper = options || {}
    return this.api.tx.dataAvailability.createApplicationKey(key).signAndSend(account, optionWrapper)
  }
}
