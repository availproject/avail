import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { Keyring, SDK, utils } from "."
import { ApiPromise } from "@polkadot/api"

export interface AccountBalance {
  free: BN
  reserved: BN
  frozen: BN
  flags: BN
}

export class Account {
  constructor(
    public api: ApiPromise,
    public keyring: KeyringPair,
  ) {}

  static alice(api: ApiPromise): Account {
    return new Account(api, new Keyring({ type: "sr25519" }).addFromUri("//Alice"))
  }

  address(): string {
    return this.keyring.address
  }

  async getBalance(): Promise<AccountBalance> {
    const r: any = await this.api.query.system.account(this.keyring.address)
    return { free: r.data.free, reserved: r.data.reserved, frozen: r.data.frozen, flags: r.data.flags }
  }

  async getNonceState(): Promise<number> {
    return await utils.getNonceState(this.api, this.keyring.address)
  }

  async getNonceNode(): Promise<number> {
    return await utils.getNonceNode(this.api, this.keyring.address)
  }

  async getAppKeys(): Promise<[string, number][]> {
    return await utils.getAppKeys(this.api, this.keyring.address)
  }

  async getAppIds(): Promise<number[]> {
    return await utils.getAppIds(this.api, this.keyring.address)
  }

  oneAvail(): BN {
    return SDK.oneAvail()
  }
}
