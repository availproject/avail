import { SDK } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // System Account Iter
  const accountIter = await sdk.api.query.system.account.entries()
  for (const [key, value] of accountIter) {
    const acc: any = value
    console.log(key.toHuman())
    console.log(acc.nonce.toNumber())
    console.log(acc.consumers.toNumber())
    console.log(acc.providers.toNumber())
    console.log(acc.sufficients.toNumber())
    console.log(acc.data.free.toString())
    console.log(acc.data.reserved.toString())
    console.log(acc.data.frozen.toString())
    console.log(acc.data.flags.toString())
  }

  // System Account
  const account: any = await sdk.api.query.system.account("5GNJqTPyNqANBkUVMN1LPPrxXnFouWXoe2wNSmmEoLctxiZY")
  console.log(account.nonce.toNumber())
  console.log(account.consumers.toNumber())
  console.log(account.providers.toNumber())
  console.log(account.sufficients.toNumber())
  console.log(account.data.free.toString())
  console.log(account.data.reserved.toString())
  console.log(account.data.frozen.toString())
  console.log(account.data.flags.toString())

  process.exit()
}
main()
