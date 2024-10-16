import { SDK, WaitFor, Keyring, BN, KeyringPair, Weight, ParsedTxResult, MultisigTimepoint } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Multisig Signatures
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const bob = new Keyring({ type: "sr25519" }).addFromUri("//Bob")
  const charlie = new Keyring({ type: "sr25519" }).addFromUri("//Charlie")

  // Create Multisig Account
  const threshold = 3
  const multisigAddress = sdk.util.generateMultisig([alice.address, bob.address, charlie.address], threshold)
  await fundMultisigAccount(sdk, alice, multisigAddress)

  // Define what action will be taken by the multisig account
  const amount = new BN(10).pow(new BN(18)) // one Avail
  const call = sdk.api.tx.balances.transferKeepAlive(multisigAddress, amount)
  // Data needed for multisig approval and execution
  const callHash = call.method.hash.toString()
  const callData = call.unwrap().toHex()
  const maxWeight = (await call.paymentInfo(alice.address)).weight

  /*
    The first signature creates and approves the multisig transaction. All the next signatures (besides the last one) should 
    use the `nextApproval` function to approve the tx. The last signature should use the `lastApproval` function to approve
    and execute the multisig tx.

    In practice it means the following:
    - If the threshold is 2 do the following:
      - firstApproval
      - lastApproval
    - If the threshold is 4 do the following:
      - firstApproval
      - nextApproval
      - nextApproval
      - lastApproval
  */

  // Create New Multisig
  const call1signatures = sdk.util.sortMultisigAddresses([bob.address, charlie.address])
  const firstResult = await firstApproval(sdk, alice, callHash, threshold, call1signatures, maxWeight)

  // Approve existing Multisig
  const timepoint: MultisigTimepoint = { height: firstResult.blockNumber, index: firstResult.txIndex }
  const call2signatures = sdk.util.sortMultisigAddresses([alice.address, charlie.address])
  const _secondResult = await nextApproval(sdk, bob, callHash, threshold, call2signatures, timepoint)

  // Execute Multisig
  const call3signatures = sdk.util.sortMultisigAddresses([alice.address, bob.address])
  const _thirdResult = await lastApproval(sdk, charlie, threshold, call3signatures, timepoint, callData, maxWeight)

  process.exit()
}

async function fundMultisigAccount(sdk: SDK, alice: KeyringPair, multisigAddress: string): Promise<string> {
  console.log("Funding multisig account...")
  const amount = new BN(10).pow(new BN(18)).mul(new BN(100)) // 100 Avail
  const result = await sdk.tx.balances.transferKeepAlive(multisigAddress, amount, WaitFor.BlockInclusion, alice)
  if (result.isErr) {
    console.log(result.reason)
    process.exit(1)
  }

  return multisigAddress
}

async function firstApproval(
  sdk: SDK,
  account: KeyringPair,
  callHash: string,
  threshold: number,
  otherSignatures: string[],
  maxWeight: Weight,
): Promise<ParsedTxResult> {
  console.log("Alice is creating a Multisig Transaction...")

  const maybeTxResult = await sdk.util.firstMultisigApproval(
    callHash,
    threshold,
    otherSignatures,
    maxWeight,
    WaitFor.BlockInclusion,
    account,
  )
  if (maybeTxResult.isErr()) {
    console.log(maybeTxResult.error)
    process.exit(1)
  }
  return maybeTxResult.value
}

async function nextApproval(
  sdk: SDK,
  account: KeyringPair,
  callHash: string,
  threshold: number,
  otherSignatures: string[],
  timepoint: MultisigTimepoint,
): Promise<ParsedTxResult> {
  console.log("Bob is approving the existing Multisig Transaction...")

  const maybeTxResult = await sdk.util.nextMultisigApproval(
    callHash,
    threshold,
    otherSignatures,
    timepoint,
    WaitFor.BlockInclusion,
    account,
  )
  if (maybeTxResult.isErr()) {
    console.log(maybeTxResult.error)
    process.exit(1)
  }
  return maybeTxResult.value
}

async function lastApproval(
  sdk: SDK,
  account: KeyringPair,
  threshold: number,
  otherSignatures: string[],
  timepoint: MultisigTimepoint,
  callData: string,
  maxWeight: Weight,
): Promise<ParsedTxResult> {
  console.log("Charlie is approving and executing the existing Multisig Transaction...")

  const maybeTxResult = await sdk.util.lastMultisigApproval(
    threshold,
    otherSignatures,
    timepoint,
    callData,
    maxWeight,
    WaitFor.BlockInclusion,
    account,
  )
  if (maybeTxResult.isErr()) {
    console.log(maybeTxResult.error)
    process.exit(1)
  }
  return maybeTxResult.value
}

main()
