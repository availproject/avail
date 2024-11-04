import { SDK, Keyring, Account, WaitFor } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const data = "My Data"
  const appId = 1

  // AppId can be passed as part of transaction options instance
  const tx = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { app_id: appId })
  if (tx.isErr == true) throw Error() // We expect that the call will succeed
  console.log(tx.appId) // 1

  // Account instance can be set to use a specific appId
  const account = new Account(sdk, alice)
  account.setAppId(appId)
  const tx2 = await account.submitData(data)
  if (tx2.isErr == true) throw Error() // We expect that the call will succeed
  console.log(tx.appId) // 1

  // Creating App Id via SDK
  const tx3 = await sdk.tx.dataAvailability.createApplicationKey("My New Key 1", WaitFor.BlockInclusion, alice)
  if (tx3.isErr) throw Error(tx3.reason)
  console.log("Generated App Id: " + tx3.event.id) // Generated App Id: 10

  // Creating App Id via Account instance
  account.setAppId(null)
  const tx4 = await account.createApplicationKey("My New Key 2")
  if (tx4.isErr) throw Error(tx4.reason)
  console.log("Generated App Id: " + tx4.event.id) // Generated App Id: 11

  process.exit()
}
main()
