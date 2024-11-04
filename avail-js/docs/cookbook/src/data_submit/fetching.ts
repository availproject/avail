import { SDK, Keyring, Account, Block, sdkBlock, WaitFor } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const api = sdk.api
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

  // Submitting data that we will query later
  const data = "My Data"
  const tx = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { app_id: 1 })
  if (tx.isErr == true) throw Error() // We expect that the call will succeed
  const { blockHash, txHash, txIndex } = tx

  // Data

  // Fetching all data-submission data from block via Block instance
  const block = await Block.New(api, blockHash)
  const data1 = block.submitDataAll()
  data1.forEach((data) => console.log(data.hexData)) // `4d792044617461`,...

  // Fetching all data-submission data by transaction hash from block via Block instance
  const data2 = block.submitDataByHash(txHash)
  console.log(data2._unsafeUnwrap().hexData) // `4d792044617461`

  // Fetching all data-submission data by transaction index from block via Block instance
  const data3 = block.submitDataByIndex(txIndex)
  console.log(data3._unsafeUnwrap().hexData) // `4d792044617461`

  // Fetching all data-submission data by signer(author) from block via Block instance
  const data4 = block.submitDataBySigner(alice.address)
  data4.forEach((data) => console.log(data.hexData)) // `4d792044617461`,...

  // Fetching all data-submission data from block via free function
  const signedBlock = await api.rpc.chain.getBlock(blockHash)
  const data5 = sdkBlock.submitDataAll(signedBlock)
  data5.forEach((data) => console.log(data.hexData)) // `4d792044617461`,...

  // Fetching all data-submission data by transaction hash from block via free function
  const data6 = sdkBlock.submitDataByHash(signedBlock, txHash)
  console.log(data6._unsafeUnwrap().hexData) // `4d792044617461`

  // Fetching all data-submission data by transaction index from block via free function
  const data7 = sdkBlock.submitDataByIndex(signedBlock, txIndex)
  console.log(data7._unsafeUnwrap().hexData) // `4d792044617461`

  // Fetching all data-submission data by signer(author) from block via free function
  const data8 = sdkBlock.submitDataBySigner(signedBlock, alice.address)
  data8.forEach((data) => console.log(data.hexData)) // `4d792044617461`,...

  // Manually fetching data-submission data
  const data9 = signedBlock.block.extrinsics.flatMap((tx, index) =>
    sdkBlock
      .extractDataSubmissionFromTx(tx, index)
      .map((data) => [data])
      .unwrapOr([]),
  )
  data9.forEach((data) => console.log(data.hexData)) // `4d792044617461`,...

  // Transactions

  // Fetching all data-submission transaction from block via Block instance
  const tx1 = block.transactionAll().flatMap((tx) => {
    if (!sdkBlock.isTransactionDataSubmission(tx)) return []
    return [tx]
  })
  tx1.forEach((tx) => console.log(`Section: ${tx.method.section} Method: ${tx.method.method}`)) // `Section: dataAvailability Method: submitData`,...

  // Manually fetching data-submission transactions
  const tx2 = signedBlock.block.extrinsics.flatMap((tx) => {
    if (!sdkBlock.isTransactionDataSubmission(tx)) return []
    return [tx]
  })
  tx2.forEach((tx) => console.log(`Section: ${tx.method.section} Method: ${tx.method.method}`)) // `Section: dataAvailability Method: submitData`,...

  // Extracting only the data from data-submission transaction
  tx2.forEach((tx) => console.log(sdkBlock.extractDataSubmissionDataFromTx(tx)._unsafeUnwrap())) // `4d792044617461`,...

  // Extracting only the data from a new data-submission transaction
  const tx3 = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { app_id: 1 })
  if (tx3.isErr) throw Error(tx3.reason)
  console.log(tx3.txData.data) // `4d792044617461`

  // Extracting only the data from a new data-submission transaction via Account instance
  const account = new Account(sdk, alice)
  account.setAppId(1)
  const tx4 = await account.submitData(data)
  if (tx4.isErr) throw Error(tx4.reason)
  console.log(tx4.txData.data) // `4d792044617461`

  process.exit()
}
main()
