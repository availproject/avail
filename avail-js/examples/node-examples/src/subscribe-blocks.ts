import { DaHeader, initialize } from "avail-js-sdk"
import config from "../../config"

/**
 * Subscribes to new blocks and displays block number every time new block is seen by the node.
 */
const main = async () => {
  try {
    const api = await initialize(config.endpoint)
    const waitForBlocks = 3
    let count = 0

    const unsubscribe = await api.rpc.chain.subscribeNewHeads((header: DaHeader) => {
      console.log(`Chain is at block: #${header.number}`)
      console.log("Header data", header.toJSON())
      if (header.extension.isV3) {
        console.log("Extension", header.extension.asV3.toJSON())
      }
      count++
      if (waitForBlocks === count) {
        console.log(`Unsubscribing from a new headers subscription after ${count} blocks`)
        unsubscribe()
        process.exit(0)
      }
    })
  } catch (err) {
    console.error(err)
    process.exit(1)
  }
}
main()
