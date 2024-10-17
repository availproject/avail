import { ApiPromise } from "@polkadot/api"
import { Balances } from "./balances"
import { Staking } from "./staking"
import { DataAvailability } from "./da"
import { NominationPools } from "./nomination_pools"
import { Multisig } from "./multisig"

export { WaitFor } from "./common"
export { DispatchFeeModifier } from "./da"
export { StakingRewardDestination } from "./staking"
export { BondExtra, ClaimPermission, NewCommission, PoolState } from "./nomination_pools"

export class Transactions {
  private api: ApiPromise
  dataAvailability: DataAvailability
  balances: Balances
  staking: Staking
  nominationPools: NominationPools
  multisig: Multisig

  constructor(api: ApiPromise) {
    this.api = api
    this.dataAvailability = new DataAvailability(api)
    this.balances = new Balances(api)
    this.staking = new Staking(api)
    this.nominationPools = new NominationPools(api)
    this.multisig = new Multisig(api)
  }
}
