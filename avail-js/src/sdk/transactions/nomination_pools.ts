import { ApiPromise } from "@polkadot/api"
import { BN } from "@polkadot/util"
import { Transaction } from "./common"
import { commissionNumberToPerbill } from "../utils"

export interface BondExtra {
  FreeBalance?: BN
  Rewards?: null
}

export type ClaimPermission = "Permissioned" | "PermissionlessCompound" | "PermissionlessWithdraw" | "PermissionlessAll"
export type PoolState = "Open" | "Blocked" | "Destroying"

export interface NewCommission {
  amount: number
  payee: string
}

export class NominationPools {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  create(amount: BN, root: string, nominator: string, bouncer: string): Transaction {
    const tx = this.api.tx.nominationPools.create(amount, root, nominator, bouncer)
    return new Transaction(this.api, tx)
  }

  createWithPoolId(amount: BN, root: string, nominator: string, bouncer: string, poolId: number): Transaction {
    const tx = this.api.tx.nominationPools.createWithPoolId(amount, root, nominator, bouncer, poolId)
    return new Transaction(this.api, tx)
  }

  join(amount: BN, poolId: number): Transaction {
    const tx = this.api.tx.nominationPools.join(amount, poolId)
    return new Transaction(this.api, tx)
  }

  nominate(poolId: number, validators: string[]): Transaction {
    const tx = this.api.tx.nominationPools.nominate(poolId, validators)
    return new Transaction(this.api, tx)
  }

  bondExtra(extra: BondExtra): Transaction {
    const tx = this.api.tx.nominationPools.bondExtra(extra)
    return new Transaction(this.api, tx)
  }

  setMetadata(poolId: number, metadata: string): Transaction {
    const tx = this.api.tx.nominationPools.setMetadata(poolId, metadata)
    return new Transaction(this.api, tx)
  }

  unbond(memberAccount: string, unbondingPoints: BN): Transaction {
    const tx = this.api.tx.nominationPools.unbond(memberAccount, unbondingPoints)
    return new Transaction(this.api, tx)
  }

  chill(poolId: number): Transaction {
    const tx = this.api.tx.nominationPools.chill(poolId)
    return new Transaction(this.api, tx)
  }

  claimCommission(poolId: number): Transaction {
    const tx = this.api.tx.nominationPools.claimCommission(poolId)
    return new Transaction(this.api, tx)
  }

  claimPayout(): Transaction {
    const tx = this.api.tx.nominationPools.claimPayout()
    return new Transaction(this.api, tx)
  }

  claimPayoutOther(other: string): Transaction {
    const tx = this.api.tx.nominationPools.claimPayoutOther(other)
    return new Transaction(this.api, tx)
  }

  setClaimPermission(permission: ClaimPermission): Transaction {
    const tx = this.api.tx.nominationPools.setClaimPermission(permission)
    return new Transaction(this.api, tx)
  }

  setCommission(poolId: number, newCommission: NewCommission | null): Transaction {
    let commission: string[] | null = null
    if (newCommission != null) {
      const amount = commissionNumberToPerbill(newCommission.amount)
      if (amount.isErr()) throw Error(amount.error)

      commission = [amount.value, newCommission.payee]
    }

    const tx = this.api.tx.nominationPools.setCommission(poolId, commission)
    return new Transaction(this.api, tx)
  }

  withdrawUnbonded(memberAccount: string, numSlashingSpans: number): Transaction {
    const tx = this.api.tx.nominationPools.withdrawUnbonded(memberAccount, numSlashingSpans)
    return new Transaction(this.api, tx)
  }

  setState(poolId: number, state: PoolState): Transaction {
    const tx = this.api.tx.nominationPools.setState(poolId, state)
    return new Transaction(this.api, tx)
  }
}
