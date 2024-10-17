import { SDK, WaitFor, Keyring } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Multisig Signatures
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const bobAddress = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"

  // Create Multisig Account
  const threshold = 2
  const multisigAddress = sdk.util.generateMultisig([alice.address, bobAddress], threshold)

  // Define what action will be taken by the multisig account
  const amount = SDK.oneAvail()
  const call = sdk.api.tx.balances.transferKeepAlive(multisigAddress, amount)
  // Data needed for multisig approval and execution
  const callHash = call.method.hash.toString()
  const maxWeight = (await call.paymentInfo(alice.address)).weight

  // Create New Multisig
  console.log("Alice is creating a Multisig Transaction...")
  const call1signatures = sdk.util.sortMultisigAddresses([bobAddress])
  const result = await sdk.tx.multisig.approveAsMulti(
    threshold,
    call1signatures,
    null,
    callHash,
    maxWeight,
    WaitFor.BlockInclusion,
    alice,
  )
  if (result.isErr()) {
    console.log(result.error)
    process.exit(1)
  }

  console.log(JSON.stringify(result.value, null, 2))
  process.exit()
}
main()
