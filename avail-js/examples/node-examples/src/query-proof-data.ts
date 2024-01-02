import { initialize, getKeyringFromSeed } from "avail-js-sdk"
import config from "../../config"

/**
 * Example of getting the proof for the particular leaf in the block.
 */
const main = async () => {
  try {
    const data = "Any data"
    const api = await initialize(config.endpoint)
    const rpc: any = api.rpc
    const keyring = getKeyringFromSeed(config.seed)
    const options = { app_id: config.appId, nonce: -1 }

    await api.tx.dataAvailability.submitData(data).signAndSend(keyring, options, async ({ status, txIndex }) => {
      // We need to wait for finalization (1 mn)
      if (status.isFinalized) {
        // Print inclusion data
        console.log(`Transaction included at blockHash ${status.asFinalized}`)

        // After block finalization we can query for the Merkle proof of the data submitted
        const hash = status.asFinalized
        const daHeader = await rpc.kate.queryDataProof(txIndex, hash)

        console.log(`Fetched proof from Avail for txn index ${txIndex} inside block ${hash}`)
        console.log(`Header: ${JSON.stringify(daHeader, undefined, 2)}`)
        process.exit(0)
      }
    })
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
