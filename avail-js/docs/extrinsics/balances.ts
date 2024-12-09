import { SDK, Keyring, Events, BN } from "../../src/sdk"

export async function run() {
  console.log("Balances_TransferAll")
  await TransferAll.run()
  await TransferAll.clean()
  console.log("Balances_TransferAllowDeath")
  await TransferAllowDeath.run()
  console.log("Balances_TransferKeepAlive")
  await TransferKeepAlive.run()
}

namespace TransferAll {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
    const keepAlive = false

    const tx = sdk.tx.balances.transferAll(dest, keepAlive)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event1 = details.findFirstEvent(Events.Balances.Transfer)
    if (event1 != null) {
      console.log(event1)
    }

    let event2 = details.findFirstEvent(Events.System.KilledAccount)
    if (event2 != null) {
      console.log(event2)
    }
  }

  export async function clean() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Eve")
    const dest = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
    let value = SDK.oneAvail().mul(new BN("900000"))

    const tx = sdk.tx.balances.transferKeepAlive(dest, value)
    const result = await tx.executeWaitForInclusion(account)
    result._unsafeUnwrap()
  }
}

namespace TransferAllowDeath {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
    const value = SDK.oneAvail()

    const tx = sdk.tx.balances.transferAllowDeath(dest, value)
    const result = await tx.executeWaitForInclusion(account, undefined)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event1 = details.findFirstEvent(Events.Balances.Transfer)
    if (event1 != null) {
      console.log(event1)
    }

    let event2 = details.findFirstEvent(Events.System.KilledAccount)
    if (event2 != null) {
      console.log(event2)
    }
  }
}

namespace TransferKeepAlive {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
    const value = SDK.oneAvail()

    const tx = sdk.tx.balances.transferKeepAlive(dest, value)
    const result = await tx.executeWaitForInclusion(account, undefined)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.Balances.Transfer)
    if (event != null) {
      console.log(event)
    }
  }
}
