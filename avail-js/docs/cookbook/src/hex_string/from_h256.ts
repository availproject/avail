import { SDK } from "./../../../../src/index"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const api = sdk.api

  // Converting from H256 to Hex String
  const hash = await api.rpc.chain.getFinalizedHead()
  console.log(hash.toHex()) // `0xb410c0c0b5939567e5a558a4930ae030375894043c2dd5f3c35cea4133470f7f`

  process.exit()
}
main()
