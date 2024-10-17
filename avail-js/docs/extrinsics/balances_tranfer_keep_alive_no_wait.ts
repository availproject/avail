import { SDK, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const dest = "5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw" // Eve
  const amount = SDK.oneAvail()

  const txHash = await sdk.tx.balances.transferKeepAliveNoWait(dest, amount, account)

  console.log(txHash)
  process.exit()
}
main()
