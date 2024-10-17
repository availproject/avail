import { SDK, WaitFor, Keyring, TransactionOptions } from "./../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice")

  /*
    The SDK provides two easy to use but slightly different nonce functions.
    - `getNonceState` - Returns nonce from the state of network. This only changes
                        on a per block basis
    - `getNonceNode` -  Returns nonce from the node's storage. This gets the nonce
                        from the state and it updates it if there are existing
                        transactions already in the mem pool. 
    
    If in doubt, use `getNonceNode`.
  */
  let nonceState1 = await sdk.util.getNonceState(alice.address)
  let nonceNode1 = await sdk.util.getNonceNode(alice.address)
  console.log(`Nonce State: ${nonceState1}, Nonce Node: ${nonceNode1}`)
  let _result = await sdk.tx.dataAvailability.submitDataNoWait("Data", alice)
  let nonceState2 = await sdk.util.getNonceState(alice.address)
  let nonceNode2 = await sdk.util.getNonceNode(alice.address)
  console.log(`Nonce State: ${nonceState2}, Nonce Node: ${nonceNode2}`)

  process.exit()
}
main()
