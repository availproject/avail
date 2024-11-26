import { ApiPromise } from "@polkadot/api"
import { BN } from "@polkadot/util"
import { Transaction } from "./common"
import { commissionNumberToPerbill } from "../utils"

type ValidatorPerfs = { commission: string; blocked: boolean }
export type StakingRewardDestination = "Staked" | "Stash" | "None" | { account: string }

export class Staking {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  bond(value: BN, payee: StakingRewardDestination): Transaction {
    const tx = this.api.tx.staking.bond(value, payee)
    return new Transaction(this.api, tx)
  }

  bondExtra(maxAdditional: BN): Transaction {
    const tx = this.api.tx.staking.bondExtra(maxAdditional)
    return new Transaction(this.api, tx)
  }

  chill(): Transaction {
    const tx = this.api.tx.staking.chill()
    return new Transaction(this.api, tx)
  }

  chillOther(stash: string): Transaction {
    const tx = this.api.tx.staking.chillOther(stash)
    return new Transaction(this.api, tx)
  }

  nominate(targets: string[]): Transaction {
    const tx = this.api.tx.staking.nominate(targets)
    return new Transaction(this.api, tx)
  }

  unbond(value: BN): Transaction {
    const tx = this.api.tx.staking.unbond(value)
    return new Transaction(this.api, tx)
  }

  validate(commission: number, blocked: boolean): Transaction {
    const maybeCommission = commissionNumberToPerbill(commission)
    if (maybeCommission.isErr()) throw Error(maybeCommission.error)

    const validatorPerfs = { commission: maybeCommission.value, blocked } as ValidatorPerfs
    const tx = this.api.tx.staking.validate(validatorPerfs)
    return new Transaction(this.api, tx)
  }

  payoutStakers(validatorStash: String, era: number): Transaction {
    const tx = this.api.tx.staking.payoutStakers(validatorStash, era)
    return new Transaction(this.api, tx)
  }
}
