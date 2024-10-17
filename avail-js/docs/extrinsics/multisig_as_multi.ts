import { SDK, WaitFor, Keyring, MultisigTimepoint } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Multisig Signatures
  const bob = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const aliceAddress = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

  // Create Multisig Account
  const threshold = 2
  const multisigAddress = sdk.util.generateMultisig([aliceAddress, bob.address], threshold)

  // Define what action will be taken by the multisig account
  const amount = SDK.oneAvail()
  const call = sdk.api.tx.balances.transferKeepAlive(multisigAddress, amount)
  // Data needed for multisig approval and execution
  const callData = call.unwrap().toHex()
  const maxWeight = (await call.paymentInfo(aliceAddress)).weight
  const timepoint: MultisigTimepoint = { height: 4, index: 1 }

  // Approving and executing Multisig transaction
  console.log("Bob is approving and executing the existing Multisig Transaction...")
  const call2signatures = sdk.util.sortMultisigAddresses([aliceAddress])
  const secondResult = await sdk.tx.multisig.asMulti(
    threshold,
    call2signatures,
    timepoint,
    callData,
    maxWeight,
    WaitFor.BlockInclusion,
    bob,
  )
  if (secondResult.isErr()) {
    console.log(secondResult.error)
    process.exit(1)
  }

  console.log(JSON.stringify(secondResult.value, null, 2))
  process.exit()
}
main()
