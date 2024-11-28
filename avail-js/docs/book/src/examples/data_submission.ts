import { SDK, Events, Block, DataSubmission, CallData } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  const account = SDK.alice()

  // Application Key Creation
  const key = "My JS Key"
  const tx = sdk.tx.dataAvailability.createApplicationKey(key)
  const keyRes = (await tx.executeWaitForInclusion(account))._unsafeUnwrap()

  const keyEvent = keyRes.findFirstEvent(Events.DataAvailability.ApplicationKeyCreated)
  if (keyEvent == null) throw Error("Failed to find Key Event")
  const appId = keyEvent.id

  // Data Submission
  const data = "My Data"
  const tx2 = sdk.tx.dataAvailability.submitData(data)
  const submitRes = (await tx2.executeWaitForInclusion(account, { app_id: appId }))._unsafeUnwrap()

  console.log(
    `Block Hash: ${submitRes.blockHash}, Block Number: ${submitRes.blockNumber}, Tx Hash: ${submitRes.txHash}, Tx Index: ${submitRes.txIndex}`,
  )

  const callData = await submitRes.getData(api, CallData.DataAvailability.SubmitData)
  if (callData != null) {
    console.log(`Call data: 0x${callData.data}`)
  }

  // Getting Data Submission from Block #1
  const block = await Block.New(api, submitRes.blockHash)

  // dataSubmissionsBySigner, dataSubmissionsByIndex, dataSubmissionsByHash, dataSubmissionsByAppId
  const dataSubmissions = block.dataSubmissionsAll()
  for (const ds of dataSubmissions) {
    console.log(
      `Tx Hash: ${ds.txHash}, Tx Index: ${ds.txIndex}, Data: ${ds.hexData}, Tx Signer: ${ds.txSigner}, App Id: ${ds.appId}`,
    )
    console.log(`Ascii data: ${ds.toAscii()}`)
  }

  // Getting Data Submission from Block #2
  const dataSubmissions2 = block.transactionAll().flatMap((tx, index) => {
    const ds = DataSubmission.fromGenericTx(tx, index)
    return ds ? ds : []
  })
  dataSubmissions2.forEach((ds) => {
    console.log(
      `Tx Hash: ${ds.txHash}, Tx Index: ${ds.txIndex}, Data: ${ds.hexData}, Tx Signer: ${ds.txSigner}, App Id: ${ds.appId}`,
    )
    console.log(`Ascii data: ${ds.toAscii()}`)
  })
}
