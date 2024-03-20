import { getKeyringFromSeed, initialize, signedExtensions, types } from "avail-js-sdk" // Global import
import config from "../../config"

/**
 * Example returning data that belongs to application id.
 */
const main = async () => {
  const data = "Any data"
  const api = await initialize(config.endpoint)
  const rpc: any = api.rpc
  const keyring = getKeyringFromSeed(config.seed)
  const appId = config.appId === 0 ? 1 : config.appId
  const options = { app_id: appId, nonce: -1 }

  await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options, async ({ status }) => {
    // We need to wait for finalization (1 mn)
    if (status.isFinalized) {
      // Print inclusion data
      console.log(`Transaction included at blockHash ${status.asFinalized}`)

      // After block finalization we can query for the App data
      const hash = status.asFinalized
      const appData = await rpc.kate.queryAppData(appId, hash)
      console.log(`Application data: ${appData}`)
      process.exit(0)
    }
  })
}
main()
