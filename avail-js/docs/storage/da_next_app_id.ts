import { SDK } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  const value = await sdk.api.query.dataAvailability.nextAppId()
  console.log(value.toHuman())

  process.exit()
}

main()
