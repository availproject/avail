import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import config from "../../config"

/**
 * Example of getting the proof for data submission.
 */
const main = async () => {
  try {
    const data = "Any data"

    const api = await initialize(config.endpoint)
    const rpc: any = api.rpc
    const keyring = getKeyringFromSeed(config.seed)
    const options = { app_id: config.appId, nonce: -1 }

    await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options, async ({ status }) => {
      // We need to wait for finalization (1 mn)
      if (status.isFinalized) {
        // Print inclusion data
        console.log(`Transaction included at blockHash ${status.asFinalized}`)

        // After block finalization we can get block hash and query proof
        const hash = status.asFinalized

        // query proof for the row and col in the provided block
        const proof = await rpc.kate.queryProof([{ row: 0, col: 0 }], hash)

        // Print proof
        console.log(`Proof: ${proof}`)
        process.exit(0)
      }
    })
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
