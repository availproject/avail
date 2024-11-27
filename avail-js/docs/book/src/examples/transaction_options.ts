import { SDK, WaitFor, TransactionOptions } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const alice = SDK.alice()

  /*
      Setting up application id and/or nonce for any transaction is as simple as just defining them
      and passing them as the last argument to any sdk transaction call.
  
      TransactionOptions interface has the following fields:
        - app_id?: number
        - nonce?: number
        - era?: number
        - blockHash?: H256
    */
  const nonce = await sdk.util.getNonceNode(alice.address)
  const options: TransactionOptions = { app_id: 1, nonce }
  const result = await sdk.tx.dataAvailability.submitData("Data", WaitFor.BlockInclusion, alice, options)

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
