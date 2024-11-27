import { SDK } from "./../../../../src/index"

const main = async () => {
  const sdk = await SDK.New(SDK.localEndpoint())
  const api = sdk.api

  // Author Rotate Keys
  const keys = await api.rpc.author.rotateKeys()
  console.log(keys.toString())

  process.exit()
}
main()
