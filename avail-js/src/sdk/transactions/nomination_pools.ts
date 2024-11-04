import { ApiPromise } from "@polkadot/api"
import { ISubmittableResult } from "@polkadot/types/types/extrinsic"
import { H256, EventRecord } from "@polkadot/types/interfaces/types"
import { BN } from "@polkadot/util"
import { KeyringPair } from "@polkadot/keyring/types"
import { err, Result } from "neverthrow"
import { WaitFor, GenericFailure, standardCallback, TransactionOptions } from "./common"
import { commissionNumberToPerbill, parseTransactionResult } from "../utils"

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

export type PoolCreateTxSuccess = {
  isErr: false
  event: Events.Created
  event2: Events.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolCreateWithPoolIdTxSuccess = {
  isErr: false
  event: Events.Created
  event2: Events.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolJoinTxSuccess = {
  isErr: false
  event: Events.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolNominateTxSuccess = {
  isErr: false
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolBondExtraTxSuccess = {
  isErr: false
  event: Events.Bonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolSetMetadataTxSuccess = {
  isErr: false
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolUnbondTxSuccess = {
  isErr: false
  event?: Events.Unbonded
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolChillTxSuccess = {
  isErr: false
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolClaimCommissionTxSuccess = {
  isErr: false
  event: Events.PoolCommissionClaimed
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolClaimPayoutTxSuccess = {
  isErr: false
  event?: Events.PaidOut
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolClaimPayoutOtherTxSuccess = {
  isErr: false
  event?: Events.PaidOut
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolSetClaimPermissionOtherTxSuccess = {
  isErr: false
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolSetCommissionTxSuccess = {
  isErr: false
  event: Events.PoolCommissionUpdated
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolWithdrawUnbodedTxSuccess = {
  isErr: false
  event: Events.Withdrawn
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
}

export type PoolSetStateTxSuccess = {
  isErr: false
  event?: Events.StateChanged
  events: EventRecord[]
  txHash: H256
  txIndex: number
  blockHash: H256
  blockNumber: number
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
  ): Promise<PoolCreateTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .create(amount, root, nominator, bouncer)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Created.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Created event." }
    }

    const event2 = Events.Bonded.New(events)
    if (event2 == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." }
    }

    return { isErr: false, event, event2, events, txHash, txIndex, blockHash, blockNumber }
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
  ): Promise<PoolCreateWithPoolIdTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .createWithPoolId(amount, root, nominator, bouncer, poolId)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Created.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Created event." }
    }

    const event2 = Events.Bonded.New(events)
    if (event2 == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." }
    }

    return {
      isErr: false,
      event,
      event2,
      events,
      txHash,
      txIndex,
      blockHash,
      blockNumber,
    }
  }

  async join(
    amount: BN,
    poolId: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolJoinTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .join(amount, poolId)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Bonded.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async nominate(
    poolId: number,
    validators: string[],
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolNominateTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .nominate(poolId, validators)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    return { isErr: false, events, txHash, txIndex, blockHash, blockNumber }
  }

  async bondExtra(
    extra: BondExtra,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolBondExtraTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .bondExtra(extra)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Bonded.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Bonded event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async setMetadata(
    poolId: number,
    metadata: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolSetMetadataTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .setMetadata(poolId, metadata)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    return { isErr: false, events, txHash, txIndex, blockHash, blockNumber }
  }

  async unbond(
    memberAccount: string,
    unbondingPoints: BN,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolUnbondTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .unbond(memberAccount, unbondingPoints)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Unbonded.New(events)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async chill(
    poolId: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolChillTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .chill(poolId)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    return { isErr: false, events, txHash, txIndex, blockHash, blockNumber } as PoolChillTxSuccess
  }

  async claimCommission(
    poolId: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolClaimCommissionTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .claimCommission(poolId)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.PoolCommissionClaimed.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find PoolCommissionClaimed event." }
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async claimPayout(
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolClaimPayoutTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .claimPayout()
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.PaidOut.New(events)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async claimPayoutOther(
    other: string,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolClaimPayoutOtherTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .claimPayoutOther(other)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.PaidOut.New(events)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async setClaimPermission(
    permission: ClaimPermission,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolSetClaimPermissionOtherTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .setClaimPermission(permission)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    return { isErr: false, events, txHash, txIndex, blockHash, blockNumber }
  }

  async setCommission(
    poolId: number,
    newCommission: NewCommission | null,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolSetCommissionTxSuccess | GenericFailure> {
    const optionWrapper = options || {}

    let commission: string[] | null = null
    if (newCommission != null) {
      const amount = commissionNumberToPerbill(newCommission.amount)
      if (amount.isErr()) {
        return { isErr: true, reason: amount.error } as GenericFailure
      }
      commission = [amount.value, newCommission.payee]
    }
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .setCommission(poolId, commission)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.PoolCommissionUpdated.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find PoolCommissionUpdated event." }
    }
    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async withdrawUnbonded(
    memberAccount: string,
    numSlashingSpans: number,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolWithdrawUnbodedTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .withdrawUnbonded(memberAccount, numSlashingSpans)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.Withdrawn.New(events)
    if (event == undefined) {
      return { isErr: true, reason: "Failed to find Withdraw event." } as GenericFailure
    }

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
  }

  async setState(
    poolId: number,
    state: PoolState,
    waitFor: WaitFor,
    account: KeyringPair,
    options?: TransactionOptions,
  ): Promise<PoolSetStateTxSuccess | GenericFailure> {
    const optionWrapper = options || {}
    const maybeTxResult = await new Promise<Result<ISubmittableResult, string>>((res, _) => {
      this.api.tx.nominationPools
        .setState(poolId, state)
        .signAndSend(account, optionWrapper, (result: ISubmittableResult) => {
          standardCallback(result, res, waitFor)
        })
        .catch((reason) => {
          res(err(reason))
        })
    })

    if (maybeTxResult.isErr()) {
      return { isErr: true, reason: maybeTxResult.error }
    }
    const maybeParsed = await parseTransactionResult(this.api, maybeTxResult.value, waitFor)
    if (maybeParsed.isErr()) {
      return { isErr: true, reason: maybeParsed.error.reason }
    }
    const details = maybeParsed.value
    const { events, txHash, txIndex, blockHash, blockNumber } = details

    const event = Events.StateChanged.New(events)

    return { isErr: false, event, events, txHash, txIndex, blockHash, blockNumber }
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
      if (ed == undefined) {
        return undefined
      }

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
      if (ed == undefined) {
        return undefined
      }

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
      if (ed == undefined) {
        return undefined
      }

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
      if (ed == undefined) {
        return undefined
      }

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
      if (ed == undefined) {
        return undefined
      }

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
      if (ed == undefined) {
        return undefined
      }

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
      if (ed == undefined) {
        return undefined
      }

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
      if (ed == undefined) {
        return undefined
      }

      return new StateChanged(ed["poolId"].toString(), ed["newState"].toString())
    }
  }
}
