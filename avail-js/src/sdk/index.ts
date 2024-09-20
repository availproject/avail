import { ApiPromise } from "@polkadot/api"
import { initialize } from "../chain"
import { Transactions } from "./transactions"

export * as sdkEvents from "./events"
export * as sdkTransactions from "./transactions"
export * as sdkTransactionData from "./transaction_data"

export { BN } from "@polkadot/util"
export { Keyring } from "@polkadot/api"
export { Bytes } from "@polkadot/types-codec"
export {
  WaitFor,
  StakingRewardDestination,
  DispatchFeeModifier,
  BondExtra,
  ClaimPermission,
  NewCommission,
  PoolState,
} from "./transactions"

export class SDK {
  api: ApiPromise
  tx: Transactions

  static async New(endpoint: string): Promise<SDK> {
    const api = await initialize(endpoint)
    return new SDK(api)
  }

  private constructor(api: ApiPromise) {
    this.api = api
    this.tx = new Transactions(api)
  }
}
