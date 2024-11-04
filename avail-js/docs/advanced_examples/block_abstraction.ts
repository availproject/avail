import { SDK, Block, sdkBlock, Keyring, Account } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const account = new Account(sdk, new Keyring({ type: "sr25519" }).addFromUri("//Alice"))

  // Submitting data that we will query later
  const tx = await account.submitData("aabbcc")
  if (tx.isErr) {
    process.exit(0)
  }
  const { blockHash, txHash, txIndex } = tx
  console.log("Reference hex value: " + tx.txData.data)

  /*
    Data Submissions from a block can be fetched in two ways:
    - Using the Block object
    - Using helper functions
  */

  /*
    The block abstraction object has helper functions to get data from
    submit data transactions.
  */
  const block = await Block.New(sdk.api, blockHash)
  const r1 = block.getSubmitDataAll()
  console.log(r1)
  const r2 = block.getSubmitDataHash(txHash)
  if (r2.isOk()) {
    console.log(r2.value)
  }
  const r3 = block.getSubmitDataIndex(txIndex)
  if (r3.isOk()) {
    console.log(r3.value)
  }

  /// The Signed block can be accessed if necessary.
  const _signedBlock = block.signedBlock

  /*
    The same interface can be found as free functions inside
    sdkBlock namespace. 

    Block can as well be instated if `signedBlock` is available. 
    const _block = new Block(signedBlock)
  */
  const signedBlock = await sdk.api.rpc.chain.getBlock(blockHash)
  const r4 = sdkBlock.getSubmitDataAll(signedBlock)
  console.log(r4)
  const r5 = sdkBlock.getSubmitDataHash(signedBlock, txHash)
  if (r5.isOk()) {
    console.log(r5.value)
  }
  const r6 = sdkBlock.getSubmitDataIndex(signedBlock, txIndex)
  if (r6.isErr()) {
    process.exit(1)
  }
  const dataSubmission = r6.value

  /*
    DataSubmission structure:
    {
      "txHash": "0x2965fa2a2df6f818f0725be2b92b0dd399f1cf1e1620e1477fcc2b3e9ba9f491",
      "txIndex": 1,
      "hexData": "616162626363"
      "txSigner": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    }
  */
  console.log("Tx Hash: " + dataSubmission.txHash.toString())
  console.log("Tx Index: " + dataSubmission.txIndex)
  console.log("Hex data: " + dataSubmission.hexData)
  console.log("Tx Signer: " + dataSubmission.txSigner)

  /*
    There is a helper function to get the Ascii version of the hex data
  */
  console.log("Ascii data: " + dataSubmission.toAscii())

  process.exit()
}
main()
