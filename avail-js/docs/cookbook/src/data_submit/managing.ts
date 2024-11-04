import { SubmitDataTxSuccess } from "../../../../src/sdk/transactions/da"
import { SDK, Keyring, Account, Block, sdkBlock, WaitFor, DataSubmission } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const api = sdk.api
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")
  const account = new Account(sdk, alice)

  const tx1 = await account.createApplicationKey("My Key")
  if (tx1.isErr) throw Error(tx1.reason)
  const appId = tx1.event.id

  // Calling Submit Data transaction via SDK
  const data = "My Data"
  const tx2 = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { app_id: appId })
  if (tx2.isErr) throw Error(tx2.reason)
  displayTx(tx2)
  /*
    Block Hash: 0x0b41bdac4d23dbb9deff74875f01c7666d8153239112e4fdd99735de437be982, Block Number: 3, 
    Transaction Hash: 0x0437d152899d4733dc3106610b93a1026289644d5e9659e1126545e860d8aaa8, Transaction Index: 1,
    App Id: 10, Submitted Data (in Hex): 4d792044617461, Data Hash: 0x7d5ece96efb6832141afe1af96b337d5fec3839bd3cedd31317e4fa7279001e0, 
    Author: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, Events count: 7
  */
  /*
    JSON format
    {
      "isErr": false,
      "txData": {
        "data": "4d792044617461"
      },
      "event": {
        "who": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
        "dataHash": "0x7d5ece96efb6832141afe1af96b337d5fec3839bd3cedd31317e4fa7279001e0"
      },
      "appId": 10,
      "events": [...],
      "txHash": "0x0437d152899d4733dc3106610b93a1026289644d5e9659e1126545e860d8aaa8",
      "txIndex": 1,
      "blockHash": "0x0b41bdac4d23dbb9deff74875f01c7666d8153239112e4fdd99735de437be982",
      "blockNumber": 3
    }
  */

  // Calling Submit Data transaction via Account Instance
  account.setAppId(appId)
  const tx3 = await account.submitData(data)
  if (tx3.isErr) throw Error(tx3.reason)
  displayTx(tx3)
  /*
    Block Hash: 0x1f6dc25004db61e52ac571b9f30fdf34f3f5c03e8355e928734c31ca0c0c6b01, Block Number: 4, 
    Transaction Hash: 0x65d60b24c5a5e4fd2ad8779896a5108ff516dd8eade0cfeeaf4b565aa512cef6, Transaction Index: 1, 
    App Id: 10, Submitted Data (in Hex): 4d792044617461, Data Hash: 0x7d5ece96efb6832141afe1af96b337d5fec3839bd3cedd31317e4fa7279001e0, 
    Author: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY, Events count: 7
  */

  // Dissecting Data Submission Object via Block instance
  const block = await Block.New(api, tx3.blockHash)
  block.submitDataAll().forEach((ds) => displayDataSubmission(ds)) // You can use `submitDataBySigner`, `submitDataByIndex`, `submitDataByHash` as well here. Check the `Block` section
  /*
    Transaction Hash: 0x65d60b24c5a5e4fd2ad8779896a5108ff516dd8eade0cfeeaf4b565aa512cef6, Transaction Index: 1, 
    Transaction Signer(Author): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY), Transaction App Id: 10, 
    Submitted Data(in Hex): 4d792044617461, Submitted Data(in Ascii): My Data
  */
  /*
    JSON format
    {
      "txHash": "0x65d60b24c5a5e4fd2ad8779896a5108ff516dd8eade0cfeeaf4b565aa512cef6",
      "txIndex": 1,
      "hexData": "4d792044617461",
      "txSigner": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "appId": 10
    }
  */

  // Dissecting Data Submission Object via free function
  const signedBlock = await api.rpc.chain.getBlock(tx3.blockHash)
  sdkBlock.submitDataAll(signedBlock).forEach((ds) => displayDataSubmission(ds)) // You can use `submitDataBySigner`, `submitDataByIndex`, `submitDataByHash` as well here. Check the `Block` section
  /*
    Transaction Hash: 0x65d60b24c5a5e4fd2ad8779896a5108ff516dd8eade0cfeeaf4b565aa512cef6, Transaction Index: 1, 
    Transaction Signer(Author): 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY), Transaction App Id: 10, 
    Submitted Data(in Hex): 4d792044617461, Submitted Data(in Ascii): My Data
  */

  process.exit()
}

function displayTx(tx: SubmitDataTxSuccess) {
  console.log(
    `Block Hash: ${tx.blockHash}, Block Number: ${tx.blockNumber}, Transaction Hash: ${tx.txHash}, Transaction Index: ${tx.txIndex}`,
  )
  console.log(
    `App Id: ${tx.appId}, Submitted Data (in Hex): ${tx.txData.data}, Data Hash: ${tx.event.dataHash}, Author: ${tx.event.who}`,
  )
  console.log("Events count: " + tx.events.length) // We have access to all events that were generated for that transaction
}

function displayDataSubmission(ds: DataSubmission) {
  console.log(
    `Transaction Hash: ${ds.txHash}, Transaction Index: ${ds.txIndex}, Transaction Signer(Author): ${ds.txSigner})`,
  )
  console.log(
    `Transaction App Id: ${ds.appId}, Submitted Data(in Hex): ${ds.hexData}, Submitted Data(in Ascii): ${ds.toAscii()}`,
  )
}
main()
