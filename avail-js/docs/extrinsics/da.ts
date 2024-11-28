import { SDK, Keyring, Events } from "../../src/sdk"

export async function run() {
  console.log("DA_SubmitData")
  await SubmitData.run()
  console.log("DA_CreateApplicationKey")
  await CreateApplicationKey.run()
}

namespace SubmitData {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const data = "My Data"
    const options = { app_id: 1 }

    const tx = sdk.tx.dataAvailability.submitData(data)
    const result = await tx.executeWaitForInclusion(account, options)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.DataAvailability.DataSubmitted)
    if (event != null) {
      console.log(event)
    }
  }
}

namespace CreateApplicationKey {
  export async function run() {
    const sdk = await SDK.New(SDK.localEndpoint())

    const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
    const key = "My Key"

    const tx = sdk.tx.dataAvailability.createApplicationKey(key)
    const result = await tx.executeWaitForInclusion(account)
    if (result.isErr()) throw Error(result.error.reason)
    const details = result.value

    details.printDebug()
    let event = details.findFirstEvent(Events.DataAvailability.ApplicationKeyCreated)
    if (event != null) {
      console.log(event)
    }
  }
}
