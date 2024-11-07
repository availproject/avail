import { SDK, Account, WaitFor } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const alice = SDK.alice()
  const data = "My Data"
  const appId = 1

  // AppId can be passed as part of transaction options instance
  const mtx = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { app_id: appId })
  const tx = mtx._unsafeUnwrap()
  console.log(tx.appId) // 1

  // Account instance can be set to use a specific appId
  const account = new Account(sdk, alice)
  account.setAppId(appId)
  const tx2 = (await account.submitData(data))._unsafeUnwrap()
  console.log(tx2.appId) // 1

  // Creating App Id via SDK
  const mtx3 = await sdk.tx.dataAvailability.createApplicationKey("My New Key 1", WaitFor.BlockInclusion, alice)
  const tx3 = mtx3._unsafeUnwrap()
  console.log("Generated App Id: " + tx3.event.id) // Generated App Id: 10

  // Creating App Id via Account instance
  account.setAppId(null)
  const tx4 = (await account.createApplicationKey("My New Key 2"))._unsafeUnwrap()
  console.log("Generated App Id: " + tx4.event.id) // Generated App Id: 11

  process.exit()
}
main()
