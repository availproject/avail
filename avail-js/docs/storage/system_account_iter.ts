import { SDK } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  const accounts = await sdk.api.query.system.account.entries()
  for (const [key, value] of accounts) {
    console.log(key.toHuman())
    console.log(value.toHuman())
  }

  process.exit()
}

main()
