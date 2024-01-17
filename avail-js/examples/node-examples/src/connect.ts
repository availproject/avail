import { initialize } from "avail-js-sdk" // Global import
import { isConnected, disconnect } from "avail-js-sdk/chain" // Modular import
import config from "../../config"

/**
 * Example to connect to a chain and get the ApiPromise.
 */
const main = async () => {
  const api = await initialize(config.endpoint)
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version(),
  ])

  console.log(
    `Connected to chain ${chain} using ${nodeName} and node version ${nodeVersion} - is connected: ${isConnected()}`,
  )
  await disconnect()
  process.exit(0)
}
main()
