import { SDK, WaitFor, Keyring, ClaimPermission } from "../../src"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Input
  const account = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const permission: ClaimPermission = "PermissionlessAll"

  const result = await sdk.tx.nominationPools.setClaimPermission(permission, WaitFor.BlockInclusion, account)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  console.log(JSON.stringify(result, null, 2))
  process.exit()
}
main()
