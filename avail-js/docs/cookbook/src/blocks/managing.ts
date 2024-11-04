import { SDK, Block, sdkBlock, Keyring, Account } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const api = sdk.api

  // Submitting data that we will query later
  const alice = new Account(sdk, new Keyring({ type: "sr25519" }).addFromUri("//Alice"))
  const tx = await alice.submitData("My Data")
  if (tx.isErr == true) throw Error() // We expect that the call will succeed
  const { blockHash, txHash, txIndex } = tx
  console.log("Reference hex value: " + tx.txData.data)

  // SDK

  // Getting transaction count via Block instance
  const block = await Block.New(api, blockHash)
  console.log("Tx Count: " + block.transactionCount()) // Tx Count: 3

  // Getting all transactions via Block instance
  block.transactionAll().forEach((tx) => console.log(tx.hash.toHex())) // `0x7fa15c263e2898978633dc38cfee0fe2697a5f9d46300323a8b0e6053c1be320`,...

  // Getting transaction by transaction hash via Block instance
  block.transactionByHash(txHash).forEach((tx) => console.log(tx.hash.toHex())) // Transaction hashes are not unique. `0x77bd8ff000693150624c97337ff734bf1d76be042d61f91881e59207fee1c5eb`,...

  // Getting transaction by transaction index via Block instance
  const tx1 = block.transactionByIndex(txIndex)
  if (tx1.isErr()) throw Error(tx1.error)
  console.log(tx1.value.hash.toHex()) // `0x77bd8ff000693150624c97337ff734bf1d76be042d61f91881e59207fee1c5eb`

  // Getting transaction by transaction signer(author) via Block instance
  block.transactionBySigner(alice.address()).forEach((tx) => console.log(tx.hash.toHex())) // `0x77bd8ff000693150624c97337ff734bf1d76be042d61f91881e59207fee1c5eb`,...

  // Getting data-submission transaction count via Block instance
  console.log("Data Submission Count: " + block.submitDataCount()) // Data Submission Count: 1

  // Getting all data-submission instances via Block instance
  block.submitDataAll().forEach((sd) => console.log(sd.hexData)) // `4d792044617461`,...

  // Getting data-submission data by transaction hash via Block instance
  const sd1 = block.submitDataByHash(txHash)
  if (sd1.isErr()) throw Error(sd1.error)
  console.log(sd1.value.hexData) // `4d792044617461`

  // Getting data-submission data by transaction index via Block instance
  const sd2 = block.submitDataByIndex(txIndex)
  if (sd2.isErr()) throw Error(sd2.error)
  console.log(sd2.value.hexData) // `4d792044617461`

  // Getting data-submission data by transaction signer(author) via Block instance
  block.submitDataBySigner(alice.address()).forEach((sd) => console.log(sd.hexData)) // `4d792044617461`,...

  // Free Functions

  // Getting transaction count via free function
  const signedBlock = await api.rpc.chain.getBlock(blockHash)
  console.log("Tx Count: " + sdkBlock.transactionCount(signedBlock)) // Tx Count: 3

  // Getting all transactions via free function
  sdkBlock.transactionAll(signedBlock).forEach((tx) => console.log(tx.hash.toHex())) // `0x7fa15c263e2898978633dc38cfee0fe2697a5f9d46300323a8b0e6053c1be320`,.

  // Getting transaction by transaction hash via free function
  sdkBlock.transactionByHash(signedBlock, txHash).forEach((tx) => console.log(tx.hash.toHex())) // Transaction hashes are not unique. `0x77bd8ff000693150624c97337ff734bf1d76be042d61f91881e59207fee1c5eb`,...

  // Getting transaction by transaction index via free function
  const tx2 = sdkBlock.transactionByIndex(signedBlock, txIndex)
  if (tx2.isErr()) throw Error(tx2.error)
  console.log(tx1.value.hash.toHex()) // `0x77bd8ff000693150624c97337ff734bf1d76be042d61f91881e59207fee1c5eb`

  // Getting transaction by transaction signer(author) via free function
  sdkBlock.transactionBySigner(signedBlock, alice.address()).forEach((tx) => console.log(tx.hash.toHex())) // `0x77bd8ff000693150624c97337ff734bf1d76be042d61f91881e59207fee1c5eb`,...

  // Getting data-submission transaction count via free function
  console.log("Data Submission Count: " + sdkBlock.submitDataCount(signedBlock)) // Data Submission Count: 1

  // Getting all data-submission instances via free function
  sdkBlock.submitDataAll(signedBlock).forEach((sd) => console.log(sd.hexData)) // `4d792044617461`,...

  // Getting data-submission data by transaction hash via free function
  const sd3 = sdkBlock.submitDataByHash(signedBlock, txHash)
  if (sd3.isErr()) throw Error(sd3.error)
  console.log(sd3.value.hexData) // `4d792044617461`

  // Getting data-submission data by transaction index via free function
  const sd4 = sdkBlock.submitDataByIndex(signedBlock, txIndex)
  if (sd4.isErr()) throw Error(sd4.error)
  console.log(sd4.value.hexData) // `4d792044617461`

  // Getting data-submission data by transaction signer(author) via free function
  sdkBlock.submitDataBySigner(signedBlock, alice.address()).forEach((sd) => console.log(sd.hexData)) // `4d792044617461`,...

  process.exit()
}
main()
