import { EventRecord, H256 } from "@polkadot/types/interfaces/types"
import { BN, MultisigTimepoint } from ".."
import { ApiPromise } from "@polkadot/api"

export function findFirstEvent<T>(c: { decode(arg0: EventRecord): T | null }, eventRecord: EventRecord[]): T | null {
  for (const event of eventRecord) {
    const decoded_event = c.decode(event)
    if (decoded_event != null) {
      return decoded_event
    }
  }
  return null
}

export function findLastEvent<T>(c: { decode(arg0: EventRecord): T | null }, eventRecord: EventRecord[]): T | null {
  let result = null

  for (const event of eventRecord) {
    const decoded_event = c.decode(event)
    if (decoded_event != null) {
      result = decoded_event
    }
  }

  return result
}

export function findEvent<T>(c: { decode(arg0: EventRecord): T | null }, eventRecord: EventRecord[]): T[] {
  const decoded_events = []

  for (const event of eventRecord) {
    const decoded_event = c.decode(event)
    if (decoded_event != null) {
      decoded_events.push(decoded_event)
    }
  }

  return decoded_events
}

export async function fetchEvents(api: ApiPromise, blockHash: H256, txIndex?: number): Promise<EventRecord[]> {
  const apiAt = await api.at(blockHash)
  const eventRecords = (await apiAt.query.system.events()) as any as EventRecord[]
  if (txIndex != undefined) {
    return eventRecords.filter((e) => {
      e.phase.isApplyExtrinsic && e.phase.asApplyExtrinsic.toNumber() == txIndex
    })
  }

  return eventRecords
}

export namespace Balances {
  export class Transfer {
    constructor(
      public from: string,
      public to: string,
      public amount: BN,
    ) {}

    static decode(event: EventRecord): Transfer | null {
      if (event.event.section != "balances" || event.event.method != "Transfer") {
        return null
      }
      const ed: any = event.event.data

      return new Transfer(ed["from"].toString(), ed["to"].toString(), ed["amount"])
    }
  }
}

export namespace System {
  export class KilledAccount {
    constructor(public account: string) {}

    static decode(event: EventRecord): KilledAccount | null {
      if (event.event.section != "system" || event.event.method != "Killed") {
        return null
      }
      const ed: any = event.event.data

      return new KilledAccount(ed["account"].toString())
    }
  }

  export class ExtrinsicSuccess {
    constructor() {}

    static decode(event: EventRecord): ExtrinsicSuccess | null {
      if (event.event.section != "system" || event.event.method != "ExtrinsicSuccess") {
        return null
      }
      const ed: any = event.event.data

      return new ExtrinsicSuccess()
    }
  }

  export class ExtrinsicFailed {
    constructor() {}

    static decode(event: EventRecord): ExtrinsicFailed | null {
      if (event.event.section != "system" || event.event.method != "ExtrinsicFailed") {
        return null
      }
      const ed: any = event.event.data

      return new ExtrinsicFailed()
    }
  }
}

export namespace DataAvailability {
  export class DataSubmitted {
    constructor(
      public who: string,
      public dataHash: string,
    ) {}

    static decode(event: EventRecord): DataSubmitted | null {
      if (event.event.section != "dataAvailability" || event.event.method != "DataSubmitted") {
        return null
      }
      const ed: any = event.event.data

      return new DataSubmitted(ed["who"].toString(), ed["dataHash"].toString())
    }
  }

  export class ApplicationKeyCreated {
    constructor(
      public key: string,
      public owner: string,
      public id: number,
    ) {}

    static decode(event: EventRecord): ApplicationKeyCreated | null {
      if (event.event.section != "dataAvailability" || event.event.method != "ApplicationKeyCreated") {
        return null
      }
      const ed: any = event.event.data

      return new ApplicationKeyCreated(ed["key"].toString(), ed["owner"].toString(), parseInt(ed["id"].toString()))
    }
  }
}

export namespace Multisig {
  export class MultisigApproval {
    constructor(
      public approving: string,
      public timepoint: MultisigTimepoint,
      public multisig: string,
      public callHash: string,
    ) {}

