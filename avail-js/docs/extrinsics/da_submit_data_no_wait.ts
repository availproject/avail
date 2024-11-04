import { SDK, Keyring, TransactionOptions } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const data = "My Awesome Data"

  const options: TransactionOptions = { app_id: 1 }
  const txHash = await sdk.tx.dataAvailability.submitDataNoWait(data, account, options)

  console.log(JSON.stringify(txHash, null, 2))
  process.exit()
}
main()
