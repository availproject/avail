import { SDK, utils } from "./../../../../src/index"

export async function run() {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  const account = SDK.alice()
  const dest = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty" // Bob
  const value = SDK.oneAvail()

  const tx = sdk.tx.balances.transferKeepAlive(dest, value)

  await tx.executeAndForget(account, {})

  const nonce = await utils.getNonceNode(api, account.address)
  await tx.executeAndForget(account, { nonce })
  await tx.executeAndForget(account, { nonce: nonce + 1 })
}
