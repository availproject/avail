import { SDK } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)

  const key = "5HKPmK9GYtE1PSLsS1qiYU9xQ9Si1NcEhdeCq9sw5bqu4ns8"
  const value = await sdk.api.query.system.account(key)
  console.log(value.toHuman())

  process.exit()
}

main()
