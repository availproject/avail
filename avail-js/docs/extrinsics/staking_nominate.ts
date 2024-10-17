import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const targets = [
    "5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY", // Alice Stash
    "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", // Bob
  ]

  const result = await sdk.tx.staking.nominate(targets, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
