import { SDK, WaitFor, Keyring } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const oldKey = "MyNewKeyAwesome1"
  const newKey = "MyNewKeyAwesome2"

  const result = await sdk.tx.dataAvailability.setApplicationKey(oldKey, newKey, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 4))

  process.exit()
}
main()
