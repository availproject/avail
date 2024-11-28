import { SDK, BN, KeyringPair, Weight, TxResultDetails, MultisigTimepoint, utils } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())

  // Multisig Signatures
  const [alice, bob, charlie] = [SDK.alice(), SDK.bob(), SDK.charlie()]

  // Create Multisig Account
  const threshold = 3
  const multisigAddress = utils.generateMultisig([alice.address, bob.address, charlie.address], threshold)
  await fundMultisigAccount(sdk, alice, multisigAddress)

  // Define what action will be taken by the multisig account
  const amount = SDK.oneAvail()
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
  const call1signatures = utils.sortMultisigAddresses([bob.address, charlie.address])
  const firstResult = await firstApproval(sdk, alice, threshold, call1signatures, callHash, maxWeight)

  // Approve existing Multisig
  const timepoint: MultisigTimepoint = { height: firstResult.blockNumber, index: firstResult.txIndex }
  const call2signatures = utils.sortMultisigAddresses([alice.address, charlie.address])
  const _secondResult = await nextApproval(sdk, bob, threshold, call2signatures, timepoint, callHash, maxWeight)

  // Execute Multisig
  const call3signatures = utils.sortMultisigAddresses([alice.address, bob.address])
  const _thirdResult = await lastApproval(sdk, charlie, threshold, call3signatures, timepoint, callData, maxWeight)

  process.exit()
}

async function fundMultisigAccount(sdk: SDK, alice: KeyringPair, multisigAddress: string): Promise<string> {
  console.log("Funding multisig account...")
  const amount = SDK.oneAvail().mul(new BN(100)) // 100 Avail
  const tx = sdk.tx.balances.transferKeepAlive(multisigAddress, amount)
  const result = await tx.executeWaitForInclusion(alice)
  if (result.isErr()) throw Error(result.error.reason)

  return multisigAddress
}

async function firstApproval(
  sdk: SDK,
  account: KeyringPair,
  threshold: number,
  otherSignatures: string[],
  callHash: string,
  maxWeight: Weight,
): Promise<TxResultDetails> {
  console.log("Alice is creating a Multisig Transaction...")

  const tx = sdk.tx.multisig.approveAsMulti(threshold, otherSignatures, null, callHash, maxWeight)
  const result = await tx.executeWaitForInclusion(account)
  if (result.isErr()) throw Error(result.error.reason)

  return result.value
}

async function nextApproval(
  sdk: SDK,
  account: KeyringPair,
  threshold: number,
  otherSignatures: string[],
  timepoint: MultisigTimepoint,
  callHash: string,
  maxWeight: Weight,
): Promise<TxResultDetails> {
  console.log("Bob is approving the existing Multisig Transaction...")

  const tx = sdk.tx.multisig.approveAsMulti(threshold, otherSignatures, timepoint, callHash, maxWeight)
  const result = await tx.executeWaitForInclusion(account)
  if (result.isErr()) throw Error(result.error.reason)

  return result.value
}

async function lastApproval(
  sdk: SDK,
  account: KeyringPair,
  threshold: number,
  otherSignatures: string[],
  timepoint: MultisigTimepoint,
  callData: string,
  maxWeight: Weight,
): Promise<TxResultDetails> {
  console.log("Charlie is approving and executing the existing Multisig Transaction...")

  const tx = sdk.tx.multisig.asMulti(threshold, otherSignatures, timepoint, callData, maxWeight)
  const result = await tx.executeWaitForInclusion(account)
  if (result.isErr()) throw Error(result.error.reason)

  return result.value
}

main()