    static decode(event: EventRecord): MultisigApproval | null {
      if (event.event.section != "multisig" || event.event.method != "MultisigApproval") {
        return null
      }
      const ed: any = event.event.data

      const timepoint = {
        height: parseInt(ed["timepoint"].height.toString()),
        index: parseInt(ed["timepoint"].index.toString()),
      }

      return new MultisigApproval(
        ed["approving"].toString(),
        timepoint,
        ed["multisig"].toString(),
        ed["callHash"].toString(),
      )
    }
  }

  export class MultisigExecuted {
    constructor(
      public approving: string,
      public timepoint: MultisigTimepoint,
      public multisig: string,
      public callHash: string,
      public result: string,
    ) {}

    static decode(event: EventRecord): MultisigExecuted | null {
      if (event.event.section != "multisig" || event.event.method != "MultisigExecuted") {
        return null
      }
      const ed: any = event.event.data

      const timepoint = {
        height: parseInt(ed["timepoint"].height.toString()),
        index: parseInt(ed["timepoint"].index.toString()),
      }

      return new MultisigExecuted(
        ed["approving"].toString(),
        timepoint,
        ed["multisig"].toString(),
        ed["callHash"].toString(),
        ed["result"].toString(),
      )
    }
  }

  export class NewMultisig {
    constructor(
      public approving: string,
      public multisig: string,
      public callHash: string,
    ) {}

    static decode(event: EventRecord): NewMultisig | null {
      if (event.event.section != "multisig" || event.event.method != "NewMultisig") {
        return null
      }
      const ed: any = event.event.data

      return new NewMultisig(ed["approving"].toString(), ed["multisig"].toString(), ed["callHash"].toString())
    }
  }
}

export namespace NominationPools {
  export class Bonded {
    constructor(
      public member: string,
      public poolId: string,
      public bonded: string,
      public joined: string,
    ) {}

