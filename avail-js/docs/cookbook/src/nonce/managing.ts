import { SDK, Account, WaitFor } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const alice = SDK.alice()
  const nonce = await sdk.util.getNonceNode(alice.address)
  const data = "My Data"

  // Nonce can be passed as part of transaction options instance
  const tx = await sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { nonce: nonce })
  if (tx.isErr()) throw Error(tx.error.reason) // We expect that the call will succeed

  // Account instance can be set to use a specific nonce
  const account = new Account(sdk, alice)
  account.setNonce(await account.getNonceNode())
  const tx2 = (await account.submitData(data))._unsafeUnwrap()

  // Setting nonce for each transaction individually
  const nonce2 = await sdk.util.getNonceNode(alice.address)
  const txs3 = await Promise.all([
    sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { nonce: nonce2 }),
    sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { nonce: nonce2 + 1 }),
    sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { nonce: nonce2 + 2 }),
    sdk.tx.dataAvailability.submitData(data, WaitFor.BlockInclusion, alice, { nonce: nonce2 + 3 }),
  ])
  txs3.forEach((tx) => {
    if (tx.isErr()) throw Error(tx.error.reason) // We expect that the call will succeed
  })

  // Setting nonce for each transaction individually
  let nonce3 = await sdk.util.getNonceNode(alice.address)
  for (let i = 0; i < 4; ++i) {
    await sdk.tx.dataAvailability.submitDataNoWait(data, alice, { nonce: nonce3 })
    nonce3 += 1
  }

  // Setting nonce for each transaction individually
  for (let i = 0; i < 4; ++i) {
    await sdk.tx.dataAvailability.submitDataNoWait(data, alice, { nonce: await sdk.util.getNonceNode(alice.address) })
  }

  // Can be done as well with Account instance
  for (let i = 0; i < 4; ++i) {
    account.setNonce(await sdk.util.getNonceNode(alice.address))
    await account.submitDataNoWait(data)
  }

  process.exit()
}
main()
