import { ApiPromise } from "@polkadot/api"
import { Weight } from "@polkadot/types/interfaces/types"
import { MultisigTimepoint, Transaction } from "./common"

export class Multisig {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  asMulti(
    threshold: number,
    otherSignatures: string[],
    timepoint: MultisigTimepoint | null,
    call: string,
    maxWeight: Weight,
  ): Transaction {
    const tx = this.api.tx.multisig.asMulti(threshold, otherSignatures, timepoint, call, maxWeight)
    return new Transaction(this.api, tx)
  }

  approveAsMulti(
    threshold: number,
    otherSignatures: string[],
    timepoint: MultisigTimepoint | null,
    callHash: string,
    maxWeight: Weight,
  ): Transaction {
    const tx = this.api.tx.multisig.approveAsMulti(threshold, otherSignatures, timepoint, callHash, maxWeight)
    return new Transaction(this.api, tx)
  }
}
