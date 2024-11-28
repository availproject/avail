import { wait_for_new_era } from "."
import { SDK, Keyring, Events, BN, BondExtra, NewCommission } from "../../src/sdk"

export async function run() {
  console.log("NominationPools_Create")
  await Create.run()
  console.log("NominationPools_CreateWithPoolId")
  await CreateWithPoolId.run()
  console.log("NominationPools_Join")
  await Join.run()
  console.log("NominationPools_BondExtra")
  await BondExtra.run()
  console.log("NominationPools_Unbond")
  await Unbond.run()
  console.log("NominationPools_SetCommission")
  await SetCommission.run()
  console.log("NominationPools_SetMetadata")
  await SetMetadata.run()
  console.log("NominationPools_SetState")
  await SetState.run()
  console.log("NominationPools_SetClaimPermission")
  await SetClaimPermission.run()
  console.log("NominationPools_Nominate")
  await Nominate.run()
  console.log("NominationPools_Chill")
  await Chill.run()

  // Wait for new era
  await newEra()
  await PayoutStakers.run()

  console.log("NominationPools_WithdrawUnbonded")
  await WithdrawUnbonded.run()
  console.log("NominationPools_ClaimPayout")
  await ClaimPayout.run()
  console.log("NominationPools_ClaimPayoutOther")
  await ClaimPayoutOther.run()
  console.log("NominationPools_ClaimCommission")
  await ClaimCommission.run()
}

namespace Create {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")

    const amount = SDK.oneAvail().mul(new BN("100000")) // 100_000 Avail
    const root: string = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" // Bob
    const nominator: string = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" // Bob
    const bouncer: string = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" // Bob

    const tx = sdk.tx.nominationPools.create(amount, root, nominator, bouncer)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event1 = details.findFirstEvent(Events.NominationPools.Created)
    if (event1 != null) {
      console.log(event1)
    }

    let event2 = details.findFirstEvent(Events.NominationPools.Bonded)
    if (event2 != null) {
      console.log(event2)
    }
  }
}

namespace CreateWithPoolId {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Eve")

    const amount = SDK.oneAvail().mul(new BN("10000")) // 10_000 Avail
    const root: string = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
    const nominator: string = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
    const bouncer: string = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
    const poolId = 0

    const tx = sdk.tx.nominationPools.createWithPoolId(amount, root, nominator, bouncer, poolId)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event1 = details.findFirstEvent(Events.NominationPools.Created)
    if (event1 != null) {
      console.log(event1)
    }

    let event2 = details.findFirstEvent(Events.NominationPools.Bonded)
    if (event2 != null) {
      console.log(event2)
    }
  }
}

namespace Join {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Dave")

    const amount = SDK.oneAvail().mul(new BN("10000")) // 10_000 Avail
    const poolId = 1

    const tx = sdk.tx.nominationPools.join(amount, poolId)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.Bonded)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace BondExtra {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Dave")
    const amount = SDK.oneAvail()
    const bondExtra = { FreeBalance: amount } as BondExtra

    const tx = sdk.tx.nominationPools.bondExtra(bondExtra)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.Bonded)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace Unbond {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Dave")
    const memberAccount = account.address
    const unbondingPoints = SDK.oneAvail()

    const tx = sdk.tx.nominationPools.unbond(memberAccount, unbondingPoints)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.Unbonded)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace WithdrawUnbonded {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Dave")
    const memberAccount = account.address
    const numSlashingSpans = 0

    const tx = sdk.tx.nominationPools.withdrawUnbonded(memberAccount, numSlashingSpans)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.Withdrawn)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace SetCommission {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
    const poolId = 1
    const newCommission = { amount: 25, payee: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" }

    // TODO
    const tx = sdk.tx.nominationPools.setCommission(poolId, newCommission)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.PoolCommissionUpdated)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace SetMetadata {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
    const poolId = 1
    const metadata = "My Metadata"

    // TODO
    const tx = sdk.tx.nominationPools.setMetadata(poolId, metadata)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
  }
}

namespace SetState {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Eve")
    const poolId = 0
    const state = "Blocked"

    const tx = sdk.tx.nominationPools.setState(poolId, state)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.StateChanged)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace SetClaimPermission {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Dave")
    const permission = "PermissionlessAll"

    const tx = sdk.tx.nominationPools.setClaimPermission(permission)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
  }
}

namespace Nominate {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
    const poolId = 1
    const validators = ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"]

    const tx = sdk.tx.nominationPools.nominate(poolId, validators)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
  }
}

namespace Chill {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Eve")
    const poolId = 0

    const tx = sdk.tx.nominationPools.chill(poolId)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
  }
}

namespace ClaimPayout {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")

    const tx = sdk.tx.nominationPools.claimPayout()
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.PaidOut)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace ClaimPayoutOther {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
    const other = "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy" // Dave

    const tx = sdk.tx.nominationPools.claimPayoutOther(other)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.PaidOut)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace ClaimCommission {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
    const poolId = 1

    const tx = sdk.tx.nominationPools.claimCommission(poolId)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.NominationPools.PoolCommissionClaimed)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace PayoutStakers {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const stash = "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"

    const activeEra: any = await sdk.api.query.staking.activeEra()
    let era = activeEra.__internal__raw.index.toNumber(0)
    if (era > 0) era -= 1

    const tx = sdk.tx.staking.payoutStakers(stash, era)
    const result = await tx.executeWaitForInclusion(account)
    result._unsafeUnwrap()
  }
}

async function newEra() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const activeEra: any = await sdk.api.query.staking.activeEra()
  let era = activeEra.__internal__raw.index.toNumber(0) + 3
  console.log("Waiting for era: ", era)

  await wait_for_new_era(era)
}
