import { SDK, WaitFor, Keyring, BN } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const memberAccount = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const unbondingPoints = SDK.oneAvail().mul(new BN(100)) // 100 Avail

  const result = await sdk.tx.nominationPools.unbond(memberAccount, unbondingPoints, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
