import { SDK } from "avail-js-sdk"

const main = async () => {
  const providerEndpoint = "ws://127.0.0.1:9944"
  const sdk = await SDK.New(providerEndpoint)
  const api = sdk.api

  console.log("Genesis Hash: " + api.genesisHash)
  console.log("Runtime Version Impl Name: " + api.runtimeVersion.implName)
  console.log("Runtime Version Spec Version: " + api.runtimeVersion.specVersion)
  console.log("Runtime Chain: " + api.runtimeChain)

  process.exit()
}
main()
