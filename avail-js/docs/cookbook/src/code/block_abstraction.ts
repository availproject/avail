import { SDK, Block, sdkBlock, Keyring, Account, sdkUtil } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  // Submitting data that we will query later
  const alice = new Account(sdk, new Keyring({ type: "sr25519" }).addFromUri("//Alice"))
  const tx = await alice.submitData("My Data")
  if (tx.isErr) {
    process.exit(0)
  }
  const { blockHash, txHash, txIndex } = tx
  console.log("Reference hex value: " + tx.txData.data)

  // Instantiate Block object
  const block = await Block.New(sdk.api, blockHash)
  // Block is just an wrapper over a SignedBlock object.
  const _signedBlock = block.signedBlock

  // Transaction via SDK

  // Getting transaction count
  const txCount1 = block.transactionCount()
  console.log("Tx Count: " + txCount1)

  // Getting transaction by transaction index
  const maybeTx1 = block.transactionByIndex(txIndex)
  if (maybeTx1.isErr()) {
    console.log(maybeTx1.error)
    process.exit(1)
  }
  const tx1 = maybeTx1.value // GenericExtrinsic
  console.log(tx1.hash.toHex()) // `0x2fc9abb314fb77d3fce4bfa019ca1562c23b796c975da38f5779be262ecbd4a8`

  // Getting transaction by hash returns multiple results because transaction hashes are not unique
  const txs2 = block.transactionByHash(txHash) // GenericExtrinsic[]
  console.log(txs2.length)

  // Getting transaction by transaction signer(author)
  const txs3 = block.transactionBySigner(alice.address()) // GenericExtrinsic[]
  console.log(txs3.length)

  // Transaction via free functions

  // Getting transaction count
  const signedBlock = await sdk.api.rpc.chain.getBlock()
  const txCount2 = sdkBlock.transactionCount(signedBlock)
  console.log("Tx Count: " + txCount2)

  // Getting transaction by transaction index
  const maybeTx4 = sdkBlock.transactionByIndex(signedBlock, txIndex)
  if (maybeTx4.isErr()) {
    console.log(maybeTx4.error)
    process.exit(1)
  }
  const tx4 = maybeTx4.value // GenericExtrinsic
  console.log(tx4.hash.toHex()) // `0x2fc9abb314fb77d3fce4bfa019ca1562c23b796c975da38f5779be262ecbd4a8`

  // Getting transaction by hash returns multiple results because transaction hashes are not unique
  const txs5 = sdkBlock.transactionByHash(signedBlock, txHash) // GenericExtrinsic[]
  console.log(txs5.length)

  // Getting transaction by transaction signer(author)
  const txs6 = sdkBlock.transactionBySigner(signedBlock, alice.address()) // GenericExtrinsic[]
  console.log(txs6.length)

  // Data Submission via SDK

  // Getting all data submission transaction data
  const dss1 = block.submitDataAll() // DataSubmission[]
  console.log(dss1.length)

  // Getting data submission transaction data by transaction index
  const maybeDs2 = block.submitDataByIndex(txIndex)
  if (maybeDs2.isErr()) {
    console.log(maybeDs2.error)
    process.exit(1)
  }
  const ds2 = maybeDs2.value // DataSubmission
  console.log(ds2.hexData)

  // Getting data submission transaction data by transaction hash. In most cases transaction hash for data submission transaction is unique inside a block
  const maybeDs3 = block.submitDataByHash(txHash)
  if (maybeDs3.isErr()) {
    console.log(maybeDs3.error)
    process.exit(1)
  }
  const ds3 = maybeDs3.value // DataSubmission
  console.log(ds3.hexData)

  // Getting data submission transaction data by transaction signer(author)
  const ds4 = block.submitDataBySigner(alice.address()) // DataSubmission[]
  console.log(ds4.length)

  // Data Submission via free functions

  // Getting all data submission transaction data
  const dss5 = sdkBlock.submitDataAll(signedBlock) // DataSubmission[]
  console.log(dss5.length)

  // Getting data submission transaction data by transaction index
  const maybeDs6 = sdkBlock.submitDataByIndex(signedBlock, txIndex)
  if (maybeDs6.isErr()) {
    console.log(maybeDs6.error)
    process.exit(1)
  }
  const ds6 = maybeDs6.value // DataSubmission
  console.log(ds6.hexData)

  // Getting data submission transaction data by transaction hash. In most cases transaction hash for data submission transaction is unique inside a block
  const maybeDs7 = sdkBlock.submitDataByHash(signedBlock, txHash)
  if (maybeDs7.isErr()) {
    console.log(maybeDs7.error)
    process.exit(1)
  }
  const ds7 = maybeDs7.value // DataSubmission
  console.log(ds7.hexData)

  // Getting data submission transaction data by transaction signer(author)
  const ds8 = sdkBlock.submitDataBySigner(signedBlock, alice.address()) // DataSubmission[]
  console.log(ds8.length)

  const maybeDs = block.submitDataByIndex(txIndex)
  if (maybeDs.isErr()) {
    process.exit(1)
  }
  const dataSubmission = maybeDs.value // DataSubmission
  /*
    DataSubmission structure:
    {
      "txHash": "0x2965fa2a2df6f818f0725be2b92b0dd399f1cf1e1620e1477fcc2b3e9ba9f491",
      "txIndex": 1,
      "hexData": "616162626363"
      "txSigner": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
    }
  */
  console.log("Tx Hash: " + dataSubmission.txHash.toHex())
  console.log("Tx Index: " + dataSubmission.txIndex)
  console.log("Hex data: " + dataSubmission.hexData)
  console.log("Tx Signer: " + dataSubmission.txSigner)

  // There is a helper function to get the Ascii version of the hex data
  console.log("Ascii data: " + dataSubmission.toAscii())

  process.exit()
}
main()
