import { SDK, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const key = "MyAwesomeKey"

  const txHash = await sdk.tx.dataAvailability.createApplicationKeyNoWait(key, account)

  console.log(txHash)
  process.exit()
}
main()
