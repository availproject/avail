import { ApiPromise } from "@polkadot/api"
import { BN } from "@polkadot/util"
import { Transaction } from "./common"
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

  createApplicationKey(key: string): Transaction {
    const tx = this.api.tx.dataAvailability.createApplicationKey(key)
    return new Transaction(this.api, tx)
  }
}
