import { ApiPromise } from "@polkadot/api"
import { EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import {
  WaitFor,
  TransactionOptions,
  singAndSendAndParseTransaction,
  TxResultDetails,
  TransactionFailed,
} from "./common"
import { commissionNumberToPerbill } from "../utils"
import { err, ok, Result } from "neverthrow"

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

export class CreateTx {
  constructor(
    public event: Events.Created,
    public event2: Events.Bonded,
    public details: TxResultDetails,
  ) {}
}

export class CreateWithPoolIdTx {
  constructor(
    public event: Events.Created,
    public event2: Events.Bonded,
    public details: TxResultDetails,
  ) {}
}

export class JoinTx {
  constructor(
    public event: Events.Bonded,
    public details: TxResultDetails,
  ) {}
}

export class NominateTx {
  constructor(public details: TxResultDetails) {}
}

export class BondExtraTx {
  constructor(
    public event: Events.Bonded,
    public details: TxResultDetails,
  ) {}
}

export class SetMetadataTx {
  constructor(public details: TxResultDetails) {}
}

export class UnbondTx {
  constructor(
    public event: Events.Unbonded | undefined,
    public details: TxResultDetails,
  ) {}
}

export class ChillTx {
  constructor(public details: TxResultDetails) {}
}

export class ClaimCommissionTx {
  constructor(
    public event: Events.PoolCommissionClaimed,
    public details: TxResultDetails,
  ) {}
}

export class ClaimPayoutTx {
  constructor(
    public event: Events.PaidOut | undefined,
    public details: TxResultDetails,
  ) {}
}

export class ClaimPayoutOtherTx {
  constructor(
    public event: Events.PaidOut | undefined,
    public details: TxResultDetails,
  ) {}
}

export class SetClaimPermissionTx {
  constructor(public details: TxResultDetails) {}
}

export class SetCommissionTx {
  constructor(
    public event: Events.PoolCommissionUpdated,
    public details: TxResultDetails,
  ) {}
}

export class WithdrawUnbodedTx {
  constructor(
    public event: Events.Withdrawn,
    public details: TxResultDetails,
  ) {}
}

export class SetStateTx {
  constructor(
    public event: Events.StateChanged | undefined,
    public details: TxResultDetails,
  ) {}
}

export class NominationPools {
  private api: ApiPromise

  constructor(api: ApiPromise) {
    this.api = api
  }

  async create(
    amount: BN,
    root: string,
    nominator: string,
    bouncer: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<CreateTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.create(amount, root, nominator, bouncer)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Created.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Created event", details))

    const event2 = Events.Bonded.New(details.events)
    if (event2 == undefined) return err(new TransactionFailed("Failed to find Bonded event", details))

    return ok(new CreateTx(event, event2, details))
  }

  async createWithPoolId(
    amount: BN,
    root: string,
    nominator: string,
    bouncer: string,
    poolId: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<CreateWithPoolIdTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.createWithPoolId(amount, root, nominator, bouncer, poolId)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Created.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Created event", details))

    const event2 = Events.Bonded.New(details.events)
    if (event2 == undefined) return err(new TransactionFailed("Failed to find Bonded event", details))

    return ok(new CreateWithPoolIdTx(event, event2, details))
  }

  async join(
    amount: BN,
    poolId: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<JoinTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.join(amount, poolId)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Bonded.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Bonded event", details))

    return ok(new JoinTx(event, details))
  }

  async nominate(
    poolId: number,
    validators: string[],
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<NominateTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.nominate(poolId, validators)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    return ok(new NominateTx(details))
  }

  async bondExtra(
    extra: BondExtra,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<BondExtraTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.bondExtra(extra)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Bonded.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Bonded event", details))

    return ok(new BondExtraTx(event, details))
  }

  async setMetadata(
    poolId: number,
    metadata: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SetMetadataTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.setMetadata(poolId, metadata)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    return ok(new SetMetadataTx(details))
  }

  async unbond(
    memberAccount: string,
    unbondingPoints: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<UnbondTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.unbond(memberAccount, unbondingPoints)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value
    const event = Events.Unbonded.New(details.events)

    return ok(new UnbondTx(event, details))
  }

  async chill(
    poolId: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ChillTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.chill(poolId)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    return ok(new ChillTx(details))
  }

  async claimCommission(
    poolId: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ClaimCommissionTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.claimCommission(poolId)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.PoolCommissionClaimed.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find PoolCommissionClaimed event", details))

    return ok(new ClaimCommissionTx(event, details))
  }

  async claimPayout(
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ClaimPayoutTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.claimPayout()
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value
    const event = Events.PaidOut.New(details.events)

    return ok(new ClaimPayoutTx(event, details))
  }

  async claimPayoutOther(
    other: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<ClaimPayoutOtherTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.claimPayoutOther(other)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value
    const event = Events.PaidOut.New(details.events)

    return ok(new ClaimPayoutOtherTx(event, details))
  }

  async setClaimPermission(
    permission: ClaimPermission,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SetClaimPermissionTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.setClaimPermission(permission)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    return ok(new SetClaimPermissionTx(details))
  }

  async setCommission(
    poolId: number,
    newCommission: NewCommission | null,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SetCommissionTx, TransactionFailed>> {
    let commission: string[] | null = null
    if (newCommission != null) {
      const amount = commissionNumberToPerbill(newCommission.amount)
      if (amount.isErr()) return err(new TransactionFailed(amount.error, null))

      commission = [amount.value, newCommission.payee]
    }

    const tx = this.api.tx.nominationPools.setCommission(poolId, commission)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.PoolCommissionUpdated.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find PoolCommissionUpdated event", details))

    return ok(new SetCommissionTx(event, details))
  }

  async withdrawUnbonded(
    memberAccount: string,
    numSlashingSpans: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<WithdrawUnbodedTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.withdrawUnbonded(memberAccount, numSlashingSpans)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value

    const event = Events.Withdrawn.New(details.events)
    if (event == undefined) return err(new TransactionFailed("Failed to find Withdrawn event", details))

    return ok(new WithdrawUnbodedTx(event, details))
  }

  async setState(
    poolId: number,
    state: PoolState,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<Result<SetStateTx, TransactionFailed>> {
    const tx = this.api.tx.nominationPools.setState(poolId, state)
    const maybeParsed = await singAndSendAndParseTransaction(this.api, tx, account, waitFor, options)
    if (maybeParsed.isErr()) return err(maybeParsed.error)

    const details = maybeParsed.value
    const event = Events.StateChanged.New(details.events)

    return ok(new SetStateTx(event, details))
  }
}

namespace Events {
  export class Bonded {
    constructor(
      public member: string,
      public poolId: string,
      public bonded: string,
      public joined: string,
    ) {}
    static New(events: EventRecord[]): Bonded | undefined {
      const ed: any = events.find((e) => e.event.method == "Bonded" && e.event.section == "nominationPools")?.event.data
      if (ed == undefined) return undefined

      return new Bonded(
        ed["member"].toString(),
        ed["poolId"].toString(),
        ed["bonded"].toString(),
        ed["joined"].toString(),
      )
    }
  }

  export class Created {
    constructor(
      public depositor: string,
      public poolId: string,
    ) {}
    static New(events: EventRecord[]): Created | undefined {
      const ed: any = events.find((e) => e.event.method == "Created" && e.event.section == "nominationPools")?.event
        .data
      if (ed == undefined) return undefined

      return new Created(ed["depositor"].toString(), ed["poolId"].toString())
    }
  }

  export class Unbonded {
    constructor(
      public member: string,
      public poolId: string,
      public balance: string,
      public points: string,
      public era: string,
    ) {}
    static New(events: EventRecord[]): Unbonded | undefined {
      const ed: any = events.find((e) => e.event.method == "Unbonded" && e.event.section == "nominationPools")?.event
        .data
      if (ed == undefined) return undefined

      return new Unbonded(
        ed["member"].toString(),
        ed["poolId"].toString(),
        ed["balance"].toString(),
        ed["points"].toString(),
        ed["era"].toString(),
      )
    }
  }

  export class PoolCommissionClaimed {
    constructor(
      public poolId: string,
      public commission: string,
    ) {}
    static New(events: EventRecord[]): PoolCommissionClaimed | undefined {
      const ed: any = events.find(
        (e) => e.event.method == "PoolCommissionClaimed" && e.event.section == "nominationPools",
      )?.event.data
      if (ed == undefined) return undefined

      return new PoolCommissionClaimed(ed["poolId"].toString(), ed["commission"].toString())
    }
  }

  export class PaidOut {
    constructor(
      public member: string,
      public poolId: string,
      public payout: string,
    ) {}
    static New(events: EventRecord[]): PaidOut | undefined {
      const ed: any = events.find((e) => e.event.method == "PaidOut" && e.event.section == "nominationPools")?.event
        .data
      if (ed == undefined) return undefined

      return new PaidOut(ed["member"].toString(), ed["poolId"].toString(), ed["payout"].toString())
    }
  }

  export class PoolCommissionUpdated {
    constructor(
      public poolId: string,
      public current: string,
    ) {}
    static New(events: EventRecord[]): PoolCommissionUpdated | undefined {
      const ed: any = events.find(
        (e) => e.event.method == "PoolCommissionUpdated" && e.event.section == "nominationPools",
      )?.event.data
      if (ed == undefined) return undefined

      return new PoolCommissionUpdated(ed["poolId"].toString(), ed["current"].toString())
    }
  }

  export class Withdrawn {
    constructor(
      public member: string,
      public poolId: string,
      public balance: string,
      public points: string,
    ) {}
    static New(events: EventRecord[]): Withdrawn | undefined {
      const ed: any = events.find((e) => e.event.method == "Withdrawn" && e.event.section == "nominationPools")?.event
        .data
      if (ed == undefined) return undefined

      return new Withdrawn(
        ed["member"].toString(),
        ed["poolId"].toString(),
        ed["balance"].toString(),
        ed["points"].toString(),
      )
    }
  }

  export class StateChanged {
    constructor(
      public poolId: string,
      public newState: string,
    ) {}
    static New(events: EventRecord[]): StateChanged | undefined {
      const ed: any = events.find((e) => e.event.method == "StateChanged" && e.event.section == "nominationPools")
        ?.event.data
      if (ed == undefined) return undefined

      return new StateChanged(ed["poolId"].toString(), ed["newState"].toString())
    }
  }
}
