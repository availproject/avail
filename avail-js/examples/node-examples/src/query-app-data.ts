import { initialize } from "avail-js-sdk" // Global import
import { disconnect } from "avail-js-sdk/chain" // Modular import
import config from "../../config"

/**
 * Example returning data that belongs to application id.
 */
const main = async () => {
  const api = await initialize(config.endpoint)
  const rpc: any = api.rpc
  const appId = config.appId
  const finalizedHead = await api.rpc.chain.getFinalizedHead()
  console.log(`Latest finalized block: ${finalizedHead}`)

  const appData = await rpc.kate.queryAppData(appId, finalizedHead)
  console.log(`Application data: ${appData}`)
  await disconnect()
  process.exit(0)
}
main()
