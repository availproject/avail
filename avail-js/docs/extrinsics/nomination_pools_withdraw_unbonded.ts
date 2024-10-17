import { SDK, WaitFor, Keyring } from "../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const memberAccount = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice
  const numSlashingSpans = 0

  const result = await sdk.tx.nominationPools.withdrawUnbonded(
    memberAccount,
    numSlashingSpans,
    WaitFor.BlockInclusion,
    account,
  )
  if (result.isErr()) {
    console.log(result.error.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
