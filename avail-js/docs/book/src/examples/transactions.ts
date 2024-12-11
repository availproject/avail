import { SDK, Block, Events, CallData } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  const account = SDK.alice()

  const dest = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
  const value = SDK.oneAvail()
  const tx = sdk.tx.balances.transferKeepAlive(dest, value)
  const res = (await tx.executeWaitForInclusion(account))._unsafeUnwrap()

  const block = await Block.New(api, res.blockHash)

  // transactionAll, transactionBySigner, transactionByIndex, transactionByHash, transactionByAppId
  for (const [index, tx] of block.transactionAll().entries()) {
    console.log(`Tx Pallet name: ${tx.method.section}, Tx Name: ${tx.method.method}, Tx Hash: ${tx.hash.toHex()}`)

    const eventRecords = await Events.fetchEvents(api, res.blockHash, index)
    for (const eventRecord of eventRecords) {
      console.log(`\tEvent Pallet name: ${eventRecord.event.section}, Event Name: ${eventRecord.event.method}`)
    }
    const balance_tx = CallData.Balances.TransferKeepAlive.decode(tx)
    if (balance_tx != null) {
      console.log(`Transfer dest: ${balance_tx.dest}, value: ${balance_tx.value}`)
    }
  }
}
