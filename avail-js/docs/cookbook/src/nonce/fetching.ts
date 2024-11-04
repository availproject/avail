import { SDK, Keyring, Account, sdkUtil } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const api = sdk.api

  // Fetching the nonce via chain state query
  const address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" // Alice's address
  const systemAccount: any = await api.query.system.account(address)
  const nonce = parseInt(systemAccount.nonce)
  console.log(nonce) // 1

  // Fetching the nonce via SDK
  const nonce2 = await sdk.util.getNonceState(address)
  console.log(nonce2) // 1

  // Fetching the nonce via free function
  const nonce3 = await sdkUtil.getNonceState(api, address)
  console.log(nonce3) // 1

  // Fetching the nonce via Account instance
  const account = new Account(sdk, new Keyring({ type: "sr25519" }).addFromUri("//Alice"))
  console.log(await account.getNonceState()) // 1

  // Fetching the nonce via rpc call
  const r: any = await api.rpc.system.accountNextIndex(address)
  const nonce4 = parseInt(r.toString())
  console.log(nonce4) // 1

  // Fetching the nonce via SDK
  const nonce5 = await sdk.util.getNonceNode(address)
  console.log(nonce5) // 1

  // Fetching the nonce via free function
  const nonce6 = await sdkUtil.getNonceNode(api, address)
  console.log(nonce6) // 1

  // Fetching the nonce via Account instance
  console.log(await account.getNonceNode()) // 1

  process.exit()
}
main()
