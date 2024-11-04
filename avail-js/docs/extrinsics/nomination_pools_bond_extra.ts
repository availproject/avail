import { SDK, WaitFor, Keyring, BondExtra, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const amount = SDK.oneAvail().mul(new BN(10000)) // 10_000 Avail
  const bondExtra = { FreeBalance: amount } as BondExtra

  const result = await sdk.tx.nominationPools.bondExtra(bondExtra, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
