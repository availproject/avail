import { ApiPromise, Keyring } from "@polkadot/api"
import { initialize } from "../chain"
import { Transactions } from "./transactions"
import { Utils } from "./utils"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"

export { BN } from "@polkadot/util"
export { Keyring } from "@polkadot/api"
export { KeyringPair } from "@polkadot/keyring/types"
export { Bytes } from "@polkadot/types-codec"
export { H256, Weight } from "@polkadot/types/interfaces"
export { Account } from "./account"
export { DataSubmission } from "./block"
export {
  WaitFor,
  StakingRewardDestination,
  DispatchFeeModifier,
  BondExtra,
  ClaimPermission,
  NewCommission,
  PoolState,
  TransactionOptions,
  TxResultDetails,
  MultisigTimepoint,
} from "./transactions"
export { Block } from "./block"

export * as sdkBlock from "./block"
export * as sdkUtil from "./utils"
export * as sdkTransactions from "./transactions"

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

  static localEndpoint(): string {
    return "ws://127.0.0.1:9944"
  }

  static turingEndpoint(): string {
    return "wss://turing-rpc.avail.so/ws"
  }

  static mainnetEndpoint(): string {
    return "wss://mainnet-rpc.avail.so/ws"
  }

  static alice(): KeyringPair {
    return new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  }

  static bob(): KeyringPair {
    return new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  }

  static charlie(): KeyringPair {
    return new Keyring({ type: "sr25519" }).addFromUri("//Charlie")
  }
}
