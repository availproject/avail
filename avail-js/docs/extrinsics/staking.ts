import { SDK, Keyring, Events, BN } from "../../src/sdk"
import { wait_for_new_era } from "./index"

export async function run() {
  console.log("Staking_Bond")
  await Bond.run()
  console.log("Staking_BondExtra")
  await BondExtra.run()
  console.log("Staking_Nominate")
  await Nominate.run()
  console.log("Staking_Chill")
  await Chill.run()
  console.log("Staking_ChillOther")
  await ChillOther.prepare()
  await ChillOther.run()
  console.log("Staking_Unbond")
  await Unbond.run()
  console.log("Staking_Validate")
  await Validate.run()
  await Validate.clean()

  await wait_for_new_era()

  console.log("Staking_PayoutStakers")
  await PayoutStakers.run()
}

namespace Bond {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const value = SDK.oneAvail().mul(new BN("100000"))
    const payee = "Staked"

    const tx = sdk.tx.staking.bond(value, payee)
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.Staking.Bonded)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace BondExtra {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const value = SDK.oneAvail()

    const tx = sdk.tx.staking.bondExtra(value)
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.Staking.Bonded)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace Nominate {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const targets = ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"]

    const tx = sdk.tx.staking.nominate(targets)
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
  }
}

namespace Chill {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

    const tx = sdk.tx.staking.chill()
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.Staking.Chilled)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace ChillOther {
  export async function prepare() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const targets = ["5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY"]

    const tx = sdk.tx.staking.nominate(targets)
    const result = await tx.execute_wait_for_inclusion(account)
    result._unsafeUnwrap()
  }

  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const stash = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

    const tx = sdk.tx.staking.chillOther(stash)
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.Staking.Chilled)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace Unbond {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const value = SDK.oneAvail()

    const tx = sdk.tx.staking.unbond(value)
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.Staking.Unbonded)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace Validate {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const commission = 50
    const blocked = false

    const tx = sdk.tx.staking.validate(commission, blocked)
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.Staking.ValidatorPrefsSet)
    if (event != null) {
      console.log(event)
    }
  }

  export async function clean() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

    const tx = sdk.tx.staking.chill()
    const result = await tx.execute_wait_for_inclusion(account)
    result._unsafeUnwrap()
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
    const result = await tx.execute_wait_for_inclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
  }
}