    static decode(event: EventRecord): Bonded | null {
      if (event.event.section != "nominationPools" || event.event.method != "Bonded") {
        return null
      }
      const ed: any = event.event.data

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

    static decode(event: EventRecord): Created | null {
      if (event.event.section != "nominationPools" || event.event.method != "Created") {
        return null
      }
      const ed: any = event.event.data

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

    static decode(event: EventRecord): Unbonded | null {
      if (event.event.section != "nominationPools" || event.event.method != "Unbonded") {
        return null
      }
      const ed: any = event.event.data

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

    static decode(event: EventRecord): PoolCommissionClaimed | null {
      if (event.event.section != "nominationPools" || event.event.method != "PoolCommissionClaimed") {
        return null
      }
      const ed: any = event.event.data

      return new PoolCommissionClaimed(ed["poolId"].toString(), ed["commission"].toString())
    }
  }

  export class PaidOut {
    constructor(
      public member: string,
      public poolId: string,
      public payout: string,
    ) {}

    static decode(event: EventRecord): PaidOut | null {
      if (event.event.section != "nominationPools" || event.event.method != "PaidOut") {
        return null
      }
      const ed: any = event.event.data

      return new PaidOut(ed["member"].toString(), ed["poolId"].toString(), ed["payout"].toString())
    }
  }

  export class PoolCommissionUpdated {
    constructor(
      public poolId: string,
      public current: string,
    ) {}

    static decode(event: EventRecord): PoolCommissionUpdated | null {
      if (event.event.section != "nominationPools" || event.event.method != "PoolCommissionUpdated") {
        return null
      }
      const ed: any = event.event.data

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

    static decode(event: EventRecord): Withdrawn | null {
      if (event.event.section != "nominationPools" || event.event.method != "Withdrawn") {
        return null
      }
      const ed: any = event.event.data

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

    static decode(event: EventRecord): StateChanged | null {
      if (event.event.section != "nominationPools" || event.event.method != "StateChanged") {
        return null
      }
      const ed: any = event.event.data

      return new StateChanged(ed["poolId"].toString(), ed["newState"].toString())
    }
  }
}

export namespace Staking {
  export class Bonded {
    constructor(
      public stash: string,
      public amount: string,
    ) {}

    static decode(event: EventRecord): Bonded | null {
      if (event.event.section != "staking" || event.event.method != "Bonded") {
        return null
      }
      const ed: any = event.event.data

      const amountString = ed["amount"].toString()
      const amount = new BN(amountString).div(new BN(10).pow(new BN(18))).toString()

      return new Bonded(ed["stash"].toString(), amount)
    }
  }

  export class Chilled {
    constructor(public stash: string) {}

    static decode(event: EventRecord): Chilled | null {
      if (event.event.section != "staking" || event.event.method != "Chilled") {
        return null
      }
      const ed: any = event.event.data

      return new Chilled(ed["stash"].toString())
    }
  }

  export class Unbonded {
    constructor(
      public stash: string,
      public amount: string,
    ) {}

    static decode(event: EventRecord): Unbonded | null {
      if (event.event.section != "staking" || event.event.method != "Unbonded") {
        return null
      }
      const ed: any = event.event.data

      return new Unbonded(ed["stash"].toString(), ed["amount"].toString())
    }
  }

  export class ValidatorPrefsSet {
    constructor(
      public stash: string,
      public commission: string,
      public blocked: string,
    ) {}

    static decode(event: EventRecord): ValidatorPrefsSet | null {
      if (event.event.section != "staking" || event.event.method != "ValidatorPrefsSet") {
        return null
      }
      const ed: any = event.event.data

      return new ValidatorPrefsSet(
        ed["stash"].toString(),
        ed["prefs"]["commission"].toString(),
        ed["prefs"]["blocked"].toString(),
      )
    }
  }
}

export namespace Utility {
  export class BatchCompleted {
    constructor() {}

    static decode(event: EventRecord): BatchCompleted | null {
      if (event.event.section != "utility" || event.event.method != "BatchCompleted") {
        return null
      }

      return new BatchCompleted()
    }
  }

  export class BatchCompletedWithErrors {
    constructor() {}

    static decode(event: EventRecord): BatchCompletedWithErrors | null {
      if (event.event.section != "utility" || event.event.method != "BatchCompletedWithErrors") {
        return null
      }

      return new BatchCompletedWithErrors()
    }
  }

  export class ItemFailed {
    constructor() {}

    static decode(event: EventRecord): ItemFailed | null {
      if (event.event.section != "utility" || event.event.method != "ItemFailed") {
        return null
      }
      const ed: any = event.event.data

      return new ItemFailed()
    }
  }

  export class ItemCompleted {
    constructor() {}

    static decode(event: EventRecord): ItemCompleted | null {
      if (event.event.section != "utility" || event.event.method != "ItemCompleted") {
        return null
      }
      const ed: any = event.event.data

      return new ItemCompleted()
    }
  }

  export class BatchInterrupted {
    constructor() {}

    static decode(event: EventRecord): BatchInterrupted | null {
      if (event.event.section != "utility" || event.event.method != "BatchInterrupted") {
        return null
      }
      const ed: any = event.event.data

      return new BatchInterrupted()
    }
  }
}

/* export namespace TransactionData {
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
 */

/* export namespace TransactionData {
  export class SubmitData {
    constructor(public data: string) {}

    static async New(api: ApiPromise, txHash: H256, blockHash: H256): Promise<Result<SubmitData, string>> {
      const block = await api.rpc.chain.getBlock(blockHash)
      const tx = block.block.extrinsics.find((tx) => tx.hash.toHex() == txHash.toHex())
      if (tx == undefined) return err("Failed to find submit data transaction.")

      // Data retrieved from the extrinsic data
      let dataHex = tx.method.args.map((a) => a.toString()).join(", ")
      if (dataHex.startsWith("0x")) {
        dataHex = dataHex.slice(2)
      }

      return ok(new SubmitData(dataHex))
    }
  }
}
 */
