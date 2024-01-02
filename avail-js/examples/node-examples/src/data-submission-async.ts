import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import config from "../../config"

/**
 * Example to submit data and retrieve the data from the block.
 */
const main = async () => {
  try {
    const data = "Any data"

    const api = await initialize(config.endpoint)
    const keyring = getKeyringFromSeed(config.seed)
    const options = { app_id: config.appId, nonce: -1 }
    let blockHash: string | undefined = undefined

    await new Promise(async (resolve) => {
      api.tx.dataAvailability.submitData(data).signAndSend(keyring, options, async (result) => {
        console.log(`Tx status: ${result.status}`)
        if (result.isInBlock) {
          blockHash = result.status.asInBlock.toString()
          resolve(result)
        }
      })
    })

    console.log(`Transaction included at block hash ${blockHash}`)
    process.exit(0)
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
