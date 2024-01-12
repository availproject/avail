import { initialize, getKeyringFromSeed, extractData } from "avail-js-sdk"
import config from "../../config"

/**
 * Example to remark with event.
 */
const main = async () => {
  try {
    const data =  "event"
    const api = await initialize(config.endpoint)
    const keyring = getKeyringFromSeed(config.seed)
    const options = { app_id: config.appId, nonce: -1 }

    await api.tx.system.remarkWithEvent(data).signAndSend(keyring, options, async ({ status, events, txHash }) => {
      if (status.isInBlock) {
        // Print inclusion data
        console.log(`Transaction included at blockHash ${status.asInBlock}`)
        events.forEach(({ event: { data, method, section } }) => {
          console.log(`\t' ${section}.${method}:: ${data}`)
        })

        // Print input
        const data = await extractData(api, status.asInBlock.toString(), txHash.toString())
        console.log(`Event remark: ${data}`)

        process.exit(0)
      }
    })
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()