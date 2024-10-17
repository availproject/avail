import { SDK, WaitFor, Keyring, TransactionOptions } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const data = "My Awesome Data"

  const options: TransactionOptions = { app_id: 1 }
  const result = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, account, options)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
