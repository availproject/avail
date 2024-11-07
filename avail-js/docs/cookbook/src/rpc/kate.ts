import { SDK, Account, WaitFor } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api
  const alice = Account.alice(sdk)

  alice.setWaitFor(WaitFor.BlockFinalization)
  const tx = (await alice.submitData("My Data"))._unsafeUnwrap()
  const { blockHash, txIndex } = tx.details

  // Kate Block Length
  const blockLength = await (api.rpc as any).kate.blockLength(blockHash)
  console.log(blockLength.max.normal.toNumber())
  console.log(blockLength.max.operational.toNumber())
  console.log(blockLength.max.mandatory.toNumber())
  console.log(blockLength.cols.toNumber())
  console.log(blockLength.rows.toNumber())
  console.log(blockLength.chunkSize.toNumber())

  // Kate Query Data Proof
  const dataProof = await (api.rpc as any).kate.queryDataProof(txIndex, blockHash)
  console.log(dataProof.dataProof.roots.dataRoot.toString())
  console.log(dataProof.dataProof.roots.blobRoot.toString())
  console.log(dataProof.dataProof.roots.bridgeRoot.toString())
  dataProof.dataProof.proof.forEach((e: any) => console.log(e))
  console.log(dataProof.dataProof.numberOfLeaves.toNumber())
  console.log(dataProof.dataProof.leafIndex.toNumber())
  console.log(dataProof.dataProof.leaf.toString())
  console.log(dataProof.message.toString())

  // Kate Query Proof
  const cell = [[0, 0]]
  const proof = await (api.rpc as any).kate.queryProof(cell, blockHash)
  proof.forEach((e: any) => e.forEach((g: any) => console.log(g.toString())))

  // Kate Query Rows
  const rows = [0]
  const rowsResult = await (api.rpc as any).kate.queryRows(rows, blockHash)
  rowsResult.forEach((e: any) => e.forEach((g: any) => console.log(g.toString())))

  process.exit()
}
main()
