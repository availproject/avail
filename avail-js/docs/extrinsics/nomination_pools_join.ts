import { SDK, WaitFor, Keyring, BN } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const amount = SDK.oneAvail().mul(new BN(10000)) // 10_000 Avail
  const poolId = 1

  const result = await sdk.tx.nominationPools.join(amount, poolId, WaitFor.BlockInclusion, account)
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
