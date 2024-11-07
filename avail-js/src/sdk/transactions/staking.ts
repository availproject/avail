import { ApiPromise } from "@polkadot/api"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result, ok } from "neverthrow"
import {
  WaitFor,
  TransactionOptions,
  singAndSendAndParseTransaction,
  TxResultDetails,
  TransactionFailed,
} from "./common"
import { commissionNumberToPerbill } from "../utils"

type ValidatorPerfs = { commission: string; blocked: boolean }
export type StakingRewardDestination = "Staked" | "Stash" | "None" | { account: string }

export class BondTx {
  constructor(
    public event: Events.Bonded,
    public details: TxResultDetails,
  ) {}
}

export class BondExtraTx {
  constructor(
    public event: Events.Bonded,
    public details: TxResultDetails,
  ) {}
}

export class ChillTx {
  constructor(
    public event: Events.Chilled,
    public details: TxResultDetails,
  ) {}
}

export class ChillOtherTx {
  constructor(
    public event: Events.Chilled,
    public details: TxResultDetails,
  ) {}
}

export class UnbondTx {
  constructor(
    public event: Events.Unbonded,
    public details: TxResultDetails,
  ) {}
}

export class ValidateTx {
  constructor(
    public event: Events.ValidatorPrefsSet,
    public details: TxResultDetails,
  ) {}
}

export class NominateTx {
  constructor(
    public txData: TransactionData.Nominate,
    public details: TxResultDetails,
  ) {}
}

export class Staking {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  async bond(
    value: BN,
    payee: StakingRewardDestination,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<BondTx, TransactionFailed>> {
    const tx = this.api.tx.staking.bond(value, payee)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Bonded.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Bonded event", details))

    return ok(new BondTx(event, details))
  }

  async bondExtra(
    maxAdditional: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<BondExtraTx, TransactionFailed>> {
    const tx = this.api.tx.staking.bondExtra(maxAdditional)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Bonded.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Bonded event", details))

    return ok(new BondExtraTx(event, details))
  }

  async chill(
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ChillTx, TransactionFailed>> {
    const tx = this.api.tx.staking.chill()
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Chilled.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Chilled event", details))

    return ok(new ChillTx(event, details))
  }

  async chillOther(
    stash: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ChillOtherTx, TransactionFailed>> {
    const tx = this.api.tx.staking.chillOther(stash)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Chilled.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Chilled event", details))

    return ok(new ChillOtherTx(event, details))
  }

  async nominate(
    targets: string[],
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<NominateTx, TransactionFailed>> {
    const tx = this.api.tx.staking.nominate(targets)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const maybeTxData = await TransactionData.Nominate.New(this.api, details.txHash, details.blockHash)
    if (maybeTxData.isErr()) return err(new TransactionFailed(maybeTxData.error, details))

    return ok(new NominateTx(maybeTxData.value, details))
  }

  async unbond(
    value: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<UnbondTx, TransactionFailed>> {
    const tx = this.api.tx.staking.unbond(value)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Unbonded.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Unbonded event", details))

    return ok(new UnbondTx(event, details))
  }

  async validate(
    commission: number,
    blocked: boolean,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ValidateTx, TransactionFailed>> {
    const maybeCommission = commissionNumberToPerbill(commission)
    if (maybeCommission.isErr()) return err(new TransactionFailed(maybeCommission.error, null))

    const validatorPerfs = { commission: maybeCommission.value, blocked } as ValidatorPerfs
    const tx = this.api.tx.staking.validate(validatorPerfs)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.ValidatorPrefsSet.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Chilled ValidatorPrefsSet", details))

    return ok(new ValidateTx(event, details))
  }
}

export namespace Events {
  export class Bonded {
    constructor(
      public stash: string,
      public amount: string,
    ) {}
    static New(events: EventRecord[]): Bonded | undefined {
      const ed: any = events.find((e) => e.event.method == "Bonded")?.event.data
      if (ed == undefined) return undefined

      const amountString = ed["amount"].toString()
      const amount = new BN(amountString).div(new BN(10).pow(new BN(18))).toString()

      return new Bonded(ed["stash"].toString(), amount)
    }
  }

  export class Chilled {
    constructor(public stash: string) {}
    static New(events: EventRecord[]): Chilled | undefined {
      const ed: any = events.find((e) => e.event.method == "Chilled")?.event.data
      if (ed == undefined) return undefined

      return new Chilled(ed["stash"].toString())
    }
  }

  export class Unbonded {
    constructor(
      public stash: string,
      public amount: string,
    ) {}
    static New(events: EventRecord[]): Unbonded | undefined {
      const ed: any = events.find((e) => e.event.method == "Unbonded")?.event.data
      if (ed == undefined) return undefined

      return new Unbonded(ed["stash"].toString(), ed["amount"].toString())
    }
  }

  export class ValidatorPrefsSet {
    constructor(
      public stash: string,
      public commission: string,
      public blocked: string,
    ) {}
    static New(events: EventRecord[]): ValidatorPrefsSet | undefined {
      const ed: any = events.find((e) => e.event.method == "ValidatorPrefsSet")?.event.data
      if (ed == undefined) return undefined

      return new ValidatorPrefsSet(
        ed["stash"].toString(),
        ed["prefs"]["commission"].toString(),
        ed["prefs"]["blocked"].toString(),
      )
    }
  }
}

export namespace TransactionData {
  export class Nominate {
    constructor(public targets: string[]) {}

    static async New(api: ApiPromise, txHash: H256, blockHash: H256): Promise<Result<Nominate, string>> {
      const block = await api.rpc.chain.getBlock(blockHash)
      const tx = block.block.extrinsics.find((tx) => tx.hash.toHex() == txHash.toHex())
      if (tx == undefined) return err("Failed to find nominate transaction.")

      const targets = []
      const txTargets = tx.method.args[0] as any
      for (let i = 0; i < txTargets.length; ++i) {
        targets.push(txTargets[i].toString())
      }

      return ok(new Nominate(targets))
    }
  }
}
