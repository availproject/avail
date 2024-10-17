import { ApiPromise } from "@polkadot/api"
import { initialize } from "../chain"
import { Transactions } from "./transactions"
import { Utils } from "./utils"
import { BN } from "@polkadot/util"

export * as sdkTransactions from "./transactions"
export * as sdkTransactionData from "./transaction_data"

export { BN } from "@polkadot/util"
export { Keyring } from "@polkadot/api"
export { KeyringPair } from "@polkadot/keyring/types"
export { Bytes } from "@polkadot/types-codec"
export { H256, Weight } from "@polkadot/types/interfaces"
export { TxResultDetails, MultisigTimepoint } from "./utils"
export { Account } from "./account"

export {
  WaitFor,
  StakingRewardDestination,
  DispatchFeeModifier,
  BondExtra,
  ClaimPermission,
  NewCommission,
  PoolState,
  TransactionOptions,
} from "./transactions"

export class SDK {
  api: ApiPromise
  tx: Transactions
  util: Utils

  static async New(endpoint: string): Promise<SDK> {
    const api = await initialize(endpoint)
    return new SDK(api)
  }

  private constructor(api: ApiPromise) {
    this.api = api
    this.tx = new Transactions(api)
    this.util = new Utils(api)
  }

  static oneAvail(): BN {
    return new BN(10).pow(new BN(18))
  }
}
