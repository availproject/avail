import { SDK, WaitFor, Keyring, TransactionOptions } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

  /*
      Setting up application id and/or nonce for any transaction is as simple as just defining them
      and passing them as the last argument to any sdk transaction call.
  
      TransactionOptions interface has the following fields:
        - app_id?: number
        - nonce?: number
        - era?: number
        - blockHash?: H256
    */
  let nonce = await sdk.util.getNonceNode(alice.address)
  let options: TransactionOptions = { app_id: 1, nonce }
  let result = await sdk.tx.dataAvailability.submitData("Data", WaitFor.BlockInclusion, alice, options)

  console.log(JSON.stringify(result, null, 2))

  process.exit()
}
main()
