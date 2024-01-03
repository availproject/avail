import { initialize } from "avail-js-sdk"
import { Header } from "@polkadot/types/interfaces/runtime"
import config from "../../config"

/**
 * Subscribes to new blocks and displays block number every time new block is seen by the node.
 */
const main = async () => {
  try {
    const api = await initialize(config.endpoint)
    const waitForBlocks = 3
    let count = 0

    const unsubscribe = await api.rpc.chain.subscribeNewHeads((header: Header) => {
      console.log(`Chain is at block: #${header.number}`)
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
